package rules

import (
	"strings"

	"ecs-go/internal/token"
	"ecs-go/internal/tokens"
)

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Operator/OperatorLinebreakFixer.php
//
// OperatorLinebreak moves a multiline operator to the beginning of the next line
// (default position). This is a DOCUMENTED SAFE SUBSET: it only handles operators
// that are a single, unambiguous token in the flat lexer. It deliberately never
// touches ":" "?" "|" "&" or the object operators "->"/"?->", because telling a
// ternary/switch/return-type/nullable/union/reference apart needs a CT-token
// classification layer the lexer does not have; guessing there would corrupt code.
type OperatorLinebreak struct{}

var operatorLinebreakPunct = map[string]bool{
	"||": true, "&&": true, ".": true, "+": true, "-": true, "*": true,
	"/": true, "%": true, "**": true, "==": true, "===": true, "!=": true,
	"!==": true, "<>": true, "<": true, ">": true, "<=": true, ">=": true,
	"<=>": true, "??": true, "=>": true, "=": true, ".=": true, "+=": true,
	"-=": true, "*=": true, "/=": true, "%=": true, "**=": true, "&=": true,
	"|=": true, "^=": true, "<<=": true, ">>=": true, "??=": true,
}

func (OperatorLinebreak) Name() string {
	return `PhpCsFixer\Fixer\Operator\OperatorLinebreakFixer`
}

func (OperatorLinebreak) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Operator/OperatorLinebreakFixer.php"
}

func isOperatorLinebreakToken(t token.Token) bool {
	switch t.Kind {
	case token.Keyword:
		switch strings.ToLower(t.Value) {
		case "and", "or", "xor":
			return true
		}
	case token.Punct:
		return operatorLinebreakPunct[t.Value]
	}
	return false
}

func (OperatorLinebreak) Fix(s *tokens.Stream) bool {
	changed := false
	// bottom-up: inserting tokens only shifts indices after the operator
	for i := s.Len() - 1; i > 0; i-- {
		if !isOperatorLinebreakToken(s.At(i)) {
			continue
		}
		prevM := sigPrev(s, i)
		nextM := sigNext(s, i)
		if prevM < 0 || nextM < 0 {
			continue
		}
		// operator must be surrounded by multiline whitespace
		if !operatorSpanMultiline(s, prevM+1, nextM-1) {
			continue
		}
		// already at the beginning of a line if there is no newline after it
		if !operatorSpanMultiline(s, i, nextM-1) {
			continue
		}
		moveOperatorToLineStart(s, i, nextM)
		changed = true
	}
	return changed
}

// operatorSpanMultiline mirrors OperatorLinebreak::isMultiline - whether any
// token in the inclusive range [from, to] contains a newline.
func operatorSpanMultiline(s *tokens.Stream, from, to int) bool {
	if from < 0 {
		from = 0
	}
	for j := from; j <= to && j < s.Len(); j++ {
		if strings.ContainsAny(s.At(j).Value, "\n\r") {
			return true
		}
	}
	return false
}

// moveOperatorToLineStart clears the operator (and the space before it) and
// re-inserts it just before the next meaningful token, adding a single space
// after it when a space preceded it - matching fixMoveToTheBeginning.
func moveOperatorToLineStart(s *tokens.Stream, opIndex, nextM int) {
	opVal := s.At(opIndex).Value
	opKind := s.At(opIndex).Kind
	hadSpaceBefore := opIndex > 0 && s.At(opIndex-1).Kind == token.Whitespace
	if hadSpaceBefore {
		s.SetValue(opIndex-1, "")
	}
	s.SetValue(opIndex, "")
	s.InsertAt(nextM, token.Token{Kind: opKind, Value: opVal})
	if hadSpaceBefore {
		s.InsertAt(nextM+1, token.Token{Kind: token.Whitespace, Value: " "})
	}
}
