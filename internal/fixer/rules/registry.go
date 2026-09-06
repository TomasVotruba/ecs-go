package rules

import "ecs-go/internal/fixer"

// SpacingFixers are the whitespace/operator spacing rules, safest first. This is
// the ordered "spaces" set that gradual levels slice.
func SpacingFixers() []fixer.Fixer {
	return []fixer.Fixer{
		NoLeadingNamespaceWhitespace{},
		NoSinglelineWhitespaceBeforeSemicolons{},
		NoWhitespaceInBlankLine{},
		NoExtraBlankLines{},
		SpaceAfterSemicolon{},
		BinaryOperatorSpaces{},
		TernaryOperatorSpaces{},
		ConcatSpace{},
		CastSpaces{},
		BlankLineAfterOpeningTag{},
		NoTrailingWhitespace{},
		NoTrailingWhitespaceInComment{},
		SingleBlankLineAtEndOfFile{},
	}
}

// CasingFixers normalize keyword, constant and cast casing.
func CasingFixers() []fixer.Fixer {
	return []fixer.Fixer{
		LowercaseKeywords{},
		ConstantCase{},
		LowercaseStaticReference{},
		LowercaseCast{},
		ShortScalarCast{},
	}
}

// ConstructFixers cover keyword/parenthesis/operator spacing and import cleanups
// from the PSR-12 set.
func ConstructFixers() []fixer.Fixer {
	return []fixer.Fixer{
		DeclareEqualNormalize{},
		SingleSpaceAroundConstruct{},
		NoSpacesAfterFunctionName{},
		NoSpacesInsideParenthesis{},
		UnaryOperatorSpaces{},
		NoLeadingImportSlash{},
		Elseif{},
		SwitchCaseSemicolonToColon{},
		SwitchCaseSpace{},
		NoMultipleStatementsPerLine{},
		MethodArgumentSpace{},
		ReturnTypeDeclaration{},
	}
}

// StructuralFixers reflow imports, namespace/class blank lines and indentation.
func StructuralFixers() []fixer.Fixer {
	return []fixer.Fixer{
		IndentationType{},
		ClassDefinition{},
		BracesPosition{},
		VisibilityRequired{},
		SingleTraitInsertPerStatement{},
		SingleClassElementPerStatement{},
		OrderedClassElements{},
		BlankLinesBeforeNamespace{},
		BlankLineAfterNamespace{},
		SingleImportPerStatement{},
		OrderedImports{},
		BlankLineBetweenImportGroups{},
		SingleLineAfterImports{},
		NoBlankLinesAfterClassOpening{},
		StatementIndentation{},
	}
}

// All returns every built-in fixer in execution order. FullOpeningTag runs first
// (normalize the tag) and NoClosingTag last (trailing tag/EOF cleanup).
func All() []fixer.Fixer {
	all := []fixer.Fixer{FullOpeningTag{}}
	all = append(all, CasingFixers()...)
	all = append(all, SpacingFixers()...)
	all = append(all, ConstructFixers()...)
	all = append(all, StructuralFixers()...)
	all = append(all, NoClosingTag{})
	return all
}

// ByName returns the fixer whose Name matches, if any.
func ByName(name string) (fixer.Fixer, bool) {
	for _, f := range All() {
		if f.Name() == name {
			return f, true
		}
	}
	return nil, false
}
