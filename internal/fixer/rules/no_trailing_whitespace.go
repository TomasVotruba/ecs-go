package rules

import (
	"regexp"

	"ecs-go/internal/token"
	"ecs-go/internal/tokens"
)

var (
	trailingBeforeNewline = regexp.MustCompile(`[ \t]+(\r?\n)`)
	trailingAtEnd         = regexp.MustCompile(`[ \t]+$`)
)

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Whitespace/NoTrailingWhitespaceFixer.php
//
// NoTrailingWhitespace removes spaces and tabs at the end of a line.
type NoTrailingWhitespace struct{}

func (NoTrailingWhitespace) Name() string {
	return `PhpCsFixer\Fixer\Whitespace\NoTrailingWhitespaceFixer`
}

func (NoTrailingWhitespace) Fix(s *tokens.Stream) bool {
	changed := false
	last := s.Len() - 1
	for i := 0; i < s.Len(); i++ {
		t := s.At(i)
		if t.Kind != token.Whitespace {
			continue
		}
		v := trailingBeforeNewline.ReplaceAllString(t.Value, "$1")
		if i == last {
			// trailing spaces/tabs at end of file (no newline after)
			v = trailingAtEnd.ReplaceAllString(v, "")
		}
		if v != t.Value {
			s.SetValue(i, v)
			changed = true
		}
	}
	return changed
}
