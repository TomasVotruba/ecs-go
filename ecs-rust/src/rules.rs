// The ported PSR-12 rule subset, applied in the same order ecs-go's rules.All()
// yields them (Casing before Spacing), so the Rust output matches the Go output
// for the matching `--rules` subset. Each rule mirrors the PHP-CS-Fixer fixer of
// the same name.

use crate::stream::Stream;
use crate::token::Kind;

// FQCNs of the ported fixers, matching ecs-go's names. Used to build the Go-side
// `--rules` subset for a fair, identical-work comparison.
pub const RULE_NAMES: &[&str] = &[
    r"PhpCsFixer\Fixer\PhpTag\FullOpeningTagFixer",
    r"PhpCsFixer\Fixer\Whitespace\LineEndingFixer",
    r"PhpCsFixer\Fixer\Operator\ObjectOperatorWithoutWhitespaceFixer",
    r"PhpCsFixer\Fixer\Operator\StandardizeNotEqualsFixer",
    r"PhpCsFixer\Fixer\Semicolon\NoEmptyStatementFixer",
    r"PhpCsFixer\Fixer\Casing\LowercaseKeywordsFixer",
    r"PhpCsFixer\Fixer\Casing\ConstantCaseFixer",
    r"PhpCsFixer\Fixer\Casing\LowercaseStaticReferenceFixer",
    r"PhpCsFixer\Fixer\CastNotation\LowercaseCastFixer",
    r"PhpCsFixer\Fixer\CastNotation\ShortScalarCastFixer",
    r"PhpCsFixer\Fixer\Casing\MagicConstantCasingFixer",
    r"PhpCsFixer\Fixer\Casing\MagicMethodCasingFixer",
    r"PhpCsFixer\Fixer\Casing\NativeFunctionCasingFixer",
    r"PhpCsFixer\Fixer\Casing\IntegerLiteralCaseFixer",
    r"PhpCsFixer\Fixer\Casing\ClassReferenceNameCasingFixer",
    r"PhpCsFixer\Fixer\NamespaceNotation\NoLeadingNamespaceWhitespaceFixer",
    r"PhpCsFixer\Fixer\Semicolon\NoSinglelineWhitespaceBeforeSemicolonsFixer",
    r"PhpCsFixer\Fixer\Whitespace\NoWhitespaceInBlankLineFixer",
    r"PhpCsFixer\Fixer\Semicolon\SpaceAfterSemicolonFixer",
    r"PhpCsFixer\Fixer\Operator\BinaryOperatorSpacesFixer",
    r"PhpCsFixer\Fixer\Operator\TernaryOperatorSpacesFixer",
    r"PhpCsFixer\Fixer\Operator\ConcatSpaceFixer",
    r"PhpCsFixer\Fixer\CastNotation\CastSpacesFixer",
    r"PhpCsFixer\Fixer\PhpTag\BlankLineAfterOpeningTagFixer",
    r"PhpCsFixer\Fixer\Whitespace\NoTrailingWhitespaceFixer",
    r"PhpCsFixer\Fixer\Comment\NoTrailingWhitespaceInCommentFixer",
    r"PhpCsFixer\Fixer\Whitespace\SingleBlankLineAtEofFixer",
    r"PhpCsFixer\Fixer\LanguageConstruct\DeclareEqualNormalizeFixer",
    r"PhpCsFixer\Fixer\LanguageConstruct\SingleSpaceAroundConstructFixer",
    r"PhpCsFixer\Fixer\FunctionNotation\NoSpacesAfterFunctionNameFixer",
    r"PhpCsFixer\Fixer\Whitespace\SpacesInsideParenthesesFixer",
    r"PhpCsFixer\Fixer\Operator\UnaryOperatorSpacesFixer",
    r"PhpCsFixer\Fixer\Import\NoLeadingImportSlashFixer",
    r"PhpCsFixer\Fixer\ControlStructure\ElseifFixer",
    r"PhpCsFixer\Fixer\ControlStructure\SwitchCaseSemicolonToColonFixer",
    r"PhpCsFixer\Fixer\ControlStructure\SwitchCaseSpaceFixer",
    r"PhpCsFixer\Fixer\Basic\NoMultipleStatementsPerLineFixer",
    r"PhpCsFixer\Fixer\FunctionNotation\ReturnTypeDeclarationFixer",
    r"PhpCsFixer\Fixer\Whitespace\IndentationTypeFixer",
    r"PhpCsFixer\Fixer\NamespaceNotation\BlankLinesBeforeNamespaceFixer",
    r"PhpCsFixer\Fixer\NamespaceNotation\BlankLineAfterNamespaceFixer",
    r"PhpCsFixer\Fixer\ClassNotation\NoBlankLinesAfterClassOpeningFixer",
    r"PhpCsFixer\Fixer\Whitespace\NoExtraBlankLinesFixer",
    r"PhpCsFixer\Fixer\PhpTag\NoClosingTagFixer",
];

