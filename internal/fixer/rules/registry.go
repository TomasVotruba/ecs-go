package rules

import "ecs-go/internal/fixer"

// All returns every built-in fixer, safest first, echoing the ordering of
// ECS's SpacesLevel set.
func All() []fixer.Fixer {
	return []fixer.Fixer{
		NoLeadingNamespaceWhitespace{},
		NoSinglelineWhitespaceBeforeSemicolons{},
		NoWhitespaceInBlankLine{},
		SpaceAfterSemicolon{},
		BlankLineAfterOpeningTag{},
		NoTrailingWhitespace{},
		SingleBlankLineAtEndOfFile{},
	}
}
