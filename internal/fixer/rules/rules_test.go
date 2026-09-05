package rules

import (
	"testing"

	"ecs-go/internal/lexer"
	"ecs-go/internal/tokens"
)

type fixerRule interface {
	Fix(*tokens.Stream) bool
}

func apply(t *testing.T, r fixerRule, src string) (string, bool) {
	t.Helper()
	s := tokens.New(lexer.Lex(src))
	changed := r.Fix(s)
	return s.Render(), changed
}

func TestNoSinglelineWhitespaceBeforeSemicolons(t *testing.T) {
	got, changed := apply(t, NoSinglelineWhitespaceBeforeSemicolons{}, "<?php $x = 1 ; $y = 2  ;")
	if want := "<?php $x = 1; $y = 2;"; !changed || got != want {
		t.Fatalf("changed=%v got=%q want=%q", changed, got, want)
	}
	if _, changed := apply(t, NoSinglelineWhitespaceBeforeSemicolons{}, "<?php $x = 1\n;"); changed {
		t.Fatal("multi-line whitespace before ; should be kept")
	}
}

func TestSpaceAfterSemicolon(t *testing.T) {
	got, changed := apply(t, SpaceAfterSemicolon{}, "<?php $a=1;$b=2;")
	if want := "<?php $a=1; $b=2;"; !changed || got != want {
		t.Fatalf("changed=%v got=%q want=%q", changed, got, want)
	}
	// empty for-loop head must not gain spaces
	if _, changed := apply(t, SpaceAfterSemicolon{}, "<?php for(;;){}"); changed {
		t.Fatal("';' before ')' should be left alone")
	}
}

func TestNoWhitespaceInBlankLine(t *testing.T) {
	got, changed := apply(t, NoWhitespaceInBlankLine{}, "<?php\n$a = 1;\n   \n$b = 2;\n")
	if want := "<?php\n$a = 1;\n\n$b = 2;\n"; !changed || got != want {
		t.Fatalf("changed=%v got=%q want=%q", changed, got, want)
	}
}

func TestBlankLineAfterOpeningTag(t *testing.T) {
	got, changed := apply(t, BlankLineAfterOpeningTag{}, "<?php\n$a = 1;\n")
	if want := "<?php\n\n$a = 1;\n"; !changed || got != want {
		t.Fatalf("changed=%v got=%q want=%q", changed, got, want)
	}
	if _, changed := apply(t, BlankLineAfterOpeningTag{}, "<?php\n\n$a = 1;\n"); changed {
		t.Fatal("existing blank line must not change")
	}
	if _, changed := apply(t, BlankLineAfterOpeningTag{}, "<?php $a = 1;\n"); changed {
		t.Fatal("code on tag line must not change")
	}
}

func TestNoLeadingNamespaceWhitespace(t *testing.T) {
	got, changed := apply(t, NoLeadingNamespaceWhitespace{}, "<?php\n    namespace App;\n")
	if want := "<?php\nnamespace App;\n"; !changed || got != want {
		t.Fatalf("changed=%v got=%q want=%q", changed, got, want)
	}
}

func TestNoTrailingWhitespace(t *testing.T) {
	got, changed := apply(t, NoTrailingWhitespace{}, "<?php $x = 1;   \n$y = 2;\t\n")
	if want := "<?php $x = 1;\n$y = 2;\n"; !changed || got != want {
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
