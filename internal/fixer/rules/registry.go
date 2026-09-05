package rules

import "ecs-go/internal/fixer"

// ByName returns the fixer whose Name matches, if any.
func ByName(name string) (fixer.Fixer, bool) {
	for _, f := range All() {
		if f.Name() == name {
			return f, true
		}
	}
	return nil, false
}

// All returns every built-in fixer, safest first, echoing the ordering of
// ECS's SpacesLevel set.
func All() []fixer.Fixer {
	return []fixer.Fixer{
		NoLeadingNamespaceWhitespace{},
		NoSinglelineWhitespaceBeforeSemicolons{},
		NoWhitespaceInBlankLine{},
		SpaceAfterSemicolon{},
		BinaryOperatorSpaces{},
		ConcatSpace{},
		CastSpaces{},
		BlankLineAfterOpeningTag{},
		NoTrailingWhitespace{},
		SingleBlankLineAtEndOfFile{},
	}
}
