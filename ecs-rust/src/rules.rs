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
    r"PhpCsFixer\Fixer\LanguageConstruct\IsNullFixer",
    r"PhpCsFixer\Fixer\ControlStructure\YodaStyleFixer",
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
    r"PhpCsFixer\Fixer\AttributeNotation\AttributeBlockNoSpacesFixer",
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
    r"PhpCsFixer\Fixer\ControlStructure\SwitchContinueToBreakFixer",
    r"PhpCsFixer\Fixer\Import\NoUnneededImportAliasFixer",
    r"PhpCsFixer\Fixer\NamespaceNotation\CleanNamespaceFixer",
    r"PhpCsFixer\Fixer\Comment\MultilineCommentOpeningClosingFixer",
    r"PhpCsFixer\Fixer\Basic\EncodingFixer",
    r"PhpCsFixer\Fixer\LanguageConstruct\DeclareParenthesesFixer",
    r"PhpCsFixer\Fixer\Whitespace\TypeDeclarationSpacesFixer",
    r"PhpCsFixer\Fixer\Whitespace\CompactNullableTypeDeclarationFixer",
    r"PhpCsFixer\Fixer\Whitespace\TypesSpacesFixer",
    r"PhpCsFixer\Fixer\Phpdoc\AlignMultilineCommentFixer",
    r"PhpCsFixer\Fixer\Operator\AssignNullCoalescingToCoalesceEqualFixer",
    r"PhpCsFixer\Fixer\FunctionNotation\NullableTypeDeclarationForDefaultNullValueFixer",
    r"PhpCsFixer\Fixer\Comment\SingleLineCommentStyleFixer",
    r"PhpCsFixer\Fixer\LanguageConstruct\ExplicitIndirectVariableFixer",
    r"PhpCsFixer\Fixer\StringNotation\ExplicitStringVariableFixer",
    r"PhpCsFixer\Fixer\ClassNotation\NoNullPropertyInitializationFixer",
    r"PhpCsFixer\Fixer\ControlStructure\IncludeFixer",
    r"PhpCsFixer\Fixer\ControlStructure\EmptyLoopBodyFixer",
    r"PhpCsFixer\Fixer\ControlStructure\EmptyLoopConditionFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocScalarFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocTypesFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocNoAliasTagFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocNoPackageFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocNoAccessFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocSingleLineVarSpacingFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocNoEmptyReturnFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocTrimFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocTrimConsecutiveBlankLineSeparationFixer",
    r"PhpCsFixer\Fixer\Phpdoc\NoEmptyPhpdocFixer",
    r"PhpCsFixer\Fixer\Phpdoc\NoBlankLinesAfterPhpdocFixer",
    r"PhpCsFixer\Fixer\Casing\LowercaseKeywordsFixer",
    r"PhpCsFixer\Fixer\Casing\ConstantCaseFixer",
    r"PhpCsFixer\Fixer\Casing\LowercaseStaticReferenceFixer",
    r"PhpCsFixer\Fixer\CastNotation\LowercaseCastFixer",
    r"PhpCsFixer\Fixer\CastNotation\ShortScalarCastFixer",
    r"PhpCsFixer\Fixer\Casing\MagicConstantCasingFixer",
    r"PhpCsFixer\Fixer\Casing\MagicMethodCasingFixer",
    r"PhpCsFixer\Fixer\Casing\NativeFunctionCasingFixer",
    r"PhpCsFixer\Fixer\Casing\IntegerLiteralCaseFixer",
    r"PhpCsFixer\Fixer\Casing\NativeTypeDeclarationCasingFixer",
    r"PhpCsFixer\Fixer\Casing\NativeFunctionTypeDeclarationCasingFixer",
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
    r"PhpCsFixer\Fixer\FunctionNotation\MethodArgumentSpaceFixer",
    r"Symplify\CodingStandard\Fixer\Spacing\StandaloneLinePromotedPropertyFixer",
    r"PhpCsFixer\Fixer\FunctionNotation\ReturnTypeDeclarationFixer",
    r"PhpCsFixer\Fixer\Operator\NewWithParenthesesFixer",
    r"PhpCsFixer\Fixer\FunctionNotation\FunctionDeclarationFixer",
    r"PhpCsFixer\Fixer\Whitespace\IndentationTypeFixer",
    r"PhpCsFixer\Fixer\ClassNotation\ClassDefinitionFixer",
    r"PhpCsFixer\Fixer\Basic\BracesPositionFixer",
    r"PhpCsFixer\Fixer\ClassNotation\VisibilityRequiredFixer",
    r"PhpCsFixer\Fixer\ClassNotation\SingleTraitInsertPerStatementFixer",
    r"PhpCsFixer\Fixer\ClassNotation\SingleClassElementPerStatementFixer",
    r"PhpCsFixer\Fixer\ClassNotation\OrderedClassElementsFixer",
    r"PhpCsFixer\Fixer\ClassNotation\ClassAttributesSeparationFixer",
    r"PhpCsFixer\Fixer\NamespaceNotation\BlankLinesBeforeNamespaceFixer",
    r"PhpCsFixer\Fixer\NamespaceNotation\BlankLineAfterNamespaceFixer",
    r"PhpCsFixer\Fixer\Import\NoUnusedImportsFixer",
    r"PhpCsFixer\Fixer\Import\SingleImportPerStatementFixer",
    r"PhpCsFixer\Fixer\Import\OrderedImportsFixer",
    r"PhpCsFixer\Fixer\Whitespace\BlankLineBetweenImportGroupsFixer",
    r"PhpCsFixer\Fixer\Import\SingleLineAfterImportsFixer",
    r"PhpCsFixer\Fixer\ClassNotation\NoBlankLinesAfterClassOpeningFixer",
    r"PhpCsFixer\Fixer\Whitespace\StatementIndentationFixer",
    r"Symplify\CodingStandard\Fixer\Spacing\MethodChainingNewlineFixer",
    r"Symplify\CodingStandard\Fixer\ArrayNotation\ArrayListItemNewlineFixer",
    r"PhpCsFixer\Fixer\Whitespace\ArrayIndentationFixer",
    r"PhpCsFixer\Fixer\Whitespace\NoExtraBlankLinesFixer",
    r"PhpCsFixer\Fixer\PhpTag\NoClosingTagFixer",
];

// Rules run in ecs-go's rules.All() order, restricted to the ported set, so the
// Rust output matches the Go output for the matching `--rules` subset.
pub fn fix(s: &mut Stream) -> bool {
    let mut changed = false;
    changed |= full_opening_tag(s);
    changed |= line_ending(s);
    changed |= is_null(s);
    changed |= yoda_style(s);
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
    changed |= attribute_block_no_spaces(s);
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
    changed |= switch_continue_to_break(s);
    changed |= no_unneeded_import_alias(s);
    changed |= clean_namespace(s);
    changed |= multiline_comment_opening_closing(s);
    changed |= encoding(s);
    changed |= declare_parentheses(s);
    changed |= type_declaration_spaces(s);
    changed |= compact_nullable_type_declaration(s);
    changed |= types_spaces(s);
    changed |= align_multiline_comment(s);
    changed |= assign_null_coalescing_to_coalesce_equal(s);
    changed |= nullable_type_declaration_for_default_null_value(s);
    changed |= single_line_comment_style(s);
    changed |= explicit_indirect_variable(s);
    changed |= explicit_string_variable(s);
    changed |= no_null_property_initialization(s);
    changed |= include(s);
    changed |= empty_loop_body(s);
    changed |= empty_loop_condition(s);
    changed |= phpdoc_scalar(s);
    changed |= phpdoc_types(s);
    changed |= phpdoc_no_alias_tag(s);
    changed |= phpdoc_no_package(s);
    changed |= phpdoc_no_access(s);
    changed |= phpdoc_single_line_var_spacing(s);
    changed |= phpdoc_no_empty_return(s);
    changed |= phpdoc_trim(s);
    changed |= phpdoc_trim_consecutive_blank_line_separation(s);
    changed |= no_empty_phpdoc(s);
    changed |= no_blank_lines_after_phpdoc(s);
    changed |= lowercase_keywords(s);
    changed |= constant_case(s);
    changed |= lowercase_static_reference(s);
    changed |= lowercase_cast(s);
    changed |= short_scalar_cast(s);
    changed |= magic_constant_casing(s);
    changed |= magic_method_casing(s);
    changed |= native_function_casing(s);
    changed |= integer_literal_case(s);
    changed |= native_type_declaration_casing(s);
    changed |= native_function_type_declaration_casing(s);
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
    changed |= method_argument_space(s);
    changed |= standalone_line_promoted_property(s);
    changed |= return_type_declaration(s);
    changed |= new_with_parentheses(s);
    changed |= function_declaration(s);
    changed |= indentation_type(s);
    changed |= class_definition(s);
    changed |= braces_position(s);
    changed |= visibility_required(s);
    changed |= single_trait_insert_per_statement(s);
    changed |= single_class_element_per_statement(s);
    changed |= ordered_class_elements(s);
    changed |= class_attributes_separation(s);
    changed |= blank_lines_before_namespace(s);
    changed |= blank_line_after_namespace(s);
    changed |= no_unused_imports(s);
    changed |= single_import_per_statement(s);
    changed |= ordered_imports(s);
    changed |= blank_line_between_import_groups(s);
    changed |= single_line_after_imports(s);
    changed |= no_blank_lines_after_class_opening(s);
    changed |= statement_indentation(s);
    changed |= method_chaining_newline(s);
    changed |= array_list_item_newline(s);
    changed |= array_indentation(s);
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

// --- ported batch 6: lang / control / misc ---------------------------------

fn is_member_access_prev(s: &Stream, i: usize) -> bool {
    match sig_prev(s, i) {
        Some(p) => s.kind(p) == Kind::Punct && matches!(s.bytes(p), b"->" | b"?->" | b"::"),
        None => false,
    }
}

fn is_control_keyword(lw: &[u8]) -> bool {
    matches!(
        lw,
        b"if" | b"elseif" | b"else" | b"for" | b"foreach" | b"while" | b"do"
            | b"switch" | b"try" | b"catch" | b"finally" | b"match"
    )
}

fn is_class_like_keyword(lw: &[u8]) -> bool {
    matches!(lw, b"class" | b"interface" | b"trait" | b"enum")
}

#[derive(PartialEq, Clone, Copy)]
enum BraceKind {
    Other,
    ClassLike,
    FunctionDecl,
    Closure,
    Control,
}

fn classify_brace(s: &Stream, brace: usize) -> (BraceKind, Option<usize>) {
    let mut j = brace as isize - 1;
    while j >= 0 {
        let ju = j as usize;
        match s.kind(ju) {
            Kind::Whitespace | Kind::Comment | Kind::DocComment => {
                j -= 1;
                continue;
            }
            _ => {}
        }
        if s.kind(ju) == Kind::Punct {
            match s.bytes(ju) {
                b")" => match match_backward(s, ju) {
                    Some(open) => {
                        j = open as isize - 1;
                        continue;
                    }
                    None => return (BraceKind::Other, None),
                },
                b";" | b"{" | b"}" => return (BraceKind::Other, None),
                _ => {
                    j -= 1;
                    continue;
                }
            }
        }
        if s.kind(ju) == Kind::Keyword {
            let lw = s.bytes(ju).to_ascii_lowercase();
            if is_class_like_keyword(lw.as_slice()) {
                return (BraceKind::ClassLike, Some(ju));
            } else if lw == b"function" {
                return (function_brace_kind(s, ju), Some(ju));
            } else if is_control_keyword(lw.as_slice()) {
                return (BraceKind::Control, Some(ju));
            }
        }
        j -= 1;
    }
    (BraceKind::Other, None)
}

fn function_brace_kind(s: &Stream, fnx: usize) -> BraceKind {
    let mut k = skip_ws(s, fnx + 1);
    if k < s.len() && s.kind(k) == Kind::Punct && s.bytes(k) == b"&" {
        k = skip_ws(s, k + 1);
    }
    if k < s.len() && s.kind(k) == Kind::Punct && s.bytes(k) == b"(" {
        return BraceKind::Closure;
    }
    BraceKind::FunctionDecl
}

fn class_member_starts(s: &Stream, open: usize) -> Vec<usize> {
    let close_idx = match match_forward(s, open) {
        Some(c) => c,
        None => return Vec::new(),
    };
    let mut starts = Vec::new();
    let mut expect = true;
    let mut k = open + 1;
    while k < close_idx {
        match s.kind(k) {
            Kind::Whitespace | Kind::Comment | Kind::DocComment => {
                k += 1;
                continue;
            }
            _ => {}
        }
        if expect {
            starts.push(k);
            expect = false;
        }
        if s.kind(k) == Kind::Punct {
            match s.bytes(k) {
                b"{" | b"(" | b"[" => {
                    if let Some(m) = match_forward(s, k) {
                        let was_body = s.bytes(k) == b"{";
                        k = m;
                        if was_body {
                            expect = true;
                        }
                    }
                }
                b";" => expect = true,
                _ => {}
            }
        }
        k += 1;
    }
    starts
}

fn class_body_brace(s: &Stream, kw: usize) -> Option<usize> {
    let mut j = kw + 1;
    while j < s.len() {
        if s.kind(j) == Kind::Punct {
            match s.bytes(j) {
                b"{" => return Some(j),
                b";" => return None,
                _ => {}
            }
        }
        j += 1;
    }
    None
}

fn parse_int_base0(b: &[u8]) -> Option<i64> {
    let str_val = std::str::from_utf8(b).ok()?;
    let (neg, rest) = if let Some(r) = str_val.strip_prefix('-') {
        (true, r)
    } else if let Some(r) = str_val.strip_prefix('+') {
        (false, r)
    } else {
        (false, str_val)
    };
    if rest.is_empty() {
        return None;
    }
    let val: i64 = if let Some(h) = rest.strip_prefix("0x").or_else(|| rest.strip_prefix("0X")) {
        i64::from_str_radix(h, 16).ok()?
    } else if let Some(bn) = rest.strip_prefix("0b").or_else(|| rest.strip_prefix("0B")) {
        i64::from_str_radix(bn, 2).ok()?
    } else if let Some(o) = rest.strip_prefix("0o").or_else(|| rest.strip_prefix("0O")) {
        i64::from_str_radix(o, 8).ok()?
    } else if rest.len() > 1 && rest.starts_with('0') {
        i64::from_str_radix(&rest[1..], 8).ok()?
    } else {
        rest.parse::<i64>().ok()?
    };
    Some(if neg { -val } else { val })
}

fn is_static_ref(v: &[u8]) -> bool {
    let lw = v.to_ascii_lowercase();
    matches!(lw.as_slice(), b"self" | b"parent" | b"static")
}

fn is_path_token(s: &Stream, i: usize) -> bool {
    s.kind(i) == Kind::Ident || (s.kind(i) == Kind::Punct && s.bytes(i) == b"\\")
}

fn prev_path_token(s: &Stream, i: usize, from: usize) -> Option<usize> {
    let mut j = i;
    loop {
        if j == 0 {
            return None;
        }
        j -= 1;
        if j <= from {
            return None;
        }
        match s.kind(j) {
            Kind::Whitespace | Kind::Comment | Kind::DocComment => {}
            _ => return Some(j),
        }
    }
}

fn next_path_token(s: &Stream, i: usize, end: usize) -> usize {
    let mut j = i + 1;
    while j < end {
        match s.kind(j) {
            Kind::Whitespace | Kind::Comment | Kind::DocComment => {}
            _ => return j,
        }
        j += 1;
    }
    end
}

fn is_property_modifier(lw: &[u8]) -> bool {
    matches!(
        lw,
        b"public" | b"protected" | b"private" | b"static" | b"var" | b"final" | b"abstract" | b"readonly"
    )
}

fn is_null_literal(s: &Stream, i: usize) -> bool {
    matches!(s.kind(i), Kind::Ident | Kind::Keyword) && s.bytes(i).eq_ignore_ascii_case(b"null")
}

fn arg_has_depth0_operator(arg: &[(Kind, Vec<u8>)]) -> bool {
    let mut depth = 0i32;
    for (kind, val) in arg {
        if *kind == Kind::Punct {
            match val.as_slice() {
                b"(" | b"[" | b"{" => {
                    depth += 1;
                    continue;
                }
                b")" | b"]" | b"}" => {
                    depth -= 1;
                    continue;
                }
                _ => {}
            }
            if depth == 0 {
                match val.as_slice() {
                    b"->" | b"?->" | b"::" | b"\\" | b"..." => {}
                    _ => return true,
                }
            }
        }
        if depth == 0 && *kind == Kind::Keyword {
            match val.to_ascii_lowercase().as_slice() {
                b"and" | b"or" | b"xor" | b"instanceof" => return true,
                _ => {}
            }
        }
    }
    false
}

fn is_null(s: &mut Stream) -> bool {
    struct Site {
        start: usize,
        arg_open: usize,
        arg_close: usize,
        neg: bool,
    }
    let mut sites: Vec<Site> = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Ident || !s.bytes(i).eq_ignore_ascii_case(b"is_null") {
            i += 1;
            continue;
        }
        let pj = prev_significant_index(s, i);
        if let Some(p) = pj {
            if s.kind(p) == Kind::Punct {
                let pv = s.bytes(p);
                if pv == b"->" || pv == b"?->" || pv == b"::" || pv == b"\\" {
                    i += 1;
                    continue;
                }
            }
            if s.kind(p) == Kind::Keyword && s.bytes(p).eq_ignore_ascii_case(b"function") {
                i += 1;
                continue;
            }
        }
        let op = match next_significant_index(s, i) {
            Some(o) => o,
            None => {
                i += 1;
                continue;
            }
        };
        if s.kind(op) != Kind::Punct || s.bytes(op) != b"(" {
            i += 1;
            continue;
        }
        let cl = match match_forward(s, op) {
            Some(c) => c,
            None => {
                i += 1;
                continue;
            }
        };
        let mut start = i;
        let mut neg = false;
        if let Some(p) = pj {
            if s.kind(p) == Kind::Punct && s.bytes(p) == b"!" {
                neg = true;
                start = p;
            }
        }
        sites.push(Site { start, arg_open: op, arg_close: cl, neg });
        i += 1;
    }
    if sites.is_empty() {
        return false;
    }
    for st in sites.iter().rev() {
        let mut a = st.arg_open + 1;
        let mut b = st.arg_close - 1;
        while a <= b && s.kind(a) == Kind::Whitespace {
            a += 1;
        }
        while b >= a && s.kind(b) == Kind::Whitespace {
            b -= 1;
        }
        let mut arg: Vec<(Kind, Vec<u8>)> = Vec::new();
        if a <= b {
            for x in a..=b {
                arg.push((s.kind(x), s.bytes(x).to_vec()));
            }
        }
        let op_tok: &[u8] = if st.neg { b"!==" } else { b"===" };
        let mut repl: Vec<(Kind, Vec<u8>)> = vec![
            (Kind::Ident, b"null".to_vec()),
            (Kind::Whitespace, b" ".to_vec()),
            (Kind::Punct, op_tok.to_vec()),
            (Kind::Whitespace, b" ".to_vec()),
        ];
        if arg_has_depth0_operator(&arg) {
            repl.push((Kind::Punct, b"(".to_vec()));
            repl.extend(arg.iter().cloned());
            repl.push((Kind::Punct, b")".to_vec()));
        } else {
            repl.extend(arg.iter().cloned());
        }
        replace_range(s, st.start, st.arg_close, repl);
    }
    true
}

