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
    r"PhpCsFixer\Fixer\ArrayNotation\ArraySyntaxFixer",
    r"PhpCsFixer\Fixer\ListNotation\ListSyntaxFixer",
    r"PhpCsFixer\Fixer\ArrayNotation\NoWhitespaceBeforeCommaInArrayFixer",
    r"PhpCsFixer\Fixer\ArrayNotation\WhitespaceAfterCommaInArrayFixer",
    r"PhpCsFixer\Fixer\ControlStructure\TrailingCommaInMultilineFixer",
    r"PhpCsFixer\Fixer\Basic\NoTrailingCommaInSinglelineFixer",
    r"PhpCsFixer\Fixer\Whitespace\NoSpacesAroundOffsetFixer",
    r"PhpCsFixer\Fixer\Operator\ObjectOperatorWithoutWhitespaceFixer",
    r"PhpCsFixer\Fixer\Operator\StandardizeNotEqualsFixer",
    r"PhpCsFixer\Fixer\Operator\TernaryToNullCoalescingFixer",
    r"PhpCsFixer\Fixer\Semicolon\NoEmptyStatementFixer",
    r"PhpCsFixer\Fixer\Comment\NoEmptyCommentFixer",
    r"PhpCsFixer\Fixer\Comment\SingleLineCommentSpacingFixer",
    r"PhpCsFixer\Fixer\StringNotation\SingleQuoteFixer",
    r"PhpCsFixer\Fixer\ArrayNotation\TrimArraySpacesFixer",
    r"PhpCsFixer\Fixer\Operator\NoSpaceAroundDoubleColonFixer",
    r"PhpCsFixer\Fixer\StringNotation\HeredocToNowdocFixer",
    r"PhpCsFixer\Fixer\StringNotation\NoBinaryStringFixer",
    r"PhpCsFixer\Fixer\Operator\NoUselessConcatOperatorFixer",
    r"PhpCsFixer\Fixer\CastNotation\NoShortBoolCastFixer",
    r"PhpCsFixer\Fixer\CastNotation\NoUnsetCastFixer",
    r"PhpCsFixer\Fixer\ArrayNotation\NoWhitespaceInEmptyArrayFixer",
    r"PhpCsFixer\Fixer\ArrayNotation\NormalizeIndexBraceFixer",
    r"PhpCsFixer\Fixer\ArrayNotation\NoMultilineWhitespaceAroundDoubleArrowFixer",
    r"PhpCsFixer\Fixer\Operator\StandardizeIncrementFixer",
    r"PhpCsFixer\Fixer\Operator\LongToShorthandOperatorFixer",
    r"PhpCsFixer\Fixer\Comment\MultilineCommentOpeningClosingFixer",
    r"PhpCsFixer\Fixer\Phpdoc\AlignMultilineCommentFixer",
    r"PhpCsFixer\Fixer\Operator\AssignNullCoalescingToCoalesceEqualFixer",
    r"PhpCsFixer\Fixer\Comment\SingleLineCommentStyleFixer",
    r"PhpCsFixer\Fixer\LanguageConstruct\ExplicitIndirectVariableFixer",
    r"PhpCsFixer\Fixer\StringNotation\ExplicitStringVariableFixer",
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
    changed |= array_syntax(s);
    changed |= list_syntax(s);
    changed |= no_whitespace_before_comma_in_array(s);
    changed |= whitespace_after_comma_in_array(s);
    changed |= trailing_comma_in_multiline(s);
    changed |= no_trailing_comma_in_singleline(s);
    changed |= no_spaces_around_offset(s);
    changed |= object_operator_without_whitespace(s);
    changed |= standardize_not_equals(s);
    changed |= ternary_to_null_coalescing(s);
    changed |= no_empty_statement(s);
    changed |= no_empty_comment(s);
    changed |= single_line_comment_spacing(s);
    changed |= single_quote(s);
    changed |= trim_array_spaces(s);
    changed |= no_space_around_double_colon(s);
    changed |= heredoc_to_nowdoc(s);
    changed |= no_binary_string(s);
    changed |= no_useless_concat_operator(s);
    changed |= no_short_bool_cast(s);
    changed |= no_unset_cast(s);
    changed |= no_whitespace_in_empty_array(s);
    changed |= normalize_index_brace(s);
    changed |= no_multiline_whitespace_around_double_arrow(s);
    changed |= standardize_increment(s);
    changed |= long_to_shorthand_operator(s);
    changed |= multiline_comment_opening_closing(s);
    changed |= align_multiline_comment(s);
    changed |= assign_null_coalescing_to_coalesce_equal(s);
    changed |= single_line_comment_style(s);
    changed |= explicit_indirect_variable(s);
    changed |= explicit_string_variable(s);
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

// --- ported batch 2: arrays / offset / trailing-comma ----------------------

fn top_bracket_is_array(stack: &[u8]) -> bool {
    stack.last() == Some(&b'[')
}

fn is_offset_open(s: &Stream, open: usize) -> bool {
    match prev_significant_index(s, open) {
        None => false,
        Some(p) => {
            let k = s.kind(p);
            if k == Kind::Variable || k == Kind::Ident || k == Kind::String {
                return true;
            }
            s.bytes(p) == b")" || s.bytes(p) == b"]"
        }
    }
}

fn is_index_brace_target(s: &Stream, i: usize) -> bool {
    let p = match prev_significant_index(s, i) {
        Some(p) => p,
        None => return false,
    };
    match s.kind(p) {
        Kind::Variable => s.bytes(p) != b"$",
        Kind::String => true,
        Kind::Punct => s.bytes(p) == b"]",
        _ => false,
    }
}

fn is_comment_tok(s: &Stream, i: usize) -> bool {
    matches!(s.kind(i), Kind::Comment | Kind::DocComment)
}

