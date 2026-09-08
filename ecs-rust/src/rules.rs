// The ported PSR-12 rule subset, applied in the same order ecs-go's rules.All()
// yields them (Casing before Spacing), so the Rust output matches the Go output
// for the matching `--rules` subset. Each rule mirrors the PHP-CS-Fixer fixer of
// the same name.

use crate::stream::Stream;
use crate::token::Kind;

// FQCNs of the ported fixers, matching ecs-go's names. Used to build the Go-side
// `--rules` subset for a fair, identical-work comparison.
pub const RULE_NAMES: &[&str] = &[
    r"PhpCsFixer\Fixer\Casing\LowercaseKeywordsFixer",
    r"PhpCsFixer\Fixer\Casing\ConstantCaseFixer",
    r"PhpCsFixer\Fixer\Casing\LowercaseStaticReferenceFixer",
    r"PhpCsFixer\Fixer\NamespaceNotation\NoLeadingNamespaceWhitespaceFixer",
    r"PhpCsFixer\Fixer\Semicolon\NoSinglelineWhitespaceBeforeSemicolonsFixer",
    r"PhpCsFixer\Fixer\Whitespace\NoWhitespaceInBlankLineFixer",
    r"PhpCsFixer\Fixer\Semicolon\SpaceAfterSemicolonFixer",
    r"PhpCsFixer\Fixer\PhpTag\BlankLineAfterOpeningTagFixer",
    r"PhpCsFixer\Fixer\Whitespace\NoTrailingWhitespaceFixer",
    r"PhpCsFixer\Fixer\Whitespace\SingleBlankLineAtEofFixer",
];

pub fn fix(s: &mut Stream) -> bool {
    let mut changed = false;
    changed |= lowercase_keywords(s);
    changed |= constant_case(s);
    changed |= lowercase_static_reference(s);
    changed |= no_leading_namespace_whitespace(s);
    changed |= no_singleline_whitespace_before_semicolons(s);
    changed |= no_whitespace_in_blank_line(s);
    changed |= space_after_semicolon(s);
    changed |= blank_line_after_opening_tag(s);
    changed |= no_trailing_whitespace(s);
    changed |= single_blank_line_at_eof(s);
    changed
}

// --- helpers ---------------------------------------------------------------

fn next_significant_value(s: &Stream, i: usize) -> Vec<u8> {
    let mut j = i + 1;
    while j < s.len() {
        if s.kind(j) != Kind::Whitespace {
            return s.bytes(j).to_vec();
        }
        j += 1;
    }
    Vec::new()
}

fn member_prev(s: &Stream, i: usize) -> bool {
    let mut j = i as isize - 1;
    while j >= 0 {
        let k = j as usize;
        if s.kind(k) != Kind::Whitespace {
            let b = s.bytes(k);
            return b == b"->" || b == b"?->" || b == b"::";
        }
        j -= 1;
    }
    false
}

fn has_newline(v: &[u8]) -> bool {
    v.iter().any(|&c| c == b'\n' || c == b'\r')
}

// --- rules -----------------------------------------------------------------

fn lowercase_keywords(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::Keyword {
            continue;
        }
        if next_significant_value(s, i) == b"=" {
            continue;
        }
        let lower = s.bytes(i).to_ascii_lowercase();
        if lower.as_slice() != s.bytes(i) {
            s.set_owned(i, lower);
            changed = true;
        }
    }
    changed
}

fn constant_case(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::Ident {
            continue;
        }
        let lower = s.bytes(i).to_ascii_lowercase();
        if lower != b"true" && lower != b"false" && lower != b"null" {
            continue;
        }
        if member_prev(s, i) {
            continue;
        }
        if lower.as_slice() != s.bytes(i) {
            s.set_owned(i, lower);
            changed = true;
        }
    }
    changed
}

fn lowercase_static_reference(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        let kind = s.kind(i);
        if kind != Kind::Ident && kind != Kind::Keyword {
            continue;
        }
        let lower = s.bytes(i).to_ascii_lowercase();
        if lower != b"self" && lower != b"static" && lower != b"parent" {
            continue;
        }
        if member_prev(s, i) || next_significant_value(s, i) == b"=" {
            continue;
        }
        if lower.as_slice() != s.bytes(i) {
            s.set_owned(i, lower);
            changed = true;
        }
    }
    changed
}

