package rules

import "testing"

func TestMethodChainingNewline(t *testing.T) {
	f := MethodChainingNewline{}

	cases := []struct {
		src, want string
		changed   bool
	}{
		// plain variable-rooted chain at statement level: split
		{"<?php\n$x->one()->two();", "<?php\n$x->one()\n    ->two();", true},
		{"<?php\nreturn $this->a()->b()->c();", "<?php\nreturn $this->a()\n    ->b()\n    ->c();", true},
		{"<?php\n$a = $x->one()->two();", "<?php\n$a = $x->one()\n    ->two();", true},

		// single method call: nothing to split
		{"<?php\n$x->one();", "<?php\n$x->one();", false},

		// a "::" earlier on the line suppresses the split (symplify heuristic)
		{"<?php\nstatic::$r = $b->one()->two();", "<?php\nstatic::$r = $b->one()->two();", false},
		// a "[" earlier on the line likewise
		{"<?php\n$s = $a->b($m[2])->c();", "<?php\n$s = $a->b($m[2])->c();", false},

		// grouped / call-argument chains: left inline
		{"<?php\nfoo($x->a()->b());", "<?php\nfoo($x->a()->b());", false},
		{"<?php\n$y = (new Foo())->bar()->baz();", "<?php\n$y = (new Foo())->bar()->baz();", false},

		// already multi-line: no-op
		{"<?php\n$x->one()\n    ->two();", "<?php\n$x->one()\n    ->two();", false},
	}
	for _, c := range cases {
		got, changed := apply(t, f, c.src)
		if got != c.want || changed != c.changed {
			t.Errorf("src=%q\n got=%q changed=%v\nwant=%q changed=%v", c.src, got, changed, c.want, c.changed)
		}
	}
}