fn is_line_comment(s: &Stream, i: usize) -> bool {
    s.kind(i) == Kind::Comment && (s.bytes(i).starts_with(b"//") || s.bytes(i).starts_with(b"#"))
}

fn collapse_arrow_whitespace(s: &mut Stream, i: isize) -> bool {
    if i < 0 || i as usize >= s.len() {
        return false;
    }
    let i = i as usize;
    if s.kind(i) != Kind::Whitespace || !has_newline(s.bytes(i)) {
        return false;
    }
    s.set_owned(i, b" ".to_vec());
    true
}

fn convert_long_array(s: &mut Stream, name: &[u8]) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        let k = s.kind(i);
        if (k != Kind::Keyword && k != Kind::Ident) || !s.bytes(i).eq_ignore_ascii_case(name) {
            i += 1;
            continue;
        }
        if let Some(p) = prev_significant_index(s, i) {
            match s.bytes(p) {
                b"->" | b"?->" | b"::" => {
                    i += 1;
                    continue;
                }
                _ => {}
            }
        }
        let j = skip_ws(s, i + 1);
        if j >= s.len() || s.kind(j) != Kind::Punct || s.bytes(j) != b"(" {
            i += 1;
            continue;
        }
        let close_idx = match match_forward(s, j) {
            Some(c) => c,
            None => {
                i += 1;
                continue;
            }
        };
        s.set_owned(close_idx, b"]".to_vec());
        s.set_owned(j, b"[".to_vec());
        let mut kk = j as isize - 1;
        while kk >= i as isize {
            s.remove_at(kk as usize);
            kk -= 1;
        }
        changed = true;
        i += 1;
    }
    changed
}

fn array_syntax(s: &mut Stream) -> bool {
    convert_long_array(s, b"array")
}

fn list_syntax(s: &mut Stream) -> bool {
    convert_long_array(s, b"list")
}

fn no_whitespace_before_comma_in_array(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut stack: Vec<u8> = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Punct {
            match s.bytes(i) {
                b"(" | b"[" | b"{" => stack.push(s.bytes(i)[0]),
                b")" | b"]" | b"}" => {
                    stack.pop();
                }
                b"," => {
                    if top_bracket_is_array(&stack)
                        && i > 0
                        && s.kind(i - 1) == Kind::Whitespace
                        && !has_newline(s.bytes(i - 1))
                    {
                        s.remove_at(i - 1);
                        changed = true;
                        i -= 1;
                        continue;
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    changed
}

fn whitespace_after_comma_in_array(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut stack: Vec<u8> = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct {
            i += 1;
            continue;
        }
        match s.bytes(i) {
            b"(" | b"[" | b"{" => stack.push(s.bytes(i)[0]),
            b")" | b"]" | b"}" => {
                stack.pop();
            }
            b"," => {
                if !top_bracket_is_array(&stack) || i + 1 >= s.len() {
                    i += 1;
                    continue;
                }
                if s.kind(i + 1) == Kind::Whitespace {
                    i += 1;
                    continue;
                }
                if s.kind(i + 1) == Kind::Punct && s.bytes(i + 1) == b"]" {
                    i += 1;
                    continue;
                }
                s.insert_owned(i + 1, Kind::Whitespace, b" ".to_vec());
                changed = true;
                i += 2;
                continue;
            }
            _ => {}
        }
        i += 1;
    }
    changed
}

fn trailing_comma_in_multiline(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"]" {
            i += 1;
            continue;
        }
        let open = match match_backward(s, i) {
            Some(o) if !is_offset_open(s, o) => o,
            _ => {
                i += 1;
                continue;
            }
        };
        let mut p = i - 1;
        while p > open && matches!(s.kind(p), Kind::Whitespace | Kind::Comment | Kind::DocComment) {
            p -= 1;
        }
        if p <= open {
            i += 1;
            continue;
        }
        if s.bytes(p) == b"," || s.bytes(p) == b"..." {
            i += 1;
            continue;
        }
        let mut multiline = false;
        let mut kk = p + 1;
        while kk < i {
            if s.kind(kk) == Kind::Whitespace && has_newline(s.bytes(kk)) {
                multiline = true;
                break;
            }
            kk += 1;
        }
        if !multiline {
            i += 1;
            continue;
        }
        s.insert_owned(p + 1, Kind::Punct, b",".to_vec());
        changed = true;
        i += 1;
    }
    changed
}

fn no_trailing_comma_in_singleline(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"," {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        let mut broke = false;
        while j < s.len() && s.kind(j) == Kind::Whitespace {
            if has_newline(s.bytes(j)) {
                broke = true;
                break;
            }
            j += 1;
        }
        if broke || j >= s.len() {
            i += 1;
            continue;
        }
        let v = s.bytes(j);
        if v == b")" || v == b"]" || v == b"}" {
            s.remove_at(i);
            changed = true;
            continue;
        }
        i += 1;
    }
    changed
}

fn no_spaces_around_offset(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"[" || !is_offset_open(s, i) {
            i += 1;
            continue;
        }
        let close_idx = match match_forward(s, i) {
            Some(c) => c,
            None => {
                i += 1;
                continue;
            }
        };
        if close_idx > i + 1
            && s.kind(close_idx - 1) == Kind::Whitespace
            && !has_newline(s.bytes(close_idx - 1))
        {
            s.remove_at(close_idx - 1);
            changed = true;
        }
        if i + 1 < s.len() && s.kind(i + 1) == Kind::Whitespace && !has_newline(s.bytes(i + 1)) {
            s.remove_at(i + 1);
            changed = true;
        }
        i += 1;
    }
    changed
}

fn trim_array_spaces(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"[" || is_offset_open(s, i) {
            i += 1;
            continue;
        }
        let close_idx = match match_forward(s, i) {
            Some(c) => c,
            None => {
                i += 1;
                continue;
            }
        };
        if close_idx > i + 1
            && s.kind(close_idx - 1) == Kind::Whitespace
            && !has_newline(s.bytes(close_idx - 1))
        {
            s.remove_at(close_idx - 1);
            changed = true;
        }
        if i + 1 < s.len() && s.kind(i + 1) == Kind::Whitespace && !has_newline(s.bytes(i + 1)) {
            s.remove_at(i + 1);
            changed = true;
        }
        i += 1;
    }
    changed
}

