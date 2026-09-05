package rules

import (
	"ecs-go/internal/token"
	"ecs-go/internal/tokens"
)

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Operator/BinaryOperatorSpacesFixer.php
//
// BinaryOperatorSpaces normalizes spacing to a single space around the "="
// assignment and "=>" arrow operators, matching the ECS spaces set config.
// Whitespace spanning a newline is left alone to preserve alignment. Reference
// assignment ("=& $x") is skipped.
type BinaryOperatorSpaces struct{}

func (BinaryOperatorSpaces) Name() string {
	return `PhpCsFixer\Fixer\Operator\BinaryOperatorSpacesFixer`
}

func (BinaryOperatorSpaces) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Operator/BinaryOperatorSpacesFixer.php"
}

func (BinaryOperatorSpaces) Fix(s *tokens.Stream) bool {
	return normalizeSpaceAround(s, func(s *tokens.Stream, i int) bool {
		t := s.At(i)
		if t.Kind != token.Punct {
			return false
		}
		switch t.Value {
		case "=>":
			return true
		case "=":
			// leave reference assignment ("=& $x") untouched
			return nextSignificantValue(s, i) != "&"
		default:
			return false
		}
	})
}