// Rules run in ecs-go's rules.All() order, restricted to the ported set, so the
// Rust output matches the Go output for the matching `--rules` subset.
pub fn fix(s: &mut Stream) -> bool {
    let mut changed = false;
    changed |= full_opening_tag(s);
    changed |= line_ending(s);
    changed |= object_operator_without_whitespace(s);
    changed |= standardize_not_equals(s);
    changed |= no_empty_statement(s);
    changed |= lowercase_keywords(s);
    changed |= constant_case(s);
    changed |= lowercase_static_reference(s);
    changed |= lowercase_cast(s);
    changed |= short_scalar_cast(s);
    changed |= magic_constant_casing(s);
    changed |= magic_method_casing(s);
    changed |= native_function_casing(s);
    changed |= integer_literal_case(s);
    changed |= class_reference_name_casing(s);
    changed |= no_leading_namespace_whitespace(s);
    changed |= no_singleline_whitespace_before_semicolons(s);
    changed |= no_whitespace_in_blank_line(s);
    changed |= space_after_semicolon(s);
    changed |= binary_operator_spaces(s);
    changed |= ternary_operator_spaces(s);
    changed |= concat_space(s);
    changed |= cast_spaces(s);
    changed |= blank_line_after_opening_tag(s);
    changed |= no_trailing_whitespace(s);
    changed |= no_trailing_whitespace_in_comment(s);
    changed |= single_blank_line_at_eof(s);
    changed |= declare_equal_normalize(s);
    changed |= single_space_around_construct(s);
    changed |= no_spaces_after_function_name(s);
    changed |= spaces_inside_parentheses(s);
    changed |= unary_operator_spaces(s);
    changed |= no_leading_import_slash(s);
    changed |= elseif(s);
    changed |= switch_case_semicolon_to_colon(s);
    changed |= switch_case_space(s);
    changed |= no_multiple_statements_per_line(s);
    changed |= return_type_declaration(s);
    changed |= indentation_type(s);
    changed |= blank_lines_before_namespace(s);
    changed |= blank_line_after_namespace(s);
    changed |= no_blank_lines_after_class_opening(s);
    changed |= no_extra_blank_lines(s);
    changed |= no_closing_tag(s);
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

// Index of the first non-whitespace token before i, or None.
fn prev_significant_index(s: &Stream, i: usize) -> Option<usize> {
    let mut j = i as isize - 1;
    while j >= 0 {
        if s.kind(j as usize) != Kind::Whitespace {
            return Some(j as usize);
        }
        j -= 1;
    }
    None
}

// First non-whitespace token index at or after i.
fn skip_ws(s: &Stream, mut i: usize) -> usize {
    while i < s.len() && s.kind(i) == Kind::Whitespace {
        i += 1;
    }
    i
}

// Case-insensitive keyword match.
fn kw_eq(s: &Stream, i: usize, w: &[u8]) -> bool {
    s.kind(i) == Kind::Keyword && s.bytes(i).eq_ignore_ascii_case(w)
}

fn is_blank(v: &[u8]) -> bool {
    v.iter()
        .all(|&c| c == b' ' || c == b'\t' || c == b'\n' || c == b'\r')
}

// Index of the delimiter matching the opener "(", "{" or "[" at i, or None.
fn match_forward(s: &Stream, i: usize) -> Option<usize> {
    if s.kind(i) != Kind::Punct {
        return None;
    }
    let (open, closer): (&[u8], &[u8]) = match s.bytes(i) {
        b"(" => (b"(", b")"),
        b"{" => (b"{", b"}"),
        b"[" => (b"[", b"]"),
        _ => return None,
    };
    let mut depth = 0i32;
    for j in i..s.len() {
        if s.kind(j) != Kind::Punct {
            continue;
        }
        let b = s.bytes(j);
        if b == open {
            depth += 1;
        } else if b == closer {
            depth -= 1;
            if depth == 0 {
                return Some(j);
            }
        }
    }
    None
}

// Index of the delimiter matching the closer ")", "}" or "]" at i, or None.
fn match_backward(s: &Stream, i: usize) -> Option<usize> {
    if s.kind(i) != Kind::Punct {
        return None;
    }
    let (opener, closer): (&[u8], &[u8]) = match s.bytes(i) {
        b")" => (b"(", b")"),
        b"}" => (b"{", b"}"),
        b"]" => (b"[", b"]"),
        _ => return None,
    };
    let mut depth = 0i32;
    let mut j = i as isize;
    while j >= 0 {
        let k = j as usize;
        if s.kind(k) == Kind::Punct {
            let b = s.bytes(k);
            if b == closer {
                depth += 1;
            } else if b == opener {
                depth -= 1;
                if depth == 0 {
                    return Some(k);
                }
            }
        }
        j -= 1;
    }
    None
}

// Leading indentation of the line containing token idx.
fn line_indent(s: &Stream, idx: usize) -> Vec<u8> {
    let mut j = idx as isize;
    while j >= 0 {
        let k = j as usize;
        if s.kind(k) == Kind::Whitespace {
            let v = s.bytes(k);
            if let Some(p) = v.iter().rposition(|&c| c == b'\n') {
                return v[p + 1..].to_vec();
            }
        }
        j -= 1;
    }
    Vec::new()
}

const CAST_TYPES: &[&[u8]] = &[
    b"int", b"integer", b"bool", b"boolean", b"float", b"double", b"real", b"string", b"array",
    b"object", b"unset", b"binary",
];

fn is_cast_type(s: &Stream, i: usize) -> bool {
    let k = s.kind(i);
    if k != Kind::Ident && k != Kind::Keyword {
        return false;
    }
    let lo = s.bytes(i).to_ascii_lowercase();
    CAST_TYPES.iter().any(|t| *t == lo.as_slice())
}

// Detects a cast at an opening "(" and returns (type token index, closing ")"
// index). Mirrors ecs-go's castAt guard so grouping/calls are not mistaken.
fn cast_at(s: &Stream, open: usize) -> Option<(usize, usize)> {
    if s.kind(open) != Kind::Punct || s.bytes(open) != b"(" {
        return None;
    }
    let mut j = open + 1;
    if j < s.len() && s.kind(j) == Kind::Whitespace {
        j += 1;
    }
    if j >= s.len() || !is_cast_type(s, j) {
        return None;
    }
    let type_idx = j;
    j += 1;
    if j < s.len() && s.kind(j) == Kind::Whitespace {
        j += 1;
    }
    if j >= s.len() || s.kind(j) != Kind::Punct || s.bytes(j) != b")" {
        return None;
    }
    if let Some(p) = prev_significant_index(s, open) {
        let pk = s.kind(p);
        let pb = s.bytes(p);
        if pk == Kind::Ident || pk == Kind::Variable || pb == b")" || pb == b"]" {
            return None;
        }
    }
    Some((type_idx, j))
}

// Whether index i sits inside a declare( ... ) header.
fn inside_declare_args(s: &Stream, i: usize) -> bool {
    let mut depth = 0i32;
    let mut j = i as isize - 1;
    while j >= 0 {
        let k = j as usize;
        if s.kind(k) == Kind::Punct {
            match s.bytes(k) {
                b")" => depth += 1,
                b"(" => {
                    if depth == 0 {
                        return match prev_significant_index(s, k) {
                            Some(p) => kw_eq(s, p, b"declare"),
                            None => false,
                        };
                    }
                    depth -= 1;
                }
                b";" => {
                    if depth == 0 {
                        return false;
                    }
                }
                _ => {}
            }
        }
        j -= 1;
    }
    false
}

const CLASS_LIKE_KW: &[&[u8]] = &[b"class", b"interface", b"trait", b"enum"];

fn is_class_like_kw(s: &Stream, i: usize) -> bool {
    if s.kind(i) != Kind::Keyword {
        return false;
    }
    let lo = s.bytes(i).to_ascii_lowercase();
    CLASS_LIKE_KW.iter().any(|t| *t == lo.as_slice())
}

fn brace_opens_class_like(s: &Stream, brace: usize) -> bool {
    let mut j = brace as isize - 1;
    while j >= 0 {
        let k = j as usize;
        if s.kind(k) == Kind::Punct {
            let b = s.bytes(k);
            if b == b";" || b == b"{" || b == b"}" {
                return false;
            }
        }
        if is_class_like_kw(s, k) {
            return true;
        }
        j -= 1;
    }
    false
}

fn in_class_like_body(s: &Stream, i: usize) -> bool {
    let mut depth = 0i32;
    let mut j = i as isize - 1;
    while j >= 0 {
        let k = j as usize;
        if s.kind(k) == Kind::Punct {
            match s.bytes(k) {
                b"}" => depth += 1,
                b"{" => {
                    if depth == 0 {
                        return brace_opens_class_like(s, k);
                    }
                    depth -= 1;
                }
                _ => {}
            }
        }
        j -= 1;
    }
    false
}

fn is_operand(s: &Stream, i: usize) -> bool {
    match s.kind(i) {
        Kind::Variable | Kind::Ident => true,
        Kind::Punct => {
            let b = s.bytes(i);
            b == b")" || b == b"]"
        }
        _ => false,
    }
}

// Forces exactly one space on both sides of every token the predicate selects.
// Whitespace spanning a newline is left alone.
fn normalize_space_around<F: Fn(&Stream, usize) -> bool>(s: &mut Stream, target: F) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if !target(s, i) {
            i += 1;
            continue;
        }
        if i > 0 {
            if s.kind(i - 1) == Kind::Whitespace {
                if !has_newline(s.bytes(i - 1)) && s.bytes(i - 1) != b" " {
                    s.set_owned(i - 1, b" ".to_vec());
                    changed = true;
                }
            } else {
                s.insert_owned(i, Kind::Whitespace, b" ".to_vec());
                i += 1;
                changed = true;
            }
        }
        if i + 1 < s.len() {
            if s.kind(i + 1) == Kind::Whitespace {
                if !has_newline(s.bytes(i + 1)) && s.bytes(i + 1) != b" " {
                    s.set_owned(i + 1, b" ".to_vec());
                    changed = true;
                }
            } else {
                s.insert_owned(i + 1, Kind::Whitespace, b" ".to_vec());
                changed = true;
            }
        }
        i += 1;
    }
    changed
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

