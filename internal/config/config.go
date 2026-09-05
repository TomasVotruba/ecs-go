// Package config mirrors ECSConfig: paths to scan, skip patterns, active rules.
package config

import (
	"runtime"

	"ecs-go/internal/fixer"
	"ecs-go/internal/fixer/rules"
)

type Config struct {
	Paths []string
	Skip  []string // filepath.Match globs tested against each path
	Rules []fixer.Fixer
	Jobs  int // parallel workers
}

// Configure returns a config seeded with all built-in rules, echoing
// ECSConfig::configure()->withPreparedSets(...).
func Configure() *Config {
	return &Config{
		Paths: []string{"."},
		Rules: rules.All(),
		Jobs:  runtime.NumCPU(),
	}
}

func (c *Config) WithPaths(paths ...string) *Config {
	if len(paths) > 0 {
		c.Paths = paths
	}
	return c
}

func (c *Config) WithSkip(patterns ...string) *Config {
	c.Skip = append(c.Skip, patterns...)
	return c
}

func (c *Config) WithRules(rs ...fixer.Fixer) *Config {
	c.Rules = rs
	return c
}

func (c *Config) WithJobs(n int) *Config {
	if n > 0 {
		c.Jobs = n
	}
	return c
}