fn continue_level(s: &Stream, i: usize) -> Option<usize> {
    let j = sig_next(s, i)?;
    if s.kind(j) == Kind::Punct && s.bytes(j) == b";" {
        return Some(1);
    }
    if s.kind(j) != Kind::Number {
        return None;
    }
    match sig_next(s, j) {
        Some(k) if s.kind(k) == Kind::Punct && s.bytes(k) == b";" => {}
        _ => return None,
    }
    let raw: Vec<u8> = s.bytes(j).iter().cloned().filter(|&c| c != b'_').collect();
    let n = parse_int_base0(&raw)?;
    if n < 0 {
        return None;
    }
    Some(if n == 0 { 1 } else { n as usize })
}

fn enclosing_loops_and_switches(s: &Stream, i: usize) -> Vec<bool> {
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut j = i as isize - 1;
    while j >= 0 {
        let ju = j as usize;
        if s.kind(ju) == Kind::Punct {
            match s.bytes(ju) {
                b"}" => depth += 1,
                b"{" => {
                    if depth > 0 {
                        depth -= 1;
                    } else {
                        let (kind, kw) = classify_brace(s, ju);
                        match kind {
                            BraceKind::FunctionDecl | BraceKind::Closure | BraceKind::ClassLike => {
                                return out
                            }
                            BraceKind::Control => {
                                if let Some(kwi) = kw {
                                    match s.bytes(kwi).to_ascii_lowercase().as_slice() {
                                        b"switch" => out.push(true),
                                        b"for" | b"foreach" | b"while" | b"do" => out.push(false),
                                        _ => {}
                                    }
                                }
                            }
                            BraceKind::Other => {}
                        }
                    }
                }
                _ => {}
            }
        }
        j -= 1;
    }
    out
}

fn switch_continue_to_break(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Keyword || !s.bytes(i).eq_ignore_ascii_case(b"continue") {
            i += 1;
            continue;
        }
        if let Some(level) = continue_level(s, i) {
            let targets = enclosing_loops_and_switches(s, i);
            if level >= 1 && level <= targets.len() && targets[level - 1] {
                s.set_owned(i, b"break".to_vec());
                changed = true;
            }
        }
        i += 1;
    }
    changed
}

fn remove_unneeded_aliases(s: &mut Stream, from: usize, semi: usize) -> bool {
    let mut changed = false;
    let mut k = from + 1;
    let mut end = semi;
    while k < end {
        if s.kind(k) == Kind::Keyword && s.bytes(k).eq_ignore_ascii_case(b"as") {
            let mut seg = k - 1;
            while seg > from && s.kind(seg) == Kind::Whitespace {
                seg -= 1;
            }
            let alias = skip_ws(s, k + 1);
            if seg > from
                && alias < end
                && s.kind(seg) == Kind::Ident
                && s.kind(alias) == Kind::Ident
                && s.bytes(seg) == s.bytes(alias)
            {
                let mut r = alias;
                while r >= seg + 1 {
                    s.remove_at(r);
                    r -= 1;
                }
                end -= alias - seg;
                k = seg + 1;
                changed = true;
                continue;
            }
        }
        k += 1;
    }
    changed
}

fn no_unneeded_import_alias(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Keyword || !s.bytes(i).eq_ignore_ascii_case(b"use") {
            i += 1;
            continue;
        }
        let j = skip_ws(s, i + 1);
        if j < s.len() && s.kind(j) == Kind::Punct && s.bytes(j) == b"(" {
            i += 1;
            continue;
        }
        let mut semi: Option<usize> = None;
        let mut k = i + 1;
        while k < s.len() {
            if s.kind(k) == Kind::Punct && s.bytes(k) == b";" {
                semi = Some(k);
                break;
            }
            k += 1;
        }
        let semi = match semi {
            Some(x) => x,
            None => {
                i += 1;
                continue;
            }
        };
        if remove_unneeded_aliases(s, i, semi) {
            changed = true;
        }
        i += 1;
    }
    changed
}

fn collapse_namespace_spaces(s: &mut Stream, from: usize, end0: usize) -> bool {
    let mut changed = false;
    let mut end = end0;
    let mut k = from + 1;
    while k < end {
        if matches!(s.kind(k), Kind::Whitespace | Kind::Comment | Kind::DocComment) {
            let p = prev_path_token(s, k, from);
            let n = next_path_token(s, k, end);
            if let Some(pi) = p {
                if n < end && is_path_token(s, pi) && is_path_token(s, n) {
                    s.remove_at(k);
                    end -= 1;
                    changed = true;
                    continue;
                }
            }
        }
        k += 1;
    }
    changed
}

fn clean_namespace(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Keyword {
            i += 1;
            continue;
        }
        let lw = s.bytes(i).to_ascii_lowercase();
        if lw != b"namespace" && lw != b"use" {
            i += 1;
            continue;
        }
        let j = skip_ws(s, i + 1);
        if lw == b"use" && j < s.len() && s.kind(j) == Kind::Punct && s.bytes(j) == b"(" {
            i += 1;
            continue;
        }
        let mut end = s.len();
        let mut k = i + 1;
        while k < s.len() {
            if s.kind(k) == Kind::Punct && (s.bytes(k) == b";" || s.bytes(k) == b"{") {
                end = k;
                break;
            }
            k += 1;
        }
        if collapse_namespace_spaces(s, i, end) {
            changed = true;
        }
        i += 1;
    }
    changed
}

fn encoding(s: &mut Stream) -> bool {
    if s.len() == 0 {
        return false;
    }
    const BOM: &[u8] = &[0xEF, 0xBB, 0xBF];
    if !s.bytes(0).starts_with(BOM) {
        return false;
    }
    let rest = s.bytes(0)[BOM.len()..].to_vec();
    if rest.is_empty() {
        s.remove_at(0);
    } else {
        s.set_owned(0, rest);
    }
    true
}

fn declare_parentheses(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Keyword || !s.bytes(i).eq_ignore_ascii_case(b"declare") {
            i += 1;
            continue;
        }
        let mut op = i + 1;
        let mut ws_between: Option<usize> = None;
        if op < s.len() && s.kind(op) == Kind::Whitespace {
            ws_between = Some(op);
            op += 1;
        }
        if op >= s.len() || s.kind(op) != Kind::Punct || s.bytes(op) != b"(" {
            i += 1;
            continue;
        }
        let cp = match match_forward(s, op) {
            Some(c) => c,
            None => {
                i += 1;
                continue;
            }
        };
        let mut drop: Vec<usize> = Vec::new();
        if cp >= 1 && cp - 1 > op && s.kind(cp - 1) == Kind::Whitespace {
            drop.push(cp - 1);
        }
        if op + 1 < cp && s.kind(op + 1) == Kind::Whitespace {
            drop.push(op + 1);
        }
        if let Some(w) = ws_between {
            drop.push(w);
        }
        drop.sort_unstable();
        drop.reverse();
        for idx in drop {
            s.remove_at(idx);
            changed = true;
        }
        i += 1;
    }
    changed
}

fn null_defaults_in_property(s: &Stream, member: usize) -> Vec<usize> {
    let mut i = member;
    loop {
        let is_mod = matches!(s.kind(i), Kind::Keyword | Kind::Ident)
            && is_property_modifier(&s.bytes(i).to_ascii_lowercase());
        if is_mod {
            match sig_next(s, i) {
                Some(n) => {
                    i = n;
                    continue;
                }
                None => return Vec::new(),
            }
        }
        break;
    }
    if s.kind(i) != Kind::Variable {
        return Vec::new();
    }
    let mut remove: Vec<usize> = Vec::new();
    loop {
        let var_idx = i;
        let eq = match sig_next(s, var_idx) {
            Some(e) => e,
            None => return remove,
        };
        if s.kind(eq) != Kind::Punct || s.bytes(eq) != b"=" {
            return remove;
        }
        let mut val_idx = match sig_next(s, eq) {
            Some(v) => v,
            None => return remove,
        };
        if s.kind(val_idx) == Kind::Punct && s.bytes(val_idx) == b"\\" {
            val_idx = match sig_next(s, val_idx) {
                Some(v) => v,
                None => return remove,
            };
        }
        if !is_null_literal(s, val_idx) {
            return remove;
        }
        for k in (var_idx + 1)..=val_idx {
            let tk = s.kind(k);
            if tk == Kind::Comment || tk == Kind::DocComment {
                continue;
            }
            if tk == Kind::Whitespace && s.bytes(k).iter().any(|&c| c == b'\n') {
                continue;
            }
            remove.push(k);
        }
        let after = match sig_next(s, val_idx) {
            Some(a) => a,
            None => return remove,
        };
        if s.kind(after) != Kind::Punct || s.bytes(after) != b"," {
            return remove;
        }
        let next = match sig_next(s, after) {
            Some(n) => n,
            None => return remove,
        };
        if s.kind(next) != Kind::Variable {
            return remove;
        }
        i = next;
    }
}

fn no_null_property_initialization(s: &mut Stream) -> bool {
    let mut remove: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword {
            let lw = s.bytes(i).to_ascii_lowercase();
            if lw == b"class" || lw == b"trait" {
                if let Some(open) = class_body_brace(s, i) {
                    for m in class_member_starts(s, open) {
                        remove.extend(null_defaults_in_property(s, m));
                    }
                }
            }
        }
        i += 1;
    }
    if remove.is_empty() {
        return false;
    }
    remove.sort_unstable();
    remove.dedup();
    remove.reverse();
    for idx in remove {
        s.remove_at(idx);
    }
    true
}

fn include(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Keyword {
            i += 1;
            continue;
        }
        let lw = s.bytes(i).to_ascii_lowercase();
        if !matches!(
            lw.as_slice(),
            b"include" | b"include_once" | b"require" | b"require_once"
        ) {
            i += 1;
            continue;
        }
        if is_member_access_prev(s, i) {
            i += 1;
            continue;
        }
        let n = i + 1;
        if n >= s.len() {
            i += 1;
            continue;
        }
        if s.kind(n) == Kind::Whitespace {
            if has_newline(s.bytes(n)) {
                i += 1;
                continue;
            }
            match sig_next(s, i) {
                None => {
                    i += 1;
                    continue;
                }
                Some(j) => {
                    if s.kind(j) == Kind::Punct && s.bytes(j) == b"(" {
                        i += 1;
                        continue;
                    }
                }
            }
            if s.bytes(n) != b" " {
                s.set_owned(n, b" ".to_vec());
                changed = true;
            }
            i += 1;
            continue;
        }
        let nk = s.kind(n);
        let ok = nk == Kind::Variable
            || nk == Kind::String
            || (nk == Kind::Punct && s.bytes(n) == b"\\");
        if !ok {
            i += 1;
            continue;
        }
        s.insert_owned(n, Kind::Whitespace, b" ".to_vec());
        changed = true;
        i += 1;
    }
    changed
}

fn empty_loop_body(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Keyword {
            i += 1;
            continue;
        }
        let lw = s.bytes(i).to_ascii_lowercase();
        if !matches!(lw.as_slice(), b"for" | b"foreach" | b"while") {
            i += 1;
            continue;
        }
        if is_member_access_prev(s, i) {
            i += 1;
            continue;
        }
        let open = match sig_next(s, i) {
            Some(o) => o,
            None => {
                i += 1;
                continue;
            }
        };
        if s.kind(open) != Kind::Punct || s.bytes(open) != b"(" {
            i += 1;
            continue;
        }
        let close_paren = match match_forward(s, open) {
            Some(c) => c,
            None => {
                i += 1;
                continue;
            }
        };
        let brace_open = match sig_next(s, close_paren) {
            Some(b) => b,
            None => {
                i += 1;
                continue;
            }
        };
        if s.kind(brace_open) != Kind::Punct || s.bytes(brace_open) != b"{" {
            i += 1;
            continue;
        }
        let brace_close = match next_significant_index(s, brace_open) {
            Some(b) => b,
            None => {
                i += 1;
                continue;
            }
        };
        if s.kind(brace_close) != Kind::Punct || s.bytes(brace_close) != b"}" {
            i += 1;
            continue;
        }
        s.set_owned(brace_open, b";".to_vec());
        let mut k = brace_close;
        while k > brace_open {
            s.remove_at(k);
            k -= 1;
        }
        changed = true;
        i += 1;
    }
    changed
}

