package rules

import (
	"testing"

	"ecs-go/internal/lexer"
	"ecs-go/internal/tokens"
)

func apply(t *testing.T, r interface {
	Fix(*tokens.Stream) bool
}, src string) (string, bool) {
	t.Helper()
	s := tokens.New(lexer.Lex(src))
	changed := r.Fix(s)
	return s.Render(), changed
}

func TestNoTrailingWhitespace(t *testing.T) {
	got, changed := apply(t, NoTrailingWhitespace{}, "<?php $x = 1;   \n$y = 2;\t\n")
	want := "<?php $x = 1;\n$y = 2;\n"
	if !changed || got != want {
		t.Fatalf("changed=%v got=%q want=%q", changed, got, want)
	}

	if _, changed := apply(t, NoTrailingWhitespace{}, "<?php $x = 1;\n"); changed {
		t.Fatal("clean input should not change")
	}
}

func TestSingleBlankLineAtEndOfFile(t *testing.T) {
	got, changed := apply(t, SingleBlankLineAtEndOfFile{}, "<?php echo 1;\n\n\n")
	if !changed || got != "<?php echo 1;\n" {
		t.Fatalf("collapse: changed=%v got=%q", changed, got)
	}

	got, changed = apply(t, SingleBlankLineAtEndOfFile{}, "<?php echo 1;")
	if !changed || got != "<?php echo 1;\n" {
		t.Fatalf("append: changed=%v got=%q", changed, got)
	}

	if _, changed := apply(t, SingleBlankLineAtEndOfFile{}, "<?php echo 1;\n"); changed {
		t.Fatal("single newline should not change")
	}
}

func TestNoSpaceBeforeSemicolon(t *testing.T) {
	got, changed := apply(t, NoSpaceBeforeSemicolon{}, "<?php $x = 1 ; $y = 2  ;")
	want := "<?php $x = 1; $y = 2;"
	if !changed || got != want {
		t.Fatalf("changed=%v got=%q want=%q", changed, got, want)
	}

	// whitespace spanning a newline before ; is preserved
	if _, changed := apply(t, NoSpaceBeforeSemicolon{}, "<?php $x = 1\n;"); changed {
		t.Fatal("multi-line whitespace before ; should be kept")
	}
}