fn no_whitespace_in_empty_array(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i + 2 < s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"[" {
            i += 1;
            continue;
        }
        if s.kind(i + 1) != Kind::Whitespace {
            i += 1;
            continue;
        }
        if s.kind(i + 2) != Kind::Punct || s.bytes(i + 2) != b"]" {
            i += 1;
            continue;
        }
        s.remove_at(i + 1);
        changed = true;
        i += 1;
    }
    changed
}

fn normalize_index_brace(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"{" || !is_index_brace_target(s, i) {
            i += 1;
            continue;
        }
        let close_idx = match match_forward(s, i) {
            Some(c) => c,
            None => {
                i += 1;
                continue;
            }
        };
        s.set_owned(close_idx, b"]".to_vec());
        s.set_owned(i, b"[".to_vec());
        changed = true;
        i += 1;
    }
    changed
}

fn no_multiline_whitespace_around_double_arrow(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"=>" {
            i += 1;
            continue;
        }
        if i < 2 || !is_line_comment(s, i - 2) {
            changed = collapse_arrow_whitespace(s, i as isize - 1) || changed;
        }
        if i + 2 >= s.len() || !is_comment_tok(s, i + 2) {
            changed = collapse_arrow_whitespace(s, i as isize + 1) || changed;
        }
        i += 1;
    }
    changed
}

// --- ported batch 3: operators / casts -------------------------------------

fn sig_prev(s: &Stream, i: usize) -> Option<usize> {
    let mut j = i as isize - 1;
    while j >= 0 {
        match s.kind(j as usize) {
            Kind::Whitespace | Kind::Comment | Kind::DocComment => {}
            _ => return Some(j as usize),
        }
        j -= 1;
    }
    None
}

fn sig_next(s: &Stream, i: usize) -> Option<usize> {
    let mut j = i + 1;
    while j < s.len() {
        match s.kind(j) {
            Kind::Whitespace | Kind::Comment | Kind::DocComment => {}
            _ => return Some(j),
        }
        j += 1;
    }
    None
}

fn span_clean(s: &Stream, a: usize, b: usize) -> bool {
    let mut j = a;
    while j <= b && j < s.len() {
        if matches!(s.kind(j), Kind::Comment | Kind::DocComment) {
            return false;
        }
        j += 1;
    }
    true
}

fn lvalue_prefix(v: &[u8]) -> bool {
    matches!(v, b"->" | b"?->" | b"::" | b"$" | b"&")
}

fn short_op(v: &[u8]) -> bool {
    matches!(v, b"+" | b"-" | b"*" | b"/" | b"." | b"%" | b"&" | b"|" | b"^")
}

fn short_operand(s: &Stream, i: usize) -> bool {
    matches!(s.kind(i), Kind::Variable | Kind::Number | Kind::String | Kind::Ident)
}

fn inc_expr_end(v: &[u8]) -> bool {
    matches!(v, b";" | b")" | b"]" | b"," | b":")
}

fn is_bang(s: &Stream, i: usize) -> bool {
    s.kind(i) == Kind::Punct && s.bytes(i) == b"!"
}

fn string_quote(v: &[u8]) -> u8 {
    if v.len() >= 2 && (v[0] == b'\'' || v[0] == b'"') && v[v.len() - 1] == v[0] {
        v[0]
    } else {
        0
    }
}

fn range_has_newline(s: &Stream, lo: usize, hi: usize) -> bool {
    let mut j = lo + 1;
    while j < hi {
        if s.kind(j) == Kind::Whitespace && has_newline(s.bytes(j)) {
            return true;
        }
        j += 1;
    }
    false
}

fn replace_range(s: &mut Stream, lo: usize, hi: usize, repl: Vec<(Kind, Vec<u8>)>) {
    let mut k = hi;
    loop {
        s.remove_at(k);
        if k == lo {
            break;
        }
        k -= 1;
    }
    for (off, (kind, bytes)) in repl.into_iter().enumerate() {
        s.insert_owned(lo + off, kind, bytes);
    }
}

fn ternary_colon(s: &Stream, from: usize) -> Option<usize> {
    let mut depth = 0i32;
    let mut tern = 0i32;
    let mut j = from;
    while j < s.len() {
        if s.kind(j) == Kind::Punct {
            match s.bytes(j) {
                b"(" | b"[" | b"{" => depth += 1,
                b")" | b"]" | b"}" => {
                    if depth == 0 {
                        return None;
                    }
                    depth -= 1;
                }
                b"?" => {
                    if depth == 0 {
                        tern += 1;
                    }
                }
                b":" => {
                    if depth == 0 {
                        if tern == 0 {
                            return Some(j);
                        }
                        tern -= 1;
                    }
                }
                b";" => {
                    if depth == 0 {
                        return None;
                    }
                }
                _ => {}
            }
        }
        j += 1;
    }
    None
}

fn has_top_level_comma(s: &Stream, lo: usize, hi: usize) -> bool {
    let mut depth = 0i32;
    let mut j = lo;
    while j <= hi && j < s.len() {
        if s.kind(j) == Kind::Punct {
            match s.bytes(j) {
                b"(" | b"[" | b"{" => depth += 1,
                b")" | b"]" | b"}" => depth -= 1,
                b"," => {
                    if depth == 0 {
                        return true;
                    }
                }
                _ => {}
            }
        }
        j += 1;
    }
    false
}