fn full_opening_tag(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::OpenTag && s.bytes(i) == b"<?" {
            s.set_owned(i, b"<?php".to_vec());
            if i + 1 < s.len() && s.kind(i + 1) != Kind::Whitespace {
                s.insert_owned(i + 1, Kind::Whitespace, b" ".to_vec());
            }
            changed = true;
        }
        i += 1;
    }
    changed
}

fn lowercase_cast(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if let Some((type_idx, _)) = cast_at(s, i) {
            let lo = s.bytes(type_idx).to_ascii_lowercase();
            if lo.as_slice() != s.bytes(type_idx) {
                s.set_owned(type_idx, lo);
                changed = true;
            }
        }
    }
    changed
}

fn short_scalar_cast(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if let Some((type_idx, _)) = cast_at(s, i) {
            let lo = s.bytes(type_idx).to_ascii_lowercase();
            let short: Option<&[u8]> = match lo.as_slice() {
                b"integer" => Some(b"int"),
                b"boolean" => Some(b"bool"),
                b"double" | b"real" => Some(b"float"),
                b"binary" => Some(b"string"),
                _ => None,
            };
            if let Some(sh) = short {
                s.set_owned(type_idx, sh.to_vec());
                changed = true;
            }
        }
    }
    changed
}

const BINARY_OPERATORS: &[&[u8]] = &[
    b"==", b"===", b"!=", b"!==", b"<>", b"<=", b">=", b"<=>", b"<", b">", b"&&", b"||", b"??",
    b"=>",
];

fn binary_operator_spaces(s: &mut Stream) -> bool {
    normalize_space_around(s, |s, i| {
        if s.kind(i) != Kind::Punct {
            return false;
        }
        let b = s.bytes(i);
        if b == b"=" {
            return next_significant_value(s, i).as_slice() != b"&" && !inside_declare_args(s, i);
        }
        BINARY_OPERATORS.iter().any(|o| *o == b)
    })
}

fn concat_space(s: &mut Stream) -> bool {
    normalize_space_around(s, |s, i| s.kind(i) == Kind::Punct && s.bytes(i) == b".")
}

fn match_ternary_colon(s: &Stream, i: usize) -> isize {
    let mut depth = 0i32;
    let mut bracket = 0i32;
    for j in (i + 1)..s.len() {
        if s.kind(j) != Kind::Punct {
            continue;
        }
        match s.bytes(j) {
            b"{" => {
                if bracket == 0 {
                    return -1;
                }
                bracket += 1;
            }
            b"(" | b"[" => bracket += 1,
            b")" | b"]" | b"}" => {
                if bracket == 0 {
                    return -1;
                }
                bracket -= 1;
            }
            b";" => {
                if bracket == 0 {
                    return -1;
                }
            }
            b"?" => {
                if bracket == 0 {
                    depth += 1;
                }
            }
            b":" => {
                if bracket == 0 {
                    if depth == 0 {
                        return j as isize;
                    }
                    depth -= 1;
                }
            }
            _ => {}
        }
    }
    -1
}

fn ternary_is_elvis(s: &Stream, i: usize, m: usize) -> bool {
    for k in (i + 1)..m {
        if s.kind(k) != Kind::Whitespace {
            return false;
        }
    }
    true
}

fn ternary_operator_spaces(s: &mut Stream) -> bool {
    use std::collections::HashMap;
    let mut mark: HashMap<usize, (bool, bool)> = HashMap::new(); // (before, after)
    for i in 0..s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"?" {
            continue;
        }
        if let Some(p) = prev_significant_index(s, i) {
            match s.bytes(p) {
                b":" | b"(" | b"," | b"|" | b"&" => continue,
                _ => {}
            }
        }
        let m = match_ternary_colon(s, i);
        if m < 0 {
            continue;
        }
        let m = m as usize;
        if ternary_is_elvis(s, i, m) {
            mark.insert(i, (true, false));
            mark.insert(m, (false, true));
        } else {
            mark.insert(i, (true, true));
            mark.insert(m, (true, true));
        }
    }
    if mark.is_empty() {
        return false;
    }
    let mut changed = false;
    let mut i = s.len();
    while i > 0 {
        i -= 1;
        let (before, after) = match mark.get(&i) {
            Some(&v) => v,
            None => continue,
        };
        if after && i + 1 < s.len() {
            if s.kind(i + 1) == Kind::Whitespace {
                if !has_newline(s.bytes(i + 1)) && s.bytes(i + 1) != b" " {
                    s.set_owned(i + 1, b" ".to_vec());
                    changed = true;
                }
            } else {
                s.insert_owned(i + 1, Kind::Whitespace, b" ".to_vec());
                changed = true;
            }
        }
        if before && i > 0 {
            if s.kind(i - 1) == Kind::Whitespace {
                if !has_newline(s.bytes(i - 1)) && s.bytes(i - 1) != b" " {
                    s.set_owned(i - 1, b" ".to_vec());
                    changed = true;
                }
            } else {
                s.insert_owned(i, Kind::Whitespace, b" ".to_vec());
                changed = true;
            }
        }
    }
    changed
}

