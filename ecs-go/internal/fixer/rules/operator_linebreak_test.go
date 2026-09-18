package rules

import "testing"

func TestOperatorLinebreak(t *testing.T) {
	f := OperatorLinebreak{}

	got, changed := apply(t, f, "<?php\n$a = $b ||\n    $c;\n")
	if want := "<?php\n$a = $b\n    || $c;\n"; !changed || got != want {
		t.Fatalf("|| move: changed=%v got=%q", changed, got)
	}

	got, changed = apply(t, f, "<?php\n$s = $a .\n    $b;\n")
	if want := "<?php\n$s = $a\n    . $b;\n"; !changed || got != want {
		t.Fatalf(". move: changed=%v got=%q", changed, got)
	}

	if _, changed := apply(t, f, "<?php\n$a = $b\n    || $c;\n"); changed {
		t.Fatal("already-correct must be a no-op")
	}
	if _, changed := apply(t, f, "<?php\n$a = $b || $c;\n"); changed {
		t.Fatal("single-line must be a no-op")
	}

	// subset: a ternary is left unchanged (":" and "?" are excluded)
	if _, changed := apply(t, f, "<?php\n$a = $b ?\n    $c :\n    $d;\n"); changed {
		t.Fatal("ternary must be left unchanged by the safe subset")
	}
}
