package rules

import (
	"regexp"

	"ecs-go/internal/token"
	"ecs-go/internal/tokens"
)

var threeOrMoreNewlines = regexp.MustCompile(`\n{3,}`)

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Whitespace/NoExtraBlankLinesFixer.php
//
// NoExtraBlankLines collapses two or more consecutive blank lines into one. It
// runs after no_whitespace_in_blank_line, so blank lines are already bare.
type NoExtraBlankLines struct{}

func (NoExtraBlankLines) Name() string {
	return `PhpCsFixer\Fixer\Whitespace\NoExtraBlankLinesFixer`
}

func (NoExtraBlankLines) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Whitespace/NoExtraBlankLinesFixer.php"
}

func (NoExtraBlankLines) Fix(s *tokens.Stream) bool {
	changed := false
	for i := range s.Len() {
		t := s.At(i)
		if t.Kind != token.Whitespace {
			continue
		}
		if v := threeOrMoreNewlines.ReplaceAllString(t.Value, "\n\n"); v != t.Value {
			s.SetValue(i, v)
			changed = true
		}
	}
	return changed
}
