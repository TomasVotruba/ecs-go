package rules

import (
	"strings"

	"ecs-go/internal/token"
	"ecs-go/internal/tokens"
)

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Operator/NewWithParenthesesFixer.php
//
// NewWithParentheses adds parentheses to a parameterless "new": "new Foo" ->
// "new Foo()". Anonymous classes and dynamic "new $var" are left alone.
type NewWithParentheses struct{}

func (NewWithParentheses) Name() string {
	return `PhpCsFixer\Fixer\Operator\NewWithParenthesesFixer`
}

func (NewWithParentheses) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Operator/NewWithParenthesesFixer.php"
}

func (NewWithParentheses) Fix(s *tokens.Stream) bool {
	changed := false
	for i := 0; i < s.Len(); i++ {
		t := s.At(i)
		if t.Kind != token.Keyword || strings.ToLower(t.Value) != "new" {
			continue
		}
		j := skipWhitespace(s, i+1)
		if j >= s.Len() {
			continue
		}
		// only a static class name (Ident, \-qualified, self/parent/static)
		nt := s.At(j)
		if nt.Kind == token.Keyword && strings.ToLower(nt.Value) == "class" {
			continue // anonymous class
		}
		isName := nt.Kind == token.Ident ||
			(nt.Kind == token.Punct && nt.Value == `\`) ||
			(nt.Kind == token.Keyword && isStaticRef(nt.Value))
		if !isName {
			continue // dynamic new $var, new (expr), ...
		}
		// consume the class reference (Ident / \ / self-parent-static)
		k := j
		for k < s.Len() {
			c := s.At(k)
			if c.Kind == token.Ident || (c.Kind == token.Punct && c.Value == `\`) ||
				(c.Kind == token.Keyword && isStaticRef(c.Value)) {
				k++
				continue
			}
			break
		}
		if nextSignificantValue(s, k-1) == "(" {
			continue // already has parentheses
		}
		s.InsertAt(k, token.Token{Kind: token.Punct, Value: "("})
		s.InsertAt(k+1, token.Token{Kind: token.Punct, Value: ")"})
		changed = true
	}
	return changed
}

func isStaticRef(v string) bool {
	switch strings.ToLower(v) {
	case "self", "parent", "static":
		return true
	}
	return false
}

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Basic/SingleLineEmptyBodyFixer.php
//
// SingleLineEmptyBody collapses an empty class or function body to "{}" on the
// declaration line ("function f()\n{\n}" -> "function f() {}").
type SingleLineEmptyBody struct{}

func (SingleLineEmptyBody) Name() string {
	return `PhpCsFixer\Fixer\Basic\SingleLineEmptyBodyFixer`
}

func (SingleLineEmptyBody) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Basic/SingleLineEmptyBodyFixer.php"
}

func (SingleLineEmptyBody) Fix(s *tokens.Stream) bool {
	changed := false
	for i := 0; i < s.Len(); i++ {
		if s.At(i).Kind != token.Punct || s.At(i).Value != "{" {
			continue
		}
		switch kind, _ := classifyBrace(s, i); kind {
		case braceClassLike, braceFunctionDecl:
		default:
			continue
		}
		closeIdx := s.MatchForward(i)
		if closeIdx < 0 {
			continue
		}
		empty := true
		for k := i + 1; k < closeIdx; k++ {
			if s.At(k).Kind != token.Whitespace {
				empty = false
				break
			}
		}
		if !empty || closeIdx == i+1 {
			continue
		}
		// remove the whitespace between { and }
		for k := closeIdx - 1; k > i; k-- {
			s.RemoveAt(k)
		}
		// pull the brace onto the declaration line
		if i > 0 && s.At(i-1).Kind == token.Whitespace && hasNewline(s.At(i-1).Value) {
			s.SetValue(i-1, " ")
		}
		changed = true
	}
	return changed
}