fn empty_loop_condition(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Keyword || !s.bytes(i).eq_ignore_ascii_case(b"for") {
            i += 1;
            continue;
        }
        if is_member_access_prev(s, i) {
            i += 1;
            continue;
        }
        let open = match sig_next(s, i) {
            Some(o) => o,
            None => {
                i += 1;
                continue;
            }
        };
        if s.kind(open) != Kind::Punct || s.bytes(open) != b"(" {
            i += 1;
            continue;
        }
        let close_paren = match match_forward(s, open) {
            Some(c) => c,
            None => {
                i += 1;
                continue;
            }
        };
        let a = match sig_next(s, open) {
            Some(a) => a,
            None => {
                i += 1;
                continue;
            }
        };
        if s.kind(a) != Kind::Punct || s.bytes(a) != b";" {
            i += 1;
            continue;
        }
        let b = match sig_next(s, a) {
            Some(b) => b,
            None => {
                i += 1;
                continue;
            }
        };
        if s.kind(b) != Kind::Punct || s.bytes(b) != b";" {
            i += 1;
            continue;
        }
        match sig_next(s, b) {
            Some(c) if c == close_paren => {}
            _ => {
                i += 1;
                continue;
            }
        }
        if !span_clean(s, i, close_paren) {
            i += 1;
            continue;
        }
        replace_range(
            s,
            i,
            close_paren,
            vec![
                (Kind::Keyword, b"while".to_vec()),
                (Kind::Whitespace, b" ".to_vec()),
                (Kind::Punct, b"(".to_vec()),
                (Kind::Ident, b"true".to_vec()),
                (Kind::Punct, b")".to_vec()),
            ],
        );
        changed = true;
        i += 1;
    }
    changed
}

fn consume_new_var_ref(s: &Stream, start: usize) -> usize {
    let mut k = start + 1;
    while k < s.len() {
        let ck = s.kind(k);
        if ck == Kind::Punct && matches!(s.bytes(k), b"->" | b"?->" | b"::") {
            let m = skip_ws(s, k + 1);
            if m < s.len() && (s.kind(m) == Kind::Ident || s.kind(m) == Kind::Variable) {
                k = m + 1;
                continue;
            }
            break;
        }
        if ck == Kind::Punct && s.bytes(k) == b"[" {
            match match_forward(s, k) {
                Some(cl) => {
                    k = cl + 1;
                    continue;
                }
                None => break,
            }
        }
        break;
    }
    k
}

fn new_with_parentheses(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Keyword || !s.bytes(i).eq_ignore_ascii_case(b"new") {
            i += 1;
            continue;
        }
        let j = skip_ws(s, i + 1);
        if j >= s.len() {
            i += 1;
            continue;
        }
        let nk = s.kind(j);
        if nk == Kind::Keyword && s.bytes(j).eq_ignore_ascii_case(b"class") {
            i += 1;
            continue;
        }
        if nk == Kind::Variable {
            let k = consume_new_var_ref(s, j);
            if k >= 1 && next_significant_value(s, k - 1) == b"(" {
                i += 1;
                continue;
            }
            s.insert_owned(k, Kind::Punct, b"(".to_vec());
            s.insert_owned(k + 1, Kind::Punct, b")".to_vec());
            changed = true;
            i += 1;
            continue;
        }
        let is_name = nk == Kind::Ident
            || (nk == Kind::Punct && s.bytes(j) == b"\\")
            || (nk == Kind::Keyword && is_static_ref(s.bytes(j)));
        if !is_name {
            i += 1;
            continue;
        }
        let mut k = j;
        while k < s.len() {
            let ck = s.kind(k);
            if ck == Kind::Ident
                || (ck == Kind::Punct && s.bytes(k) == b"\\")
                || (ck == Kind::Keyword && is_static_ref(s.bytes(k)))
            {
                k += 1;
                continue;
            }
            break;
        }
        if k >= 1 && next_significant_value(s, k - 1) == b"(" {
            i += 1;
            continue;
        }
        s.insert_owned(k, Kind::Punct, b"(".to_vec());
        s.insert_owned(k + 1, Kind::Punct, b")".to_vec());
        changed = true;
        i += 1;
    }
    changed
}

// --- ported batch 5: type / casing / attribute -----------------------------

fn is_native_type_name(b: &[u8]) -> bool {
    matches!(
        b,
        b"int" | b"string" | b"bool" | b"float" | b"void"
            | b"array" | b"iterable" | b"object" | b"mixed" | b"null"
            | b"false" | b"true" | b"never" | b"callable" | b"self"
            | b"parent" | b"static"
    )
}

fn is_visibility_modifier(v: &[u8]) -> bool {
    matches!(
        v.to_ascii_lowercase().as_slice(),
        b"public" | b"private" | b"protected" | b"readonly" | b"static" | b"var"
    )
}

fn is_type_name_token(s: &Stream, i: usize) -> bool {
    match s.kind(i) {
        Kind::Ident => true,
        Kind::Keyword => is_native_type_name(s.bytes(i).to_ascii_lowercase().as_slice()),
        _ => false,
    }
}

fn type_context_prev(s: &Stream, i: usize) -> bool {
    let p = match prev_significant_index(s, i) {
        Some(p) => p,
        None => return false,
    };
    match s.kind(p) {
        Kind::Punct => matches!(s.bytes(p), b"(" | b"," | b"|" | b"?" | b"&" | b":"),
        Kind::Keyword => matches!(
            s.bytes(p).to_ascii_lowercase().as_slice(),
            b"public" | b"private" | b"protected" | b"static" | b"readonly" | b"var"
        ),
        _ => false,
    }
}

fn type_context_next(s: &Stream, i: usize) -> bool {
    let n = match next_significant_index(s, i) {
        Some(n) => n,
        None => return false,
    };
    match s.kind(n) {
        Kind::Variable => true,
        Kind::Punct => matches!(s.bytes(n), b"|" | b"&" | b"{" | b";"),
        _ => false,
    }
}

fn is_function_param_open(s: &Stream, open: usize) -> bool {
    if s.kind(open) != Kind::Punct || s.bytes(open) != b"(" {
        return false;
    }
    let p = match prev_significant_index(s, open) {
        Some(p) => p,
        None => return false,
    };
    match s.kind(p) {
        Kind::Keyword => {
            let v = s.bytes(p).to_ascii_lowercase();
            v == b"function" || v == b"fn"
        }
        Kind::Ident => {
            let mut q = prev_significant_index(s, p);
            if let Some(qi) = q {
                if s.kind(qi) == Kind::Punct && s.bytes(qi) == b"&" {
                    q = prev_significant_index(s, qi);
                }
            }
            matches!(q, Some(qi)
                if s.kind(qi) == Kind::Keyword
                    && s.bytes(qi).eq_ignore_ascii_case(b"function"))
        }
        _ => false,
    }
}

fn enclosing_func_param_open(s: &Stream, i: usize) -> Option<usize> {
    let mut depth = 0i32;
    for j in (0..i).rev() {
        if s.kind(j) != Kind::Punct {
            continue;
        }
        match s.bytes(j) {
            b")" => depth += 1,
            b"(" => {
                if depth == 0 {
                    return if is_function_param_open(s, j) { Some(j) } else { None };
                }
                depth -= 1;
            }
            b"{" | b"}" | b";" => {
                if depth == 0 {
                    return None;
                }
            }
            _ => {}
        }
    }
    None
}

fn is_nullable_type_pos(s: &Stream, i: usize) -> bool {
    let p = match prev_significant_index(s, i) {
        Some(p) => p,
        None => return false,
    };
    match s.kind(p) {
        Kind::Punct => matches!(s.bytes(p), b":" | b"(" | b"," | b"|"),
        Kind::Keyword => matches!(
            s.bytes(p).to_ascii_lowercase().as_slice(),
            b"public" | b"private" | b"protected" | b"readonly" | b"static" | b"var"
        ),
        _ => false,
    }
}

fn in_return_type(s: &Stream, i: usize) -> bool {
    let mut j = i;
    loop {
        let p = match prev_significant_index(s, j) {
            Some(p) => p,
            None => return false,
        };
        if s.kind(p) == Kind::Punct && s.bytes(p) == b":" {
            return is_return_type_colon(s, p);
        }
        if s.kind(p) == Kind::Ident
            || (s.kind(p) == Kind::Punct
                && matches!(s.bytes(p), b"?" | b"|" | b"&" | b"\\"))
        {
            j = p;
            continue;
        }
        return false;
    }
}

fn is_function_signature_type(s: &Stream, i: usize) -> bool {
    if enclosing_func_param_open(s, i).is_some()
        && type_context_prev(s, i)
        && type_context_next(s, i)
    {
        return true;
    }
    in_return_type(s, i)
}

fn is_type_union_part(s: &Stream, i: usize) -> bool {
    if is_type_name_token(s, i) {
        return true;
    }
    s.kind(i) == Kind::Punct && matches!(s.bytes(i), b"?" | b"\\" | b"|" | b"&")
}

fn type_run_boundary_prev(s: &Stream, i: usize) -> Option<usize> {
    let mut j = i;
    loop {
        let p = prev_significant_index(s, j)?;
        if is_type_union_part(s, p) {
            j = p;
            continue;
        }
        return Some(p);
    }
}

fn type_run_boundary_next(s: &Stream, i: usize) -> Option<usize> {
    let mut j = i;
    loop {
        let n = next_significant_index(s, j)?;
        if is_type_union_part(s, n) {
            j = n;
            continue;
        }
        return Some(n);
    }
}

fn is_type_union_operator(s: &Stream, i: usize) -> bool {
    let p = match prev_significant_index(s, i) {
        Some(p) => p,
        None => return false,
    };
    let n = match next_significant_index(s, i) {
        Some(n) => n,
        None => return false,
    };
    if !is_type_name_token(s, p) {
        return false;
    }
    if !is_type_name_token(s, n)
        && (s.kind(n) != Kind::Punct || (s.bytes(n) != b"?" && s.bytes(n) != b"\\"))
    {
        return false;
    }
    if in_return_type(s, i) {
        return true;
    }
    let end = match type_run_boundary_next(s, i) {
        Some(e) => e,
        None => return false,
    };
    if s.kind(end) != Kind::Variable {
        return false;
    }
    let start = match type_run_boundary_prev(s, i) {
        Some(st) => st,
        None => return false,
    };
    if enclosing_func_param_open(s, i).is_some() {
        if s.kind(start) == Kind::Punct && matches!(s.bytes(start), b"(" | b",") {
            return true;
        }
        return s.kind(start) == Kind::Keyword && is_visibility_modifier(s.bytes(start));
    }
    s.kind(start) == Kind::Keyword && is_visibility_modifier(s.bytes(start))
}

fn is_type_declaration_variable(s: &Stream, v: usize, p: usize) -> bool {
    if enclosing_func_param_open(s, v).is_some() {
        return true;
    }
    let boundary = match type_run_boundary_prev(s, p) {
        Some(b) => b,
        None => return false,
    };
    s.kind(boundary) == Kind::Keyword && is_visibility_modifier(s.bytes(boundary))
}

fn has_null_default(s: &Stream, v: usize) -> bool {
    let eq = match next_significant_index(s, v) {
        Some(e) => e,
        None => return false,
    };
    if s.kind(eq) != Kind::Punct || s.bytes(eq) != b"=" {
        return false;
    }
    let nv = match next_significant_index(s, eq) {
        Some(n) => n,
        None => return false,
    };
    if s.kind(nv) != Kind::Ident || !s.bytes(nv).eq_ignore_ascii_case(b"null") {
        return false;
    }
    let after = match next_significant_index(s, nv) {
        Some(a) => a,
        None => return false,
    };
    s.kind(after) == Kind::Punct && matches!(s.bytes(after), b"," | b")")
}

fn type_run_start(s: &Stream, end: usize) -> (usize, bool, bool) {
    let mut start = end;
    let mut has_union = false;
    let mut has_nullable = false;
    loop {
        let pp = match prev_significant_index(s, start) {
            Some(p) => p,
            None => break,
        };
        if s.kind(pp) == Kind::Punct {
            match s.bytes(pp) {
                b"|" | b"&" => {
                    has_union = true;
                    start = pp;
                    continue;
                }
                b"?" => {
                    has_nullable = true;
                    start = pp;
                    continue;
                }
                b"\\" => {
                    start = pp;
                    continue;
                }
                _ => {}
            }
            break;
        }
        if is_type_name_token(s, pp) {
            start = pp;
            continue;
        }
        break;
    }
    (start, has_union, has_nullable)
}

fn attribute_block_no_spaces(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::Comment {
            continue;
        }
        let b = s.bytes(i);
        if !b.starts_with(b"#[") || !b.ends_with(b"]") || b.len() < 3 {
            continue;
        }
        let inner = &b[2..b.len() - 1];
        let lo = inner
            .iter()
            .position(|&c| c != b' ' && c != b'\t')
            .unwrap_or(inner.len());
        let hi = inner
            .iter()
            .rposition(|&c| c != b' ' && c != b'\t')
            .map(|p| p + 1)
            .unwrap_or(lo);
        let trimmed = &inner[lo..hi];
        if trimmed.len() == inner.len() {
            continue;
        }
        let mut v = Vec::with_capacity(trimmed.len() + 3);
        v.extend_from_slice(b"#[");
        v.extend_from_slice(trimmed);
        v.push(b']');
        s.set_owned(i, v);
        changed = true;
    }
    changed
}

fn type_declaration_spaces(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::Variable {
            continue;
        }
        let p = match prev_significant_index(s, i) {
            Some(p) if is_type_name_token(s, p) => p,
            _ => continue,
        };
        if !is_type_declaration_variable(s, i, p) {
            continue;
        }
        if p == i - 1 {
            s.insert_owned(i, Kind::Whitespace, b" ".to_vec());
            changed = true;
            continue;
        }
        if p == i - 2 && s.kind(i - 1) == Kind::Whitespace {
            let ws = s.bytes(i - 1);
            if ws != b" " && !has_newline(ws) {
                s.set_owned(i - 1, b" ".to_vec());
                changed = true;
            }
        }
    }
    changed
}

fn compact_nullable_type_declaration(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"?" {
            i += 1;
            continue;
        }
        if !is_nullable_type_pos(s, i) {
            i += 1;
            continue;
        }
        let n = match next_significant_index(s, i) {
            Some(n) if is_type_name_token(s, n) => n,
            _ => {
                i += 1;
                continue;
            }
        };
        let mut j = n - 1;
        while j > i {
            if s.kind(j) == Kind::Whitespace && !has_newline(s.bytes(j)) {
                s.remove_at(j);
                changed = true;
            }
            j -= 1;
        }
        i += 1;
    }
    changed
}