fn cast_spaces(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if !(s.kind(i) == Kind::Punct && s.bytes(i) == b"(") {
            i += 1;
            continue;
        }
        let open = i;
        let mut j = open + 1;
        let mut open_ws: isize = -1;
        if j < s.len() && s.kind(j) == Kind::Whitespace {
            open_ws = j as isize;
            j += 1;
        }
        if j >= s.len() || !is_cast_type(s, j) {
            i += 1;
            continue;
        }
        j += 1;
        let mut close_ws: isize = -1;
        if j < s.len() && s.kind(j) == Kind::Whitespace {
            close_ws = j as isize;
            j += 1;
        }
        if j >= s.len() || s.kind(j) != Kind::Punct || s.bytes(j) != b")" {
            i += 1;
            continue;
        }
        let close_idx = j;
        if let Some(p) = prev_significant_index(s, open) {
            let pk = s.kind(p);
            let pb = s.bytes(p);
            if pk == Kind::Ident || pk == Kind::Variable || pb == b")" || pb == b"]" {
                i += 1;
                continue;
            }
        }
        // mutate high index to low so earlier indices stay valid
        if close_idx + 1 < s.len() {
            if s.kind(close_idx + 1) == Kind::Whitespace {
                if !has_newline(s.bytes(close_idx + 1)) && s.bytes(close_idx + 1) != b" " {
                    s.set_owned(close_idx + 1, b" ".to_vec());
                    changed = true;
                }
            } else {
                s.insert_owned(close_idx + 1, Kind::Whitespace, b" ".to_vec());
                changed = true;
            }
        }
        if close_ws >= 0 {
            s.remove_at(close_ws as usize);
            changed = true;
        }
        if open_ws >= 0 {
            s.remove_at(open_ws as usize);
            changed = true;
        }
        i += 1;
    }
    changed
}

fn no_trailing_whitespace_in_comment(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        let k = s.kind(i);
        if k != Kind::Comment && k != Kind::DocComment {
            continue;
        }
        let val = s.bytes(i).to_vec();
        let lines: Vec<&[u8]> = val.split(|&c| c == b'\n').collect();
        let n = lines.len();
        let mut out: Vec<u8> = Vec::with_capacity(val.len());
        let mut touched = false;
        for (idx, line) in lines.iter().enumerate() {
            let mut end = line.len();
            while end > 0 && (line[end - 1] == b' ' || line[end - 1] == b'\t') {
                end -= 1;
            }
            if end != line.len() {
                touched = true;
            }
            out.extend_from_slice(&line[..end]);
            if idx + 1 < n {
                out.push(b'\n');
            }
        }
        if touched {
            s.set_owned(i, out);
            changed = true;
        }
    }
    changed
}

fn declare_equal_normalize(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if !(s.kind(i) == Kind::Punct && s.bytes(i) == b"=" && inside_declare_args(s, i)) {
            i += 1;
            continue;
        }
        if i + 1 < s.len() && s.kind(i + 1) == Kind::Whitespace {
            s.remove_at(i + 1);
            changed = true;
        }
        if i >= 1 && s.kind(i - 1) == Kind::Whitespace {
            s.remove_at(i - 1);
            i -= 1;
            changed = true;
        }
        i += 1;
    }
    changed
}

const CONSTRUCT_KW: &[&[u8]] = &[
    b"abstract", b"as", b"case", b"catch", b"class", b"do", b"else", b"elseif", b"final",
    b"finally", b"for", b"foreach", b"function", b"if", b"insteadof", b"interface", b"namespace",
    b"new", b"private", b"protected", b"public", b"readonly", b"static", b"switch", b"trait",
    b"try", b"use", b"while",
];

fn is_construct_kw(s: &Stream, i: usize) -> bool {
    if s.kind(i) != Kind::Keyword {
        return false;
    }
    let lo = s.bytes(i).to_ascii_lowercase();
    CONSTRUCT_KW.iter().any(|t| *t == lo.as_slice())
}

fn single_space_around_construct(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if !is_construct_kw(s, i) || i + 1 >= s.len() {
            i += 1;
            continue;
        }
        let kw = s.bytes(i).to_ascii_lowercase();
        let class_ref = kw == b"static" || kw == b"class";
        if s.kind(i + 1) == Kind::Whitespace {
            if class_ref
                && !has_newline(s.bytes(i + 1))
                && i + 2 < s.len()
                && s.kind(i + 2) == Kind::Punct
                && s.bytes(i + 2) == b"("
            {
                s.remove_at(i + 1);
                changed = true;
                i += 1;
                continue;
            }
            if !has_newline(s.bytes(i + 1)) && s.bytes(i + 1) != b" " {
                s.set_owned(i + 1, b" ".to_vec());
                changed = true;
            }
            i += 1;
            continue;
        }
        match s.bytes(i + 1) {
            b"::" | b"->" | b"?->" | b";" | b"," | b")" | b":" => {}
            b"(" => {
                if !class_ref {
                    s.insert_owned(i + 1, Kind::Whitespace, b" ".to_vec());
                    changed = true;
                    i += 1;
                }
            }
            _ => {
                s.insert_owned(i + 1, Kind::Whitespace, b" ".to_vec());
                changed = true;
                i += 1;
            }
        }
        i += 1;
    }
    changed
}

fn no_spaces_after_function_name(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Ident
            && i + 2 < s.len()
            && s.kind(i + 1) == Kind::Whitespace
            && !has_newline(s.bytes(i + 1))
            && s.kind(i + 2) == Kind::Punct
            && s.bytes(i + 2) == b"("
        {
            s.remove_at(i + 1);
            changed = true;
        }
        i += 1;
    }
    changed
}

fn spaces_inside_parentheses(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Punct
            && s.bytes(i) == b"("
            && i + 1 < s.len()
            && s.kind(i + 1) == Kind::Whitespace
            && !has_newline(s.bytes(i + 1))
        {
            s.remove_at(i + 1);
            changed = true;
        }
        if s.kind(i) == Kind::Punct
            && s.bytes(i) == b")"
            && i >= 1
            && s.kind(i - 1) == Kind::Whitespace
            && !has_newline(s.bytes(i - 1))
        {
            s.remove_at(i - 1);
            i -= 1;
            changed = true;
        }
        i += 1;
    }
    changed
}

