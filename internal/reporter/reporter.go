// Package reporter prints run results in a compact, ECS-like summary.
package reporter

import (
	"fmt"
	"io"
	"strings"

	"ecs-go/internal/runner"
)

// Report writes results to w. When write is false (check mode) it frames issues
// as "would fix"; when true, as "fixed". Returns the number of affected files.
func Report(w io.Writer, results []runner.FileResult, write bool) int {
	if len(results) == 0 {
		fmt.Fprintln(w, "[OK] No coding standard issues found.")
		return 0
	}

	verb := "would fix"
	if write {
		verb = "fixed"
	}
	for _, r := range results {
		fmt.Fprintf(w, "%s: %s (%s)\n", verb, r.Path, strings.Join(r.AppliedRules, ", "))
	}

	fmt.Fprintf(w, "\n[%s] %d file(s) with issues.\n", strings.ToUpper(verb), len(results))
	if !write {
		fmt.Fprintln(w, "Run with --fix to apply changes.")
	}
	return len(results)
}
