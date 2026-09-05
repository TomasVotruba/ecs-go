package lexer

import (
	"strings"
	"testing"

	"ecs-go/internal/token"
)

// lossless is the core invariant: rendering all token values equals the source.
func TestLexLossless(t *testing.T) {
	cases := []string{
		"",
		"<?php $x = 1;\n",
		"plain html only",
		"<html><?php echo $a ; ?><div>after</div>",
		"<?php\n// line comment\n$foo = 'bar';\n/** doc */\nfunction f() {}\n",
		"<?php $s = \"a ; b\"; $t = 'c ; d';\n",
		"text <?= $v ?> more",
	}
	for _, src := range cases {
		var got strings.Builder
		for _, tk := range Lex(src) {
			got.WriteString(tk.Value)
		}
		if got.String() != src {
			t.Errorf("not lossless\n src: %q\n got: %q", src, got.String())
		}
	}
}

func TestLexClassifies(t *testing.T) {
	toks := Lex("<?php $x = 1;")
	want := []token.Kind{
		token.OpenTag,    // <?php
		token.Whitespace, // space
		token.Variable,   // $x
		token.Whitespace,
		token.Punct, // =
		token.Whitespace,
		token.Number, // 1
		token.Punct,  // ;
	}
	if len(toks) != len(want) {
		t.Fatalf("got %d tokens, want %d: %+v", len(toks), len(want), toks)
	}
	for i, k := range want {
		if toks[i].Kind != k {
			t.Errorf("token %d: got %s, want %s (value %q)", i, toks[i].Kind, k, toks[i].Value)
		}
	}
}

func TestDocCommentVsComment(t *testing.T) {
	toks := Lex("<?php /** doc */ /* plain */ /**/")
	var kinds []token.Kind
	for _, tk := range toks {
		if tk.Kind == token.DocComment || tk.Kind == token.Comment {
			kinds = append(kinds, tk.Kind)
		}
	}
	want := []token.Kind{token.DocComment, token.Comment, token.Comment}
	if len(kinds) != len(want) {
		t.Fatalf("got %v", kinds)
	}
	for i := range want {
		if kinds[i] != want[i] {
			t.Errorf("comment %d: got %s want %s", i, kinds[i], want[i])
		}
	}
}