fn unary_operator_spaces(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        let is_incdec = s.kind(i) == Kind::Punct && (s.bytes(i) == b"++" || s.bytes(i) == b"--");
        if !is_incdec {
            i += 1;
            continue;
        }
        if i >= 2
            && s.kind(i - 1) == Kind::Whitespace
            && !has_newline(s.bytes(i - 1))
            && is_operand(s, i - 2)
        {
            s.remove_at(i - 1);
            i -= 1;
            changed = true;
        }
        if i + 2 < s.len()
            && s.kind(i + 1) == Kind::Whitespace
            && !has_newline(s.bytes(i + 1))
            && is_operand(s, i + 2)
        {
            s.remove_at(i + 1);
            changed = true;
        }
        i += 1;
    }
    changed
}

fn no_leading_import_slash(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if kw_eq(s, i, b"use") {
            let mut j = skip_ws(s, i + 1);
            if j < s.len() && s.kind(j) == Kind::Keyword {
                let lo = s.bytes(j).to_ascii_lowercase();
                if lo == b"function" || lo == b"const" {
                    j = skip_ws(s, j + 1);
                }
            }
            if j < s.len() && s.kind(j) == Kind::Punct && s.bytes(j) == b"\\" {
                s.remove_at(j);
                changed = true;
            }
        }
        i += 1;
    }
    changed
}

fn elseif(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if kw_eq(s, i, b"else")
            && i + 2 < s.len()
            && s.kind(i + 1) == Kind::Whitespace
            && !has_newline(s.bytes(i + 1))
            && kw_eq(s, i + 2, b"if")
        {
            s.set_owned(i, b"elseif".to_vec());
            s.remove_at(i + 2);
            s.remove_at(i + 1);
            changed = true;
        }
        i += 1;
    }
    changed
}

fn is_case_kw(s: &Stream, i: usize) -> bool {
    if s.kind(i) != Kind::Keyword {
        return false;
    }
    let lo = s.bytes(i).to_ascii_lowercase();
    lo == b"case" || lo == b"default"
}

// Finds the ":" (or ";") ending the case/default label at i, skipping ternary
// "?:" pairs. Returns (index, is_semicolon), or None.
fn case_terminator(s: &Stream, i: usize) -> Option<(usize, bool)> {
    let mut ternary = 0i32;
    for k in (i + 1)..s.len() {
        if s.kind(k) != Kind::Punct {
            continue;
        }
        match s.bytes(k) {
            b"?" => ternary += 1,
            b":" => {
                if ternary > 0 {
                    ternary -= 1;
                } else {
                    return Some((k, false));
                }
            }
            b";" => {
                if ternary == 0 {
                    return Some((k, true));
                }
            }
            b"{" | b"}" => return None,
            _ => {}
        }
    }
    None
}

fn switch_case_semicolon_to_colon(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if !is_case_kw(s, i) || in_class_like_body(s, i) {
            continue;
        }
        if let Some((idx, true)) = case_terminator(s, i) {
            s.set_owned(idx, b":".to_vec());
            changed = true;
        }
    }
    changed
}

fn switch_case_space(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if is_case_kw(s, i) && !in_class_like_body(s, i) {
            if let Some((idx, _)) = case_terminator(s, i) {
                if idx > 0 && s.kind(idx - 1) == Kind::Whitespace && !has_newline(s.bytes(idx - 1)) {
                    s.remove_at(idx - 1);
                    changed = true;
                }
            }
        }
        i += 1;
    }
    changed
}

fn no_multiple_statements_per_line(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut depth = 0i32;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct {
            i += 1;
            continue;
        }
        match s.bytes(i) {
            b"(" | b"[" => {
                depth += 1;
                i += 1;
                continue;
            }
            b")" | b"]" => {
                depth -= 1;
                i += 1;
                continue;
            }
            b";" => {}
            _ => {
                i += 1;
                continue;
            }
        }
        if depth != 0 || i + 1 >= s.len() {
            i += 1;
            continue;
        }
        let mut next_idx = i + 1;
        let mut inline_ws = false;
        if s.kind(next_idx) == Kind::Whitespace {
            if has_newline(s.bytes(next_idx)) {
                i += 1;
                continue;
            }
            inline_ws = true;
            next_idx += 1;
        }
        if next_idx >= s.len() {
            i += 1;
            continue;
        }
        if s.kind(next_idx) == Kind::Punct
            && (s.bytes(next_idx) == b"}" || s.bytes(next_idx) == b";")
        {
            i += 1;
            continue;
        }
        if s.kind(next_idx) == Kind::CloseTag {
            i += 1;
            continue;
        }
        let indent = line_indent(s, i);
        let mut v = vec![b'\n'];
        v.extend_from_slice(&indent);
        if inline_ws {
            s.set_owned(i + 1, v);
        } else {
            s.insert_owned(i + 1, Kind::Whitespace, v);
        }
        changed = true;
        i += 1;
    }
    changed
}

fn is_return_type_colon(s: &Stream, i: usize) -> bool {
    let c = match prev_significant_index(s, i) {
        Some(x) => x,
        None => return false,
    };
    if s.kind(c) != Kind::Punct || s.bytes(c) != b")" {
        return false;
    }
    let open = match match_backward(s, c) {
        Some(x) => x,
        None => return false,
    };
    let p = match prev_significant_index(s, open) {
        Some(x) => x,
        None => return false,
    };
    if s.kind(p) == Kind::Keyword {
        let v = s.bytes(p).to_ascii_lowercase();
        return v == b"function" || v == b"fn";
    }
    if s.kind(p) == Kind::Ident {
        let mut q = prev_significant_index(s, p);
        if let Some(qi) = q {
            if s.kind(qi) == Kind::Punct && s.bytes(qi) == b"&" {
                q = prev_significant_index(s, qi);
            }
        }
        if let Some(qi) = q {
            return kw_eq(s, qi, b"function");
        }
    }
    false
}

fn return_type_declaration(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if !(s.kind(i) == Kind::Punct && s.bytes(i) == b":") {
            i += 1;
            continue;
        }
        if !is_return_type_colon(s, i) {
            i += 1;
            continue;
        }
        if i + 1 < s.len() {
            if s.kind(i + 1) == Kind::Whitespace {
                if !has_newline(s.bytes(i + 1)) && s.bytes(i + 1) != b" " {
                    s.set_owned(i + 1, b" ".to_vec());
                    changed = true;
                }
            } else {
                s.insert_owned(i + 1, Kind::Whitespace, b" ".to_vec());
                changed = true;
            }
        }
        if i > 0 && s.kind(i - 1) == Kind::Whitespace && !has_newline(s.bytes(i - 1)) {
            s.remove_at(i - 1);
            changed = true;
            i -= 1;
        }
        i += 1;
    }
    changed
}

