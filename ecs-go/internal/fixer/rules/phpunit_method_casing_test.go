package rules

import "testing"

func TestPhpUnitMethodCasingDefaultsToSnakeCase(t *testing.T) {
	src := "<?php class FooTest extends TestCase {\n    public function testKnownThing(): void {}\n}"
	got, changed := apply(t, PhpUnitMethodCasing{}, src)
	want := "<?php class FooTest extends TestCase {\n    public function test_known_thing(): void {}\n}"
	if !changed || got != want {
		t.Fatalf("changed=%v got=%q want=%q", changed, got, want)
	}
}

func TestPhpUnitMethodCasingCamelCaseLeavesCamelNamesUnchanged(t *testing.T) {
	src := "<?php class FooTest extends TestCase {\n    public function testKnownThing(): void {}\n}"
	fixer := PhpUnitMethodCasing{}.WithConfig(map[string]any{"case": "camel_case"})
	got, changed := apply(t, fixer, src)
	if changed || got != src {
		t.Fatalf("changed=%v got=%q want unchanged", changed, got)
	}
}

func TestPhpUnitMethodCasingCamelCaseConvertsSnakeNames(t *testing.T) {
	src := "<?php class FooTest extends TestCase {\n    public function test_known_thing(): void {}\n}"
	fixer := PhpUnitMethodCasing{}.WithConfig(map[string]any{"case": "camel_case"})
	got, changed := apply(t, fixer, src)
	want := "<?php class FooTest extends TestCase {\n    public function testKnownThing(): void {}\n}"
	if !changed || got != want {
		t.Fatalf("changed=%v got=%q want=%q", changed, got, want)
	}
}
