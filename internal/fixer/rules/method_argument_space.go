package rules

import (
	"ecs-go/internal/token"
	"ecs-go/internal/tokens"
)

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/FunctionNotation/MethodArgumentSpaceFixer.php
//
// MethodArgumentSpace normalizes single-line spacing around commas inside
// parentheses (calls and signatures): no space before a comma, exactly one
// space after it. Commas inside arrays "[...]", commas followed by a newline
// (multiline alignment) and a trailing comma right before ")" are left alone.
type MethodArgumentSpace struct{}

func (MethodArgumentSpace) Name() string {
	return `PhpCsFixer\Fixer\FunctionNotation\MethodArgumentSpaceFixer`
}

func (MethodArgumentSpace) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/FunctionNotation/MethodArgumentSpaceFixer.php"
}

func (MethodArgumentSpace) Fix(s *tokens.Stream) bool {
	changed := false
	var stack []string
	for i := 0; i < s.Len(); i++ {
		t := s.At(i)
		if t.Kind != token.Punct {
			continue
		}
		switch t.Value {
		case "(", "[", "{":
			stack = append(stack, t.Value)
			continue
		case ")", "]", "}":
			if len(stack) > 0 {
				stack = stack[:len(stack)-1]
			}
			continue
		case ",":
			if len(stack) == 0 || stack[len(stack)-1] != "(" {
				continue
			}
			// Multiline arg list: leave alignment untouched.
			if i+1 < s.Len() && s.At(i+1).Kind == token.Whitespace && hasNewline(s.At(i+1).Value) {
				continue
			}
			// No space before the comma (single-line only).
			if i > 0 && s.At(i-1).Kind == token.Whitespace && !hasNewline(s.At(i-1).Value) {
				s.RemoveAt(i - 1)
				i--
				changed = true
			}
			// Exactly one space after the comma, except a trailing comma before ")".
			if i+1 < s.Len() {
				next := s.At(i + 1)
				if next.Kind == token.Whitespace {
					if next.Value != " " {
						s.SetValue(i+1, " ")
						changed = true
					}
				} else if next.Kind != token.Punct || next.Value != ")" {
					s.InsertAt(i+1, token.Token{Kind: token.Whitespace, Value: " "})
					i++
					changed = true
				}
			}
		}
	}
	return changed
}
