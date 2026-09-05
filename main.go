// Command ecs-go is a token-based coding-standard checker/fixer for PHP,
// modeled on symplify/easy-coding-standard.
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
	configPath := ""
	var paths []string

	for i := 0; i < len(args); i++ {
		a := args[i]
		switch {
		case a == "--fix":
			fix = true
		case a == "list-checkers":
			return listCheckers()
		case a == "-h" || a == "--help":
			usage()
			return 0
		case a == "--config":
			if i+1 < len(args) {
				configPath = args[i+1]
				i++
			}
		case len(a) > 9 && a[:9] == "--config=":
			configPath = a[9:]
		default:
			paths = append(paths, a)
		}
	}

	cfg, err := loadConfig(configPath)
	if err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		return 2
	}
	// CLI paths override the config; runs across all CPU cores by default
	cfg.WithPaths(paths...)

	results, err := runner.Run(cfg, fix)
	if err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		return 2
	}

	return reporter.Report(os.Stdout, results, fix)
}

// loadConfig uses an explicit --config path, else an ecs-go.json in the working
// directory, else the built-in defaults (all rules).
func loadConfig(configPath string) (*config.Config, error) {
	if configPath != "" {
		return config.Load(configPath)
	}
	if _, err := os.Stat("ecs-go.json"); err == nil {
		return config.Load("ecs-go.json")
	}
	return config.Configure(), nil
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
  ecs-go [paths...]            check paths (default: .)
  ecs-go --fix [paths...]      fix paths in place
  ecs-go --config FILE ...     use an ecs-go.json config
  ecs-go list-checkers         list registered fixers

Loads ecs-go.json from the working directory when present.
Runs across all CPU cores by default.
`)
}
