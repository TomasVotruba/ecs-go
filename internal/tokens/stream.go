// Package tokens holds a flat, mutable, index-addressable token stream. This is
// the ECS/PHP-CS-Fixer working model: fixers read tokens by index and insert,
// replace or remove them in place, then Render rebuilds the source.
package tokens

import (
	"strings"

	"ecs-go/internal/token"
)

type Stream struct {
	toks []token.Token
}

func New(toks []token.Token) *Stream {
	return &Stream{toks: toks}
}

func (s *Stream) Len() int { return len(s.toks) }

func (s *Stream) At(i int) token.Token { return s.toks[i] }

func (s *Stream) Set(i int, t token.Token) { s.toks[i] = t }

// SetValue replaces only the Value of the token at i, keeping its kind.
func (s *Stream) SetValue(i int, v string) { s.toks[i].Value = v }

func (s *Stream) RemoveAt(i int) {
	s.toks = append(s.toks[:i], s.toks[i+1:]...)
}

func (s *Stream) InsertAt(i int, t token.Token) {
	s.toks = append(s.toks, token.Token{})
	copy(s.toks[i+1:], s.toks[i:])
	s.toks[i] = t
}

// Render concatenates every token value back into source.
func (s *Stream) Render() string {
	var b strings.Builder
	for _, t := range s.toks {
		b.WriteString(t.Value)
	}
	return b.String()
}

// Tokens returns the underlying slice (read-only use).
func (s *Stream) Tokens() []token.Token { return s.toks }