fn indentation_type(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::Whitespace {
            continue;
        }
        let v = s.bytes(i).to_vec();
        if !v.contains(&b'\t') {
            continue;
        }
        let parts: Vec<&[u8]> = v.split(|&c| c == b'\n').collect();
        let mut out: Vec<u8> = Vec::with_capacity(v.len());
        let mut touched = false;
        for (k, part) in parts.iter().enumerate() {
            if k > 0 {
                out.push(b'\n');
            }
            if k >= 1 && part.contains(&b'\t') {
                for &c in part.iter() {
                    if c == b'\t' {
                        out.extend_from_slice(b"    ");
                    } else {
                        out.push(c);
                    }
                }
                touched = true;
            } else {
                out.extend_from_slice(part);
            }
        }
        if touched {
            s.set_owned(i, out);
            changed = true;
        }
    }
    changed
}

fn blank_lines_before_namespace(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if !kw_eq(s, i, b"namespace") || member_prev(s, i) {
            continue;
        }
        if i == 0 {
            continue;
        }
        if s.kind(i - 1) == Kind::Whitespace
            && has_newline(s.bytes(i - 1))
            && s.bytes(i - 1) != b"\n\n"
        {
            s.set_owned(i - 1, b"\n\n".to_vec());
            changed = true;
        }
    }
    changed
}

fn blank_line_after_namespace(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if !kw_eq(s, i, b"namespace") || member_prev(s, i) {
            i += 1;
            continue;
        }
        let mut semi: isize = -1;
        let mut j = i + 1;
        while j < s.len() {
            if s.kind(j) == Kind::Punct {
                if s.bytes(j) == b"{" {
                    break;
                }
                if s.bytes(j) == b";" {
                    semi = j as isize;
                    break;
                }
            }
            j += 1;
        }
        if semi < 0 {
            i += 1;
            continue;
        }
        let semi = semi as usize;
        if semi + 1 >= s.len() {
            i += 1;
            continue;
        }
        if next_significant_value(s, semi).is_empty() {
            i += 1;
            continue;
        }
        if s.kind(semi + 1) == Kind::Whitespace {
            if has_newline(s.bytes(semi + 1)) && s.bytes(semi + 1) != b"\n\n" {
                s.set_owned(semi + 1, b"\n\n".to_vec());
                changed = true;
            }
        } else {
            s.insert_owned(semi + 1, Kind::Whitespace, b"\n\n".to_vec());
            changed = true;
        }
        i += 1;
    }
    changed
}

fn no_blank_lines_after_class_opening(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if !is_class_like_kw(s, i) || member_prev(s, i) {
            continue;
        }
        let mut brace: isize = -1;
        let mut j = i + 1;
        while j < s.len() {
            if s.kind(j) == Kind::Punct {
                if s.bytes(j) == b"{" {
                    brace = j as isize;
                    break;
                }
                if s.bytes(j) == b";" {
                    break;
                }
            }
            j += 1;
        }
        if brace < 0 {
            continue;
        }
        let brace = brace as usize;
        if brace + 1 >= s.len() || s.kind(brace + 1) != Kind::Whitespace {
            continue;
        }
        let ws = s.bytes(brace + 1).to_vec();
        if ws.iter().filter(|&&c| c == b'\n').count() <= 1 {
            continue;
        }
        let idx = ws.iter().rposition(|&c| c == b'\n').unwrap();
        let mut want = vec![b'\n'];
        want.extend_from_slice(&ws[idx + 1..]);
        if ws != want {
            s.set_owned(brace + 1, want);
            changed = true;
        }
    }
    changed
}

// Collapse runs of 3+ '\n' to exactly 2, mirroring ecs-go's \n{3,} -> \n\n.
fn collapse_3plus_newlines(v: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(v.len());
    let mut i = 0;
    while i < v.len() {
        if v[i] == b'\n' {
            let start = i;
            while i < v.len() && v[i] == b'\n' {
                i += 1;
            }
            let run = i - start;
            if run >= 3 {
                out.extend_from_slice(b"\n\n");
            } else {
                for _ in 0..run {
                    out.push(b'\n');
                }
            }
        } else {
            out.push(v[i]);
            i += 1;
        }
    }
    out
}

fn no_extra_blank_lines(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Whitespace {
            i += 1;
            continue;
        }
        // merge adjacent whitespace tokens so a run of blank lines is one token
        while i + 1 < s.len() && s.kind(i + 1) == Kind::Whitespace {
            let mut merged = s.bytes(i).to_vec();
            merged.extend_from_slice(s.bytes(i + 1));
            s.set_owned(i, merged);
            s.remove_at(i + 1);
        }
        let cur = s.bytes(i).to_vec();
        let collapsed = collapse_3plus_newlines(&cur);
        if collapsed != cur {
            s.set_owned(i, collapsed);
            changed = true;
        }
        i += 1;
    }
    changed
}

fn no_closing_tag(s: &mut Stream) -> bool {
    let mut last: isize = -1;
    for i in 0..s.len() {
        if s.kind(i) == Kind::InlineHtml && !is_blank(s.bytes(i)) {
            return false;
        }
        if s.kind(i) == Kind::CloseTag {
            last = i as isize;
        }
    }
    if last < 0 {
        return false;
    }
    let last = last as usize;
    for j in (last + 1)..s.len() {
        let k = s.kind(j);
        if k == Kind::Whitespace {
            continue;
        }
        if k == Kind::InlineHtml && is_blank(s.bytes(j)) {
            continue;
        }
        return false;
    }
    let mut k = s.len();
    while k > last {
        k -= 1;
        s.remove_at(k);
    }
    if s.len() > 0 {
        let li = s.len() - 1;
        if s.kind(li) == Kind::Whitespace {
            s.set_owned(li, b"\n".to_vec());
        } else {
            s.insert_owned(s.len(), Kind::Whitespace, b"\n".to_vec());
        }
    }
    true
}

// --- ported batch 1: casing + simple token rewrites -------------------------

// First non-whitespace token index at or after i+1, or None.
fn next_significant_index(s: &Stream, i: usize) -> Option<usize> {
    let mut j = i + 1;
    while j < s.len() {
        if s.kind(j) != Kind::Whitespace {
            return Some(j);
        }
        j += 1;
    }
    None
}

fn line_ending(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        let k = s.kind(i);
        if k != Kind::Whitespace && k != Kind::Comment && k != Kind::DocComment {
            continue;
        }
        if s.bytes(i).windows(2).any(|w| w == b"\r\n") {
            let v: Vec<u8> = {
                let b = s.bytes(i);
                let mut out = Vec::with_capacity(b.len());
                let mut j = 0;
                while j < b.len() {
                    if j + 1 < b.len() && b[j] == b'\r' && b[j + 1] == b'\n' {
                        out.push(b'\n');
                        j += 2;
                    } else {
                        out.push(b[j]);
                        j += 1;
                    }
                }
                out
            };
            s.set_owned(i, v);
            changed = true;
        }
    }
    changed
}

