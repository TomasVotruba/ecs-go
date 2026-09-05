// Package lexer is a small, self-contained PHP tokenizer producing a flat,
// lossless token stream. It mirrors the shape of PHP's token_get_all rather
// than building an AST, so fixers can walk and mutate tokens by index.
//
// It is a pragmatic subset, not a full PHP grammar: string interpolation is
// not split, heredoc/nowdoc are treated as generic content, and operators are
// emitted as single-char Punct tokens. Enough for coding-standard fixers.
package lexer

import (
	"strings"

	"ecs-go/internal/token"
)

// Lex converts source into a flat token slice. Concatenating the Value of each
// returned token reproduces src exactly.
func Lex(src string) []token.Token {
	l := &lexer{src: src}
	l.run()
	return l.toks
}

type lexer struct {
	src   string
	pos   int
	inPHP bool
	toks  []token.Token
}

func (l *lexer) emit(k token.Kind, start int) {
	l.toks = append(l.toks, token.Token{Kind: k, Value: l.src[start:l.pos], Pos: start})
}

func (l *lexer) run() {
	for l.pos < len(l.src) {
		if l.inPHP {
			l.lexPHP()
		} else {
			l.lexHTML()
		}
	}
}

func (l *lexer) lexHTML() {
	start := l.pos
	idx := strings.Index(l.src[l.pos:], "<?")
	if idx < 0 {
		l.pos = len(l.src)
		l.emit(token.InlineHTML, start)
		return
	}
	l.pos += idx
	if l.pos > start {
		// emit the HTML preceding the tag
		l.toks = append(l.toks, token.Token{Kind: token.InlineHTML, Value: l.src[start:l.pos], Pos: start})
	}
	l.lexOpenTag()
	l.inPHP = true
}

func (l *lexer) lexOpenTag() {
	start := l.pos
	switch {
	case l.hasPrefix("<?php"):
		l.pos += len("<?php")
	case l.hasPrefix("<?="):
		l.pos += len("<?=")
	default:
		l.pos += len("<?")
	}
	l.emit(token.OpenTag, start)
}

func (l *lexer) lexPHP() {
	start := l.pos
	c := l.src[l.pos]

	switch {
	case l.hasPrefix("?>"):
		l.pos += 2
		l.emit(token.CloseTag, start)
		l.inPHP = false
	case isSpace(c):
		for l.pos < len(l.src) && isSpace(l.src[l.pos]) {
			l.pos++
		}
		l.emit(token.Whitespace, start)
	case c == '$':
		l.pos++
		for l.pos < len(l.src) && isIdent(l.src[l.pos]) {
			l.pos++
		}
		l.emit(token.Variable, start)
	case l.hasPrefix("//") || c == '#':
		l.lexLineComment(start)
	case l.hasPrefix("/*"):
		l.lexBlockComment(start)
	case c == '\'':
		l.lexString(start, '\'')
	case c == '"':
		l.lexString(start, '"')
	case isDigit(c):
		for l.pos < len(l.src) && isNumber(l.src[l.pos]) {
			l.pos++
		}
		l.emit(token.Number, start)
	case isIdentStart(c):
		for l.pos < len(l.src) && isIdent(l.src[l.pos]) {
			l.pos++
		}
		l.emit(token.Ident, start)
	default:
		l.pos++
		l.emit(token.Punct, start)
	}
}

func (l *lexer) lexLineComment(start int) {
	for l.pos < len(l.src) && l.src[l.pos] != '\n' {
		// a ?> ends a line comment in PHP; stop before it
		if l.hasPrefix("?>") {
			break
		}
		l.pos++
	}
	l.emit(token.Comment, start)
}

func (l *lexer) lexBlockComment(start int) {
	l.pos += 2 // consume /*
	for l.pos < len(l.src) && !l.hasPrefix("*/") {
		l.pos++
	}
	if l.hasPrefix("*/") {
		l.pos += 2
	}
	val := l.src[start:l.pos]
	kind := token.Comment
	// /** ... */ is a doc comment, but /**/ is not
	if strings.HasPrefix(val, "/**") && val != "/**/" {
		kind = token.DocComment
	}
	l.toks = append(l.toks, token.Token{Kind: kind, Value: val, Pos: start})
}

func (l *lexer) lexString(start int, quote byte) {
	l.pos++ // opening quote
	for l.pos < len(l.src) {
		c := l.src[l.pos]
		if c == '\\' && l.pos+1 < len(l.src) {
			l.pos += 2
			continue
		}
		if c == quote {
			l.pos++
			break
		}
		l.pos++
	}
	l.emit(token.String, start)
}

func (l *lexer) hasPrefix(s string) bool {
	return strings.HasPrefix(l.src[l.pos:], s)
}

func isSpace(c byte) bool { return c == ' ' || c == '\t' || c == '\n' || c == '\r' }
func isDigit(c byte) bool { return c >= '0' && c <= '9' }
func isNumber(c byte) bool {
	return isDigit(c) || c == '.' || c == '_' || c == 'x' || c == 'X' || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')
}
func isIdentStart(c byte) bool {
	return c == '_' || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c >= 0x80
}
func isIdent(c byte) bool { return isIdentStart(c) || isDigit(c) }