fn types_spaces(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || (s.bytes(i) != b"|" && s.bytes(i) != b"&") {
            i += 1;
            continue;
        }
        if !is_type_union_operator(s, i) {
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

fn nullable_type_declaration_for_default_null_value(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = s.len() as isize - 1;
    while i >= 0 {
        let idx = i as usize;
        if s.kind(idx) != Kind::Variable {
            i -= 1;
            continue;
        }
        if enclosing_func_param_open(s, idx).is_none() {
            i -= 1;
            continue;
        }
        if !has_null_default(s, idx) {
            i -= 1;
            continue;
        }
        let mut p = match prev_significant_index(s, idx) {
            Some(p) => p,
            None => {
                i -= 1;
                continue;
            }
        };
        if s.kind(p) == Kind::Punct && s.bytes(p) == b"&" {
            p = match prev_significant_index(s, p) {
                Some(p) => p,
                None => {
                    i -= 1;
                    continue;
                }
            };
        }
        if !is_type_name_token(s, p) {
            i -= 1;
            continue;
        }
        let (start, has_union, has_nullable) = type_run_start(s, p);
        if has_nullable || has_union {
            i -= 1;
            continue;
        }
        if start == p {
            let lo = s.bytes(p).to_ascii_lowercase();
            if lo == b"mixed" || lo == b"null" {
                i -= 1;
                continue;
            }
        }
        s.insert_owned(start, Kind::Punct, b"?".to_vec());
        changed = true;
        i = start as isize;
        i -= 1;
    }
    changed
}

fn native_type_declaration_casing(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::Ident && s.kind(i) != Kind::Keyword {
            continue;
        }
        let lower = s.bytes(i).to_ascii_lowercase();
        if !is_native_type_name(lower.as_slice()) || lower.as_slice() == s.bytes(i) {
            continue;
        }
        if !type_context_prev(s, i) || !type_context_next(s, i) {
            continue;
        }
        s.set_owned(i, lower);
        changed = true;
    }
    changed
}

fn native_function_type_declaration_casing(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::Ident && s.kind(i) != Kind::Keyword {
            continue;
        }
        let lower = s.bytes(i).to_ascii_lowercase();
        if !is_native_type_name(lower.as_slice()) || lower.as_slice() == s.bytes(i) {
            continue;
        }
        if !is_function_signature_type(s, i) {
            continue;
        }
        s.set_owned(i, lower);
        changed = true;
    }
    changed
}

// --- ported batch 7: imports ------------------------------------------------

struct ImportStmt {
    start: usize,
    semi: usize,
    rank: u8,
}

fn indent_before(s: &Stream, i: usize) -> Vec<u8> {
    if i == 0 || s.kind(i - 1) != Kind::Whitespace {
        return Vec::new();
    }
    let v = s.bytes(i - 1);
    match v.iter().rposition(|&c| c == b'\n') {
        Some(idx) => v[idx + 1..].to_vec(),
        None => Vec::new(),
    }
}

fn trim_ws_tokens(mut toks: Vec<(Kind, Vec<u8>)>) -> Vec<(Kind, Vec<u8>)> {
    while toks.first().map_or(false, |t| t.0 == Kind::Whitespace) {
        toks.remove(0);
    }
    while toks.last().map_or(false, |t| t.0 == Kind::Whitespace) {
        toks.pop();
    }
    toks
}

fn tokens_equal_toks(a: &[(Kind, Vec<u8>)], b: &[(Kind, Vec<u8>)]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.0 == y.0 && x.1 == y.1)
}

fn use_rank(s: &Stream, use_idx: usize) -> u8 {
    let j = skip_ws(s, use_idx + 1);
    if j < s.len() && s.kind(j) == Kind::Keyword {
        let lw = s.bytes(j).to_ascii_lowercase();
        if lw == b"function" {
            return 1;
        }
        if lw == b"const" {
            return 2;
        }
    }
    0
}

fn collect_import_run(s: &Stream, start: usize) -> Vec<ImportStmt> {
    let mut stmts = Vec::new();
    let mut k = start;
    while k < s.len() {
        if s.kind(k) != Kind::Keyword
            || !s.bytes(k).eq_ignore_ascii_case(b"use")
            || in_class_like_body(s, k)
        {
            break;
        }
        let j = skip_ws(s, k + 1);
        if j < s.len() && s.kind(j) == Kind::Punct && s.bytes(j) == b"(" {
            break;
        }
        let mut semi: isize = -1;
        let mut group = false;
        let mut m = k + 1;
        while m < s.len() {
            if s.kind(m) == Kind::Punct {
                if s.bytes(m) == b"{" {
                    group = true;
                    break;
                }
                if s.bytes(m) == b";" {
                    semi = m as isize;
                    break;
                }
            }
            m += 1;
        }
        if group || semi < 0 {
            break;
        }
        let semi = semi as usize;
        stmts.push(ImportStmt { start: k, semi, rank: use_rank(s, k) });
        let n = skip_ws(s, semi + 1);
        if n < s.len() && s.kind(n) == Kind::Keyword && s.bytes(n).eq_ignore_ascii_case(b"use") {
            k = n;
            continue;
        }
        break;
    }
    stmts
}

fn split_import_parts(
    s: &Stream,
    j: usize,
    semi: usize,
    commas: &[usize],
) -> Vec<Vec<(Kind, Vec<u8>)>> {
    let mut bounds: Vec<isize> = vec![j as isize - 1];
    for &c in commas {
        bounds.push(c as isize);
    }
    bounds.push(semi as isize);
    let mut parts = Vec::new();
    for b in 0..bounds.len() - 1 {
        let mut part = Vec::new();
        let mut k = (bounds[b] + 1) as usize;
        while (k as isize) < bounds[b + 1] {
            part.push((s.kind(k), s.bytes(k).to_vec()));
            k += 1;
        }
        parts.push(trim_ws_tokens(part));
    }
    parts
}

fn split_use_at(s: &mut Stream, i: usize, in_class: bool) -> (usize, bool) {
    if in_class_like_body(s, i) != in_class {
        return (i + 1, false);
    }
    let mut j = skip_ws(s, i + 1);
    let mut modifier: Option<Vec<u8>> = None;
    if j < s.len() && s.kind(j) == Kind::Keyword {
        let lw = s.bytes(j).to_ascii_lowercase();
        if lw == b"function" || lw == b"const" {
            modifier = Some(s.bytes(j).to_vec());
            j = skip_ws(s, j + 1);
        }
    }
    if j < s.len() && s.kind(j) == Kind::Punct && s.bytes(j) == b"(" {
        return (i + 1, false);
    }
    let mut semi: isize = -1;
    let mut group = false;
    let mut commas: Vec<usize> = Vec::new();
    let mut k = j;
    while k < s.len() {
        if s.kind(k) == Kind::Punct {
            match s.bytes(k) {
                b"{" => group = true,
                b";" => semi = k as isize,
                b"," => commas.push(k),
                _ => {}
            }
            if group || semi >= 0 {
                break;
            }
        }
        k += 1;
    }
    if group || semi < 0 || commas.is_empty() {
        return (i + 1, false);
    }
    let semi = semi as usize;
    let indent = indent_before(s, i);
    let parts = split_import_parts(s, j, semi, &commas);

    let mut repl: Vec<(Kind, Vec<u8>)> = Vec::new();
    for (p, part) in parts.iter().enumerate() {
        if p > 0 {
            let mut nl = vec![b'\n'];
            nl.extend_from_slice(&indent);
            repl.push((Kind::Whitespace, nl));
        }
        repl.push((Kind::Keyword, b"use".to_vec()));
        repl.push((Kind::Whitespace, b" ".to_vec()));
        if let Some(m) = &modifier {
            repl.push((Kind::Keyword, m.clone()));
            repl.push((Kind::Whitespace, b" ".to_vec()));
        }
        for t in part {
            repl.push(t.clone());
        }
        repl.push((Kind::Punct, b";".to_vec()));
    }
    let n = repl.len();
    replace_range(s, i, semi, repl);
    (i + n, true)
}

fn split_use_statements(s: &mut Stream, in_class: bool) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Keyword || !s.bytes(i).eq_ignore_ascii_case(b"use") {
            i += 1;
            continue;
        }
        let (next, c) = split_use_at(s, i, in_class);
        i = next;
        if c {
            changed = true;
        }
    }
    changed
}

fn import_short_name(s: &Stream, from: usize, semi: usize) -> Option<Vec<u8>> {
    let mut k = from;
    while k < semi {
        if s.kind(k) == Kind::Keyword && s.bytes(k).eq_ignore_ascii_case(b"as") {
            let a = skip_ws(s, k + 1);
            if a < semi && s.kind(a) == Kind::Ident {
                return Some(s.bytes(a).to_vec());
            }
        }
        k += 1;
    }
    let mut k = semi as isize - 1;
    while k >= from as isize {
        if s.kind(k as usize) == Kind::Ident {
            return Some(s.bytes(k as usize).to_vec());
        }
        k -= 1;
    }
    None
}

fn single_import_per_statement(s: &mut Stream) -> bool {
    split_use_statements(s, false)
}

fn single_trait_insert_per_statement(s: &mut Stream) -> bool {
    split_use_statements(s, true)
}

fn reorder_imports(s: &mut Stream, run: &[ImportStmt]) -> (usize, bool) {
    let first = run[0].start;
    let last = run[run.len() - 1].semi;
    let indent = indent_before(s, first);

    let mut ordered: Vec<&ImportStmt> = run.iter().collect();
    ordered.sort_by_key(|st| st.rank);

    let mut repl: Vec<(Kind, Vec<u8>)> = Vec::new();
    for (p, st) in ordered.iter().enumerate() {
        if p > 0 {
            let mut nl = vec![b'\n'];
            nl.extend_from_slice(&indent);
            repl.push((Kind::Whitespace, nl));
        }
        let mut k = st.start;
        while k <= st.semi {
            repl.push((s.kind(k), s.bytes(k).to_vec()));
            k += 1;
        }
    }

    let mut orig: Vec<(Kind, Vec<u8>)> = Vec::new();
    let mut k = first;
    while k <= last {
        orig.push((s.kind(k), s.bytes(k).to_vec()));
        k += 1;
    }
    let changed = !tokens_equal_toks(&orig, &repl);
    let n = repl.len();
    replace_range(s, first, last, repl);
    (first + n, changed)
}

fn ordered_imports(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Keyword
            || !s.bytes(i).eq_ignore_ascii_case(b"use")
            || in_class_like_body(s, i)
        {
            i += 1;
            continue;
        }
        let run = collect_import_run(s, i);
        if run.len() < 2 {
            i += 1;
            continue;
        }
        let (new_end, c) = reorder_imports(s, &run);
        if c {
            changed = true;
        }
        i = new_end;
    }
    changed
}

fn blank_line_between_import_groups(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Keyword
            || !s.bytes(i).eq_ignore_ascii_case(b"use")
            || in_class_like_body(s, i)
        {
            i += 1;
            continue;
        }
        let run = collect_import_run(s, i);
        if run.len() >= 2 {
            for p in 1..run.len() {
                if run[p].rank == run[p - 1].rank {
                    continue;
                }
                if run[p].start >= 1 {
                    let ws = run[p].start - 1;
                    if s.kind(ws) == Kind::Whitespace
                        && has_newline(s.bytes(ws))
                        && s.bytes(ws) != b"\n\n"
                    {
                        s.set_owned(ws, b"\n\n".to_vec());
                        changed = true;
                    }
                }
            }
        }
        if !run.is_empty() {
            i = run[run.len() - 1].semi + 1;
        } else {
            i += 1;
        }
    }
    changed
}

fn single_line_after_imports(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Keyword || !s.bytes(i).eq_ignore_ascii_case(b"use") {
            i += 1;
            continue;
        }
        if in_class_like_body(s, i) {
            i += 1;
            continue;
        }
        let j = skip_ws(s, i + 1);
        if j < s.len() && s.kind(j) == Kind::Punct && s.bytes(j) == b"(" {
            i += 1;
            continue;
        }
        let mut semi: isize = -1;
        let mut k = i + 1;
        while k < s.len() {
            if s.kind(k) == Kind::Punct && s.bytes(k) == b";" {
                semi = k as isize;
                break;
            }
            k += 1;
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
        let nx = skip_ws(s, semi + 1);
        if nx >= s.len() {
            i += 1;
            continue;
        }
        if s.kind(nx) == Kind::Keyword && s.bytes(nx).eq_ignore_ascii_case(b"use") {
            i += 1;
            continue;
        }
        if s.kind(nx) == Kind::Punct && s.bytes(nx) == b"}" {
            i += 1;
            continue;
        }
        if s.kind(semi + 1) == Kind::Whitespace
            && has_newline(s.bytes(semi + 1))
            && s.bytes(semi + 1) != b"\n\n"
        {
            s.set_owned(semi + 1, b"\n\n".to_vec());
            changed = true;
        }
        i += 1;
    }
    changed
}

fn no_unused_imports(s: &mut Stream) -> bool {
    struct II {
        start: usize,
        semi: usize,
        short_lower: Vec<u8>,
    }
    let mut imports: Vec<II> = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Keyword
            || !s.bytes(i).eq_ignore_ascii_case(b"use")
            || in_class_like_body(s, i)
        {
            i += 1;
            continue;
        }
        let mut j = skip_ws(s, i + 1);
        if j < s.len() && s.kind(j) == Kind::Punct && s.bytes(j) == b"(" {
            i += 1;
            continue;
        }
        if j < s.len() && s.kind(j) == Kind::Keyword {
            let lw = s.bytes(j).to_ascii_lowercase();
            if lw == b"function" || lw == b"const" {
                j = skip_ws(s, j + 1);
            }
        }
        let mut semi: isize = -1;
        let mut group = false;
        let mut k = i + 1;
        while k < s.len() {
            if s.kind(k) == Kind::Punct {
                if s.bytes(k) == b"{" {
                    group = true;
                    break;
                }
                if s.bytes(k) == b";" {
                    semi = k as isize;
                    break;
                }
            }
            k += 1;
        }
        if group || semi < 0 {
            i += 1;
            continue;
        }
        let semi = semi as usize;
        let short = match import_short_name(s, j, semi) {
            Some(x) if !x.is_empty() => x,
            _ => {
                i += 1;
                continue;
            }
        };
        imports.push(II { start: i, semi, short_lower: short.to_ascii_lowercase() });
        i = semi + 1;
    }
    if imports.is_empty() {
        return false;
    }

    let in_import = |idx: usize| -> bool {
        imports.iter().any(|im| idx >= im.start && idx <= im.semi)
    };
    let mut used: std::collections::HashSet<Vec<u8>> = std::collections::HashSet::new();
    let mut comment_text: Vec<u8> = Vec::new();
    for k in 0..s.len() {
        if s.kind(k) == Kind::Ident && !in_import(k) {
            used.insert(s.bytes(k).to_ascii_lowercase());
        }
        if s.kind(k) == Kind::Comment || s.kind(k) == Kind::DocComment {
            comment_text.extend_from_slice(&s.bytes(k).to_ascii_lowercase());
        }
    }

    let mut changed = false;
    for im in imports.iter().rev() {
        if used.contains(&im.short_lower) || contains_subslice(&comment_text, &im.short_lower) {
            continue;
        }
        let mut r = im.semi;
        loop {
            s.remove_at(r);
            if r == im.start {
                break;
            }
            r -= 1;
        }
        if im.start < s.len() && s.kind(im.start) == Kind::Whitespace {
            let v = s.bytes(im.start);
            if v.first() == Some(&b'\n') {
                let nv = v[1..].to_vec();
                s.set_owned(im.start, nv);
            }
        }
        changed = true;
    }
    changed
}

// --- ported batch 9: functions / arrays / yoda -----------------------------

fn copy_range(s: &Stream, a: usize, b: usize) -> Vec<(Kind, Vec<u8>)> {
    let mut out = Vec::new();
    let mut j = a;
    while j <= b {
        out.push((s.kind(j), s.bytes(j).to_vec()));
        j += 1;
    }
    out
}

fn is_array_literal_open(s: &Stream, open: usize) -> bool {
    let p = match sig_prev(s, open) {
        Some(p) => p,
        None => return true,
    };
    match s.kind(p) {
        Kind::Variable | Kind::Ident | Kind::String | Kind::Number => false,
        Kind::Punct => !matches!(s.bytes(p), b")" | b"]" | b"}"),
        _ => true,
    }
}

fn line_indent_before(s: &Stream, idx: usize) -> Vec<u8> {
    let mut i = idx as isize - 1;
    while i >= 0 {
        let k = i as usize;
        if s.kind(k) == Kind::Whitespace && s.bytes(k).contains(&b'\n') {
            let b = s.bytes(k);
            if let Some(nl) = b.iter().rposition(|&c| c == b'\n') {
                return b[nl + 1..].to_vec();
            }
        }
        i -= 1;
    }
    Vec::new()
}

