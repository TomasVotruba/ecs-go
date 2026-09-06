package rules

import (
	"testing"

	fixerpkg "ecs-go/internal/fixer"
)

func TestMethodArgumentSpace(t *testing.T) {
	cases := []struct {
		name    string
		src     string
		want    string
		changed bool
	}{
		{"already correct", "<?php foo($a, $b);", "<?php foo($a, $b);", false},
		{"missing and stray spaces", "<?php foo($a ,$b,$c);", "<?php foo($a, $b, $c);", true},
		{"space before comma", "<?php bar($a , $b);", "<?php bar($a, $b);", true},
		{"inner paren spaces left alone", "<?php bar( $a , $b );", "<?php bar( $a, $b );", true},
		{"array comma left alone", "<?php $x = [1,2];", "<?php $x = [1,2];", false},
		{"array inside call", "<?php foo([1,2],$b);", "<?php foo([1,2], $b);", true},
		{"multiline comma left alone", "<?php foo($a,\n    $b);", "<?php foo($a,\n    $b);", false},
		{"trailing comma before paren", "<?php foo($a,);", "<?php foo($a,);", false},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got, changed := apply(t, MethodArgumentSpace{}, tc.src)
			if got != tc.want || changed != tc.changed {
				t.Fatalf("changed=%v got=%q want=%q", changed, got, tc.want)
			}
			// Idempotent: running the fixer again must not change the result.
			again, changedAgain := apply(t, MethodArgumentSpace{}, got)
			if changedAgain || again != got {
				t.Fatalf("not idempotent: changed=%v got=%q", changedAgain, again)
			}
		})
	}
}

func TestMethodArgumentSpaceSourceURL(t *testing.T) {
	r := MethodArgumentSpace{}
	if got := fixerpkg.SourceURLFor(r.Name()); got != r.SourceURL() {
		t.Fatalf("SourceURL mismatch: derived=%q declared=%q", got, r.SourceURL())
	}
}
