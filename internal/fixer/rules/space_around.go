package rules

import (
	"strings"

	"ecs-go/internal/token"
	"ecs-go/internal/tokens"
)

func hasNewline(s string) bool { return strings.ContainsAny(s, "\n\r") }

// nextSignificantValue returns the value of the first non-whitespace token after
// index i, or "" if none.
func nextSignificantValue(s *tokens.Stream, i int) string {
	for j := i + 1; j < s.Len(); j++ {
		if s.At(j).Kind != token.Whitespace {
			return s.At(j).Value
		}
	}
	return ""
}

// prevSignificant returns the first non-whitespace token before index i.
func prevSignificant(s *tokens.Stream, i int) (token.Token, bool) {
	for j := i - 1; j >= 0; j-- {
		if s.At(j).Kind != token.Whitespace {
			return s.At(j), true
		}
	}
	return token.Token{}, false
}

// normalizeSpaceAround forces exactly one space on both sides of every token the
// target predicate selects. Whitespace spanning a newline is left alone so
// intentional alignment survives.
func normalizeSpaceAround(s *tokens.Stream, target func(*tokens.Stream, int) bool) bool {
	changed := false
	i := 0
	for i < s.Len() {
		if !target(s, i) {
			i++
			continue
		}

		if i > 0 {
			prev := s.At(i - 1)
			if prev.Kind == token.Whitespace {
				if !hasNewline(prev.Value) && prev.Value != " " {
					s.SetValue(i-1, " ")
					changed = true
				}
			} else {
				s.InsertAt(i, token.Token{Kind: token.Whitespace, Value: " "})
				i++ // operator shifted right by the inserted space
				changed = true
			}
		}

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