fn edit_slot_after(s: &mut Stream, idx: usize, val: &[u8]) -> bool {
    if idx + 1 < s.len() && s.kind(idx + 1) == Kind::Whitespace {
        if s.bytes(idx + 1) != val {
            s.set_owned(idx + 1, val.to_vec());
            return true;
        }
        return false;
    }
    s.insert_owned(idx + 1, Kind::Whitespace, val.to_vec());
    true
}

fn edit_slot_before(s: &mut Stream, idx: usize, val: &[u8]) -> bool {
    if idx > 0 && s.kind(idx - 1) == Kind::Whitespace {
        if s.bytes(idx - 1) != val {
            s.set_owned(idx - 1, val.to_vec());
            return true;
        }
        return false;
    }
    s.insert_owned(idx, Kind::Whitespace, val.to_vec());
    true
}

fn is_call_or_decl_paren(s: &Stream, open: usize) -> bool {
    let p = match sig_prev(s, open) {
        Some(p) => p,
        None => return false,
    };
    match s.kind(p) {
        Kind::Ident | Kind::Variable => true,
        Kind::Punct => s.bytes(p) == b")" || s.bytes(p) == b"]",
        Kind::Keyword => {
            let lv = s.bytes(p).to_ascii_lowercase();
            lv == b"function" || lv == b"fn"
        }
        _ => false,
    }
}

fn reflow_paren(s: &mut Stream, open: usize, close_idx: usize) -> bool {
    let mut changed = false;
    let base = line_indent_before(s, open);
    let mut arg_nl = vec![b'\n'];
    arg_nl.extend_from_slice(&base);
    arg_nl.extend_from_slice(b"    ");

    let mut commas: Vec<usize> = Vec::new();
    let mut depth = 0i32;
    let mut j = open + 1;
    while j < close_idx {
        if s.kind(j) == Kind::Punct {
            match s.bytes(j) {
                b"(" | b"[" | b"{" => depth += 1,
                b")" | b"]" | b"}" => depth -= 1,
                b"," => {
                    if depth == 0 && sig_next(s, j) != Some(close_idx) {
                        commas.push(j);
                    }
                }
                _ => {}
            }
        }
        j += 1;
    }

    let mut close_nl = vec![b'\n'];
    close_nl.extend_from_slice(&base);
    if edit_slot_before(s, close_idx, &close_nl) {
        changed = true;
    }
    for &c in commas.iter().rev() {
        if edit_slot_after(s, c, &arg_nl) {
            changed = true;
        }
    }
    if edit_slot_after(s, open, &arg_nl) {
        changed = true;
    }
    changed
}

fn yoda_op(v: &[u8]) -> bool {
    matches!(v, b"==" | b"===" | b"!=" | b"!==" | b"<" | b">" | b"<=" | b">=")
}

fn yoda_mirror(v: &[u8]) -> Vec<u8> {
    match v {
        b"<" => b">".to_vec(),
        b">" => b"<".to_vec(),
        b"<=" => b">=".to_vec(),
        b">=" => b"<=".to_vec(),
        _ => v.to_vec(),
    }
}

fn is_yoda_literal(s: &Stream, i: usize) -> bool {
    match s.kind(i) {
        Kind::Number | Kind::String | Kind::Ident => true,
        Kind::Keyword => matches!(
            s.bytes(i).to_ascii_lowercase().as_slice(),
            b"true" | b"false" | b"null"
        ),
        _ => false,
    }
}

fn is_primary_start(s: &Stream, i: usize) -> bool {
    match s.kind(i) {
        Kind::Variable | Kind::Ident => true,
        Kind::Keyword => matches!(
            s.bytes(i).to_ascii_lowercase().as_slice(),
            b"static" | b"self" | b"parent"
        ),
        Kind::Punct => s.bytes(i) == b"\\",
        _ => false,
    }
}

fn is_left_boundary(s: &Stream, p: Option<usize>) -> bool {
    let p = match p {
        Some(p) => p,
        None => return true,
    };
    match s.kind(p) {
        Kind::Punct => matches!(
            s.bytes(p),
            b"(" | b"[" | b"{" | b"," | b";" | b"&&" | b"||" | b"?" | b"??" | b":" | b"=" | b"!" | b"=>" | b"."
        ),
        Kind::Keyword => matches!(
            s.bytes(p).to_ascii_lowercase().as_slice(),
            b"return" | b"and" | b"or" | b"xor" | b"echo" | b"print" | b"case" | b"if" | b"elseif" | b"while"
        ),
        _ => false,
    }
}

fn is_right_boundary(s: &Stream, j: Option<usize>) -> bool {
    let j = match j {
        Some(j) => j,
        None => return true,
    };
    match s.kind(j) {
        Kind::Punct => matches!(
            s.bytes(j),
            b")" | b"]" | b"}" | b";" | b"," | b":" | b"&&" | b"||" | b"?" | b"??" | b"." | b"=>"
        ),
        Kind::Keyword => matches!(
            s.bytes(j).to_ascii_lowercase().as_slice(),
            b"and" | b"or" | b"xor"
        ),
        _ => false,
    }
}

fn left_literal_operand(s: &Stream, op: usize) -> Option<(usize, usize)> {
    let le = prev_significant_index(s, op)?;
    if s.kind(le) == Kind::Punct && s.bytes(le) == b"]" {
        if let Some(o) = match_backward(s, le) {
            if next_significant_index(s, o) == Some(le) {
                return Some((o, le));
            }
        }
        return None;
    }
    if s.kind(le) == Kind::Ident && s.bytes(le).eq_ignore_ascii_case(b"class") {
        if let Some(p) = prev_significant_index(s, le) {
            if s.bytes(p) == b"::" {
                if let Some(n) = prev_significant_index(s, p) {
                    if s.kind(n) == Kind::Ident {
                        return Some((n, le));
                    }
                }
            }
        }
        return None;
    }
    if is_yoda_literal(s, le) {
        if s.kind(le) == Kind::Number {
            if let Some(p) = prev_significant_index(s, le) {
                if s.kind(p) == Kind::Punct && (s.bytes(p) == b"-" || s.bytes(p) == b"+") {
                    if is_left_boundary(s, prev_significant_index(s, p)) {
                        return Some((p, le));
                    }
                }
            }
        }
        return Some((le, le));
    }
    None
}

fn right_primary_end(s: &Stream, rs: usize) -> Option<usize> {
    if !is_primary_start(s, rs) {
        return None;
    }
    let mut end = rs;
    loop {
        let n = match next_significant_index(s, end) {
            Some(n) => n,
            None => return Some(end),
        };
        if s.kind(n) == Kind::Punct {
            match s.bytes(n) {
                b"->" | b"?->" | b"::" | b"\\" => {
                    let m = match next_significant_index(s, n) {
                        Some(m) => m,
                        None => return Some(end),
                    };
                    end = m;
                    continue;
                }
                b"(" | b"[" => {
                    let c = match match_forward(s, n) {
                        Some(c) => c,
                        None => return Some(end),
                    };
                    end = c;
                    continue;
                }
                _ => {}
            }
        }
        return Some(end);
    }
}

fn yoda_style(s: &mut Stream) -> bool {
    let mut swaps: Vec<(usize, usize, usize, usize)> = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || !yoda_op(s.bytes(i)) {
            i += 1;
            continue;
        }
        let (ls, le) = match left_literal_operand(s, i) {
            Some(x) => x,
            None => {
                i += 1;
                continue;
            }
        };
        if !is_left_boundary(s, prev_significant_index(s, ls)) {
            i += 1;
            continue;
        }
        let rs = match next_significant_index(s, i) {
            Some(r) => r,
            None => {
                i += 1;
                continue;
            }
        };
        let re = match right_primary_end(s, rs) {
            Some(r) => r,
            None => {
                i += 1;
                continue;
            }
        };
        if rs == re && is_yoda_literal(s, rs) {
            i += 1;
            continue;
        }
        if !is_right_boundary(s, next_significant_index(s, re)) {
            i += 1;
            continue;
        }
        swaps.push((ls, le, rs, re));
        i += 1;
    }
    if swaps.is_empty() {
        return false;
    }
    for &(ls, le, rs, re) in swaps.iter().rev() {
        let left = copy_range(s, ls, le);
        let mut mid = if rs > le + 1 { copy_range(s, le + 1, rs - 1) } else { Vec::new() };
        let right = copy_range(s, rs, re);
        for m in mid.iter_mut() {
            if m.0 == Kind::Punct {
                m.1 = yoda_mirror(&m.1);
            }
        }
        let mut repl = right;
        repl.extend(mid);
        repl.extend(left);
        replace_range(s, ls, re, repl);
    }
    true
}