fn no_leading_namespace_whitespace(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 1..s.len() {
        if s.kind(i) != Kind::Keyword || s.bytes(i) != b"namespace" {
            continue;
        }
        if s.kind(i - 1) != Kind::Whitespace {
            continue;
        }
        let prev = s.bytes(i - 1).to_vec();
        match prev.iter().rposition(|&c| c == b'\n') {
            None => continue,
            Some(idx) => {
                let trimmed = prev[..idx + 1].to_vec();
                if trimmed != prev {
                    s.set_owned(i - 1, trimmed);
                    changed = true;
                }
            }
        }
    }
    changed
}

fn no_singleline_whitespace_before_semicolons(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = s.len();
    while i >= 2 {
        i -= 1;
        if s.kind(i) != Kind::Punct || s.bytes(i) != b";" {
            continue;
        }
        if s.kind(i - 1) == Kind::Whitespace && !has_newline(s.bytes(i - 1)) {
            s.remove_at(i - 1);
            changed = true;
        }
    }
    changed
}

fn no_whitespace_in_blank_line(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::Whitespace {
            continue;
        }
        let val = s.bytes(i).to_vec();
        let mut segments: Vec<&[u8]> = val.split(|&c| c == b'\n').collect();
        if segments.len() <= 2 {
            continue;
        }
        let mut touched = false;
        let n = segments.len();
        for seg in segments.iter_mut().take(n - 1).skip(1) {
            if !seg.is_empty() {
                *seg = b"";
                touched = true;
            }
        }
        if touched {
            let joined = segments.join(&b'\n');
            s.set_owned(i, joined);
            changed = true;
        }
    }
    changed
}

fn space_after_semicolon(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b";" {
            i += 1;
            continue;
        }
        if i + 1 >= s.len() {
            break;
        }
        let nk = s.kind(i + 1);
        if nk == Kind::Whitespace || nk == Kind::CloseTag {
            i += 1;
            continue;
        }
        if nk == Kind::Punct && (s.bytes(i + 1) == b")" || s.bytes(i + 1) == b";") {
            i += 1;
            continue;
        }
        s.insert_owned(i + 1, Kind::Whitespace, b" ".to_vec());
        changed = true;
        i += 2; // skip inserted whitespace
    }
    changed
}

fn blank_line_after_opening_tag(s: &mut Stream) -> bool {
    if s.len() < 3 {
        return false;
    }
    if s.kind(0) != Kind::OpenTag || s.bytes(0) != b"<?php" {
        return false;
    }
    if s.kind(1) != Kind::Whitespace {
        return false;
    }
    let ws = s.bytes(1);
    if !ws.starts_with(b"\n") || ws.starts_with(b"\n\n") {
        return false;
    }
    let mut v = vec![b'\n'];
    v.extend_from_slice(ws);
    s.set_owned(1, v);
    true
}

fn no_trailing_whitespace(s: &mut Stream) -> bool {
    let mut changed = false;
    let last = s.len().wrapping_sub(1);
    for i in 0..s.len() {
        if s.kind(i) != Kind::Whitespace {
            continue;
        }
        let v = strip_trailing_ws(s.bytes(i), i == last);
        if v.as_slice() != s.bytes(i) {
            s.set_owned(i, v);
            changed = true;
        }
    }
    changed
}

// Mirrors NoTrailingWhitespace's two regexes: drop [ \t]+ immediately before a
// (\r?\n), and, only in the file's final token, drop [ \t]+ at the very end.
fn strip_trailing_ws(val: &[u8], is_last: bool) -> Vec<u8> {
    let mut out = Vec::with_capacity(val.len());
    let mut i = 0;
    while i < val.len() {
        if val[i] == b' ' || val[i] == b'\t' {
            let start = i;
            while i < val.len() && (val[i] == b' ' || val[i] == b'\t') {
                i += 1;
            }
            let before_newline = (i < val.len() && val[i] == b'\n')
                || (i + 1 < val.len() && val[i] == b'\r' && val[i + 1] == b'\n');
            let at_end = i == val.len();
            if before_newline || (at_end && is_last) {
                // drop the run
            } else {
                out.extend_from_slice(&val[start..i]);
            }
        } else {
            out.push(val[i]);
            i += 1;
        }
    }
    out
}

fn single_blank_line_at_eof(s: &mut Stream) -> bool {
    if s.len() == 0 {
        return false;
    }
    let last = s.len() - 1;
    if s.kind(last) == Kind::Whitespace {
        if s.bytes(last) != b"\n" {
            s.set_owned(last, b"\n".to_vec());
            return true;
        }
        return false;
    }
    if s.bytes(last).ends_with(b"\n") {
        return false;
    }
    s.insert_owned(s.len(), Kind::Whitespace, b"\n".to_vec());
    true
}