fn sig_slice(s: &Stream, lo: usize, hi: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut j = lo;
    while j <= hi && j < s.len() {
        if !matches!(s.kind(j), Kind::Whitespace | Kind::Comment | Kind::DocComment) {
            out.push(j);
        }
        j += 1;
    }
    out
}

fn tokens_equal_at(s: &Stream, a: &[usize], b: &[usize]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for k in 0..a.len() {
        if s.kind(a[k]) != s.kind(b[k]) || s.bytes(a[k]) != s.bytes(b[k]) {
            return false;
        }
    }
    true
}

fn trimmed_copy(s: &Stream, mut lo: usize, mut hi: usize) -> Vec<(Kind, Vec<u8>)> {
    while lo <= hi && s.kind(lo) == Kind::Whitespace {
        lo += 1;
    }
    while hi >= lo && s.kind(hi) == Kind::Whitespace {
        if hi == 0 {
            break;
        }
        hi -= 1;
    }
    let mut out = Vec::new();
    let mut j = lo;
    while j <= hi {
        out.push((s.kind(j), s.bytes(j).to_vec()));
        j += 1;
    }
    out
}

fn ternary_to_null_coalescing(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Keyword || !s.bytes(i).eq_ignore_ascii_case(b"isset") {
            i += 1;
            continue;
        }
        let b = match sig_next(s, i) {
            Some(b) if s.kind(b) == Kind::Punct && s.bytes(b) == b"(" => b,
            _ => {
                i += 1;
                continue;
            }
        };
        let c = match match_forward(s, b) {
            Some(c) => c,
            None => {
                i += 1;
                continue;
            }
        };
        let d = match sig_next(s, c) {
            Some(d) if s.kind(d) == Kind::Punct && s.bytes(d) == b"?" => d,
            _ => {
                i += 1;
                continue;
            }
        };
        let e = match ternary_colon(s, d + 1) {
            Some(e) if sig_next(s, e).is_some() => e,
            _ => {
                i += 1;
                continue;
            }
        };
        if !span_clean(s, i, e) {
            i += 1;
            continue;
        }
        if has_top_level_comma(s, b + 1, c - 1) {
            i += 1;
            continue;
        }
        let isset_toks = sig_slice(s, b + 1, c - 1);
        let true_toks = sig_slice(s, d + 1, e - 1);
        if isset_toks.is_empty() || !tokens_equal_at(s, &isset_toks, &true_toks) {
            i += 1;
            continue;
        }
        let mut repl = trimmed_copy(s, b + 1, c - 1);
        repl.push((Kind::Whitespace, b" ".to_vec()));
        repl.push((Kind::Punct, b"??".to_vec()));
        repl.push((Kind::Whitespace, b" ".to_vec()));
        let mut end = e;
        if e + 1 < s.len() && s.kind(e + 1) == Kind::Whitespace {
            end = e + 1;
        }
        replace_range(s, i, end, repl);
        changed = true;
        i += 1;
    }
    changed
}

fn no_space_around_double_colon(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"::" {
            i += 1;
            continue;
        }
        if i + 1 < s.len() && s.kind(i + 1) == Kind::Whitespace && !has_newline(s.bytes(i + 1)) {
            s.remove_at(i + 1);
            changed = true;
        }
        if i >= 1 && s.kind(i - 1) == Kind::Whitespace && !has_newline(s.bytes(i - 1)) {
            s.remove_at(i - 1);
            i -= 1;
            changed = true;
        }
        i += 1;
    }
    changed
}

fn no_useless_concat_operator(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::String {
            i += 1;
            continue;
        }
        let q = string_quote(s.bytes(i));
        if q == 0 || (q == b'"' && s.bytes(i).contains(&b'$')) {
            i += 1;
            continue;
        }
        let j = match next_significant_index(s, i) {
            Some(j) if s.kind(j) == Kind::Punct && s.bytes(j) == b"." => j,
            _ => {
                i += 1;
                continue;
            }
        };
        let k = match next_significant_index(s, j) {
            Some(k) if s.kind(k) == Kind::String => k,
            _ => {
                i += 1;
                continue;
            }
        };
        let next = s.bytes(k).to_vec();
        if string_quote(&next) != q || (q == b'"' && next.contains(&b'$')) {
            i += 1;
            continue;
        }
        if range_has_newline(s, i, k) {
            i += 1;
            continue;
        }
        let cur = s.bytes(i);
        let mut merged = cur[..cur.len() - 1].to_vec();
        merged.extend_from_slice(&next[1..]);
        s.set_owned(i, merged);
        let mut idx = k;
        while idx > i {
            s.remove_at(idx);
            idx -= 1;
        }
        changed = true;
    }
    changed
}

fn no_short_bool_cast(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = s.len();
    while i > 1 {
        i -= 1;
        if !is_bang(s, i) {
            continue;
        }
        let j = match prev_significant_index(s, i) {
            Some(j) if is_bang(s, j) => j,
            _ => continue,
        };
        replace_range(
            s,
            j,
            i,
            vec![
                (Kind::Punct, b"(".to_vec()),
                (Kind::Ident, b"bool".to_vec()),
                (Kind::Punct, b")".to_vec()),
            ],
        );
        let after = j + 3;
        if after < s.len() && s.kind(after) != Kind::Whitespace {
            s.insert_owned(after, Kind::Whitespace, b" ".to_vec());
        }
        changed = true;
        i = j;
    }
    changed
}

