// ecs-rust: a Rust port of a PSR-12 rule subset of ecs-go, built to benchmark
// Rust vs Go on the same token-based fixer engine. Walks .php files and applies
// the ported rules across all cores (rayon), mirroring ecs-go's defaults.

mod lexer;
mod rules;
mod stream;
mod token;

// A parallel-friendly allocator: many worker threads each churn token buffers,
// so a contention-aware allocator (mimalloc) beats the system one here.
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut fix = false;
    let mut paths: Vec<String> = Vec::new();
    for a in &args {
        match a.as_str() {
            "--fix" => fix = true,
            "list-rules" => {
                for name in rules::RULE_NAMES {
                    println!("{name}");
                }
                return;
            }
            "-h" | "--help" => {
                usage();
                return;
            }
            _ => paths.push(a.clone()),
        }
    }
    if paths.is_empty() {
        paths.push(".".to_string());
    }

    configure_thread_pool();

    let start = Instant::now();
    let files = find_php_files(&paths);
    let total = files.len();

    // map/reduce: no shared atomics or locks on the hot path.
    let changed: usize = files
        .par_iter()
        .with_min_len(16) // batch small files so scheduling overhead stays low
        .map(|path| fix_file(path, fix) as usize)
        .sum();

    let elapsed = start.elapsed();
    eprintln!(
        "ecs-rust: {} of {} files changed in {:.3}s",
        changed,
        total,
        elapsed.as_secs_f64()
    );
}

// Work is a mix of CPU (lex/fix) and blocking I/O (read/write). Oversubscribing
// cores lets a thread make progress while a peer waits on the filesystem. Honor
// an explicit RAYON_NUM_THREADS if the user set one.
fn configure_thread_pool() {
    if std::env::var_os("RAYON_NUM_THREADS").is_some() {
        return;
    }
    if let Ok(n) = std::thread::available_parallelism() {
        let _ = rayon::ThreadPoolBuilder::new()
            .num_threads(n.get() * 2)
            .build_global();
    }
}

fn fix_file(path: &Path, write: bool) -> bool {
    let src = match std::fs::read(path) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let toks = lexer::lex(&src);
    let mut s = stream::Stream::new(&src, toks);
    if !rules::fix(&mut s) {
        return false;
    }
    let after = s.render();
    if after == src {
        return false;
    }
    if write {
        let _ = std::fs::write(path, &after);
    }
    true
}

fn find_php_files(paths: &[String]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for p in paths {
        for entry in WalkDir::new(p).into_iter().filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !(e.file_type().is_dir()
                && (name == "vendor" || name == "node_modules" || name == ".git"))
        }) {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            if entry.file_type().is_file()
                && entry.path().extension().map(|x| x == "php").unwrap_or(false)
            {
                out.push(entry.into_path());
            }
        }
    }
    out
}

fn usage() {
    print!(
        "ecs-rust - Rust port of an ecs-go PSR-12 subset (benchmark)\n\n\
         Usage:\n  \
         ecs-rust [paths...]        check paths (default: .)\n  \
         ecs-rust --fix [paths...]  fix paths in place\n  \
         ecs-rust list-rules        print the ported rule FQCNs\n\n\
         Runs across all CPU cores by default.\n"
    );
}