fn arg_list_is_multiline(s: &Stream, open: usize, close_idx: usize) -> bool {
    let n = open + 1;
    if n < close_idx && s.kind(n) == Kind::Whitespace && has_newline(s.bytes(n)) {
        return true;
    }
    let mut depth = 0i32;
    let mut j = open + 1;
    while j < close_idx {
        if s.kind(j) == Kind::Punct {
            match s.bytes(j) {
                b"(" | b"[" | b"{" => depth += 1,
                b")" | b"]" | b"}" => depth -= 1,
                b"," => {
                    if depth == 0
                        && j + 1 < close_idx
                        && s.kind(j + 1) == Kind::Whitespace
                        && has_newline(s.bytes(j + 1))
                    {
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

fn reflow_multiline_args(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut open = 0;
    while open < s.len() {
        if s.kind(open) == Kind::Punct && s.bytes(open) == b"(" {
            if let Some(close_idx) = match_forward(s, open) {
                if arg_list_is_multiline(s, open, close_idx)
                    && is_call_or_decl_paren(s, open)
                    && sig_next(s, open) != Some(close_idx)
                {
                    if reflow_paren(s, open, close_idx) {
                        changed = true;
                    }
                }
            }
        }
        open += 1;
    }
    changed
}

fn method_argument_space(s: &mut Stream) -> bool {
    let mut changed = reflow_multiline_args(s);
    let mut stack: Vec<u8> = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct {
            i += 1;
            continue;
        }
        match s.bytes(i) {
            b"(" | b"[" | b"{" => {
                stack.push(s.bytes(i)[0]);
                i += 1;
                continue;
            }
            b")" | b"]" | b"}" => {
                stack.pop();
                i += 1;
                continue;
            }
            b"," => {
                if stack.last() != Some(&b'(') {
                    i += 1;
                    continue;
                }
                if i + 1 < s.len() && s.kind(i + 1) == Kind::Whitespace && has_newline(s.bytes(i + 1)) {
                    i += 1;
                    continue;
                }
                if i > 0 && s.kind(i - 1) == Kind::Whitespace && !has_newline(s.bytes(i - 1)) {
                    s.remove_at(i - 1);
                    i -= 1;
                    changed = true;
                }
                if i + 1 < s.len() {
                    if s.kind(i + 1) == Kind::Whitespace {
                        if s.bytes(i + 1) != b" " {
                            s.set_owned(i + 1, b" ".to_vec());
                            changed = true;
                        }
                    } else if s.kind(i + 1) != Kind::Punct || s.bytes(i + 1) != b")" {
                        s.insert_owned(i + 1, Kind::Whitespace, b" ".to_vec());
                        i += 1;
                        changed = true;
                    }
                }
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    changed
}

fn fn_ensure_single_space_after(s: &mut Stream, i: usize) -> bool {
    if i + 1 >= s.len() {
        return false;
    }
    if s.kind(i + 1) == Kind::Whitespace {
        if has_newline(s.bytes(i + 1)) || s.bytes(i + 1) == b" " {
            return false;
        }
        s.set_owned(i + 1, b" ".to_vec());
        return true;
    }
    s.insert_owned(i + 1, Kind::Whitespace, b" ".to_vec());
    true
}

fn fn_ensure_single_space_before(s: &mut Stream, i: usize) -> bool {
    if i == 0 {
        return false;
    }
    if s.kind(i - 1) == Kind::Whitespace {
        if has_newline(s.bytes(i - 1)) || s.bytes(i - 1) == b" " {
            return false;
        }
        s.set_owned(i - 1, b" ".to_vec());
        return true;
    }
    s.insert_owned(i, Kind::Whitespace, b" ".to_vec());
    true
}

fn fn_glue_after(s: &mut Stream, i: usize) -> bool {
    if i + 1 < s.len() && s.kind(i + 1) == Kind::Whitespace && !has_newline(s.bytes(i + 1)) {
        s.remove_at(i + 1);
        return true;
    }
    false
}

fn find_function_param_open(s: &Stream, fi: usize) -> Option<usize> {
    let mut j = fi + 1;
    while j < s.len() {
        match s.kind(j) {
            Kind::Whitespace | Kind::Ident => {
                j += 1;
                continue;
            }
            Kind::Punct => match s.bytes(j) {
                b"(" => return Some(j),
                b"&" => {
                    j += 1;
                    continue;
                }
                _ => return None,
            },
            _ => return None,
        }
    }
    None
}

fn fix_function_declaration(s: &mut Stream, fi: usize) -> bool {
    let start_paren = match find_function_param_open(s, fi) {
        Some(p) => p,
        None => return false,
    };
    let mut a = next_significant_index(s, fi);
    let mut amp_index: Option<usize> = None;
    if let Some(ai) = a {
        if s.kind(ai) == Kind::Punct && s.bytes(ai) == b"&" {
            amp_index = Some(ai);
            a = next_significant_index(s, ai);
        }
    }
    let mut name_index: Option<usize> = None;
    if let Some(ai) = a {
        if ai < start_paren {
            if s.kind(ai) == Kind::Ident && next_significant_index(s, ai) == Some(start_paren) {
                name_index = Some(ai);
            } else {
                return false;
            }
        }
    }
    let named = name_index.is_some();
    let mut changed = false;

    if !named {
        if let Some(close_paren) = match_forward(s, start_paren) {
            if let Some(u) = next_significant_index(s, close_paren) {
                if s.kind(u) == Kind::Keyword && s.bytes(u).eq_ignore_ascii_case(b"use") {
                    if fn_ensure_single_space_after(s, u) {
                        changed = true;
                    }
                    if fn_ensure_single_space_before(s, u) {
                        changed = true;
                    }
                }
            }
        }
    }

    if named {
        let ni = name_index.unwrap();
        if fn_glue_after(s, ni) {
            changed = true;
        }
        if let Some(ai) = amp_index {
            if fn_glue_after(s, ai) {
                changed = true;
            }
        }
    }

    if fn_ensure_single_space_after(s, fi) {
        changed = true;
    }

    if !named {
        if let Some(p) = prev_significant_index(s, fi) {
            if s.kind(p) == Kind::Keyword && s.bytes(p).eq_ignore_ascii_case(b"static") {
                if fi >= 1
                    && s.kind(fi - 1) == Kind::Whitespace
                    && !has_newline(s.bytes(fi - 1))
                    && s.bytes(fi - 1) != b" "
                {
                    s.set_owned(fi - 1, b" ".to_vec());
                    changed = true;
                }
            }
        }
    }
    changed
}

fn function_declaration(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword && s.bytes(i).eq_ignore_ascii_case(b"function") {
            if fix_function_declaration(s, i) {
                changed = true;
            }
        }
        i += 1;
    }
    changed
}

fn array_top_level_multiline(s: &Stream, open: usize, close_idx: usize) -> bool {
    let mut depth = 0i32;
    let mut j = open + 1;
    while j < close_idx {
        if s.kind(j) == Kind::Punct {
            match s.bytes(j) {
                b"(" | b"[" | b"{" => {
                    depth += 1;
                    j += 1;
                    continue;
                }
                b")" | b"]" | b"}" => {
                    depth -= 1;
                    j += 1;
                    continue;
                }
                _ => {}
            }
        }
        if depth == 0 && s.kind(j) == Kind::Whitespace && has_newline(s.bytes(j)) {
            return true;
        }
        j += 1;
    }
    false
}

fn array_has_top_level_arrow(s: &Stream, open: usize, close_idx: usize) -> bool {
    let mut depth = 0i32;
    let mut j = open + 1;
    while j < close_idx {
        if s.kind(j) == Kind::Punct {
            match s.bytes(j) {
                b"(" | b"[" | b"{" => depth += 1,
                b")" | b"]" | b"}" => depth -= 1,
                b"=>" => {
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

fn array_list_item_newline(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut open = 0;
    while open < s.len() {
        if s.kind(open) == Kind::Punct && s.bytes(open) == b"[" && is_array_literal_open(s, open) {
            if let Some(mut close_idx) = match_forward(s, open) {
                if sig_next(s, open) != Some(close_idx)
                    && !array_top_level_multiline(s, open, close_idx)
                    && array_has_top_level_arrow(s, open, close_idx)
                {
                    if let Some(last) = sig_prev(s, close_idx) {
                        if last > open && s.bytes(last) != b"," {
                            s.insert_owned(last + 1, Kind::Punct, b",".to_vec());
                            close_idx += 1;
                            changed = true;
                        }
                    }
                    if reflow_paren(s, open, close_idx) {
                        changed = true;
                    }
                }
            }
        }
        open += 1;
    }
    changed
}

struct AiScope {
    kind_array: bool,
    end_index: usize,
    initial_indent: Vec<u8>,
    new_indent: Vec<u8>,
}

fn ai_extract_indent(ws: &[u8]) -> Vec<u8> {
    if let Some(i) = ws.iter().rposition(|&c| c == b'\n') {
        ws[i + 1..].to_vec()
    } else {
        ws.to_vec()
    }
}

fn ai_reindent_array(ws: &[u8], target: &[u8]) -> Vec<u8> {
    if let Some(i) = ws.iter().rposition(|&c| c == b'\n') {
        let mut out = ws[..=i].to_vec();
        out.extend_from_slice(target);
        out
    } else {
        ws.to_vec()
    }
}

fn ai_reindent_expression(ws: &[u8], initial_indent: &[u8], new_indent: &[u8]) -> Vec<u8> {
    let i = match ws.iter().rposition(|&c| c == b'\n') {
        Some(i) => i,
        None => return ws.to_vec(),
    };
    let line = &ws[i + 1..];
    if !line.starts_with(initial_indent) {
        return ws.to_vec();
    }
    let mut out = ws[..=i].to_vec();
    out.extend_from_slice(new_indent);
    out.extend_from_slice(&line[initial_indent.len()..]);
    out
}

fn ai_array_open(s: &Stream, index: usize) -> Option<usize> {
    if s.kind(index) != Kind::Punct {
        return None;
    }
    match s.bytes(index) {
        b"[" => {
            if !is_array_literal_open(s, index) {
                return None;
            }
            match_forward(s, index)
        }
        b"(" => {
            if let Some(p) = sig_prev(s, index) {
                if s.kind(p) == Kind::Keyword {
                    let lv = s.bytes(p).to_ascii_lowercase();
                    if lv == b"array" || lv == b"list" {
                        return match_forward(s, index);
                    }
                }
            }
            None
        }
        _ => None,
    }
}

fn ai_expression_end(s: &Stream, index: usize, parent_end: usize) -> usize {
    let mut end: isize = -1;
    let mut k = index + 1;
    while k < parent_end {
        if s.kind(k) == Kind::Punct && matches!(s.bytes(k), b"(" | b"{" | b"[") {
            if let Some(be) = match_forward(s, k) {
                k = be;
            }
        } else if s.kind(k) == Kind::Punct && s.bytes(k) == b"," {
            end = match sig_prev(s, k) {
                Some(p) => p as isize,
                None => -1,
            };
            break;
        }
        k += 1;
    }
    if end >= 0 {
        return end as usize;
    }
    sig_prev(s, parent_end).unwrap_or(parent_end)
}

fn array_indentation(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut last_indent: Vec<u8> = Vec::new();
    let mut scopes: Vec<AiScope> = Vec::new();
    let mut prev_line_initial_indent: Vec<u8> = Vec::new();
    let mut prev_line_new_indent: Vec<u8> = Vec::new();
    let unit: &[u8] = b"    ";

    let n = s.len();
    for index in 0..n {
        let cur = scopes.len() as isize - 1;
        let k = s.kind(index);
        if k == Kind::Comment || k == Kind::DocComment {
            continue;
        }

        if let Some(end) = ai_array_open(s, index) {
            scopes.push(AiScope {
                kind_array: true,
                end_index: end,
                initial_indent: last_indent.clone(),
                new_indent: Vec::new(),
            });
            continue;
        }

        if k == Kind::Whitespace && has_newline(s.bytes(index)) {
            last_indent = ai_extract_indent(s.bytes(index));
        }

        if cur < 0 {
            continue;
        }
        let ci = cur as usize;

        if k == Kind::Whitespace {
            if !has_newline(s.bytes(index)) {
                continue;
            }
            let content: Vec<u8>;
            if scopes[ci].kind_array {
                let end_index = scopes[ci].end_index;
                let mut indent = false;
                let mut kk = index + 1;
                while kk < end_index {
                    let kt = s.kind(kk);
                    let non_trivia = kt != Kind::Whitespace && kt != Kind::Comment && kt != Kind::DocComment;
                    if non_trivia || (kt == Kind::Whitespace && has_newline(s.bytes(kk))) {
                        indent = true;
                        break;
                    }
                    kk += 1;
                }
                let mut target = scopes[ci].initial_indent.clone();
                if indent {
                    target.extend_from_slice(unit);
                }
                content = ai_reindent_array(s.bytes(index), &target);
                prev_line_initial_indent = ai_extract_indent(s.bytes(index));
                prev_line_new_indent = ai_extract_indent(&content);
            } else {
                content = ai_reindent_expression(
                    s.bytes(index),
                    &scopes[ci].initial_indent,
                    &scopes[ci].new_indent,
                );
            }
            if content.as_slice() != s.bytes(index) {
                s.set_owned(index, content.clone());
                changed = true;
            }
            last_indent = ai_extract_indent(&content);
            continue;
        }

        if index == scopes[ci].end_index {
            while !scopes.is_empty() && index == scopes[scopes.len() - 1].end_index {
                scopes.pop();
            }
            continue;
        }

        if s.kind(index) == Kind::Punct && s.bytes(index) == b"," {
            continue;
        }

        if scopes[ci].kind_array {
            let end = ai_expression_end(s, index, scopes[ci].end_index);
            if end != index {
                scopes.push(AiScope {
                    kind_array: false,
                    end_index: end,
                    initial_indent: prev_line_initial_indent.clone(),
                    new_indent: prev_line_new_indent.clone(),
                });
            }
        }
    }
    changed
}

// --- ported batch 8: class structure ---------------------------------------

fn brace_depth_at(s: &Stream, idx: usize) -> usize {
    let mut d = 0usize;
    for j in 0..idx {
        if s.kind(j) != Kind::Punct {
            continue;
        }
        match s.bytes(j) {
            b"{" => d += 1,
            b"}" => {
                if d > 0 {
                    d -= 1;
                }
            }
            _ => {}
        }
    }
    d
}

fn has_promoted_param(s: &Stream, open: usize, close_idx: usize) -> bool {
    let mut depth = 0i32;
    let mut j = open + 1;
    while j < close_idx {
        if s.kind(j) == Kind::Punct {
            match s.bytes(j) {
                b"(" | b"[" | b"{" => depth += 1,
                b")" | b"]" | b"}" => depth -= 1,
                _ => {}
            }
            j += 1;
            continue;
        }
        if depth == 0 && s.kind(j) == Kind::Keyword {
            match s.bytes(j).to_ascii_lowercase().as_slice() {
                b"public" | b"protected" | b"private" | b"readonly" => return true,
                _ => {}
            }
        }
        j += 1;
    }
    false
}

fn standalone_line_promoted_property(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Ident || !s.bytes(i).eq_ignore_ascii_case(b"__construct") {
            i += 1;
            continue;
        }
        match sig_prev(s, i) {
            Some(p) if s.kind(p) == Kind::Keyword && s.bytes(p).eq_ignore_ascii_case(b"function") => {}
            _ => {
                i += 1;
                continue;
            }
        }
        let open = match sig_next(s, i) {
            Some(o) if s.kind(o) == Kind::Punct && s.bytes(o) == b"(" => o,
            _ => {
                i += 1;
                continue;
            }
        };
        let close_idx = match match_forward(s, open) {
            Some(c) => c,
            None => {
                i += 1;
                continue;
            }
        };
        if sig_next(s, open) == Some(close_idx) {
            i += 1;
            continue;
        }
        if !has_promoted_param(s, open, close_idx) {
            i += 1;
            continue;
        }
        if reflow_paren(s, open, close_idx) {
            changed = true;
        }
        i += 1;
    }
    changed
}

fn collapse_space(s: &mut Stream, i: usize) -> bool {
    if i >= s.len() {
        return false;
    }
    if s.kind(i) != Kind::Whitespace || has_newline(s.bytes(i)) || s.bytes(i) == b" " {
        return false;
    }
    s.set_owned(i, b" ".to_vec());
    true
}

fn normalize_header_spacing(s: &mut Stream, kw: usize) -> bool {
    let mut changed = collapse_space(s, kw + 1);
    let mut j = kw + 1;
    while j < s.len() {
        if s.kind(j) == Kind::Punct && s.bytes(j) == b"{" {
            break;
        }
        if s.kind(j) == Kind::Keyword {
            match s.bytes(j).to_ascii_lowercase().as_slice() {
                b"extends" | b"implements" => {
                    if j > 0 && collapse_space(s, j - 1) {
                        changed = true;
                    }
                    if collapse_space(s, j + 1) {
                        changed = true;
                    }
                }
                _ => {}
            }
        }
        j += 1;
    }
    changed
}

fn class_definition(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Keyword
            || !is_class_like_keyword(s.bytes(i).to_ascii_lowercase().as_slice())
        {
            i += 1;
            continue;
        }
        if member_prev(s, i) {
            i += 1;
            continue;
        }
        if let Some(p) = prev_significant_index(s, i) {
            if s.bytes(p).eq_ignore_ascii_case(b"new") {
                i += 1;
                continue;
            }
        }
        if normalize_header_spacing(s, i) {
            changed = true;
        }
        i += 1;
    }
    changed
}

fn func_signature_multiline(s: &Stream, brace: usize) -> bool {
    let mut close_paren: isize = -1;
    let mut j = brace as isize - 1;
    while j >= 0 {
        let k = j as usize;
        if s.kind(k) == Kind::Punct {
            match s.bytes(k) {
                b")" => {
                    close_paren = k as isize;
                    break;
                }
                b"{" | b"}" | b";" => return false,
                _ => {}
            }
        }
        j -= 1;
    }
    if close_paren < 0 {
        return false;
    }
    let open = match match_backward(s, close_paren as usize) {
        Some(o) => o,
        None => return false,
    };
    let mut k = open;
    while k <= close_paren as usize {
        if s.kind(k) == Kind::Whitespace && has_newline(s.bytes(k)) {
            return true;
        }
        k += 1;
    }
    false
}

fn braces_position(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"{" {
            i += 1;
            continue;
        }
        let (kind, _) = classify_brace(s, i);
        let next_line = match kind {
            BraceKind::ClassLike => true,
            BraceKind::FunctionDecl => !func_signature_multiline(s, i),
            BraceKind::Control => false,
            _ => {
                i += 1;
                continue;
            }
        };
        let want: Vec<u8> = if next_line {
            let mut w = b"\n".to_vec();
            w.extend_from_slice(&b"    ".repeat(brace_depth_at(s, i)));
            w
        } else {
            b" ".to_vec()
        };
        if i > 0 && s.kind(i - 1) == Kind::Whitespace {
            if s.bytes(i - 1) != want.as_slice() {
                s.set_owned(i - 1, want);
                changed = true;
            }
        } else {
            s.insert_owned(i, Kind::Whitespace, want);
            i += 1;
            changed = true;
        }
        i += 1;
    }
    changed
}

fn add_visibility(s: &mut Stream, m: usize) -> bool {
    let mut has_vis = false;
    let mut var_idx: isize = -1;
    let mut k = m;
    while k < s.len() {
        if s.kind(k) != Kind::Keyword {
            break;
        }
        let lw = s.bytes(k).to_ascii_lowercase();
        if lw == b"use" || lw == b"case" {
            return false;
        }
        if !is_property_modifier(&lw) {
            break;
        }
        if lw == b"public" || lw == b"private" || lw == b"protected" {
            has_vis = true;
        }
        if lw == b"var" {
            var_idx = k as isize;
        }
        k = skip_ws(s, k + 1);
    }
    if has_vis {
        return false;
    }
    if var_idx >= 0 {
        s.set_owned(var_idx as usize, b"public".to_vec());
        return true;
    }
    s.insert_owned(m, Kind::Keyword, b"public".to_vec());
    s.insert_owned(m + 1, Kind::Whitespace, b" ".to_vec());
    true
}

fn visibility_required(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"{" {
            i += 1;
            continue;
        }
        if classify_brace(s, i).0 != BraceKind::ClassLike {
            i += 1;
            continue;
        }
        let starts = class_member_starts(s, i);
        for &m in starts.iter().rev() {
            if add_visibility(s, m) {
                changed = true;
            }
        }
        i += 1;
    }
    changed
}

fn trim_ws_toks(mut toks: Vec<(Kind, Vec<u8>)>) -> Vec<(Kind, Vec<u8>)> {
    while toks.first().map(|t| t.0 == Kind::Whitespace).unwrap_or(false) {
        toks.remove(0);
    }
    while toks.last().map(|t| t.0 == Kind::Whitespace).unwrap_or(false) {
        toks.pop();
    }
    toks
}

fn member_end_semi(s: &Stream, m: usize) -> isize {
    let mut k = m;
    while k < s.len() {
        if s.kind(k) == Kind::Punct {
            match s.bytes(k) {
                b"(" | b"[" | b"{" => {
                    if let Some(mm) = match_forward(s, k) {
                        k = mm;
                    }
                }
                b";" => return k as isize,
                b"}" => return -1,
                _ => {}
            }
        }
        k += 1;
    }
    -1
}

fn split_class_element(s: &mut Stream, m: usize) -> bool {
    let mut k = m;
    while k < s.len()
        && s.kind(k) == Kind::Keyword
        && is_property_modifier(&s.bytes(k).to_ascii_lowercase())
    {
        k = skip_ws(s, k + 1);
    }
    if k >= s.len() {
        return false;
    }
    let mut is_const = false;
    if s.kind(k) == Kind::Keyword {
        match s.bytes(k).to_ascii_lowercase().as_slice() {
            b"const" => is_const = true,
            _ => return false,
        }
    }
    let semi = member_end_semi(s, m);
    if semi < 0 {
        return false;
    }
    let semi = semi as usize;

    let mut first_elem: isize = -1;
    if is_const {
        first_elem = skip_ws(s, k + 1) as isize;
    } else {
        let mut depth = 0i32;
        let mut x = m;
        while x < semi {
            if s.kind(x) == Kind::Punct {
                match s.bytes(x) {
                    b"(" | b"[" => depth += 1,
                    b")" | b"]" => depth -= 1,
                    _ => {}
                }
            }
            if depth == 0 && s.kind(x) == Kind::Variable {
                first_elem = x as isize;
                break;
            }
            x += 1;
        }
    }
    if first_elem < 0 || first_elem as usize >= semi {
        return false;
    }
    let first_elem = first_elem as usize;

    let mut commas: Vec<usize> = Vec::new();
    let mut depth = 0i32;
    let mut x = first_elem;
    while x < semi {
        if s.kind(x) == Kind::Punct {
            match s.bytes(x) {
                b"(" | b"[" => depth += 1,
                b")" | b"]" => depth -= 1,
                b"," => {
                    if depth == 0 {
                        commas.push(x);
                    }
                }
                _ => {}
            }
        }
        x += 1;
    }
    if commas.is_empty() {
        return false;
    }

    let mut prefix: Vec<(Kind, Vec<u8>)> = Vec::new();
    for x in m..first_elem {
        prefix.push((s.kind(x), s.bytes(x).to_vec()));
    }
    let indent = line_indent(s, m);

    let mut bounds: Vec<usize> = Vec::with_capacity(commas.len() + 2);
    bounds.push(first_elem - 1);
    bounds.extend_from_slice(&commas);
    bounds.push(semi);

    let mut repl: Vec<(Kind, Vec<u8>)> = Vec::new();
    for b in 0..bounds.len() - 1 {
        let mut part: Vec<(Kind, Vec<u8>)> = Vec::new();
        for x in bounds[b] + 1..bounds[b + 1] {
            part.push((s.kind(x), s.bytes(x).to_vec()));
        }
        let part = trim_ws_toks(part);
        if b > 0 {
            let mut nl = b"\n".to_vec();
            nl.extend_from_slice(&indent);
            repl.push((Kind::Whitespace, nl));
        }
        repl.extend(prefix.iter().cloned());
        repl.extend(part);
        repl.push((Kind::Punct, b";".to_vec()));
    }
    replace_range(s, m, semi, repl);
    true
}

fn single_class_element_per_statement(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"{" {
            i += 1;
            continue;
        }
        if classify_brace(s, i).0 != BraceKind::ClassLike {
            i += 1;
            continue;
        }
        let starts = class_member_starts(s, i);
        for &m in starts.iter().rev() {
            if split_class_element(s, m) {
                changed = true;
            }
        }
        i += 1;
    }
    changed
}

const GROUP_TRAIT_USE: i32 = 0;
const GROUP_CONST: i32 = 1;
const GROUP_PROPERTY: i32 = 2;
const GROUP_METHOD: i32 = 3;

fn member_span_end(s: &Stream, m: usize) -> isize {
    let mut k = m;
    while k < s.len() {
        if s.kind(k) == Kind::Punct {
            match s.bytes(k) {
                b"(" | b"[" => match match_forward(s, k) {
                    Some(mm) => k = mm,
                    None => return -1,
                },
                b"{" => return match match_forward(s, k) {
                    Some(mm) => mm as isize,
                    None => -1,
                },
                b";" => return k as isize,
                b"}" => return -1,
                _ => {}
            }
        }
        k += 1;
    }
    -1
}

fn classify_member_group(s: &Stream, m: usize, end: usize) -> i32 {
    let mut k = m;
    while k <= end
        && s.kind(k) == Kind::Keyword
        && is_property_modifier(&s.bytes(k).to_ascii_lowercase())
    {
        k = skip_ws(s, k + 1);
    }
    if k > end {
        return -1;
    }
    if s.kind(k) == Kind::Keyword {
        return match s.bytes(k).to_ascii_lowercase().as_slice() {
            b"use" => GROUP_TRAIT_USE,
            b"const" => GROUP_CONST,
            b"function" => GROUP_METHOD,
            _ => -1,
        };
    }
    let mut x = k;
    while x <= end {
        if s.kind(x) == Kind::Punct {
            match s.bytes(x) {
                b"(" | b"[" | b"{" => {
                    if let Some(mm) = match_forward(s, x) {
                        x = mm;
                        x += 1;
                        continue;
                    }
                }
                _ => {}
            }
        }
        if s.kind(x) == Kind::Variable {
            return GROUP_PROPERTY;
        }
        x += 1;
    }
    -1
}

fn is_identity_order(order: &[usize]) -> bool {
    for (i, &v) in order.iter().enumerate() {
        if i != v {
            return false;
        }
    }
    true
}

fn reorder_class_body(s: &mut Stream, open: usize) -> bool {
    let close_idx = match match_forward(s, open) {
        Some(c) => c,
        None => return false,
    };
    for k in open + 1..close_idx {
        if matches!(s.kind(k), Kind::Comment | Kind::DocComment) {
            return false;
        }
    }
    let starts = class_member_starts(s, open);
    if starts.len() < 2 {
        return false;
    }
    let mut ends = vec![0usize; starts.len()];
    let mut groups = vec![0i32; starts.len()];
    for (idx, &m) in starts.iter().enumerate() {
        let end = member_span_end(s, m);
        if end < 0 || end as usize >= close_idx {
            return false;
        }
        let end = end as usize;
        let g = classify_member_group(s, m, end);
        if g < 0 {
            return false;
        }
        ends[idx] = end;
        groups[idx] = g;
    }
    for idx in 0..starts.len() - 1 {
        for k in ends[idx] + 1..starts[idx + 1] {
            if s.kind(k) != Kind::Whitespace {
                return false;
            }
        }
    }
    let mut order: Vec<usize> = Vec::with_capacity(starts.len());
    for g in GROUP_TRAIT_USE..=GROUP_METHOD {
        for (idx, &gg) in groups.iter().enumerate() {
            if gg == g {
                order.push(idx);
            }
        }
    }
    if is_identity_order(&order) {
        return false;
    }
    let mut repl: Vec<(Kind, Vec<u8>)> = Vec::new();
    for pos in 0..order.len() {
        let member_idx = order[pos];
        for k in starts[member_idx]..=ends[member_idx] {
            repl.push((s.kind(k), s.bytes(k).to_vec()));
        }
        if pos + 1 < order.len() {
            for k in ends[pos] + 1..starts[pos + 1] {
                repl.push((s.kind(k), s.bytes(k).to_vec()));
            }
        }
    }
    let last_end = ends[ends.len() - 1];
    replace_range(s, starts[0], last_end, repl);
    true
}

fn ordered_class_elements(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Punct && s.bytes(i) == b"{" && classify_brace(s, i).0 == BraceKind::ClassLike {
            if reorder_class_body(s, i) {
                changed = true;
                i = 0;
                continue;
            }
        }
        i += 1;
    }
    changed
}

const SEP_UNKNOWN: i32 = 0;
const SEP_CONST: i32 = 1;
const SEP_METHOD: i32 = 2;
const SEP_PROPERTY: i32 = 3;
const SEP_TRAIT_IMPORT: i32 = 4;
const SEP_CASE: i32 = 5;

fn is_attribute(s: &Stream, i: usize) -> bool {
    s.kind(i) == Kind::Comment && s.bytes(i).starts_with(b"#[")
}

fn set_newline_count(s: &mut Stream, idx: usize, req: usize) -> bool {
    let v = s.bytes(idx).to_vec();
    let indent: Vec<u8> = match v.iter().rposition(|&c| c == b'\n') {
        Some(nl) => v[nl + 1..].to_vec(),
        None => Vec::new(),
    };
    let mut want = b"\n".repeat(req);
    want.extend_from_slice(&indent);
    if v == want {
        return false;
    }
    s.set_owned(idx, want);
    true
}

fn has_doc_or_attr_above(s: &Stream, start: usize) -> bool {
    let mut j = start as isize - 1;
    while j >= 0 && s.kind(j as usize) == Kind::Whitespace {
        j -= 1;
    }
    if j < 0 {
        return false;
    }
    let k = j as usize;
    s.kind(k) == Kind::DocComment || is_attribute(s, k)
}

fn class_sep_member_type(s: &Stream, m: usize) -> i32 {
    let end = member_span_end(s, m);
    if end < 0 {
        return SEP_UNKNOWN;
    }
    let end = end as usize;
    let mut k = m;
    while k <= end
        && s.kind(k) == Kind::Keyword
        && is_property_modifier(&s.bytes(k).to_ascii_lowercase())
    {
        k = skip_ws(s, k + 1);
    }
    if k > end {
        return SEP_UNKNOWN;
    }
    if s.kind(k) == Kind::Keyword {
        return match s.bytes(k).to_ascii_lowercase().as_slice() {
            b"use" => SEP_TRAIT_IMPORT,
            b"const" => SEP_CONST,
            b"function" => SEP_METHOD,
            b"case" => SEP_CASE,
            _ => SEP_UNKNOWN,
        };
    }
    let mut x = k;
    while x <= end {
        if s.kind(x) == Kind::Punct {
            match s.bytes(x) {
                b"(" | b"[" | b"{" => {
                    if let Some(mm) = match_forward(s, x) {
                        x = mm;
                        x += 1;
                        continue;
                    }
                }
                _ => {}
            }
        }
        if s.kind(x) == Kind::Variable {
            return SEP_PROPERTY;
        }
        x += 1;
    }
    SEP_UNKNOWN
}

fn required_newlines(s: &Stream, prev_start: usize, prev_type: i32, next_type: i32) -> usize {
    if next_type == SEP_TRAIT_IMPORT || next_type == SEP_CASE {
        if prev_type == next_type && !has_doc_or_attr_above(s, prev_start) {
            return 1;
        }
        return 2;
    }
    2
}

fn fix_member_gap(s: &mut Stream, prev_start: usize, next_start: usize) -> bool {
    let prev_end = member_span_end(s, prev_start);
    if prev_end < 0 || prev_end as usize >= next_start {
        return false;
    }
    let prev_end = prev_end as usize;
    let next_type = class_sep_member_type(s, next_start);
    if next_type == SEP_UNKNOWN {
        return false;
    }

    let mut first_trivia: isize = -1;
    let mut ws_count = 0;
    let mut ws_idx: isize = -1;
    for j in prev_end + 1..next_start {
        let k = s.kind(j);
        if k == Kind::Whitespace {
            ws_count += 1;
            ws_idx = j as isize;
        } else if k == Kind::DocComment || is_attribute(s, j) {
            if first_trivia < 0 {
                first_trivia = j as isize;
            }
        } else if k == Kind::Comment {
            return false;
        } else {
            return false;
        }
    }

    if first_trivia < 0 {
        if ws_count != 1 || ws_idx < 0 || !has_newline(s.bytes(ws_idx as usize)) {
            return false;
        }
        let prev_type = class_sep_member_type(s, prev_start);
        let req = required_newlines(s, prev_start, prev_type, next_type);
        return set_newline_count(s, ws_idx as usize, req);
    }

    let first_trivia = first_trivia as usize;
    if first_trivia != prev_end + 2 {
        return false;
    }
    let ws0 = prev_end + 1;
    if s.kind(ws0) != Kind::Whitespace || !has_newline(s.bytes(ws0)) {
        return false;
    }
    for j in first_trivia + 1..next_start {
        if s.kind(j) == Kind::Whitespace
            && s.bytes(j).iter().filter(|&&c| c == b'\n').count() != 1
        {
            return false;
        }
    }
    set_newline_count(s, ws0, 2)
}

fn class_attributes_separation(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"{" {
            i += 1;
            continue;
        }
        if classify_brace(s, i).0 != BraceKind::ClassLike {
            i += 1;
            continue;
        }
        let starts = class_member_starts(s, i);
        let mut k = 0;
        while k + 1 < starts.len() {
            if fix_member_gap(s, starts[k], starts[k + 1]) {
                changed = true;
            }
            k += 1;
        }
        i += 1;
    }
    changed
}

fn statement_indentation(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut brace: i32 = 0;
    let mut paren: i32 = 0;
    for i in 0..s.len() {
        if s.kind(i) == Kind::Punct {
            match s.bytes(i) {
                b"{" => brace += 1,
                b"}" => {
                    if brace > 0 {
                        brace -= 1;
                    }
                }
                b"(" | b"[" => paren += 1,
                b")" | b"]" => {
                    if paren > 0 {
                        paren -= 1;
                    }
                }
                _ => {}
            }
        }
        if s.kind(i) != Kind::Whitespace || !has_newline(s.bytes(i)) || paren != 0 {
            continue;
        }
        match prev_significant_index(s, i) {
            Some(p) if matches!(s.bytes(p), b";" | b"{" | b"}") => {}
            _ => continue,
        }
        let nsv = next_significant_value(s, i);
        if nsv.is_empty() {
            continue;
        }
        let mut level = brace;
        if nsv == b"}" {
            level -= 1;
        }
        if level < 0 {
            level = 0;
        }
        let target = b"    ".repeat(level as usize);
        let v = s.bytes(i).to_vec();
        let nl = v.iter().rposition(|&c| c == b'\n').unwrap();
        if v[nl + 1..] != target[..] {
            let mut nv = v[..nl + 1].to_vec();
            nv.extend_from_slice(&target);
            s.set_owned(i, nv);
            changed = true;
        }
    }
    changed
}

fn chain_is_call_close(s: &Stream, close_idx: usize) -> bool {
    let open = match match_backward(s, close_idx) {
        Some(o) => o,
        None => return false,
    };
    let p = match sig_prev(s, open) {
        Some(p) => p,
        None => return false,
    };
    if s.kind(p) == Kind::Ident || s.kind(p) == Kind::Variable {
        return true;
    }
    s.kind(p) == Kind::Punct && (s.bytes(p) == b")" || s.bytes(p) == b"]")
}

fn chain_base_indent(s: &Stream, index: usize) -> Vec<u8> {
    let mut i = index as isize - 1;
    while i >= 0 {
        let k = i as usize;
        if s.kind(k) == Kind::Whitespace && has_newline(s.bytes(k)) {
            let v = s.bytes(k);
            if let Some(nl) = v.iter().rposition(|&c| c == b'\n') {
                return v[nl + 1..].to_vec();
            }
        }
        i -= 1;
    }
    Vec::new()
}

fn chain_first_line_indent(s: &Stream, op_idx: usize) -> Vec<u8> {
    let mut i = op_idx;
    let mut root = op_idx;
    while i > 0 {
        let k = i - 1;
        let kind = s.kind(k);
        if matches!(kind, Kind::Whitespace | Kind::Comment | Kind::DocComment) {
            i -= 1;
        } else if kind == Kind::Ident || kind == Kind::Variable {
            root = k;
            i -= 1;
        } else if kind == Kind::Punct && matches!(s.bytes(k), b"->" | b"?->" | b"::") {
            i -= 1;
        } else if kind == Kind::Keyword && s.bytes(k).eq_ignore_ascii_case(b"new") {
            root = k;
            i -= 1;
        } else if kind == Kind::Punct && matches!(s.bytes(k), b")" | b"]" | b"}") {
            match match_backward(s, k) {
                Some(m) => {
                    root = m;
                    i = m;
                }
                None => return chain_base_indent(s, root),
            }
        } else {
            return chain_base_indent(s, root);
        }
    }
    chain_base_indent(s, root)
}

fn chain_line_has_breaking_char(s: &Stream, pos: usize) -> bool {
    let mut nesting = 0i32;
    let mut i = pos as isize;
    while i >= 0 {
        let k = i as usize;
        if s.kind(k) == Kind::Whitespace && has_newline(s.bytes(k)) {
            return false;
        }
        if s.kind(k) == Kind::Punct && matches!(s.bytes(k), b"[" | b"::" | b".") {
            return true;
        }
        if s.kind(k) == Kind::Keyword && s.bytes(k).eq_ignore_ascii_case(b"array") {
            return true;
        }
        if s.kind(k) == Kind::Punct && s.bytes(k) == b")" {
            nesting -= 1;
        } else if s.kind(k) == Kind::Punct && s.bytes(k) == b"(" {
            if nesting != 0 {
                nesting += 1;
            } else {
                return true;
            }
        }
        i -= 1;
    }
    false
}

fn method_chaining_newline(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 1;
    while i < s.len() {
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"->" {
            i += 1;
            continue;
        }
        let prev = match sig_prev(s, i) {
            Some(p) if s.kind(p) == Kind::Punct && s.bytes(p) == b")" => p,
            _ => {
                i += 1;
                continue;
            }
        };
        if !chain_is_call_close(s, prev) {
            i += 1;
            continue;
        }
        if range_has_newline(s, prev, i) {
            i += 1;
            continue;
        }
        if let Some(open) = match_backward(s, prev) {
            if range_has_newline(s, open, prev) {
                i += 1;
                continue;
            }
        }
        if chain_line_has_breaking_char(s, prev) {
            i += 1;
            continue;
        }
        let mut nl = b"\n".to_vec();
        nl.extend_from_slice(&chain_first_line_indent(s, i));
        nl.extend_from_slice(b"    ");
        if s.kind(i - 1) == Kind::Whitespace {
            s.set_owned(i - 1, nl);
        } else {
            s.insert_owned(i, Kind::Whitespace, nl);
            i += 1;
        }
        changed = true;
        i += 1;
    }
    changed
}

// --- ported batch: phpdoc infrastructure + content rules --------------------

struct DocLine {
    prefix: Vec<u8>,
    content: Vec<u8>,
}

struct Doc {
    open: Vec<u8>,
    inner: Vec<DocLine>,
    close: Vec<u8>,
    single: bool,
}

fn trim_left_space(b: &[u8]) -> &[u8] {
    let mut i = 0;
    while i < b.len() && b[i] == b' ' {
        i += 1;
    }
    &b[i..]
}

fn parse_doc(v: &[u8]) -> Option<Doc> {
    if !v.starts_with(b"/**") || !v.ends_with(b"*/") || v.len() < 5 {
        return None;
    }
    if !v.contains(&b'\n') {
        let body = trim_go_space(&v[3..v.len() - 2]).to_vec();
        return Some(Doc {
            open: b"/**".to_vec(),
            inner: vec![DocLine { prefix: Vec::new(), content: body }],
            close: b"*/".to_vec(),
            single: true,
        });
    }
    let lines: Vec<&[u8]> = v.split(|&c| c == b'\n').collect();
    if lines.len() < 3 {
        return None;
    }
    let mut d = Doc {
        open: lines[0].to_vec(),
        inner: Vec::new(),
        close: lines[lines.len() - 1].to_vec(),
        single: false,
    };
    for line in &lines[1..lines.len() - 1] {
        let star = match line.iter().position(|&c| c == b'*') {
            Some(x) => x,
            None => return None,
        };
        let mut prefix = line[..star + 1].to_vec();
        let mut rest = &line[star + 1..];
        if rest.first() == Some(&b' ') {
            prefix.push(b' ');
            rest = &rest[1..];
        }
        d.inner.push(DocLine { prefix, content: rest.to_vec() });
    }
    Some(d)
}

fn doc_render(d: &Doc) -> Vec<u8> {
    if d.single {
        if d.inner.is_empty() || d.inner[0].content.is_empty() {
            return b"/** */".to_vec();
        }
        let mut out = b"/** ".to_vec();
        out.extend_from_slice(&d.inner[0].content);
        out.extend_from_slice(b" */");
        return out;
    }
    let mut out = d.open.clone();
    for l in &d.inner {
        out.push(b'\n');
        if l.content.is_empty() {
            let mut p = l.prefix.clone();
            while p.last() == Some(&b' ') {
                p.pop();
            }
            out.extend_from_slice(&p);
        } else {
            out.extend_from_slice(&l.prefix);
            out.extend_from_slice(&l.content);
        }
    }
    out.push(b'\n');
    out.extend_from_slice(&d.close);
    out
}

fn apply_to_docblocks<F: Fn(&mut Doc) -> bool>(s: &mut Stream, f: F) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::DocComment {
            continue;
        }
        let mut d = match parse_doc(s.bytes(i)) {
            Some(d) => d,
            None => continue,
        };
        if f(&mut d) {
            let r = doc_render(&d);
            s.set_owned(i, r);
            changed = true;
        }
    }
    changed
}

// Match the phpdoc type-tag regex on a leading-space-trimmed content.
// Returns (m1_len, type_start, type_end): m1 = "@tag" + whitespace,
// type = trimmed[type_start..type_end] (non-space), rest = trimmed[type_end..].
fn match_type_tag(t: &[u8]) -> Option<(usize, usize, usize)> {
    if t.first() != Some(&b'@') {
        return None;
    }
    let mut i = 1;
    while i < t.len() && (t[i].is_ascii_alphabetic() || t[i] == b'-') {
        i += 1;
    }
    let name = t[1..i].to_ascii_lowercase();
    let ok = matches!(
        name.as_slice(),
        b"param" | b"return" | b"var" | b"throws" | b"property"
            | b"property-read" | b"property-write" | b"method"
    );
    if !ok {
        return None;
    }
    let ws_start = i;
    while i < t.len() && (t[i] == b' ' || t[i] == b'\t') {
        i += 1;
    }
    if i == ws_start {
        return None; // needs \s+
    }
    let m1_len = i;
    let type_start = i;
    while i < t.len() && t[i] != b' ' && t[i] != b'\t' {
        i += 1;
    }
    if i == type_start {
        return None; // \S+
    }
    Some((m1_len, type_start, i))
}

fn split_type_parts(typ: &[u8], map: fn(&[u8]) -> Option<Vec<u8>>) -> Vec<u8> {
    let nullable = typ.first() == Some(&b'?');
    let body = if nullable { &typ[1..] } else { typ };
    let parts: Vec<&[u8]> = body.split(|&c| c == b'|').collect();
    let mut out_parts: Vec<Vec<u8>> = Vec::new();
    for p in parts {
        let mut base = p;
        let mut suffix: Vec<u8> = Vec::new();
        while base.ends_with(b"[]") {
            base = &base[..base.len() - 2];
            let mut ns = b"[]".to_vec();
            ns.extend_from_slice(&suffix);
            suffix = ns;
        }
        if let Some(repl) = map(base) {
            let mut np = repl;
            np.extend_from_slice(&suffix);
            out_parts.push(np);
        } else {
            out_parts.push(p.to_vec());
        }
    }
    let mut out = Vec::new();
    if nullable {
        out.push(b'?');
    }
    for (i, p) in out_parts.iter().enumerate() {
        if i > 0 {
            out.push(b'|');
        }
        out.extend_from_slice(p);
    }
    out
}

fn scalar_map(base: &[u8]) -> Option<Vec<u8>> {
    match base {
        b"boolean" => Some(b"bool".to_vec()),
        b"integer" => Some(b"int".to_vec()),
        b"double" => Some(b"float".to_vec()),
        b"real" => Some(b"float".to_vec()),
        b"str" => Some(b"string".to_vec()),
        b"callback" => Some(b"callable".to_vec()),
        _ => None,
    }
}

fn is_phpdoc_type_keyword(low: &[u8]) -> bool {
    matches!(
        low,
        b"array" | b"bool" | b"callable" | b"false" | b"float" | b"int" | b"iterable"
            | b"mixed" | b"null" | b"object" | b"parent" | b"self" | b"static"
            | b"string" | b"true" | b"void" | b"never" | b"$this"
    )
}

fn typecase_map(base: &[u8]) -> Option<Vec<u8>> {
    let low = base.to_ascii_lowercase();
    if base != low.as_slice() && is_phpdoc_type_keyword(low.as_slice()) {
        Some(low)
    } else {
        None
    }
}

// shared for phpdoc_scalar / phpdoc_types: rewrite the type in each type-tag line
fn rewrite_type_lines(d: &mut Doc, map: fn(&[u8]) -> Option<Vec<u8>>) -> bool {
    let mut changed = false;
    for l in d.inner.iter_mut() {
        let lead = l.content.len() - trim_left_space(&l.content).len();
        let trimmed = l.content[lead..].to_vec();
        let (m1_len, ts, te) = match match_type_tag(&trimmed) {
            Some(x) => x,
            None => continue,
        };
        let old_type = &trimmed[ts..te];
        let new_type = split_type_parts(old_type, map);
        if new_type.as_slice() == old_type {
            continue;
        }
        let mut nc = l.content[..lead].to_vec();
        nc.extend_from_slice(&trimmed[..m1_len]);
        nc.extend_from_slice(&new_type);
        nc.extend_from_slice(&trimmed[te..]);
        l.content = nc;
        changed = true;
    }
    changed
}

fn phpdoc_scalar(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| rewrite_type_lines(d, scalar_map))
}

fn phpdoc_types(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| rewrite_type_lines(d, typecase_map))
}

fn phpdoc_trim(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| {
        let before = d.inner.len();
        while !d.inner.is_empty() && trim_go_space(&d.inner[0].content).is_empty() {
            d.inner.remove(0);
        }
        while !d.inner.is_empty() && trim_go_space(&d.inner[d.inner.len() - 1].content).is_empty() {
            d.inner.pop();
        }
        d.inner.len() != before
    })
}