fn no_unset_cast(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        let (type_idx, close_idx) = match cast_at(s, i) {
            Some(t) => t,
            None => {
                i += 1;
                continue;
            }
        };
        if !s.bytes(type_idx).eq_ignore_ascii_case(b"unset") {
            i += 1;
            continue;
        }
        let assign_idx = match prev_significant_index(s, i) {
            Some(a) if s.kind(a) == Kind::Punct && s.bytes(a) == b"=" => a,
            _ => {
                i += 1;
                continue;
            }
        };
        let var_idx = match next_significant_index(s, close_idx) {
            Some(v) if s.kind(v) == Kind::Variable => v,
            _ => {
                i += 1;
                continue;
            }
        };
        let after_var = match next_significant_index(s, var_idx) {
            Some(a) => a,
            None => {
                i += 1;
                continue;
            }
        };
        if s.kind(after_var) != Kind::CloseTag
            && (s.kind(after_var) != Kind::Punct || s.bytes(after_var) != b";")
        {
            i += 1;
            continue;
        }
        let mut repl = vec![(Kind::Ident, b"null".to_vec())];
        if s.kind(assign_idx + 1) != Kind::Whitespace {
            repl.insert(0, (Kind::Whitespace, b" ".to_vec()));
        }
        replace_range(s, i, var_idx, repl);
        changed = true;
        i += 1;
    }
    changed
}

fn standardize_increment(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || (s.bytes(i) != b"+=" && s.bytes(i) != b"-=") {
            i += 1;
            continue;
        }
        let lv = match sig_prev(s, i) {
            Some(lv) if s.kind(lv) == Kind::Variable => lv,
            _ => {
                i += 1;
                continue;
            }
        };
        if let Some(p) = sig_prev(s, lv) {
            if lvalue_prefix(s.bytes(p)) {
                i += 1;
                continue;
            }
        }
        let num_idx = match sig_next(s, i) {
            Some(n) if s.kind(n) == Kind::Number && s.bytes(n) == b"1" => n,
            _ => {
                i += 1;
                continue;
            }
        };
        let end_idx = match sig_next(s, num_idx) {
            Some(e) if s.kind(e) == Kind::Punct && inc_expr_end(s.bytes(e)) => e,
            _ => {
                i += 1;
                continue;
            }
        };
        let _ = end_idx;
        if !span_clean(s, lv, num_idx) {
            i += 1;
            continue;
        }
        let op: &[u8] = if s.bytes(i) == b"-=" { b"--" } else { b"++" };
        let op = op.to_vec();
        let mut k = num_idx;
        loop {
            s.remove_at(k);
            if k == i {
                break;
            }
            k -= 1;
        }
        if lv + 1 < s.len() && s.kind(lv + 1) == Kind::Whitespace && !has_newline(s.bytes(lv + 1)) {
            s.remove_at(lv + 1);
        }
        s.insert_owned(lv, Kind::Punct, op);
        changed = true;
        i = lv;
        i += 1;
    }
    changed
}

fn long_to_shorthand_operator(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"=" {
            i += 1;
            continue;
        }
        let lv = match sig_prev(s, i) {
            Some(lv) if s.kind(lv) == Kind::Variable => lv,
            _ => {
                i += 1;
                continue;
            }
        };
        if let Some(p) = sig_prev(s, lv) {
            if lvalue_prefix(s.bytes(p)) {
                i += 1;
                continue;
            }
        }
        let rv = match sig_next(s, i) {
            Some(rv) if s.kind(rv) == Kind::Variable && s.bytes(rv) == s.bytes(lv) => rv,
            _ => {
                i += 1;
                continue;
            }
        };
        let op_idx = match sig_next(s, rv) {
            Some(o) if s.kind(o) == Kind::Punct && short_op(s.bytes(o)) => o,
            _ => {
                i += 1;
                continue;
            }
        };
        let operand_idx = match sig_next(s, op_idx) {
            Some(o) if short_operand(s, o) => o,
            _ => {
                i += 1;
                continue;
            }
        };
        match sig_next(s, operand_idx) {
            Some(e) if s.bytes(e) == b";" => {}
            _ => {
                i += 1;
                continue;
            }
        }
        if !span_clean(s, i, op_idx) {
            i += 1;
            continue;
        }
        let mut new_op = s.bytes(op_idx).to_vec();
        new_op.push(b'=');
        s.set_owned(i, new_op);
        let mut k = op_idx;
        while k >= i + 1 {
            s.remove_at(k);
            k -= 1;
        }
        changed = true;
        i += 1;
    }
    changed
}

fn assign_null_coalescing_to_coalesce_equal(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"=" {
            i += 1;
            continue;
        }
        let lv = match sig_prev(s, i) {
            Some(lv) if s.kind(lv) == Kind::Variable => lv,
            _ => {
                i += 1;
                continue;
            }
        };
        if let Some(p) = sig_prev(s, lv) {
            if lvalue_prefix(s.bytes(p)) {
                i += 1;
                continue;
            }
        }
        let rv = match sig_next(s, i) {
            Some(rv) if s.kind(rv) == Kind::Variable && s.bytes(rv) == s.bytes(lv) => rv,
            _ => {
                i += 1;
                continue;
            }
        };
        let op_idx = match sig_next(s, rv) {
            Some(o) if s.kind(o) == Kind::Punct && s.bytes(o) == b"??" => o,
            _ => {
                i += 1;
                continue;
            }
        };
        let operand_idx = match sig_next(s, op_idx) {
            Some(o) if short_operand(s, o) => o,
            _ => {
                i += 1;
                continue;
            }
        };
        match sig_next(s, operand_idx) {
            Some(e) if s.bytes(e) == b";" => {}
            _ => {
                i += 1;
                continue;
            }
        }
        if !span_clean(s, i, op_idx) {
            i += 1;
            continue;
        }
        s.set_owned(i, b"??=".to_vec());
        let mut k = op_idx;
        while k >= i + 1 {
            s.remove_at(k);
            k -= 1;
        }
        changed = true;
        i += 1;
    }
    changed
}

// --- ported batch 4: string / comment rules --------------------------------

fn is_go_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

