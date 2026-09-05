package rules

import (
	"strings"

	"ecs-go/internal/token"
	"ecs-go/internal/tokens"
)

// BlankLineAfterOpeningTag ensures one blank line after the opening "<?php" tag
// when code starts on a following line: "<?php\n$x" becomes "<?php\n\n$x". Code
// on the same line as the tag is left untouched.
type BlankLineAfterOpeningTag struct{}

func (BlankLineAfterOpeningTag) Name() string {
	return `PhpCsFixer\Fixer\PhpTag\BlankLineAfterOpeningTagFixer`
}

func (BlankLineAfterOpeningTag) Fix(s *tokens.Stream) bool {
	if s.Len() < 3 {
		return false
	}
	if s.At(0).Kind != token.OpenTag || s.At(0).Value != "<?php" {
		return false
	}
	ws := s.At(1)
	if ws.Kind != token.Whitespace {
		return false
	}
	if !strings.HasPrefix(ws.Value, "\n") || strings.HasPrefix(ws.Value, "\n\n") {
		return false
	}
	s.SetValue(1, "\n"+ws.Value)
	return true
}