fn object_operator_without_whitespace(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || (s.bytes(i) != b"->" && s.bytes(i) != b"?->") {
            i += 1;
            continue;
        }
        if i + 1 < s.len() && s.kind(i + 1) == Kind::Whitespace && !has_newline(s.bytes(i + 1)) {
            s.remove_at(i + 1);
            changed = true;
        }
        if i > 0 && s.kind(i - 1) == Kind::Whitespace && !has_newline(s.bytes(i - 1)) {
            s.remove_at(i - 1);
            i -= 1;
            changed = true;
        }
        i += 1;
    }
    changed
}

fn standardize_not_equals(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) == Kind::Punct && s.bytes(i) == b"<>" {
            s.set_owned(i, b"!=".to_vec());
            changed = true;
        }
    }
    changed
}

fn no_empty_statement(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut depth: i32 = 0;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct {
            i += 1;
            continue;
        }
        match s.bytes(i) {
            b"(" | b"[" => depth += 1,
            b")" | b"]" => depth -= 1,
            b";" => {
                if depth == 0 {
                    if let Some(p) = prev_significant_index(s, i) {
                        if s.bytes(p) == b";" {
                            s.remove_at(i);
                            i = i.saturating_sub(1);
                            changed = true;
                            continue;
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    changed
}

fn magic_constant_casing(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::Ident {
            continue;
        }
        let lower = s.bytes(i).to_ascii_lowercase();
        let canonical: &[u8] = match lower.as_slice() {
            b"__line__" => b"__LINE__",
            b"__file__" => b"__FILE__",
            b"__dir__" => b"__DIR__",
            b"__function__" => b"__FUNCTION__",
            b"__class__" => b"__CLASS__",
            b"__trait__" => b"__TRAIT__",
            b"__method__" => b"__METHOD__",
            b"__namespace__" => b"__NAMESPACE__",
            b"__compiler_halt_offset__" => b"__COMPILER_HALT_OFFSET__",
            _ => continue,
        };
        if canonical != s.bytes(i) {
            s.set_owned(i, canonical.to_vec());
            changed = true;
        }
    }
    changed
}

fn magic_method_casing(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::Ident {
            continue;
        }
        let lower = s.bytes(i).to_ascii_lowercase();
        let canonical: &[u8] = match lower.as_slice() {
            b"__construct" => b"__construct",
            b"__destruct" => b"__destruct",
            b"__call" => b"__call",
            b"__callstatic" => b"__callStatic",
            b"__get" => b"__get",
            b"__set" => b"__set",
            b"__isset" => b"__isset",
            b"__unset" => b"__unset",
            b"__sleep" => b"__sleep",
            b"__wakeup" => b"__wakeup",
            b"__serialize" => b"__serialize",
            b"__unserialize" => b"__unserialize",
            b"__tostring" => b"__toString",
            b"__invoke" => b"__invoke",
            b"__set_state" => b"__set_state",
            b"__clone" => b"__clone",
            b"__debuginfo" => b"__debugInfo",
            _ => continue,
        };
        if canonical == s.bytes(i) {
            continue;
        }
        // a magic method is always a declaration or call ("__set("); a constant
        // named "__SET" (followed by "=") must not be recased
        if next_significant_value(s, i) != b"(" {
            continue;
        }
        s.set_owned(i, canonical.to_vec());
        changed = true;
    }
    changed
}

fn integer_literal_case(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::Number {
            continue;
        }
        let b = s.bytes(i);
        if b.len() < 2 || b[0] != b'0' {
            continue;
        }
        if matches!(b[1], b'x' | b'X' | b'b' | b'B' | b'o' | b'O') {
            let lower = b.to_ascii_lowercase();
            if lower.as_slice() != s.bytes(i) {
                s.set_owned(i, lower);
                changed = true;
            }
        }
    }
    changed
}

fn native_function_casing(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::Ident {
            continue;
        }
        let lower = s.bytes(i).to_ascii_lowercase();
        if !is_native_function(lower.as_slice()) || lower.as_slice() == s.bytes(i) {
            continue;
        }
        // must be a function call, not a method or a namespaced name
        if let Some(p) = prev_significant_index(s, i) {
            match s.bytes(p) {
                b"->" | b"?->" | b"::" | b"\\" | b"function" => continue,
                _ => {}
            }
        }
        if next_significant_value(s, i) != b"(" {
            continue;
        }
        s.set_owned(i, lower);
        changed = true;
    }
    changed
}

fn class_reference_name_casing(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::Ident {
            continue;
        }
        let lower = s.bytes(i).to_ascii_lowercase();
        let canonical = match builtin_class_name(lower.as_slice()) {
            Some(c) => c,
            None => continue,
        };
        if canonical == s.bytes(i) {
            continue;
        }
        // only a fully-qualified reference "\Name" is unambiguously the global built-in
        let bs = match prev_significant_index(s, i) {
            Some(b) if s.kind(b) == Kind::Punct && s.bytes(b) == b"\\" => b,
            _ => continue,
        };
        // "\Name\..." - the name heads a namespace, not the class itself
        if let Some(n) = next_significant_index(s, i) {
            if s.kind(n) == Kind::Punct && s.bytes(n) == b"\\" {
                continue;
            }
        }
        // "Foo\Name" - part of a namespaced name, not a global built-in
        let before = prev_significant_index(s, bs);
        if let Some(bidx) = before {
            if s.kind(bidx) == Kind::Ident {
                continue;
            }
        }
        if !is_class_reference_position(s, before, i) {
            continue;
        }
        s.set_owned(i, canonical.to_vec());
        changed = true;
    }
    changed
}

fn is_class_reference_position(s: &Stream, before: Option<usize>, name_idx: usize) -> bool {
    let next = next_significant_index(s, name_idx);
    if let (Some(b), Some(n)) = (before, next) {
        if is_block_open_or_comma(s, b) && is_block_close_or_comma(s, n) {
            return false;
        }
    }
    if let Some(b) = before {
        if s.kind(b) == Kind::Keyword && s.bytes(b).eq_ignore_ascii_case(b"new") {
            return true;
        }
    }
    if let Some(n) = next {
        if s.kind(n) == Kind::CloseTag {
            return false;
        }
        match s.bytes(n) {
            b"(" | b";" | b"=" => return false,
            _ => {}
        }
    }
    true
}

fn is_block_open_or_comma(s: &Stream, i: usize) -> bool {
    s.kind(i) == Kind::Punct && matches!(s.bytes(i), b"," | b"(" | b"[" | b"{")
}

