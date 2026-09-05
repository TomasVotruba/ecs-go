package rules

import (
	"strings"

	"ecs-go/internal/token"
	"ecs-go/internal/tokens"
)

// NoSpaceBeforeSemicolon removes a single-line whitespace token sitting directly
// before a semicolon (e.g. "$x = 1 ;" becomes "$x = 1;"). This is the rule that
// truly exercises the token stream: it deletes tokens by index.
type NoSpaceBeforeSemicolon struct{}

func (NoSpaceBeforeSemicolon) Name() string { return "no_space_before_semicolon" }

func (NoSpaceBeforeSemicolon) Fix(s *tokens.Stream) bool {
	changed := false
	// walk backwards so removals do not shift indices we still need
	for i := s.Len() - 1; i >= 1; i-- {
		t := s.At(i)
		if t.Kind != token.Punct || t.Value != ";" {
			continue
		}
		prev := s.At(i - 1)
		// keep whitespace that spans lines (multi-line statement layout)
		if prev.Kind == token.Whitespace && !strings.ContainsAny(prev.Value, "\n\r") {
			s.RemoveAt(i - 1)
			changed = true
		}
	}
	return changed
}
