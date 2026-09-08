// ecs-rust: a Rust port of a PSR-12 rule subset of ecs-go, built to benchmark
// Rust vs Go on the same token-based fixer engine. Walks .php files and applies
// the ported rules across all cores (rayon), mirroring ecs-go's defaults.

mod lexer;
mod rules;
mod stream;
mod token;

use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
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

    let start = Instant::now();
    let files = find_php_files(&paths);
    let total = files.len();

    let changed = AtomicUsize::new(0);
    files.par_iter().for_each(|path| {
        if fix_file(path, fix) {
            changed.fetch_add(1, Ordering::Relaxed);
        }
    });

    let elapsed = start.elapsed();
    eprintln!(
        "ecs-rust: {} of {} files changed in {:.3}s",
        changed.load(Ordering::Relaxed),
        total,
        elapsed.as_secs_f64()
    );
}

fn fix_file(path: &PathBuf, write: bool) -> bool {
    let src = match std::fs::read(path) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let mut s = stream::Stream::new(lexer::lex(&src));
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