fn empty_return_match(trimmed: &[u8]) -> bool {
    // (?i)^@return\s+(void|null)($|\s)
    let p = b"@return";
    if trimmed.len() < p.len() || !trimmed[..p.len()].eq_ignore_ascii_case(p) {
        return false;
    }
    let mut i = p.len();
    let ws_start = i;
    while i < trimmed.len() && (trimmed[i] == b' ' || trimmed[i] == b'\t') {
        i += 1;
    }
    if i == ws_start {
        return false;
    }
    let word_start = i;
    while i < trimmed.len() && trimmed[i] != b' ' && trimmed[i] != b'\t' {
        i += 1;
    }
    let word = trimmed[word_start..i].to_ascii_lowercase();
    word == b"void" || word == b"null"
}

fn phpdoc_no_empty_return(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| {
        let mut kept: Vec<DocLine> = Vec::new();
        let mut removed = false;
        for l in d.inner.drain(..) {
            if empty_return_match(trim_go_space(&l.content)) {
                removed = true;
                continue;
            }
            kept.push(l);
        }
        d.inner = kept;
        removed
    })
}

fn docblock_is_empty(v: &[u8]) -> bool {
    if !v.starts_with(b"/**") || !v.ends_with(b"*/") || v.len() < 5 {
        return false;
    }
    v[3..v.len() - 2]
        .iter()
        .all(|&c| matches!(c, b'*' | b' ' | b'\t' | b'\n' | b'\r'))
}