fn is_block_close_or_comma(s: &Stream, i: usize) -> bool {
    s.kind(i) == Kind::Punct && matches!(s.bytes(i), b"," | b")" | b"]" | b"}")
}

fn is_native_function(b: &[u8]) -> bool {
    matches!(
        b,
        b"strlen" | b"count" | b"sizeof" | b"is_array" | b"is_string" | b"is_int" | b"is_integer" | b"is_bool" | b"is_null" | b"is_object" | b"is_callable" | b"is_numeric" | b"is_float" | b"is_a" | b"is_iterable" | b"array_map" | b"array_filter" | b"array_merge" | b"array_keys" | b"array_values" | b"array_key_exists" | b"in_array" | b"implode" | b"explode" | b"str_replace" | b"str_repeat" | b"substr" | b"strpos" | b"stripos" | b"strrpos" | b"strtolower" | b"strtoupper" | b"ucfirst" | b"lcfirst" | b"ucwords" | b"trim" | b"ltrim" | b"rtrim" | b"sprintf" | b"printf" | b"vsprintf" | b"number_format" | b"json_encode" | b"json_decode" | b"preg_match" | b"preg_replace" | b"preg_split" | b"preg_match_all" | b"preg_quote" | b"sort" | b"rsort" | b"usort" | b"uasort" | b"uksort" | b"ksort" | b"krsort" | b"asort" | b"arsort" | b"array_push" | b"array_pop" | b"array_shift" | b"array_unshift" | b"array_slice" | b"array_splice" | b"array_reverse" | b"array_unique" | b"array_flip" | b"array_combine" | b"array_column" | b"array_sum" | b"array_product" | b"array_reduce" | b"array_search" | b"array_fill" | b"array_diff" | b"array_intersect" | b"array_pad" | b"array_chunk" | b"array_key_first" | b"array_key_last" | b"max" | b"min" | b"abs" | b"ceil" | b"floor" | b"round" | b"intval" | b"floatval" | b"strval" | b"boolval" | b"gettype" | b"settype" | b"function_exists" | b"method_exists" | b"class_exists" | b"interface_exists" | b"property_exists" | b"defined" | b"define" | b"constant" | b"call_user_func" | b"call_user_func_array" | b"func_get_args" | b"func_num_args" | b"compact" | b"extract" | b"print_r" | b"var_dump" | b"var_export" | b"serialize" | b"unserialize" | b"base64_encode" | b"base64_decode" | b"md5" | b"sha1" | b"hash" | b"dechex" | b"hexdec" | b"date" | b"time" | b"mktime" | b"strtotime" | b"microtime" | b"str_pad" | b"str_split" | b"str_contains" | b"str_starts_with" | b"str_ends_with" | b"wordwrap" | b"nl2br" | b"htmlspecialchars" | b"htmlentities" | b"strip_tags" | b"addslashes" | b"stripslashes" | b"ord" | b"chr" | b"intdiv" | b"fmod" | b"pow" | b"sqrt" | b"rand" | b"mt_rand" | b"random_int" | b"array_is_list"
    )
}

fn builtin_class_name(b: &[u8]) -> Option<&'static [u8]> {
    Some(match b {
        b"stdclass" => b"stdClass",
        b"closure" => b"Closure",
        b"generator" => b"Generator",
        b"fiber" => b"Fiber",
        b"weakmap" => b"WeakMap",
        b"weakreference" => b"WeakReference",
        b"stringable" => b"Stringable",
        b"throwable" => b"Throwable",
        b"traversable" => b"Traversable",
        b"iterator" => b"Iterator",
        b"iteratoraggregate" => b"IteratorAggregate",
        b"arrayaccess" => b"ArrayAccess",
        b"countable" => b"Countable",
        b"jsonserializable" => b"JsonSerializable",
        b"unitenum" => b"UnitEnum",
        b"backedenum" => b"BackedEnum",
        b"attribute" => b"Attribute",
        b"exception" => b"Exception",
        b"errorexception" => b"ErrorException",
        b"error" => b"Error",
        b"typeerror" => b"TypeError",
        b"valueerror" => b"ValueError",
        b"argumentcounterror" => b"ArgumentCountError",
        b"arithmeticerror" => b"ArithmeticError",
        b"divisionbyzeroerror" => b"DivisionByZeroError",
        b"unhandledmatcherror" => b"UnhandledMatchError",
        b"jsonexception" => b"JsonException",
        b"logicexception" => b"LogicException",
        b"badfunctioncallexception" => b"BadFunctionCallException",
        b"badmethodcallexception" => b"BadMethodCallException",
        b"domainexception" => b"DomainException",
        b"invalidargumentexception" => b"InvalidArgumentException",
        b"lengthexception" => b"LengthException",
        b"outofrangeexception" => b"OutOfRangeException",
        b"runtimeexception" => b"RuntimeException",
        b"outofboundsexception" => b"OutOfBoundsException",
        b"overflowexception" => b"OverflowException",
        b"rangeexception" => b"RangeException",
        b"underflowexception" => b"UnderflowException",
        b"unexpectedvalueexception" => b"UnexpectedValueException",
        b"arrayobject" => b"ArrayObject",
        b"arrayiterator" => b"ArrayIterator",
        b"splstack" => b"SplStack",
        b"splqueue" => b"SplQueue",
        b"spldoublylinkedlist" => b"SplDoublyLinkedList",
        b"splfixedarray" => b"SplFixedArray",
        b"splheap" => b"SplHeap",
        b"splminheap" => b"SplMinHeap",
        b"splmaxheap" => b"SplMaxHeap",
        b"splpriorityqueue" => b"SplPriorityQueue",
        b"splobjectstorage" => b"SplObjectStorage",
        b"splfileinfo" => b"SplFileInfo",
        b"splfileobject" => b"SplFileObject",
        b"spltempfileobject" => b"SplTempFileObject",
        b"datetime" => b"DateTime",
        b"datetimeimmutable" => b"DateTimeImmutable",
        b"datetimeinterface" => b"DateTimeInterface",
        b"dateinterval" => b"DateInterval",
        b"dateperiod" => b"DatePeriod",
        b"datetimezone" => b"DateTimeZone",
        b"reflectionclass" => b"ReflectionClass",
        b"reflectionmethod" => b"ReflectionMethod",
        b"reflectionproperty" => b"ReflectionProperty",
        b"reflectionfunction" => b"ReflectionFunction",
        b"reflectionparameter" => b"ReflectionParameter",
        b"reflectionnamedtype" => b"ReflectionNamedType",
        b"reflectionexception" => b"ReflectionException",
        b"reflectionenum" => b"ReflectionEnum",
        _ => return None,
    })
}