fn trim_go_space(b: &[u8]) -> &[u8] {
    let mut st = 0;
    let mut en = b.len();
    while st < en && is_go_space(b[st]) {
        st += 1;
    }
    while en > st && is_go_space(b[en - 1]) {
        en -= 1;
    }
    &b[st..en]
}

fn trim_set<'a>(b: &'a [u8], set: &[u8]) -> &'a [u8] {
    let mut st = 0;
    let mut en = b.len();
    while st < en && set.contains(&b[st]) {
        st += 1;
    }
    while en > st && set.contains(&b[en - 1]) {
        en -= 1;
    }
    &b[st..en]
}

fn contains_subslice(hay: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty() && hay.windows(needle.len()).any(|w| w == needle)
}

fn comment_body(v: &[u8]) -> &[u8] {
    if v.starts_with(b"//") {
        trim_go_space(&v[2..])
    } else if v.starts_with(b"#") {
        trim_go_space(&v[1..])
    } else if v.starts_with(b"/*") {
        let mut x = &v[2..];
        if x.ends_with(b"*/") {
            x = &x[..x.len() - 2];
        }
        trim_go_space(x)
    } else {
        trim_go_space(v)
    }
}

fn no_empty_comment(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Comment
            && !s.bytes(i).starts_with(b"#[")
            && comment_body(s.bytes(i)).is_empty()
        {
            s.remove_at(i);
            changed = true;
            if i >= 1
                && i < s.len()
                && s.kind(i - 1) == Kind::Whitespace
                && s.kind(i) == Kind::Whitespace
            {
                let mut merged = s.bytes(i - 1).to_vec();
                merged.extend_from_slice(s.bytes(i));
                s.set_owned(i - 1, merged);
                s.remove_at(i);
            }
            continue;
        }
        i += 1;
    }
    changed
}

fn single_line_comment_spacing(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::Comment {
            continue;
        }
        let v = s.bytes(i);
        let marker: &[u8] = if v.starts_with(b"//") {
            b"//"
        } else if v.starts_with(b"#") && !v.starts_with(b"#[") {
            b"#"
        } else {
            continue;
        };
        let rest = &v[marker.len()..];
        if rest.is_empty() || rest[0] == b' ' || rest[0] == b'\t' {
            continue;
        }
        let mut nv = Vec::with_capacity(v.len() + 1);
        nv.extend_from_slice(marker);
        nv.push(b' ');
        nv.extend_from_slice(rest);
        s.set_owned(i, nv);
        changed = true;
    }
    changed
}

fn single_quote(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::String {
            continue;
        }
        let v = s.bytes(i);
        if v.len() < 2 {
            continue;
        }
        if v[0] != b'"' || v[v.len() - 1] != b'"' {
            continue;
        }
        let content = v[1..v.len() - 1].to_vec();
        if content.iter().any(|&c| c == b'$' || c == b'\\' || c == b'\'') {
            continue;
        }
        let mut nv = Vec::with_capacity(content.len() + 2);
        nv.push(b'\'');
        nv.extend_from_slice(&content);
        nv.push(b'\'');
        s.set_owned(i, nv);
        changed = true;
    }
    changed
}

fn heredoc_is_label_byte(c: u8) -> bool {
    c == b'_' || c.is_ascii_alphanumeric()
}

fn heredoc_to_nowdoc_value(v: &[u8]) -> Option<Vec<u8>> {
    if !v.starts_with(b"<<<") {
        return None;
    }
    let mut i = 3;
    while i < v.len() && (v[i] == b' ' || v[i] == b'\t') {
        i += 1;
    }
    if i < v.len() && (v[i] == b'\'' || v[i] == b'"') {
        return None;
    }
    let label_start = i;
    while i < v.len() && heredoc_is_label_byte(v[i]) {
        i += 1;
    }
    if i == label_start {
        return None;
    }
    let label_end = i;
    let nl = v.iter().position(|&c| c == b'\n')?;
    let body = &v[nl + 1..];
    if body.iter().any(|&c| c == b'$' || c == b'\\') || body.contains(&b'{') {
        return None;
    }
    let mut out = Vec::with_capacity(v.len() + 2);
    out.extend_from_slice(&v[..label_start]);
    out.push(b'\'');
    out.extend_from_slice(&v[label_start..label_end]);
    out.push(b'\'');
    out.extend_from_slice(&v[label_end..]);
    Some(out)
}

fn heredoc_to_nowdoc(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::String || !s.bytes(i).starts_with(b"<<<") {
            continue;
        }
        if let Some(out) = heredoc_to_nowdoc_value(s.bytes(i)) {
            s.set_owned(i, out);
            changed = true;
        }
    }
    changed
}

fn no_binary_string(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if !(s.kind(i) == Kind::Ident && (s.bytes(i) == b"b" || s.bytes(i) == b"B")) {
            i += 1;
            continue;
        }
        if i + 1 >= s.len() || s.kind(i + 1) != Kind::String {
            i += 1;
            continue;
        }
        let skip = if let Some(p) = prev_significant_index(s, i) {
            matches!(s.bytes(p), b"->" | b"?->" | b"::" | b"\\")
        } else {
            false
        };
        if skip {
            i += 1;
            continue;
        }
        s.remove_at(i);
        changed = true;
        continue;
    }
    changed
}

fn is_doc_block_opening(v: &[u8]) -> bool {
    v.len() >= 4 && &v[..3] == b"/**" && v[3] != b'*' && v[3] != b'/'
}

