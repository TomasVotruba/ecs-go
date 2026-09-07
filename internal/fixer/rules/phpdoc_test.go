package rules

import "testing"

func TestPhpdocTrim(t *testing.T) {
	src := "<?php\n/**\n *\n * Summary.\n *\n */\nfunction f() {}"
	got, changed := apply(t, PhpdocTrim{}, src)
	want := "<?php\n/**\n * Summary.\n */\nfunction f() {}"
	if !changed || got != want {
		t.Fatalf("changed=%v\n got: %q\nwant: %q", changed, got, want)
	}
	// already-trimmed docblock is untouched
	if _, changed := apply(t, PhpdocTrim{}, want); changed {
		t.Fatal("trimmed docblock must not change")
	}
}

func TestPhpdocNoEmptyReturn(t *testing.T) {
	src := "<?php\n/**\n * Do it.\n *\n * @return void\n */\nfunction f() {}"
	got, changed := apply(t, PhpdocNoEmptyReturn{}, src)
	want := "<?php\n/**\n * Do it.\n *\n */\nfunction f() {}"
	if !changed || got != want {
		t.Fatalf("changed=%v\n got: %q\nwant: %q", changed, got, want)
	}
	// a real return type is kept
	keep := "<?php\n/**\n * @return int\n */\nfunction f() {}"
	if _, changed := apply(t, PhpdocNoEmptyReturn{}, keep); changed {
		t.Fatal("@return int must be kept")
	}
}

func TestPhpdocScalar(t *testing.T) {
	src := "<?php\n/**\n * @param integer $a\n * @param boolean|null $b\n * @return double\n */\nfunction f($a, $b) {}"
	got, changed := apply(t, PhpdocScalar{}, src)
	want := "<?php\n/**\n * @param int $a\n * @param bool|null $b\n * @return float\n */\nfunction f($a, $b) {}"
	if !changed || got != want {
		t.Fatalf("changed=%v\n got: %q\nwant: %q", changed, got, want)
	}
	// a description mentioning "integer" is not a type and stays
	keep := "<?php\n/**\n * This returns an integer value.\n */\nfunction f() {}"
	if _, changed := apply(t, PhpdocScalar{}, keep); changed {
		t.Fatal("prose 'integer' must not change")
	}
}
