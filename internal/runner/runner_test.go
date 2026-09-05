package runner

import (
	"os"
	"path/filepath"
	"testing"

	"ecs-go/internal/config"
)

func TestRunFixesFile(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "sample.php")
	dirty := "<?php $x = 1 ;   \n\n\n"
	if err := os.WriteFile(path, []byte(dirty), 0o644); err != nil {
		t.Fatal(err)
	}

	cfg := config.Configure().WithPaths(dir)

	// check mode: reports but does not write
	results, err := Run(cfg, false)
	if err != nil {
		t.Fatal(err)
	}
	if len(results) != 1 || !results[0].Changed() {
		t.Fatalf("check: expected 1 changed file, got %+v", results)
	}
	if b, _ := os.ReadFile(path); string(b) != dirty {
		t.Fatal("check mode must not modify the file")
	}

	// fix mode: writes cleaned content
	if _, err := Run(cfg, true); err != nil {
		t.Fatal(err)
	}
	b, _ := os.ReadFile(path)
	want := "<?php $x = 1;\n"
	if string(b) != want {
		t.Fatalf("fix: got %q want %q", string(b), want)
	}

	// second check run is clean
	results, _ = Run(cfg, false)
	if len(results) != 0 {
		t.Fatalf("expected clean after fix, got %+v", results)
	}
}