fn fix_comment_opening(v: &[u8]) -> Vec<u8> {
    if !v.starts_with(b"/*") {
        return v.to_vec();
    }
    let mut a = 0;
    while 1 + a < v.len() && v[1 + a] == b'*' {
        a += 1;
    }
    if a < 2 {
        return v.to_vec();
    }
    let next = if 1 + a < v.len() { v[1 + a] } else { 0 };
    if next != b'/' {
        let mut out = b"/*".to_vec();
        out.extend_from_slice(&v[1 + a..]);
        return out;
    }
    if a >= 3 {
        let mut out = b"/*".to_vec();
        out.extend_from_slice(&v[a..]);
        return out;
    }
    v.to_vec()
}

fn fix_comment_closing(v: &[u8]) -> Vec<u8> {
    if !v.ends_with(b"/") {
        return v.to_vec();
    }
    let mut k: isize = v.len() as isize - 2;
    let mut b = 0;
    while k >= 0 && v[k as usize] == b'*' {
        b += 1;
        k -= 1;
    }
    if b < 2 {
        return v.to_vec();
    }
    let prev = if k >= 0 { v[k as usize] } else { 0 };
    if prev != b'/' {
        let mut out = v[..(k + 1) as usize].to_vec();
        out.extend_from_slice(b"*/");
        return out;
    }
    if b >= 3 {
        let mut out = v[..(k + 2) as usize].to_vec();
        out.extend_from_slice(b"*/");
        return out;
    }
    v.to_vec()
}

fn multiline_comment_opening_closing(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        let k = s.kind(i);
        if k != Kind::Comment && k != Kind::DocComment {
            continue;
        }
        let v = s.bytes(i).to_vec();
        if !v.starts_with(b"/*") {
            continue;
        }
        let mut nv = v.clone();
        if !is_doc_block_opening(&nv) {
            nv = fix_comment_opening(&nv);
        }
        nv = fix_comment_closing(&nv);
        if nv != v {
            s.set_owned(i, nv);
            changed = true;
        }
    }
    changed
}

fn comment_opening_indent(s: &Stream, i: usize) -> Option<Vec<u8>> {
    let whitespace: &[u8] = if i >= 1 && s.kind(i - 1) == Kind::Whitespace {
        s.bytes(i - 1)
    } else {
        b""
    };
    let mut cut: isize = -1;
    for j in 0..whitespace.len() {
        if whitespace[j] == b'\n' || whitespace[j] == b'\r' {
            cut = j as isize;
        }
    }
    if cut < 0 {
        return None;
    }
    Some(whitespace[(cut as usize) + 1..].to_vec())
}

fn realign_comment_lines(v: &[u8], indent: &[u8]) -> Vec<u8> {
    let mut lines: Vec<Vec<u8>> = v.split(|&c| c == b'\n').map(|l| l.to_vec()).collect();
    for k in 1..lines.len() {
        let line = &lines[k];
        let mut st = 0;
        while st < line.len() && (line[st] == b' ' || line[st] == b'\t') {
            st += 1;
        }
        let trimmed = &line[st..];
        if !trimmed.starts_with(b"*") {
            continue;
        }
        let mut nl = indent.to_vec();
        nl.push(b' ');
        nl.extend_from_slice(trimmed);
        lines[k] = nl;
    }
    let mut out = Vec::with_capacity(v.len());
    for (idx, l) in lines.iter().enumerate() {
        if idx > 0 {
            out.push(b'\n');
        }
        out.extend_from_slice(l);
    }
    out
}

fn align_multiline_comment(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        let k = s.kind(i);
        if k != Kind::Comment && k != Kind::DocComment {
            continue;
        }
        if !s.bytes(i).contains(&b'\n') {
            continue;
        }
        let indent = match comment_opening_indent(s, i) {
            Some(x) => x,
            None => continue,
        };
        let v = s.bytes(i).to_vec();
        let nv = realign_comment_lines(&v, &indent);
        if nv != v {
            s.set_owned(i, nv);
            changed = true;
        }
    }
    changed
}

fn single_line_comment_style(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::Comment {
            continue;
        }
        let v = s.bytes(i).to_vec();

        if v.starts_with(b"#") {
            if v.len() >= 2 && v[1] == b'[' {
                continue;
            }
            let mut nv = b"//".to_vec();
            nv.extend_from_slice(&v[1..]);
            s.set_owned(i, nv);
            changed = true;
            continue;
        }

        if !v.starts_with(b"/*") || v.len() < 4 {
            continue;
        }
        if v.iter().any(|&c| c == b'\n' || c == b'\r') {
            continue;
        }
        let content = &v[2..v.len() - 2];
        if contains_subslice(content, b"?>") {
            continue;
        }
        if i + 1 < s.len()
            && (s.kind(i + 1) != Kind::Whitespace
                || !s.bytes(i + 1).iter().any(|&c| c == b'\n' || c == b'\r'))
        {
            continue;
        }

        let inner = trim_set(content, b" \t\r\n\x0c\x0b*");
        let mut nv = b"//".to_vec();
        if !inner.is_empty() {
            nv = b"// ".to_vec();
            nv.extend_from_slice(inner);
        }
        s.set_owned(i, nv);
        changed = true;

        if i + 1 < s.len() {
            let nt = s.bytes(i + 1);
            let mut st = 0;
            while st < nt.len() && (nt[st] == b' ' || nt[st] == b'\t') {
                st += 1;
            }
            if st > 0 {
                let trimmed = nt[st..].to_vec();
                s.set_owned(i + 1, trimmed);
            }
        }
    }
    changed
}

fn interp_is_name_start(c: u8) -> bool {
    c == b'_' || c.is_ascii_lowercase() || c.is_ascii_uppercase() || c >= 0x80
}

fn interp_is_name_byte(c: u8) -> bool {
    interp_is_name_start(c) || c.is_ascii_digit()
}

