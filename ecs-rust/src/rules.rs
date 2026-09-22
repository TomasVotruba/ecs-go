// The ported PSR-12 rule subset, applied in the same order ecs-go's rules.All()
// yields them (Casing before Spacing), so the Rust output matches the Go output
// for the matching `--rules` subset. Each rule mirrors the PHP-CS-Fixer fixer of
// the same name.

use crate::stream::Stream;
use crate::token::Kind;

// FQCNs of the ported fixers, matching ecs-go's names. Used to build the Go-side
// `--rules` subset for a fair, identical-work comparison.
pub const RULE_NAMES: &[&str] = &[
    r"PhpCsFixer\Fixer\ClassNotation\ProtectedToPrivateFixer",
    r"PhpCsFixer\Fixer\Alias\NoAliasFunctionsFixer",
    r"PhpCsFixer\Fixer\Operator\IncrementStyleFixer",
    r"PhpCsFixer\Fixer\LanguageConstruct\FunctionToConstantFixer",
    r"PhpCsFixer\Fixer\Alias\NoAliasLanguageConstructCallFixer",
    r"PhpCsFixer\Fixer\Alias\NoMixedEchoPrintFixer",
    r"PhpCsFixer\Fixer\Operator\NotOperatorWithSuccessorSpaceFixer",
    r"PhpCsFixer\Fixer\ClassNotation\SelfStaticAccessorFixer",
    r"PhpCsFixer\Fixer\ClassNotation\SelfAccessorFixer",
    r"PhpCsFixer\Fixer\Basic\SingleLineEmptyBodyFixer",
    r"PhpCsFixer\Fixer\Basic\EncodingFixer",
    r"PhpCsFixer\Fixer\PhpTag\FullOpeningTagFixer",
    r"PhpCsFixer\Fixer\ClassNotation\OrderedClassElementsFixer",
    r"PhpCsFixer\Fixer\ClassNotation\SingleClassElementPerStatementFixer",
    r"PhpCsFixer\Fixer\ClassNotation\ClassAttributesSeparationFixer",
    r"PhpCsFixer\Fixer\Whitespace\IndentationTypeFixer",
    r"PhpCsFixer\Fixer\Semicolon\NoEmptyStatementFixer",
    r"PhpCsFixer\Fixer\StringNotation\NoBinaryStringFixer",
    r"PhpCsFixer\Fixer\ControlStructure\ElseifFixer",
    r"PhpCsFixer\Fixer\ControlStructure\NoSuperfluousElseifFixer",
    r"PhpCsFixer\Fixer\ControlStructure\ControlStructureBracesFixer",
    r"PhpCsFixer\Fixer\ControlStructure\NoAlternativeSyntaxFixer",
    r"Symplify\CodingStandard\Fixer\Spacing\StandaloneLinePromotedPropertyFixer",
    r"Symplify\CodingStandard\Fixer\Spacing\StandaloneLinePlainConstructorParamFixer",
    r"Symplify\CodingStandard\Fixer\Spacing\StandaloneLineRequiredParamFixer",
    r"Symplify\CodingStandard\Fixer\Spacing\StandaloneLineSymfonyAttributeParamFixer",
    r"PhpCsFixer\Fixer\ControlStructure\NoBreakCommentFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocSummaryFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocTagTypeFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocOrderFixer",
    r"PhpCsFixer\Fixer\Phpdoc\GeneralPhpdocAnnotationRemoveFixer",
    r"PhpCsFixer\Fixer\Whitespace\MethodChainingIndentationFixer",
    r"PhpCsFixer\Fixer\Semicolon\MultilineWhitespaceBeforeSemicolonsFixer",
    r"Symplify\CodingStandard\Fixer\Commenting\AddMissingParamNameFixer",
    r"PhpCsFixer\Fixer\ClassNotation\OrderedTypesFixer",
    r"PhpCsFixer\Fixer\ClassNotation\OrderedInterfacesFixer",
    r"PhpCsFixer\Fixer\ClassNotation\OrderedTraitsFixer",
    r"PhpCsFixer\Fixer\DoctrineAnnotation\DoctrineAnnotationSpacesFixer",
    r"Symplify\CodingStandard\Fixer\Commenting\RemoveUselessDefaultCommentFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocSeparationFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocToCommentFixer",
    r"PhpCsFixer\Fixer\DoctrineAnnotation\DoctrineAnnotationArrayAssignmentFixer",
    r"PhpCsFixer\Fixer\DoctrineAnnotation\DoctrineAnnotationIndentationFixer",
    r"Symplify\CodingStandard\Fixer\Commenting\ParamReturnAndVarTagMalformsFixer",
    r"Symplify\CodingStandard\Fixer\Commenting\DoubleAsteriskInlineVarFixer",
    r"Symplify\CodingStandard\Fixer\Commenting\FixTagTypoFixer",
    r"Symplify\CodingStandard\Fixer\Commenting\TypeToVarTagFixer",
    r"Symplify\CodingStandard\Fixer\Commenting\MergeDocBlockStartFixer",
    r"Symplify\CodingStandard\Fixer\Commenting\AddMissingVarNameFixer",
    r"Symplify\CodingStandard\Fixer\Commenting\SingleLineInlineVarDocBlockFixer",
    r"Symplify\CodingStandard\Fixer\Commenting\RemoveSuperfluousReturnNameFixer",
    r"Symplify\CodingStandard\Fixer\Commenting\RemoveSuperfluousVarNameFixer",
    r"Symplify\CodingStandard\Fixer\Commenting\FixParamNameTypoFixer",
    r"PhpCsFixer\Fixer\Phpdoc\GeneralPhpdocTagRenameFixer",
    r"PhpCsFixer\Fixer\FunctionNotation\LambdaNotUsedImportFixer",
    r"PhpCsFixer\Fixer\Import\FullyQualifiedStrictTypesFixer",
    r"Symplify\CodingStandard\Fixer\ArrayNotation\ArrayListItemNewlineFixer",
    r"Symplify\CodingStandard\Fixer\ArrayNotation\StandaloneLineInMultilineArrayFixer",
    r"PhpCsFixer\Fixer\ControlStructure\EmptyLoopBodyFixer",
    r"Symplify\CodingStandard\Fixer\Spacing\MethodChainingNewlineFixer",
    r"PhpCsFixer\Fixer\Operator\NewWithParenthesesFixer",
    r"PhpCsFixer\Fixer\ArrayNotation\ArraySyntaxFixer",
    r"PhpCsFixer\Fixer\LanguageConstruct\SingleSpaceAroundConstructFixer",
    r"PhpCsFixer\Fixer\ClassNotation\ClassDefinitionFixer",
    r"PhpCsFixer\Fixer\ClassNotation\SingleTraitInsertPerStatementFixer",
    r"PhpCsFixer\Fixer\ArrayNotation\NoMultilineWhitespaceAroundDoubleArrowFixer",
    r"PhpCsFixer\Fixer\FunctionNotation\FunctionDeclarationFixer",
    r"Symplify\CodingStandard\Fixer\Commenting\RemoveDeadParamFixer",
    r"Symplify\CodingStandard\Fixer\Commenting\RemoveDeadVarThisFixer",
    r"Symplify\CodingStandard\Fixer\Commenting\RemoveParamNameReferenceFixer",
    r"Symplify\CodingStandard\Fixer\Commenting\SwitchedTypeAndNameFixer",
    r"Symplify\CodingStandard\Fixer\Annotation\RemovePHPStormAnnotationFixer",
    r"Symplify\CodingStandard\Fixer\Annotation\RemovePropertyVariableNameDescriptionFixer",
    r"Symplify\CodingStandard\Fixer\Annotation\RemoveMethodNameDuplicateDescriptionFixer",
    r"Symplify\CodingStandard\Fixer\Annotation\RemoveEventSubscriberDescriptionFixer",
    r"PhpCsFixer\Fixer\PhpUnit\PhpUnitMethodCasingFixer",
    r"PhpCsFixer\Fixer\PhpUnit\PhpUnitSetUpTearDownVisibilityFixer",
    r"PhpCsFixer\Fixer\Operator\OperatorLinebreakFixer",
    r"PhpCsFixer\Fixer\FunctionNotation\NoUnreachableDefaultArgumentValueFixer",
    r"PhpCsFixer\Fixer\FunctionNotation\MethodArgumentSpaceFixer",
    r"PhpCsFixer\Fixer\Whitespace\ArrayIndentationFixer",
    r"PhpCsFixer\Fixer\Phpdoc\AlignMultilineCommentFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocIndentFixer",
    r"PhpCsFixer\Fixer\Operator\LongToShorthandOperatorFixer",
    r"PhpCsFixer\Fixer\Operator\StandardizeIncrementFixer",
    r"PhpCsFixer\Fixer\StringNotation\SingleQuoteFixer",
    r"PhpCsFixer\Fixer\NamespaceNotation\CleanNamespaceFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocReturnSelfReferenceFixer",
    r"PhpCsFixer\Fixer\StringNotation\ExplicitStringVariableFixer",
    r"PhpCsFixer\Fixer\Phpdoc\NoSuperfluousPhpdocTagsFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocNoUselessInheritdocFixer",
    r"PhpCsFixer\Fixer\Operator\NoUselessConcatOperatorFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocLineSpanFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocNoEmptyReturnFixer",
    r"PhpCsFixer\Fixer\FunctionNotation\NullableTypeDeclarationForDefaultNullValueFixer",
    r"PhpCsFixer\Fixer\Phpdoc\NoEmptyPhpdocFixer",
    r"PhpCsFixer\Fixer\FunctionNotation\NoSpacesAfterFunctionNameFixer",
    r"PhpCsFixer\Fixer\Whitespace\SpacesInsideParenthesesFixer",
    r"PhpCsFixer\Fixer\ListNotation\ListSyntaxFixer",
    r"PhpCsFixer\Fixer\Comment\NoEmptyCommentFixer",
    r"PhpCsFixer\Fixer\LanguageConstruct\IsNullFixer",
    r"PhpCsFixer\Fixer\Comment\SingleLineCommentSpacingFixer",
    r"PhpCsFixer\Fixer\Operator\NoSpaceAroundDoubleColonFixer",
    r"PhpCsFixer\Fixer\Import\NoUnneededImportAliasFixer",
    r"PhpCsFixer\Fixer\ControlStructure\EmptyLoopConditionFixer",
    r"PhpCsFixer\Fixer\Operator\TernaryOperatorSpacesFixer",
    r"PhpCsFixer\Fixer\PhpTag\BlankLineAfterOpeningTagFixer",
    r"PhpCsFixer\Fixer\Import\SingleImportPerStatementFixer",
    r"PhpCsFixer\Fixer\Whitespace\LineEndingFixer",
    r"PhpCsFixer\Fixer\ControlStructure\YodaStyleFixer",
    r"PhpCsFixer\Fixer\ArrayNotation\NoWhitespaceBeforeCommaInArrayFixer",
    r"PhpCsFixer\Fixer\ArrayNotation\WhitespaceAfterCommaInArrayFixer",
    r"PhpCsFixer\Fixer\ControlStructure\TrailingCommaInMultilineFixer",
    r"PhpCsFixer\Fixer\Basic\NoTrailingCommaInSinglelineFixer",
    r"PhpCsFixer\Fixer\Whitespace\NoSpacesAroundOffsetFixer",
    r"PhpCsFixer\Fixer\Operator\ObjectOperatorWithoutWhitespaceFixer",
    r"PhpCsFixer\Fixer\Operator\NoUselessNullsafeOperatorFixer",
    r"PhpCsFixer\Fixer\Operator\StandardizeNotEqualsFixer",
    r"PhpCsFixer\Fixer\Operator\TernaryToNullCoalescingFixer",
    r"PhpCsFixer\Fixer\ArrayNotation\TrimArraySpacesFixer",
    r"PhpCsFixer\Fixer\AttributeNotation\AttributeBlockNoSpacesFixer",
    r"PhpCsFixer\Fixer\StringNotation\HeredocToNowdocFixer",
    r"PhpCsFixer\Fixer\CastNotation\NoUnsetCastFixer",
    r"PhpCsFixer\Fixer\ArrayNotation\NoWhitespaceInEmptyArrayFixer",
    r"PhpCsFixer\Fixer\ArrayNotation\NormalizeIndexBraceFixer",
    r"PhpCsFixer\Fixer\ControlStructure\SwitchContinueToBreakFixer",
    r"PhpCsFixer\Fixer\ControlStructure\NoUnneededCurlyBracesFixer",
    r"PhpCsFixer\Fixer\Comment\MultilineCommentOpeningClosingFixer",
    r"PhpCsFixer\Fixer\LanguageConstruct\DeclareParenthesesFixer",
    r"PhpCsFixer\Fixer\Whitespace\TypeDeclarationSpacesFixer",
    r"PhpCsFixer\Fixer\Whitespace\CompactNullableTypeDeclarationFixer",
    r"PhpCsFixer\Fixer\LanguageConstruct\ExplicitIndirectVariableFixer",
    r"PhpCsFixer\Fixer\ClassNotation\NoNullPropertyInitializationFixer",
    r"PhpCsFixer\Fixer\ControlStructure\IncludeFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocScalarFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocTypesFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocNoAliasTagFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocNoPackageFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocNoAccessFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocTagCasingFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocInlineTagNormalizerFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocNoDuplicateTypesFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocVarWithoutNameFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocTypesOrderFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocVarAnnotationCorrectOrderFixer",
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
    r"PhpCsFixer\Fixer\Operator\ConcatSpaceFixer",
    r"PhpCsFixer\Fixer\Whitespace\NoTrailingWhitespaceFixer",
    r"PhpCsFixer\Fixer\Comment\NoTrailingWhitespaceInCommentFixer",
    r"PhpCsFixer\Fixer\LanguageConstruct\DeclareEqualNormalizeFixer",
    r"PhpCsFixer\Fixer\Operator\UnaryOperatorSpacesFixer",
    r"PhpCsFixer\Fixer\ControlStructure\SwitchCaseSemicolonToColonFixer",
    r"PhpCsFixer\Fixer\ControlStructure\SwitchCaseSpaceFixer",
    r"PhpCsFixer\Fixer\ClassNotation\VisibilityRequiredFixer",
    r"PhpCsFixer\Fixer\ClassNotation\ModifierKeywordsFixer",
    r"PhpCsFixer\Fixer\ClassNotation\NoBlankLinesAfterClassOpeningFixer",
    r"PhpCsFixer\Fixer\PhpTag\NoClosingTagFixer",
    r"PhpCsFixer\Fixer\Whitespace\TypesSpacesFixer",
    r"PhpCsFixer\Fixer\Operator\AssignNullCoalescingToCoalesceEqualFixer",
    r"PhpCsFixer\Fixer\Semicolon\SpaceAfterSemicolonFixer",
    r"PhpCsFixer\Fixer\Basic\NoMultipleStatementsPerLineFixer",
    r"PhpCsFixer\Fixer\Basic\BracesPositionFixer",
    r"PhpCsFixer\Fixer\Whitespace\StatementIndentationFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocTrimFixer",
    r"PhpCsFixer\Fixer\CastNotation\NoShortBoolCastFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocSingleLineVarSpacingFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocOrderByValueFixer",
    r"PhpCsFixer\Fixer\CastNotation\CastSpacesFixer",
    r"PhpCsFixer\Fixer\Import\NoUnusedImportsFixer",
    r"PhpCsFixer\Fixer\Import\SingleLineAfterImportsFixer",
    r"Symplify\CodingStandard\Fixer\Spacing\NoBlankLineBetweenImportsFixer",
    r"Symplify\CodingStandard\Fixer\Spacing\SpaceAfterCommaHereNowDocFixer",
    r"Symplify\CodingStandard\Fixer\ArrayNotation\ArrayOpenerAndCloserNewlineFixer",
    r"PhpCsFixer\Fixer\FunctionNotation\ReturnTypeDeclarationFixer",
    r"PhpCsFixer\Fixer\Phpdoc\NoBlankLinesAfterPhpdocFixer",
    r"PhpCsFixer\Fixer\Import\NoLeadingImportSlashFixer",
    r"PhpCsFixer\Fixer\NamespaceNotation\BlankLineAfterNamespaceFixer",
    r"PhpCsFixer\Fixer\Whitespace\NoExtraBlankLinesFixer",
    r"PhpCsFixer\Fixer\Import\OrderedImportsFixer",
    r"PhpCsFixer\Fixer\Comment\SingleLineCommentStyleFixer",
    r"PhpCsFixer\Fixer\NamespaceNotation\BlankLinesBeforeNamespaceFixer",
    r"PhpCsFixer\Fixer\Operator\BinaryOperatorSpacesFixer",
    r"PhpCsFixer\Fixer\Whitespace\BlankLineBetweenImportGroupsFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocTrimConsecutiveBlankLineSeparationFixer",
    r"PhpCsFixer\Fixer\Phpdoc\PhpdocAlignFixer",
    r"PhpCsFixer\Fixer\Whitespace\NoWhitespaceInBlankLineFixer",
    r"PhpCsFixer\Fixer\Whitespace\SingleBlankLineAtEofFixer",
    r"PhpCsFixer\Fixer\ControlStructure\ControlStructureContinuationPositionFixer",
    r"PhpCsFixer\Fixer\ControlStructure\NoUnneededControlParenthesesFixer",
    r"PhpCsFixer\Fixer\NamespaceNotation\SingleBlankLineBeforeNamespaceFixer",
    r"PhpCsFixer\Fixer\PhpTag\LinebreakAfterOpeningTagFixer",
    r"PhpCsFixer\Fixer\ReturnNotation\NoUselessReturnFixer",
    r"PhpCsFixer\Fixer\ReturnNotation\SimplifiedNullReturnFixer",
    r"PhpCsFixer\Fixer\Whitespace\BlankLineBeforeStatementFixer",
    r"Symplify\CodingStandard\Fixer\Strict\BlankLineAfterStrictTypesFixer",
    r"PhpCsFixer\Fixer\LanguageConstruct\NullableTypeDeclarationFixer",
];

// Rules run in ecs-go's rules.All() order, restricted to the ported set, so the
// Rust output matches the Go output for the matching `--rules` subset.
pub fn fix(s: &mut Stream) -> bool {
    let mut changed = false;
    changed |= encoding(s);
    changed |= full_opening_tag(s);
    changed |= protected_to_private(s);
    changed |= ordered_class_elements(s);
    changed |= single_class_element_per_statement(s);
    changed |= class_attributes_separation(s);
    changed |= indentation_type(s);
    changed |= no_empty_statement(s);
    changed |= no_binary_string(s);
    changed |= no_alternative_syntax(s);
    changed |= remove_useless_default_comment(s);
    changed |= elseif(s);
    changed |= no_superfluous_elseif(s);
    changed |= control_structure_braces(s);
    changed |= standalone_line_promoted_property(s);
    changed |= standalone_line_plain_constructor_param(s);
    changed |= standalone_line_required_param(s);
    changed |= standalone_line_symfony_attribute_param(s);
    changed |= no_break_comment(s);
    changed |= phpdoc_separation(s);
    changed |= phpdoc_summary(s);
    changed |= phpdoc_tag_type(s);
    changed |= phpdoc_to_comment(s);
    changed |= param_return_and_var_tag_malforms(s);
    changed |= general_phpdoc_tag_rename(s);
    changed |= lambda_not_used_import(s);
    changed |= array_list_item_newline(s);
    changed |= empty_loop_body(s);
    changed |= method_chaining_newline(s);
    changed |= new_with_parentheses(s);
    changed |= array_syntax(s);
    changed |= single_space_around_construct(s);
    changed |= class_definition(s);
    changed |= single_trait_insert_per_statement(s);
    changed |= no_multiline_whitespace_around_double_arrow(s);
    changed |= function_declaration(s);
    changed |= no_unneeded_control_parentheses(s);
    changed |= add_missing_param_name(s);
    changed |= no_unreachable_default_argument_value(s);
    changed |= method_argument_space(s);
    changed |= array_opener_and_closer_newline(s);
    changed |= array_indentation(s);
    changed |= standalone_line_in_multiline_array(s);
    changed |= align_multiline_comment(s);
    changed |= phpdoc_indent(s);
    changed |= long_to_shorthand_operator(s);
    changed |= standardize_increment(s);
    changed |= increment_style(s);
    changed |= simplified_null_return(s);
    changed |= single_quote(s);
    changed |= clean_namespace(s);
    changed |= phpdoc_return_self_reference(s);
    changed |= fully_qualified_strict_types(s);
    changed |= explicit_string_variable(s);
    changed |= no_superfluous_phpdoc_tags(s);
    changed |= phpdoc_no_useless_inheritdoc(s);
    changed |= no_useless_concat_operator(s);
    changed |= phpdoc_line_span(s);
    changed |= phpdoc_no_empty_return(s);
    changed |= nullable_type_declaration_for_default_null_value(s);
    changed |= no_empty_phpdoc(s);
    changed |= remove_phpstorm_annotation(s);
    changed |= remove_event_subscriber_description(s);
    changed |= remove_method_name_duplicate_description(s);
    changed |= remove_property_variable_name_description(s);
    changed |= no_spaces_after_function_name(s);
    changed |= spaces_inside_parentheses(s);
    changed |= list_syntax(s);
    changed |= no_empty_comment(s);
    changed |= function_to_constant(s);
    changed |= nullable_type_declaration(s);
    changed |= linebreak_after_opening_tag(s);
    changed |= is_null(s);
    changed |= single_line_comment_spacing(s);
    changed |= no_space_around_double_colon(s);
    changed |= no_unneeded_import_alias(s);
    changed |= empty_loop_condition(s);
    changed |= ternary_operator_spaces(s);
    changed |= blank_line_after_opening_tag(s);
    changed |= single_import_per_statement(s);
    changed |= line_ending(s);
    changed |= doctrine_annotation_array_assignment(s);
    changed |= doctrine_annotation_spaces(s);
    changed |= doctrine_annotation_indentation(s);
    changed |= yoda_style(s);
    changed |= no_whitespace_before_comma_in_array(s);
    changed |= whitespace_after_comma_in_array(s);
    changed |= trailing_comma_in_multiline(s);
    changed |= no_trailing_comma_in_singleline(s);
    changed |= no_spaces_around_offset(s);
    changed |= object_operator_without_whitespace(s);
    changed |= no_useless_nullsafe_operator(s);
    changed |= standardize_not_equals(s);
    changed |= ternary_to_null_coalescing(s);
    changed |= trim_array_spaces(s);
    changed |= attribute_block_no_spaces(s);
    changed |= heredoc_to_nowdoc(s);
    changed |= no_unset_cast(s);
    changed |= no_whitespace_in_empty_array(s);
    changed |= normalize_index_brace(s);
    changed |= switch_continue_to_break(s);
    changed |= no_unneeded_braces(s);
    changed |= no_alias_functions(s);
    changed |= multiline_comment_opening_closing(s);
    changed |= declare_parentheses(s);
    changed |= type_declaration_spaces(s);
    changed |= compact_nullable_type_declaration(s);
    changed |= ordered_types(s);
    changed |= explicit_indirect_variable(s);
    changed |= no_null_property_initialization(s);
    changed |= include(s);
    changed |= no_alias_language_construct_call(s);
    changed |= phpdoc_scalar(s);
    changed |= general_phpdoc_annotation_remove(s);
    changed |= phpdoc_types(s);
    changed |= phpdoc_no_alias_tag(s);
    changed |= phpdoc_no_package(s);
    changed |= phpdoc_no_access(s);
    changed |= phpdoc_tag_casing(s);
    changed |= phpdoc_inline_tag_normalizer(s);
    changed |= phpdoc_no_duplicate_types(s);
    changed |= phpdoc_var_without_name(s);
    changed |= phpdoc_types_order(s);
    changed |= phpdoc_var_annotation_correct_order(s);
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
    changed |= multiline_whitespace_before_semicolons(s);
    changed |= concat_space(s);
    changed |= blank_line_after_strict_types(s);
    changed |= ordered_interfaces(s);
    changed |= ordered_traits(s);
    changed |= phpunit_method_casing(s);
    changed |= phpunit_setup_teardown_visibility(s);
    changed |= operator_linebreak(s);
    changed |= no_trailing_whitespace(s);
    changed |= no_trailing_whitespace_in_comment(s);
    changed |= declare_equal_normalize(s);
    changed |= unary_operator_spaces(s);
    changed |= control_structure_continuation_position(s);
    changed |= switch_case_semicolon_to_colon(s);
    changed |= switch_case_space(s);
    changed |= visibility_required(s);
    changed |= modifier_keywords(s);
    changed |= single_blank_line_before_namespace(s);
    changed |= space_after_comma_here_now_doc(s);
    changed |= no_blank_lines_after_class_opening(s);
    changed |= method_chaining_indentation(s);
    changed |= no_closing_tag(s);
    changed |= types_spaces(s);
    changed |= assign_null_coalescing_to_coalesce_equal(s);
    changed |= space_after_semicolon(s);
    changed |= no_multiple_statements_per_line(s);
    changed |= phpdoc_order(s);
    changed |= braces_position(s);
    changed |= statement_indentation(s);
    changed |= phpdoc_trim(s);
    changed |= no_short_bool_cast(s);
    changed |= no_mixed_echo_print(s);
    changed |= phpdoc_single_line_var_spacing(s);
    changed |= phpdoc_order_by_value(s);
    changed |= cast_spaces(s);
    changed |= not_operator_with_successor_space(s);
    changed |= self_static_accessor(s);
    changed |= no_unused_imports(s);
    changed |= self_accessor(s);
    changed |= single_line_after_imports(s);
    changed |= return_type_declaration(s);
    changed |= no_useless_return(s);
    changed |= single_line_empty_body(s);
    changed |= no_blank_lines_after_phpdoc(s);
    changed |= no_leading_import_slash(s);
    changed |= blank_line_after_namespace(s);
    changed |= no_extra_blank_lines(s);
    changed |= blank_line_before_statement(s);
    changed |= ordered_imports(s);
    changed |= single_line_comment_style(s);
    changed |= blank_lines_before_namespace(s);
    changed |= binary_operator_spaces(s);
    changed |= remove_dead_param(s);
    changed |= remove_dead_var_this(s);
    changed |= remove_param_name_reference(s);
    changed |= switched_type_and_name(s);
    changed |= double_asterisk_inline_var(s);
    changed |= fix_tag_typo(s);
    changed |= type_to_var_tag(s);
    changed |= merge_doc_block_start(s);
    changed |= add_missing_var_name(s);
    changed |= single_line_inline_var_doc_block(s);
    changed |= remove_superfluous_return_name(s);
    changed |= remove_superfluous_var_name(s);
    changed |= fix_param_name_typo(s);
    changed |= blank_line_between_import_groups(s);
    changed |= phpdoc_trim_consecutive_blank_line_separation(s);
    changed |= phpdoc_align(s);
    changed |= no_blank_line_between_imports(s);
    changed |= no_whitespace_in_blank_line(s);
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
        // a keyword-spelled class/member name (e.g. "Enum" in "extends Enum",
        // "class Enum", or a named argument "instanceOf:") is an identifier
        if keyword_used_as_identifier(s, i) {
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

fn keyword_used_as_identifier(s: &Stream, i: usize) -> bool {
    if let Some(p) = prev_significant_index(s, i) {
        if s.kind(p) == Kind::Punct {
            match s.bytes(p) {
                b"\\" | b"->" | b"?->" | b"::" => return true,
                _ => {}
            }
        }
        if s.kind(p) == Kind::Keyword {
            match s.bytes(p).to_ascii_lowercase().as_slice() {
                b"extends" | b"implements" | b"new" | b"instanceof" | b"class"
                | b"interface" | b"trait" | b"enum" | b"function" | b"const"
                | b"namespace" | b"use" | b"as" | b"goto" | b"insteadof" => return true,
                _ => {}
            }
        }
    }
    if let Some(n) = next_significant_index(s, i) {
        if s.kind(n) == Kind::Punct {
            match s.bytes(n) {
                b"\\" | b"::" | b":" => return true,
                _ => {}
            }
        }
    }
    false
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
        // a trailing comment after ";" is not a second statement
        if matches!(s.kind(next_idx), Kind::Comment | Kind::DocComment) {
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
        let v = s.bytes(i).to_vec();
        if !v.contains(&b'\t') {
            continue;
        }
        match s.kind(i) {
            Kind::Whitespace => {
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
            Kind::Comment | Kind::DocComment => {
                let out = reindent_comment_tabs(&v);
                if out != v {
                    s.set_owned(i, out);
                    changed = true;
                }
            }
            _ => {}
        }
    }
    changed
}

// Convert leading-indentation tabs to four spaces on each continuation line of a
// multi-line comment or docblock, matching ECS's indentation_type.
fn reindent_comment_tabs(v: &[u8]) -> Vec<u8> {
    let parts: Vec<&[u8]> = v.split(|&c| c == b'\n').collect();
    let mut out: Vec<u8> = Vec::with_capacity(v.len());
    for (k, part) in parts.iter().enumerate() {
        if k > 0 {
            out.push(b'\n');
        }
        if k == 0 {
            out.extend_from_slice(part);
            continue;
        }
        let mut j = 0;
        while j < part.len() && (part[j] == b' ' || part[j] == b'\t') {
            j += 1;
        }
        for &c in &part[..j] {
            if c == b'\t' {
                out.extend_from_slice(b"    ");
            } else {
                out.push(c);
            }
        }
        out.extend_from_slice(&part[j..]);
    }
    out
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
        let fixed: Vec<u8> = match b[1] {
            b'x' | b'X' => {
                // prefix lowercase, hex digits uppercase ("0Xff" -> "0xFF")
                let mut out = b"0x".to_vec();
                out.extend(b[2..].iter().map(|c| c.to_ascii_uppercase()));
                out
            }
            b'b' | b'B' => {
                let mut out = b"0b".to_vec();
                out.extend_from_slice(&b[2..]);
                out
            }
            b'o' | b'O' => {
                let mut out = b"0o".to_vec();
                out.extend_from_slice(&b[2..]);
                out
            }
            _ => continue,
        };
        if fixed.as_slice() != s.bytes(i) {
            s.set_owned(i, fixed);
            changed = true;
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
                b"->" | b"?->" | b"::" | b"\\" => continue,
                _ => {}
            }
            if s.bytes(p).eq_ignore_ascii_case(b"function") || s.bytes(p).eq_ignore_ascii_case(b"new") {
                continue;
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
        // single-line block comment: ensure one space inside "/* ... */"
        if v.starts_with(b"/*") && !v.starts_with(b"/**") && v.ends_with(b"*/")
            && v.len() >= 4 && !v.iter().any(|&c| c == b'\n' || c == b'\r')
        {
            let inner = &v[2..v.len() - 2];
            let mut nv = inner.to_vec();
            if !nv.is_empty() && nv[0] != b' ' && nv[0] != b'\t' {
                nv.insert(0, b' ');
            }
            if !nv.is_empty() && *nv.last().unwrap() != b' ' && *nv.last().unwrap() != b'\t' {
                nv.push(b' ');
            }
            if nv.as_slice() != inner {
                let mut out = b"/*".to_vec();
                out.extend_from_slice(&nv);
                out.extend_from_slice(b"*/");
                s.set_owned(i, out);
                changed = true;
            }
            continue;
        }
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
            // anonymous class: "new class extends X" -> "new class() extends X"
            if next_significant_value(s, j) == b"(" {
                i += 1;
                continue;
            }
            s.insert_owned(j + 1, Kind::Punct, b"(".to_vec());
            s.insert_owned(j + 2, Kind::Punct, b")".to_vec());
            changed = true;
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
    key: Vec<u8>,
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

fn import_sort_key(s: &Stream, use_idx: usize, semi: usize) -> Vec<u8> {
    let mut j = skip_ws(s, use_idx + 1);
    if j < s.len() && s.kind(j) == Kind::Keyword {
        let lw = s.bytes(j).to_ascii_lowercase();
        if lw == b"function" || lw == b"const" {
            j = skip_ws(s, j + 1);
        }
    }
    let mut out: Vec<u8> = Vec::new();
    let mut k = j;
    while k < semi {
        if s.kind(k) == Kind::Keyword && s.bytes(k).eq_ignore_ascii_case(b"as") {
            break;
        }
        if s.kind(k) != Kind::Whitespace {
            for &c in s.bytes(k) {
                // "\" sorts before any other char (segment-wise alpha order)
                out.push(if c == b'\\' { 0 } else { c.to_ascii_lowercase() });
            }
        }
        k += 1;
    }
    out
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
        stmts.push(ImportStmt { start: k, semi, rank: use_rank(s, k), key: import_sort_key(s, k, semi) });
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
    // ECS default imports_order: class, then function, then const; alpha within
    ordered.sort_by(|a, b| a.rank.cmp(&b.rank).then_with(|| a.key.cmp(&b.key)));

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
        kind: u8, // 0 class, 1 function, 2 const
        full_lower: Vec<u8>,
        aliased: bool,
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
        let j0 = skip_ws(s, i + 1);
        if j0 < s.len() && s.kind(j0) == Kind::Punct && s.bytes(j0) == b"(" {
            i += 1;
            continue;
        }
        let mut kind = 0u8;
        if j0 < s.len() && s.kind(j0) == Kind::Keyword {
            let lw = s.bytes(j0).to_ascii_lowercase();
            if lw == b"function" {
                kind = 1;
            } else if lw == b"const" {
                kind = 2;
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
        let (short, full, aliased) = nui_import_parts(s, i, semi, kind);
        if short.is_empty() {
            i += 1;
            continue;
        }
        imports.push(II {
            start: i,
            semi,
            short_lower: short.to_ascii_lowercase(),
            kind,
            full_lower: full.to_ascii_lowercase(),
            aliased,
        });
        i = semi + 1;
    }
    if imports.is_empty() {
        return false;
    }

    let ns_lower = nui_file_namespace(s).to_ascii_lowercase();
    let ranges: Vec<(usize, usize)> = imports.iter().map(|im| (im.start, im.semi)).collect();
    let in_import = |idx: usize| ranges.iter().any(|&(a, b)| idx >= a && idx <= b);

    let mut changed = false;
    for im in imports.iter().rev() {
        let redundant = !im.aliased
            && !ns_lower.is_empty()
            && im.full_lower.starts_with(&ns_lower)
            && im.full_lower.len() > ns_lower.len()
            && im.full_lower[ns_lower.len()] == b'\\'
            && !im.full_lower[ns_lower.len() + 1..].contains(&b'\\');
        if !redundant && nui_import_used(s, im.short_lower.as_slice(), im.kind, &in_import) {
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

fn nui_import_used<F: Fn(usize) -> bool>(s: &Stream, short_lower: &[u8], kind: u8, in_import: &F) -> bool {
    for k in 0..s.len() {
        let kd = s.kind(k);
        if kd == Kind::Ident && !in_import(k) && s.bytes(k).to_ascii_lowercase() == short_lower {
            if nui_usage_matches_kind(s, k, kind) {
                return true;
            }
            continue;
        }
        if kd == Kind::Keyword && kind == 0 && !in_import(k)
            && s.bytes(k).to_ascii_lowercase() == short_lower && nui_keyword_is_class_ref(s, k)
        {
            return true;
        }
        if (kd == Kind::Comment || kd == Kind::DocComment) && nui_comment_references(s.bytes(k), short_lower) {
            return true;
        }
    }
    false
}

fn nui_usage_matches_kind(s: &Stream, k: usize, kind: u8) -> bool {
    let prev = prev_significant_index(s, k);
    if let Some(p) = prev {
        if s.kind(p) == Kind::Punct {
            match s.bytes(p) {
                b"\\" | b"->" | b"?->" | b"::" => return false,
                _ => {}
            }
        }
        if s.kind(p) == Kind::Keyword {
            let lw = s.bytes(p).to_ascii_lowercase();
            if lw == b"namespace" || lw == b"function" {
                return false;
            }
            if lw == b"const" && next_significant_value(s, k) == b"=" {
                return false;
            }
        }
    }
    let prev_new = matches!(prev, Some(p) if s.kind(p) == Kind::Keyword && s.bytes(p).eq_ignore_ascii_case(b"new"));
    let fn_call = next_significant_value(s, k) == b"(" && !prev_new;
    if kind == 1 {
        return fn_call;
    }
    !fn_call
}

fn nui_keyword_is_class_ref(s: &Stream, k: usize) -> bool {
    if let Some(p) = prev_significant_index(s, k) {
        if s.kind(p) == Kind::Keyword {
            match s.bytes(p).to_ascii_lowercase().as_slice() {
                b"extends" | b"implements" | b"new" | b"instanceof" => return true,
                _ => {}
            }
        }
        if s.kind(p) == Kind::Punct {
            match s.bytes(p) {
                b"(" | b"," | b":" | b"|" | b"&" | b"?" => return true,
                _ => {}
            }
        }
    }
    if let Some(n) = next_significant_index(s, k) {
        if s.kind(n) == Kind::Punct && s.bytes(n) == b"::" {
            return true;
        }
        if s.kind(n) == Kind::Variable {
            return true;
        }
    }
    false
}

fn nui_ident_byte(c: u8) -> bool {
    c == b'_' || c.is_ascii_alphanumeric()
}

fn nui_comment_references(v: &[u8], short_lower: &[u8]) -> bool {
    if short_lower.is_empty() {
        return false;
    }
    let lv = v.to_ascii_lowercase();
    let mut from = 0;
    while from + short_lower.len() <= lv.len() {
        if let Some(idx) = lv[from..].windows(short_lower.len()).position(|w| w == short_lower) {
            let p = from + idx;
            let before = if p > 0 { lv[p - 1] } else { b' ' };
            let after = if p + short_lower.len() < lv.len() { lv[p + short_lower.len()] } else { b' ' };
            if !nui_ident_byte(before) && before != b'$' && before != b'\\' && !nui_ident_byte(after) {
                return true;
            }
            from = p + 1;
        } else {
            break;
        }
    }
    false
}

fn nui_import_parts(s: &Stream, use_idx: usize, semi: usize, kind: u8) -> (Vec<u8>, Vec<u8>, bool) {
    let mut j = skip_ws(s, use_idx + 1);
    if kind != 0 {
        j = skip_ws(s, j + 1);
    }
    let mut path: Vec<u8> = Vec::new();
    let mut alias: Vec<u8> = Vec::new();
    let mut in_alias = false;
    let mut aliased = false;
    let mut k = j;
    while k < semi {
        let t = s.kind(k);
        if t == Kind::Keyword && s.bytes(k).eq_ignore_ascii_case(b"as") {
            in_alias = true;
            aliased = true;
            k += 1;
            continue;
        }
        if t == Kind::Whitespace {
            k += 1;
            continue;
        }
        if in_alias {
            if t == Kind::Ident {
                alias = s.bytes(k).to_vec();
            }
        } else {
            path.extend_from_slice(s.bytes(k));
        }
        k += 1;
    }
    let full: Vec<u8> = if path.first() == Some(&b'\\') { path[1..].to_vec() } else { path };
    let short = if !alias.is_empty() {
        alias
    } else if let Some(p) = full.iter().rposition(|&c| c == b'\\') {
        full[p + 1..].to_vec()
    } else {
        full.clone()
    };
    (short, full, aliased)
}

fn nui_file_namespace(s: &Stream) -> Vec<u8> {
    for i in 0..s.len() {
        if s.kind(i) != Kind::Keyword || !s.bytes(i).eq_ignore_ascii_case(b"namespace") || member_prev(s, i) {
            continue;
        }
        let mut b: Vec<u8> = Vec::new();
        let mut k = i + 1;
        while k < s.len() {
            if s.kind(k) == Kind::Punct && (s.bytes(k) == b";" || s.bytes(k) == b"{") {
                break;
            }
            if s.kind(k) != Kind::Whitespace {
                b.extend_from_slice(s.bytes(k));
            }
            k += 1;
        }
        return if b.first() == Some(&b'\\') { b[1..].to_vec() } else { b };
    }
    Vec::new()
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
        } else if s.kind(i) == Kind::Keyword && s.bytes(i).eq_ignore_ascii_case(b"fn") {
            // arrow function: one space between "fn" and "(" (closure_fn_spacing)
            if let Some(n) = next_significant_index(s, i) {
                if s.kind(n) == Kind::Punct && s.bytes(n) == b"(" && fn_ensure_single_space_after(s, i) {
                    changed = true;
                }
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

fn count_params(s: &Stream, open: usize, close_idx: usize) -> usize {
    let mut count = 0usize;
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
        if depth == 0 && s.kind(j) == Kind::Variable {
            count += 1;
        }
        j += 1;
    }
    count
}

fn standalone_line_plain_constructor_param(s: &mut Stream) -> bool {
    const MIN_PARAM_COUNT: usize = 4;
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
        if count_params(s, open, close_idx) < MIN_PARAM_COUNT {
            i += 1;
            continue;
        }
        if has_promoted_param(s, open, close_idx) {
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

// whole-word case-insensitive search, matching a `\bword\b` regex on lowercased input
fn contains_word_ci(hay: &[u8], needle_lower: &[u8]) -> bool {
    if needle_lower.is_empty() || hay.len() < needle_lower.len() {
        return false;
    }
    let lo = hay.to_ascii_lowercase();
    let mut i = 0;
    while i + needle_lower.len() <= lo.len() {
        if &lo[i..i + needle_lower.len()] == needle_lower {
            let before_ok = i == 0 || !is_word_byte(lo[i - 1]);
            let after = i + needle_lower.len();
            let after_ok = after >= lo.len() || !is_word_byte(lo[after]);
            if before_ok && after_ok {
                return true;
            }
        }
        i += 1;
    }
    false
}

// the lexer folds a #[Required] attribute into a single "#..." comment token
fn has_required_attribute(bytes: &[u8]) -> bool {
    bytes.starts_with(b"#[") && contains_word_ci(bytes, b"required")
}

fn is_public_required_method(s: &Stream, fn_pos: usize) -> bool {
    let mut is_public = false;
    let mut is_required = false;
    let mut j = fn_pos;
    while j > 0 {
        j -= 1;
        match s.kind(j) {
            Kind::Whitespace => continue,
            Kind::Comment => {
                if has_required_attribute(s.bytes(j)) {
                    is_required = true;
                }
                continue;
            }
            Kind::DocComment => {
                if contains_word_ci(s.bytes(j), b"@required") {
                    is_required = true;
                }
                continue;
            }
            Kind::Keyword => match s.bytes(j).to_ascii_lowercase().as_slice() {
                b"public" => {
                    is_public = true;
                    continue;
                }
                b"protected" | b"private" | b"static" | b"final" | b"abstract" | b"readonly" => {
                    continue;
                }
                _ => return is_public && is_required,
            },
            _ => return is_public && is_required,
        }
    }
    is_public && is_required
}

fn standalone_line_required_param(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Keyword || !s.bytes(i).eq_ignore_ascii_case(b"function") {
            i += 1;
            continue;
        }
        if !is_public_required_method(s, i) {
            i += 1;
            continue;
        }
        let name = match sig_next(s, i) {
            Some(n) => n,
            None => {
                i += 1;
                continue;
            }
        };
        let open = match sig_next(s, name) {
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
        if reflow_paren(s, open, close_idx) {
            changed = true;
        }
        i += 1;
    }
    changed
}

const SYMFONY_ATTRIBUTE_SHORT_NAMES: &[&[u8]] = &[
    b"AsCommand",
    b"Route",
    b"Autowire",
    b"AutowireIterator",
    b"AutowireLocator",
    b"AsAlias",
    b"AsDecorator",
    b"AsTaggedItem",
    b"When",
    b"AsEventListener",
    b"AsMessageHandler",
    b"AsController",
    b"MapRequestPayload",
    b"MapQueryParameter",
    b"MapQueryString",
    b"MapEntity",
    b"IsGranted",
];

fn trim_ascii(b: &[u8]) -> &[u8] {
    let mut start = 0;
    let mut end = b.len();
    while start < end && b[start].is_ascii_whitespace() {
        start += 1;
    }
    while end > start && b[end - 1].is_ascii_whitespace() {
        end -= 1;
    }
    &b[start..end]
}

fn attribute_short_name(attr: &[u8], open: usize) -> &[u8] {
    let mut start = open;
    while start > 0 {
        let c = attr[start - 1];
        if c == b'_' || c.is_ascii_alphanumeric() {
            start -= 1;
        } else {
            break;
        }
    }
    &attr[start..open]
}

fn match_paren(attr: &[u8], open: usize) -> Option<usize> {
    let mut depth = 0i32;
    let mut j = open;
    while j < attr.len() {
        match attr[j] {
            b'(' | b'[' => depth += 1,
            b')' | b']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(j);
                }
            }
            _ => {}
        }
        j += 1;
    }
    None
}

fn split_top_level_args(inner: &[u8]) -> Vec<Vec<u8>> {
    let mut args: Vec<Vec<u8>> = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    let mut in_string = 0u8;
    let mut j = 0usize;
    while j < inner.len() {
        let c = inner[j];
        if in_string != 0 {
            if c == b'\\' {
                j += 1;
            } else if c == in_string {
                in_string = 0;
            }
            j += 1;
            continue;
        }
        match c {
            b'\'' | b'"' => in_string = c,
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth -= 1,
            b',' if depth == 0 => {
                args.push(trim_ascii(&inner[start..j]).to_vec());
                start = j + 1;
            }
            _ => {}
        }
        j += 1;
    }
    let last = trim_ascii(&inner[start..]);
    if !last.is_empty() {
        args.push(last.to_vec());
    }
    args
}

fn reflow_symfony_attribute(attr: &[u8], base: &[u8]) -> Option<Vec<u8>> {
    let open = attr.iter().position(|&c| c == b'(')?;
    let short_name = attribute_short_name(attr, open);
    if !SYMFONY_ATTRIBUTE_SHORT_NAMES.contains(&short_name) {
        return None;
    }
    let close_idx = match_paren(attr, open)?;
    let inner = &attr[open + 1..close_idx];
    let args = split_top_level_args(inner);
    if args.is_empty() {
        return None;
    }
    if short_name != b"AsCommand" && args.len() < 2 {
        return None;
    }
    let mut indent = base.to_vec();
    indent.extend_from_slice(b"    ");
    let mut out: Vec<u8> = Vec::new();
    out.extend_from_slice(&attr[..=open]);
    for (idx, arg) in args.iter().enumerate() {
        out.push(b'\n');
        out.extend_from_slice(&indent);
        out.extend_from_slice(arg);
        if idx < args.len() - 1 {
            out.push(b',');
        }
    }
    out.push(b'\n');
    out.extend_from_slice(base);
    out.extend_from_slice(&attr[close_idx..]);
    Some(out)
}

fn standalone_line_symfony_attribute_param(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Comment || !s.bytes(i).starts_with(b"#[") {
            i += 1;
            continue;
        }
        let base = line_indent_before(s, i);
        if let Some(reflowed) = reflow_symfony_attribute(s.bytes(i), &base) {
            if reflowed != s.bytes(i) {
                s.set_owned(i, reflowed);
                changed = true;
            }
        }
        i += 1;
    }
    changed
}

const NO_BREAK_STRUCTURE_KINDS: &[&[u8]] = &[
    b"for", b"foreach", b"while", b"if", b"elseif", b"switch", b"function", b"match",
];

fn nb_is_structure_kind(b: &[u8]) -> bool {
    let lo = b.to_ascii_lowercase();
    NO_BREAK_STRUCTURE_KINDS.iter().any(|k| *k == lo.as_slice())
}

fn kw_is(s: &Stream, i: usize, name: &[u8]) -> bool {
    s.kind(i) == Kind::Keyword && s.bytes(i).eq_ignore_ascii_case(name)
}

fn is_punct_val(s: &Stream, i: usize, v: &[u8]) -> bool {
    i < s.len() && s.kind(i) == Kind::Punct && s.bytes(i) == v
}

fn next_punct_of_kind(s: &Stream, idx: usize, vals: &[&[u8]]) -> Option<usize> {
    let mut j = idx + 1;
    while j < s.len() {
        if s.kind(j) == Kind::Punct && vals.iter().any(|v| *v == s.bytes(j)) {
            return Some(j);
        }
        j += 1;
    }
    None
}

fn get_prev_non_whitespace(s: &Stream, i: usize) -> Option<usize> {
    let mut j = i as isize - 1;
    while j >= 0 {
        if s.kind(j as usize) != Kind::Whitespace {
            return Some(j as usize);
        }
        j -= 1;
    }
    None
}

fn prev_whitespace(s: &Stream, i: usize) -> Option<usize> {
    let mut j = i as isize - 1;
    while j >= 0 {
        if s.kind(j as usize) == Kind::Whitespace {
            return Some(j as usize);
        }
        j -= 1;
    }
    None
}

fn nb_detect_indent(s: &Stream, index: usize) -> Vec<u8> {
    let mut idx = index;
    loop {
        let wi = match prev_whitespace(s, idx) {
            Some(w) => w,
            None => return Vec::new(),
        };
        let w = s.bytes(wi);
        if w.contains(&b'\n') {
            if let Some(nl) = w.iter().rposition(|&c| c == b'\n') {
                return w[nl + 1..].to_vec();
            }
        }
        if wi >= 1 {
            let pk = s.kind(wi - 1);
            if (pk == Kind::OpenTag || pk == Kind::Comment) && s.bytes(wi - 1).last() == Some(&b'\n') {
                if let Some(nl) = w.iter().rposition(|&c| c == b'\n') {
                    return w[nl + 1..].to_vec();
                }
                return w.to_vec();
            }
        }
        idx = wi;
    }
}

// mirrors the ~^((//|#)\s*no break\s*)|(/\*\*?\s*no break(\s+.*)*\*/)$~i regex
fn is_no_break_comment_token(s: &Stream, i: usize) -> bool {
    let k = s.kind(i);
    if k != Kind::Comment && k != Kind::DocComment {
        return false;
    }
    let lo = s.bytes(i).to_ascii_lowercase();
    // case A: line comment starting with // or #
    let prefix = if lo.starts_with(b"//") {
        Some(2)
    } else if lo.starts_with(b"#") {
        Some(1)
    } else {
        None
    };
    if let Some(mut p) = prefix {
        while p < lo.len() && lo[p].is_ascii_whitespace() {
            p += 1;
        }
        if lo[p..].starts_with(b"no break") {
            return true;
        }
    }
    // case B: block comment /* ... no break ... */
    if lo.starts_with(b"/*") && lo.ends_with(b"*/") {
        let mut p = 2;
        if p < lo.len() && lo[p] == b'*' {
            p += 1;
        }
        while p < lo.len() && lo[p].is_ascii_whitespace() {
            p += 1;
        }
        if lo[p..].starts_with(b"no break") {
            let after = p + b"no break".len();
            let inner = &lo[after..lo.len() - 2];
            if inner.is_empty() || inner[0].is_ascii_whitespace() {
                return true;
            }
        }
    }
    false
}

fn is_throw_statement_start(s: &Stream, prev: usize) -> bool {
    if s.kind(prev) == Kind::OpenTag {
        return true;
    }
    s.kind(prev) == Kind::Punct && matches!(s.bytes(prev), b"{" | b";" | b"}")
}

fn no_break_structure_end(s: &Stream, position: usize) -> usize {
    let initial = s.bytes(position).to_ascii_lowercase();
    let mut position = position;

    if nb_is_structure_kind(&initial) {
        if let Some(op) = next_punct_of_kind(s, position, &[b"("]) {
            if let Some(e) = match_forward(s, op) {
                position = e;
            }
        }
    } else if initial == b"class" {
        if let Some(op) = sig_next(s, position) {
            if is_punct_val(s, op, b"(") {
                if let Some(e) = match_forward(s, op) {
                    position = e;
                }
            }
        }
    }

    if initial == b"function" {
        match next_punct_of_kind(s, position, &[b"{"]) {
            Some(p) => position = p,
            None => return s.len() - 1,
        }
    } else {
        match sig_next(s, position) {
            Some(p) => position = p,
            None => return s.len() - 1,
        }
    }

    if !is_punct_val(s, position, b"{") {
        return next_punct_of_kind(s, position, &[b";"]).unwrap_or(s.len() - 1);
    }

    position = match match_forward(s, position) {
        Some(e) => e,
        None => return s.len() - 1,
    };

    if initial == b"do" {
        if let Some(op) = next_punct_of_kind(s, position, &[b"("]) {
            if let Some(e) = match_forward(s, op) {
                position = e;
            }
        }
        return next_punct_of_kind(s, position, &[b";"]).unwrap_or(s.len() - 1);
    }
    position
}

fn nb_prev_of_kinds(s: &Stream, idx: usize) -> Option<usize> {
    let mut j = idx as isize - 1;
    while j >= 0 {
        let k = j as usize;
        if is_punct_val(s, k, b"}") {
            return Some(k);
        }
        if s.kind(k) == Kind::Keyword {
            let lo = s.bytes(k).to_ascii_lowercase();
            if lo == b"enum" || lo == b"switch" {
                return Some(k);
            }
        }
        j -= 1;
    }
    None
}

fn is_enum_case(s: &Stream, case_index: usize, has_enum: bool) -> bool {
    if !has_enum {
        return false;
    }
    let mut prev = case_index;
    loop {
        prev = match nb_prev_of_kinds(s, prev) {
            Some(p) => p,
            None => return false,
        };
        if is_punct_val(s, prev, b"}") {
            prev = match match_backward(s, prev) {
                Some(p) => p,
                None => return false,
            };
            continue;
        }
        return kw_is(s, prev, b"enum");
    }
}

fn split_trailing_newline(content: &[u8]) -> (&[u8], &[u8]) {
    match content.iter().rposition(|&c| c == b'\n') {
        Some(nl) => (&content[..nl], &content[nl..]),
        None => (content, b""),
    }
}

fn strip_newlines(v: &[u8]) -> Vec<u8> {
    v.iter().copied().filter(|&c| c != b'\n' && c != b'\r').collect()
}

fn replace_trailing_space(v: &[u8], repl: &[u8]) -> Vec<u8> {
    let mut end = v.len();
    while end > 0 && matches!(v[end - 1], b' ' | b'\t' | b'\r' | b'\n') {
        end -= 1;
    }
    if end == v.len() {
        return v.to_vec();
    }
    let mut out = v[..end].to_vec();
    out.extend_from_slice(repl);
    out
}

fn strip_leading_newline_indent(v: &[u8]) -> Vec<u8> {
    let mut i = 0;
    if i < v.len() && (v[i] == b'\n' || v[i] == b'\r') {
        while i < v.len() && (v[i] == b'\n' || v[i] == b'\r') {
            i += 1;
        }
        while i < v.len() && (v[i] == b' ' || v[i] == b'\t') {
            i += 1;
        }
        return v[i..].to_vec();
    }
    v.to_vec()
}

fn strip_trailing_newline_indent(v: &[u8]) -> Vec<u8> {
    let mut j = v.len();
    while j > 0 && (v[j - 1] == b' ' || v[j - 1] == b'\t') {
        j -= 1;
    }
    if j > 0 && (v[j - 1] == b'\n' || v[j - 1] == b'\r') {
        while j > 0 && (v[j - 1] == b'\n' || v[j - 1] == b'\r') {
            j -= 1;
        }
        return v[..j].to_vec();
    }
    v.to_vec()
}

fn nb_ensure_newline_at(s: &mut Stream, position: usize) -> usize {
    let mut content = b"\n".to_vec();
    content.extend_from_slice(&nb_detect_indent(s, position));
    if position == 0 {
        return position;
    }
    let ws = position - 1;

    if s.kind(ws) != Kind::Whitespace {
        if s.kind(ws) == Kind::OpenTag {
            content = strip_newlines(&content);
            if !has_newline(s.bytes(ws)) {
                let repl = replace_trailing_space(s.bytes(ws), b"\n");
                s.set_owned(ws, repl);
            }
        }
        if !content.is_empty() {
            s.insert_owned(position, Kind::Whitespace, content);
            return position;
        }
        return position - 1;
    }

    if position >= 2 && s.kind(position - 2) == Kind::OpenTag && has_newline(s.bytes(position - 2)) {
        if content.first() == Some(&b'\n') {
            content.remove(0);
        }
    }
    if !has_newline(s.bytes(ws)) {
        s.set_owned(ws, content);
    }
    position - 1
}

fn insert_no_break_comment_at(s: &mut Stream, case_position: usize) {
    let mut newline_position = nb_ensure_newline_at(s, case_position);
    let content = s.bytes(newline_position).to_vec();
    let mut nb_newlines = content.iter().filter(|&&c| c == b'\n').count();

    if s.kind(newline_position) == Kind::OpenTag && has_newline(&content) {
        nb_newlines += 1;
    } else if newline_position >= 1
        && s.kind(newline_position - 1) == Kind::OpenTag
        && has_newline(s.bytes(newline_position - 1))
    {
        nb_newlines += 1;
        if !has_newline(&content) {
            let mut v = b"\n".to_vec();
            v.extend_from_slice(&content);
            s.set_owned(newline_position, v);
        }
    }

    let content = s.bytes(newline_position).to_vec();
    if nb_newlines > 1 {
        let (head, tail) = split_trailing_newline(&content);
        let indent = nb_detect_indent(s, newline_position - 1);
        let mut v = head.to_vec();
        v.push(b'\n');
        v.extend_from_slice(&indent);
        s.set_owned(newline_position, v);
        newline_position += 1;
        s.insert_owned(newline_position, Kind::Whitespace, tail.to_vec());
    }

    s.insert_owned(newline_position, Kind::Comment, b"// no break".to_vec());
    nb_ensure_newline_at(s, newline_position);
}

fn remove_no_break_comment(s: &mut Stream, comment_position: usize) {
    let prev_nw = get_prev_non_whitespace(s, comment_position);
    let after_open_tag = matches!(prev_nw, Some(p) if s.kind(p) == Kind::OpenTag);

    let whitespace_position = if after_open_tag {
        comment_position + 1
    } else {
        if comment_position == 0 {
            return;
        }
        comment_position - 1
    };

    let mut comment_position = comment_position;
    if whitespace_position < s.len() && s.kind(whitespace_position) == Kind::Whitespace {
        let v = if after_open_tag {
            strip_leading_newline_indent(s.bytes(whitespace_position))
        } else {
            strip_trailing_newline_indent(s.bytes(whitespace_position))
        };
        if v.is_empty() {
            s.remove_at(whitespace_position);
            if whitespace_position < comment_position {
                comment_position -= 1;
            }
        } else {
            s.set_owned(whitespace_position, v);
        }
    }

    s.remove_at(comment_position);
}

fn handle_next_case(
    s: &mut Stream,
    case_pos: usize,
    empty: bool,
    fall_through: bool,
    comment_position: Option<usize>,
) {
    if !empty && fall_through {
        let mut cp = comment_position;
        if let Some(c) = cp {
            if get_prev_non_whitespace(s, case_pos) != Some(c) {
                remove_no_break_comment(s, c);
                cp = None;
            }
        }
        match cp {
            None => insert_no_break_comment_at(s, case_pos),
            Some(c) => {
                nb_ensure_newline_at(s, c);
            }
        }
        return;
    }
    if let Some(c) = comment_position {
        remove_no_break_comment(s, c);
    }
}

fn fix_no_break_case(s: &mut Stream, case_position: usize) {
    let mut empty = true;
    let mut fall_through = true;
    let mut comment_position: Option<usize> = None;

    let mut i = case_position + 1;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword {
            let lv = s.bytes(i).to_ascii_lowercase();
            if nb_is_structure_kind(&lv) || lv == b"else" || lv == b"do" || lv == b"class" {
                empty = false;
                i = no_break_structure_end(s, i);
                i += 1;
                continue;
            }
            match lv.as_slice() {
                b"break" | b"continue" | b"return" | b"exit" | b"die" | b"goto" => {
                    fall_through = false;
                    i += 1;
                    continue;
                }
                b"throw" => {
                    if let Some(prev) = sig_prev(s, i) {
                        if prev == case_position || is_throw_statement_start(s, prev) {
                            fall_through = false;
                        }
                    }
                    i += 1;
                    continue;
                }
                b"endswitch" => {
                    if let Some(c) = comment_position {
                        remove_no_break_comment(s, c);
                    }
                    return;
                }
                b"case" | b"default" => {
                    handle_next_case(s, i, empty, fall_through, comment_position);
                    return;
                }
                _ => {}
            }
        }

        if is_punct_val(s, i, b"}") {
            if let Some(c) = comment_position {
                remove_no_break_comment(s, c);
            }
            return;
        }

        if is_no_break_comment_token(s, i) {
            comment_position = Some(i);
            i += 1;
            continue;
        }

        let k = s.kind(i);
        if k != Kind::Comment && k != Kind::DocComment && k != Kind::Whitespace {
            empty = false;
        }
        i += 1;
    }
}

fn no_break_comment(s: &mut Stream) -> bool {
    let mut has_switch = false;
    let mut has_enum = false;
    for i in 0..s.len() {
        if s.kind(i) == Kind::Keyword {
            let lo = s.bytes(i).to_ascii_lowercase();
            if lo == b"switch" {
                has_switch = true;
            } else if lo == b"enum" {
                has_enum = true;
            }
        }
    }
    if !has_switch {
        return false;
    }

    let before = s.render();
    let start = s.len();
    let mut i = start;
    while i > 0 {
        i -= 1;
        if kw_is(s, i, b"default") {
            if let Some(n) = sig_next(s, i) {
                if is_punct_val(s, n, b"=>") {
                    continue;
                }
            }
        } else if !kw_is(s, i, b"case") || is_enum_case(s, i, has_enum) {
            continue;
        }

        if let Some(colon) = next_punct_of_kind(s, i, &[b":", b";"]) {
            fix_no_break_case(s, colon);
        }
    }
    s.render() != before
}

fn doc_is_ws(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0c)
}

// mirrors DocBlock's split on /([^\n\r]+\R*)/: each line keeps its trailing newlines
fn split_doc_lines(content: &[u8]) -> Vec<Vec<u8>> {
    let mut lines = Vec::new();
    let mut i = 0;
    let n = content.len();
    while i < n {
        // a line must start with a non-newline char; leading newlines are dropped
        // (PREG_SPLIT_NO_EMPTY), which never happens inside a real docblock anyway
        if content[i] == b'\n' || content[i] == b'\r' {
            i += 1;
            continue;
        }
        let start = i;
        // [^\n\r]+
        while i < n && content[i] != b'\n' && content[i] != b'\r' {
            i += 1;
        }
        // \R* (any run of \r\n, \r, \n)
        while i < n && (content[i] == b'\n' || content[i] == b'\r') {
            i += 1;
        }
        lines.push(content[start..i].to_vec());
    }
    lines
}

fn line_contains_tag(line: &[u8]) -> bool {
    let mut i = 0;
    while i < line.len() {
        if line[i] == b'*' {
            let mut j = i + 1;
            while j < line.len() && doc_is_ws(line[j]) {
                j += 1;
            }
            if j < line.len() && line[j] == b'@' {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn line_contains_useful_content(line: &[u8]) -> bool {
    // /\*\s*\S+/
    let mut matched = false;
    let mut i = 0;
    while i < line.len() {
        if line[i] == b'*' {
            let mut j = i + 1;
            while j < line.len() && doc_is_ws(line[j]) {
                j += 1;
            }
            if j < line.len() && !doc_is_ws(line[j]) {
                matched = true;
                break;
            }
        }
        i += 1;
    }
    if !matched {
        return false;
    }
    // trim(str_replace(['/','*'],' ', line)) !== ''
    line.iter()
        .any(|&c| c != b'/' && c != b'*' && !doc_is_ws(c))
}

fn line_add_blank(line: &[u8]) -> Vec<u8> {
    // ^([\t ]*\*)[^\r\n]*(\r?\n)(?:\r?\n)?$
    let mut p = 0;
    while p < line.len() && (line[p] == b' ' || line[p] == b'\t') {
        p += 1;
    }
    if p >= line.len() || line[p] != b'*' {
        return line.to_vec();
    }
    let prefix_end = p + 1;
    // [^\r\n]* up to first newline
    let mut q = prefix_end;
    while q < line.len() && line[q] != b'\n' && line[q] != b'\r' {
        q += 1;
    }
    // (\r?\n)
    let nl_start = q;
    if q < line.len() && line[q] == b'\r' {
        q += 1;
    }
    if q < line.len() && line[q] == b'\n' {
        q += 1;
    } else {
        return line.to_vec();
    }
    let first_nl = &line[nl_start..q];
    // optional (\r?\n) then end
    let mut r = q;
    if r < line.len() && line[r] == b'\r' {
        r += 1;
    }
    if r < line.len() && line[r] == b'\n' {
        r += 1;
    }
    if r != line.len() {
        return line.to_vec();
    }
    let mut out = line.to_vec();
    out.extend_from_slice(&line[..prefix_end]);
    out.extend_from_slice(first_nl);
    out
}

struct DocAnnotation {
    start: usize,
    end: usize,
    name: Vec<u8>,
}

fn find_annotation_length(lines: &[Vec<u8>], start: usize) -> usize {
    let mut index = start;
    loop {
        index += 1;
        if index >= lines.len() {
            break;
        }
        if line_contains_tag(&lines[index]) {
            break;
        }
        if !line_contains_useful_content(&lines[index]) {
            if index + 1 >= lines.len()
                || !line_contains_useful_content(&lines[index + 1])
                || line_contains_tag(&lines[index + 1])
            {
                break;
            }
        }
    }
    index - start
}

fn annotation_tag_name(lines: &[Vec<u8>]) -> Vec<u8> {
    let mut content: Vec<u8> = Vec::new();
    for l in lines {
        content.extend_from_slice(l);
    }
    // @([a-zA-Z0-9_\-]+) with boundary (ws|end|'(')
    let mut i = 0;
    while i < content.len() {
        if content[i] == b'@' {
            let mut j = i + 1;
            while j < content.len()
                && (content[j].is_ascii_alphanumeric() || content[j] == b'_' || content[j] == b'-')
            {
                j += 1;
            }
            if j > i + 1 {
                if j >= content.len() {
                    return content[i + 1..j].to_vec();
                }
                let c = content[j];
                if doc_is_ws(c) || c == b'(' {
                    return content[i + 1..j].to_vec();
                }
            }
        }
        i += 1;
    }
    Vec::new()
}

fn doc_annotations(lines: &[Vec<u8>]) -> Vec<DocAnnotation> {
    let mut anns = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        if !line_contains_tag(&lines[index]) {
            index += 1;
            continue;
        }
        let length = find_annotation_length(lines, index);
        anns.push(DocAnnotation {
            start: index,
            end: index + length - 1,
            name: annotation_tag_name(&lines[index..index + length]),
        });
        index += length;
    }
    anns
}

const PHPDOC_SEPARATION_GROUPS: &[&[&[u8]]] = &[
    &[b"author", b"copyright", b"license"],
    &[b"category", b"package", b"subpackage"],
    &[b"property", b"property-read", b"property-write"],
    &[b"deprecated", b"link", b"see", b"since"],
];

fn tag_in_group(tag: &[u8], group: &[&[u8]]) -> bool {
    // default groups have no wildcard; exact match
    group.iter().any(|g| *g == tag)
}

fn separation_should_be_together(first: &[u8], second: &[u8]) -> bool {
    if first.is_empty() || second.is_empty() {
        return false;
    }
    if first == second {
        return true;
    }
    for group in PHPDOC_SEPARATION_GROUPS {
        let first_in = tag_in_group(first, group);
        let second_in = tag_in_group(second, group);
        if first_in {
            return second_in;
        }
        if second_in {
            return false;
        }
    }
    false
}

fn fix_separation_description(lines: &mut [Vec<u8>]) {
    let mut i = 0;
    while i < lines.len() {
        if line_contains_tag(&lines[i]) {
            break;
        }
        if line_contains_useful_content(&lines[i]) {
            if i + 1 < lines.len() && line_contains_tag(&lines[i + 1]) {
                lines[i] = line_add_blank(&lines[i]);
                break;
            }
        }
        i += 1;
    }
}

fn fix_separation_annotations(lines: &mut [Vec<u8>]) {
    let anns = doc_annotations(lines);
    let mut idx = 0;
    while idx + 1 < anns.len() {
        let first = &anns[idx];
        let second = &anns[idx + 1];
        if separation_should_be_together(&first.name, &second.name) {
            for pos in first.end + 1..second.start {
                lines[pos].clear();
            }
        } else {
            let pos = first.end;
            let final_ = second.start - 1;
            if pos == final_ {
                lines[pos] = line_add_blank(&lines[pos]);
            } else {
                for p in pos + 1..final_ {
                    lines[p].clear();
                }
            }
        }
        idx += 1;
    }
}

fn bytes_contains(hay: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || needle.len() > hay.len() {
        return needle.is_empty();
    }
    hay.windows(needle.len()).any(|w| w == needle)
}

fn is_structural_skip_keyword(lo: &[u8]) -> bool {
    matches!(
        lo,
        b"private" | b"protected" | b"public" | b"var" | b"function" | b"fn" | b"abstract"
            | b"const" | b"namespace" | b"require" | b"require_once" | b"include"
            | b"include_once" | b"final" | b"readonly"
    )
}

fn is_classy_keyword(lo: &[u8]) -> bool {
    matches!(lo, b"class" | b"interface" | b"trait" | b"enum")
}

fn is_assignment_op(v: &[u8]) -> bool {
    matches!(
        v,
        b"=" | b"+=" | b"-=" | b"*=" | b"/=" | b"%=" | b"**=" | b"&=" | b"|=" | b"^="
            | b"<<=" | b">>=" | b"??=" | b".="
    )
}

fn doc_is_header_comment(s: &Stream, index: usize) -> bool {
    if sig_next(s, index).is_none() {
        return false;
    }
    let mut prev = match get_prev_non_whitespace(s, index) {
        Some(p) => p,
        None => return false,
    };
    if is_punct_val(s, prev, b";") {
        let brace_close = match sig_prev(s, prev) {
            Some(p) => p,
            None => return false,
        };
        if !is_punct_val(s, brace_close, b")") {
            return false;
        }
        let brace_open = match match_backward(s, brace_close) {
            Some(p) => p,
            None => return false,
        };
        let declare = match sig_prev(s, brace_open) {
            Some(p) => p,
            None => return false,
        };
        if !kw_is(s, declare, b"declare") {
            return false;
        }
        prev = match get_prev_non_whitespace(s, declare) {
            Some(p) => p,
            None => return false,
        };
    }
    s.kind(prev) == Kind::OpenTag
}

fn doc_next_token_index(s: &Stream, index: usize) -> Option<usize> {
    let mut next = sig_next(s, index);
    while let Some(n) = next {
        if is_punct_val(s, n, b"(") {
            next = sig_next(s, n);
        } else {
            break;
        }
    }
    next
}

fn prev_enum_or_switch(s: &Stream, index: usize) -> Option<usize> {
    let mut j = index as isize - 1;
    while j >= 0 {
        let k = j as usize;
        if s.kind(k) == Kind::Keyword {
            let lo = s.bytes(k).to_ascii_lowercase();
            if lo == b"enum" || lo == b"switch" {
                return Some(k);
            }
        }
        j -= 1;
    }
    None
}

fn doc_is_structural_element(s: &Stream, index: usize) -> bool {
    match s.kind(index) {
        Kind::Keyword => {
            let lo = s.bytes(index).to_ascii_lowercase();
            if is_classy_keyword(&lo) || is_structural_skip_keyword(&lo) {
                return true;
            }
            if lo == b"case" {
                return matches!(prev_enum_or_switch(s, index), Some(p) if kw_is(s, p, b"enum"));
            }
            if lo == b"static" {
                return !matches!(sig_next(s, index), Some(n) if is_punct_val(s, n, b"::"));
            }
            false
        }
        Kind::Ident => {
            let lo = s.bytes(index).to_ascii_lowercase();
            lo == b"get" || lo == b"set"
        }
        _ => false,
    }
}

fn doc_is_valid_control(s: &Stream, doc_content: &[u8], control_index: usize) -> bool {
    if s.kind(control_index) != Kind::Keyword {
        return false;
    }
    match s.bytes(control_index).to_ascii_lowercase().as_slice() {
        b"for" | b"foreach" | b"if" | b"switch" | b"while" => {}
        _ => return false,
    }
    let open = match sig_next(s, control_index) {
        Some(o) if is_punct_val(s, o, b"(") => o,
        _ => return false,
    };
    let close_idx = match match_forward(s, open) {
        Some(c) => c,
        None => return false,
    };
    let mut i = open + 1;
    while i < close_idx {
        if s.kind(i) == Kind::Variable && bytes_contains(doc_content, s.bytes(i)) {
            return true;
        }
        i += 1;
    }
    false
}

fn doc_is_valid_variable(s: &Stream, index: usize) -> bool {
    if s.kind(index) != Kind::Variable {
        return false;
    }
    match sig_next(s, index) {
        Some(n) => s.kind(n) == Kind::Punct && is_assignment_op(s.bytes(n)),
        None => false,
    }
}

fn doc_is_valid_variable_assignment(s: &Stream, doc_content: &[u8], lc_index: usize) -> bool {
    let end_idx = if s.kind(lc_index) == Kind::Keyword
        && matches!(
            s.bytes(lc_index).to_ascii_lowercase().as_slice(),
            b"list" | b"print" | b"echo"
        ) {
        next_punct_of_kind(s, lc_index, &[b")"])
    } else if is_punct_val(s, lc_index, b"[") {
        match_forward(s, lc_index)
    } else {
        return false;
    };
    let end_idx = match end_idx {
        Some(e) => e,
        None => return false,
    };
    let mut i = lc_index + 1;
    while i < end_idx {
        if s.kind(i) == Kind::Variable && bytes_contains(doc_content, s.bytes(i)) {
            return true;
        }
        i += 1;
    }
    false
}

fn doc_is_before_structural_element(s: &Stream, index: usize) -> bool {
    let next = match doc_next_token_index(s, index) {
        Some(n) if !is_punct_val(s, n, b"}") => n,
        _ => return false,
    };
    let doc_content = s.bytes(index).to_vec();
    if doc_is_structural_element(s, next) {
        return true;
    }
    if doc_is_valid_control(s, &doc_content, next) {
        return true;
    }
    if doc_is_valid_variable(s, next) {
        return true;
    }
    if doc_is_valid_variable_assignment(s, &doc_content, next) {
        return true;
    }
    if kw_is(s, next, b"use") {
        return true;
    }
    false
}

fn phpdoc_to_comment(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::DocComment {
            i += 1;
            continue;
        }
        if doc_is_header_comment(s, i) || doc_is_before_structural_element(s, i) {
            i += 1;
            continue;
        }
        let content = s.bytes(i);
        let mut p = 0;
        while p < content.len() && (content[p] == b'/' || content[p] == b'*') {
            p += 1;
        }
        let mut new_val = b"/*".to_vec();
        new_val.extend_from_slice(&content[p..]);
        s.set_owned(i, new_val);
        s.set_kind(i, Kind::Comment);
        changed = true;
        i += 1;
    }
    changed
}

fn general_phpdoc_tag_rename(_s: &mut Stream) -> bool {
    // configured with no replacements in the ECS set: no-op
    false
}

fn param_return_and_var_tag_malforms(_s: &mut Stream) -> bool {
    // deprecated upstream: no-op, kept for set parity
    false
}

fn phpdoc_separation(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::DocComment {
            i += 1;
            continue;
        }
        let mut lines = split_doc_lines(s.bytes(i));
        if lines.len() < 2 {
            i += 1;
            continue;
        }
        fix_separation_description(&mut lines);
        fix_separation_annotations(&mut lines);
        let rebuilt: Vec<u8> = lines.concat();
        if rebuilt != s.bytes(i) {
            s.set_owned(i, rebuilt);
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
        let (kind, kw) = classify_brace(s, i);
        // an empty class/function body already collapsed to "{}" stays on its
        // line - ECS keeps "class A {}" as-is; only reflow a body with content
        if matches!(kind, BraceKind::ClassLike | BraceKind::FunctionDecl)
            && match_forward(s, i) == Some(i + 1)
        {
            i += 1;
            continue;
        }
        let next_line = match kind {
            BraceKind::ClassLike => {
                // an anonymous class ("new class ... {") keeps its brace inline
                let anon = kw
                    .and_then(|k| prev_significant_index(s, k))
                    .map(|p| s.kind(p) == Kind::Keyword && s.bytes(p).eq_ignore_ascii_case(b"new"))
                    .unwrap_or(false);
                !anon
            }
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

fn order_modifier_keywords(s: &mut Stream, m: usize) -> bool {
    let mut abs_final: Vec<Vec<u8>> = Vec::new();
    let mut visibility: Vec<u8> = Vec::new();
    let mut static_kw: Vec<u8> = Vec::new();
    let mut readonly_kw: Vec<u8> = Vec::new();
    let mut has_visibility = false;

    let mut k = m;
    while k < s.len() {
        match s.kind(k) {
            Kind::Whitespace => {
                k += 1;
                continue;
            }
            Kind::Comment | Kind::DocComment => return false,
            Kind::Keyword => {}
            _ => break,
        }
        let lw = s.bytes(k).to_ascii_lowercase();
        if lw == b"use" || lw == b"case" {
            return false;
        }
        if !is_property_modifier(&lw) {
            break;
        }
        let next = skip_ws(s, k + 1);
        if next < s.len() && s.kind(next) == Kind::Punct && s.bytes(next) == b"(" {
            return false; // "public(set)" is not representable on the flat lexer
        }
        if lw == b"abstract" || lw == b"final" {
            abs_final.push(s.bytes(k).to_vec());
        } else if lw == b"static" {
            static_kw = s.bytes(k).to_vec();
        } else if lw == b"readonly" {
            readonly_kw = s.bytes(k).to_vec();
        } else if lw == b"var" {
            visibility = b"public".to_vec();
            has_visibility = true;
        } else if lw == b"public" || lw == b"private" || lw == b"protected" {
            visibility = s.bytes(k).to_vec();
            has_visibility = true;
        }
        k += 1;
    }
    let after = k;
    if after < s.len() && matches!(s.kind(after), Kind::Comment | Kind::DocComment) {
        return false;
    }
    if visibility.is_empty() {
        visibility = b"public".to_vec();
    }

    let mut ordered: Vec<Vec<u8>> = Vec::new();
    ordered.extend(abs_final);
    ordered.push(visibility);
    if !static_kw.is_empty() {
        ordered.push(static_kw);
    }
    if !readonly_kw.is_empty() {
        ordered.push(readonly_kw);
    }

    let mut want: Vec<u8> = Vec::new();
    for mod_ in &ordered {
        want.extend_from_slice(mod_);
        want.push(b' ');
    }
    let mut cur: Vec<u8> = Vec::new();
    for x in m..after {
        cur.extend_from_slice(s.bytes(x));
    }
    if has_visibility && cur == want {
        return false;
    }

    let mut repl: Vec<(Kind, Vec<u8>)> = Vec::new();
    for mod_ in &ordered {
        repl.push((Kind::Keyword, mod_.clone()));
        repl.push((Kind::Whitespace, b" ".to_vec()));
    }
    if after == m {
        for (idx, (kind, v)) in repl.into_iter().enumerate() {
            s.insert_owned(m + idx, kind, v);
        }
        return true;
    }
    replace_range(s, m, after - 1, repl);
    true
}

fn modifier_keywords(s: &mut Stream) -> bool {
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
            if order_modifier_keywords(s, m) {
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

#[derive(Clone)]
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
            | b"string" | b"true" | b"void" | b"never" | b"$this" | b"scalar"
    )
}

// match_generic_tag: like match_type_tag but for type-carrying generic tags.
fn match_generic_tag(t: &[u8]) -> Option<(usize, usize, usize)> {
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
        b"implements" | b"extends" | b"use" | b"template-extends" | b"template-implements"
    );
    if !ok {
        return None;
    }
    let ws_start = i;
    while i < t.len() && (t[i] == b' ' || t[i] == b'\t') {
        i += 1;
    }
    if i == ws_start {
        return None;
    }
    let m1_len = i;
    let type_start = i;
    while i < t.len() && t[i] != b' ' && t[i] != b'\t' {
        i += 1;
    }
    if i == type_start {
        return None;
    }
    Some((m1_len, type_start, i))
}

// normalize_phpdoc_type_case: lowercase keyword bases anywhere in a type -
// unions, arrays, nullables, generics (Foo<Scalar> -> Foo<scalar>). A namespaced
// word ("\Foo") or a class-constant reference ("Ref::STATIC") is left as-is.
fn normalize_phpdoc_type_case(typ: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(typ.len());
    let mut i = 0;
    while i < typ.len() {
        let c = typ[i];
        let is_start = c == b'$' || c == b'_' || c == b'\\' || c.is_ascii_alphabetic();
        if !is_start {
            out.push(c);
            i += 1;
            continue;
        }
        let st = i;
        i += 1;
        while i < typ.len()
            && (typ[i] == b'_' || typ[i] == b'\\' || typ[i].is_ascii_alphanumeric())
        {
            i += 1;
        }
        let w = &typ[st..i];
        let qualified =
            w.contains(&b'\\') || (st > 0 && (typ[st - 1] == b':' || typ[st - 1] == b'\\'));
        if !qualified {
            let low = w.to_ascii_lowercase();
            if low.as_slice() != w && is_phpdoc_type_keyword(&low) {
                out.extend_from_slice(&low);
                continue;
            }
        }
        out.extend_from_slice(w);
    }
    out
}

// shared for phpdoc_scalar: rewrite the type in each type-tag line
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
    apply_to_docblocks(s, |d| {
        let mut changed = false;
        for l in d.inner.iter_mut() {
            let lead = l.content.len() - trim_left_space(&l.content).len();
            let trimmed = l.content[lead..].to_vec();
            let (m1_len, ts, te) = match match_type_tag(&trimmed).or_else(|| match_generic_tag(&trimmed)) {
                Some(x) => x,
                None => continue,
            };
            let old_type = &trimmed[ts..te];
            let new_type = normalize_phpdoc_type_case(old_type);
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
    })
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

// PhpdocAlign (left align, spacing {param:2,_default:1}) - mirror of go phpdoc_align.
struct PaMatch {
    indent: Vec<u8>,
    tag: Vec<u8>,
    hint: Vec<u8>,
    var_name: Vec<u8>,
    static_kw: Vec<u8>,
    desc: Vec<u8>,
    is_tag: bool,
}

const PA_NAME_TAGS: &[&[u8]] = &[
    b"param", b"property", b"property-read", b"property-write", b"phpstan-param",
    b"phpstan-property", b"phpstan-property-read", b"phpstan-property-write",
    b"phpstan-assert", b"phpstan-assert-if-true", b"phpstan-assert-if-false",
    b"psalm-param", b"psalm-param-out", b"psalm-property", b"psalm-property-read",
    b"psalm-property-write", b"psalm-assert", b"psalm-assert-if-true", b"psalm-assert-if-false",
];
const PA_METHOD_TAGS: &[&[u8]] = &[b"method", b"phpstan-method", b"psalm-method"];
const PA_NONAME_TAGS: &[&[u8]] = &[b"return", b"throws", b"type", b"var"];

fn pa_spacing_for(tag: &[u8]) -> usize {
    if tag == b"param" { 2 } else { 1 }
}

fn pa_is_hspace(c: u8) -> bool { c == b' ' || c == b'\t' }
fn pa_is_ws(c: u8) -> bool {
    c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' || c == 0x0b || c == 0x0c
}
fn pa_tag_name_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_' || c == b'-'
}
fn pa_in_list(tag: &[u8], list: &[&[u8]]) -> bool {
    list.iter().any(|t| *t == tag)
}
fn pa_trim_space(b: &[u8]) -> Vec<u8> {
    let mut start = 0;
    let mut end = b.len();
    while start < end && pa_is_ws(b[start]) { start += 1; }
    while end > start && pa_is_ws(b[end - 1]) { end -= 1; }
    b[start..end].to_vec()
}
fn pa_spaces(n: usize) -> Vec<u8> { vec![b' '; n] }

fn phpdoc_align(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::DocComment {
            continue;
        }
        let v = s.bytes(i).to_vec();
        let rebuilt = pa_doc_block(&v);
        if rebuilt != v {
            s.set_owned(i, rebuilt);
            changed = true;
        }
    }
    changed
}

fn pa_doc_block(content: &[u8]) -> Vec<u8> {
    let mut lines = split_doc_lines(content);
    let l = lines.len();
    let mut i = 0usize;
    while i < l {
        let m = pa_get_matches(&lines[i], false);
        if m.is_none() {
            i += 1;
            continue;
        }
        let current = i;
        let mut items = vec![m.unwrap()];
        loop {
            i += 1;
            if i >= l {
                pa_rewrite(&mut lines, current, &items);
                return lines.concat();
            }
            match pa_get_matches(&lines[i], true) {
                Some(m2) => items.push(m2),
                None => break,
            }
        }
        pa_rewrite(&mut lines, current, &items);
        // leave i on the breaking line so it is reconsidered (go: i-- then for-i++)
    }
    lines.concat()
}

fn pa_get_matches(raw: &[u8], comment_only: bool) -> Option<PaMatch> {
    // strings.TrimRight(rawLine, "\r\n")
    let mut end = raw.len();
    while end > 0 && (raw[end - 1] == b'\r' || raw[end - 1] == b'\n') {
        end -= 1;
    }
    let line = &raw[..end];
    // ^(?P<indent>(?:\ {2}|\t)*)\ ?\*
    let mut i = 0usize;
    let mut indent_end = 0usize;
    while i < line.len() {
        if i + 1 < line.len() && line[i] == b' ' && line[i + 1] == b' ' {
            i += 2;
            indent_end = i;
        } else if line[i] == b'\t' {
            i += 1;
            indent_end = i;
        } else {
            break;
        }
    }
    let indent = line[..indent_end].to_vec();
    if i < line.len() && line[i] == b' ' {
        i += 1;
    }
    if i >= line.len() || line[i] != b'*' {
        return None;
    }
    i += 1; // past '*'
    let star_pos = i;

    // tag regex: \h*@tag...
    let mut j = i;
    while j < line.len() && pa_is_hspace(line[j]) {
        j += 1;
    }
    if j < line.len() && line[j] == b'@' {
        if let Some(m) = pa_parse_tag(&indent, line, j + 1) {
            return Some(m);
        }
    }

    if !comment_only {
        return None;
    }
    // regexCommentLine: \*(?!\h?+@)(?:\s+(?P<desc>\V+))(?<!\*\/)\r?$
    let mut nb = star_pos;
    if nb < line.len() && pa_is_hspace(line[nb]) {
        nb += 1;
    }
    if nb < line.len() && line[nb] == b'@' {
        return None;
    }
    let mut d = star_pos;
    let mut saw_ws = false;
    while d < line.len() && pa_is_hspace(line[d]) {
        d += 1;
        saw_ws = true;
    }
    if !saw_ws || d >= line.len() {
        return None;
    }
    let desc = &line[d..];
    if desc.is_empty() {
        return None;
    }
    if line.ends_with(b"*/") {
        return None;
    }
    Some(PaMatch {
        indent,
        tag: Vec::new(),
        hint: Vec::new(),
        var_name: Vec::new(),
        static_kw: Vec::new(),
        desc: desc.to_vec(),
        is_tag: false,
    })
}

fn pa_parse_tag(indent: &[u8], line: &[u8], off: usize) -> Option<PaMatch> {
    let mut e = off;
    while e < line.len() && pa_tag_name_char(line[e]) {
        e += 1;
    }
    let tag = &line[off..e];
    if tag.is_empty() {
        return None;
    }
    if pa_in_list(tag, PA_NAME_TAGS) {
        return pa_parse_name_tag(indent, tag, line, e);
    }
    if pa_in_list(tag, PA_NONAME_TAGS) {
        return pa_parse_no_name_tag(indent, tag, line, e);
    }
    if pa_in_list(tag, PA_METHOD_TAGS) {
        return pa_parse_method_tag(indent, tag, line, e);
    }
    None
}

fn pa_parse_name_tag(indent: &[u8], tag: &[u8], line: &[u8], pos: usize) -> Option<PaMatch> {
    let mut p = pos;
    while p < line.len() && pa_is_ws(line[p]) {
        p += 1;
    }
    if p == pos {
        return None;
    }
    // var: a '$' at depth 0 separated from the hint by whitespace (union members
    // like `Foo|$this` and callable params stay in the hint)
    let mut dollar: isize = -1;
    let mut depth = 0i32;
    let mut k = p;
    while k < line.len() {
        match line[k] {
            b'<' | b'[' | b'(' | b'{' => depth += 1,
            b'>' | b']' | b')' | b'}' => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            b'$' => {
                if depth == 0 {
                    let mut ps = k;
                    if k >= 1 && k - 1 >= p && line[k - 1] == b'&' {
                        ps = k - 1;
                    } else if k >= 3 && k - 3 >= p && &line[k - 3..k] == b"..." {
                        ps = k - 3;
                    }
                    if ps == p || pa_is_ws(line[ps - 1]) {
                        dollar = k as isize;
                    }
                }
            }
            _ => {}
        }
        if dollar >= 0 {
            break;
        }
        k += 1;
    }
    if dollar < 0 {
        return None;
    }
    let mut var_start = dollar as usize;
    if var_start >= 1 && var_start - 1 >= p && line[var_start - 1] == b'&' {
        var_start -= 1;
    } else if var_start >= 3 && var_start - 3 >= p && &line[var_start - 3..var_start] == b"..." {
        var_start -= 3;
    }
    let hint = pa_trim_space(&line[p..var_start]);
    let mut ve = var_start;
    while ve < line.len() && !pa_is_ws(line[ve]) {
        ve += 1;
    }
    let var_name = line[var_start..ve].to_vec();
    let mut desc: Vec<u8> = Vec::new();
    let mut q = ve;
    while q < line.len() && pa_is_ws(line[q]) {
        q += 1;
    }
    if q > ve && q < line.len() {
        desc = line[q..].to_vec();
    }
    Some(PaMatch {
        indent: indent.to_vec(),
        tag: tag.to_vec(),
        hint,
        var_name,
        static_kw: Vec::new(),
        desc,
        is_tag: true,
    })
}

fn pa_parse_no_name_tag(indent: &[u8], tag: &[u8], line: &[u8], pos: usize) -> Option<PaMatch> {
    let mut p = pos;
    while p < line.len() && pa_is_ws(line[p]) {
        p += 1;
    }
    if p == pos {
        return None;
    }
    let (hint_end, balanced) = pa_scan_type(line, p);
    if !balanced {
        // unbalanced type: REGEX_TYPES matches nothing, hint empty; desc needs its
        // own \s+, which requires 2+ leading spaces to split. Single space -> None.
        if p - pos < 2 {
            return None;
        }
        return Some(PaMatch {
            indent: indent.to_vec(),
            tag: tag.to_vec(),
            hint: Vec::new(),
            var_name: Vec::new(),
            static_kw: Vec::new(),
            desc: line[p..].to_vec(),
            is_tag: true,
        });
    }
    let hint = pa_trim_space(&line[p..hint_end]);
    let mut desc: Vec<u8> = Vec::new();
    let mut q = hint_end;
    while q < line.len() && pa_is_ws(line[q]) {
        q += 1;
    }
    if q < line.len() {
        desc = line[q..].to_vec();
    }
    Some(PaMatch {
        indent: indent.to_vec(),
        tag: tag.to_vec(),
        hint,
        var_name: Vec::new(),
        static_kw: Vec::new(),
        desc,
        is_tag: true,
    })
}

fn pa_parse_method_tag(indent: &[u8], tag: &[u8], line: &[u8], pos: usize) -> Option<PaMatch> {
    if !line[pos..].contains(&b'(') {
        return pa_parse_no_name_tag(indent, tag, line, pos);
    }
    let mut p = pos;
    while p < line.len() && pa_is_ws(line[p]) {
        p += 1;
    }
    if p == pos {
        return None;
    }
    let mut static_kw: Vec<u8> = Vec::new();
    if line[p..].starts_with(b"static") && (p + 6 >= line.len() || pa_is_ws(line[p + 6])) {
        static_kw = b"static".to_vec();
        p += 6;
        while p < line.len() && pa_is_ws(line[p]) {
            p += 1;
        }
    }
    let sig_paren = match line.iter().rposition(|&c| c == b')') {
        Some(x) => x,
        None => return None,
    };
    if p > sig_paren {
        return None;
    }
    let (hint_end, _balanced) = pa_scan_type(line, p);
    let mut hint: Vec<u8>;
    let start;
    if hint_end > sig_paren {
        // no return type: the scanned run is the name+signature itself
        hint = Vec::new();
        start = p;
    } else {
        hint = pa_trim_space(&line[p..hint_end]);
        let mut q = hint_end;
        while q < line.len() && pa_is_ws(line[q]) {
            q += 1;
        }
        if q > sig_paren {
            return None;
        }
        start = q;
    }
    // strings.TrimRight(line[start:sigParen+1], " \t")
    let mut se = sig_paren + 1;
    while se > start && (line[se - 1] == b' ' || line[se - 1] == b'\t') {
        se -= 1;
    }
    let signature = line[start..se].to_vec();
    if !signature.ends_with(b")") {
        return None;
    }
    // desc: (?:\s+ \V*) after the signature's closing paren
    let mut desc: Vec<u8> = Vec::new();
    let mut r = sig_paren + 1;
    while r < line.len() && pa_is_ws(line[r]) {
        r += 1;
    }
    if r < line.len() {
        desc = line[r..].to_vec();
    }
    if hint.is_empty() && !static_kw.is_empty() {
        hint = static_kw.clone();
        static_kw = Vec::new();
    }
    Some(PaMatch {
        indent: indent.to_vec(),
        tag: tag.to_vec(),
        hint,
        var_name: signature,
        static_kw,
        desc,
        is_tag: true,
    })
}

fn pa_scan_type(line: &[u8], pos: usize) -> (usize, bool) {
    let mut depth = 0i32;
    let mut i = pos;
    let mut last = pos;
    while i < line.len() {
        let c = line[i];
        if c == b'<' || c == b'[' || c == b'(' || c == b'{' {
            depth += 1;
            i += 1;
            last = i;
        } else if c == b'>' || c == b']' || c == b')' || c == b'}' {
            if depth > 0 {
                depth -= 1;
            }
            i += 1;
            last = i;
        } else if c == b' ' || c == b'\t' {
            if depth > 0 {
                i += 1;
                continue;
            }
            let mut k = i;
            while k < line.len() && (line[k] == b' ' || line[k] == b'\t') {
                k += 1;
            }
            if k < line.len() && (line[k] == b'|' || line[k] == b'&') {
                i = k;
                continue;
            }
            return (last, depth == 0);
        } else if c == b'\n' || c == b'\r' {
            return (last, depth == 0);
        } else {
            i += 1;
            last = i;
        }
    }
    (last, depth == 0)
}

fn pa_rewrite(lines: &mut [Vec<u8>], current: usize, items: &[PaMatch]) {
    let has_static = items.iter().any(|it| it.is_tag && !it.static_kw.is_empty());
    let mut opening_hint: Vec<u8> = Vec::new();
    let mut opening_set = false;
    for (j, item) in items.iter().enumerate() {
        let mut line: Vec<u8> = Vec::new();
        if !item.is_tag {
            if !item.desc.is_empty() && item.desc[0] == b'@' {
                line.extend_from_slice(&item.indent);
                line.extend_from_slice(b" * ");
                line.extend_from_slice(&item.desc);
                line.push(b'\n');
                lines[current + j] = line;
                continue;
            }
            let left_indent = pa_left_desc_indent(items, j);
            line.extend_from_slice(&item.indent);
            line.extend_from_slice(b" * ");
            if opening_set && !opening_hint.is_empty() {
                line.push(b' ');
            }
            line.extend_from_slice(&pa_spaces(left_indent));
            line.extend_from_slice(&item.desc);
            line.push(b'\n');
            lines[current + j] = line;
            continue;
        }
        opening_hint = item.hint.clone();
        opening_set = true;
        let spacing = pa_spacing_for(&item.tag);
        line.extend_from_slice(&item.indent);
        line.extend_from_slice(b" * @");
        line.extend_from_slice(&item.tag);
        if has_static {
            if !item.static_kw.is_empty() {
                line.extend_from_slice(&pa_spaces(spacing));
                line.extend_from_slice(&item.static_kw);
            }
        }
        if !item.hint.is_empty() {
            line.extend_from_slice(&pa_spaces(spacing));
            line.extend_from_slice(&item.hint);
        }
        if !item.var_name.is_empty() {
            line.extend_from_slice(&pa_spaces(spacing));
            line.extend_from_slice(&item.var_name);
            if !item.desc.is_empty() {
                line.extend_from_slice(&pa_spaces(spacing));
                line.extend_from_slice(&item.desc);
            }
        } else if !item.desc.is_empty() {
            line.extend_from_slice(&pa_spaces(spacing));
            line.extend_from_slice(&item.desc);
        }
        line.push(b'\n');
        lines[current + j] = line;
    }
}

fn pa_left_desc_indent(items: &[PaMatch], index: usize) -> usize {
    let mut idx = index as isize;
    let mut found: Option<&PaMatch> = None;
    while idx >= 0 {
        let it = &items[idx as usize];
        if it.is_tag {
            found = Some(it);
            break;
        }
        idx -= 1;
    }
    let item = match found {
        Some(it) => it,
        None => return 0,
    };
    let spacing = pa_spacing_for(&item.tag);
    let sent = |frag: &[u8]| -> usize {
        if frag.is_empty() { 0 } else { frag.len() + spacing }
    };
    sent(&item.static_kw) + sent(&item.tag) + sent(&item.hint) + sent(&item.var_name)
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
        // a file-level docblock before "declare" keeps its blank line
        if let Some(k) = next_significant_index(s, i) {
            if s.kind(k) == Kind::Keyword && s.bytes(k).eq_ignore_ascii_case(b"declare") {
                continue;
            }
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

// --- ported batch: phpdoc structural rules ---------------------------------

fn is_word_byte(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

// phpdoc_tag_casing: replace @inheritdoc (any case, \b) with @inheritDoc.
fn replace_inheritdoc(content: &[u8]) -> Vec<u8> {
    let needle = b"inheritdoc";
    let mut out = Vec::with_capacity(content.len());
    let mut i = 0;
    while i < content.len() {
        if content[i] == b'@'
            && i + 1 + needle.len() <= content.len()
            && content[i + 1..i + 1 + needle.len()].eq_ignore_ascii_case(needle)
        {
            let after = i + 1 + needle.len();
            let boundary = after >= content.len() || !is_word_byte(content[after]);
            if boundary {
                out.extend_from_slice(b"@inheritDoc");
                i = after;
                continue;
            }
        }
        out.push(content[i]);
        i += 1;
    }
    out
}

fn phpdoc_tag_casing(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| {
        let mut changed = false;
        for l in d.inner.iter_mut() {
            let fixed = replace_inheritdoc(&l.content);
            if fixed != l.content {
                l.content = fixed;
                changed = true;
            }
        }
        changed
    })
}

fn is_inline_tag_word(low: &[u8]) -> bool {
    matches!(
        low,
        b"example" | b"id" | b"internal" | b"inheritdoc" | b"inheritdocs"
            | b"link" | b"source" | b"toc" | b"tutorial" | b"see"
    )
}

// phpdoc_inline_tag_normalizer: normalize "@{tag ...}" / "{ @tag ... }" -> "{@tag doc}".
fn normalize_inline_tags(content: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(content.len());
    let mut i = 0;
    while i < content.len() {
        // try to match at i
        let start = i;
        let mut j = i;
        let mut matched_prefix = false;
        if content[j] == b'@' {
            // @\{+
            let mut k = j + 1;
            let bs = k;
            while k < content.len() && content[k] == b'{' {
                k += 1;
            }
            if k > bs {
                j = k;
                matched_prefix = true;
            }
        }
        if !matched_prefix && content[j] == b'{' {
            // \{+[ \t]*@
            let mut k = j;
            while k < content.len() && content[k] == b'{' {
                k += 1;
            }
            let mut m = k;
            while m < content.len() && (content[m] == b' ' || content[m] == b'\t') {
                m += 1;
            }
            if k > j && m < content.len() && content[m] == b'@' {
                j = m + 1;
                matched_prefix = true;
            }
        }
        if matched_prefix {
            // [ \t]*
            while j < content.len() && (content[j] == b' ' || content[j] == b'\t') {
                j += 1;
            }
            // tag word
            let tw = j;
            while j < content.len() && content[j].is_ascii_alphabetic() {
                j += 1;
            }
            let tag = &content[tw..j];
            let boundary = j >= content.len() || !is_word_byte(content[j]);
            if !tag.is_empty() && is_inline_tag_word(tag.to_ascii_lowercase().as_slice()) && boundary {
                // [^}]*
                let is = j;
                while j < content.len() && content[j] != b'}' {
                    j += 1;
                }
                let inner = &content[is..j];
                // \}+
                if j < content.len() && content[j] == b'}' {
                    while j < content.len() && content[j] == b'}' {
                        j += 1;
                    }
                    let doc = trim_go_space(inner);
                    out.extend_from_slice(b"{@");
                    out.extend_from_slice(tag);
                    if !doc.is_empty() {
                        out.push(b' ');
                        out.extend_from_slice(doc);
                    }
                    out.push(b'}');
                    i = j;
                    continue;
                }
            }
            // no full match: emit the single starting byte, retry from start+1
        }
        out.push(content[start]);
        i = start + 1;
    }
    out
}

fn phpdoc_inline_tag_normalizer(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| {
        let mut changed = false;
        for l in d.inner.iter_mut() {
            let fixed = normalize_inline_tags(&l.content);
            if fixed != l.content {
                l.content = fixed;
                changed = true;
            }
        }
        changed
    })
}

fn dedupe_union(typ: &[u8]) -> Vec<u8> {
    let nullable = typ.first() == Some(&b'?');
    let body = if nullable { &typ[1..] } else { typ };
    if body.iter().any(|&c| matches!(c, b'<' | b'>' | b'(' | b')' | b'{' | b'}' | b'[' | b']')) {
        return typ.to_vec();
    }
    let parts: Vec<&[u8]> = body.split(|&c| c == b'|').collect();
    if parts.len() < 2 {
        return typ.to_vec();
    }
    let mut seen: Vec<Vec<u8>> = Vec::new();
    let mut kept: Vec<&[u8]> = Vec::new();
    for p in parts {
        let key = p.to_ascii_lowercase();
        if seen.iter().any(|x| x == &key) {
            continue;
        }
        seen.push(key);
        kept.push(p);
    }
    let mut out = Vec::new();
    if nullable {
        out.push(b'?');
    }
    for (i, p) in kept.iter().enumerate() {
        if i > 0 {
            out.push(b'|');
        }
        out.extend_from_slice(p);
    }
    out
}

fn phpdoc_no_duplicate_types(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| {
        let mut changed = false;
        for l in d.inner.iter_mut() {
            let lead = l.content.len() - trim_left_space(&l.content).len();
            let trimmed = l.content[lead..].to_vec();
            let (m1, ts, te) = match match_type_tag(&trimmed) {
                Some(x) => x,
                None => continue,
            };
            let old = &trimmed[ts..te];
            let new = dedupe_union(old);
            if new.as_slice() == old {
                continue;
            }
            let mut nc = l.content[..lead].to_vec();
            nc.extend_from_slice(&trimmed[..m1]);
            nc.extend_from_slice(&new);
            nc.extend_from_slice(&trimmed[te..]);
            l.content = nc;
            changed = true;
        }
        changed
    })
}

// phpdoc_var_without_name -----
fn is_prop_modifier_kw(low: &[u8]) -> bool {
    matches!(low, b"private" | b"protected" | b"public" | b"var" | b"readonly")
}

fn doc_has_braces(d: &Doc) -> bool {
    d.inner.iter().any(|l| l.content.iter().any(|&c| c == b'{' || c == b'}'))
}

// strip " $name" (not " $this") occurrences from content
fn strip_var_names(content: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(content.len());
    let mut i = 0;
    while i < content.len() {
        if content[i] == b' ' && i + 1 < content.len() && content[i + 1] == b'$' {
            let mut j = i + 2;
            // first name char: letter/_/0x80-0xFF
            if j < content.len() && (content[j].is_ascii_alphabetic() || content[j] == b'_' || content[j] >= 0x80) {
                j += 1;
                while j < content.len()
                    && (content[j].is_ascii_alphanumeric() || content[j] == b'_' || content[j] >= 0x80)
                {
                    j += 1;
                }
                let m = &content[i..j];
                if m == b" $this" {
                    out.extend_from_slice(m);
                } // else drop
                i = j;
                continue;
            }
        }
        out.push(content[i]);
        i += 1;
    }
    out
}

fn var_without_name_tag(trimmed: &[u8]) -> bool {
    // (?i)^@(?:var|type)(?:\s|$)
    for tag in [b"@var".as_slice(), b"@type".as_slice()] {
        if trimmed.len() >= tag.len() && trimmed[..tag.len()].eq_ignore_ascii_case(tag) {
            let after = tag.len();
            if after == trimmed.len() || trimmed[after].is_ascii_whitespace() {
                return true;
            }
        }
    }
    false
}

fn phpdoc_var_without_name(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::DocComment {
            continue;
        }
        let mut next = match sig_next(s, i) {
            Some(n) => n,
            None => continue,
        };
        if s.kind(next) == Kind::Keyword && s.bytes(next).eq_ignore_ascii_case(b"static") {
            next = match sig_next(s, next) {
                Some(n) => n,
                None => continue,
            };
        }
        if s.kind(next) != Kind::Keyword || !is_prop_modifier_kw(&s.bytes(next).to_ascii_lowercase()) {
            continue;
        }
        let mut d = match parse_doc(s.bytes(i)) {
            Some(d) => d,
            None => continue,
        };
        if doc_has_braces(&d) {
            continue;
        }
        let mut ch = false;
        for l in d.inner.iter_mut() {
            if !var_without_name_tag(trim_go_space(&l.content)) {
                continue;
            }
            let nc = strip_var_names(&l.content);
            if nc != l.content {
                l.content = nc;
                ch = true;
            }
        }
        if ch {
            let r = doc_render(&d);
            s.set_owned(i, r);
            changed = true;
        }
    }
    changed
}

// phpdoc_indent -----
fn docblock_line_indent(s: &Stream, i: usize) -> Option<Vec<u8>> {
    if i == 0 || s.kind(i - 1) != Kind::Whitespace {
        return None;
    }
    let v = s.bytes(i - 1);
    let nl = v.iter().rposition(|&c| c == b'\n')?;
    Some(v[nl + 1..].to_vec())
}

// docblock_target_indent: indent of the element the docblock documents (the
// line holding the next significant token).
fn docblock_target_indent(s: &Stream, i: usize) -> Option<Vec<u8>> {
    if i + 1 >= s.len() || s.kind(i + 1) != Kind::Whitespace {
        return None;
    }
    let v = s.bytes(i + 1);
    let nl = v.iter().rposition(|&c| c == b'\n')?;
    Some(v[nl + 1..].to_vec())
}

fn phpdoc_indent(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::DocComment {
            continue;
        }
        // must begin its own line
        if i == 0
            || s.kind(i - 1) != Kind::Whitespace
            || !s.bytes(i - 1).contains(&b'\n')
        {
            continue;
        }
        // align to the documented element, not the docblock's own indent
        let indent = match docblock_target_indent(s, i) {
            Some(x) => x,
            None => continue,
        };
        let mut d = match parse_doc(s.bytes(i)) {
            Some(d) if !d.single => d,
            _ => continue,
        };
        let mut ch = false;
        for l in d.inner.iter_mut() {
            let star = match l.prefix.iter().position(|&c| c == b'*') {
                Some(x) => x,
                None => continue,
            };
            let mut np = indent.clone();
            np.extend_from_slice(b" *");
            np.extend_from_slice(&l.prefix[star + 1..]);
            if np != l.prefix {
                l.prefix = np;
                ch = true;
            }
        }
        if trim_go_space(&d.close) == b"*/" {
            let mut nc = indent.clone();
            nc.extend_from_slice(b" */");
            if nc != d.close {
                d.close = nc;
                ch = true;
            }
        }
        if ch {
            let r = doc_render(&d);
            s.set_owned(i, r);
            changed = true;
        }
        // move the opening "/**" line to the same indent
        if s.kind(i - 1) == Kind::Whitespace {
            let v = s.bytes(i - 1);
            if let Some(nl) = v.iter().rposition(|&c| c == b'\n') {
                if v[nl + 1..] != indent[..] {
                    let mut nv = v[..nl + 1].to_vec();
                    nv.extend_from_slice(&indent);
                    s.set_owned(i - 1, nv);
                    changed = true;
                }
            }
        }
    }
    changed
}

// phpdoc_order_by_value (@covers) -----
fn covers_value(content: &[u8]) -> Option<Vec<u8>> {
    let t = trim_go_space(content);
    let p = b"@covers";
    if t.len() < p.len() || &t[..p.len()] != p {
        return None;
    }
    let mut i = p.len();
    let ws = i;
    while i < t.len() && t[i].is_ascii_whitespace() {
        i += 1;
    }
    if i == ws {
        return None;
    }
    let val = trim_go_space(&t[i..]);
    if val.is_empty() {
        return None;
    }
    Some(val.to_ascii_lowercase())
}

fn phpdoc_order_by_value(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| {
        let mut changed = false;
        let n = d.inner.len();
        let mut start = 0;
        while start < n {
            if covers_value(&d.inner[start].content).is_none() {
                start += 1;
                continue;
            }
            let mut end = start + 1;
            while end < n && covers_value(&d.inner[end].content).is_some() {
                end += 1;
            }
            if end - start > 1 {
                let mut idx: Vec<usize> = (start..end).collect();
                let keys: Vec<Vec<u8>> = (start..end)
                    .map(|k| covers_value(&d.inner[k].content).unwrap())
                    .collect();
                let sorted = {
                    let mut w = idx.clone();
                    w.sort_by(|&a, &b| keys[a - start].cmp(&keys[b - start]));
                    w
                };
                if sorted != idx {
                    let moved: Vec<DocLine> = sorted
                        .iter()
                        .map(|&k| DocLine {
                            prefix: d.inner[k].prefix.clone(),
                            content: d.inner[k].content.clone(),
                        })
                        .collect();
                    for (off, l) in moved.into_iter().enumerate() {
                        d.inner[start + off] = l;
                    }
                    changed = true;
                }
                let _ = &mut idx;
            }
            start = end;
        }
        changed
    })
}

// phpdoc_line_span -----
fn documents_member(s: &Stream, i: usize) -> bool {
    let j = skip_ws(s, i + 1);
    if j >= s.len() || s.kind(j) != Kind::Keyword {
        return false;
    }
    let lw = s.bytes(j).to_ascii_lowercase();
    matches!(lw.as_slice(), b"public" | b"private" | b"protected" | b"var")
}

fn phpdoc_line_span(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::DocComment {
            continue;
        }
        let d0 = match parse_doc(s.bytes(i)) {
            Some(d) if d.single => d,
            _ => continue,
        };
        if !documents_member(s, i) {
            continue;
        }
        let indent = match docblock_line_indent(s, i) {
            Some(x) => x,
            None => continue,
        };
        let content = if d0.inner.is_empty() { Vec::new() } else { d0.inner[0].content.clone() };
        let mut prefix = indent.clone();
        prefix.extend_from_slice(b" * ");
        let mut close = indent.clone();
        close.extend_from_slice(b" */");
        let d = Doc {
            open: b"/**".to_vec(),
            inner: vec![DocLine { prefix, content }],
            close,
            single: false,
        };
        let r = doc_render(&d);
        s.set_owned(i, r);
        changed = true;
    }
    changed
}

// phpdoc_types_order -----
fn split_top_level_union(typ: &[u8]) -> Vec<Vec<u8>> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0;
    let mut i = 0;
    while i < typ.len() {
        match typ[i] {
            b'<' | b'(' | b'[' | b'{' => depth += 1,
            b'>' | b')' | b']' | b'}' => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            b'|' => {
                if depth == 0 {
                    parts.push(typ[start..i].to_vec());
                    start = i + 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    parts.push(typ[start..].to_vec());
    parts
}

fn normalize_phpdoc_compare(t: &[u8]) -> Vec<u8> {
    let mut x = t;
    while x.first() == Some(&b'(') {
        x = &x[1..];
    }
    if x.first() == Some(&b'?') {
        x = &x[1..];
    }
    if x.first() == Some(&b'\\') {
        x = &x[1..];
    }
    x.to_vec()
}

fn sort_phpdoc_union(typ: &[u8]) -> Option<Vec<u8>> {
    if typ.iter().any(|&c| matches!(c, b'<' | b'(' | b'{')) {
        return None;
    }
    let members = split_top_level_union(typ);
    if members.len() < 2 {
        return None;
    }
    let mut non_null: Vec<Vec<u8>> = Vec::new();
    let mut nulls: Vec<Vec<u8>> = Vec::new();
    for m in &members {
        if normalize_phpdoc_compare(m).eq_ignore_ascii_case(b"null") {
            nulls.push(m.clone());
        } else {
            non_null.push(m.clone());
        }
    }
    if nulls.is_empty() {
        return None;
    }
    let mut all = non_null;
    all.extend(nulls);
    let mut out = Vec::new();
    for (i, p) in all.iter().enumerate() {
        if i > 0 {
            out.push(b'|');
        }
        out.extend_from_slice(p);
    }
    Some(out)
}

fn phpdoc_types_order(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| {
        let mut changed = false;
        for l in d.inner.iter_mut() {
            let lead = l.content.len() - trim_left_space(&l.content).len();
            let trimmed = l.content[lead..].to_vec();
            let (m1, ts, te) = match match_type_tag(&trimmed) {
                Some(x) => x,
                None => continue,
            };
            let old = &trimmed[ts..te];
            let sorted = match sort_phpdoc_union(old) {
                Some(x) if x.as_slice() != old => x,
                _ => continue,
            };
            let mut nc = l.content[..lead].to_vec();
            nc.extend_from_slice(&trimmed[..m1]);
            nc.extend_from_slice(&sorted);
            nc.extend_from_slice(&trimmed[te..]);
            l.content = nc;
            changed = true;
        }
        changed
    })
}

// phpdoc_var_annotation_correct_order -----
// (?i)(@(?:type|var)\s*)(\$\S+)([ \t]+)([^$](?:[^<\s]|<[^>]*>)*)(\s|\*) -> g1 g4 g3 g2 g5
fn var_order_replace(v: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(v.len());
    let mut i = 0;
    while i < v.len() {
        let start = i;
        // g1: @type|@var then \s*
        let mut j = i;
        let mut is_tag = false;
        for tag in [b"@type".as_slice(), b"@var".as_slice()] {
            if j + tag.len() <= v.len() && v[j..j + tag.len()].eq_ignore_ascii_case(tag) {
                j += tag.len();
                is_tag = true;
                break;
            }
        }
        if is_tag {
            let g1s = start;
            while j < v.len() && v[j].is_ascii_whitespace() {
                j += 1;
            }
            let g1 = &v[g1s..j];
            // g2: \$\S+
            if j < v.len() && v[j] == b'$' {
                let g2s = j;
                j += 1;
                while j < v.len() && !v[j].is_ascii_whitespace() {
                    j += 1;
                }
                let g2 = &v[g2s..j];
                if g2.len() > 1 {
                    // g3: [ \t]+
                    let g3s = j;
                    while j < v.len() && (v[j] == b' ' || v[j] == b'\t') {
                        j += 1;
                    }
                    if j > g3s {
                        let g3 = &v[g3s..j];
                        // g4: [^$] (?:[^<\s]|<[^>]*>)*
                        let g4s = j;
                        if j < v.len() && v[j] != b'$' {
                            j += 1;
                            loop {
                                if j >= v.len() {
                                    break;
                                }
                                let c = v[j];
                                if c == b'<' {
                                    // <[^>]*>
                                    let mut k = j + 1;
                                    while k < v.len() && v[k] != b'>' {
                                        k += 1;
                                    }
                                    if k < v.len() && v[k] == b'>' {
                                        j = k + 1;
                                        continue;
                                    }
                                    break;
                                }
                                if c == b'<' || c.is_ascii_whitespace() {
                                    break;
                                }
                                j += 1;
                            }
                            let g4 = &v[g4s..j];
                            // g5: (\s|\*)
                            if j < v.len() && (v[j].is_ascii_whitespace() || v[j] == b'*') {
                                let g5 = &v[j..j + 1];
                                // emit g1 g4 g3 g2 g5
                                out.extend_from_slice(g1);
                                out.extend_from_slice(g4);
                                out.extend_from_slice(g3);
                                out.extend_from_slice(g2);
                                out.extend_from_slice(g5);
                                i = j + 1;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        out.push(v[start]);
        i = start + 1;
    }
    out
}

fn phpdoc_var_annotation_correct_order(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::DocComment {
            continue;
        }
        let lower = s.bytes(i).to_ascii_lowercase();
        if !contains_subslice(&lower, b"@var") && !contains_subslice(&lower, b"@type") {
            continue;
        }
        let nv = var_order_replace(s.bytes(i));
        if nv.as_slice() != s.bytes(i) {
            s.set_owned(i, nv);
            changed = true;
        }
    }
    changed
}

// phpdoc_return_self_reference -----
fn return_self_map(low: &[u8]) -> Option<Vec<u8>> {
    match low {
        b"this" | b"@this" => Some(b"$this".to_vec()),
        b"$self" | b"@self" => Some(b"self".to_vec()),
        b"$static" | b"@static" => Some(b"static".to_vec()),
        _ => None,
    }
}

fn match_return_tag(t: &[u8]) -> Option<(usize, usize, usize)> {
    // (?i)^(@return\s+)(\S+)(.*)$
    let p = b"@return";
    if t.len() < p.len() || !t[..p.len()].eq_ignore_ascii_case(p) {
        return None;
    }
    let mut i = p.len();
    let ws = i;
    while i < t.len() && t[i].is_ascii_whitespace() {
        i += 1;
    }
    if i == ws {
        return None;
    }
    let m1 = i;
    let ts = i;
    while i < t.len() && !t[i].is_ascii_whitespace() {
        i += 1;
    }
    if i == ts {
        return None;
    }
    Some((m1, ts, i))
}

fn phpdoc_return_self_reference(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| {
        let mut changed = false;
        for l in d.inner.iter_mut() {
            let lead = l.content.len() - trim_left_space(&l.content).len();
            let trimmed = l.content[lead..].to_vec();
            let (m1, ts, te) = match match_return_tag(&trimmed) {
                Some(x) => x,
                None => continue,
            };
            let old = &trimmed[ts..te];
            let parts: Vec<&[u8]> = old.split(|&c| c == b'|').collect();
            let mut ch = false;
            let mut newparts: Vec<Vec<u8>> = Vec::new();
            for p in &parts {
                if let Some(r) = return_self_map(p.to_ascii_lowercase().as_slice()) {
                    newparts.push(r);
                    ch = true;
                } else {
                    newparts.push(p.to_vec());
                }
            }
            if !ch {
                continue;
            }
            let mut newtype = Vec::new();
            for (k, p) in newparts.iter().enumerate() {
                if k > 0 {
                    newtype.push(b'|');
                }
                newtype.extend_from_slice(p);
            }
            let mut nc = l.content[..lead].to_vec();
            nc.extend_from_slice(&trimmed[..m1]);
            nc.extend_from_slice(&newtype);
            nc.extend_from_slice(&trimmed[te..]);
            l.content = nc;
            changed = true;
        }
        changed
    })
}

// --- ported: phpdoc_no_useless_inheritdoc + no_superfluous_phpdoc_tags ------

fn is_inheritdoc_line(content: &[u8]) -> bool {
    let c = trim_go_space(content).to_ascii_lowercase();
    c == b"{@inheritdoc}" || c == b"@inheritdoc"
}

fn all_blank_lines(lines: &[DocLine]) -> bool {
    lines.iter().all(|l| trim_go_space(&l.content).is_empty())
}

fn phpdoc_no_useless_inheritdoc(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::DocComment {
            i += 1;
            continue;
        }
        let mut d = match parse_doc(s.bytes(i)) {
            Some(d) => d,
            None => {
                i += 1;
                continue;
            }
        };
        let mut kept: Vec<DocLine> = Vec::new();
        let mut removed = false;
        for l in d.inner.drain(..) {
            if is_inheritdoc_line(&l.content) {
                removed = true;
                continue;
            }
            kept.push(l);
        }
        if !removed {
            i += 1;
            continue;
        }
        changed = true;
        if all_blank_lines(&kept) {
            s.remove_at(i);
            if i > 0 && s.kind(i - 1) == Kind::Whitespace {
                s.remove_at(i - 1);
                i -= 1;
            }
            // i stays (Go i--; loop ++)
            continue;
        }
        d.inner = kept;
        let r = doc_render(&d);
        s.set_owned(i, r);
        i += 1;
    }
    changed
}

fn nsp_is_func_modifier(v: &[u8]) -> bool {
    matches!(v, b"public" | b"private" | b"protected" | b"static" | b"final" | b"abstract")
}

fn nsp_is_param_modifier(v: &[u8]) -> bool {
    matches!(v, b"public" | b"private" | b"protected" | b"readonly")
}

fn normalize_type(t: &[u8]) -> Vec<u8> {
    let t = trim_go_space(t);
    if t.is_empty() {
        return Vec::new();
    }
    let nullable = t.first() == Some(&b'?');
    let body = if nullable { &t[1..] } else { t };
    let mut set: Vec<Vec<u8>> = Vec::new();
    let mut cur: Vec<u8> = Vec::new();
    let mut push = |cur: &mut Vec<u8>, set: &mut Vec<Vec<u8>>| {
        let m = trim_go_space(cur).to_ascii_lowercase();
        cur.clear();
        if m.is_empty() {
            return;
        }
        let short = match m.iter().rposition(|&c| c == b'\\') {
            Some(p) => m[p + 1..].to_vec(),
            None => m,
        };
        if !short.is_empty() && !set.contains(&short) {
            set.push(short);
        }
    };
    for &c in body {
        if c == b'|' || c == b'&' {
            push(&mut cur, &mut set);
        } else {
            cur.push(c);
        }
    }
    push(&mut cur, &mut set);
    if nullable {
        let n = b"null".to_vec();
        if !set.contains(&n) {
            set.push(n);
        }
    }
    set.sort();
    let mut out = Vec::new();
    for (i, m) in set.iter().enumerate() {
        if i > 0 {
            out.push(b'|');
        }
        out.extend_from_slice(m);
    }
    out
}

struct FuncSig {
    params: Vec<(Vec<u8>, Vec<u8>)>, // name (no $) -> normalized native type
    ret: Vec<u8>,
    has_ret: bool,
    var_type: Vec<u8>,
    has_var: bool,
}

fn nsp_parse_property(s: &Stream, start: usize) -> Option<FuncSig> {
    let mut type_toks: Vec<u8> = Vec::new();
    let mut k = start;
    while k < s.len() {
        match s.kind(k) {
            Kind::Whitespace | Kind::Comment => {}
            Kind::Variable => {
                return Some(FuncSig {
                    params: Vec::new(),
                    ret: Vec::new(),
                    has_ret: false,
                    var_type: normalize_type(&type_toks),
                    has_var: true,
                });
            }
            Kind::Ident | Kind::Keyword => type_toks.extend_from_slice(s.bytes(k)),
            Kind::Punct => {
                if matches!(s.bytes(k), b"?" | b"|" | b"&" | b"\\") {
                    type_toks.extend_from_slice(s.bytes(k));
                } else {
                    return None;
                }
            }
            _ => return None,
        }
        k += 1;
    }
    None
}

fn nsp_parse_params(s: &Stream, open: usize, close: usize, sig: &mut FuncSig) {
    let mut type_toks: Vec<u8> = Vec::new();
    let mut var_name: Vec<u8> = Vec::new();
    let mut depth = 0i32;
    let mut in_default = false;
    let flush = |type_toks: &mut Vec<u8>, var_name: &mut Vec<u8>, in_default: &mut bool, sig: &mut FuncSig| {
        if !var_name.is_empty() {
            sig.params.push((var_name.clone(), normalize_type(type_toks)));
        }
        type_toks.clear();
        var_name.clear();
        *in_default = false;
    };
    let mut k = open + 1;
    while k < close {
        match s.kind(k) {
            Kind::Whitespace | Kind::Comment | Kind::DocComment => {
                k += 1;
                continue;
            }
            _ => {}
        }
        if s.kind(k) == Kind::Punct {
            match s.bytes(k) {
                b"(" | b"[" | b"{" => depth += 1,
                b")" | b"]" | b"}" => depth -= 1,
                b"," => {
                    if depth == 0 {
                        flush(&mut type_toks, &mut var_name, &mut in_default, sig);
                        k += 1;
                        continue;
                    }
                }
                b"=" => {
                    if depth == 0 {
                        in_default = true;
                        k += 1;
                        continue;
                    }
                }
                _ => {}
            }
            if !in_default && depth == 0 && matches!(s.bytes(k), b"?" | b"|" | b"&" | b"\\") {
                type_toks.extend_from_slice(s.bytes(k));
            }
            k += 1;
            continue;
        }
        if in_default {
            k += 1;
            continue;
        }
        match s.kind(k) {
            Kind::Variable => {
                if depth == 0 {
                    var_name = s.bytes(k).strip_prefix(b"$").unwrap_or(s.bytes(k)).to_vec();
                }
            }
            Kind::Ident | Kind::Keyword => {
                if depth == 0 && !nsp_is_param_modifier(&s.bytes(k).to_ascii_lowercase()) {
                    type_toks.extend_from_slice(s.bytes(k));
                }
            }
            _ => {}
        }
        k += 1;
    }
    flush(&mut type_toks, &mut var_name, &mut in_default, sig);
}

fn nsp_parse_signature(s: &Stream, fnx: usize) -> Option<FuncSig> {
    let mut open = next_significant_index(s, fnx);
    while let Some(o) = open {
        if s.kind(o) != Kind::Punct {
            open = next_significant_index(s, o);
        } else {
            break;
        }
    }
    let mut o = open?;
    if s.bytes(o) == b"&" {
        o = next_significant_index(s, o)?;
    }
    if s.kind(o) != Kind::Punct || s.bytes(o) != b"(" {
        return None;
    }
    let close = match_forward(s, o)?;
    let mut sig = FuncSig {
        params: Vec::new(),
        ret: Vec::new(),
        has_ret: false,
        var_type: Vec::new(),
        has_var: false,
    };
    nsp_parse_params(s, o, close, &mut sig);
    if let Some(c) = next_significant_index(s, close) {
        if s.kind(c) == Kind::Punct && s.bytes(c) == b":" {
            let mut parts: Vec<u8> = Vec::new();
            let mut k = c + 1;
            while k < s.len() {
                if s.kind(k) == Kind::Whitespace {
                    k += 1;
                    continue;
                }
                if s.kind(k) == Kind::Punct && (s.bytes(k) == b"{" || s.bytes(k) == b";") {
                    break;
                }
                parts.extend_from_slice(s.bytes(k));
                k += 1;
            }
            sig.ret = normalize_type(&parts);
            sig.has_ret = true;
        }
    }
    Some(sig)
}

fn nsp_signature_after(s: &Stream, doc: usize) -> Option<FuncSig> {
    let mut j = doc + 1;
    while j < s.len() {
        match s.kind(j) {
            Kind::Whitespace | Kind::Comment => {
                j += 1;
                continue;
            }
            Kind::Keyword => {
                let lv = s.bytes(j).to_ascii_lowercase();
                if lv == b"function" {
                    return nsp_parse_signature(s, j);
                }
                if lv == b"const" {
                    return None;
                }
                if lv == b"var" || nsp_is_func_modifier(&lv) || lv == b"readonly" {
                    j += 1;
                    continue;
                }
                return nsp_parse_property(s, j);
            }
            Kind::Ident | Kind::Punct | Kind::Variable => return nsp_parse_property(s, j),
            _ => return None,
        }
    }
    None
}

fn type_is_superfluous(php_type: &[u8], native: &[u8]) -> bool {
    if php_type.iter().any(|&c| matches!(c, b'<' | b'{' | b'(')) {
        return false;
    }
    if native.is_empty() {
        return false;
    }
    normalize_type(php_type) == native
}

// @param: ^@param\s+(\S+)\s+(&?\.{0,3}\$name)\s*(.*)$
fn nsp_match_param(c: &[u8]) -> Option<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let p = b"@param";
    if c.len() < p.len() || !c[..p.len()].eq_ignore_ascii_case(p) {
        return None;
    }
    let mut i = p.len();
    let ws = i;
    while i < c.len() && c[i].is_ascii_whitespace() {
        i += 1;
    }
    if i == ws {
        return None;
    }
    let ts = i;
    while i < c.len() && !c[i].is_ascii_whitespace() {
        i += 1;
    }
    if i == ts {
        return None;
    }
    let phptype = c[ts..i].to_vec();
    let ws2 = i;
    while i < c.len() && c[i].is_ascii_whitespace() {
        i += 1;
    }
    if i == ws2 {
        return None;
    }
    // (&?\.{0,3}\$name)
    let ns = i;
    if i < c.len() && c[i] == b'&' {
        i += 1;
    }
    let mut dots = 0;
    while i < c.len() && c[i] == b'.' && dots < 3 {
        i += 1;
        dots += 1;
    }
    if i >= c.len() || c[i] != b'$' {
        return None;
    }
    i += 1;
    if i >= c.len() || !(c[i].is_ascii_alphabetic() || c[i] == b'_') {
        return None;
    }
    i += 1;
    while i < c.len() && (c[i].is_ascii_alphanumeric() || c[i] == b'_') {
        i += 1;
    }
    let name_full = &c[ns..i];
    let dollar = name_full.iter().position(|&x| x == b'$').unwrap();
    let varname = name_full[dollar + 1..].to_vec();
    while i < c.len() && c[i].is_ascii_whitespace() {
        i += 1;
    }
    let desc = trim_go_space(&c[i..]).to_vec();
    Some((phptype, varname, desc))
}

fn nsp_match_return(c: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
    let p = b"@return";
    if c.len() < p.len() || !c[..p.len()].eq_ignore_ascii_case(p) {
        return None;
    }
    let mut i = p.len();
    let ws = i;
    while i < c.len() && c[i].is_ascii_whitespace() {
        i += 1;
    }
    if i == ws {
        return None;
    }
    let ts = i;
    while i < c.len() && !c[i].is_ascii_whitespace() {
        i += 1;
    }
    if i == ts {
        return None;
    }
    let phptype = c[ts..i].to_vec();
    while i < c.len() && c[i].is_ascii_whitespace() {
        i += 1;
    }
    Some((phptype, trim_go_space(&c[i..]).to_vec()))
}

fn nsp_match_var(c: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
    let p = b"@var";
    if c.len() < p.len() || !c[..p.len()].eq_ignore_ascii_case(p) {
        return None;
    }
    let mut i = p.len();
    let ws = i;
    while i < c.len() && c[i].is_ascii_whitespace() {
        i += 1;
    }
    if i == ws {
        return None;
    }
    let ts = i;
    while i < c.len() && !c[i].is_ascii_whitespace() {
        i += 1;
    }
    if i == ts {
        return None;
    }
    let phptype = c[ts..i].to_vec();
    // optional \s+\$name
    let mut j = i;
    while j < c.len() && c[j].is_ascii_whitespace() {
        j += 1;
    }
    if j < c.len() && c[j] == b'$' {
        let mut k = j + 1;
        if k < c.len() && (c[k].is_ascii_alphabetic() || c[k] == b'_') {
            k += 1;
            while k < c.len() && (c[k].is_ascii_alphanumeric() || c[k] == b'_') {
                k += 1;
            }
            i = k;
        }
    }
    while i < c.len() && c[i].is_ascii_whitespace() {
        i += 1;
    }
    Some((phptype, trim_go_space(&c[i..]).to_vec()))
}

// returns (drop, matched)
fn nsp_superfluous_tag(content: &[u8], sig: &FuncSig) -> (bool, bool) {
    if let Some((phptype, varname, desc)) = nsp_match_param(content) {
        if !desc.is_empty() {
            return (false, true);
        }
        match sig.params.iter().find(|(n, _)| n == &varname) {
            Some((_, native)) => return (type_is_superfluous(&phptype, native), true),
            None => return (false, true),
        }
    }
    if let Some((phptype, desc)) = nsp_match_return(content) {
        if !desc.is_empty() {
            return (false, true);
        }
        return (type_is_superfluous(&phptype, &sig.ret), true);
    }
    if sig.has_var {
        if let Some((phptype, desc)) = nsp_match_var(content) {
            if !desc.is_empty() {
                return (false, true);
            }
            return (type_is_superfluous(&phptype, &sig.var_type), true);
        }
    }
    (false, false)
}

fn nsp_has_continuation(inner: &[DocLine], idx: usize) -> bool {
    if idx + 1 >= inner.len() {
        return false;
    }
    let next = trim_go_space(&inner[idx + 1].content);
    !next.is_empty() && next.first() != Some(&b'@')
}

fn no_superfluous_phpdoc_tags(s: &mut Stream) -> bool {
    let mut changed = false;
    for i in 0..s.len() {
        if s.kind(i) != Kind::DocComment {
            continue;
        }
        let sig = match nsp_signature_after(s, i) {
            Some(x) => x,
            None => continue,
        };
        let mut d = match parse_doc(s.bytes(i)) {
            Some(d) if !d.single => d,
            _ => continue,
        };
        let mut kept: Vec<DocLine> = Vec::new();
        let mut removed = false;
        for idx in 0..d.inner.len() {
            let content = trim_left_space(&d.inner[idx].content).to_vec();
            let (drop, ok) = nsp_superfluous_tag(&content, &sig);
            if ok && drop && !nsp_has_continuation(&d.inner, idx) {
                removed = true;
                continue;
            }
            kept.push(DocLine {
                prefix: d.inner[idx].prefix.clone(),
                content: d.inner[idx].content.clone(),
            });
        }
        if removed {
            d.inner = kept;
            let r = doc_render(&d);
            s.set_owned(i, r);
            changed = true;
        }
    }
    changed
}

// PHP-CS-Fixer NoAlternativeSyntaxFixer: replace control-structure alternative
// syntax with braces. Matches ECS default fix_non_monolithic_code=true.
const NAS_OPEN: &[&[u8]] = &[b"if", b"foreach", b"while", b"for", b"switch", b"declare"];
const NAS_END: &[&[u8]] = &[
    b"endif", b"endforeach", b"endwhile", b"endfor", b"endswitch", b"enddeclare",
];

fn nas_kw_in(s: &Stream, i: usize, set: &[&[u8]]) -> bool {
    if s.kind(i) != Kind::Keyword {
        return false;
    }
    let b = s.bytes(i);
    set.iter().any(|w| b.eq_ignore_ascii_case(w))
}

fn nas_is_punct(s: &Stream, i: usize, v: &[u8]) -> bool {
    s.kind(i) == Kind::Punct && s.bytes(i) == v
}

fn nas_next_meaningful(s: &Stream, i: usize) -> Option<usize> {
    let mut j = i + 1;
    while j < s.len() {
        let k = s.kind(j);
        if k != Kind::Whitespace && k != Kind::Comment && k != Kind::DocComment {
            return Some(j);
        }
        j += 1;
    }
    None
}

fn nas_find_parenthesis_end(s: &Stream, control_index: usize) -> usize {
    match nas_next_meaningful(s, control_index) {
        Some(ni) if nas_is_punct(s, ni, b"(") => match_forward(s, ni).unwrap_or(control_index),
        _ => control_index,
    }
}

fn nas_next_paren(s: &Stream, index: usize) -> Option<usize> {
    let mut j = index + 1;
    while j < s.len() {
        if nas_is_punct(s, j, b"(") {
            return Some(j);
        }
        j += 1;
    }
    None
}

// Replace the single token at pos with items (kind, bytes), preserving order.
fn nas_replace(s: &mut Stream, pos: usize, items: &[(Kind, &[u8])]) {
    s.remove_at(pos);
    for (i, (k, v)) in items.iter().enumerate() {
        s.insert_owned(pos + i, *k, v.to_vec());
    }
}

fn nas_add_braces(s: &mut Stream, keyword: &[u8], index: usize, colon_index: usize) {
    let trailing_ws = index + 1 < s.len() && s.kind(index + 1) != Kind::Whitespace;
    let mut open: Vec<(Kind, &[u8])> = vec![
        (Kind::Punct, b"}"),
        (Kind::Whitespace, b" "),
        (Kind::Keyword, keyword),
    ];
    if trailing_ws {
        open.push((Kind::Whitespace, b" "));
    }
    let open_len = open.len();
    nas_replace(s, index, &open);

    let colon_index = colon_index + open_len - 1;
    let mut closing: Vec<(Kind, &[u8])> = vec![(Kind::Punct, b"{")];
    if colon_index + 1 < s.len() && s.kind(colon_index + 1) != Kind::Whitespace {
        closing.push((Kind::Whitespace, b" "));
    }
    nas_replace(s, colon_index, &closing);
}

fn nas_fix_open_close(s: &mut Stream, index: usize) -> bool {
    if nas_kw_in(s, index, NAS_OPEN) {
        let open_index = match nas_next_paren(s, index) {
            Some(o) => o,
            None => return false,
        };
        let close_index = match match_forward(s, open_index) {
            Some(c) => c,
            None => return false,
        };
        let after_index = match nas_next_meaningful(s, close_index) {
            Some(a) if nas_is_punct(s, a, b":") => a,
            _ => return false,
        };
        let mut items: Vec<(Kind, &[u8])> = Vec::new();
        if after_index >= 1 && s.kind(after_index - 1) != Kind::Whitespace {
            items.push((Kind::Whitespace, b" "));
        }
        items.push((Kind::Punct, b"{"));
        if after_index + 1 < s.len() && s.kind(after_index + 1) != Kind::Whitespace {
            items.push((Kind::Whitespace, b" "));
        }
        nas_replace(s, after_index, &items);
        return true;
    }

    if !nas_kw_in(s, index, NAS_END) {
        return false;
    }
    let next_index = nas_next_meaningful(s, index);
    nas_replace(s, index, &[(Kind::Punct, b"}")]);
    if let Some(ni) = next_index {
        if nas_is_punct(s, ni, b";") {
            s.remove_at(ni);
        }
    }
    true
}

fn nas_fix_else(s: &mut Stream, index: usize) -> bool {
    match nas_next_meaningful(s, index) {
        Some(a) if nas_is_punct(s, a, b":") => {
            nas_add_braces(s, b"else", index, a);
            true
        }
        _ => false,
    }
}

fn nas_fix_elseif(s: &mut Stream, index: usize) -> bool {
    let paren_end = nas_find_parenthesis_end(s, index);
    match nas_next_meaningful(s, paren_end) {
        Some(a) if nas_is_punct(s, a, b":") => {
            nas_add_braces(s, b"elseif", index, a);
            true
        }
        _ => false,
    }
}

fn no_alternative_syntax(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut index = s.len();
    while index > 0 {
        index -= 1;
        if kw_eq(s, index, b"elseif") {
            changed |= nas_fix_elseif(s, index);
        } else if kw_eq(s, index, b"else") {
            changed |= nas_fix_else(s, index);
        } else {
            changed |= nas_fix_open_close(s, index);
        }
    }
    changed
}

// PHP-CS-Fixer ControlStructureBracesFixer: wrap each control structure body in
// braces. No-op on already-braced code; alternative syntax is left to
// no_alternative_syntax, which runs first.
const CSB_CONTROL: &[&[u8]] = &[
    b"declare", b"do", b"else", b"elseif", b"finally", b"for", b"foreach", b"if",
    b"while", b"try", b"catch", b"switch",
];
const CSB_ALT_TERMINATORS: &[&[u8]] = &[
    b"endif", b"endwhile", b"endfor", b"endforeach", b"endswitch", b"enddeclare",
];

fn csb_is_control(s: &Stream, i: usize) -> bool {
    if s.kind(i) != Kind::Keyword {
        return false;
    }
    let b = s.bytes(i);
    CSB_CONTROL.iter().any(|w| b.eq_ignore_ascii_case(w))
}

fn csb_is_punct(s: &Stream, i: usize, v: &[u8]) -> bool {
    s.kind(i) == Kind::Punct && s.bytes(i) == v
}

fn csb_next_meaningful(s: &Stream, i: usize) -> Option<usize> {
    let mut j = i + 1;
    while j < s.len() {
        let k = s.kind(j);
        if k != Kind::Whitespace && k != Kind::Comment && k != Kind::DocComment {
            return Some(j);
        }
        j += 1;
    }
    None
}

fn csb_find_parenthesis_end(s: &Stream, control_index: usize) -> usize {
    match csb_next_meaningful(s, control_index) {
        Some(ni) if csb_is_punct(s, ni, b"(") => match_forward(s, ni).unwrap_or(control_index),
        _ => control_index,
    }
}

fn csb_in_list(s: &Stream, i: usize, list: &[&[u8]]) -> bool {
    let b = s.bytes(i);
    list.iter().any(|w| b.eq_ignore_ascii_case(w))
}

fn csb_is_alt_syntax(s: &Stream, control_index: usize) -> bool {
    let pe = csb_find_parenthesis_end(s, control_index);
    matches!(csb_next_meaningful(s, pe), Some(a) if csb_is_punct(s, a, b":"))
}

fn csb_continuation(opening: &[u8]) -> &'static [&'static [u8]] {
    match opening {
        b"if" => &[b"else", b"elseif"],
        b"do" => &[b"while"],
        b"try" => &[b"catch", b"finally"],
        _ => &[],
    }
}

fn csb_final_continuation(opening: &[u8]) -> &'static [&'static [u8]] {
    match opening {
        b"if" => &[b"else"],
        b"try" => &[b"finally"],
        _ => &[],
    }
}

fn csb_find_statement_end(s: &Stream, paren_end: usize) -> Option<usize> {
    let next_index = csb_next_meaningful(s, paren_end)?;

    if csb_is_punct(s, next_index, b"{") {
        return match_forward(s, next_index);
    }

    if csb_is_control(s, next_index) {
        let pe = csb_find_parenthesis_end(s, next_index);
        let mut end_index = csb_find_statement_end(s, pe)?;

        let opening = s.bytes(next_index).to_ascii_lowercase();
        if opening == b"if" || opening == b"try" || opening == b"do" {
            loop {
                match csb_next_meaningful(s, end_index) {
                    Some(ni)
                        if s.kind(ni) == Kind::Keyword
                            && csb_in_list(s, ni, csb_continuation(&opening)) =>
                    {
                        let is_final = csb_in_list(s, ni, csb_final_continuation(&opening));
                        let pe = csb_find_parenthesis_end(s, ni);
                        end_index = csb_find_statement_end(s, pe)?;
                        if is_final {
                            return Some(end_index);
                        }
                    }
                    _ => break,
                }
            }
        }
        return Some(end_index);
    }

    let mut index = paren_end;
    loop {
        index += 1;
        if index >= s.len() {
            return None;
        }
        if csb_is_punct(s, index, b"{") {
            index = match_forward(s, index)?;
            continue;
        }
        if csb_is_punct(s, index, b";") {
            return Some(index);
        }
        if s.kind(index) == Kind::CloseTag {
            return prev_significant_index(s, index);
        }
        if s.kind(index) == Kind::Keyword && csb_in_list(s, index, CSB_ALT_TERMINATORS) {
            return None;
        }
    }
}

fn control_structure_braces(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut index = s.len();
    while index > 0 {
        index -= 1;
        if !csb_is_control(s, index) {
            continue;
        }

        if kw_eq(s, index, b"else") {
            if let Some(ni) = csb_next_meaningful(s, index) {
                if kw_eq(s, ni, b"if") {
                    continue;
                }
            }
        }

        let paren_end = csb_find_parenthesis_end(s, index);
        let next_after = match csb_next_meaningful(s, paren_end) {
            Some(n) => n,
            None => continue,
        };

        if csb_is_punct(s, next_after, b";")
            || csb_is_punct(s, next_after, b"{")
            || csb_is_punct(s, next_after, b":")
            || s.kind(next_after) == Kind::CloseTag
        {
            continue;
        }

        if csb_is_control(s, next_after) && csb_is_alt_syntax(s, next_after) {
            continue;
        }

        let statement_end = match csb_find_statement_end(s, paren_end) {
            Some(e) => e,
            None => continue,
        };

        let need_semicolon =
            !(csb_is_punct(s, statement_end, b";") || csb_is_punct(s, statement_end, b"}"));
        // insert closing at statement_end+1, preserving order
        s.insert_owned(statement_end + 1, Kind::Punct, b"}".to_vec());
        s.insert_owned(statement_end + 1, Kind::Whitespace, b" ".to_vec());
        if need_semicolon {
            s.insert_owned(statement_end + 1, Kind::Punct, b";".to_vec());
        }
        // insert opening at paren_end+1
        s.insert_owned(paren_end + 1, Kind::Punct, b"{".to_vec());
        s.insert_owned(paren_end + 1, Kind::Whitespace, b" ".to_vec());
        changed = true;
    }
    changed
}

// PHP-CS-Fixer NoUnneededBracesFixer (NoUnneededCurlyBracesFixer): remove
// superfluous standalone braces and single-element group imports.
fn nub_is_punct(s: &Stream, i: usize, v: &[u8]) -> bool {
    s.kind(i) == Kind::Punct && s.bytes(i) == v
}

fn nub_prev_meaningful(s: &Stream, i: usize) -> Option<usize> {
    let mut j = i as isize - 1;
    while j >= 0 {
        let k = s.kind(j as usize);
        if k != Kind::Whitespace && k != Kind::Comment && k != Kind::DocComment {
            return Some(j as usize);
        }
        j -= 1;
    }
    None
}

fn nub_regular_over_complete(s: &Stream, prev: usize) -> bool {
    if s.kind(prev) == Kind::OpenTag {
        return true;
    }
    if s.kind(prev) != Kind::Punct {
        return false;
    }
    let b = s.bytes(prev);
    b == b"{" || b == b"}" || b == b":" || b == b";"
}

fn nub_group_import_over_complete(s: &Stream, open_index: usize) -> bool {
    let close_index = match match_forward(s, open_index) {
        Some(c) => c,
        None => return false,
    };
    let mut j = open_index + 1;
    while j < close_index {
        if nub_is_punct(s, j, b",") {
            return false;
        }
        if s.kind(j) == Kind::Whitespace && s.bytes(j).iter().any(|&c| c == b'\n' || c == b'\r') {
            return false;
        }
        j += 1;
    }
    true
}

fn no_unneeded_braces(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = s.len();
    while i > 1 {
        i -= 1;
        if !nub_is_punct(s, i, b"{") {
            continue;
        }
        let prev = match nub_prev_meaningful(s, i) {
            Some(p) => p,
            None => continue,
        };
        if nub_is_punct(s, prev, br"\") {
            if nub_group_import_over_complete(s, i) {
                if let Some(close_index) = match_forward(s, i) {
                    s.remove_at(close_index);
                    s.remove_at(i);
                    changed = true;
                }
            }
            continue;
        }
        if nub_regular_over_complete(s, prev) {
            if let Some(close_index) = match_forward(s, i) {
                s.remove_at(close_index);
                s.remove_at(i);
                changed = true;
            }
        }
    }
    changed
}

// PHP-CS-Fixer NoUnreachableDefaultArgumentValueFixer: remove default values of
// arguments that precede a required one. "= null" on a non-nullable typed
// argument is kept.
struct NudArg {
    name_index: Option<usize>,
    variadic: bool,
    equals_idx: Option<usize>,
    default_end: usize,
    null_dflt: bool,
    has_type: bool,
    nullable: bool,
}

fn nud_next_paren(s: &Stream, index: usize) -> Option<usize> {
    let mut j = index + 1;
    while j < s.len() {
        if s.kind(j) == Kind::Punct && s.bytes(j) == b"(" {
            return Some(j);
        }
        j += 1;
    }
    None
}

fn nud_is_insignificant(k: Kind) -> bool {
    k == Kind::Whitespace || k == Kind::Comment || k == Kind::DocComment
}

fn nud_analyze_arg(s: &Stream, start: usize, end: usize) -> Option<NudArg> {
    let mut a = NudArg {
        name_index: None,
        variadic: false,
        equals_idx: None,
        default_end: 0,
        null_dflt: false,
        has_type: false,
        nullable: false,
    };
    let mut depth = 0i32;
    let mut i = start;
    while i <= end {
        let k = s.kind(i);
        if k == Kind::Punct {
            match s.bytes(i) {
                b"(" | b"[" | b"{" => {
                    depth += 1;
                    i += 1;
                    continue;
                }
                b")" | b"]" | b"}" => {
                    depth -= 1;
                    i += 1;
                    continue;
                }
                _ => {}
            }
        }
        if depth == 0 {
            if k == Kind::Variable && a.name_index.is_none() && a.equals_idx.is_none() {
                a.name_index = Some(i);
            } else if k == Kind::Punct
                && s.bytes(i) == b"="
                && a.name_index.is_some()
                && a.equals_idx.is_none()
            {
                a.equals_idx = Some(i);
            }
        }
        i += 1;
    }
    let name_index = a.name_index?;

    // variadic: "..." right before the name
    let mut j = name_index as isize - 1;
    while j >= start as isize {
        if !nud_is_insignificant(s.kind(j as usize)) {
            if s.kind(j as usize) == Kind::Punct && s.bytes(j as usize) == b"..." {
                a.variadic = true;
            }
            break;
        }
        j -= 1;
    }

    // type tokens before the name (excluding "&" and "...")
    for t in start..name_index {
        let k = s.kind(t);
        if nud_is_insignificant(k) {
            continue;
        }
        if k == Kind::Punct && (s.bytes(t) == b"&" || s.bytes(t) == b"...") {
            continue;
        }
        a.has_type = true;
        if k == Kind::Punct && s.bytes(t) == b"?" {
            a.nullable = true;
        }
        if k == Kind::Ident && s.bytes(t).eq_ignore_ascii_case(b"null") {
            a.nullable = true;
        }
    }

    if let Some(eq) = a.equals_idx {
        let mut de = end;
        while de > eq && nud_is_insignificant(s.kind(de)) {
            de -= 1;
        }
        a.default_end = de;
        let mut meaningful: Vec<usize> = Vec::new();
        for t in (eq + 1)..=end {
            if !nud_is_insignificant(s.kind(t)) {
                meaningful.push(t);
            }
        }
        if meaningful.len() == 1 {
            let v = meaningful[0];
            if (s.kind(v) == Kind::Ident || s.kind(v) == Kind::Keyword)
                && s.bytes(v).eq_ignore_ascii_case(b"null")
            {
                a.null_dflt = true;
            }
        }
    }
    Some(a)
}

fn nud_parse_args(s: &Stream, open_paren: usize, close_paren: usize) -> Vec<NudArg> {
    let mut args = Vec::new();
    let mut depth = 0i32;
    let mut start = open_paren + 1;
    let mut i = open_paren + 1;
    while i < close_paren {
        if s.kind(i) == Kind::Punct {
            match s.bytes(i) {
                b"(" | b"[" | b"{" => depth += 1,
                b")" | b"]" | b"}" => depth -= 1,
                b"," if depth == 0 => {
                    if start <= i.wrapping_sub(1) {
                        if let Some(a) = nud_analyze_arg(s, start, i - 1) {
                            args.push(a);
                        }
                    }
                    start = i + 1;
                }
                _ => {}
            }
        }
        i += 1;
    }
    if start <= close_paren - 1 {
        if let Some(a) = nud_analyze_arg(s, start, close_paren - 1) {
            args.push(a);
        }
    }
    args
}

fn nud_fix_function(s: &mut Stream, fn_index: usize) -> bool {
    let open_paren = match nud_next_paren(s, fn_index) {
        Some(o) => o,
        None => return false,
    };
    let close_paren = match match_forward(s, open_paren) {
        Some(c) => c,
        None => return false,
    };
    let args = nud_parse_args(s, open_paren, close_paren);

    let mut changed = false;
    let mut remove_default = false;
    for a in args.iter().rev() {
        if a.variadic {
            continue;
        }
        let eq = match a.equals_idx {
            None => {
                remove_default = true;
                continue;
            }
            Some(e) => e,
        };
        if !remove_default {
            continue;
        }
        if a.null_dflt && a.has_type && !a.nullable {
            continue;
        }
        let mut j = a.default_end;
        loop {
            s.remove_at(j);
            if j == eq {
                break;
            }
            j -= 1;
        }
        if eq >= 1 && s.kind(eq - 1) == Kind::Whitespace {
            if let Some(p) = prev_significant_index(s, eq - 1) {
                if s.kind(p) != Kind::Comment {
                    s.remove_at(eq - 1);
                }
            }
        }
        changed = true;
    }
    changed
}

fn no_unreachable_default_argument_value(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = s.len();
    while i > 0 {
        i -= 1;
        if s.kind(i) == Kind::Keyword
            && (s.bytes(i).eq_ignore_ascii_case(b"function") || s.bytes(i).eq_ignore_ascii_case(b"fn"))
        {
            changed |= nud_fix_function(s, i);
        }
    }
    changed
}

const OP_LB_PUNCT: &[&[u8]] = &[
    b"||", b"&&", b".", b"+", b"-", b"*", b"/", b"%", b"**", b"==", b"===", b"!=", b"!==",
    b"<>", b"<", b">", b"<=", b">=", b"<=>", b"??", b"=>", b"=", b".=", b"+=", b"-=", b"*=",
    b"/=", b"%=", b"**=", b"&=", b"|=", b"^=", b"<<=", b">>=", b"??=", b"^", b"<<", b">>",
    b"|", b"&", b":", b"?",
];

fn is_op_lb_token(s: &Stream, i: usize) -> bool {
    match s.kind(i) {
        Kind::Keyword => matches!(
            s.bytes(i).to_ascii_lowercase().as_slice(),
            b"and" | b"or" | b"xor"
        ),
        Kind::Punct => OP_LB_PUNCT.contains(&s.bytes(i)),
        _ => false,
    }
}

fn operator_span_multiline(s: &Stream, from: usize, to: usize) -> bool {
    let mut j = from;
    while j <= to && j < s.len() {
        if s.bytes(j).contains(&b'\n') || s.bytes(j).contains(&b'\r') {
            return true;
        }
        j += 1;
    }
    false
}

fn ol_kw_lower(s: &Stream, i: usize) -> Vec<u8> {
    if s.kind(i) == Kind::Keyword {
        s.bytes(i).to_ascii_lowercase()
    } else {
        Vec::new()
    }
}

fn is_type_colon(s: &Stream, index: usize) -> bool {
    if !is_punct_val(s, index, b":") {
        return false;
    }
    let end_index = match sig_prev(s, index) {
        Some(e) => e,
        None => return false,
    };
    if let Some(pe) = sig_prev(s, end_index) {
        if kw_is(s, pe, b"enum") {
            return true;
        }
    }
    if !is_punct_val(s, end_index, b")") {
        return false;
    }
    let start_index = match match_backward(s, end_index) {
        Some(x) => x,
        None => return false,
    };
    let mut prev_index = match sig_prev(s, start_index) {
        Some(p) => p,
        None => return false,
    };
    if s.kind(prev_index) == Kind::Ident {
        prev_index = match sig_prev(s, prev_index) {
            Some(p) => p,
            None => return false,
        };
    }
    matches!(ol_kw_lower(s, prev_index).as_slice(), b"function" | b"fn" | b"use")
        || (is_punct_val(s, prev_index, b"&") && is_return_ref(s, prev_index))
}

fn is_named_argument_colon(s: &Stream, index: usize) -> bool {
    if !is_punct_val(s, index, b":") {
        return false;
    }
    let string_index = match sig_prev(s, index) {
        Some(x) if s.kind(x) == Kind::Ident => x,
        _ => return false,
    };
    match sig_prev(s, string_index) {
        Some(p) => is_punct_val(s, p, b",") || is_punct_val(s, p, b"("),
        None => false,
    }
}

fn is_nullable_type(s: &Stream, index: usize) -> bool {
    if !is_punct_val(s, index, b"?") {
        return false;
    }
    let prev_index = match sig_prev(s, index) {
        Some(p) => p,
        None => return false,
    };
    let mut ok = is_punct_val(s, prev_index, b"(")
        || is_punct_val(s, prev_index, b",")
        || is_type_colon(s, prev_index);
    if !ok {
        ok = matches!(
            ol_kw_lower(s, prev_index).as_slice(),
            b"public" | b"protected" | b"private" | b"var" | b"static" | b"const" | b"abstract"
                | b"final" | b"readonly"
        );
    }
    if !ok {
        return false;
    }
    if ol_kw_lower(s, prev_index) == b"static" {
        if let Some(pp) = sig_prev(s, prev_index) {
            if kw_is(s, pp, b"instanceof") {
                return false;
            }
        }
    }
    true
}

fn is_return_ref(s: &Stream, index: usize) -> bool {
    if !is_punct_val(s, index, b"&") {
        return false;
    }
    match sig_prev(s, index) {
        Some(p) => kw_is(s, p, b"function") || kw_is(s, p, b"fn"),
        None => false,
    }
}

fn is_reference_amp(s: &Stream, index: usize) -> bool {
    if !is_punct_val(s, index, b"&") {
        return false;
    }
    let mut idx = match sig_prev(s, index) {
        Some(p) => p,
        None => return false,
    };
    if is_punct_val(s, idx, b"=")
        || is_punct_val(s, idx, b"=>")
        || kw_is(s, idx, b"as")
        || kw_is(s, idx, b"callable")
        || kw_is(s, idx, b"array")
    {
        return true;
    }
    if s.kind(idx) == Kind::Ident {
        idx = match sig_prev(s, idx) {
            Some(p) => p,
            None => return false,
        };
    }
    is_punct_val(s, idx, b"(")
        || is_punct_val(s, idx, b",")
        || is_punct_val(s, idx, b"\\")
        || (is_punct_val(s, idx, b"?") && is_nullable_type(s, idx))
}

fn belongs_to_goto_label(s: &Stream, index: usize) -> bool {
    if !is_punct_val(s, index, b":") {
        return false;
    }
    let prev = match sig_prev(s, index) {
        Some(p) if s.kind(p) == Kind::Ident => p,
        _ => return false,
    };
    let prev2 = match sig_prev(s, prev) {
        Some(p) => p,
        None => return false,
    };
    is_punct_val(s, prev2, b":")
        || is_punct_val(s, prev2, b";")
        || is_punct_val(s, prev2, b"{")
        || is_punct_val(s, prev2, b"}")
        || s.kind(prev2) == Kind::OpenTag
}

fn belongs_to_alternative_syntax(s: &Stream, index: usize) -> bool {
    if !is_punct_val(s, index, b":") {
        return false;
    }
    let prev = match sig_prev(s, index) {
        Some(p) => p,
        None => return false,
    };
    if kw_is(s, prev, b"else") {
        return true;
    }
    if !is_punct_val(s, prev, b")") {
        return false;
    }
    let open = match match_backward(s, prev) {
        Some(x) => x,
        None => return false,
    };
    let before = match sig_prev(s, open) {
        Some(b) => b,
        None => return false,
    };
    matches!(
        ol_kw_lower(s, before).as_slice(),
        b"declare" | b"elseif" | b"for" | b"foreach" | b"if" | b"switch" | b"while"
    )
}

fn ol_type_skip_token(s: &Stream, j: usize) -> bool {
    match s.kind(j) {
        Kind::Whitespace | Kind::Comment | Kind::DocComment | Kind::Ident => true,
        Kind::Punct => matches!(s.bytes(j), b"|" | b"&" | b"(" | b")" | b"\\"),
        Kind::Keyword => matches!(
            s.bytes(j).to_ascii_lowercase().as_slice(),
            b"callable" | b"static" | b"array"
        ),
        _ => false,
    }
}

fn ol_not_of_kind_sibling_type(s: &Stream, index: usize, dir: i32) -> Option<usize> {
    let mut j = index as isize + dir as isize;
    while j >= 0 && (j as usize) < s.len() {
        if !ol_type_skip_token(s, j as usize) {
            return Some(j as usize);
        }
        j += dir as isize;
    }
    None
}

fn ol_is_type_end_token(s: &Stream, idx: usize) -> bool {
    if is_punct_val(s, idx, b")") || is_punct_val(s, idx, b"\\") {
        return true;
    }
    if s.kind(idx) == Kind::Ident {
        return true;
    }
    matches!(
        ol_kw_lower(s, idx).as_slice(),
        b"callable" | b"static" | b"array"
    )
}

fn ol_get_prev_token_of_kind_type(s: &Stream, index: usize) -> Option<usize> {
    let mut j = index as isize - 1;
    while j >= 0 {
        let k = j as usize;
        if is_punct_val(s, k, b"{")
            || is_punct_val(s, k, b"}")
            || is_punct_val(s, k, b";")
            || s.kind(k) == Kind::CloseTag
            || kw_is(s, k, b"fn")
            || kw_is(s, k, b"function")
        {
            return Some(k);
        }
        j -= 1;
    }
    None
}

fn is_part_of_type(s: &Stream, index: usize) -> bool {
    if !is_punct_val(s, index, b"|") && !is_punct_val(s, index, b"&") {
        return false;
    }
    if let Some(tc) = ol_not_of_kind_sibling_type(s, index, -1) {
        if kw_is(s, tc, b"catch") || kw_is(s, tc, b"const") || is_type_colon(s, tc) {
            return true;
        }
    }
    let after_type_index = match ol_not_of_kind_sibling_type(s, index, 1) {
        Some(a) => a,
        None => return false,
    };
    if is_punct_val(s, after_type_index, b"...") {
        return true;
    }
    if s.kind(after_type_index) != Kind::Variable {
        return false;
    }
    let before_var = match sig_prev(s, after_type_index) {
        Some(b) => b,
        None => return false,
    };
    if is_punct_val(s, before_var, b"&") {
        return match ol_get_prev_token_of_kind_type(s, index) {
            Some(p) => kw_is(s, p, b"fn") || kw_is(s, p, b"function"),
            None => false,
        };
    }
    ol_is_type_end_token(s, before_var)
}

fn ol_case_colon_after(s: &Stream, case_idx: usize, limit: usize) -> Option<usize> {
    let mut depth = 0i32;
    let mut j = case_idx + 1;
    while j < limit && j < s.len() {
        if s.kind(j) == Kind::Punct {
            match s.bytes(j) {
                b"(" | b"[" | b"{" => depth += 1,
                b")" | b"]" | b"}" => depth -= 1,
                b":" if depth == 0 => return Some(j),
                b";" if depth == 0 => return None,
                _ => {}
            }
        }
        j += 1;
    }
    None
}

fn ol_alt_switch_end(s: &Stream, colon_index: usize) -> Option<usize> {
    let mut depth = 0i32;
    let mut j = colon_index + 1;
    while j < s.len() {
        if kw_is(s, j, b"switch") {
            depth += 1;
        } else if kw_is(s, j, b"endswitch") {
            if depth == 0 {
                return Some(j);
            }
            depth -= 1;
        }
        j += 1;
    }
    None
}

fn ol_scan_case_colons(s: &Stream, from: usize, to: usize, result: &mut std::collections::HashSet<usize>) {
    let mut depth = 0i32;
    let mut j = from;
    while j < to && j < s.len() {
        if is_punct_val(s, j, b"{") {
            depth += 1;
        } else if is_punct_val(s, j, b"}") {
            depth -= 1;
        } else if depth == 0 && (kw_is(s, j, b"case") || kw_is(s, j, b"default")) {
            if let Some(c) = ol_case_colon_after(s, j, to) {
                result.insert(c);
            }
        }
        j += 1;
    }
}

fn switch_case_colons(s: &Stream) -> std::collections::HashSet<usize> {
    let mut result = std::collections::HashSet::new();
    let mut i = 0;
    while i < s.len() {
        if kw_is(s, i, b"switch") {
            if let Some(open) = next_punct_of_kind(s, i, &[b"("]) {
                if let Some(close_idx) = match_forward(s, open) {
                    if let Some(body_start) = sig_next(s, close_idx) {
                        if is_punct_val(s, body_start, b"{") {
                            if let Some(body_end) = match_forward(s, body_start) {
                                ol_scan_case_colons(s, body_start + 1, body_end, &mut result);
                            }
                        } else if is_punct_val(s, body_start, b":") {
                            result.insert(body_start);
                            if let Some(end) = ol_alt_switch_end(s, body_start) {
                                if end > body_start {
                                    ol_scan_case_colons(s, body_start + 1, end, &mut result);
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    result
}

fn operator_linebreak_excluded(s: &Stream, i: usize, switch_colons: &std::collections::HashSet<usize>) -> bool {
    if s.kind(i) != Kind::Punct {
        return false;
    }
    match s.bytes(i) {
        b":" => {
            is_type_colon(s, i)
                || is_named_argument_colon(s, i)
                || belongs_to_goto_label(s, i)
                || belongs_to_alternative_syntax(s, i)
                || switch_colons.contains(&i)
        }
        b"?" => is_nullable_type(s, i),
        b"|" => is_part_of_type(s, i),
        b"&" => is_part_of_type(s, i) || is_return_ref(s, i) || is_reference_amp(s, i),
        _ => false,
    }
}

// OperatorLinebreak: moves a multiline operator to the start of the next line,
// handling the full operator set incl ":" "?" "|" "&" via CT-context classifiers
// (type/nullable/union/intersection/named-arg/reference/label/switch/alt-syntax).
fn operator_linebreak(s: &mut Stream) -> bool {
    let mut changed = false;
    if s.len() == 0 {
        return false;
    }
    let switch_colons = switch_case_colons(s);
    let mut i = s.len();
    while i > 1 {
        i -= 1;
        if !is_op_lb_token(s, i) {
            continue;
        }
        if operator_linebreak_excluded(s, i, &switch_colons) {
            continue;
        }

        let mut op_indices = vec![i];
        if is_punct_val(s, i, b":") {
            if let Some(p) = sig_prev(s, i) {
                if is_punct_val(s, p, b"?") {
                    op_indices = vec![p, i];
                }
            }
        }
        let lo = op_indices[0];
        let hi = op_indices[op_indices.len() - 1];
        let (pm, nm) = match (sig_prev(s, lo), sig_next(s, hi)) {
            (Some(a), Some(b)) => (a, b),
            _ => continue,
        };
        if !operator_span_multiline(s, pm + 1, nm - 1) {
            continue;
        }
        if !operator_span_multiline(s, hi, nm - 1) {
            continue;
        }

        let had_space = lo > 0 && s.kind(lo - 1) == Kind::Whitespace;
        let clones: Vec<(Kind, Vec<u8>)> =
            op_indices.iter().map(|&idx| (s.kind(idx), s.bytes(idx).to_vec())).collect();
        for &idx in &op_indices {
            if idx > 0 && s.kind(idx - 1) == Kind::Whitespace {
                s.set_owned(idx - 1, Vec::new());
            }
            s.set_owned(idx, Vec::new());
        }
        let mut at = nm;
        for (k, v) in clones {
            s.insert_owned(at, k, v);
            at += 1;
        }
        if had_space {
            s.insert_owned(at, Kind::Whitespace, b" ".to_vec());
        }
        changed = true;
        i = lo;
    }
    changed
}

// --- LambdaNotUsedImport ---

const LAMBDA_SUPERGLOBALS: &[&[u8]] = &[
    b"$GLOBALS", b"$_SERVER", b"$_GET", b"$_POST", b"$_REQUEST", b"$_SESSION", b"$_ENV",
    b"$_COOKIE", b"$_FILES",
];

fn lambda_is_global_function_call(s: &Stream, i: usize) -> bool {
    match sig_prev(s, i) {
        None => true,
        Some(p) => {
            if s.kind(p) == Kind::Punct {
                match s.bytes(p) {
                    b"->" | b"?->" | b"::" => return false,
                    b"\\" => {
                        if let Some(before) = sig_prev(s, p) {
                            if s.kind(before) == Kind::Ident {
                                return false;
                            }
                        }
                        return true;
                    }
                    _ => {}
                }
            }
            if s.kind(p) == Kind::Keyword {
                match s.bytes(p).to_ascii_lowercase().as_slice() {
                    b"function" | b"const" | b"new" | b"goto" => return false,
                    _ => {}
                }
            }
            true
        }
    }
}

fn lambda_is_global_call(s: &Stream, i: usize) -> bool {
    matches!(sig_next(s, i), Some(n) if is_punct_val(s, n, b"(")) && lambda_is_global_function_call(s, i)
}

fn is_lambda_function(s: &Stream, index: usize) -> bool {
    matches!(sig_next(s, index), Some(n) if is_punct_val(s, n, b"(") || is_punct_val(s, n, b"&"))
}

fn lambda_use_index(s: &Stream, index: usize) -> Option<usize> {
    if !kw_is(s, index, b"function") || !is_lambda_function(s, index) {
        return None;
    }
    let mut u = sig_next(s, index)?;
    if is_punct_val(s, u, b"&") {
        u = sig_next(s, u)?;
    }
    if !is_punct_val(s, u, b"(") {
        return None;
    }
    let close_idx = match_forward(s, u)?;
    let use_idx = sig_next(s, close_idx)?;
    if !kw_is(s, use_idx, b"use") {
        return None;
    }
    Some(use_idx)
}

fn lambda_is_trivia(s: &Stream, i: usize) -> bool {
    matches!(s.kind(i), Kind::Whitespace | Kind::Comment | Kind::DocComment)
}

fn lambda_argument_spans(s: &Stream, open: usize, close_idx: usize) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut depth = 0i32;
    let mut start: isize = -1;
    let mut flush = |s: &Stream, spans: &mut Vec<(usize, usize)>, start: &mut isize, end: usize| {
        if *start < 0 {
            return;
        }
        let mut e = end as isize;
        while e >= *start && lambda_is_trivia(s, e as usize) {
            e -= 1;
        }
        let mut st = *start;
        while st <= e && lambda_is_trivia(s, st as usize) {
            st += 1;
        }
        if st <= e {
            spans.push((st as usize, e as usize));
        }
        *start = -1;
    };
    let mut j = open + 1;
    while j < close_idx {
        if s.kind(j) == Kind::Punct {
            match s.bytes(j) {
                b"(" | b"[" | b"{" => depth += 1,
                b")" | b"]" | b"}" => depth -= 1,
                b"," if depth == 0 => {
                    flush(s, &mut spans, &mut start, j - 1);
                    j += 1;
                    continue;
                }
                _ => {}
            }
        }
        if start < 0 && !lambda_is_trivia(s, j) {
            start = j as isize;
        }
        j += 1;
    }
    flush(s, &mut spans, &mut start, close_idx - 1);
    spans
}

fn filter_lambda_imports(s: &Stream, args: &[(usize, usize)]) -> Vec<(Vec<u8>, usize)> {
    let mut imports = Vec::new();
    for &(a, b) in args {
        let mut var_idx = None;
        for j in a..=b {
            if s.kind(j) == Kind::Variable {
                var_idx = Some(j);
                break;
            }
        }
        let var_idx = match var_idx {
            Some(v) => v,
            None => continue,
        };
        if let Some(p) = sig_prev(s, var_idx) {
            if is_punct_val(s, p, b"&") {
                continue;
            }
        }
        let name = s.bytes(var_idx);
        if name == b"$this" || LAMBDA_SUPERGLOBALS.contains(&name) {
            continue;
        }
        imports.push((name.to_vec(), var_idx));
    }
    imports
}

fn lambda_arg_name(s: &Stream, sp: (usize, usize)) -> Option<Vec<u8>> {
    let mut var_idx = None;
    for j in sp.0..=sp.1 {
        if s.kind(j) == Kind::Variable {
            if var_idx.is_some() {
                return None;
            }
            var_idx = Some(j);
        }
    }
    let var_idx = var_idx?;
    if let Some(n) = sig_next(s, var_idx) {
        if n <= sp.1 && !is_punct_val(s, n, b"=") {
            return None;
        }
    }
    Some(s.bytes(var_idx).to_vec())
}

fn count_imports_used_as_argument(
    s: &Stream,
    remaining: &mut std::collections::HashSet<Vec<u8>>,
    args: &[(usize, usize)],
) {
    for &sp in args {
        if let Some(name) = lambda_arg_name(s, sp) {
            remaining.remove(&name);
        }
    }
}

fn is_lambda_bailout_keyword(s: &Stream, i: usize) -> bool {
    if s.kind(i) != Kind::Keyword {
        return false;
    }
    matches!(
        s.bytes(i).to_ascii_lowercase().as_slice(),
        b"eval" | b"include" | b"include_once" | b"require" | b"require_once"
    )
}

fn find_not_used_lambda_imports(
    s: &Stream,
    imports: &[(Vec<u8>, usize)],
    use_close_brace: usize,
) -> Vec<(Vec<u8>, usize)> {
    let lambda_open = match next_punct_of_kind(s, use_close_brace, &[b"{"]) {
        Some(o) => o,
        None => return Vec::new(),
    };
    let mut remaining: std::collections::HashSet<Vec<u8>> =
        imports.iter().map(|(n, _)| n.clone()).collect();
    let mut level = 0i32;
    let mut i = lambda_open;
    while i < s.len() {
        if is_punct_val(s, i, b"{") {
            level += 1;
            i += 1;
            continue;
        }
        if is_punct_val(s, i, b"}") {
            level -= 1;
            if level == 0 {
                break;
            }
            i += 1;
            continue;
        }
        if s.kind(i) == Kind::Ident
            && s.bytes(i).eq_ignore_ascii_case(b"compact")
            && lambda_is_global_call(s, i)
        {
            return Vec::new();
        }
        if is_lambda_bailout_keyword(s, i) {
            return Vec::new();
        }
        if is_punct_val(s, i, b"$") {
            if let Some(n) = sig_next(s, i) {
                if s.kind(n) == Kind::Variable || is_punct_val(s, n, b"{") {
                    return Vec::new();
                }
            }
        }
        if s.kind(i) == Kind::Variable && remaining.contains(s.bytes(i)) {
            remaining.remove(s.bytes(i));
            if remaining.is_empty() {
                return Vec::new();
            }
        }
        if s.kind(i) == Kind::Keyword && is_classy_keyword(&s.bytes(i).to_ascii_lowercase()) {
            let mut j = match next_punct_of_kind(s, i, &[b"(", b"{"]) {
                Some(x) => x,
                None => break,
            };
            if is_punct_val(s, j, b"(") {
                let cb = match match_forward(s, j) {
                    Some(x) => x,
                    None => break,
                };
                let spans = lambda_argument_spans(s, j, cb);
                count_imports_used_as_argument(s, &mut remaining, &spans);
                j = match next_punct_of_kind(s, cb, &[b"{"]) {
                    Some(x) => x,
                    None => break,
                };
            }
            i = match match_forward(s, j) {
                Some(x) => x,
                None => break,
            };
            i += 1;
            continue;
        }
        if kw_is(s, i, b"function") {
            let o = match next_punct_of_kind(s, i, &[b"("]) {
                Some(x) => x,
                None => break,
            };
            let c = match match_forward(s, o) {
                Some(x) => x,
                None => break,
            };
            let spans = lambda_argument_spans(s, o, c);
            count_imports_used_as_argument(s, &mut remaining, &spans);
            let mut j = match lambda_next_use_or_brace(s, i) {
                Some(x) => x,
                None => break,
            };
            if kw_is(s, j, b"use") {
                let o2 = match next_punct_of_kind(s, j, &[b"("]) {
                    Some(x) => x,
                    None => break,
                };
                let c2 = match match_forward(s, o2) {
                    Some(x) => x,
                    None => break,
                };
                let spans2 = lambda_argument_spans(s, o2, c2);
                count_imports_used_as_argument(s, &mut remaining, &spans2);
                j = match next_punct_of_kind(s, c2, &[b"{"]) {
                    Some(x) => x,
                    None => break,
                };
            }
            i = match match_forward(s, j) {
                Some(x) => x,
                None => break,
            };
            i += 1;
            continue;
        }
        i += 1;
    }
    imports
        .iter()
        .filter(|(n, _)| remaining.contains(n))
        .cloned()
        .collect()
}

fn lambda_next_use_or_brace(s: &Stream, idx: usize) -> Option<usize> {
    let mut j = idx + 1;
    while j < s.len() {
        if kw_is(s, j, b"use") || is_punct_val(s, j, b"{") {
            return Some(j);
        }
        j += 1;
    }
    None
}

fn lambda_non_empty_sibling(s: &Stream, i: usize, dir: i32) -> Option<usize> {
    let mut j = i as isize + dir as isize;
    while j >= 0 && (j as usize) < s.len() {
        if !s.bytes(j as usize).is_empty() {
            return Some(j as usize);
        }
        j += dir as isize;
    }
    None
}

fn lambda_clear_token_at(s: &mut Stream, i: usize) {
    s.set_owned(i, Vec::new());
    s.set_kind(i, Kind::Whitespace);
}

fn lambda_clear_and_merge_ws(s: &mut Stream, index: usize) {
    if index >= s.len() {
        return;
    }
    let count = s.len();
    lambda_clear_token_at(s, index);
    if index == count - 1 {
        return;
    }
    let next_idx = match lambda_non_empty_sibling(s, index, 1) {
        Some(n) if s.kind(n) == Kind::Whitespace => n,
        _ => return,
    };
    let prev_idx = lambda_non_empty_sibling(s, index, -1);
    match prev_idx {
        Some(p) if s.kind(p) == Kind::Whitespace => {
            let mut merged = s.bytes(p).to_vec();
            merged.extend_from_slice(s.bytes(next_idx));
            s.set_owned(p, merged);
        }
        Some(p) if p + 1 < s.len() && s.bytes(p + 1).is_empty() => {
            let v = s.bytes(next_idx).to_vec();
            s.set_owned(p + 1, v);
            s.set_kind(p + 1, Kind::Whitespace);
        }
        _ => {}
    }
    lambda_clear_token_at(s, next_idx);
}

fn clear_lambda_import(s: &mut Stream, remove_idx: usize) {
    lambda_clear_and_merge_ws(s, remove_idx);
    let prev = sig_prev(s, remove_idx);
    match prev {
        Some(p) if is_punct_val(s, p, b",") => lambda_clear_and_merge_ws(s, p),
        Some(p) if is_punct_val(s, p, b"(") => {
            if let Some(n) = sig_next(s, remove_idx) {
                lambda_clear_and_merge_ws(s, n);
            }
        }
        _ => {}
    }
}

fn clear_imports_and_use(s: &mut Stream, use_idx: usize, use_close_brace: usize) {
    let mut i = use_close_brace as isize;
    while i >= use_idx as isize {
        let ii = i as usize;
        if matches!(s.kind(ii), Kind::Comment | Kind::DocComment) {
            i -= 1;
            continue;
        }
        if s.kind(ii) == Kind::Whitespace {
            if let Some(pv) = get_prev_non_whitespace(s, ii) {
                if matches!(s.kind(pv), Kind::Comment | Kind::DocComment) {
                    i -= 1;
                    continue;
                }
            }
        }
        lambda_clear_and_merge_ws(s, ii);
        i -= 1;
    }
}

fn fix_lambda_imports(s: &mut Stream, use_idx: usize) {
    let open = match next_punct_of_kind(s, use_idx, &[b"("]) {
        Some(o) => o,
        None => return,
    };
    let close_idx = match match_forward(s, open) {
        Some(c) => c,
        None => return,
    };
    let args = lambda_argument_spans(s, open, close_idx);
    let imports = filter_lambda_imports(s, &args);
    if imports.is_empty() {
        return;
    }
    let not_used = find_not_used_lambda_imports(s, &imports, close_idx);
    if not_used.is_empty() {
        return;
    }
    if not_used.len() == args.len() {
        clear_imports_and_use(s, use_idx, close_idx);
        return;
    }
    for (_, idx) in not_used.iter().rev() {
        clear_lambda_import(s, *idx);
    }
}

fn lambda_compact_empty_tokens(s: &mut Stream) {
    let mut i = s.len();
    while i > 0 {
        i -= 1;
        if s.kind(i) == Kind::Whitespace && s.bytes(i).is_empty() {
            s.remove_at(i);
        }
    }
}

fn lambda_not_used_import(s: &mut Stream) -> bool {
    let before = s.render();
    if s.len() >= 4 {
        let mut i = s.len() - 4;
        while i > 0 {
            if let Some(use_idx) = lambda_use_index(s, i) {
                fix_lambda_imports(s, use_idx);
            }
            i -= 1;
        }
    }
    lambda_compact_empty_tokens(s);
    s.render() != before
}

// --- Doctrine annotation parser (DocLexer + Tokens) ---

const DA_T_NONE: i32 = 1;
const DA_T_STRING: i32 = 3;
const DA_T_IDENTIFIER: i32 = 100;
const DA_T_AT: i32 = 101;
const DA_T_CLOSE_CURLY: i32 = 102;
const DA_T_CLOSE_PAREN: i32 = 103;
const DA_T_EQUALS: i32 = 105;
const DA_T_OPEN_CURLY: i32 = 108;
const DA_T_OPEN_PAREN: i32 = 109;
const DA_T_COLON: i32 = 112;

struct DaToken {
    typ: i32,
    content: Vec<u8>,
    pos: usize,
}

fn da_is_alpha_us(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}
fn da_is_word(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}
fn da_is_word_cb(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b':' || b == b'\\'
}
fn da_is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

// mirrors DocLexer identifier pattern [a-z_\\][a-z0-9_:\\]*[a-z_][a-z0-9_]* (i)
fn da_match_identifier(b: &[u8], i: usize) -> Option<usize> {
    if !(da_is_alpha_us(b[i]) || b[i] == b'\\') {
        return None;
    }
    let mut k = i;
    while k < b.len() && da_is_word_cb(b[k]) {
        k += 1;
    }
    let mut e = k;
    while e > i + 1 && (b[e - 1] == b':' || b[e - 1] == b'\\') {
        e -= 1;
    }
    if e < i + 2 {
        return None;
    }
    // need a Z (alpha_) at some z in [i+1, e) with b[z+1..e) all word
    let mut m = e;
    while m > i && da_is_word(b[m - 1]) {
        m -= 1;
    }
    let lo = if m > i + 1 { m } else { i + 1 };
    let mut z = lo;
    while z < e {
        if da_is_alpha_us(b[z]) {
            return Some(e);
        }
        z += 1;
    }
    None
}

fn da_match_number(b: &[u8], i: usize) -> Option<usize> {
    let mut j = i;
    if j < b.len() && (b[j] == b'+' || b[j] == b'-') {
        j += 1;
    }
    let ds = j;
    while j < b.len() && b[j].is_ascii_digit() {
        j += 1;
    }
    if j == ds {
        return None;
    }
    // (?:\.[0-9]+)*
    loop {
        if j < b.len() && b[j] == b'.' && j + 1 < b.len() && b[j + 1].is_ascii_digit() {
            j += 1;
            while j < b.len() && b[j].is_ascii_digit() {
                j += 1;
            }
        } else {
            break;
        }
    }
    // (?:[eE][+-]?[0-9]+)?
    if j < b.len() && (b[j] == b'e' || b[j] == b'E') {
        let mut p = j + 1;
        if p < b.len() && (b[p] == b'+' || b[p] == b'-') {
            p += 1;
        }
        let es = p;
        while p < b.len() && b[p].is_ascii_digit() {
            p += 1;
        }
        if p > es {
            j = p;
        }
    }
    Some(j)
}

fn da_match_string(b: &[u8], i: usize) -> Option<usize> {
    if b[i] != b'"' {
        return None;
    }
    let mut j = i + 1;
    while j < b.len() {
        if b[j] == b'"' {
            if j + 1 < b.len() && b[j + 1] == b'"' {
                j += 2;
                continue;
            }
            return Some(j + 1);
        }
        j += 1;
    }
    // unterminated: PCRE would not match the string branch
    None
}

fn da_get_type(value: &[u8]) -> i32 {
    if value.is_empty() {
        return DA_T_NONE;
    }
    if value[0] == b'"' {
        return DA_T_STRING;
    }
    match value {
        b"@" => return DA_T_AT,
        b"," => return 104,
        b"(" => return DA_T_OPEN_PAREN,
        b")" => return DA_T_CLOSE_PAREN,
        b"{" => return DA_T_OPEN_CURLY,
        b"}" => return DA_T_CLOSE_CURLY,
        b"=" => return DA_T_EQUALS,
        b":" => return DA_T_COLON,
        b"-" => return 113,
        b"\\" => return 107,
        _ => {}
    }
    if value[0] == b'_' || value[0] == b'\\' || value[0].is_ascii_alphabetic() {
        return DA_T_IDENTIFIER;
    }
    if da_is_numeric(value) {
        return if value.iter().any(|&c| c == b'.' || c == b'e' || c == b'E') {
            4
        } else {
            2
        };
    }
    DA_T_NONE
}

fn da_is_numeric(v: &[u8]) -> bool {
    if v.is_empty() {
        return false;
    }
    let mut i = 0;
    if v[0] == b'+' || v[0] == b'-' {
        i += 1;
    }
    let (mut digits, mut dot, mut e) = (false, false, false);
    while i < v.len() {
        let c = v[i];
        if c.is_ascii_digit() {
            digits = true;
        } else if c == b'.' && !dot && !e {
            dot = true;
        } else if (c == b'e' || c == b'E') && !e && digits {
            e = true;
            if i + 1 < v.len() && (v[i + 1] == b'+' || v[i + 1] == b'-') {
                i += 1;
            }
        } else {
            return false;
        }
        i += 1;
    }
    digits
}

// mirrors DocLexer::scan: tokens keep offsets; whitespace and "*" runs are dropped
fn da_scan(input: &[u8]) -> Vec<DaToken> {
    let mut toks = Vec::new();
    let mut i = 0;
    while i < input.len() {
        if let Some(e) = da_match_identifier(input, i) {
            toks.push(DaToken { typ: DA_T_IDENTIFIER, content: input[i..e].to_vec(), pos: i });
            i = e;
            continue;
        }
        if let Some(e) = da_match_number(input, i) {
            let v = &input[i..e];
            toks.push(DaToken { typ: da_get_type(v), content: v.to_vec(), pos: i });
            i = e;
            continue;
        }
        if let Some(e) = da_match_string(input, i) {
            let unq = da_unquote(&input[i..e]);
            toks.push(DaToken { typ: DA_T_STRING, content: unq, pos: i });
            i = e;
            continue;
        }
        if da_is_space(input[i]) {
            while i < input.len() && da_is_space(input[i]) {
                i += 1;
            }
            continue;
        }
        if input[i] == b'*' {
            while i < input.len() && input[i] == b'*' {
                i += 1;
            }
            continue;
        }
        // single char; a lone `"` types as a string whose unquoted content is
        // empty (DocLexer::getType does substr($value, 1, strlen-2))
        let v = &input[i..i + 1];
        let typ = da_get_type(v);
        let content = if typ == DA_T_STRING { Vec::new() } else { v.to_vec() };
        toks.push(DaToken { typ, content, pos: i });
        i += 1;
    }
    toks
}

fn da_unquote(s: &[u8]) -> Vec<u8> {
    // strip outer quotes, "" -> "
    let inner = &s[1..s.len() - 1];
    let mut out = Vec::with_capacity(inner.len());
    let mut i = 0;
    while i < inner.len() {
        if inner[i] == b'"' && i + 1 < inner.len() && inner[i + 1] == b'"' {
            out.push(b'"');
            i += 2;
        } else {
            out.push(inner[i]);
            i += 1;
        }
    }
    out
}

fn da_requote(unq: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(unq.len() + 2);
    out.push(b'"');
    for &c in unq {
        if c == b'"' {
            out.push(b'"');
            out.push(b'"');
        } else {
            out.push(c);
        }
    }
    out.push(b'"');
    out
}

fn da_create_from_doc_comment(content: &[u8], ignored: &[&[u8]]) -> Vec<DaToken> {
    let mut toks: Vec<DaToken> = Vec::new();
    let mut ignored_text_position = 0usize;
    let mut current_position = 0usize;
    while let Some(rel) = content[current_position..].iter().position(|&c| c == b'@') {
        let next_at = current_position + rel;
        if next_at != 0 && !da_is_space(content[next_at - 1]) {
            current_position = next_at + 1;
            continue;
        }
        let scanned = da_scan(&content[next_at..]);
        let mut used: Vec<&DaToken> = Vec::new();
        let mut nb_to_use = 0usize;
        let mut nb_scopes: i32 = 0;
        let mut index = 0usize;
        let mut broke_clean = true;
        while index < scanned.len() {
            let tk = &scanned[index];
            if index == 0 && tk.typ != DA_T_AT {
                break;
            }
            if index == 1 {
                let is_ignored = ignored.iter().any(|t| *t == tk.content.as_slice());
                if tk.typ != DA_T_IDENTIFIER || is_ignored {
                    break;
                }
                nb_to_use = 2;
            }
            if index >= 2 && nb_scopes == 0 && tk.typ != DA_T_NONE && tk.typ != DA_T_OPEN_PAREN {
                break;
            }
            used.push(tk);
            if tk.typ == DA_T_OPEN_PAREN {
                nb_scopes += 1;
            } else if tk.typ == DA_T_CLOSE_PAREN {
                nb_scopes -= 1;
                if nb_scopes == 0 {
                    nb_to_use = used.len();
                    broke_clean = true;
                    break;
                }
            }
            index += 1;
        }
        let _ = broke_clean;
        if nb_scopes != 0 {
            break;
        }
        if nb_to_use != 0 {
            let ignored_text_length = next_at - ignored_text_position;
            if ignored_text_length != 0 {
                toks.push(DaToken {
                    typ: DA_T_NONE,
                    content: content[ignored_text_position..ignored_text_position + ignored_text_length].to_vec(),
                    pos: 0,
                });
            }
            let mut last_end = 0usize;
            let mut last_pos = 0usize;
            let mut last_len = 0usize;
            for st in used.iter().take(nb_to_use) {
                let (rewrapped, cpos, clen);
                if st.typ == DA_T_STRING {
                    let rq = da_requote(&st.content);
                    clen = rq.len();
                    cpos = st.pos;
                    rewrapped = rq;
                } else {
                    cpos = st.pos;
                    clen = st.content.len();
                    rewrapped = st.content.clone();
                }
                if cpos > last_end {
                    let missing = cpos - last_end;
                    toks.push(DaToken {
                        typ: DA_T_NONE,
                        content: content[next_at + last_end..next_at + last_end + missing].to_vec(),
                        pos: 0,
                    });
                }
                toks.push(DaToken { typ: st.typ, content: rewrapped, pos: 0 });
                last_end = cpos + clen;
                last_pos = cpos;
                last_len = clen;
            }
            current_position = next_at + last_pos + last_len;
            ignored_text_position = current_position;
        } else {
            current_position = next_at + 1;
        }
    }
    if ignored_text_position < content.len() {
        toks.push(DaToken { typ: DA_T_NONE, content: content[ignored_text_position..].to_vec(), pos: 0 });
    }
    toks
}

fn da_get_code(toks: &[DaToken]) -> Vec<u8> {
    let mut out = Vec::new();
    for t in toks {
        out.extend_from_slice(&t.content);
    }
    out
}

// mirrors /^(\r?\n\s*\*\s*)*\s*$/ on a T_NONE gap
fn da_annotation_end_none(content: &[u8]) -> bool {
    let mut i = 0;
    loop {
        let start = i;
        if i < content.len() && content[i] == b'\r' {
            i += 1;
        }
        if i < content.len() && content[i] == b'\n' {
            i += 1;
            while i < content.len() && da_is_space(content[i]) {
                i += 1;
            }
            if i < content.len() && content[i] == b'*' {
                i += 1;
                while i < content.len() && da_is_space(content[i]) {
                    i += 1;
                }
            } else {
                i = start;
                break;
            }
        } else {
            i = start;
            break;
        }
    }
    while i < content.len() && da_is_space(content[i]) {
        i += 1;
    }
    i == content.len()
}

fn da_get_annotation_end(toks: &[DaToken], index: usize) -> Option<usize> {
    let mut current: Option<usize> = None;
    if index + 2 < toks.len() {
        if toks[index + 2].typ == DA_T_OPEN_PAREN {
            current = Some(index + 2);
        } else if index + 3 < toks.len()
            && toks[index + 2].typ == DA_T_NONE
            && toks[index + 3].typ == DA_T_OPEN_PAREN
            && da_annotation_end_none(&toks[index + 2].content)
        {
            current = Some(index + 3);
        }
    }
    if let Some(mut c) = current {
        let mut level = 0i32;
        while c < toks.len() {
            match toks[c].typ {
                DA_T_OPEN_PAREN => level += 1,
                DA_T_CLOSE_PAREN => level -= 1,
                _ => {}
            }
            if level == 0 {
                return Some(c);
            }
            c += 1;
        }
        return None;
    }
    Some(index + 1)
}

const DA_IGNORED_TAGS: &[&[u8]] = &[
    b"abstract", b"access", b"code", b"deprec", b"encode", b"exception", b"final", b"ingroup",
    b"inheritdoc", b"inheritDoc", b"magic", b"name", b"toc", b"tutorial", b"private", b"static",
    b"staticvar", b"staticVar", b"throw", b"api", b"author", b"category", b"copyright", b"deprecated",
    b"example", b"filesource", b"global", b"ignore", b"internal", b"license", b"link", b"method",
    b"package", b"property", b"property-read", b"property-write", b"return", b"see", b"since",
    b"source", b"subpackage", b"throws", b"todo", b"TODO", b"usedBy", b"uses", b"var", b"version",
    b"param", b"after", b"afterClass", b"backupGlobals", b"backupStaticAttributes", b"before",
    b"beforeClass", b"codeCoverageIgnore", b"codeCoverageIgnoreStart", b"codeCoverageIgnoreEnd",
    b"covers", b"coversDefaultClass", b"coversNothing", b"dataProvider", b"depends",
    b"expectedException", b"expectedExceptionCode", b"expectedExceptionMessage",
    b"expectedExceptionMessageRegExp", b"group", b"large", b"medium", b"preserveGlobalState",
    b"requires", b"runTestsInSeparateProcesses", b"runInSeparateProcess", b"small", b"test",
    b"testdox", b"ticket", b"SuppressWarnings", b"noinspection", b"package_version", b"enduml",
    b"startuml", b"psalm", b"phpstan", b"template", b"fix", b"FIXME", b"fixme", b"override",
];

fn da_is_class_modifier(lo: &[u8]) -> bool {
    matches!(lo, b"abstract" | b"final" | b"readonly")
}
fn da_is_member_modifier(lo: &[u8]) -> bool {
    matches!(lo, b"public" | b"protected" | b"private" | b"final" | b"abstract" | b"readonly")
}

fn da_enclosing_class_is_class(s: &Stream, idx: usize) -> bool {
    let mut depth = 0i32;
    let mut j = idx as isize - 1;
    while j >= 0 {
        let k = j as usize;
        if s.kind(k) == Kind::Punct {
            match s.bytes(k) {
                b"}" => depth += 1,
                b"{" => {
                    if depth == 0 {
                        let (bk, kw) = classify_brace(s, k);
                        return bk == BraceKind::ClassLike
                            && matches!(kw, Some(w) if s.bytes(w).eq_ignore_ascii_case(b"class"));
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

fn da_eligible(s: &Stream, index: usize) -> bool {
    let mut i = sig_next(s, index);
    while let Some(x) = i {
        if s.kind(x) == Kind::Keyword && da_is_class_modifier(&s.bytes(x).to_ascii_lowercase()) {
            i = sig_next(s, x);
        } else {
            break;
        }
    }
    let x = match i {
        Some(x) => x,
        None => return false,
    };
    if s.kind(x) == Kind::Keyword && s.bytes(x).eq_ignore_ascii_case(b"class") {
        return true;
    }
    let mut i = Some(x);
    while let Some(x) = i {
        if s.kind(x) == Kind::Keyword && da_is_member_modifier(&s.bytes(x).to_ascii_lowercase()) {
            i = sig_next(s, x);
        } else {
            break;
        }
    }
    match i {
        Some(x) => da_enclosing_class_is_class(s, x),
        None => false,
    }
}

fn da_apply<F: FnMut(&mut Vec<DaToken>) -> bool>(s: &mut Stream, mut fixfn: F) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::DocComment {
            i += 1;
            continue;
        }
        if !da_eligible(s, i) {
            i += 1;
            continue;
        }
        let mut toks = da_create_from_doc_comment(s.bytes(i), DA_IGNORED_TAGS);
        if fixfn(&mut toks) {
            let rebuilt = da_get_code(&toks);
            if rebuilt != s.bytes(i) {
                s.set_owned(i, rebuilt);
                changed = true;
            }
        }
        i += 1;
    }
    changed
}

fn doctrine_annotation_array_assignment(s: &mut Stream) -> bool {
    da_apply(s, |toks| {
        let mut changed = false;
        let mut scopes: Vec<u8> = Vec::new(); // 0=annotation, 1=array
        for t in toks.iter_mut() {
            match t.typ {
                DA_T_OPEN_PAREN => scopes.push(0),
                DA_T_OPEN_CURLY => scopes.push(1),
                DA_T_CLOSE_PAREN | DA_T_CLOSE_CURLY => {
                    scopes.pop();
                }
                DA_T_EQUALS | DA_T_COLON => {
                    if scopes.last() == Some(&1) && t.content != b"=" {
                        t.content = b"=".to_vec();
                        changed = true;
                    }
                }
                _ => {}
            }
        }
        changed
    })
}

// mirrors /(\n( +\*)?) *$/ with PCRE `$`; returns (match_start, group1_end) if it matches
fn da_indent_apply(content: &[u8], indent_spaces: usize) -> Option<Vec<u8>> {
    // find the last '\n'
    let nl = content.iter().rposition(|&c| c == b'\n')?;
    // group1 = '\n' + optional ( +\*)
    let mut g1_end = nl + 1;
    // optional: one-or-more spaces then '*'
    let mut p = nl + 1;
    let mut spaces = 0;
    while p < content.len() && content[p] == b' ' {
        p += 1;
        spaces += 1;
    }
    if spaces >= 1 && p < content.len() && content[p] == b'*' {
        g1_end = p + 1;
    }
    // then ` *` (spaces) then optional (\r?\n) then end
    let mut q = g1_end;
    while q < content.len() && content[q] == b' ' {
        q += 1;
    }
    let mut r = q;
    if r < content.len() && content[r] == b'\r' {
        r += 1;
    }
    if r < content.len() && content[r] == b'\n' {
        r += 1;
    }
    if r != content.len() {
        return None;
    }
    // rebuild: content[..g1_end] + spaces + (trailing newline preserved from [q..])
    let mut out = content[..g1_end].to_vec();
    out.extend(std::iter::repeat(b' ').take(indent_spaces));
    out.extend_from_slice(&content[q..]);
    Some(out)
}

fn da_line_braces_count(toks: &[DaToken], index: usize) -> (i32, i32) {
    let mut opening = 0i32;
    let mut closing = 0i32;
    let mut i = index + 1;
    while i < toks.len() {
        if toks[i].typ == DA_T_NONE && toks[i].content.contains(&b'\n') {
            break;
        }
        match toks[i].typ {
            DA_T_OPEN_PAREN | DA_T_OPEN_CURLY => opening += 1,
            DA_T_CLOSE_PAREN | DA_T_CLOSE_CURLY => {
                if opening > 0 {
                    opening -= 1;
                } else {
                    closing += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    (opening, closing)
}

fn da_indentation_can_be_fixed(toks: &[DaToken], nl_index: usize, positions: &[(usize, usize)]) -> bool {
    for &(st, en) in positions {
        if nl_index >= st && nl_index <= en {
            return true;
        }
    }
    if nl_index + 1 < toks.len() {
        if toks[nl_index + 1].content.contains(&b'\n') {
            return false;
        }
        return toks[nl_index + 1].typ == DA_T_AT;
    }
    false
}

fn doctrine_annotation_indentation(s: &mut Stream) -> bool {
    da_apply(s, |toks| {
        let mut positions: Vec<(usize, usize)> = Vec::new();
        let mut index = 0usize;
        while index < toks.len() {
            if toks[index].typ != DA_T_AT {
                index += 1;
                continue;
            }
            match da_get_annotation_end(toks, index) {
                Some(end) => {
                    positions.push((index, end));
                    index = end + 1;
                }
                None => return false,
            }
        }
        let mut changed = false;
        let mut indent_level: i32 = 0;
        for idx in 0..toks.len() {
            if toks[idx].typ != DA_T_NONE || !toks[idx].content.contains(&b'\n') {
                continue;
            }
            if !da_indentation_can_be_fixed(toks, idx, &positions) {
                continue;
            }
            let (opening, closing) = da_line_braces_count(toks, idx);
            let delta = opening - closing;
            let mixed = delta == 0 && opening > 0;
            if indent_level > 0 && (delta < 0 || mixed) {
                indent_level -= 1;
            }
            let spaces = (4 * indent_level + 1) as usize;
            if let Some(fixed) = da_indent_apply(&toks[idx].content, spaces) {
                if fixed != toks[idx].content {
                    toks[idx].content = fixed;
                    changed = true;
                }
            }
            if delta > 0 || mixed {
                indent_level += 1;
            }
        }
        changed
    })
}

// ===== Mirrored fixers (set A) =====

fn inline_ws_between(s: &Stream, a: usize, b: usize) -> bool {
    let mut j = a + 1;
    while j < b {
        if s.kind(j) != Kind::Whitespace || has_newline(s.bytes(j)) {
            return false;
        }
        j += 1;
    }
    true
}

fn sa_is_modifier_keyword(v: &[u8]) -> bool {
    matches!(
        v.to_ascii_lowercase().as_slice(),
        b"public" | b"private" | b"protected" | b"static" | b"final" | b"abstract" | b"readonly" | b"var"
    )
}

fn sa_is_class_like_keyword(lw: &[u8]) -> bool {
    matches!(lw, b"class" | b"interface" | b"trait" | b"enum")
}

fn sa_find_body_brace(s: &Stream, name_idx: usize) -> Option<usize> {
    let mut k = name_idx + 1;
    while k < s.len() {
        if s.kind(k) == Kind::Punct {
            match s.bytes(k) {
                b"{" => return Some(k),
                b";" => return None,
                _ => {}
            }
        }
        k += 1;
    }
    None
}

fn sa_class_extends(s: &Stream, name_idx: usize, open: usize) -> bool {
    let mut k = name_idx + 1;
    while k < open {
        if s.kind(k) == Kind::Keyword && s.bytes(k).eq_ignore_ascii_case(b"extends") {
            return true;
        }
        k += 1;
    }
    false
}

fn no_mixed_echo_print(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword && s.bytes(i).eq_ignore_ascii_case(b"print") {
            if let Some(p) = sig_prev(s, i) {
                let convertible = s.kind(p) == Kind::OpenTag
                    || (s.kind(p) == Kind::Punct && matches!(s.bytes(p), b";" | b"{" | b"}" | b")"))
                    || (s.kind(p) == Kind::Keyword && s.bytes(p).eq_ignore_ascii_case(b"else"));
                if convertible {
                    s.set_owned(i, b"echo".to_vec());
                    changed = true;
                }
            }
        }
        i += 1;
    }
    changed
}

const FUNCTION_ALIASES: &[(&[u8], &[u8])] = &[
    (b"diskfreespace", b"disk_free_space"),
    (b"dns_check_record", b"checkdnsrr"),
    (b"dns_get_mx", b"getmxrr"),
    (b"session_commit", b"session_write_close"),
    (b"stream_register_wrapper", b"stream_wrapper_register"),
    (b"set_file_buffer", b"stream_set_write_buffer"),
    (b"socket_set_blocking", b"stream_set_blocking"),
    (b"socket_get_status", b"stream_get_meta_data"),
    (b"socket_set_timeout", b"stream_set_timeout"),
    (b"socket_getopt", b"socket_get_option"),
    (b"socket_setopt", b"socket_set_option"),
    (b"chop", b"rtrim"),
    (b"close", b"closedir"),
    (b"doubleval", b"floatval"),
    (b"fputs", b"fwrite"),
    (b"get_required_files", b"get_included_files"),
    (b"ini_alter", b"ini_set"),
    (b"is_double", b"is_float"),
    (b"is_integer", b"is_int"),
    (b"is_long", b"is_int"),
    (b"is_real", b"is_float"),
    (b"is_writeable", b"is_writable"),
    (b"join", b"implode"),
    (b"key_exists", b"array_key_exists"),
    (b"magic_quotes_runtime", b"set_magic_quotes_runtime"),
    (b"pos", b"current"),
    (b"show_source", b"highlight_file"),
    (b"sizeof", b"count"),
    (b"strchr", b"strstr"),
    (b"user_error", b"trigger_error"),
    (b"imap_create", b"imap_createmailbox"),
    (b"imap_fetchtext", b"imap_body"),
    (b"imap_header", b"imap_headerinfo"),
    (b"imap_listmailbox", b"imap_list"),
    (b"imap_listsubscribed", b"imap_lsub"),
    (b"imap_rename", b"imap_renamemailbox"),
    (b"imap_scan", b"imap_listscan"),
    (b"imap_scanmailbox", b"imap_listscan"),
    (b"pg_exec", b"pg_query"),
];

fn function_alias(name: &[u8]) -> Option<&'static [u8]> {
    let lo = name.to_ascii_lowercase();
    for (a, c) in FUNCTION_ALIASES {
        if *a == lo.as_slice() {
            return Some(c);
        }
    }
    None
}

fn is_global_function_call(s: &Stream, i: usize) -> bool {
    let p = match sig_prev(s, i) {
        Some(p) => p,
        None => return true,
    };
    if s.kind(p) == Kind::Punct {
        match s.bytes(p) {
            b"->" | b"?->" | b"::" => return false,
            b"\\" => {
                if let Some(before) = sig_prev(s, p) {
                    if s.kind(before) == Kind::Ident {
                        return false;
                    }
                }
                return true;
            }
            _ => {}
        }
    }
    if s.kind(p) == Kind::Keyword {
        match s.bytes(p).to_ascii_lowercase().as_slice() {
            b"function" | b"const" | b"new" | b"goto" => return false,
            _ => {}
        }
    }
    true
}

fn no_alias_functions(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Ident {
            if let Some(canon) = function_alias(s.bytes(i)) {
                if next_significant_value(s, i) == b"(" && is_global_function_call(s, i) {
                    s.set_owned(i, canon.to_vec());
                    changed = true;
                }
            }
        }
        i += 1;
    }
    changed
}

fn no_alias_language_construct_call(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Ident && s.bytes(i).eq_ignore_ascii_case(b"die") && !member_prev(s, i) {
            let mut skip = false;
            if let Some(p) = sig_prev(s, i) {
                if s.kind(p) == Kind::Punct && s.bytes(p) == b"\\" {
                    skip = true;
                }
                if s.kind(p) == Kind::Keyword {
                    match s.bytes(p).to_ascii_lowercase().as_slice() {
                        b"function" | b"const" | b"class" | b"namespace" | b"use" => skip = true,
                        _ => {}
                    }
                }
            }
            if !skip {
                s.set_owned(i, b"exit".to_vec());
                changed = true;
            }
        }
        i += 1;
    }
    changed
}

fn not_operator_with_successor_space(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Punct && s.bytes(i) == b"!" {
            if i + 1 >= s.len() {
                i += 1;
                continue;
            }
            if s.kind(i + 1) == Kind::Whitespace {
                if s.bytes(i + 1) != b" " {
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

fn increment_style(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Punct && (s.bytes(i) == b"++" || s.bytes(i) == b"--") {
            let op = s.bytes(i).to_vec();
            let lv = match sig_prev(s, i) {
                Some(l) if s.kind(l) == Kind::Variable => l,
                _ => {
                    i += 1;
                    continue;
                }
            };
            if !inline_ws_between(s, lv, i) {
                i += 1;
                continue;
            }
            let p = match sig_prev(s, lv) {
                Some(p) => p,
                None => {
                    i += 1;
                    continue;
                }
            };
            if lvalue_prefix(s.bytes(p)) {
                i += 1;
                continue;
            }
            if s.bytes(p) != b";" && s.bytes(p) != b"{" && s.bytes(p) != b"}" && s.kind(p) != Kind::OpenTag {
                i += 1;
                continue;
            }
            match sig_next(s, i) {
                Some(e) if s.bytes(e) == b";" => {}
                _ => {
                    i += 1;
                    continue;
                }
            }
            let mut k = i;
            while k >= lv + 1 {
                s.remove_at(k);
                k -= 1;
            }
            s.insert_owned(lv, Kind::Punct, op);
            changed = true;
            i = lv + 1;
        } else {
            i += 1;
        }
    }
    changed
}

fn single_line_empty_body(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Punct && s.bytes(i) == b"{" {
            let (kind, _) = classify_brace(s, i);
            if matches!(kind, BraceKind::ClassLike | BraceKind::FunctionDecl) {
                if let Some(close_idx) = match_forward(s, i) {
                    let mut empty = true;
                    let mut k = i + 1;
                    while k < close_idx {
                        if s.kind(k) != Kind::Whitespace {
                            empty = false;
                            break;
                        }
                        k += 1;
                    }
                    if empty && close_idx != i + 1 {
                        let mut k = close_idx - 1;
                        while k > i {
                            s.remove_at(k);
                            k -= 1;
                        }
                        if i > 0 && s.kind(i - 1) == Kind::Whitespace && has_newline(s.bytes(i - 1)) {
                            s.set_owned(i - 1, b" ".to_vec());
                        }
                        changed = true;
                    }
                }
            }
        }
        i += 1;
    }
    changed
}

fn is_const_name_position(s: &Stream, k: usize) -> bool {
    let mut j = prev_significant_index(s, k);
    while let Some(ji) = j {
        if s.kind(ji) == Kind::Keyword && s.bytes(ji).eq_ignore_ascii_case(b"const") {
            return true;
        }
        let is_type = s.kind(ji) == Kind::Ident
            || (s.kind(ji) == Kind::Keyword && !s.bytes(ji).eq_ignore_ascii_case(b"const"))
            || (s.kind(ji) == Kind::Punct && (s.bytes(ji) == b"?" || s.bytes(ji) == b"\\"));
        if is_type {
            if s.kind(ji) == Kind::Keyword && sa_is_modifier_keyword(s.bytes(ji)) {
                return false;
            }
            j = prev_significant_index(s, ji);
            continue;
        }
        return false;
    }
    false
}

fn protected_to_private(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword && s.bytes(i).eq_ignore_ascii_case(b"class") {
            let is_final = matches!(sig_prev(s, i), Some(p) if s.kind(p) == Kind::Keyword && s.bytes(p).eq_ignore_ascii_case(b"final"));
            if is_final {
                if let Some(name_idx) = next_significant_index(s, i) {
                    if s.kind(name_idx) == Kind::Ident {
                        if let Some(open) = sa_find_body_brace(s, name_idx) {
                            if !sa_class_extends(s, name_idx, open) {
                                if let Some(close_idx) = match_forward(s, open) {
                                    let mut depth = 0i32;
                                    let mut k = open;
                                    while k < close_idx {
                                        if s.kind(k) == Kind::Punct {
                                            match s.bytes(k) {
                                                b"{" => depth += 1,
                                                b"}" => depth -= 1,
                                                _ => {}
                                            }
                                        } else if depth == 1
                                            && s.kind(k) == Kind::Keyword
                                            && s.bytes(k).eq_ignore_ascii_case(b"protected")
                                            && !is_const_name_position(s, k)
                                        {
                                            s.set_owned(k, b"private".to_vec());
                                            changed = true;
                                        }
                                        k += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    changed
}

fn self_accessor_in_return_type(s: &Stream, k: usize) -> bool {
    let mut j = k;
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
                && (s.bytes(p) == b"?" || s.bytes(p) == b"|" || s.bytes(p) == b"&" || s.bytes(p) == b"\\"))
        {
            j = p;
            continue;
        }
        return false;
    }
}

fn self_accessor_ref(s: &Stream, k: usize) -> bool {
    let prev = sig_prev(s, k);
    let next_idx = next_significant_index(s, k);
    if let Some(p) = prev {
        if s.kind(p) == Kind::Punct {
            match s.bytes(p) {
                b"->" | b"?->" | b"::" | b"\\" => return false,
                _ => {}
            }
        }
        if s.kind(p) == Kind::Keyword {
            match s.bytes(p).to_ascii_lowercase().as_slice() {
                b"function" | b"const" | b"extends" | b"implements" | b"use" | b"namespace" | b"as" => {
                    return false
                }
                b"new" | b"instanceof" => return true,
                _ => {}
            }
        }
    }
    if let Some(ni) = next_idx {
        if s.kind(ni) == Kind::Punct && s.bytes(ni) == b"\\" {
            return false;
        }
        if s.kind(ni) == Kind::Punct && s.bytes(ni) == b"::" {
            return true;
        }
    }
    if enclosing_func_param_open(s, k).is_some() {
        return true;
    }
    self_accessor_in_return_type(s, k)
}

fn self_accessor(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword && sa_is_class_like_keyword(&s.bytes(i).to_ascii_lowercase()) {
            let mut skip = false;
            if let Some(p) = sig_prev(s, i) {
                if s.kind(p) == Kind::Keyword && s.bytes(p).eq_ignore_ascii_case(b"new") {
                    skip = true;
                }
                if s.kind(p) == Kind::Punct && matches!(s.bytes(p), b"->" | b"?->" | b"::") {
                    skip = true;
                }
            }
            if !skip {
                if let Some(name_idx) = next_significant_index(s, i) {
                    if s.kind(name_idx) == Kind::Ident {
                        let class_name = s.bytes(name_idx).to_vec();
                        if let Some(open) = sa_find_body_brace(s, name_idx) {
                            if let Some(close_idx) = match_forward(s, open) {
                                let mut k = open + 1;
                                while k < close_idx {
                                    if s.kind(k) == Kind::Ident
                                        && s.bytes(k) == class_name.as_slice()
                                        && self_accessor_ref(s, k)
                                    {
                                        s.set_owned(k, b"self".to_vec());
                                        changed = true;
                                    }
                                    k += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    changed
}

struct SelfStaticFrame {
    class_like: bool,
    final_: bool,
}

fn is_final_class_like(s: &Stream, kw: usize) -> bool {
    if s.bytes(kw).eq_ignore_ascii_case(b"enum") {
        return true;
    }
    let mut j = kw as isize - 1;
    while j >= 0 {
        let t = j as usize;
        match s.kind(t) {
            Kind::Whitespace | Kind::Comment | Kind::DocComment => {
                j -= 1;
                continue;
            }
            Kind::Keyword => {
                return match s.bytes(t).to_ascii_lowercase().as_slice() {
                    b"final" => true,
                    b"new" => false,
                    b"abstract" | b"readonly" => {
                        j -= 1;
                        continue;
                    }
                    _ => false,
                };
            }
            Kind::Punct => {
                return false;
            }
            _ => return false,
        }
    }
    false
}

fn ssa_is_static_accessor(s: &Stream, i: usize) -> bool {
    if next_significant_value(s, i) == b"::" {
        return true;
    }
    matches!(sig_prev(s, i), Some(p) if s.kind(p) == Kind::Keyword && s.bytes(p).eq_ignore_ascii_case(b"new"))
}

fn self_static_accessor(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut stack: Vec<SelfStaticFrame> = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Punct {
            match s.bytes(i) {
                b"{" => {
                    let (kind, kw) = classify_brace(s, i);
                    if kind == BraceKind::ClassLike {
                        let f = matches!(kw, Some(k) if is_final_class_like(s, k));
                        stack.push(SelfStaticFrame { class_like: true, final_: f });
                    } else {
                        stack.push(SelfStaticFrame { class_like: false, final_: false });
                    }
                }
                b"}" => {
                    stack.pop();
                }
                _ => {}
            }
            i += 1;
            continue;
        }
        if s.kind(i) == Kind::Keyword && s.bytes(i).eq_ignore_ascii_case(b"static") {
            let enclosing_final = stack
                .iter()
                .rev()
                .find(|f| f.class_like)
                .map(|f| f.final_)
                .unwrap_or(false);
            if enclosing_final && ssa_is_static_accessor(s, i) {
                s.set_owned(i, b"self".to_vec());
                changed = true;
            }
        }
        i += 1;
    }
    changed
}

fn static_class_tokens() -> Vec<(Kind, Vec<u8>)> {
    vec![
        (Kind::Keyword, b"static".to_vec()),
        (Kind::Punct, b"::".to_vec()),
        (Kind::Ident, b"class".to_vec()),
    ]
}

fn function_to_constant(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Ident {
            i += 1;
            continue;
        }
        let name = s.bytes(i).to_ascii_lowercase();
        let no_arg: Option<&[u8]> = match name.as_slice() {
            b"pi" => Some(b"M_PI"),
            b"phpversion" => Some(b"PHP_VERSION"),
            b"php_sapi_name" => Some(b"PHP_SAPI"),
            _ => None,
        };
        if no_arg.is_none() && name != b"get_called_class" && name != b"get_class" {
            i += 1;
            continue;
        }
        let mut start = i;
        if let Some(p) = sig_prev(s, i) {
            if s.kind(p) == Kind::Punct && matches!(s.bytes(p), b"->" | b"?->" | b"::") {
                i += 1;
                continue;
            }
            if s.kind(p) == Kind::Keyword && s.bytes(p).eq_ignore_ascii_case(b"function") {
                i += 1;
                continue;
            }
            if s.kind(p) == Kind::Punct && s.bytes(p) == b"\\" {
                if let Some(before) = sig_prev(s, p) {
                    if s.kind(before) == Kind::Ident || s.bytes(before) == b"\\" {
                        i += 1;
                        continue;
                    }
                }
                start = p;
            }
        }
        let open = match next_significant_index(s, i) {
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
        let repl: Vec<(Kind, Vec<u8>)>;
        if let Some(c) = no_arg {
            if next_significant_index(s, open) != Some(close_idx) {
                i += 1;
                continue;
            }
            repl = vec![(Kind::Ident, c.to_vec())];
        } else if name == b"get_called_class" {
            if next_significant_index(s, open) != Some(close_idx) {
                i += 1;
                continue;
            }
            repl = static_class_tokens();
        } else {
            let arg = match next_significant_index(s, open) {
                Some(a) if s.kind(a) == Kind::Variable && s.bytes(a).eq_ignore_ascii_case(b"$this") => a,
                _ => {
                    i += 1;
                    continue;
                }
            };
            if next_significant_index(s, arg) != Some(close_idx) {
                i += 1;
                continue;
            }
            repl = static_class_tokens();
        }
        let mut k = close_idx;
        loop {
            s.remove_at(k);
            if k == start {
                break;
            }
            k -= 1;
        }
        let mut ins = start;
        for (kind, v) in repl {
            s.insert_owned(ins, kind, v);
            ins += 1;
        }
        changed = true;
        i = start;
    }
    changed
}

// ---- set B mirrors (byte-identical to the go fixers) ----

fn closes_do_block(s: &Stream, idx: usize) -> bool {
    match match_backward(s, idx) {
        Some(open) => {
            let (k, kw) = classify_brace(s, open);
            matches!(k, BraceKind::Control)
                && matches!(kw, Some(kwi) if s.bytes(kwi).eq_ignore_ascii_case(b"do"))
        }
        None => false,
    }
}

fn control_structure_continuation_position(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword {
            let lw = s.bytes(i).to_ascii_lowercase();
            if matches!(lw.as_slice(), b"else" | b"elseif" | b"catch" | b"finally" | b"while") {
                if let Some(pi) = prev_significant_index(s, i) {
                    if i == pi + 2
                        && is_punct_val(s, pi, b"}")
                        && s.kind(pi + 1) == Kind::Whitespace
                        && has_newline(s.bytes(pi + 1))
                    {
                        let ok = if lw == b"while" { closes_do_block(s, pi) } else { true };
                        if ok {
                            s.set_owned(pi + 1, b" ".to_vec());
                            changed = true;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    changed
}

fn no_unneeded_control_parentheses(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword {
            let term: Option<u8> = match s.bytes(i).to_ascii_lowercase().as_slice() {
                b"return" | b"echo" | b"print" | b"yield" | b"break" | b"continue" | b"clone" => {
                    Some(b';')
                }
                b"case" => Some(b':'),
                _ => None,
            };
            if let Some(term) = term {
                if !member_prev(s, i) {
                    loop {
                        let j = match next_significant_index(s, i) {
                            Some(j) if is_punct_val(s, j, b"(") => j,
                            _ => break,
                        };
                        let k = match match_forward(s, j) {
                            Some(k) => k,
                            None => break,
                        };
                        match next_significant_index(s, k) {
                            Some(nj) if is_punct_val(s, nj, &[term]) => {}
                            _ => break,
                        }
                        match next_significant_index(s, j) {
                            Some(x) if x < k => {}
                            _ => break,
                        }
                        s.remove_at(k);
                        if j == i + 1 {
                            s.remove_at(j);
                            s.insert_owned(j, Kind::Whitespace, b" ".to_vec());
                        } else {
                            s.remove_at(j);
                        }
                        changed = true;
                    }
                }
            }
        }
        i += 1;
    }
    changed
}

fn nullable_type_declaration(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if !(s.kind(i) == Kind::Punct && s.bytes(i) == b"|" && is_type_union_operator(s, i)) {
            i += 1;
            continue;
        }
        let start_i: isize = match type_run_boundary_prev(s, i) {
            Some(x) => x as isize,
            None => -1,
        };
        let end = match type_run_boundary_next(s, i) {
            Some(e) => e,
            None => {
                i += 1;
                continue;
            }
        };
        let mut parts: Vec<usize> = Vec::new();
        let mut j = (start_i + 1) as usize;
        while j < end {
            match s.kind(j) {
                Kind::Whitespace | Kind::Comment | Kind::DocComment => {}
                _ => parts.push(j),
            }
            j += 1;
        }
        if parts.is_empty() {
            i += 1;
            continue;
        }
        let mut pipes = 0;
        let mut bad = false;
        for &p in &parts {
            if s.kind(p) == Kind::Punct {
                match s.bytes(p) {
                    b"|" => pipes += 1,
                    b"&" | b"?" => bad = true,
                    _ => {}
                }
            }
        }
        if bad || pipes != 1 {
            i += 1;
            continue;
        }
        let mut left: Vec<usize> = Vec::new();
        let mut right: Vec<usize> = Vec::new();
        let mut seen = false;
        for &p in &parts {
            if !seen && s.kind(p) == Kind::Punct && s.bytes(p) == b"|" {
                seen = true;
                continue;
            }
            if seen {
                right.push(p);
            } else {
                left.push(p);
            }
        }
        let right_null = right.len() == 1 && s.bytes(right[0]).eq_ignore_ascii_case(b"null");
        let left_null = left.len() == 1 && s.bytes(left[0]).eq_ignore_ascii_case(b"null");
        let typ: Vec<usize> = if right_null {
            left
        } else if left_null {
            right
        } else {
            i += 1;
            continue;
        };
        if typ.is_empty() {
            i += 1;
            continue;
        }
        let first = parts[0];
        let last = parts[parts.len() - 1];
        let mut repl: Vec<(Kind, Vec<u8>)> = vec![(Kind::Punct, b"?".to_vec())];
        for &p in &typ {
            repl.push((s.kind(p), s.bytes(p).to_vec()));
        }
        for idx in (first..=last).rev() {
            s.remove_at(idx);
        }
        for (off, (k, v)) in repl.into_iter().enumerate() {
            s.insert_owned(first + off, k, v);
        }
        changed = true;
        i = first + 1;
    }
    changed
}

fn simplified_null_return(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword && s.bytes(i).eq_ignore_ascii_case(b"return") && !member_prev(s, i)
        {
            if let Some(j) = next_significant_index(s, i) {
                if is_null_literal(s, j) {
                    if let Some(k) = next_significant_index(s, j) {
                        if is_punct_val(s, k, b";") && !enclosing_return_type_nullable(s, i) {
                            let mut x = k - 1;
                            while x > i {
                                s.remove_at(x);
                                x -= 1;
                            }
                            changed = true;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    changed
}

fn enclosing_return_type_nullable(s: &Stream, at: usize) -> bool {
    let mut depth = 0i32;
    let mut j = at as isize - 1;
    while j >= 0 {
        let k = j as usize;
        if s.kind(k) == Kind::Punct {
            match s.bytes(k) {
                b"}" => depth += 1,
                b"{" => {
                    if depth > 0 {
                        depth -= 1;
                    } else {
                        let (kind, kw) = classify_brace(s, k);
                        match kind {
                            BraceKind::FunctionDecl | BraceKind::Closure => {
                                return func_return_type_nullable(s, kw, k);
                            }
                            BraceKind::ClassLike => return false,
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        j -= 1;
    }
    false
}

fn func_return_type_nullable(s: &Stream, kw: Option<usize>, brace: usize) -> bool {
    let kw = match kw {
        Some(k) => k,
        None => return false,
    };
    let mut params_open: Option<usize> = None;
    let mut x = kw + 1;
    while x < brace {
        if is_punct_val(s, x, b"(") {
            params_open = Some(x);
            break;
        }
        x += 1;
    }
    let params_open = match params_open {
        Some(o) => o,
        None => return false,
    };
    let params_close = match match_forward(s, params_open) {
        Some(c) if c < brace => c,
        _ => return false,
    };
    let mut c = match next_significant_index(s, params_close) {
        Some(c) => c,
        None => return false,
    };
    if s.kind(c) == Kind::Keyword && s.bytes(c).eq_ignore_ascii_case(b"use") {
        let uo = match next_significant_index(s, c) {
            Some(o) if is_punct_val(s, o, b"(") => o,
            _ => return false,
        };
        let uc = match match_forward(s, uo) {
            Some(u) if u < brace => u,
            _ => return false,
        };
        c = match next_significant_index(s, uc) {
            Some(c) => c,
            None => return false,
        };
    }
    if !is_punct_val(s, c, b":") {
        return false;
    }
    let mut typ: Vec<u8> = Vec::new();
    let mut y = c + 1;
    while y < brace {
        match s.kind(y) {
            Kind::Whitespace | Kind::Comment | Kind::DocComment => {}
            _ => typ.extend_from_slice(s.bytes(y)),
        }
        y += 1;
    }
    let typ = typ.to_ascii_lowercase();
    if typ.is_empty() {
        return false;
    }
    if typ.starts_with(b"?") {
        return true;
    }
    for part in typ.split(|&c| c == b'|' || c == b'&') {
        let trimmed: Vec<u8> = part.iter().copied().filter(|&c| c != b'(' && c != b')').collect();
        if trimmed == b"null" || trimmed == b"mixed" {
            return true;
        }
    }
    false
}

fn no_useless_return(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword && s.bytes(i).eq_ignore_ascii_case(b"return") {
            let member = match prev_significant_index(s, i) {
                Some(p) => s.kind(p) == Kind::Punct && matches!(s.bytes(p), b"->" | b"?->" | b"::"),
                None => false,
            };
            if !member {
                if let Some(semi) = next_significant_index(s, i) {
                    if is_punct_val(s, semi, b";") {
                        if let Some(after) = next_significant_index(s, semi) {
                            if is_punct_val(s, after, b"}") {
                                if let Some(open) = match_backward(s, after) {
                                    let (kind, _) = classify_brace(s, open);
                                    if matches!(kind, BraceKind::FunctionDecl | BraceKind::Closure) {
                                        let mut kk = semi as isize;
                                        while kk >= i as isize {
                                            s.remove_at(kk as usize);
                                            kk -= 1;
                                        }
                                        changed = true;
                                        if i > 0 {
                                            i -= 1;
                                        }
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    changed
}

fn single_blank_line_before_namespace(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword
            && s.bytes(i).eq_ignore_ascii_case(b"namespace")
            && !member_prev(s, i)
            && i > 0
        {
            let prev = i - 1;
            if s.kind(prev) == Kind::Whitespace
                && has_newline(s.bytes(prev))
                && s.bytes(prev) != b"\n\n"
            {
                s.set_owned(prev, b"\n\n".to_vec());
                changed = true;
            }
        }
        i += 1;
    }
    changed
}

fn linebreak_after_opening_tag(s: &mut Stream) -> bool {
    if s.len() < 3 {
        return false;
    }
    if s.kind(0) != Kind::OpenTag || s.bytes(0) != b"<?php" {
        return false;
    }
    if s.kind(1) != Kind::Whitespace {
        return false;
    }
    if has_newline(s.bytes(1)) {
        return false;
    }
    if s.kind(2) == Kind::CloseTag {
        return false;
    }
    s.set_owned(1, b"\n".to_vec());
    true
}

fn declare_is_strict_types(s: &Stream, open: usize, close: usize) -> bool {
    let mut k = open + 1;
    while k < close {
        if s.kind(k) == Kind::Ident && s.bytes(k).eq_ignore_ascii_case(b"strict_types") {
            return true;
        }
        k += 1;
    }
    false
}

fn blank_line_after_strict_types(s: &mut Stream) -> bool {
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword && s.bytes(i).eq_ignore_ascii_case(b"declare") {
            let open = match next_significant_index(s, i) {
                Some(o) if is_punct_val(s, o, b"(") => o,
                _ => {
                    i += 1;
                    continue;
                }
            };
            let close = match match_forward(s, open) {
                Some(c) => c,
                None => {
                    i += 1;
                    continue;
                }
            };
            if !declare_is_strict_types(s, open, close) {
                i += 1;
                continue;
            }
            let semi = match next_significant_index(s, close) {
                Some(sm) if is_punct_val(s, sm, b";") => sm,
                _ => {
                    i += 1;
                    continue;
                }
            };
            if semi + 1 >= s.len() {
                return false;
            }
            if s.kind(semi + 1) != Kind::Whitespace || !s.bytes(semi + 1).contains(&b'\n') {
                return false;
            }
            if semi + 2 >= s.len() || s.kind(semi + 2) == Kind::CloseTag {
                return false;
            }
            let ws = s.bytes(semi + 1).to_vec();
            let nl = ws.iter().rposition(|&c| c == b'\n').unwrap();
            let indent = &ws[nl + 1..];
            let mut want = b"\n\n".to_vec();
            want.extend_from_slice(indent);
            if ws != want {
                s.set_owned(semi + 1, want);
                return true;
            }
            return false;
        }
        i += 1;
    }
    false
}

fn blbs_is_comment(s: &Stream, i: usize) -> bool {
    matches!(s.kind(i), Kind::Comment | Kind::DocComment)
}

fn blbs_prev_non_whitespace(s: &Stream, i: usize) -> isize {
    let mut j = i as isize - 1;
    while j >= 0 {
        if s.kind(j as usize) != Kind::Whitespace {
            return j;
        }
        j -= 1;
    }
    -1
}

fn blbs_count_newlines(v: &[u8]) -> usize {
    v.iter().filter(|&&c| c == b'\n').count()
}

fn blbs_insert_index(s: &Stream, index: usize) -> usize {
    let mut index = index;
    while index > 0 {
        if s.kind(index - 1) == Kind::Whitespace && blbs_count_newlines(s.bytes(index - 1)) > 1 {
            break;
        }
        let prev_index = blbs_prev_non_whitespace(s, index);
        if prev_index < 0 || !blbs_is_comment(s, prev_index as usize) {
            break;
        }
        let pi = prev_index as usize;
        if pi < 1 || s.kind(pi - 1) != Kind::Whitespace {
            break;
        }
        if blbs_count_newlines(s.bytes(pi - 1)) != 1 {
            break;
        }
        index = pi;
    }
    index
}

fn blbs_should_add(s: &Stream, prev_nw: usize) -> bool {
    if blbs_is_comment(s, prev_nw) {
        let mut j = prev_nw as isize - 1;
        while j >= 0 {
            let k = j as usize;
            if s.bytes(k).contains(&b'\n') {
                return false;
            }
            if s.kind(k) == Kind::Whitespace || blbs_is_comment(s, k) {
                j -= 1;
                continue;
            }
            return s.kind(k) == Kind::Punct && matches!(s.bytes(k), b";" | b"}");
        }
        return false;
    }
    s.kind(prev_nw) == Kind::Punct && matches!(s.bytes(prev_nw), b";" | b"}")
}

fn blbs_insert(s: &mut Stream, index: usize) -> bool {
    if index >= 1 && s.kind(index - 1) == Kind::Whitespace {
        let v = s.bytes(index - 1).to_vec();
        match blbs_count_newlines(&v) {
            0 => {
                let mut nv: Vec<u8> = v.clone();
                while matches!(nv.last(), Some(b' ') | Some(b'\t')) {
                    nv.pop();
                }
                nv.extend_from_slice(b"\n\n");
                s.set_owned(index - 1, nv);
                true
            }
            1 => {
                let mut nv = b"\n".to_vec();
                nv.extend_from_slice(&v);
                s.set_owned(index - 1, nv);
                true
            }
            _ => false,
        }
    } else {
        s.insert_owned(index, Kind::Whitespace, b"\n\n".to_vec());
        true
    }
}

fn blank_line_before_statement(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = s.len() as isize - 1;
    while i > 0 {
        let idx = i as usize;
        if s.kind(idx) == Kind::Keyword {
            let lw = s.bytes(idx).to_ascii_lowercase();
            if matches!(
                lw.as_slice(),
                b"break" | b"continue" | b"declare" | b"return" | b"throw" | b"try"
            ) {
                let insert_idx = blbs_insert_index(s, idx);
                let prev_nw = blbs_prev_non_whitespace(s, insert_idx);
                if prev_nw >= 0 && blbs_should_add(s, prev_nw as usize) {
                    if blbs_insert(s, insert_idx) {
                        changed = true;
                    }
                }
                i = prev_nw;
                if i < 1 {
                    break;
                }
                continue;
            }
        }
        i -= 1;
    }
    changed
}

// ---- mirrored fixers (set C): byte-identical to the ecs-go implementations ----

fn trim_right_ws_st(b: &[u8]) -> &[u8] {
    let mut e = b.len();
    while e > 0 && (b[e - 1] == b' ' || b[e - 1] == b'\t') {
        e -= 1;
    }
    &b[..e]
}

fn trim_right_space_st(b: &[u8]) -> &[u8] {
    let mut e = b.len();
    while e > 0 && b[e - 1] == b' ' {
        e -= 1;
    }
    &b[..e]
}

const SUMMARY_PUNCT: &[&[u8]] = &[
    b".",
    b":",
    "\u{3002}".as_bytes(),
    b"!",
    b"?",
    "\u{a1}".as_bytes(),
    "\u{bf}".as_bytes(),
    "\u{ff01}".as_bytes(),
    "\u{ff1f}".as_bytes(),
];

fn summary_correctly_formatted(content: &[u8]) -> bool {
    let lc = content.to_ascii_lowercase();
    if bytes_contains(&lc, b"{@inheritdoc}") {
        return true;
    }
    SUMMARY_PUNCT.iter().any(|p| content.ends_with(p))
}

fn summary_range(d: &Doc) -> (isize, isize) {
    let mut first: isize = -1;
    let mut last: isize = -1;
    for (i, l) in d.inner.iter().enumerate() {
        let c = trim_ascii(&l.content);
        if c.is_empty() {
            if first >= 0 {
                break;
            }
            continue;
        }
        if c.first() == Some(&b'@') {
            break;
        }
        if first < 0 {
            first = i as isize;
        }
        last = i as isize;
    }
    (first, last)
}

fn phpdoc_summary(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| {
        let (first, last) = summary_range(d);
        if first < 0 {
            return false;
        }
        let li = last as usize;
        let content = trim_right_ws_st(&d.inner[li].content).to_vec();
        if summary_correctly_formatted(&content) {
            return false;
        }
        if first != last {
            let fl = trim_right_ws_st(&d.inner[first as usize].content);
            if fl.last() == Some(&b':') {
                return false;
            }
        }
        let mut nc = content;
        nc.push(b'.');
        d.inner[li].content = nc;
        true
    })
}

fn phpdoc_tag_type(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| {
        let mut changed = false;
        for l in d.inner.iter_mut() {
            let lead_len = l.content.len() - trim_left_space(&l.content).len();
            let inner = trim_right_space_st(trim_left_space(&l.content));
            // ^\{@([a-zA-Z]+)\}$
            if inner.len() < 4 || inner[0] != b'{' || inner[1] != b'@' || *inner.last().unwrap() != b'}' {
                continue;
            }
            let name = &inner[2..inner.len() - 1];
            if name.is_empty() || !name.iter().all(|c| c.is_ascii_alphabetic()) {
                continue;
            }
            if !name.eq_ignore_ascii_case(b"inheritDoc") {
                continue;
            }
            let mut nc = l.content[..lead_len].to_vec();
            nc.push(b'@');
            nc.extend_from_slice(name);
            l.content = nc;
            changed = true;
        }
        changed
    })
}

fn general_phpdoc_annotation_remove(_s: &mut Stream) -> bool {
    // ECS's psr12+common configures no annotations to remove: no-op
    false
}

fn method_chaining_indentation(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 1;
    while i < s.len() {
        if s.kind(i) == Kind::Punct && (s.bytes(i) == b"->" || s.bytes(i) == b"?->") {
            if s.kind(i - 1) == Kind::Whitespace && s.bytes(i - 1).contains(&b'\n') {
                let ws = s.bytes(i - 1).to_vec();
                let nl = ws.iter().rposition(|&c| c == b'\n').unwrap();
                let mut want = chain_first_line_indent(s, i);
                want.extend_from_slice(b"    ");
                if &ws[nl + 1..] != want.as_slice() {
                    let mut nv = ws[..nl + 1].to_vec();
                    nv.extend_from_slice(&want);
                    s.set_owned(i - 1, nv);
                    changed = true;
                }
            }
        }
        i += 1;
    }
    changed
}

const PHPDOC_ORDER_RANK_NONE: i32 = -1;

fn phpdoc_order_rank(tag: &[u8]) -> (i32, bool) {
    let mut base = tag.to_ascii_lowercase();
    for pre in [b"phpstan-".as_slice(), b"psalm-".as_slice()] {
        if base.starts_with(pre) {
            base = base[pre.len()..].to_vec();
            break;
        }
    }
    match base.as_slice() {
        b"param" => (0, true),
        b"throws" => (1, true),
        b"return" => (2, true),
        _ => (PHPDOC_ORDER_RANK_NONE, false),
    }
}

// ^@([a-zA-Z][a-zA-Z0-9_-]*) on a whitespace-trimmed content
fn phpdoc_tag_name_of(c: &[u8]) -> Option<Vec<u8>> {
    if c.first() != Some(&b'@') {
        return None;
    }
    if c.len() < 2 || !c[1].is_ascii_alphabetic() {
        return None;
    }
    let mut j = 2;
    while j < c.len() && (c[j].is_ascii_alphanumeric() || c[j] == b'_' || c[j] == b'-') {
        j += 1;
    }
    Some(c[1..j].to_vec())
}

struct PhpdocBlock {
    lines: Vec<DocLine>,
    rank: i32,
    target: bool,
}

fn split_phpdoc_blocks(inner: &[DocLine]) -> Vec<PhpdocBlock> {
    let mut blocks = Vec::new();
    let mut i = 0;
    while i < inner.len() {
        let c = trim_ascii(&inner[i].content);
        if let Some(name) = phpdoc_tag_name_of(c) {
            let (rank, target) = phpdoc_order_rank(&name);
            let mut j = i + 1;
            while j < inner.len() {
                let cj = trim_ascii(&inner[j].content);
                if cj.is_empty() || cj.first() == Some(&b'@') {
                    break;
                }
                j += 1;
            }
            blocks.push(PhpdocBlock {
                lines: inner[i..j].to_vec(),
                rank,
                target,
            });
            i = j;
            continue;
        }
        blocks.push(PhpdocBlock {
            lines: inner[i..i + 1].to_vec(),
            rank: 0,
            target: false,
        });
        i += 1;
    }
    blocks
}

fn doc_lines_eq(a: &[DocLine], b: &[DocLine]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .all(|(x, y)| x.prefix == y.prefix && x.content == y.content)
}

fn phpdoc_order(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| {
        if d.single {
            return false;
        }
        let mut blocks = split_phpdoc_blocks(&d.inner);
        let target_pos: Vec<usize> = (0..blocks.len()).filter(|&i| blocks[i].target).collect();
        if target_pos.len() < 2 {
            return false;
        }
        let mut order: Vec<usize> = (0..target_pos.len()).collect();
        // stable insertion sort by rank
        for i in 1..order.len() {
            let mut j = i;
            while j > 0 && blocks[target_pos[order[j - 1]]].rank > blocks[target_pos[order[j]]].rank {
                order.swap(j - 1, j);
                j -= 1;
            }
        }
        // already ordered? (order is identity and lines unchanged)
        if order.iter().enumerate().all(|(k, &v)| k == v) {
            return false;
        }
        let sorted_lines: Vec<Vec<DocLine>> =
            order.iter().map(|&k| blocks[target_pos[k]].lines.clone()).collect();
        // compare original vs sorted at the target positions
        let same = order.iter().enumerate().all(|(k, _)| {
            doc_lines_eq(&blocks[target_pos[k]].lines, &sorted_lines[k])
        });
        if same {
            return false;
        }
        for (k, &pos) in target_pos.iter().enumerate() {
            blocks[pos].lines = sorted_lines[k].clone();
        }
        let mut inner = Vec::new();
        for b in &blocks {
            inner.extend_from_slice(&b.lines);
        }
        d.inner = inner;
        true
    })
}

fn prev_value_token_index(s: &Stream, i: usize) -> usize {
    let mut j = i as isize - 1;
    while j > 0 {
        let k = j as usize;
        let kind = s.kind(k);
        if kind == Kind::Number
            || kind == Kind::Ident
            || kind == Kind::Variable
            || kind == Kind::String
            || (kind == Kind::Punct && s.bytes(k) == b")")
        {
            return k;
        }
        j -= 1;
    }
    i
}

fn multiline_whitespace_before_semicolons(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword && s.bytes(i).eq_ignore_ascii_case(b"const") {
            let mut j = i + 1;
            while j < s.len() {
                if s.kind(j) == Kind::Punct && s.bytes(j) == b";" {
                    i = j;
                    break;
                }
                j += 1;
            }
            i += 1;
            continue;
        }
        if s.kind(i) != Kind::Punct || s.bytes(i) != b";" {
            i += 1;
            continue;
        }
        if i == 0 {
            i += 1;
            continue;
        }
        let prev = i - 1;
        if s.kind(prev) != Kind::Whitespace || !has_newline(s.bytes(prev)) {
            i += 1;
            continue;
        }
        let is_comment_prev2 = i >= 2 && matches!(s.kind(i - 2), Kind::Comment | Kind::DocComment);
        let starts_nl = matches!(s.bytes(prev).first(), Some(&b'\n') | Some(&b'\r'));
        if is_comment_prev2 && starts_nl {
            let sig = prev_value_token_index(s, i);
            s.remove_at(i);
            s.remove_at(prev);
            s.insert_owned(sig + 1, Kind::Punct, b";".to_vec());
            changed = true;
            i += 1;
            continue;
        }
        // drop the multi-line whitespace so ";" follows the code; re-examine the
        // token that now sits at index i (what previously followed the ";")
        s.remove_at(prev);
        changed = true;
        // ";" is now at prev; next token to examine is at i (was i+1)
    }
    changed
}

fn is_func_modifier_kw(lo: &[u8]) -> bool {
    matches!(
        lo,
        b"public" | b"protected" | b"private" | b"static" | b"final" | b"abstract"
    )
}

fn amp_param_names_after(s: &Stream, fn_idx: usize) -> Vec<Vec<u8>> {
    let mut open = next_significant_index(s, fn_idx);
    while let Some(o) = open {
        if s.kind(o) == Kind::Punct {
            break;
        }
        open = next_significant_index(s, o);
    }
    let mut open = match open {
        Some(o) => o,
        None => return Vec::new(),
    };
    if s.bytes(open) == b"&" {
        match next_significant_index(s, open) {
            Some(o) => open = o,
            None => return Vec::new(),
        }
    }
    if s.bytes(open) != b"(" {
        return Vec::new();
    }
    let close = match match_forward(s, open) {
        Some(c) => c,
        None => return Vec::new(),
    };
    let mut names = Vec::new();
    let mut depth = 0i32;
    let mut k = open + 1;
    while k < close {
        if s.kind(k) == Kind::Punct {
            match s.bytes(k) {
                b"(" | b"[" | b"{" => depth += 1,
                b")" | b"]" | b"}" => depth -= 1,
                _ => {}
            }
            k += 1;
            continue;
        }
        if depth == 0 && s.kind(k) == Kind::Variable {
            names.push(s.bytes(k).to_vec());
        }
        k += 1;
    }
    names
}

fn amp_doc_block_param_names(s: &Stream, i: usize) -> Vec<Vec<u8>> {
    let mut j = i + 1;
    while j < s.len() {
        match s.kind(j) {
            Kind::Whitespace | Kind::Comment => {
                j += 1;
                continue;
            }
            Kind::Keyword => {
                if s.bytes(j).eq_ignore_ascii_case(b"function") {
                    return amp_param_names_after(s, j);
                }
                let lo = s.bytes(j).to_ascii_lowercase();
                if is_func_modifier_kw(&lo) {
                    j += 1;
                    continue;
                }
                return Vec::new();
            }
            _ => return Vec::new(),
        }
    }
    Vec::new()
}

fn amp_add_param_names(d: &mut Doc, names: &[Vec<u8>]) -> bool {
    let mut changed = false;
    let mut idx = 0usize;
    for l in d.inner.iter_mut() {
        let trimmed = trim_left_space(&l.content);
        if trimmed.len() < 6 || !trimmed[..6].eq_ignore_ascii_case(b"@param") {
            continue;
        }
        let rest = &trimmed[6..];
        if rest.is_empty() || (rest[0] != b' ' && rest[0] != b'\t') {
            continue;
        }
        // fields = whitespace-split of rest
        let fields: Vec<&[u8]> = rest
            .split(|&c| c == b' ' || c == b'\t' || c == b'\n' || c == b'\r')
            .filter(|f| !f.is_empty())
            .collect();
        let pos = idx;
        idx += 1;
        if fields.len() != 1 {
            continue;
        }
        if pos >= names.len() {
            continue;
        }
        let mut nc = trim_right_ws_st(&l.content).to_vec();
        nc.push(b' ');
        nc.extend_from_slice(&names[pos]);
        l.content = nc;
        changed = true;
    }
    changed
}

fn add_missing_param_name(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::DocComment {
            i += 1;
            continue;
        }
        let names = amp_doc_block_param_names(s, i);
        if names.is_empty() {
            i += 1;
            continue;
        }
        if let Some(mut d) = parse_doc(s.bytes(i)) {
            if amp_add_param_names(&mut d, &names) {
                s.set_owned(i, doc_render(&d));
                changed = true;
            }
        }
        i += 1;
    }
    changed
}

struct OtTypeMember {
    toks: Vec<(Kind, Vec<u8>)>,
    key: Vec<u8>,
}

fn ot_collect_type_members(s: &Stream, from: usize, to: usize, op: &[u8]) -> Option<Vec<OtTypeMember>> {
    let mut members: Vec<OtTypeMember> = Vec::new();
    let mut cur = OtTypeMember { toks: Vec::new(), key: Vec::new() };
    let mut j = from;
    while j < to {
        match s.kind(j) {
            Kind::Whitespace => {
                if has_newline(s.bytes(j)) {
                    return None;
                }
            }
            Kind::Comment | Kind::DocComment => return None,
            Kind::Punct => {
                if s.bytes(j) == op {
                    if cur.toks.is_empty() {
                        return None;
                    }
                    cur.key = cur.key.to_ascii_lowercase();
                    members.push(cur);
                    cur = OtTypeMember { toks: Vec::new(), key: Vec::new() };
                } else if s.bytes(j) == b"\\" {
                    cur.toks.push((Kind::Punct, s.bytes(j).to_vec()));
                    cur.key.extend_from_slice(s.bytes(j));
                } else {
                    return None;
                }
            }
            k => {
                cur.toks.push((k, s.bytes(j).to_vec()));
                cur.key.extend_from_slice(s.bytes(j));
            }
        }
        j += 1;
    }
    if cur.toks.is_empty() {
        return None;
    }
    cur.key = cur.key.to_ascii_lowercase();
    members.push(cur);
    Some(members)
}

struct DsToken {
    text: Vec<u8>,
    is_punct: bool,
    gap: Vec<u8>,
}

fn ds_is_annotation_name_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'\\'
}

fn ds_is_doctrine_annotation_name(name: &[u8]) -> bool {
    if name.contains(&b'\\') {
        return true;
    }
    name[0].is_ascii_uppercase()
}

fn ds_is_punct_char(c: u8) -> bool {
    matches!(c, b'(' | b')' | b'{' | b'}' | b',' | b'=')
}

fn ds_match_annotation_paren(s: &[u8], open: usize) -> isize {
    let mut depth = 0i32;
    let mut in_str = false;
    let mut i = open;
    while i < s.len() {
        let c = s[i];
        if in_str {
            if c == b'\\' {
                i += 2;
                continue;
            }
            if c == b'"' {
                in_str = false;
            }
            i += 1;
            continue;
        }
        match c {
            b'"' => in_str = true,
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return i as isize;
                }
            }
            _ => {}
        }
        i += 1;
    }
    -1
}

fn ds_tokenize(inner: &[u8]) -> Vec<DsToken> {
    let mut toks: Vec<DsToken> = Vec::new();
    let mut i = 0;
    let n = inner.len();
    while i < n {
        let c = inner[i];
        if c == b' ' || c == b'\t' {
            let start = i;
            while i < n && (inner[i] == b' ' || inner[i] == b'\t') {
                i += 1;
            }
            if let Some(last) = toks.last_mut() {
                last.gap = inner[start..i].to_vec();
            }
            continue;
        }
        if c == b'"' {
            let start = i;
            i += 1;
            while i < n {
                if inner[i] == b'\\' {
                    i += 2;
                    continue;
                }
                if inner[i] == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            toks.push(DsToken { text: inner[start..i.min(n)].to_vec(), is_punct: false, gap: Vec::new() });
            continue;
        }
        if ds_is_punct_char(c) {
            toks.push(DsToken { text: vec![c], is_punct: true, gap: Vec::new() });
            i += 1;
            continue;
        }
        let start = i;
        while i < n {
            let ch = inner[i];
            if ch == b' ' || ch == b'\t' || ch == b'"' || ds_is_punct_char(ch) {
                break;
            }
            i += 1;
        }
        toks.push(DsToken { text: inner[start..i].to_vec(), is_punct: false, gap: Vec::new() });
    }
    toks
}

fn ds_gap_between(a: &DsToken, next: &DsToken, a_depth: i32, next_depth: i32) -> Vec<u8> {
    if a.is_punct && a.text == b"(" {
        return Vec::new();
    }
    if next.is_punct && next.text == b")" {
        return Vec::new();
    }
    if next.is_punct && next.text == b"," {
        return Vec::new();
    }
    if a.is_punct && a.text == b"," {
        if a.gap.is_empty() {
            return b" ".to_vec();
        }
        return a.gap.clone();
    }
    if a.is_punct && a.text == b"=" {
        if a_depth > 0 {
            return b" ".to_vec();
        }
        return Vec::new();
    }
    if next.is_punct && next.text == b"=" {
        if next_depth > 0 {
            return b" ".to_vec();
        }
        return Vec::new();
    }
    a.gap.clone()
}

fn ds_normalize_inner(inner: &[u8]) -> Vec<u8> {
    let toks = ds_tokenize(inner);
    if toks.is_empty() {
        return Vec::new();
    }
    let mut depth = vec![0i32; toks.len()];
    let mut d = 0i32;
    for (i, t) in toks.iter().enumerate() {
        depth[i] = d;
        if t.is_punct && t.text == b"{" {
            d += 1;
        } else if t.is_punct && t.text == b"}" && d > 0 {
            d -= 1;
        }
    }
    let mut out: Vec<u8> = Vec::new();
    for i in 0..toks.len() {
        out.extend_from_slice(&toks[i].text);
        if i == toks.len() - 1 {
            break;
        }
        out.extend_from_slice(&ds_gap_between(&toks[i], &toks[i + 1], depth[i], depth[i + 1]));
    }
    out
}

fn ds_fix_line(content: &[u8]) -> Option<Vec<u8>> {
    let mut lead_len = 0;
    while lead_len < content.len() && (content[lead_len] == b' ' || content[lead_len] == b'\t') {
        lead_len += 1;
    }
    let lead = &content[..lead_len];
    let rest = &content[lead_len..];
    if rest.first() != Some(&b'@') {
        return None;
    }
    let mut j = 1;
    while j < rest.len() && ds_is_annotation_name_byte(rest[j]) {
        j += 1;
    }
    let name = &rest[1..j];
    if name.is_empty() || !ds_is_doctrine_annotation_name(name) {
        return None;
    }
    let mut k = j;
    while k < rest.len() && (rest[k] == b' ' || rest[k] == b'\t') {
        k += 1;
    }
    if k >= rest.len() || rest[k] != b'(' {
        return None;
    }
    let close_idx = ds_match_annotation_paren(rest, k);
    if close_idx < 0 {
        return None;
    }
    let close = close_idx as usize;
    let tail = &rest[close + 1..];
    if !trim_right_ws_st(tail).is_empty() {
        return None;
    }
    let inner = ds_normalize_inner(&rest[k + 1..close]);
    let mut out = lead.to_vec();
    out.push(b'@');
    out.extend_from_slice(name);
    out.push(b'(');
    out.extend_from_slice(&inner);
    out.push(b')');
    out.extend_from_slice(tail);
    Some(out)
}

fn ruc_is_word(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}
fn ruc_is_ws(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0c)
}

fn ruc_first_class_like_name(s: &Stream) -> Vec<u8> {
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword && is_class_like_keyword(&s.bytes(i).to_ascii_lowercase()) {
            if let Some(n) = next_significant_index(s, i) {
                if s.kind(n) == Kind::Ident {
                    return s.bytes(n).to_vec();
                }
            }
        }
        i += 1;
    }
    Vec::new()
}

// trims a set of trailing/leading bytes (like strings.Trim(line, "* "))
fn ruc_trim_star_space(b: &[u8]) -> &[u8] {
    let mut st = 0;
    let mut en = b.len();
    while st < en && (b[st] == b'*' || b[st] == b' ') {
        st += 1;
    }
    while en > st && (b[en - 1] == b'*' || b[en - 1] == b' ') {
        en -= 1;
    }
    &b[st..en]
}

// --- individual useless-comment patterns (each returns the line with the
// matched span removed, mirroring Preg::replace(..., '', line)). ---

fn ruc_strip_suffix(line: &[u8], suffix: &[u8]) -> Vec<u8> {
    if line.ends_with(suffix) {
        line[..line.len() - suffix.len()].to_vec()
    } else {
        line.to_vec()
    }
}

// // TODO: Implement .*\(\) method.$
fn ruc_todo_implement(line: &[u8]) -> Vec<u8> {
    let head = b"// TODO: Implement ";
    // leftmost occurrence of head
    if line.len() < 10 {
        return line.to_vec();
    }
    // tail must be "() method" + exactly one char at end
    let n = line.len();
    if &line[n - 10..n - 1] != b"() method" {
        return line.to_vec();
    }
    // find head at or before n-10
    let limit = n - 10;
    let mut p = 0;
    while p + head.len() <= line.len() {
        if &line[p..p + head.len()] == head && p <= limit {
            return line[..p].to_vec();
        }
        p += 1;
    }
    line.to_vec()
}

// (?i)^(//|(\s|\*)+)(\s\w+\s)?constructor(\.)?$  -> whole-line match => ""
fn ruc_constructor_line(line: &[u8]) -> bool {
    let lc = line.to_ascii_lowercase();
    let n = lc.len();
    // (\s\w+\s) then constructor(\.)? $
    let matches_from = |start: usize| -> bool {
        for cand in [ruc_opt_ws_word_ws(&lc, start), Some(start)] {
            if let Some(mut q) = cand {
                if n >= q + 11 && &lc[q..q + 11] == b"constructor" {
                    q += 11;
                    if q < n && lc[q] == b'.' {
                        q += 1;
                    }
                    if q == n {
                        return true;
                    }
                }
            }
        }
        false
    };
    // prefix "//"
    if n >= 2 && &lc[0..2] == b"//" && matches_from(2) {
        return true;
    }
    // prefix (\s|*)+ ; greedy with backtracking over its length
    let mut max = 0;
    while max < n && (ruc_is_ws(lc[max]) || lc[max] == b'*') {
        max += 1;
    }
    let mut pe = max;
    while pe >= 1 {
        if matches_from(pe) {
            return true;
        }
        pe -= 1;
    }
    false
}

// optional (\s\w+\s): a whitespace, word chars, a whitespace
fn ruc_opt_ws_word_ws(lc: &[u8], start: usize) -> Option<usize> {
    let n = lc.len();
    let mut q = start;
    if q >= n || !ruc_is_ws(lc[q]) {
        return None;
    }
    q += 1;
    let ws0 = q;
    while q < n && ruc_is_word(lc[q]) {
        q += 1;
    }
    if q == ws0 {
        return None;
    }
    if q >= n || !ruc_is_ws(lc[q]) {
        return None;
    }
    q += 1;
    Some(q)
}

// generic: (?i)? PREFIX + one of KWs? — implement the four "class/trait/interface"
// and "Class representing" suffix removers directly.

// (?i)//\s+(class|trait|interface)\s+\w+$
fn ruc_slashslash_class(line: &[u8]) -> Vec<u8> {
    ruc_kw_suffix(line, b"//", true)
}
// (?i)\s\*\s(class|trait|interface)\s+(\w)+$
fn ruc_star_class(line: &[u8]) -> Vec<u8> {
    ruc_kw_suffix(line, b"*", false)
}

// removes a trailing "PREFIX \s+ (class|trait|interface) \s+ word+" ; when
// star=false the prefix is "\s*\s" (a whitespace, "*", a whitespace).
fn ruc_kw_suffix(line: &[u8], _p: &[u8], slashes: bool) -> Vec<u8> {
    let lc = line.to_ascii_lowercase();
    let n = lc.len();
    // parse from end: word+ , \s+ , kw , \s+ , prefix
    let mut e = n;
    // trailing word+
    let mut we = e;
    while we > 0 && ruc_is_word(lc[we - 1]) {
        we -= 1;
    }
    if we == e {
        return line.to_vec();
    }
    e = we;
    // \s+
    let mut se = e;
    while se > 0 && ruc_is_ws(lc[se - 1]) {
        se -= 1;
    }
    if se == e {
        return line.to_vec();
    }
    e = se;
    // kw
    let kw: &[u8] = if e >= 9 && &lc[e - 9..e] == b"interface" {
        b"interface"
    } else if e >= 5 && &lc[e - 5..e] == b"class" {
        b"class"
    } else if e >= 5 && &lc[e - 5..e] == b"trait" {
        b"trait"
    } else {
        return line.to_vec();
    };
    e -= kw.len();
    if slashes {
        // //\s+  : need "//" then \s+ up to e ... prefix is "//" then \s+
        // \s+ before kw
        let mut ws = e;
        while ws > 0 && ruc_is_ws(lc[ws - 1]) {
            ws -= 1;
        }
        if ws == e {
            return line.to_vec();
        }
        // now expect "//" ending at ws
        if ws >= 2 && &lc[ws - 2..ws] == b"//" {
            let start = ws - 2;
            return line[..start].to_vec();
        }
        line.to_vec()
    } else {
        // \s\*\s : a whitespace, "*", a whitespace, then kw
        // one \s before kw
        if e == 0 || !ruc_is_ws(lc[e - 1]) {
            return line.to_vec();
        }
        let mut m = e - 1;
        if m == 0 || lc[m - 1] != b'*' {
            return line.to_vec();
        }
        m -= 1;
        if m == 0 || !ruc_is_ws(lc[m - 1]) {
            return line.to_vec();
        }
        m -= 1;
        line[..m].to_vec()
    }
}

// (?i)\s\*\sClass\s+representing\s+(\w+)$  (case-insensitive on the whole)
fn ruc_class_representing(line: &[u8]) -> Vec<u8> {
    let lc = line.to_ascii_lowercase();
    let n = lc.len();
    let mut e = n;
    // word+
    let mut we = e;
    while we > 0 && ruc_is_word(lc[we - 1]) {
        we -= 1;
    }
    if we == e {
        return line.to_vec();
    }
    e = we;
    // \s+
    let mut se = e;
    while se > 0 && ruc_is_ws(lc[se - 1]) {
        se -= 1;
    }
    if se == e {
        return line.to_vec();
    }
    e = se;
    if e < 12 || &lc[e - 12..e] != b"representing" {
        return line.to_vec();
    }
    e -= 12;
    // \s+
    let mut s2 = e;
    while s2 > 0 && ruc_is_ws(lc[s2 - 1]) {
        s2 -= 1;
    }
    if s2 == e {
        return line.to_vec();
    }
    e = s2;
    if e < 5 || &lc[e - 5..e] != b"class" {
        return line.to_vec();
    }
    e -= 5;
    // \s\*\s
    if e == 0 || !ruc_is_ws(lc[e - 1]) {
        return line.to_vec();
    }
    let mut m = e - 1;
    if m == 0 || lc[m - 1] != b'*' {
        return line.to_vec();
    }
    m -= 1;
    if m == 0 || !ruc_is_ws(lc[m - 1]) {
        return line.to_vec();
    }
    m -= 1;
    line[..m].to_vec()
}

fn ruc_apply_line_regexes(line: &[u8]) -> Vec<u8> {
    let mut v = ruc_strip_suffix(line, b"// TODO: Change the autogenerated stub");
    v = ruc_todo_implement(&v);
    if ruc_constructor_line(&v) {
        v = Vec::new();
    }
    v = ruc_class_representing(&v);
    // R5 (single-line "/** * class X */") is extremely rare; handled by the
    // whole-block empty check below when it applies.
    v = ruc_slashslash_class(&v);
    v = ruc_star_class(&v);
    v
}

// (?i)^(/\*{2}\s+?)?(\*|//)\s+This class was generated by the Doctrine ORM\. Add
//  your own custom\r?\n\s+\*\s+repository methods below\.(\s+\*/)$
fn ruc_doctrine_generated(text: &[u8]) -> Vec<u8> {
    let lc = text.to_ascii_lowercase();
    let needle1 = b"this class was generated by the doctrine orm. add your own custom";
    let needle2 = b"repository methods below.";
    if bytes_contains(&lc, needle1) && bytes_contains(&lc, needle2) && lc.ends_with(b"*/") {
        // conservative: matches the full generated block; ECS replaces it with ""
        return Vec::new();
    }
    text.to_vec()
}

fn ruc_clear_useless_doc_content(content: &[u8], class_name: &[u8]) -> Vec<u8> {
    let lines: Vec<&[u8]> = content.split(|&c| c == b'\n').collect();
    let mut cleaned: Vec<Vec<u8>> = Vec::new();
    for line in &lines {
        if !class_name.is_empty() && ruc_trim_star_space(line) == class_name {
            continue;
        }
        cleaned.push(ruc_apply_line_regexes(line));
    }
    let kept: Vec<Vec<u8>> = cleaned.into_iter().filter(|l| !l.is_empty()).collect();
    if kept.len() == 2 && kept[0] == b"/**" && trim_ascii(&kept[1]) == b"*/" {
        return Vec::new();
    }
    let joined = kept.join(&b'\n');
    ruc_doctrine_generated(&joined)
}

fn remove_useless_default_comment(s: &mut Stream) -> bool {
    let class_name = ruc_first_class_like_name(s);
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Comment && s.kind(i) != Kind::DocComment {
            i += 1;
            continue;
        }
        let original = s.bytes(i).to_vec();
        let cleaned = ruc_clear_useless_doc_content(&original, &class_name);
        if cleaned.is_empty() {
            s.remove_at(i);
            changed = true;
            continue;
        } else if cleaned != original {
            s.set_owned(i, cleaned);
            changed = true;
        }
        i += 1;
    }
    changed
}

fn doctrine_annotation_spaces(s: &mut Stream) -> bool {
    apply_to_docblocks(s, |d| {
        let mut changed = false;
        for l in d.inner.iter_mut() {
            if let Some(fixed) = ds_fix_line(&l.content) {
                if fixed != l.content {
                    l.content = fixed;
                    changed = true;
                }
            }
        }
        changed
    })
}

fn ordered_types(s: &mut Stream) -> bool {
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
        let start = match type_run_boundary_prev(s, i) {
            Some(st) => st,
            None => {
                i += 1;
                continue;
            }
        };
        let end = match type_run_boundary_next(s, i) {
            Some(e) => e,
            None => {
                i += 1;
                continue;
            }
        };
        let mut run_start = start + 1;
        while run_start < end && s.kind(run_start) == Kind::Whitespace {
            run_start += 1;
        }
        let mut run_end = end - 1;
        while run_end > run_start && s.kind(run_end) == Kind::Whitespace {
            run_end -= 1;
        }
        // process the run once, at its leftmost operator
        let mut first = None;
        let mut j = run_start;
        while j <= run_end {
            if s.kind(j) == Kind::Punct && (s.bytes(j) == b"|" || s.bytes(j) == b"&") {
                first = Some(j);
                break;
            }
            j += 1;
        }
        if first != Some(i) {
            i += 1;
            continue;
        }
        let op = s.bytes(i).to_vec();
        let members = match ot_collect_type_members(s, run_start, run_end + 1, &op) {
            Some(m) if m.len() >= 2 => m,
            _ => {
                i += 1;
                continue;
            }
        };
        let mut order: Vec<usize> = (0..members.len()).collect();
        // stable sort: null first, then key ascending
        order.sort_by(|&a, &b| {
            let an = members[a].key == b"null";
            let bn = members[b].key == b"null";
            if an != bn {
                return bn.cmp(&an); // null (true) sorts first
            }
            members[a].key.cmp(&members[b].key)
        });
        // already ordered? compare keys positionally (matches sameMemberOrder)
        if (0..members.len()).all(|k| members[order[k]].key == members[k].key) {
            i += 1;
            continue;
        }
        // rebuild replacement tokens
        let mut repl: Vec<(Kind, Vec<u8>)> = Vec::new();
        for (idx, &mi) in order.iter().enumerate() {
            if idx > 0 {
                repl.push((Kind::Punct, op.clone()));
            }
            for t in &members[mi].toks {
                repl.push((t.0, t.1.clone()));
            }
        }
        // remove [run_start..=run_end], insert repl at run_start
        for k in (run_start..=run_end).rev() {
            s.remove_at(k);
        }
        for (off, (k, v)) in repl.iter().enumerate() {
            s.insert_owned(run_start + off, *k, v.clone());
        }
        changed = true;
        i = run_start + repl.len();
    }
    changed
}

// --- FullyQualifiedStrictTypes (full default-config fidelity) ---

const FQ_RESERVED: &[&[u8]] = &[
    b"array", b"bool", b"callable", b"false", b"float", b"int", b"iterable",
    b"list", b"mixed", b"never", b"null", b"object", b"parent", b"resource",
    b"self", b"static", b"string", b"true", b"void",
];

fn fq_is_reserved_type(lower: &[u8]) -> bool {
    FQ_RESERVED.iter().any(|r| *r == lower)
}

// Keyed by full name (like ECS's $uses): importing the same class twice keeps
// only the last alias; caches are derived from that in build().
struct FqUses {
    long_to_short: Vec<(Vec<u8>, Vec<u8>)>,
    nbsl: Vec<(Vec<u8>, Vec<u8>)>,   // (lower(short), long)
    sbn: Vec<(Vec<u8>, Vec<u8>)>,    // (long, short)
    sbnorm: Vec<(Vec<u8>, Vec<u8>)>, // (normalize(long), short)
}

impl FqUses {
    fn new() -> Self {
        FqUses { long_to_short: Vec::new(), nbsl: Vec::new(), sbn: Vec::new(), sbnorm: Vec::new() }
    }
    fn add(&mut self, long: Vec<u8>, short: Vec<u8>) {
        if let Some(e) = self.long_to_short.iter_mut().find(|(l, _)| *l == long) {
            e.1 = short;
        } else {
            self.long_to_short.push((long, short));
        }
    }
    fn build(&mut self) {
        for (long, short) in &self.long_to_short {
            self.nbsl.push((short.to_ascii_lowercase(), long.clone()));
            self.sbn.push((long.clone(), short.clone()));
            self.sbnorm.push((fq_normalize(long), short.clone()));
        }
    }
    fn is_empty(&self) -> bool {
        self.long_to_short.is_empty()
    }
    fn name_by_short_lower(&self, short_lower: &[u8]) -> Option<&[u8]> {
        self.nbsl.iter().rev().find(|(k, _)| k.as_slice() == short_lower).map(|(_, l)| l.as_slice())
    }
    fn short_by_name(&self, name: &[u8]) -> Option<&[u8]> {
        self.sbn.iter().rev().find(|(k, _)| k.as_slice() == name).map(|(_, s)| s.as_slice())
    }
    fn short_by_normalized(&self, norm: &[u8]) -> Option<&[u8]> {
        self.sbnorm.iter().rev().find(|(k, _)| k.as_slice() == norm).map(|(_, s)| s.as_slice())
    }
}

fn fq_normalize(input: &[u8]) -> Vec<u8> {
    match input.iter().rposition(|&c| c == b'\\') {
        None => input.to_ascii_lowercase(),
        Some(bs) => {
            let mut out = input[..bs + 1].to_vec();
            out.extend_from_slice(&input[bs + 1..].to_ascii_lowercase());
            out
        }
    }
}

fn fq_is_reserved(symbol: &[u8], reserved: &[Vec<u8>]) -> bool {
    if symbol.contains(&b'\\') {
        return false;
    }
    if fq_is_reserved_type(&symbol.to_ascii_lowercase()) {
        return true;
    }
    reserved.iter().any(|r| r.as_slice() == symbol)
}

fn fq_cut<'a>(s: &'a [u8], sep: u8) -> (&'a [u8], Option<&'a [u8]>) {
    match s.iter().position(|&c| c == sep) {
        Some(i) => (&s[..i], Some(&s[i + 1..])),
        None => (s, None),
    }
}

fn fq_resolve_symbol(symbol: &[u8], u: &FqUses, ns: &[u8], reserved: &[Vec<u8>]) -> Vec<u8> {
    if symbol.first() == Some(&b'\\') {
        return symbol[1..].to_vec();
    }
    if fq_is_reserved(symbol, reserved) {
        return symbol.to_vec();
    }
    let (first, rest) = fq_cut(symbol, b'\\');
    if let Some(long) = u.name_by_short_lower(&first.to_ascii_lowercase()) {
        let mut out = long.to_vec();
        if let Some(r) = rest {
            out.push(b'\\');
            out.extend_from_slice(r);
        }
        return out;
    }
    if !ns.is_empty() {
        let mut out = ns.to_vec();
        out.push(b'\\');
        out.extend_from_slice(symbol);
        return out;
    }
    symbol.to_vec()
}

fn fq_count(hay: &[u8], b: u8) -> usize {
    hay.iter().filter(|&&c| c == b).count()
}

fn fq_shorten_symbol(fqcn: &[u8], u: &FqUses, ns: &[u8], reserved: &[Vec<u8>]) -> Vec<u8> {
    if fq_is_reserved(fqcn, reserved) {
        return fqcn.to_vec();
    }
    let mut res: Option<Vec<u8>> = None;
    let mut i_min: isize = 0;

    if !ns.is_empty() {
        let mut prefix = ns.to_vec();
        prefix.push(b'\\');
        if fqcn.starts_with(&prefix) {
            let tmp_res = &fqcn[ns.len() + 1..];
            let (first_seg, _) = fq_cut(tmp_res, b'\\');
            if u.name_by_short_lower(&first_seg.to_ascii_lowercase()).is_none()
                && !fq_is_reserved(tmp_res, reserved)
            {
                res = Some(tmp_res.to_vec());
                i_min = fq_count(ns, b'\\') as isize + 1;
            }
        }
    }

    let mut tmp = fqcn.to_vec();
    let mut i = fq_count(fqcn, b'\\') as isize;
    while i >= i_min {
        if let Some(short) = u.short_by_name(&tmp) {
            let mut tmp_res = short.to_vec();
            tmp_res.extend_from_slice(&fqcn[tmp.len()..]);
            if !fq_is_reserved(&tmp_res, reserved) {
                res = Some(tmp_res);
                break;
            }
        }
        if i > 0 {
            if let Some(bs) = tmp.iter().rposition(|&c| c == b'\\') {
                tmp.truncate(bs);
            }
        }
        i -= 1;
    }

    if res.is_none() {
        if let Some(short) = u.short_by_normalized(&fq_normalize(fqcn)) {
            if !fq_is_reserved(short, reserved) {
                res = Some(short.to_vec());
            }
        }
    }

    match res {
        Some(r) => r,
        None => {
            let (first_seg, _) = fq_cut(fqcn, b'\\');
            let collide = u.name_by_short_lower(&first_seg.to_ascii_lowercase()).is_some();
            if !ns.is_empty() || collide {
                let mut out = vec![b'\\'];
                out.extend_from_slice(fqcn);
                out
            } else {
                fqcn.to_vec()
            }
        }
    }
}

fn fq_determine_short(type_name: &[u8], u: &FqUses, ns: &[u8], reserved: &[Vec<u8>]) -> Option<Vec<u8>> {
    let fqcn = fq_resolve_symbol(type_name, u, ns, reserved);
    let shortened = fq_shorten_symbol(&fqcn, u, ns, reserved);
    if shortened == type_name {
        None
    } else {
        Some(shortened)
    }
}

struct FqRepl {
    start: usize,
    end: usize,
    tokens: Vec<(Kind, Vec<u8>)>,
}

fn fq_string_to_tokens(input: &[u8]) -> Vec<(Kind, Vec<u8>)> {
    let mut out = Vec::new();
    let mut inp = input;
    if inp.first() == Some(&b'\\') {
        out.push((Kind::Punct, b"\\".to_vec()));
        inp = &inp[1..];
    }
    let parts: Vec<&[u8]> = inp.split(|&c| c == b'\\').collect();
    for (i, p) in parts.iter().enumerate() {
        out.push((Kind::Ident, p.to_vec()));
        if i != parts.len() - 1 {
            out.push((Kind::Punct, b"\\".to_vec()));
        }
    }
    out
}

fn fq_read_namespace_name(s: &Stream, kw: usize) -> Option<(Vec<u8>, usize, usize)> {
    let mut b: Vec<u8> = Vec::new();
    let mut cur = sig_next(s, kw);
    while let Some(i) = cur {
        let k = s.kind(i);
        if k == Kind::Ident {
            b.extend_from_slice(s.bytes(i));
            cur = sig_next(s, i);
            continue;
        }
        if k == Kind::Punct && s.bytes(i) == b"\\" {
            b.push(b'\\');
            cur = sig_next(s, i);
            continue;
        }
        if k == Kind::Punct && (s.bytes(i) == b";" || s.bytes(i) == b"{") {
            return Some((b, i + 1, i));
        }
        break;
    }
    None
}

fn fq_namespace_regions(s: &Stream) -> Vec<(Vec<u8>, usize, usize)> {
    let mut decls: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if kw_is(s, i, b"namespace") {
            if let Some(n) = sig_next(s, i) {
                if s.kind(n) == Kind::Punct && s.bytes(n) == b"\\" {
                    i += 1;
                    continue;
                }
            }
            decls.push(i);
        }
        i += 1;
    }
    if decls.is_empty() {
        return vec![(Vec::new(), 0, s.len())];
    }
    let mut regions = Vec::new();
    for k in 0..decls.len() {
        let d = decls[k];
        if let Some((name, body_start, term)) = fq_read_namespace_name(s, d) {
            if s.bytes(term) == b"{" {
                let end = match_forward(s, term).unwrap_or(s.len());
                regions.push((name, body_start, end));
            } else {
                let end = if k + 1 < decls.len() { decls[k + 1] } else { s.len() };
                regions.push((name, body_start, end));
            }
        }
    }
    regions
}

fn fq_read_import(s: &Stream, start: usize) -> Option<(Vec<u8>, Vec<u8>, usize)> {
    let mut b: Vec<u8> = Vec::new();
    let mut i = start;
    if is_punct_val(s, i, b"\\") {
        i += 1;
    }
    let mut expect_name = true;
    while i < s.len() {
        let k = s.kind(i);
        if k == Kind::Whitespace || k == Kind::Comment || k == Kind::DocComment {
            i += 1;
            continue;
        }
        if expect_name {
            if k != Kind::Ident {
                return None;
            }
            b.extend_from_slice(s.bytes(i));
            expect_name = false;
            i += 1;
            continue;
        }
        if k == Kind::Punct && s.bytes(i) == b"\\" {
            b.push(b'\\');
            expect_name = true;
            i += 1;
            continue;
        }
        if k == Kind::Punct && s.bytes(i) == b";" {
            return Some((b, Vec::new(), i));
        }
        if kw_is(s, i, b"as") {
            let a = sig_next(s, i)?;
            if s.kind(a) != Kind::Ident {
                return None;
            }
            let semi = sig_next(s, a)?;
            if !is_punct_val(s, semi, b";") {
                return None;
            }
            return Some((b, s.bytes(a).to_vec(), semi));
        }
        return None;
    }
    None
}

fn fq_collect_uses_region(s: &Stream, start: usize, end: usize) -> FqUses {
    let mut u = FqUses::new();
    let mut depth = 0i32;
    let mut i = start;
    while i < end && i < s.len() {
        if s.kind(i) == Kind::Punct {
            match s.bytes(i) {
                b"{" => depth += 1,
                b"}" => depth -= 1,
                _ => {}
            }
            i += 1;
            continue;
        }
        if depth != 0 || !kw_is(s, i, b"use") {
            i += 1;
            continue;
        }
        let n = match sig_next(s, i) {
            Some(n) => n,
            None => {
                i += 1;
                continue;
            }
        };
        if s.kind(n) == Kind::Keyword {
            let lv = s.bytes(n).to_ascii_lowercase();
            if lv == b"function" || lv == b"const" {
                i += 1;
                continue;
            }
        }
        match fq_read_import(s, n) {
            Some((fqcn, alias, e)) => {
                let short = if !alias.is_empty() {
                    alias
                } else if let Some(idx) = fqcn.iter().rposition(|&c| c == b'\\') {
                    fqcn[idx + 1..].to_vec()
                } else {
                    fqcn.clone()
                };
                u.add(fqcn, short);
                i = e + 1;
            }
            None => i += 1,
        }
    }
    u.build();
    u
}

fn fq_is_name_tok(s: &Stream, i: usize) -> bool {
    s.kind(i) == Kind::Ident || (s.kind(i) == Kind::Punct && s.bytes(i) == b"\\")
}

fn fq_read_run_forward(s: &Stream, start: usize) -> Option<(Vec<u8>, usize)> {
    if start >= s.len() || !fq_is_name_tok(s, start) {
        return None;
    }
    let mut b: Vec<u8> = Vec::new();
    let mut i = start;
    let mut last = start;
    while i < s.len() && fq_is_name_tok(s, i) {
        b.extend_from_slice(s.bytes(i));
        last = i;
        i += 1;
    }
    Some((b, last))
}

fn fq_read_run_backward(s: &Stream, end: usize) -> Option<(Vec<u8>, usize)> {
    if end >= s.len() {
        return None;
    }
    let mut first = end;
    let mut i = end as isize;
    while i >= 0 && fq_is_name_tok(s, i as usize) {
        first = i as usize;
        i -= 1;
    }
    if !fq_is_name_tok(s, first) {
        return None;
    }
    let mut b: Vec<u8> = Vec::new();
    for j in first..=end {
        b.extend_from_slice(s.bytes(j));
    }
    if b.is_empty() {
        return None;
    }
    Some((b, first))
}

fn fq_next_name(s: &Stream, kw: usize, u: &FqUses, ns: &[u8], reserved: &[Vec<u8>]) -> Option<FqRepl> {
    let n = sig_next(s, kw)?;
    let (content, end) = fq_read_run_forward(s, n)?;
    let repl = fq_determine_short(&content, u, ns, reserved)?;
    Some(FqRepl { start: n, end, tokens: fq_string_to_tokens(&repl) })
}

fn fq_prev_name(s: &Stream, idx: usize, u: &FqUses, ns: &[u8], reserved: &[Vec<u8>]) -> Option<FqRepl> {
    let p = sig_prev(s, idx)?;
    if s.kind(p) != Kind::Ident {
        return None;
    }
    let (content, start) = fq_read_run_backward(s, p)?;
    // a name after an object operator (`$this->grammar::`) is a member, not a class
    if let Some(b) = sig_prev(s, start) {
        if s.kind(b) == Kind::Punct && (s.bytes(b) == b"->" || s.bytes(b) == b"?->") {
            return None;
        }
    }
    let repl = fq_determine_short(&content, u, ns, reserved)?;
    Some(FqRepl { start, end: p, tokens: fq_string_to_tokens(&repl) })
}

fn fq_extends_implements(s: &Stream, kw: usize, u: &FqUses, ns: &[u8], reserved: &[Vec<u8>]) -> Vec<FqRepl> {
    let mut out = Vec::new();
    let mut cur = sig_next(s, kw);
    while let Some(i) = cur {
        if s.kind(i) == Kind::Punct && s.bytes(i) == b"{" {
            break;
        }
        if s.kind(i) == Kind::Keyword {
            break;
        }
        if fq_is_name_tok(s, i) {
            if let Some((content, end)) = fq_read_run_forward(s, i) {
                if let Some(repl) = fq_determine_short(&content, u, ns, reserved) {
                    out.push(FqRepl { start: i, end, tokens: fq_string_to_tokens(&repl) });
                }
                cur = sig_next(s, end);
                continue;
            }
        }
        cur = sig_next(s, i);
    }
    out
}

fn fq_catch(s: &Stream, kw: usize, u: &FqUses, ns: &[u8], reserved: &[Vec<u8>]) -> Vec<FqRepl> {
    let open = match sig_next(s, kw) {
        Some(o) if is_punct_val(s, o, b"(") => o,
        _ => return Vec::new(),
    };
    let mut out = Vec::new();
    let mut cur = sig_next(s, open);
    while let Some(i) = cur {
        if (s.kind(i) == Kind::Punct && s.bytes(i) == b")") || s.kind(i) == Kind::Variable {
            break;
        }
        if fq_is_name_tok(s, i) {
            if let Some((content, end)) = fq_read_run_forward(s, i) {
                if let Some(repl) = fq_determine_short(&content, u, ns, reserved) {
                    out.push(FqRepl { start: i, end, tokens: fq_string_to_tokens(&repl) });
                }
                cur = sig_next(s, end);
                continue;
            }
        }
        cur = sig_next(s, i);
    }
    out
}

fn fq_is_type_boundary_before(s: &Stream, p: usize) -> bool {
    if s.kind(p) == Kind::Punct {
        return matches!(s.bytes(p), b"(" | b"," | b"|" | b"&" | b"?" | b":");
    }
    if s.kind(p) == Kind::Keyword {
        return matches!(
            s.bytes(p).to_ascii_lowercase().as_slice(),
            b"public" | b"protected" | b"private" | b"readonly" | b"static" | b"var" | b"const" | b"function" | b"fn"
        );
    }
    false
}

fn fq_is_type_boundary_after(s: &Stream, end: usize) -> bool {
    let n = match sig_next(s, end) {
        Some(n) => n,
        None => return true,
    };
    if s.kind(n) == Kind::Variable {
        return true;
    }
    if s.kind(n) == Kind::Punct {
        return matches!(s.bytes(n), b"|" | b"&" | b")" | b"{" | b";" | b"," | b"...");
    }
    false
}

fn fq_type_runs_in(s: &Stream, from: usize, to: usize, u: &FqUses, ns: &[u8], reserved: &[Vec<u8>]) -> Vec<FqRepl> {
    let mut out = Vec::new();
    let mut cur = Some(from);
    while let Some(i) = cur {
        if i > to || i >= s.len() {
            break;
        }
        if fq_is_name_tok(s, i) {
            if let Some(p) = sig_prev(s, i) {
                if fq_is_type_boundary_before(s, p) {
                    if let Some((content, end)) = fq_read_run_forward(s, i) {
                        if fq_is_type_boundary_after(s, end) {
                            if let Some(repl) = fq_determine_short(&content, u, ns, reserved) {
                                out.push(FqRepl { start: i, end, tokens: fq_string_to_tokens(&repl) });
                            }
                            cur = sig_next(s, end);
                            continue;
                        }
                    }
                }
            }
        }
        cur = sig_next(s, i);
    }
    out
}

fn fq_next_punct(s: &Stream, from: usize, v: &[u8]) -> Option<usize> {
    let mut i = from + 1;
    while i < s.len() {
        if s.kind(i) == Kind::Punct && s.bytes(i) == v {
            return Some(i);
        }
        if s.kind(i) == Kind::Punct && (s.bytes(i) == b";" || s.bytes(i) == b"{") {
            return None;
        }
        i += 1;
    }
    None
}

fn fq_return_type_end(s: &Stream, colon: usize, region_end: usize) -> Option<usize> {
    let mut i = colon + 1;
    while i < s.len() && i < region_end {
        if s.kind(i) == Kind::Punct && (s.bytes(i) == b"{" || s.bytes(i) == b";") {
            return sig_prev(s, i);
        }
        i += 1;
    }
    None
}

fn fq_function(s: &Stream, kw: usize, region_end: usize, u: &FqUses, ns: &[u8], reserved: &[Vec<u8>]) -> Vec<FqRepl> {
    let open = match fq_next_punct(s, kw, b"(") {
        Some(o) => o,
        None => return Vec::new(),
    };
    let close_idx = match match_forward(s, open) {
        Some(c) => c,
        None => return Vec::new(),
    };
    let mut out = fq_type_runs_in(s, open + 1, close_idx.saturating_sub(1), u, ns, reserved);
    if let Some(c) = sig_next(s, close_idx) {
        if is_punct_val(s, c, b":") {
            if let Some(te) = fq_return_type_end(s, c, region_end) {
                if te > c {
                    if let Some(rs) = sig_next(s, c) {
                        out.extend(fq_type_runs_in(s, rs, te, u, ns, reserved));
                    }
                }
            }
        }
    }
    out
}

const FQ_PHPDOC_TAGS: &[&[u8]] = &[
    b"param", b"phpstan-param", b"phpstan-property", b"phpstan-property-read",
    b"phpstan-property-write", b"phpstan-return", b"phpstan-var", b"property",
    b"property-read", b"property-write", b"psalm-param", b"psalm-property",
    b"psalm-property-read", b"psalm-property-write", b"psalm-return", b"psalm-var",
    b"return", b"see", b"throws", b"var",
];

const FQ_DOC_KEYWORDS: &[&[u8]] = &[
    b"min", b"max", b"class-string", b"int", b"positive-int", b"negative-int",
    b"non-empty-string", b"numeric-string", b"array-key", b"scalar",
    b"non-empty-array", b"non-empty-list", b"key-of", b"value-of", b"literal-string",
    b"callable-string", b"double", b"boolean", b"integer", b"this", b"$this",
    b"lowercase-string", b"non-falsy-string", b"truthy-string",
];

fn fq_hspace(c: u8) -> bool {
    c == b' ' || c == b'\t'
}
fn fq_ws(c: u8) -> bool {
    matches!(c, b'\t' | b'\n' | 0x0c | b'\r' | b' ')
}
fn fq_tag_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_' || c == b'-'
}
fn fq_name_start(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_' || c >= 0x80
}
fn fq_name_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_' || c >= 0x80
}

// shorten each class-name atom in a type, skipping variables/keys/members.
fn fq_shorten_doc_type(type_str: &[u8], u: &FqUses, ns: &[u8], reserved: &[Vec<u8>]) -> Vec<u8> {
    // a wildcard type argument (`*`) makes the whole expression unparseable for
    // ECS's TypeExpression, which then leaves it entirely fully qualified.
    if type_str.contains(&b'*') {
        return type_str.to_vec();
    }
    let mut out: Vec<u8> = Vec::new();
    let mut i = 0;
    let n = type_str.len();
    let mut in_string: u8 = 0;
    while i < n {
        let c = type_str[i];
        if in_string != 0 {
            out.push(c);
            if c == in_string {
                in_string = 0;
            }
            i += 1;
            continue;
        }
        if c == b'\'' || c == b'"' {
            in_string = c;
            out.push(c);
            i += 1;
            continue;
        }
        let mut j = i;
        if type_str[j] == b'\\' && j + 1 < n && fq_name_start(type_str[j + 1]) {
            j += 1;
        }
        if j < n && fq_name_start(type_str[j]) {
            let start = i;
            j += 1;
            while j < n && fq_name_char(type_str[j]) {
                j += 1;
            }
            loop {
                if j < n && type_str[j] == b'\\' && j + 1 < n && fq_name_start(type_str[j + 1]) {
                    j += 1;
                    while j < n && fq_name_char(type_str[j]) {
                        j += 1;
                    }
                } else {
                    break;
                }
            }
            let atom = &type_str[start..j];
            if fq_doc_atom_shortenable(type_str, start, j, n) {
                out.extend_from_slice(&fq_map_doc_atom(atom, u, ns, reserved));
            } else {
                out.extend_from_slice(atom);
            }
            i = j;
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
}

fn fq_doc_atom_shortenable(b: &[u8], start: usize, end: usize, n: usize) -> bool {
    if start > 0 {
        let p = b[start - 1];
        if p == b'$' || p == b':' {
            return false;
        }
    }
    if end < n && b[end] == b':' && !(end + 1 < n && b[end + 1] == b':') {
        return false;
    }
    // generic base (`Foo<...>`): kept fully qualified (never over-shortens)
    if end < n && b[end] == b'<' {
        return false;
    }
    true
}

fn fq_map_doc_atom(atom: &[u8], u: &FqUses, ns: &[u8], reserved: &[Vec<u8>]) -> Vec<u8> {
    if !atom.contains(&b'\\') {
        let lower = atom.to_ascii_lowercase();
        if fq_is_reserved_type(&lower) || FQ_DOC_KEYWORDS.iter().any(|k| *k == lower.as_slice()) {
            return atom.to_vec();
        }
    }
    match fq_determine_short(atom, u, ns, reserved) {
        Some(r) => r,
        None => atom.to_vec(),
    }
}

// mirrors fqDocTagRe.ReplaceAllStringFunc for the allowed tags.
fn fq_php_doc_content(content: &[u8], u: &FqUses, ns: &[u8], reserved: &[Vec<u8>]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let n = content.len();
    let mut i = 0;
    while i < n {
        // try to match at i: [*{] [ \t]* @ tag [ \t]+ type
        if content[i] == b'*' || content[i] == b'{' {
            let mut j = i + 1;
            while j < n && fq_hspace(content[j]) {
                j += 1;
            }
            if j < n && content[j] == b'@' {
                let g1_end = j + 1;
                let mut t = g1_end;
                while t < n && fq_tag_char(content[t]) {
                    t += 1;
                }
                if t > g1_end {
                    let tag = &content[g1_end..t];
                    let mut h = t;
                    while h < n && fq_hspace(content[h]) {
                        h += 1;
                    }
                    if h > t && h < n && !fq_ws(content[h]) && content[h] != b'*' {
                        // type: [^\s*][^\s]*
                        let type_start = h;
                        let mut te = h + 1;
                        while te < n && !fq_ws(content[te]) {
                            te += 1;
                        }
                        let g1 = &content[i..g1_end];
                        let g3 = &content[t..h];
                        let g4 = &content[type_start..te];
                        out.extend_from_slice(g1);
                        out.extend_from_slice(tag);
                        out.extend_from_slice(g3);
                        if FQ_PHPDOC_TAGS.iter().any(|x| *x == tag.to_ascii_lowercase().as_slice()) {
                            out.extend_from_slice(&fq_shorten_doc_type(g4, u, ns, reserved));
                        } else {
                            out.extend_from_slice(g4);
                        }
                        i = te;
                        continue;
                    }
                }
            }
        }
        out.push(content[i]);
        i += 1;
    }
    out
}

fn fq_php_doc(s: &Stream, idx: usize, u: &FqUses, ns: &[u8], reserved: &[Vec<u8>]) -> Option<FqRepl> {
    let content = s.bytes(idx).to_vec();
    let new_content = fq_php_doc_content(&content, u, ns, reserved);
    if new_content == content {
        return None;
    }
    Some(FqRepl { start: idx, end: idx, tokens: vec![(Kind::DocComment, new_content)] })
}

// mirrors fqDocTemplateRe: collect @template/@type declared identifiers.
fn fq_doc_template_names(content: &[u8]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    let n = content.len();
    let mut i = 0;
    while i < n {
        if content[i] == b'*' {
            let mut j = i + 1;
            while j < n && fq_hspace(content[j]) {
                j += 1;
            }
            if j < n && content[j] == b'@' {
                let rest = &content[j + 1..];
                let lower = rest.to_ascii_lowercase();
                // optional psalm-/phpstan- prefix
                let mut off = 0;
                for pfx in [b"psalm-".as_slice(), b"phpstan-".as_slice()] {
                    if lower.starts_with(pfx) {
                        off = pfx.len();
                        break;
                    }
                }
                let after = &lower[off..];
                let kw = [
                    b"template-covariant".as_slice(),
                    b"template-contravariant".as_slice(),
                    b"template".as_slice(),
                    b"import-type".as_slice(),
                    b"type".as_slice(),
                ]
                .into_iter()
                .find(|k| after.starts_with(k));
                if let Some(k) = kw {
                    let mut p = j + 1 + off + k.len();
                    let mut hs = false;
                    while p < n && fq_hspace(content[p]) {
                        p += 1;
                        hs = true;
                    }
                    if hs && p < n && fq_name_start(content[p]) {
                        let st = p;
                        p += 1;
                        while p < n && fq_name_char(content[p]) {
                            p += 1;
                        }
                        out.push(content[st..p].to_vec());
                    }
                }
            }
        }
        i += 1;
    }
    out
}

fn fq_apply(s: &mut Stream, r: &FqRepl) {
    // set first slot, then remove the rest, then splice extra tokens
    // Replace [start..=end] with r.tokens
    // remove end..start+1
    let mut k = r.end;
    while k > r.start {
        s.remove_at(k);
        k -= 1;
    }
    // now index start holds the (old) first token; overwrite it with tokens[0]
    let (k0, v0) = r.tokens[0].clone();
    s.set_owned(r.start, v0);
    s.set_kind(r.start, k0);
    // insert the remaining tokens after start
    for (off, (kind, val)) in r.tokens[1..].iter().enumerate() {
        s.insert_owned(r.start + 1 + off, *kind, val.clone());
    }
}

fn fully_qualified_strict_types(s: &mut Stream) -> bool {
    let regions = fq_namespace_regions(s);
    let mut repls: Vec<FqRepl> = Vec::new();
    for (name, start, end) in &regions {
        let u = fq_collect_uses_region(s, *start, *end);
        let mut reserved: Vec<Vec<u8>> = Vec::new();
        let mut seen: Vec<usize> = Vec::new();
        let mut push = |r: FqRepl, repls: &mut Vec<FqRepl>, seen: &mut Vec<usize>| {
            if seen.contains(&r.start) {
                return;
            }
            seen.push(r.start);
            repls.push(r);
        };
        let mut depth = 0i32;
        let mut i = *start;
        while i < *end && i < s.len() {
            let k = s.kind(i);
            if k == Kind::Punct && s.bytes(i) == b"{" {
                depth += 1;
            } else if k == Kind::Punct && s.bytes(i) == b"}" {
                depth -= 1;
            } else if k == Kind::Variable {
                if let Some(p) = sig_prev(s, i) {
                    if s.kind(p) == Kind::Ident {
                        if let Some(r) = fq_prev_name(s, i, &u, name, &reserved) {
                            push(r, &mut repls, &mut seen);
                        }
                    }
                }
            } else if k == Kind::Punct && s.bytes(i) == b"::" {
                if let Some(r) = fq_prev_name(s, i, &u, name, &reserved) {
                    push(r, &mut repls, &mut seen);
                }
            } else if k == Kind::Keyword {
                match s.bytes(i).to_ascii_lowercase().as_slice() {
                    b"function" | b"fn" => {
                        for r in fq_function(s, i, *end, &u, name, &reserved) {
                            push(r, &mut repls, &mut seen);
                        }
                    }
                    b"catch" => {
                        for r in fq_catch(s, i, &u, name, &reserved) {
                            push(r, &mut repls, &mut seen);
                        }
                    }
                    b"extends" | b"implements" => {
                        for r in fq_extends_implements(s, i, &u, name, &reserved) {
                            push(r, &mut repls, &mut seen);
                        }
                    }
                    b"new" | b"instanceof" => {
                        if let Some(r) = fq_next_name(s, i, &u, name, &reserved) {
                            push(r, &mut repls, &mut seen);
                        }
                    }
                    b"use" => {
                        if depth >= 1 {
                            if let Some(r) = fq_next_name(s, i, &u, name, &reserved) {
                                push(r, &mut repls, &mut seen);
                            }
                        }
                    }
                    _ => {}
                }
            } else if k == Kind::DocComment {
                for id in fq_doc_template_names(s.bytes(i)) {
                    reserved.push(id);
                }
                if let Some(r) = fq_php_doc(s, i, &u, name, &reserved) {
                    push(r, &mut repls, &mut seen);
                }
            }
            i += 1;
        }
    }
    if repls.is_empty() {
        return false;
    }
    repls.sort_by(|a, b| b.start.cmp(&a.start));
    for r in &repls {
        fq_apply(s, r);
    }
    true
}



fn no_useless_nullsafe_operator(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = s.len();
    while i > 0 {
        i -= 1;
        if s.kind(i) != Kind::Punct || s.bytes(i) != b"?->" {
            continue;
        }
        match prev_significant_index(s, i) {
            Some(p) if s.kind(p) == Kind::Variable && s.bytes(p).eq_ignore_ascii_case(b"$this") => {
                s.set_owned(i, b"->".to_vec());
                changed = true;
            }
            _ => {}
        }
    }
    changed
}


// ---- W2: SpaceAfterCommaHereNowDoc + NoBlankLineBetweenImports ----

fn space_after_comma_here_now_doc(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = s.len();
    while i > 0 {
        i -= 1;
        if s.kind(i) != Kind::String || !s.bytes(i).starts_with(b"<<<") {
            continue;
        }
        let n = i + 1;
        if n < s.len() && s.kind(n) == Kind::Punct && (s.bytes(n) == b"," || s.bytes(n) == b"]") {
            s.insert_owned(n, Kind::Whitespace, b"\n".to_vec());
            changed = true;
        }
    }
    changed
}

fn no_blank_line_between_imports(s: &mut Stream) -> bool {
    let mut uses: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword && s.bytes(i).eq_ignore_ascii_case(b"use") && !in_class_like_body(s, i) {
            let j = skip_ws(s, i + 1);
            if !(j < s.len() && s.kind(j) == Kind::Punct && s.bytes(j) == b"(") {
                uses.push(i);
            }
        }
        i += 1;
    }
    let mut changed = false;
    let mut k = 1;
    while k < uses.len() {
        let use_index = uses[k];
        if let Some(prev) = sig_prev(s, use_index) {
            if s.kind(prev) == Kind::Punct && s.bytes(prev) == b";" && use_index >= 1 && s.kind(use_index - 1) == Kind::Whitespace {
                let ws = use_index - 1;
                if s.bytes(ws).iter().filter(|&&c| c == b'\n').count() >= 2 {
                    s.set_owned(ws, b"\n".to_vec());
                    changed = true;
                }
            }
        }
        k += 1;
    }
    changed
}

fn array_opener_and_closer_newline(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut open = s.len();
    while open > 0 {
        open -= 1;
        if s.kind(open) != Kind::Punct || s.bytes(open) != b"[" {
            continue;
        }
        if !is_array_literal_open(s, open) {
            continue;
        }
        let close_idx = match match_forward(s, open) {
            Some(c) => c,
            None => continue,
        };
        let first = match sig_next(s, open) {
            Some(f) if f != close_idx => f,
            _ => continue,
        };
        if s.kind(first) == Kind::Punct && s.bytes(first) == b"[" && is_array_literal_open(s, first) {
            continue;
        }
        if !array_has_top_level_arrow(s, open, close_idx) {
            continue;
        }
        if !(close_idx > 0 && s.kind(close_idx - 1) == Kind::Whitespace && has_newline(s.bytes(close_idx - 1))) {
            if edit_slot_before(s, close_idx, b"\n") {
                changed = true;
            }
        }
        if !(open + 1 < s.len() && s.kind(open + 1) == Kind::Whitespace && has_newline(s.bytes(open + 1))) {
            if edit_slot_after(s, open, b"\n") {
                changed = true;
            }
        }
    }
    changed
}



// ===== Symplify docblock fixers (byte-identical to the go implementations) =====

fn sdb_is_ws(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0c)
}

// @(psalm-|phpstan-)?(param|return|var) anywhere
fn sdb_has_type_annotation(v: &[u8]) -> bool {
    let mut i = 0;
    while i < v.len() {
        if v[i] == b'@' {
            let mut j = i + 1;
            if v[j..].starts_with(b"psalm-") {
                j += 6;
            } else if v[j..].starts_with(b"phpstan-") {
                j += 8;
            }
            if v[j..].starts_with(b"param") || v[j..].starts_with(b"return") || v[j..].starts_with(b"var") {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn sdb_candidate(s: &Stream) -> bool {
    let mut has_comment = false;
    for i in 0..s.len() {
        if matches!(s.kind(i), Kind::Comment | Kind::DocComment) {
            has_comment = true;
            break;
        }
    }
    if !has_comment {
        return false;
    }
    for i in 0..s.len() {
        if s.kind(i) == Kind::Keyword && s.bytes(i).eq_ignore_ascii_case(b"callable") {
            if i + 3 < s.len() && s.kind(i + 3) == Kind::Punct && s.bytes(i + 3) == b")" {
                return false;
            }
        }
    }
    for i in 0..s.len() {
        if s.kind(i) == Kind::Variable
            || (s.kind(i) == Kind::Keyword && s.bytes(i).eq_ignore_ascii_case(b"function"))
        {
            return true;
        }
    }
    false
}

fn sdb_is_empty(content: &[u8]) -> bool {
    // strip "/**", "*/", "*", and whitespace -> empty?
    let mut i = 0;
    while i < content.len() {
        if content[i..].starts_with(b"/**") {
            i += 3;
            continue;
        }
        if content[i..].starts_with(b"*/") {
            i += 2;
            continue;
        }
        if content[i] == b'*' || sdb_is_ws(content[i]) {
            i += 1;
            continue;
        }
        return false;
    }
    true
}

fn sdb_apply(s: &mut Stream, transform: fn(&[u8], &Stream, usize) -> Vec<u8>) -> bool {
    if !sdb_candidate(s) {
        return false;
    }
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        let k = s.kind(i);
        if k != Kind::Comment && k != Kind::DocComment {
            i += 1;
            continue;
        }
        if !sdb_has_type_annotation(s.bytes(i)) {
            i += 1;
            continue;
        }
        let new_content = transform(s.bytes(i), s, i);
        if new_content == s.bytes(i) {
            i += 1;
            continue;
        }
        if sdb_is_empty(&new_content) {
            let follow_ws = i + 1 < s.len() && s.kind(i + 1) == Kind::Whitespace;
            s.remove_at(i);
            if follow_ws {
                s.remove_at(i);
            }
            changed = true;
            continue;
        }
        s.set_owned(i, new_content);
        s.set_kind(i, Kind::DocComment);
        changed = true;
        i += 1;
    }
    changed
}

// find first occurrence of needle in hay from `from`
fn sdb_find(hay: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    if needle.is_empty() || from > hay.len() {
        return None;
    }
    let mut i = from;
    while i + needle.len() <= hay.len() {
        if &hay[i..i + needle.len()] == needle {
            return Some(i);
        }
        i += 1;
    }
    None
}

// --- DoubleAsteriskInlineVar ---
fn double_asterisk_inline_var_t(content: &[u8], s: &Stream, i: usize) -> Vec<u8> {
    if s.kind(i) != Kind::Comment {
        return content.to_vec();
    }
    // ^/\*(\n?\s+@(?:psalm-|phpstan-)?var)  -> /**$1
    if !content.starts_with(b"/*") {
        return content.to_vec();
    }
    let mut j = 2;
    let after_slash_star = j;
    if j < content.len() && content[j] == b'\n' {
        j += 1;
    }
    let ws_start = j;
    while j < content.len() && sdb_is_ws(content[j]) {
        j += 1;
    }
    if j == ws_start {
        return content.to_vec();
    }
    let mut t = j;
    if content[t..].starts_with(b"psalm-") {
        t += 6;
    } else if content[t..].starts_with(b"phpstan-") {
        t += 8;
    }
    if !content[t..].starts_with(b"var") {
        return content.to_vec();
    }
    // rebuild: "/**" + content[after_slash_star..]
    let mut out = b"/**".to_vec();
    out.extend_from_slice(&content[after_slash_star..]);
    out
}
fn double_asterisk_inline_var(s: &mut Stream) -> bool {
    sdb_apply(s, double_asterisk_inline_var_t)
}

// --- FixTagTypo: @(...param|return|var)s\b -> @$1 ---
fn fix_tag_typo_t(content: &[u8], _s: &Stream, _i: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(content.len());
    let mut i = 0;
    while i < content.len() {
        if content[i] == b'@' {
            let mut j = i + 1;
            let mut k = j;
            if content[k..].starts_with(b"psalm-") {
                k += 6;
            } else if content[k..].starts_with(b"phpstan-") {
                k += 8;
            }
            let tag = if content[k..].starts_with(b"param") {
                Some(5)
            } else if content[k..].starts_with(b"return") {
                Some(6)
            } else if content[k..].starts_with(b"var") {
                Some(3)
            } else {
                None
            };
            if let Some(taglen) = tag {
                let s_pos = k + taglen;
                if s_pos < content.len() && content[s_pos] == b's' {
                    let after = s_pos + 1;
                    let boundary = after >= content.len()
                        || !(content[after].is_ascii_alphanumeric() || content[after] == b'_');
                    if boundary {
                        // keep @ + [prefix]+tag, drop the trailing s
                        out.extend_from_slice(&content[i..s_pos]);
                        i = s_pos + 1;
                        j = i;
                        let _ = j;
                        continue;
                    }
                }
            }
        }
        out.push(content[i]);
        i += 1;
    }
    out
}
fn fix_tag_typo(s: &mut Stream) -> bool {
    sdb_apply(s, fix_tag_typo_t)
}

// --- TypeToVarTag: @type\b -> @var (+ single asterisk to double) ---
fn type_to_var_tag(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        let k = s.kind(i);
        if k != Kind::Comment && k != Kind::DocComment {
            i += 1;
            continue;
        }
        // has @type\b ?
        let content = s.bytes(i);
        let mut has = false;
        let mut p = 0;
        while let Some(at) = sdb_find(content, b"@type", p) {
            let after = at + 5;
            if after >= content.len() || !(content[after].is_ascii_alphanumeric() || content[after] == b'_') {
                has = true;
                break;
            }
            p = at + 1;
        }
        if !has {
            i += 1;
            continue;
        }
        // replace @type\b -> @var
        let mut nc: Vec<u8> = Vec::new();
        let mut q = 0;
        while q < content.len() {
            if content[q..].starts_with(b"@type") {
                let after = q + 5;
                if after >= content.len() || !(content[after].is_ascii_alphanumeric() || content[after] == b'_') {
                    nc.extend_from_slice(b"@var");
                    q += 5;
                    continue;
                }
            }
            nc.push(content[q]);
            q += 1;
        }
        // ^/\*(\n?\s+@var) -> /**$1
        if nc.starts_with(b"/*") && !nc.starts_with(b"/**") {
            let mut j = 2;
            if j < nc.len() && nc[j] == b'\n' {
                j += 1;
            }
            let wss = j;
            while j < nc.len() && sdb_is_ws(nc[j]) {
                j += 1;
            }
            if j > wss && nc[j..].starts_with(b"@var") {
                let mut out = b"/**".to_vec();
                out.extend_from_slice(&nc[2..]);
                nc = out;
            }
        }
        s.set_owned(i, nc);
        s.set_kind(i, Kind::DocComment);
        changed = true;
        i += 1;
    }
    changed
}

// --- MergeDocBlockStart ---
fn merge_doc_block_start(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) != Kind::Comment {
            i += 1;
            continue;
        }
        let content = s.bytes(i);
        if !content.starts_with(b"/*") || content.starts_with(b"/**") {
            i += 1;
            continue;
        }
        // must carry a phpdoc tag on a "* @" line: \n\s*\*\s*@
        if !merge_has_tag_line(content) {
            i += 1;
            continue;
        }
        let lines: Vec<&[u8]> = split_on_newline(content);
        let mut kept: Vec<Vec<u8>> = Vec::new();
        let mut removed = false;
        for (k, line) in lines.iter().enumerate() {
            if k != 0 && merge_is_empty_asterisk_line(line) {
                removed = true;
                continue;
            }
            kept.push(line.to_vec());
        }
        if !removed {
            i += 1;
            continue;
        }
        // opener /* -> /**
        if kept[0].starts_with(b"/*") && !kept[0].starts_with(b"/**") {
            let mut o = b"/**".to_vec();
            o.extend_from_slice(&kept[0][2..]);
            kept[0] = o;
        }
        let joined = join_newline(&kept);
        s.set_owned(i, joined);
        s.set_kind(i, Kind::DocComment);
        changed = true;
        i += 1;
    }
    changed
}
fn split_on_newline(v: &[u8]) -> Vec<&[u8]> {
    let mut out = Vec::new();
    let mut start = 0;
    for i in 0..v.len() {
        if v[i] == b'\n' {
            out.push(&v[start..i]);
            start = i + 1;
        }
    }
    out.push(&v[start..]);
    out
}
fn join_newline(lines: &[Vec<u8>]) -> Vec<u8> {
    let mut out = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        if i > 0 {
            out.push(b'\n');
        }
        out.extend_from_slice(l);
    }
    out
}
// ^\s*\*\s*$
fn merge_is_empty_asterisk_line(line: &[u8]) -> bool {
    let mut i = 0;
    while i < line.len() && sdb_is_ws(line[i]) {
        i += 1;
    }
    if i >= line.len() || line[i] != b'*' {
        return false;
    }
    i += 1;
    while i < line.len() && sdb_is_ws(line[i]) {
        i += 1;
    }
    i == line.len()
}
// \n\s*\*\s*@
fn merge_has_tag_line(content: &[u8]) -> bool {
    let mut i = 0;
    while i < content.len() {
        if content[i] == b'\n' {
            let mut j = i + 1;
            while j < content.len() && sdb_is_ws(content[j]) {
                j += 1;
            }
            if j < content.len() && content[j] == b'*' {
                j += 1;
                while j < content.len() && sdb_is_ws(content[j]) {
                    j += 1;
                }
                if j < content.len() && content[j] == b'@' {
                    return true;
                }
            }
        }
        i += 1;
    }
    false
}

fn is_name_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

// match a type run: [|\\\w]+  (returns end index) starting at i, or None
fn match_type_run(v: &[u8], i: usize) -> Option<usize> {
    let mut j = i;
    while j < v.len() && (v[j] == b'|' || v[j] == b'\\' || is_name_char(v[j])) {
        j += 1;
    }
    if j > i {
        Some(j)
    } else {
        None
    }
}

// match "$" + \w+ at i, return end or None
fn match_dollar_name(v: &[u8], i: usize) -> Option<usize> {
    if i >= v.len() || v[i] != b'$' {
        return None;
    }
    let mut j = i + 1;
    while j < v.len() && is_name_char(v[j]) {
        j += 1;
    }
    if j > i + 1 {
        Some(j)
    } else {
        None
    }
}

fn ws_run(v: &[u8], i: usize) -> usize {
    let mut j = i;
    while j < v.len() && sdb_is_ws(v[j]) {
        j += 1;
    }
    j
}

// --- AddMissingVarName ---
fn add_missing_next_variable(s: &Stream, i: usize) -> Option<Vec<u8>> {
    let n = sig_next(s, i)?;
    if s.kind(n) != Kind::Variable {
        return None;
    }
    if let Some(a) = sig_next(s, n) {
        if s.kind(a) == Kind::Punct && (s.bytes(a) == b"->" || s.bytes(a) == b"?->") {
            return None;
        }
    }
    Some(s.bytes(n).to_vec())
}
fn add_missing_var_name_t(content: &[u8], s: &Stream, i: usize) -> Vec<u8> {
    // ^(/\*\* @(?:psalm-|phpstan-)?var )(type)(\s+\*/)$
    let open = b"/** @";
    if !content.starts_with(open) {
        return content.to_vec();
    }
    let mut p = open.len();
    if content[p..].starts_with(b"psalm-") {
        p += 6;
    } else if content[p..].starts_with(b"phpstan-") {
        p += 8;
    }
    if !content[p..].starts_with(b"var ") {
        return content.to_vec();
    }
    p += 4;
    let open_end = p;
    // type = [\\\w|\[\]&-]+
    let type_start = p;
    while p < content.len()
        && (content[p] == b'\\'
            || content[p] == b'|'
            || content[p] == b'['
            || content[p] == b']'
            || content[p] == b'&'
            || content[p] == b'-'
            || is_name_char(content[p]))
    {
        p += 1;
    }
    if p == type_start {
        return content.to_vec();
    }
    let type_end = p;
    // (\s+\*/)$
    let close_start = p;
    let wend = ws_run(content, p);
    if wend == p {
        return content.to_vec();
    }
    if !(content[wend..] == *b"*/") {
        return content.to_vec();
    }
    let var = match add_missing_next_variable(s, i) {
        Some(v) => v,
        None => return content.to_vec(),
    };
    let mut out = content[..open_end].to_vec();
    out.extend_from_slice(&content[type_start..type_end]);
    out.push(b' ');
    out.extend_from_slice(&var);
    out.extend_from_slice(&content[close_start..]);
    out
}
fn add_missing_var_name(s: &mut Stream) -> bool {
    sdb_apply(s, add_missing_var_name_t)
}

// --- SingleLineInlineVarDocBlock ---
fn single_line_is_variable_comment(s: &Stream, i: usize) -> bool {
    let n = match sig_next(s, i) {
        Some(n) => n,
        None => return false,
    };
    let nn = match sig_next(s, n + 2) {
        Some(nn) => nn,
        None => return false,
    };
    if s.kind(nn) == Kind::Keyword {
        let lo = s.bytes(nn).to_ascii_lowercase();
        if lo == b"static" || lo == b"function" {
            return false;
        }
    }
    s.kind(n) == Kind::Variable
}
fn single_line_inline_var_doc_block_t(content: &[u8], s: &Stream, i: usize) -> Vec<u8> {
    if !single_line_is_variable_comment(s, i) {
        return content.to_vec();
    }
    if content.iter().filter(|&&c| c == b'\n').count() > 2 {
        return content.to_vec();
    }
    // ^/\*\s+\*(\s+@(?:psalm-|phpstan-)?var) -> /**$1
    let mut v = content.to_vec();
    if v.starts_with(b"/*") {
        let mut j = 2;
        let ws1 = ws_run(&v, j);
        if ws1 > j && ws1 < v.len() && v[ws1] == b'*' {
            j = ws1 + 1;
            let ws2s = j;
            let ws2 = ws_run(&v, j);
            if ws2 > ws2s {
                let mut t = ws2;
                if v[t..].starts_with(b"psalm-") {
                    t += 6;
                } else if v[t..].starts_with(b"phpstan-") {
                    t += 8;
                }
                if v[t..].starts_with(b"var") {
                    // replace "/* ... *" (up to and incl that asterisk) leaving group1 = v[ws2s-?..]
                    // group1 = "\s+@...var..." starts at ws2s (the \s+ before @). Reconstruct: /** + v[ws2s..]
                    let mut out = b"/**".to_vec();
                    out.extend_from_slice(&v[ws2s..]);
                    v = out;
                }
            }
        }
    }
    // \s+ -> ' '
    let mut collapsed: Vec<u8> = Vec::with_capacity(v.len());
    let mut k = 0;
    while k < v.len() {
        if sdb_is_ws(v[k]) {
            collapsed.push(b' ');
            k = ws_run(&v, k);
        } else {
            collapsed.push(v[k]);
            k += 1;
        }
    }
    // (\*\*)(\s+\*) -> $1   i.e. "** *" (double asterisk, ws, single) -> "**"
    let mut out: Vec<u8> = Vec::with_capacity(collapsed.len());
    let mut m = 0;
    while m < collapsed.len() {
        if collapsed[m..].starts_with(b"**") {
            let wsn = ws_run(&collapsed, m + 2);
            if wsn > m + 2 && wsn < collapsed.len() && collapsed[wsn] == b'*' {
                out.extend_from_slice(b"**");
                m = wsn + 1;
                continue;
            }
        }
        out.push(collapsed[m]);
        m += 1;
    }
    out
}
fn single_line_inline_var_doc_block(s: &mut Stream) -> bool {
    sdb_apply(s, single_line_inline_var_doc_block_t)
}

// --- RemoveSuperfluousReturnName / VarName (shared per-line tag+type+$name stripper) ---
// matches (tag)(\s+[|\\\w]+)?(\s+)(\$\w+) inside a line starting at the tag; returns
// (match_start, tag_end, type_end_or_tag_end, name) where type is optional.
struct TagVarMatch {
    start: usize,
    end: usize,   // end of whole match
    type_end: usize, // end of tag(+type) portion to keep
    name_start: usize,
    name_end: usize,
}
fn find_tag_var(line: &[u8], tag_variants: &[&[u8]]) -> Option<TagVarMatch> {
    let mut i = 0;
    while i < line.len() {
        if line[i] == b'@' {
            for tag in tag_variants {
                if line[i..].starts_with(tag) {
                    let tag_end = i + tag.len();
                    // optional (\s+[|\\\w]+)
                    let mut cur = tag_end;
                    let mut type_end = tag_end;
                    let ws1 = ws_run(line, cur);
                    if ws1 > cur {
                        if let Some(te) = match_type_run(line, ws1) {
                            cur = te;
                            type_end = te;
                        } else {
                            cur = tag_end;
                        }
                    }
                    // (\s+)
                    let ws2 = ws_run(line, cur);
                    if ws2 == cur {
                        continue;
                    }
                    // (\$\w+)
                    if let Some(ne) = match_dollar_name(line, ws2) {
                        return Some(TagVarMatch {
                            start: i,
                            end: ne,
                            type_end,
                            name_start: ws2,
                            name_end: ne,
                        });
                    }
                }
            }
        }
        i += 1;
    }
    None
}

const RETURN_TAGS: &[&[u8]] = &[b"@return", b"@psalm-return", b"@phpstan-return"];
const VAR_TAGS: &[&[u8]] = &[b"@var", b"@psalm-var", b"@phpstan-var"];

fn count_dollar_names(line: &[u8]) -> usize {
    let mut n = 0;
    let mut i = 0;
    while i < line.len() {
        if let Some(e) = match_dollar_name(line, i) {
            n += 1;
            i = e;
        } else {
            i += 1;
        }
    }
    n
}

fn remove_superfluous_return_name_t(content: &[u8], _s: &Stream, _i: usize) -> Vec<u8> {
    let lines = split_doc_lines(content);
    let mut out: Vec<u8> = Vec::new();
    for line in &lines {
        if let Some(m) = find_tag_var(line, RETURN_TAGS) {
            let name = &line[m.name_start..m.name_end];
            if name != b"$this" && count_dollar_names(line) < 2 {
                // replace [m.start..m.end] with tag(+type) = line[m.start..m.type_end]
                out.extend_from_slice(&line[..m.start]);
                out.extend_from_slice(&line[m.start..m.type_end]);
                out.extend_from_slice(&line[m.end..]);
                continue;
            }
        }
        out.extend_from_slice(line);
    }
    out
}
fn remove_superfluous_return_name(s: &mut Stream) -> bool {
    sdb_apply(s, remove_superfluous_return_name_t)
}

fn remove_superfluous_var_name_t(content: &[u8], s: &Stream, i: usize) -> Vec<u8> {
    let n = match sig_next(s, i) {
        Some(n) => n,
        None => return content.to_vec(),
    };
    if s.kind(n) != Kind::Keyword {
        return content.to_vec();
    }
    match s.bytes(n).to_ascii_lowercase().as_slice() {
        b"public" | b"protected" | b"private" | b"static" => {}
        _ => return content.to_vec(),
    }
    let lines = split_doc_lines(content);
    let mut out: Vec<u8> = Vec::new();
    for line in &lines {
        if let Some(m) = find_tag_var(line, VAR_TAGS) {
            let name = &line[m.name_start..m.name_end];
            out.extend_from_slice(&line[..m.start]);
            if name == b"$this" {
                let taglen = var_tag_len(&line[m.start..]);
                out.extend_from_slice(&line[m.start..m.start + taglen]);
                out.extend_from_slice(b" self");
            } else {
                out.extend_from_slice(&line[m.start..m.type_end]);
            }
            out.extend_from_slice(&line[m.end..]);
            continue;
        }
        out.extend_from_slice(line);
    }
    out
}
fn var_tag_len(v: &[u8]) -> usize {
    for tag in VAR_TAGS {
        if v.starts_with(tag) {
            return tag.len();
        }
    }
    0
}
fn remove_superfluous_var_name(s: &mut Stream) -> bool {
    sdb_apply(s, remove_superfluous_var_name_t)
}

// --- FixParamNameTypo ---
fn fpnt_resolve_arg_names(s: &Stream, doc_pos: usize) -> Vec<Vec<u8>> {
    let mut fnpos = None;
    let mut j = doc_pos + 1;
    while j < s.len() {
        if s.kind(j) == Kind::Keyword {
            let lo = s.bytes(j).to_ascii_lowercase();
            if lo == b"function" || lo == b"fn" {
                fnpos = Some(j);
                break;
            }
        }
        j += 1;
    }
    let fnpos = match fnpos {
        Some(f) => f,
        None => return vec![],
    };
    let open = match next_punct_of_kind(s, fnpos, &[b"("]) {
        Some(o) => o,
        None => return vec![],
    };
    let close_idx = match match_forward(s, open) {
        Some(c) => c,
        None => return vec![],
    };
    let mut names = vec![];
    let mut depth = 0i32;
    let mut i = open + 1;
    while i < close_idx {
        if s.kind(i) == Kind::Punct {
            match s.bytes(i) {
                b"(" | b"[" | b"{" => depth += 1,
                b")" | b"]" | b"}" => depth -= 1,
                _ => {}
            }
            i += 1;
            continue;
        }
        if depth == 0 && s.kind(i) == Kind::Variable {
            names.push(s.bytes(i).to_vec());
        }
        i += 1;
    }
    names
}

// per @param line, first $name (skip callable), preserving order
// go-regexp `\s` = [\t\n\f\r ] (no vertical tab); lines are already newline-split
fn fpnt_is_ws(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | 0x0C | b'\r')
}

// mirror `\*\s*@param`: a real doc-block tag line has `*` then ws then @param
fn fpnt_line_has_param_ann(line: &[u8]) -> bool {
    let mut i = 0;
    while i < line.len() {
        if line[i] == b'*' {
            let mut j = i + 1;
            while j < line.len() && fpnt_is_ws(line[j]) {
                j += 1;
            }
            if line[j..].starts_with(b"@param") {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn fpnt_param_names(content: &[u8]) -> Vec<Vec<u8>> {
    let mut out = vec![];
    for line in split_on_newline(content) {
        if !fpnt_line_has_param_ann(line) {
            continue;
        }
        if let Some(at) = sdb_find(line, b"@param", 0) {
            let after = at + 6;
            let ws = ws_run(line, after);
            if ws == after {
                continue; // need \s+
            }
            // optional callable right after ws
            if line[ws..].starts_with(b"callable") {
                let cb = ws + 8;
                let boundary = cb >= line.len() || !is_name_char(line[cb]);
                if boundary {
                    continue;
                }
            }
            // first $\w+ from ws
            let mut k = ws;
            let mut found = None;
            while k < line.len() {
                if let Some(e) = match_dollar_name(line, k) {
                    found = Some(line[k..e].to_vec());
                    break;
                }
                k += 1;
            }
            if let Some(name) = found {
                out.push(name);
            }
        }
    }
    out
}

fn fix_param_name_typo_t(content: &[u8], s: &Stream, i: usize) -> Vec<u8> {
    let argument_names = fpnt_resolve_arg_names(s, i);
    if argument_names.is_empty() {
        return content.to_vec();
    }
    let param_names_vec = fpnt_param_names(content);
    // map index -> name
    let mut param_names: std::collections::BTreeMap<usize, Vec<u8>> = std::collections::BTreeMap::new();
    for (idx, n) in param_names_vec.iter().enumerate() {
        param_names.insert(idx, n.clone());
    }
    let mut miss: std::collections::BTreeMap<usize, Vec<u8>> = std::collections::BTreeMap::new();
    for (k, arg) in argument_names.iter().enumerate() {
        let mut found = None;
        for (pk, pn) in param_names.iter() {
            if pn == arg {
                found = Some(*pk);
                break;
            }
        }
        if let Some(pk) = found {
            param_names.remove(&pk);
        } else {
            miss.insert(k, arg.clone());
        }
    }
    if miss.is_empty() || param_names.is_empty() {
        return content.to_vec();
    }
    // replaced table keyed by argument name
    let mut replaced: std::collections::HashMap<Vec<u8>, bool> = std::collections::HashMap::new();
    for a in &argument_names {
        replaced.insert(a.clone(), false);
    }
    let mut doc = content.to_vec();
    for (k, arg_name) in miss.iter() {
        let typo_name = match param_names.get(k) {
            Some(t) => t.clone(),
            None => continue,
        };
        doc = fpnt_replace(&doc, &typo_name, arg_name, &mut replaced);
    }
    doc
}

// replace every @param(.*?)(typo_name\b) applying the defer/replace callback
fn fpnt_replace(
    content: &[u8],
    typo_name: &[u8],
    arg_name: &[u8],
    replaced: &mut std::collections::HashMap<Vec<u8>, bool>,
) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let mut pos = 0;
    while pos < content.len() {
        // find next @param
        let at = match sdb_find(content, b"@param", pos) {
            Some(a) => a,
            None => break,
        };
        // lazily find first typo_name after @param with trailing word-boundary.
        // Go regexp `@param(.*?)(typo\b)` uses `.` which never crosses `\n`, so
        // the match must stay on @param's own line: bound the search to line end.
        let g1_start = at + 6;
        let mut line_end = g1_start;
        while line_end < content.len() && content[line_end] != b'\n' {
            line_end += 1;
        }
        let mut j = g1_start;
        let mut matched_name_start = None;
        while j + typo_name.len() <= line_end {
            if &content[j..j + typo_name.len()] == typo_name {
                let after = j + typo_name.len();
                let boundary = after >= content.len() || !is_name_char(content[after]);
                if boundary {
                    matched_name_start = Some(j);
                    break;
                }
            }
            j += 1;
        }
        let name_start = match matched_name_start {
            Some(n) => n,
            None => {
                // no match for this @param; emit up to end of @param and continue
                out.extend_from_slice(&content[pos..g1_start]);
                pos = g1_start;
                continue;
            }
        };
        let name_end = name_start + typo_name.len();
        // group1 = content[g1_start..name_start]
        out.extend_from_slice(&content[pos..at]);
        // decide
        let param_name = &content[name_start..name_end];
        let deferred = match replaced.get(param_name) {
            Some(false) => true,
            _ => false,
        };
        if deferred {
            replaced.insert(param_name.to_vec(), true);
            // keep original match (matched[0] = content[at..name_end])
            out.extend_from_slice(&content[at..name_end]);
        } else {
            out.extend_from_slice(b"@param");
            out.extend_from_slice(&content[g1_start..name_start]);
            out.extend_from_slice(arg_name);
        }
        pos = name_end;
    }
    out.extend_from_slice(&content[pos..]);
    out
}
fn fix_param_name_typo(s: &mut Stream) -> bool {
    sdb_apply(s, fix_param_name_typo_t)
}


// ===== Symplify docblock/annotation fixers (mirror of go implementations) =====

fn sdb_word_byte(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

fn sdb_is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

// matches @(psalm-|phpstan-)?(param|return|var) starting at i; returns true if found anywhere
fn sdb4_has_type_annotation(b: &[u8]) -> bool {
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'@' {
            let mut j = i + 1;
            if b[j..].starts_with(b"psalm-") {
                j += 6;
            } else if b[j..].starts_with(b"phpstan-") {
                j += 8;
            }
            if b[j..].starts_with(b"param") || b[j..].starts_with(b"return") || b[j..].starts_with(b"var") {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn sdb_is_empty_docblock(content: &[u8]) -> bool {
    // strip "/**", "*/", "*" then trim whitespace -> empty?
    let mut out: Vec<u8> = Vec::with_capacity(content.len());
    let mut i = 0;
    while i < content.len() {
        if content[i..].starts_with(b"/**") {
            i += 3;
        } else if content[i..].starts_with(b"*/") {
            i += 2;
        } else if content[i] == b'*' {
            i += 1;
        } else {
            out.push(content[i]);
            i += 1;
        }
    }
    out.iter().all(|&c| sdb_is_space(c))
}

fn sdb4_candidate(s: &Stream) -> bool {
    let mut has_comment = false;
    let mut has_func_or_var = false;
    let mut i = 0;
    while i < s.len() {
        match s.kind(i) {
            Kind::DocComment | Kind::Comment => has_comment = true,
            Kind::Variable => has_func_or_var = true,
            Kind::Keyword => {
                let lo = s.bytes(i).to_ascii_lowercase();
                if lo == b"function" {
                    has_func_or_var = true;
                }
                if lo == b"callable" && i + 3 < s.len() && s.bytes(i + 3) == b")" {
                    return false;
                }
            }
            _ => {}
        }
        i += 1;
    }
    has_comment && has_func_or_var
}

fn sdb4_apply<F: Fn(&[u8]) -> Vec<u8>>(s: &mut Stream, process: F) -> bool {
    if !sdb4_candidate(s) {
        return false;
    }
    let mut changed = false;
    let mut i = s.len();
    while i > 0 {
        i -= 1;
        if s.kind(i) != Kind::DocComment && s.kind(i) != Kind::Comment {
            continue;
        }
        if !sdb4_has_type_annotation(s.bytes(i)) {
            continue;
        }
        let nc = process(s.bytes(i));
        if nc == s.bytes(i) {
            continue;
        }
        if sdb_is_empty_docblock(&nc) {
            s.remove_at(i);
            if i < s.len() && s.kind(i) == Kind::Whitespace {
                s.remove_at(i);
            }
        } else {
            s.set_owned(i, nc);
            s.set_kind(i, Kind::DocComment);
        }
        changed = true;
    }
    changed
}

// split into lines each keeping trailing "\n" (matches go strings via join(""))
fn sdb_lines_keepnl(content: &[u8]) -> Vec<Vec<u8>> {
    let mut lines = Vec::new();
    let mut start = 0;
    let mut i = 0;
    while i < content.len() {
        if content[i] == b'\n' {
            lines.push(content[start..=i].to_vec());
            start = i + 1;
        }
        i += 1;
    }
    if start < content.len() {
        lines.push(content[start..].to_vec());
    }
    lines
}

// split by '\n' dropping the separators (matches go strings.Split(x,"\n"))
fn sdb_split_nl(content: &[u8]) -> Vec<Vec<u8>> {
    let mut lines = Vec::new();
    let mut start = 0;
    let mut i = 0;
    while i < content.len() {
        if content[i] == b'\n' {
            lines.push(content[start..i].to_vec());
            start = i + 1;
        }
        i += 1;
    }
    lines.push(content[start..].to_vec());
    lines
}

fn sdb_join_nl(lines: &[Vec<u8>]) -> Vec<u8> {
    let mut out = Vec::new();
    for (k, l) in lines.iter().enumerate() {
        if k > 0 {
            out.push(b'\n');
        }
        out.extend_from_slice(l);
    }
    out
}

// --- RemoveDeadParam: line matches @(psalm-|phpstan-)?param\s+$\w+\s*$ ---
fn sdb_dead_param_line(line: &[u8]) -> bool {
    // find @ + opt prefix + param + ws+ + $ + word+ + ws* then end (ignore trailing \n)
    let mut i = 0;
    while i < line.len() {
        if line[i] == b'@' {
            let mut j = i + 1;
            if line[j..].starts_with(b"psalm-") {
                j += 6;
            } else if line[j..].starts_with(b"phpstan-") {
                j += 8;
            }
            if line[j..].starts_with(b"param") {
                j += 5;
                let ws_start = j;
                while j < line.len() && sdb_is_space(line[j]) {
                    j += 1;
                }
                if j > ws_start && j < line.len() && line[j] == b'$' {
                    j += 1;
                    let name_start = j;
                    while j < line.len() && sdb_word_byte(line[j]) {
                        j += 1;
                    }
                    if j > name_start {
                        // \s* then end-of-line ($ matches before trailing \n)
                        while j < line.len() && sdb_is_space(line[j]) && line[j] != b'\n' {
                            j += 1;
                        }
                        if j == line.len() || line[j] == b'\n' {
                            return true;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    false
}

fn remove_dead_param(s: &mut Stream) -> bool {
    sdb4_apply(s, |content| {
        let mut lines = sdb_lines_keepnl(content);
        let mut ch = false;
        for l in lines.iter_mut() {
            if sdb_dead_param_line(l) {
                l.clear();
                ch = true;
            }
        }
        if !ch {
            content.to_vec()
        } else {
            lines.concat()
        }
    })
}

// --- RemoveDeadVarThis: line has @(psalm-|phpstan-)?var\b ... $this\b ---
fn sdb_dead_var_this_line(line: &[u8]) -> bool {
    let mut i = 0;
    while i < line.len() {
        if line[i] == b'@' {
            let mut j = i + 1;
            if line[j..].starts_with(b"psalm-") {
                j += 6;
            } else if line[j..].starts_with(b"phpstan-") {
                j += 8;
            }
            if line[j..].starts_with(b"var") {
                let after = j + 3;
                // \b: next char is not a word byte
                if after >= line.len() || !sdb_word_byte(line[after]) {
                    // find $this with word boundary after, in the rest of the line (no \n)
                    let mut k = after;
                    while k < line.len() && line[k] != b'\n' {
                        if line[k..].starts_with(b"$this") {
                            let end = k + 5;
                            if end >= line.len() || !sdb_word_byte(line[end]) {
                                return true;
                            }
                        }
                        k += 1;
                    }
                }
            }
        }
        i += 1;
    }
    false
}

fn remove_dead_var_this(s: &mut Stream) -> bool {
    sdb4_apply(s, |content| {
        let mut lines = sdb_lines_keepnl(content);
        let mut ch = false;
        for l in lines.iter_mut() {
            if sdb_dead_var_this_line(l) {
                l.clear();
                ch = true;
            }
        }
        if !ch {
            content.to_vec()
        } else {
            lines.concat()
        }
    })
}

// --- RemoveParamNameReference: (@param.*?)&($\w+) -> drop the & ---
fn remove_param_name_reference(s: &mut Stream) -> bool {
    sdb4_apply(s, |content| {
        let mut out: Vec<u8> = Vec::with_capacity(content.len());
        let mut i = 0;
        while i < content.len() {
            if content[i..].starts_with(b"@param") {
                // scan for first '&' followed by $\w+ before end of content (.*? no newline)
                let mut j = i + 6;
                let mut found = None;
                while j < content.len() && content[j] != b'\n' {
                    if content[j] == b'&'
                        && j + 1 < content.len()
                        && content[j + 1] == b'$'
                        && j + 2 < content.len()
                        && sdb_word_byte(content[j + 2])
                    {
                        found = Some(j);
                        break;
                    }
                    j += 1;
                }
                if let Some(amp) = found {
                    out.extend_from_slice(&content[i..amp]); // @param ...
                    // skip the '&', keep $\w+
                    i = amp + 1;
                    continue;
                }
            }
            out.push(content[i]);
            i += 1;
        }
        out
    })
}

// --- RemovePHPStormAnnotation ---
fn sdb_created_by_phpstorm_only(content: &[u8]) -> bool {
    // (?is) /\*\* \s+ \* \s+ Created by PHPStorm (.*?) \*/   ; if removing it empties content
    // content must start with /** then ws+ then * then ws+ then "Created by PHPStorm" (case-insens),
    // and end with */. Then the whole block is the annotation.
    let lo = content.to_ascii_lowercase();
    if !lo.starts_with(b"/**") || !lo.ends_with(b"*/") {
        return false;
    }
    let mut i = 3;
    let ws1 = i;
    while i < lo.len() && sdb_is_space(lo[i]) {
        i += 1;
    }
    if i == ws1 || i >= lo.len() || lo[i] != b'*' {
        return false;
    }
    i += 1;
    let ws2 = i;
    while i < lo.len() && sdb_is_space(lo[i]) {
        i += 1;
    }
    if i == ws2 {
        return false;
    }
    lo[i..].starts_with(b"created by phpstorm")
}

fn remove_phpstorm_annotation(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = s.len();
    while i > 0 {
        i -= 1;
        if s.kind(i) != Kind::DocComment && s.kind(i) != Kind::Comment {
            continue;
        }
        if sdb_created_by_phpstorm_only(s.bytes(i)) {
            s.remove_at(i);
            changed = true;
        }
    }
    changed
}

// --- SwitchedTypeAndName ---
const SDB_PRIMITIVES: &[&[u8]] = &[
    b"string", b"int", b"integer", b"float", b"bool", b"boolean", b"array", b"object",
    b"callable", b"iterable", b"mixed", b"void", b"null", b"false", b"true", b"self",
    b"static", b"parent", b"resource", b"scalar", b"never", b"number", b"double",
];

fn sdb_type_char(c: u8) -> bool {
    // [|\\\w\[\]] from the type group (excluding the <...> generic handled separately)
    c == b'|' || c == b'\\' || sdb_word_byte(c) || c == b'[' || c == b']'
}

fn sdb_known_type(t: &[u8]) -> bool {
    if t.iter().any(|&c| matches!(c, b'\\' | b'[' | b']' | b'<' | b'>' | b'|')) {
        return true;
    }
    let lo = t.to_ascii_lowercase();
    SDB_PRIMITIVES.iter().any(|p| *p == lo.as_slice())
}

// try to rewrite one line "@tag $name type rest" -> "@tag type $name rest"; returns Some(newline) or None
fn sdb_switch_line(line: &[u8]) -> Option<Vec<u8>> {
    // @((?:psalm-|phpstan-)?(?:param|var))(\s+)($\w+)(\s+)(type)(\s+\S.*)?$
    let mut i = 0;
    while i < line.len() {
        if line[i] == b'@' {
            let tag_start = i + 1;
            let mut j = tag_start;
            if line[j..].starts_with(b"psalm-") {
                j += 6;
            } else if line[j..].starts_with(b"phpstan-") {
                j += 8;
            }
            let kw = if line[j..].starts_with(b"param") {
                Some(5)
            } else if line[j..].starts_with(b"var") {
                Some(3)
            } else {
                None
            };
            if let Some(kwlen) = kw {
                j += kwlen;
                let tag_end = j;
                // (\s+)
                let ws1s = j;
                while j < line.len() && sdb_is_space(line[j]) && line[j] != b'\n' {
                    j += 1;
                }
                if j == ws1s {
                    i += 1;
                    continue;
                }
                let ws1e = j;
                // ($\w+)
                if j >= line.len() || line[j] != b'$' {
                    i += 1;
                    continue;
                }
                let name_s = j;
                j += 1;
                while j < line.len() && sdb_word_byte(line[j]) {
                    j += 1;
                }
                if j == name_s + 1 {
                    i += 1;
                    continue;
                }
                let name_e = j;
                // (\s+)
                let ws2s = j;
                while j < line.len() && sdb_is_space(line[j]) && line[j] != b'\n' {
                    j += 1;
                }
                if j == ws2s {
                    i += 1;
                    continue;
                }
                let ws2e = j;
                // (type): ((?:[|\\\w\[\]]|<[^<>]*>)+)
                let type_s = j;
                loop {
                    if j < line.len() && sdb_type_char(line[j]) {
                        j += 1;
                    } else if j < line.len() && line[j] == b'<' {
                        // <[^<>]*>
                        let mut k = j + 1;
                        while k < line.len() && line[k] != b'<' && line[k] != b'>' {
                            k += 1;
                        }
                        if k < line.len() && line[k] == b'>' {
                            j = k + 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if j == type_s {
                    i += 1;
                    continue;
                }
                let type_e = j;
                // (\s+\S.*)? then end-of-line
                let rest_s = j;
                let mut rest_e = j;
                {
                    let mut k = j;
                    let wss = k;
                    while k < line.len() && sdb_is_space(line[k]) && line[k] != b'\n' {
                        k += 1;
                    }
                    if k > wss && k < line.len() && line[k] != b'\n' {
                        // \S then .*
                        while k < line.len() && line[k] != b'\n' {
                            k += 1;
                        }
                        rest_e = k;
                    }
                }
                // must reach end-of-line
                let end = rest_e;
                if !(end == line.len() || line[end] == b'\n') {
                    i += 1;
                    continue;
                }
                let typ = &line[type_s..type_e];
                if !sdb_known_type(typ) {
                    i += 1;
                    continue;
                }
                // rebuild: prefix + @ + tag + ws1 + type + ws2 + name + rest + suffix(\n)
                let mut out: Vec<u8> = Vec::new();
                out.extend_from_slice(&line[..i]);
                out.push(b'@');
                out.extend_from_slice(&line[tag_start..tag_end]);
                out.extend_from_slice(&line[ws1s..ws1e]);
                out.extend_from_slice(typ);
                out.extend_from_slice(&line[ws2s..ws2e]);
                out.extend_from_slice(&line[name_s..name_e]);
                out.extend_from_slice(&line[rest_s..rest_e]);
                out.extend_from_slice(&line[end..]);
                return Some(out);
            }
        }
        i += 1;
    }
    None
}

fn switched_type_and_name(s: &mut Stream) -> bool {
    sdb4_apply(s, |content| {
        let mut lines = sdb_lines_keepnl(content);
        let mut ch = false;
        for l in lines.iter_mut() {
            if let Some(nl) = sdb_switch_line(l) {
                *l = nl;
                ch = true;
            }
        }
        if !ch {
            content.to_vec()
        } else {
            lines.concat()
        }
    })
}

// --- name resolvers ---
fn sdb_method_name_after(s: &Stream, comment_idx: usize) -> Option<Vec<u8>> {
    let mut i = comment_idx + 1;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword && s.bytes(i).eq_ignore_ascii_case(b"function") {
            if let Some(n) = sig_next(s, i) {
                if s.kind(n) == Kind::Ident {
                    return Some(s.bytes(n).to_vec());
                }
            }
        }
        i += 1;
    }
    None
}

fn sdb_property_name_after(s: &Stream, comment_idx: usize) -> Option<Vec<u8>> {
    let mut i = comment_idx + 1;
    while i < s.len() {
        if s.kind(i) == Kind::Variable {
            return Some(s.bytes(i).to_vec());
        }
        i += 1;
    }
    None
}

fn sdb_resolve_words(value: &[u8]) -> Vec<Vec<u8>> {
    // split camelCase (lower followed by upper), lowercase, extract [a-zA-Z]+
    let mut spaced: Vec<u8> = Vec::with_capacity(value.len());
    for (i, &c) in value.iter().enumerate() {
        if i > 0 && value[i - 1].is_ascii_lowercase() && c.is_ascii_uppercase() {
            spaced.push(b' ');
        }
        spaced.push(c.to_ascii_lowercase());
    }
    let mut words = Vec::new();
    let mut k = 0;
    while k < spaced.len() {
        if spaced[k].is_ascii_alphabetic() {
            let s0 = k;
            while k < spaced.len() && spaced[k].is_ascii_alphabetic() {
                k += 1;
            }
            words.push(spaced[s0..k].to_vec());
        } else {
            k += 1;
        }
    }
    words
}

fn sdb_stream_has_kw(s: &Stream, kw: &[u8]) -> bool {
    let mut i = 0;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword && s.bytes(i).eq_ignore_ascii_case(kw) {
            return true;
        }
        i += 1;
    }
    false
}
fn sdb_stream_has_variable(s: &Stream) -> bool {
    (0..s.len()).any(|i| s.kind(i) == Kind::Variable)
}
fn sdb_stream_has_comment(s: &Stream) -> bool {
    (0..s.len()).any(|i| matches!(s.kind(i), Kind::DocComment | Kind::Comment))
}

// --- RemovePropertyVariableNameDescription ---
fn sdb_count_var_tag(content: &[u8]) -> usize {
    let mut n = 0;
    let mut i = 0;
    while i < content.len() {
        if content[i] == b'@' {
            let mut j = i + 1;
            if content[j..].starts_with(b"psalm-") {
                j += 6;
            } else if content[j..].starts_with(b"phpstan-") {
                j += 8;
            }
            if content[j..].starts_with(b"var") {
                n += 1;
            }
        }
        i += 1;
    }
    n
}
fn sdb_line_has_var_tag(line: &[u8]) -> bool {
    let mut i = 0;
    while i < line.len() {
        if line[i] == b'@' {
            let mut j = i + 1;
            if line[j..].starts_with(b"psalm-") {
                j += 6;
            } else if line[j..].starts_with(b"phpstan-") {
                j += 8;
            }
            if line[j..].starts_with(b"var") {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn remove_property_variable_name_description(s: &mut Stream) -> bool {
    if !sdb_stream_has_variable(s) || !sdb_stream_has_comment(s) {
        return false;
    }
    let mut changed = false;
    let mut i = s.len();
    while i > 0 {
        i -= 1;
        if !matches!(s.kind(i), Kind::DocComment | Kind::Comment) {
            continue;
        }
        let prop = match sdb_property_name_after(s, i) {
            Some(p) => p,
            None => continue,
        };
        if sdb_count_var_tag(s.bytes(i)) != 1 {
            continue;
        }
        let content = s.bytes(i).to_vec();
        let mut lines = sdb_split_nl(&content);
        let mut suffix = vec![b' '];
        suffix.extend_from_slice(&prop);
        let mut ch = false;
        for l in lines.iter_mut() {
            if l.ends_with(&suffix) && sdb_line_has_var_tag(l) {
                let cut = l.len() - suffix.len();
                let mut trimmed = l[..cut].to_vec();
                while trimmed.last().map(|&c| sdb_is_space(c)).unwrap_or(false) {
                    trimmed.pop();
                }
                *l = trimmed;
                ch = true;
            }
        }
        if ch {
            s.set_owned(i, sdb_join_nl(&lines));
            s.set_kind(i, Kind::DocComment);
            changed = true;
        }
    }
    changed
}

// --- RemoveMethodNameDuplicateDescription ---
fn sdb_getter_setter_noun(v: &[u8]) -> Option<Vec<u8>> {
    for p in [b"get".as_slice(), b"set".as_slice()] {
        if v.starts_with(p) && v.len() > p.len() {
            return Some(v[p.len()..].to_vec());
        }
    }
    None
}
fn sdb_is_duplicate_desc(spaceless: &[u8], method_lower: &[u8]) -> bool {
    // desc = lower(ltrim(spaceless,'*'))
    let mut d = spaceless;
    while d.first() == Some(&b'*') {
        d = &d[1..];
    }
    let dl = d.to_ascii_lowercase();
    if dl == method_lower {
        return true;
    }
    match (sdb_getter_setter_noun(&dl), sdb_getter_setter_noun(method_lower)) {
        (Some(a), Some(b)) => a == b,
        _ => false,
    }
}
// remove articles a/an/the (word-bounded, case-insensitive), like `\b(?:a|an|the)\b` -> ""
fn sdb_drop_articles(line: &[u8]) -> Vec<u8> {
    let lo = line.to_ascii_lowercase();
    let mut out = Vec::with_capacity(line.len());
    let mut i = 0;
    while i < line.len() {
        let before_ok = i == 0 || !sdb_word_byte(lo[i - 1]);
        let mut matched = 0usize;
        if before_ok {
            for art in [b"the".as_slice(), b"an".as_slice(), b"a".as_slice()] {
                if lo[i..].starts_with(art) {
                    let end = i + art.len();
                    if end >= line.len() || !sdb_word_byte(lo[end]) {
                        matched = art.len();
                        break;
                    }
                }
            }
        }
        if matched > 0 {
            i += matched;
        } else {
            out.push(line[i]);
            i += 1;
        }
    }
    out
}

fn sdb_drop_third_person_s(line: &[u8]) -> Vec<u8> {
    // ^([\s*]*\w+?)s\b -> $1  (remove a trailing 's' of the first word)
    let mut p = 0;
    while p < line.len() && (sdb_is_space(line[p]) || line[p] == b'*') {
        p += 1;
    }
    // \w+? non-greedy then s\b: find smallest word end where next is 's' and then boundary
    let word_start = p;
    let mut k = word_start;
    while k < line.len() && sdb_word_byte(line[k]) {
        k += 1;
    }
    // word runs word_start..k; need last char 's' and preceding at least one word char
    if k > word_start + 1 && line[k - 1] == b's' {
        // boundary after 's' already holds (k is non-word or end)
        let mut out = Vec::with_capacity(line.len() - 1);
        out.extend_from_slice(&line[..k - 1]);
        out.extend_from_slice(&line[k..]);
        return out;
    }
    line.to_vec()
}
fn sdb_remove_space_nl(line: &[u8]) -> Vec<u8> {
    line.iter().copied().filter(|&c| !sdb_is_space(c)).collect()
}

fn remove_method_name_duplicate_description(s: &mut Stream) -> bool {
    if !sdb_stream_has_kw(s, b"function") || !sdb_stream_has_comment(s) {
        return false;
    }
    let mut changed = false;
    let mut i = s.len();
    while i > 0 {
        i -= 1;
        if !matches!(s.kind(i), Kind::DocComment | Kind::Comment) {
            continue;
        }
        let method = match sdb_method_name_after(s, i) {
            Some(m) => m,
            None => continue,
        };
        let method_lower = method.to_ascii_lowercase();
        let content = s.bytes(i).to_vec();
        let lines = sdb_split_nl(&content);
        let mut kept: Vec<Vec<u8>> = Vec::new();
        let mut ch = false;
        for line in &lines {
            let mut l = sdb_drop_articles(line);
            l = sdb_drop_third_person_s(&l);
            let mut spaceless = sdb_remove_space_nl(&l);
            while matches!(spaceless.last(), Some(&b'.') | Some(&b'!')) {
                spaceless.pop();
            }
            if sdb_is_duplicate_desc(&spaceless, &method_lower) {
                ch = true;
                continue;
            }
            kept.push(line.clone());
        }
        if ch {
            s.set_owned(i, sdb_join_nl(&kept));
            s.set_kind(i, Kind::DocComment);
            changed = true;
        }
    }
    changed
}

// --- RemoveEventSubscriberDescription ---
fn sdb_next_function_index(s: &Stream, comment_idx: usize) -> Option<usize> {
    let mut i = comment_idx + 1;
    while i < s.len() {
        if s.kind(i) == Kind::Keyword && s.bytes(i).eq_ignore_ascii_case(b"function") {
            return Some(i);
        }
        i += 1;
    }
    None
}
fn sdb_is_public_between(s: &Stream, comment_idx: usize, fn_idx: usize) -> bool {
    let mut i = comment_idx + 1;
    while i < fn_idx {
        if s.kind(i) == Kind::Keyword {
            let lo = s.bytes(i).to_ascii_lowercase();
            if lo == b"private" || lo == b"protected" {
                return false;
            }
        }
        i += 1;
    }
    true
}
fn sdb_is_event_desc_line(line: &[u8], method: &[u8]) -> bool {
    let words = sdb_resolve_words(line);
    let mut has_event = false;
    let mut desc: Vec<Vec<u8>> = Vec::new();
    for w in words {
        if w == b"event" {
            has_event = true;
        } else {
            desc.push(w);
        }
    }
    if !has_event || desc.is_empty() {
        return false;
    }
    let mut mwords: Vec<Vec<u8>> = sdb_resolve_words(method)
        .into_iter()
        .filter(|w| w.as_slice() != b"on")
        .collect();
    desc.sort();
    mwords.sort();
    desc == mwords
}

fn remove_event_subscriber_description(s: &mut Stream) -> bool {
    if !sdb_stream_has_kw(s, b"function") || !sdb_stream_has_comment(s) {
        return false;
    }
    let mut changed = false;
    let mut i = s.len();
    while i > 0 {
        i -= 1;
        if !matches!(s.kind(i), Kind::DocComment | Kind::Comment) {
            continue;
        }
        let fn_idx = match sdb_next_function_index(s, i) {
            Some(f) => f,
            None => continue,
        };
        if !sdb_is_public_between(s, i, fn_idx) {
            continue;
        }
        let method = match sdb_method_name_after(s, i) {
            Some(m) => m,
            None => continue,
        };
        let content = s.bytes(i).to_vec();
        let lines = sdb_split_nl(&content);
        let mut kept: Vec<Vec<u8>> = Vec::new();
        let mut ch = false;
        for line in &lines {
            if sdb_is_event_desc_line(line, &method) {
                ch = true;
                continue;
            }
            kept.push(line.clone());
        }
        if !ch {
            continue;
        }
        let joined_all: Vec<u8> = kept.concat();
        let empty = joined_all
            .iter()
            .all(|&c| matches!(c, b'/' | b'*') || sdb_is_space(c));
        if empty {
            s.remove_at(i);
        } else {
            s.set_owned(i, sdb_join_nl(&kept));
            s.set_kind(i, Kind::DocComment);
        }
        changed = true;
    }
    changed
}


// Symplify StandaloneLineInMultilineArray - associative arrays one item per line
fn standalone_array_item_info(s: &Stream, open: usize, close_idx: usize) -> (usize, bool) {
    let mut count = 1usize;
    let mut depth = 0i32;
    let mut first_arrow_checked = false;
    let mut first_is_array = false;
    let mut j = open + 1;
    while j < close_idx {
        if s.kind(j) == Kind::Punct {
            match s.bytes(j) {
                b"(" | b"[" | b"{" => depth += 1,
                b")" | b"]" | b"}" => depth -= 1,
                b"," => {
                    if depth == 0 && sig_next(s, j) != Some(close_idx) {
                        count += 1;
                    }
                }
                b"=>" => {
                    if depth == 0 && !first_arrow_checked {
                        first_arrow_checked = true;
                        if let Some(v) = sig_next(s, j) {
                            if s.kind(v) == Kind::Punct && s.bytes(v) == b"[" {
                                first_is_array = true;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        j += 1;
    }
    (count, first_is_array)
}

fn standalone_array_should_skip(s: &Stream, open: usize, close_idx: usize) -> bool {
    if !array_has_top_level_arrow(s, open, close_idx) {
        return true;
    }
    let (count, first_is_array) = standalone_array_item_info(s, open, close_idx);
    if count == 1 && !first_is_array {
        match sig_prev(s, open) {
            None => return false,
            Some(prev) => return s.bytes(prev) != b"=>",
        }
    }
    false
}

fn standalone_line_in_multiline_array(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut open = 0;
    while open < s.len() {
        if s.kind(open) == Kind::Punct && s.bytes(open) == b"[" && is_array_literal_open(s, open) {
            if let Some(close_idx) = match_forward(s, open) {
                if sig_next(s, open) != Some(close_idx) {
                    if standalone_array_should_skip(s, open, close_idx) {
                        return changed;
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


// ---- NoSuperfluousElseif (byte-identical to the ecs-go fixer) ----

fn nse_is_elseif(s: &Stream, i: usize) -> bool {
    if kw_is(s, i, b"elseif") {
        return true;
    }
    if kw_is(s, i, b"else") {
        if let Some(n) = sig_next(s, i) {
            return kw_is(s, n, b"if");
        }
    }
    false
}

// scan back before idx for the first token matching pred
fn nse_prev_of<F: Fn(&Stream, usize) -> bool>(s: &Stream, idx: usize, pred: F) -> Option<usize> {
    let mut j = idx as isize - 1;
    while j >= 0 {
        if pred(s, j as usize) {
            return Some(j as usize);
        }
        j -= 1;
    }
    None
}

fn nse_is_terminator(s: &Stream, i: usize) -> bool {
    if s.kind(i) == Kind::CloseTag {
        return true;
    }
    if is_punct_val(s, i, b";") {
        return true;
    }
    s.kind(i) == Kind::Keyword
        && matches!(
            s.bytes(i).to_ascii_lowercase().as_slice(),
            b"break" | b"continue" | b"exit" | b"die" | b"goto" | b"if" | b"return" | b"throw"
        )
}

fn nse_is_if_else_elseif(s: &Stream, i: usize) -> bool {
    s.kind(i) == Kind::Keyword
        && matches!(s.bytes(i).to_ascii_lowercase().as_slice(), b"if" | b"else" | b"elseif")
}

fn nse_is_paren_semi_colon(s: &Stream, i: usize) -> bool {
    is_punct_val(s, i, b")") || is_punct_val(s, i, b";") || is_punct_val(s, i, b":")
}

fn nse_get_previous_block(s: &Stream, index: usize) -> (Option<usize>, Option<usize>) {
    let close_idx = sig_prev(s, index);
    let mut previous = close_idx;
    if let Some(c) = close_idx {
        if is_punct_val(s, c, b"}") {
            previous = match_backward(s, c);
        }
    }
    let previous = match previous {
        Some(p) => p,
        None => return (None, close_idx),
    };
    let open = match nse_prev_of(s, previous, nse_is_if_else_elseif) {
        Some(o) => o,
        None => return (None, close_idx),
    };
    if kw_is(s, open, b"if") {
        if let Some(ec) = sig_prev(s, open) {
            if kw_is(s, ec, b"else") {
                return (Some(ec), close_idx);
            }
        }
    }
    (Some(open), close_idx)
}

fn nse_is_in_conditional(s: &Stream, index: usize, lower_limit: usize) -> bool {
    let candidate = match nse_prev_of(s, index, nse_is_paren_semi_colon) {
        Some(c) => c,
        None => return false,
    };
    if is_punct_val(s, candidate, b":") {
        return true;
    }
    if !is_punct_val(s, candidate, b")") {
        return false;
    }
    let open = match match_backward(s, candidate) {
        Some(o) => o,
        None => return false,
    };
    matches!(sig_prev(s, open), Some(p) if p > lower_limit)
}

fn nse_is_in_condition_without_braces(s: &Stream, mut index: usize, lower_limit: usize) -> bool {
    while index > lower_limit {
        let k = s.kind(index);
        if k == Kind::Comment || k == Kind::DocComment || k == Kind::Whitespace {
            index = match sig_prev(s, index) {
                Some(p) => p,
                None => return false,
            };
        }
        if s.kind(index) == Kind::Keyword {
            match s.bytes(index).to_ascii_lowercase().as_slice() {
                b"if" | b"elseif" | b"else" => return true,
                _ => {}
            }
        }
        if is_punct_val(s, index, b";") {
            return false;
        }
        if is_punct_val(s, index, b"{") {
            index = match sig_prev(s, index) {
                Some(p) => p,
                None => return false,
            };
            if kw_is(s, index, b"do") {
                if index == 0 {
                    return false;
                }
                index -= 1;
                continue;
            }
            if !is_punct_val(s, index, b")") {
                return false;
            }
            index = match match_backward(s, index) {
                Some(p) => p,
                None => return false,
            };
            index = match sig_prev(s, index) {
                Some(p) => p,
                None => return false,
            };
            if kw_is(s, index, b"if") || kw_is(s, index, b"elseif") {
                return false;
            }
        } else if is_punct_val(s, index, b")") {
            index = match match_backward(s, index) {
                Some(p) => p,
                None => return false,
            };
            index = match sig_prev(s, index) {
                Some(p) => p,
                None => return false,
            };
        } else {
            if index == 0 {
                return false;
            }
            index -= 1;
        }
    }
    false
}

fn nse_is_superfluous_else(s: &Stream, index: usize) -> bool {
    let mut previous_block_start = index;
    loop {
        let (pbs, pbe) = nse_get_previous_block(s, previous_block_start);
        let (pbs, pbe) = match (pbs, pbe) {
            (Some(a), Some(b)) => (a, b),
            _ => return false,
        };
        previous_block_start = pbs;

        let mut previous = pbe;
        if is_punct_val(s, previous, b"}") {
            previous = match sig_prev(s, previous) {
                Some(p) => p,
                None => return false,
            };
        }
        if !is_punct_val(s, previous, b";") {
            return false;
        }
        if matches!(sig_prev(s, previous), Some(p) if is_punct_val(s, p, b"{")) {
            return false;
        }

        let candidate = match nse_prev_of(s, previous, nse_is_terminator) {
            Some(c) => c,
            None => return false,
        };
        if is_punct_val(s, candidate, b";") || s.kind(candidate) == Kind::CloseTag || kw_is(s, candidate, b"if") {
            return false;
        }
        if kw_is(s, candidate, b"throw") {
            match sig_prev(s, candidate) {
                Some(pi) if is_punct_val(s, pi, b";") || is_punct_val(s, pi, b"{") => {}
                _ => return false,
            }
        }
        if nse_is_in_conditional(s, candidate, previous_block_start)
            || nse_is_in_condition_without_braces(s, candidate, previous_block_start)
        {
            return false;
        }

        if kw_is(s, previous_block_start, b"if") {
            break;
        }
    }
    true
}

fn nse_trailing_indent(ws: &[u8]) -> Option<Vec<u8>> {
    let mut last = None;
    for (i, &c) in ws.iter().enumerate() {
        if c == b'\n' || c == b'\r' {
            last = Some(i);
        }
    }
    let li = last?;
    let start = if ws[li] == b'\n' && li > 0 && ws[li - 1] == b'\r' { li - 1 } else { li };
    Some(ws[start..].to_vec())
}

fn nse_convert_elseif_to_if(s: &mut Stream, index: usize) {
    let mut index = index;
    if kw_is(s, index, b"else") {
        let before = index as isize - 1;
        let after = index + 1;
        if before >= 0
            && s.kind(before as usize) == Kind::Whitespace
            && after < s.len()
            && s.kind(after) == Kind::Whitespace
        {
            let mut merged = s.bytes(before as usize).to_vec();
            merged.extend_from_slice(s.bytes(after));
            s.set_owned(before as usize, merged);
            s.remove_at(after);
        }
        s.remove_at(index);
    } else {
        s.set_kind(index, Kind::Keyword);
        s.set_owned(index, b"if".to_vec());
    }

    let mut whitespace: Vec<u8> = Vec::new();
    let mut previous = index as isize - 1;
    while previous > 0 {
        let p = previous as usize;
        if s.kind(p) == Kind::Whitespace {
            if let Some(m) = nse_trailing_indent(s.bytes(p)) {
                whitespace = m;
                break;
            }
        }
        previous -= 1;
    }
    if whitespace.is_empty() {
        return;
    }
    if index == 0 {
        return;
    }
    let prev = index - 1;
    if s.kind(prev) != Kind::Whitespace {
        s.insert_owned(index, Kind::Whitespace, whitespace);
    } else if !s.bytes(prev).iter().any(|&c| c == b'\r' || c == b'\n') {
        s.set_owned(prev, whitespace);
    }
    let _ = &mut index;
}

fn no_superfluous_elseif(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        if nse_is_elseif(s, i) && nse_is_superfluous_else(s, i) {
            nse_convert_elseif_to_if(s, i);
            changed = true;
        }
        i += 1;
    }
    changed
}


// --- OrderedInterfaces + OrderedTraits (ClassNotation) ---

fn replace_range_owned(s: &mut Stream, start: usize, end: usize, repl: Vec<(Kind, Vec<u8>)>) {
    for _ in start..=end {
        s.remove_at(start);
    }
    let mut off = 0;
    for (k, v) in repl {
        s.insert_owned(start + off, k, v);
        off += 1;
    }
}

fn ordered_interfaces(s: &mut Stream) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < s.len() {
        let mut is_clause = false;
        if kw_is(s, i, b"implements") {
            is_clause = true;
        } else if kw_is(s, i, b"extends") {
            if let Some(name) = sig_prev(s, i) {
                if let Some(iface) = sig_prev(s, name) {
                    if kw_is(s, iface, b"interface") {
                        is_clause = true;
                    }
                }
            }
        }
        if !is_clause {
            i += 1;
            continue;
        }
        let start = i + 1;
        let brace = match next_punct_of_kind(s, start, &[b"{"]) {
            Some(b) => b,
            None => {
                i += 1;
                continue;
            }
        };
        let end = match sig_prev(s, brace) {
            Some(e) => e,
            None => {
                i += 1;
                continue;
            }
        };
        if end < start {
            i += 1;
            continue;
        }
        let groups = interface_groups(s, start, end);
        if groups.len() <= 1 {
            i += 1;
            continue;
        }
        let mut items: Vec<(Vec<(Kind, Vec<u8>)>, String, usize)> = Vec::with_capacity(groups.len());
        for (gi, g) in groups.into_iter().enumerate() {
            let key = interface_sort_key(&g);
            items.push((g, key, gi));
        }
        items.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));
        let mut reordered = false;
        for (gi, it) in items.iter().enumerate() {
            if it.2 != gi {
                reordered = true;
                break;
            }
        }
        if !reordered {
            i += 1;
            continue;
        }
        let mut repl: Vec<(Kind, Vec<u8>)> = Vec::new();
        for (gi, it) in items.iter().enumerate() {
            if gi > 0 {
                repl.push((Kind::Punct, b",".to_vec()));
            }
            repl.extend(it.0.iter().cloned());
        }
        let repl_len = repl.len();
        replace_range_owned(s, start, end, repl);
        changed = true;
        i = start + repl_len;
    }
    changed
}

fn interface_groups(s: &Stream, start: usize, end: usize) -> Vec<Vec<(Kind, Vec<u8>)>> {
    let mut groups: Vec<Vec<(Kind, Vec<u8>)>> = Vec::new();
    let mut cur: Vec<(Kind, Vec<u8>)> = Vec::new();
    let mut i = start;
    while i <= end && i < s.len() {
        if is_punct_val(s, i, b",") {
            groups.push(std::mem::take(&mut cur));
            i += 1;
            continue;
        }
        cur.push((s.kind(i), s.bytes(i).to_vec()));
        i += 1;
    }
    groups.push(cur);
    groups
}

fn interface_sort_key(group: &[(Kind, Vec<u8>)]) -> String {
    let mut b: Vec<u8> = Vec::new();
    let mut started = false;
    for (k, v) in group {
        if !started {
            if matches!(k, Kind::Whitespace | Kind::Comment | Kind::DocComment) {
                continue;
            }
            started = true;
        }
        if matches!(k, Kind::Whitespace | Kind::Comment | Kind::DocComment) {
            break;
        }
        for &c in v.iter() {
            if c == b'\\' {
                b.push(b' ');
            } else {
                b.push(c);
            }
        }
    }
    String::from_utf8_lossy(&b).into_owned()
}

fn ordered_traits(s: &mut Stream) -> bool {
    let mut groups: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut cur: Vec<(usize, usize)> = Vec::new();
    let mut i = 0;
    while i < s.len() {
        match s.kind(i) {
            Kind::Whitespace | Kind::Comment | Kind::DocComment => {
                i += 1;
                continue;
            }
            _ => {}
        }
        if !(kw_is(s, i, b"use") && is_trait_use(s, i)) {
            if !cur.is_empty() {
                groups.push(std::mem::take(&mut cur));
            }
            i += 1;
            continue;
        }
        let mut end = match next_punct_of_kind(s, i, &[b";", b"{"]) {
            Some(e) => e,
            None => break,
        };
        if is_punct_val(s, end, b"{") {
            end = match match_forward(s, end) {
                Some(e) => e,
                None => break,
            };
        }
        cur.push((i, end));
        i = end + 1;
    }
    if !cur.is_empty() {
        groups.push(cur);
    }

    let mut changed = false;
    for gi in (0..groups.len()).rev() {
        let g = groups[gi].clone();
        let mut slices: Vec<Vec<(Kind, Vec<u8>)>> = Vec::with_capacity(g.len());
        let mut names: Vec<String> = Vec::with_capacity(g.len());
        for &(st, en) in g.iter() {
            let mut toks: Vec<(Kind, Vec<u8>)> = Vec::with_capacity(en - st + 1);
            for j in st..=en {
                toks.push((s.kind(j), s.bytes(j).to_vec()));
            }
            let toks = sort_traits_in_statement(toks);
            names.push(trait_name(&toks));
            slices.push(toks);
        }
        let mut order: Vec<usize> = (0..g.len()).collect();
        order.sort_by(|&a, &b| names[a].to_lowercase().cmp(&names[b].to_lowercase()));
        let mut reordered = false;
        for k in 0..order.len() {
            if order[k] != k {
                reordered = true;
                break;
            }
        }
        if !reordered {
            let mut within = false;
            for (k, &(st, en)) in g.iter().enumerate() {
                if !same_tokens(&slices[k], s, st, en) {
                    within = true;
                    break;
                }
            }
            if !within {
                continue;
            }
        }
        for k in (0..g.len()).rev() {
            let (st, en) = g[k];
            replace_range_owned(s, st, en, slices[order[k]].clone());
        }
        changed = true;
    }
    changed
}

fn same_tokens(toks: &[(Kind, Vec<u8>)], s: &Stream, start: usize, end: usize) -> bool {
    if toks.len() != end - start + 1 {
        return false;
    }
    for (k, (kind, v)) in toks.iter().enumerate() {
        let o = start + k;
        if s.kind(o) != *kind || s.bytes(o) != v.as_slice() {
            return false;
        }
    }
    true
}

fn is_trait_use(s: &Stream, i: usize) -> bool {
    let n = match sig_next(s, i) {
        Some(n) => n,
        None => return false,
    };
    if s.kind(n) != Kind::Ident && !is_punct_val(s, n, b"\\") {
        return false;
    }
    let mut depth = 0;
    let mut j = i as isize - 1;
    while j >= 0 {
        let ju = j as usize;
        if s.kind(ju) != Kind::Punct {
            j -= 1;
            continue;
        }
        match s.bytes(ju) {
            b"}" => depth += 1,
            b"{" => {
                if depth == 0 {
                    return brace_is_classy(s, ju);
                }
                depth -= 1;
            }
            _ => {}
        }
        j -= 1;
    }
    false
}

fn brace_is_classy(s: &Stream, brace: usize) -> bool {
    let mut jopt = sig_prev(s, brace);
    while let Some(j) = jopt {
        match s.kind(j) {
            Kind::Keyword => {
                let lb = s.bytes(j).to_ascii_lowercase();
                match lb.as_slice() {
                    b"class" | b"trait" | b"enum" => return true,
                    b"extends" | b"implements" | b"abstract" | b"final" | b"readonly" => {}
                    _ => return false,
                }
            }
            Kind::Ident => {}
            Kind::Punct => {
                let v = s.bytes(j);
                if v != b"," && v != b"\\" {
                    return false;
                }
            }
            _ => return false,
        }
        jopt = sig_prev(s, j);
    }
    false
}

fn trait_name(toks: &[(Kind, Vec<u8>)]) -> String {
    let mut b: Vec<u8> = Vec::new();
    for (k, v) in toks {
        if *k == Kind::Punct && (v.as_slice() == b";" || v.as_slice() == b"{" || v.as_slice() == b",") {
            break;
        }
        if *k == Kind::Ident || (*k == Kind::Punct && v.as_slice() == b"\\") {
            b.extend_from_slice(v);
        }
    }
    let out = String::from_utf8_lossy(&b).into_owned();
    out.trim_start_matches('\\').to_string()
}

fn sort_traits_in_statement(toks: Vec<(Kind, Vec<u8>)>) -> Vec<(Kind, Vec<u8>)> {
    let mut runs: Vec<(usize, usize)> = Vec::new();
    let mut rs: isize = -1;
    for (idx, (k, v)) in toks.iter().enumerate() {
        let is_name = *k == Kind::Ident || (*k == Kind::Punct && v.as_slice() == b"\\");
        if is_name {
            if rs < 0 {
                rs = idx as isize;
            }
            continue;
        }
        if *k == Kind::Punct && (v.as_slice() == b"," || v.as_slice() == b";" || v.as_slice() == b"{") {
            if rs >= 0 {
                runs.push((rs as usize, idx - 1));
                rs = -1;
            }
            if v.as_slice() == b"{" {
                break;
            }
        }
    }
    if runs.len() <= 1 {
        return toks;
    }
    let mut keys: Vec<String> = Vec::with_capacity(runs.len());
    for &(a, b) in runs.iter() {
        let mut buf: Vec<u8> = Vec::new();
        for j in a..=b {
            buf.extend_from_slice(&toks[j].1);
        }
        let skey = String::from_utf8_lossy(&buf).into_owned();
        keys.push(skey.trim_start_matches('\\').to_string());
    }
    let mut order: Vec<usize> = (0..runs.len()).collect();
    order.sort_by(|&a, &b| keys[a].to_lowercase().cmp(&keys[b].to_lowercase()));
    let mut same = true;
    for k in 0..order.len() {
        if order[k] != k {
            same = false;
            break;
        }
    }
    if same {
        return toks;
    }
    let mut res: Vec<(Kind, Vec<u8>)> = Vec::new();
    let mut prev = 0usize;
    for (k, &(rstart, _rend)) in runs.iter().enumerate() {
        res.extend_from_slice(&toks[prev..rstart]);
        let (sstart, send) = runs[order[k]];
        res.extend_from_slice(&toks[sstart..=send]);
        prev = runs[k].1 + 1;
    }
    res.extend_from_slice(&toks[prev..]);
    res
}


// ===== PHPUnit fixers (byte-identical to the go implementations) =====

fn ends_with_test_name(name: &[u8], iface: bool) -> bool {
    if name.ends_with(b"Test") || name.ends_with(b"TestCase") {
        return true;
    }
    if iface && (name.ends_with(b"TestInterface") || name.ends_with(b"TestCaseInterface")) {
        return true;
    }
    false
}

fn is_phpunit_class(s: &Stream, index: usize) -> bool {
    let mut extends = false;
    let mut j = index + 1;
    while j < s.len() {
        if is_punct_val(s, j, b"{") {
            break;
        }
        if kw_is(s, j, b"extends") {
            extends = true;
            break;
        }
        j += 1;
    }
    if !extends {
        return false;
    }
    if let Some(name) = sig_next(s, index) {
        if s.kind(name) == Kind::Ident && ends_with_test_name(s.bytes(name), false) {
            return true;
        }
    }
    let mut j = index + 1;
    while j < s.len() {
        if is_punct_val(s, j, b"{") {
            break;
        }
        if s.kind(j) == Kind::Ident && ends_with_test_name(s.bytes(j), true) {
            return true;
        }
        j += 1;
    }
    false
}

fn phpunit_test_classes(s: &Stream) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if kw_is(s, i, b"class") && is_phpunit_class(s, i) {
            if let Some(open) = next_punct_of_kind(s, i, &[b"{"]) {
                if let Some(end) = match_forward(s, open) {
                    out.push((open, end));
                }
            }
        }
        i += 1;
    }
    out
}

fn pu_is_method(s: &Stream, index: usize) -> bool {
    if !kw_is(s, index, b"function") {
        return false;
    }
    matches!(sig_next(s, index), Some(n) if s.kind(n) == Kind::Ident)
}

fn pu_is_modifier_kw(v: &[u8]) -> bool {
    matches!(
        v.to_ascii_lowercase().as_slice(),
        b"public" | b"protected" | b"private" | b"final" | b"abstract" | b"readonly" | b"static"
    )
}

fn pu_doc_block_index(s: &Stream, index: usize) -> Option<usize> {
    let mut idx = index;
    loop {
        let p = get_prev_non_whitespace(s, idx)?;
        match s.kind(p) {
            Kind::Comment => {
                idx = p;
                continue;
            }
            Kind::Keyword if pu_is_modifier_kw(s.bytes(p)) => {
                idx = p;
                continue;
            }
            _ => return Some(p),
        }
    }
}

fn camel_case_to_underscore(str_: &[u8]) -> Vec<u8> {
    let is_upper = |c: u8| c.is_ascii_uppercase();
    let mut out = Vec::with_capacity(str_.len() + 4);
    for i in 0..str_.len() {
        let c = str_[i];
        if i > 0 && str_[i - 1] != b'_' {
            let has_next = i + 1 < str_.len();
            let next = if has_next { str_[i + 1] } else { 0 };
            let cond_a = is_upper(c) && has_next && !is_upper(next);
            let cond_b = !is_upper(str_[i - 1]) && is_upper(c);
            if cond_a || cond_b {
                out.push(b'_');
            }
        }
        out.push(c);
    }
    out.make_ascii_lowercase();
    out
}

fn pu_test_attr(v: &[u8]) -> bool {
    if !v.starts_with(b"#[") {
        return false;
    }
    let inner = &v[2..v.len().saturating_sub(1).max(2)];
    for part in inner.split(|&c| c == b',') {
        let mut part = part;
        if let Some(pos) = part.iter().position(|&c| c == b'(') {
            part = &part[..pos];
        }
        let part = trim_ascii(part);
        let short = match part.iter().rposition(|&c| c == b'\\') {
            Some(p) => &part[p + 1..],
            None => part,
        };
        if short == b"Test" {
            return true;
        }
    }
    false
}

fn is_test_attribute_present(s: &Stream, index: usize) -> bool {
    let mut p = get_prev_non_whitespace(s, index);
    while let Some(pi) = p {
        match s.kind(pi) {
            Kind::Comment => {
                if pu_test_attr(s.bytes(pi)) {
                    return true;
                }
                p = get_prev_non_whitespace(s, pi);
            }
            Kind::Keyword if pu_is_modifier_kw(s.bytes(pi)) => {
                p = get_prev_non_whitespace(s, pi);
            }
            _ => break,
        }
    }
    false
}

fn is_test_method(s: &Stream, index: usize) -> bool {
    if !pu_is_method(s, index) {
        return false;
    }
    let name_idx = match sig_next(s, index) {
        Some(n) => n,
        None => return false,
    };
    if s.bytes(name_idx).starts_with(b"test") {
        return true;
    }
    if is_test_attribute_present(s, index) {
        return true;
    }
    match pu_doc_block_index(s, index) {
        Some(d) if s.kind(d) == Kind::DocComment => {
            windows_contains(s.bytes(d), b"@test")
        }
        _ => false,
    }
}

fn windows_contains(hay: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || needle.len() > hay.len() {
        return needle.is_empty();
    }
    hay.windows(needle.len()).any(|w| w == needle)
}

fn update_depends_doc(s: &mut Stream, doc_idx: usize) -> bool {
    let content = s.bytes(doc_idx).to_vec();
    if !windows_contains(&content, b"@depends") {
        return false;
    }
    let mut changed = false;
    let mut out: Vec<u8> = Vec::with_capacity(content.len());
    for (li, line) in split_keep(&content, b'\n').iter().enumerate() {
        if li > 0 {
            out.push(b'\n');
        }
        if let Some(idx) = find_sub(line, b"@depends") {
            let after = idx + b"@depends".len();
            let mut j = after;
            while j < line.len() && (line[j] == b' ' || line[j] == b'\t') {
                j += 1;
            }
            let mut k = j;
            while k < line.len()
                && line[k] != b' '
                && line[k] != b'\t'
                && line[k] != b'\r'
                && line[k] != b'\n'
            {
                k += 1;
            }
            let refname = &line[j..k];
            if !refname.is_empty() {
                let newref = camel_case_to_underscore(refname);
                if newref != refname {
                    out.extend_from_slice(&line[..j]);
                    out.extend_from_slice(&newref);
                    out.extend_from_slice(&line[k..]);
                    changed = true;
                    continue;
                }
            }
        }
        out.extend_from_slice(line);
    }
    if changed {
        s.set_owned(doc_idx, out);
    }
    changed
}

fn split_keep(content: &[u8], sep: u8) -> Vec<Vec<u8>> {
    content.split(|&c| c == sep).map(|p| p.to_vec()).collect()
}

fn find_sub(hay: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > hay.len() {
        return None;
    }
    hay.windows(needle.len()).position(|w| w == needle)
}

fn method_casing_class(s: &mut Stream, start: usize, end: usize) -> bool {
    let mut changed = false;
    let mut existing: std::collections::HashSet<Vec<u8>> = std::collections::HashSet::new();
    let mut i = end;
    while i > start + 1 {
        i -= 1;
        if !pu_is_method(s, i) {
            continue;
        }
        if let Some(n) = sig_next(s, i) {
            existing.insert(s.bytes(n).to_ascii_lowercase());
        }
    }
    let mut i = end;
    while i > start + 1 {
        i -= 1;
        if !is_test_method(s, i) {
            continue;
        }
        let name_idx = match sig_next(s, i) {
            Some(n) => n,
            None => continue,
        };
        let name = s.bytes(name_idx).to_vec();
        let name_lower = name.to_ascii_lowercase();
        let new_name = camel_case_to_underscore(&name);
        let new_lower = new_name.to_ascii_lowercase();
        if existing.contains(&new_lower) && name_lower != new_lower {
            continue;
        }
        existing.insert(new_lower);
        if new_name != name {
            s.set_owned(name_idx, new_name);
            changed = true;
        }
        if let Some(d) = pu_doc_block_index(s, i) {
            if s.kind(d) == Kind::DocComment && update_depends_doc(s, d) {
                changed = true;
            }
        }
    }
    changed
}

fn phpunit_method_casing(s: &mut Stream) -> bool {
    let mut changed = false;
    for (start, end) in phpunit_test_classes(s) {
        if method_casing_class(s, start, end) {
            changed = true;
        }
    }
    changed
}

fn method_visibility_index(s: &Stream, fn_idx: usize) -> Option<usize> {
    let mut p = get_prev_non_whitespace(s, fn_idx);
    while let Some(pi) = p {
        if s.kind(pi) == Kind::Keyword {
            match s.bytes(pi).to_ascii_lowercase().as_slice() {
                b"public" | b"protected" | b"private" => return Some(pi),
                b"final" | b"abstract" | b"readonly" | b"static" => {
                    p = get_prev_non_whitespace(s, pi);
                    continue;
                }
                _ => break,
            }
        }
        if s.kind(pi) == Kind::Comment {
            p = get_prev_non_whitespace(s, pi);
            continue;
        }
        break;
    }
    None
}

fn setup_teardown_class(s: &mut Stream, start: usize, end: usize) -> bool {
    let mut changed = false;
    let mut counter = 0;
    let mut end = end;
    let mut i = start + 1;
    while i < end && i < s.len() {
        if counter == 2 {
            break;
        }
        if is_punct_val(s, i, b"{") {
            if let Some(e) = match_forward(s, i) {
                if e > i {
                    i = e;
                }
            }
            i += 1;
            continue;
        }
        if !kw_is(s, i, b"function") {
            i += 1;
            continue;
        }
        let name_idx = match sig_next(s, i) {
            Some(n) => n,
            None => {
                i += 1;
                continue;
            }
        };
        let fnl = s.bytes(name_idx).to_ascii_lowercase();
        if fnl != b"setup" && fnl != b"teardown" {
            i += 1;
            continue;
        }
        counter += 1;
        if let Some(vis) = method_visibility_index(s, i) {
            if s.bytes(vis).eq_ignore_ascii_case(b"public") {
                s.set_owned(vis, b"protected".to_vec());
                changed = true;
            }
            i += 1;
            continue;
        }
        s.insert_owned(i, Kind::Whitespace, b" ".to_vec());
        s.insert_owned(i, Kind::Keyword, b"protected".to_vec());
        changed = true;
        end += 2;
        i += 1;
    }
    changed
}

fn phpunit_setup_teardown_visibility(s: &mut Stream) -> bool {
    let mut changed = false;
    for (start, end) in phpunit_test_classes(s) {
        if setup_teardown_class(s, start, end) {
            changed = true;
        }
    }
    changed
}
