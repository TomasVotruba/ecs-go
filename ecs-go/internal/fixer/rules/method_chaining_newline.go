package rules

import (
	"strings"

	"ecs-go/internal/token"
	"ecs-go/internal/tokens"
)

// Symplify: https://github.com/symplify/coding-standard/blob/main/src/Fixer/Spacing/MethodChainingNewlineFixer.php
//
// MethodChainingNewline puts each chained method call on its own line. This is a
// deliberately conservative port: it only splits a chain whose root is a plain
// variable ($x, $this) sitting at a statement boundary and entirely on one line
// - the unambiguous case symplify always breaks. Chains that are call/array
// arguments, grouped expressions ((new X)->y()), or already multi-line are left
// untouched, so the fixer is a no-op on code ECS has already formatted.
type MethodChainingNewline struct{}

func (MethodChainingNewline) Name() string {
	return `Symplify\CodingStandard\Fixer\Spacing\MethodChainingNewlineFixer`
}

func (MethodChainingNewline) SourceURL() string {
	return "https://github.com/symplify/coding-standard/blob/main/src/Fixer/Spacing/MethodChainingNewlineFixer.php"
}

func (MethodChainingNewline) Fix(s *tokens.Stream) bool {
	changed := false
	for i := 1; i < s.Len(); i++ {
		if s.At(i).Kind != token.Punct || s.At(i).Value != "->" {
			continue
		}
		// the "->" must follow a call's ")"
		prev := sigPrev(s, i)
		if prev < 0 || s.At(prev).Kind != token.Punct || s.At(prev).Value != ")" {
			continue
		}
		// already split across lines between the ")" and the "->": leave it
		if rangeHasNewline(s, prev, i) {
			continue
		}
		// the call/group closing at ")" spans multiple lines: leave the chain
		if open := s.MatchBackward(prev); open >= 0 && rangeHasNewline(s, open, prev) {
			continue
		}
		root, ok := chainRootVariable(s, i)
		if !ok || rootPrecededByExpression(s, root) {
			continue
		}
		// symplify suppresses the break when a "::", "[", "." or "array" appears
		// earlier on the line (its isPartOfMethodCallOrArray heuristic)
		if chainLineHasBreakingChar(s, prev) {
			continue
		}
		nl := "\n" + chainBaseIndent(s, root) + "    "
		if s.At(i-1).Kind == token.Whitespace {
			s.SetValue(i-1, nl)
		} else {
			s.InsertAt(i, token.Token{Kind: token.Whitespace, Value: nl})
			i++
		}
		changed = true
	}
	return changed
}

// chainBaseIndent returns the leading indentation of the statement line holding
// the chain root, so every continuation aligns to the same column.
func chainBaseIndent(s *tokens.Stream, root int) string {
	for i := root - 1; i >= 0; i-- {
		if t := s.At(i); t.Kind == token.Whitespace && strings.Contains(t.Value, "\n") {
			if nl := strings.LastIndexByte(t.Value, '\n'); nl >= 0 {
				return t.Value[nl+1:]
			}
		}
	}
	return ""
}

// chainRootVariable walks back from a "->" to the variable a chain is rooted on,
// stepping over balanced () and [] and over "->name" / "::name" segments. It
// fails if the chain is enclosed by an unmatched "(" or "[" (i.e. it is a call
// or array argument) or if the root is anything but a plain variable.
func chainRootVariable(s *tokens.Stream, opIdx int) (int, bool) {
	depth := 0
	for i := opIdx - 1; i >= 0; i-- {
		t := s.At(i)
		if t.Kind == token.Punct {
			switch t.Value {
			case ")", "]", "}":
				depth++
				continue
			case "(", "[":
				if depth == 0 {
					return 0, false // enclosed by an open bracket: it's an argument
				}
				depth--
				continue
			case "{":
				if depth == 0 {
					return 0, false // statement/block start reached, no variable root
				}
				depth--
				continue
			case ";", ",":
				if depth == 0 {
					return 0, false
				}
				continue
			}
		}
		if depth > 0 {
			continue
		}
		switch {
		case t.Kind == token.Variable:
			return i, true
		case t.Kind == token.Ident, t.Kind == token.Whitespace,
			t.Kind == token.Comment, t.Kind == token.DocComment:
			continue
		case t.Kind == token.Punct && (t.Value == "->" || t.Value == "?->" || t.Value == "::"):
			continue
		default:
			return 0, false // an operator/keyword/literal: not a plain-variable chain
		}
	}
	return 0, false
}

// chainLineHasBreakingChar mirrors symplify's isPartOfMethodCallOrArray: walking
// back from the ")" at pos to the start of the line, a "[", "::", "." or "array"
// (or an unmatched enclosing "(") means the chain is treated as part of a call or
// array and is left inline.
func chainLineHasBreakingChar(s *tokens.Stream, pos int) bool {
	nesting := 0
	for i := pos; i >= 0; i-- {
		t := s.At(i)
		if t.Kind == token.Whitespace && strings.Contains(t.Value, "\n") {
			return false
		}
		if t.Kind == token.Punct && (t.Value == "[" || t.Value == "::" || t.Value == ".") {
			return true
		}
		if t.Kind == token.Keyword && strings.ToLower(t.Value) == "array" {
			return true
		}
		if t.Kind == token.Punct && t.Value == ")" {
			nesting--
		} else if t.Kind == token.Punct && t.Value == "(" {
			if nesting != 0 {
				nesting++
			} else {
				return true
			}
		}
	}
	return false
}

// rootPrecededByExpression reports whether the chain root is part of a larger
// expression (so breaking it would be unsafe). A safe root directly follows a
// statement boundary or a simple assignment / return.
func rootPrecededByExpression(s *tokens.Stream, root int) bool {
	p := sigPrev(s, root)
	if p < 0 {
		return false
	}
	t := s.At(p)
	if t.Kind == token.OpenTag {
		return false
	}
	if t.Kind == token.Keyword && strings.ToLower(t.Value) == "return" {
		return false
	}
	if t.Kind == token.Punct {
		switch t.Value {
		case ";", "{", "}", "=":
			return false
		}
	}
	return true
}