fn quote_index(idx: &[u8]) -> Vec<u8> {
    if idx.is_empty() {
        return idx.to_vec();
    }
    if idx[0] == b'$' {
        return idx.to_vec();
    }
    let mut numeric = true;
    let mut ss = idx;
    if ss[0] == b'-' {
        ss = &ss[1..];
    }
    if ss.is_empty() {
        numeric = false;
    }
    for &c in ss {
        if !c.is_ascii_digit() {
            numeric = false;
            break;
        }
    }
    if numeric {
        return idx.to_vec();
    }
    let mut out = Vec::with_capacity(idx.len() + 2);
    out.push(b'\'');
    out.extend_from_slice(idx);
    out.push(b'\'');
    out
}

fn parse_simple_var(b: &[u8], i: usize) -> (Vec<u8>, usize) {
    let mut j = i + 1;
    while j < b.len() && interp_is_name_byte(b[j]) {
        j += 1;
    }
    let mut expr: Vec<u8> = Vec::with_capacity(j - i + 4);
    expr.extend_from_slice(&b[i..j]);

    if j + 2 < b.len() && b[j] == b'-' && b[j + 1] == b'>' && interp_is_name_start(b[j + 2]) {
        let mut k = j + 2;
        while k < b.len() && interp_is_name_byte(b[k]) {
            k += 1;
        }
        expr.extend_from_slice(&b[j..k]);
        return (expr, k);
    } else if j < b.len() && b[j] == b'[' {
        let mut k = j + 1;
        while k < b.len() && b[k] != b']' {
            k += 1;
        }
        if k < b.len() && b[k] == b']' {
            let idx = &b[j + 1..k];
            expr.push(b'[');
            expr.extend_from_slice(&quote_index(idx));
            expr.push(b']');
            return (expr, k + 1);
        }
    }
    (expr, j)
}

fn wrap_interpolations(b: &[u8]) -> (Vec<u8>, bool) {
    let mut out = Vec::with_capacity(b.len() + 8);
    let mut changed = false;
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if c == b'\\' && i + 1 < b.len() {
            out.push(c);
            out.push(b[i + 1]);
            i += 2;
            continue;
        }
        if c == b'{' && i + 1 < b.len() && b[i + 1] == b'$' {
            let mut depth = 0i32;
            let mut j = i;
            while j < b.len() {
                if b[j] == b'{' {
                    depth += 1;
                } else if b[j] == b'}' {
                    depth -= 1;
                    if depth == 0 {
                        j += 1;
                        break;
                    }
                }
                j += 1;
            }
            out.extend_from_slice(&b[i..j]);
            i = j;
            continue;
        }
        if c == b'$' && i + 1 < b.len() {
            if b[i + 1] == b'{' {
                out.push(b'$');
                out.push(b'{');
                i += 2;
                continue;
            }
            if i > 0 && b[i - 1] == b'$' {
                out.push(c);
                i += 1;
                continue;
            }
            if interp_is_name_start(b[i + 1]) {
                let (expr, end) = parse_simple_var(b, i);
                out.push(b'{');
                out.extend_from_slice(&expr);
                out.push(b'}');
                i = end;
                changed = true;
                continue;
            }
        }
        out.push(c);
        i += 1;
    }
    (out, changed)
}

fn interpolated_body(v: &[u8]) -> Option<usize> {
    if !v.is_empty() && v[0] == b'"' {
        return Some(1);
    }
    if v.len() >= 3 && v[0] == b'<' && v[1] == b'<' && v[2] == b'<' {
        let mut p = 3;
        while p < v.len() && (v[p] == b' ' || v[p] == b'\t') {
            p += 1;
        }
        if p < v.len() && v[p] == b'\'' {
            return None;
        }
        while p < v.len() && v[p] != b'\n' {
            p += 1;
        }
        if p < v.len() {
            return Some(p + 1);
        }
    }
    None
}

fn suffix_len(v: &[u8], body_start: usize) -> usize {
    if v[0] == b'"' {
        return 1;
    }
    let mut last: isize = -1;
    for i in body_start..v.len() {
        if v[i] == b'\n' {
            last = i as isize;
        }
    }
    if last < 0 {
        return 0;
    }
    v.len() - last as usize
}

fn explicit_string_variable(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::String {
            continue;
        }
        let v = s.bytes(i).to_vec();
        let start = match interpolated_body(&v) {
            Some(x) => x,
            None => continue,
        };
        let end = v.len() - suffix_len(&v, start);
        if end < start {
            continue;
        }
        let (body, hit) = wrap_interpolations(&v[start..end]);
        if hit {
            let mut nv = Vec::with_capacity(v.len() + 8);
            nv.extend_from_slice(&v[..start]);
            nv.extend_from_slice(&body);
            nv.extend_from_slice(&v[end..]);
            s.set_owned(i, nv);
            changed = true;
        }
    }
    changed
}

fn is_lone_dollar(s: &Stream, i: usize) -> bool {
    s.kind(i) == Kind::Variable && s.bytes(i) == b"$"
}

fn is_real_variable(s: &Stream, i: usize) -> bool {
    s.kind(i) == Kind::Variable && s.bytes(i).len() > 1
}

fn is_dynamic_var_prefix(s: &Stream, i: usize) -> bool {
    if is_lone_dollar(s, i) {
        return true;
    }
    s.kind(i) == Kind::Punct && (s.bytes(i) == b"->" || s.bytes(i) == b"?->")
}

fn explicit_indirect_variable(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i: isize = s.len() as isize - 1;
    while i >= 0 {
        let idx = i as usize;
        if !is_real_variable(s, idx) {
            i -= 1;
            continue;
        }
        let is_prefix = match prev_significant_index(s, idx) {
            Some(p) => is_dynamic_var_prefix(s, p),
            None => false,
        };
        if !is_prefix {
            i -= 1;
            continue;
        }
        s.insert_owned(idx + 1, Kind::Punct, b"}".to_vec());
        s.insert_owned(idx, Kind::Punct, b"{".to_vec());
        changed = true;
        i -= 1;
    }
    changed
}
