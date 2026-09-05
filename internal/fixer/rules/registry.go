package rules

import "ecs-go/internal/fixer"

// All returns every built-in fixer, in a stable, deterministic order.
func All() []fixer.Fixer {
	return []fixer.Fixer{
		NoSpaceBeforeSemicolon{},
		NoTrailingWhitespace{},
		SingleBlankLineAtEndOfFile{},
	}
}
