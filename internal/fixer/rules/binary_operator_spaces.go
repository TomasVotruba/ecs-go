package rules

import (
	"strings"

	"ecs-go/internal/token"
	"ecs-go/internal/tokens"
)

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Operator/BinaryOperatorSpacesFixer.php
//
// BinaryOperatorSpaces normalizes spacing around the "=>" arrow to a single
// space: "['a'=>1]" becomes "['a' => 1]". Whitespace spanning a newline is left
// alone to preserve intentional alignment. This is the first slice of the
// upstream fixer, unlocked by multi-char operator tokenization ("=>" is now one
// token); more operators follow.
type BinaryOperatorSpaces struct{}

func (BinaryOperatorSpaces) Name() string {
	return `PhpCsFixer\Fixer\Operator\BinaryOperatorSpacesFixer`
}

func (BinaryOperatorSpaces) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Operator/BinaryOperatorSpacesFixer.php"
}

func (BinaryOperatorSpaces) Fix(s *tokens.Stream) bool {
	changed := false
	i := 0
	for i < s.Len() {
		t := s.At(i)
		if t.Kind != token.Punct || t.Value != "=>" {
			i++
			continue
		}

		// single space before the arrow
		if i > 0 {
			prev := s.At(i - 1)
			if prev.Kind == token.Whitespace {
				if !hasNewline(prev.Value) && prev.Value != " " {
					s.SetValue(i-1, " ")
					changed = true
				}
			} else {
				s.InsertAt(i, token.Token{Kind: token.Whitespace, Value: " "})
				i++ // arrow shifted right by the inserted space
				changed = true
			}
		}

		// single space after the arrow
		if i+1 < s.Len() {
			next := s.At(i + 1)
			if next.Kind == token.Whitespace {
				if !hasNewline(next.Value) && next.Value != " " {
					s.SetValue(i+1, " ")
					changed = true
				}
			} else {
				s.InsertAt(i+1, token.Token{Kind: token.Whitespace, Value: " "})
				changed = true
			}
		}
		i++
	}
	return changed
}

func hasNewline(s string) bool { return strings.ContainsAny(s, "\n\r") }