fn no_empty_phpdoc(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::DocComment || !docblock_is_empty(s.bytes(i)) {
            i += 1;
            continue;
        }
        s.remove_at(i);
        changed = true;
        if i < s.len() && s.kind(i) == Kind::Whitespace && s.bytes(i).first() == Some(&b'\n') {
            let v = s.bytes(i)[1..].to_vec();
            if v.is_empty() {
                s.remove_at(i);
            } else {
                s.set_owned(i, v);
            }
        }
        // i stays (Go does i--; loop ++)
    }
    changed
}

// alias-tag helper: returns tag name if content (trimleft) starts with @<name>\b
// where name in the given set.
fn tag_word_at(t: &[u8]) -> Option<Vec<u8>> {
    if t.first() != Some(&b'@') {
        return None;
    }
    let mut i = 1;
    while i < t.len() && (t[i].is_ascii_alphanumeric() || t[i] == b'-' || t[i] == b'_') {
        i += 1;
    }
    if i == 1 {
        return None;
    }
    Some(t[1..i].to_vec())
}

fn phpdoc_no_alias_tag(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| {
        let mut changed = false;
        for l in d.inner.iter_mut() {
            let lead = l.content.len() - trim_left_space(&l.content).len();
            let trimmed = &l.content[lead..];
            let name = match tag_word_at(trimmed) {
                Some(n) => n,
                None => continue,
            };
            let repl: &[u8] = match name.as_slice() {
                b"type" => b"var",
                b"link" => b"see",
                _ => continue,
            };
            // @<name> is 1 + name.len() bytes
            let after = &trimmed[1 + name.len()..];
            let mut nc = l.content[..lead].to_vec();
            nc.push(b'@');
            nc.extend_from_slice(repl);
            nc.extend_from_slice(after);
            l.content = nc;
            changed = true;
        }
        changed
    })
}

fn remove_doc_lines_by_tag(d: &mut Doc, names: &[&[u8]]) -> bool {
    let mut kept: Vec<DocLine> = Vec::new();
    let mut removed = false;
    for l in d.inner.drain(..) {
        let trimmed = trim_left_space(&l.content);
        let matched = match tag_word_at(trimmed) {
            Some(n) => names.iter().any(|x| x == &n.as_slice()),
            None => false,
        };
        if matched {
            removed = true;
            continue;
        }
        kept.push(l);
    }
    d.inner = kept;
    removed
}

fn phpdoc_no_package(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| remove_doc_lines_by_tag(d, &[b"package", b"subpackage"]))
}

fn phpdoc_no_access(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| remove_doc_lines_by_tag(d, &[b"access"]))
}

fn collapse_ws_to_single(b: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(b.len());
    let mut in_ws = false;
    for &c in b {
        if c.is_ascii_whitespace() {
            if !in_ws {
                out.push(b' ');
                in_ws = true;
            }
        } else {
            out.push(c);
            in_ws = false;
        }
    }
    out
}

fn phpdoc_single_line_var_spacing(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| {
        if !d.single || d.inner.is_empty() {
            return false;
        }
        let content = d.inner[0].content.clone();
        // ^(@(?:var|type|param))\s+([^\s$]+)\s*(\$\S+)?\s*(.*)$
        if content.first() != Some(&b'@') {
            return false;
        }
        let mut i = 1;
        while i < content.len() && content[i].is_ascii_alphabetic() {
            i += 1;
        }
        let tag = content[..i].to_vec();
        if !matches!(tag.as_slice(), b"@var" | b"@type" | b"@param") {
            return false;
        }
        let ws1 = i;
        while i < content.len() && content[i].is_ascii_whitespace() {
            i += 1;
        }
        if i == ws1 {
            return false;
        }
        let type_start = i;
        while i < content.len() && !content[i].is_ascii_whitespace() && content[i] != b'$' {
            i += 1;
        }
        if i == type_start {
            return false;
        }
        let type_b = content[type_start..i].to_vec();
        while i < content.len() && content[i].is_ascii_whitespace() {
            i += 1;
        }
        let mut var_b: Vec<u8> = Vec::new();
        if i < content.len() && content[i] == b'$' {
            let vs = i;
            while i < content.len() && !content[i].is_ascii_whitespace() {
                i += 1;
            }
            var_b = content[vs..i].to_vec();
        }
        while i < content.len() && content[i].is_ascii_whitespace() {
            i += 1;
        }
        let rest = &content[i..];
        let mut normalized = tag;
        normalized.push(b' ');
        normalized.extend_from_slice(&type_b);
        if !var_b.is_empty() {
            normalized.push(b' ');
            normalized.extend_from_slice(&var_b);
        }
        let rest_norm = trim_go_space(&collapse_ws_to_single(rest)).to_vec();
        if !rest_norm.is_empty() {
            normalized.push(b' ');
            normalized.extend_from_slice(&rest_norm);
        }
        if normalized == content {
            return false;
        }
        d.inner[0].content = normalized;
        true
    })
}

fn phpdoc_trim_consecutive_blank_line_separation(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| {
        let mut kept: Vec<DocLine> = Vec::new();
        let mut prev_blank = false;
        let mut changed = false;
        for l in d.inner.drain(..) {
            let blank = trim_go_space(&l.content).is_empty();
            if blank && prev_blank {
                changed = true;
                continue;
            }
            prev_blank = blank;
            kept.push(l);
        }
        d.inner = kept;
        changed
    })
}

fn no_blank_lines_after_phpdoc(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::DocComment {
            continue;
        }
        let j = i + 1;
        if j >= s.len() || s.kind(j) != Kind::Whitespace {
            continue;
        }
        let v = s.bytes(j);
        if v.iter().filter(|&&c| c == b'\n').count() < 2 {
            continue;
        }
        let last = v.iter().rposition(|&c| c == b'\n').unwrap();
        let nv = v[last..].to_vec();
        if nv.as_slice() != s.bytes(j) {
            s.set_owned(j, nv);
            changed = true;
        }
    }
    changed
}
