// Command ecs-go is a thin, token-based coding-standard checker/fixer for PHP,
// modeled on symplify/easy-coding-standard. It ships a small standalone PHP
// lexer and a handful of fixers as a proof of the architecture.
package main

import (
	"fmt"
	"os"

	"ecs-go/internal/config"
	"ecs-go/internal/fixer/rules"
	"ecs-go/internal/reporter"
	"ecs-go/internal/runner"
)

func main() {
	os.Exit(run(os.Args[1:]))
}

func run(args []string) int {
	fix := false
	var paths []string

	for _, a := range args {
		switch a {
		case "--fix":
			fix = true
		case "list-checkers":
			return listCheckers()
		case "-h", "--help":
			usage()
			return 0
		default:
			paths = append(paths, a)
		}
	}

	cfg := config.Configure().WithPaths(paths...)

	results, err := runner.Run(cfg, fix)
	if err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		return 2
	}

	n := reporter.Report(os.Stdout, results, fix)
	// check mode with findings -> non-zero, like ECS
	if n > 0 && !fix {
		return 1
	}
	return 0
}

func listCheckers() int {
	fmt.Println("Registered checkers:")
	for _, r := range rules.All() {
		fmt.Printf("  - %s\n", r.Name())
	}
	return 0
}

func usage() {
	fmt.Print(`ecs-go - token-based PHP coding standard tool

Usage:
  ecs-go [paths...]          check paths (default: .)
  ecs-go --fix [paths...]    fix paths in place
  ecs-go list-checkers       list registered rules
`)
}
