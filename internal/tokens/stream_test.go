package tokens

import (
	"testing"

	"ecs-go/internal/token"
)

func mk(vals ...string) *Stream {
	var toks []token.Token
	for _, v := range vals {
		toks = append(toks, token.Token{Kind: token.Punct, Value: v})
	}
	return New(toks)
}

func TestInsertRemoveRender(t *testing.T) {
	s := mk("a", "b", "d")
	s.InsertAt(2, token.Token{Value: "c"})
	if got := s.Render(); got != "abcd" {
		t.Fatalf("insert: got %q", got)
	}
	s.RemoveAt(0)
	if got := s.Render(); got != "bcd" {
		t.Fatalf("remove: got %q", got)
	}
}

func TestInsertAtEnd(t *testing.T) {
	s := mk("a", "b")
	s.InsertAt(s.Len(), token.Token{Value: "c"})
	if got := s.Render(); got != "abc" {
		t.Fatalf("append: got %q", got)
	}
}

func TestSetValue(t *testing.T) {
	s := mk("a", "b")
	s.SetValue(1, "B")
	if got := s.Render(); got != "aB" {
		t.Fatalf("setvalue: got %q", got)
	}
}
