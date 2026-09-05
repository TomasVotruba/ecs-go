// Package runner ties finder, lexer, token stream and fixers together, echoing
// ECS's application layer. Files are processed by a pool of parallel workers.
package runner

import (
	"os"
	"sort"
	"sync"

	"ecs-go/internal/config"
	"ecs-go/internal/diff"
	"ecs-go/internal/finder"
	"ecs-go/internal/lexer"
	"ecs-go/internal/tokens"
)

// FileResult records the change made to one file.
type FileResult struct {
	Path         string
	AppliedRules []string // rules that changed this file, in application order
	Diff         string   // unified diff (Original vs fixed)
	After        string   // fixed content
}

func (r FileResult) Changed() bool { return len(r.AppliedRules) > 0 }

// Run scans the config's paths and applies rules across Jobs workers. When
// write is true, changed files are written back to disk. Results are returned
// sorted by path so output is deterministic regardless of worker scheduling.
func Run(cfg *config.Config, write bool) ([]FileResult, error) {
	files, err := finder.Find(cfg.Paths, cfg.Skip)
	if err != nil {
		return nil, err
	}

	jobs := cfg.Jobs
	if jobs < 1 {
		jobs = 1
	}

	paths := make(chan string)
	results := make(chan FileResult)
	var firstErr error
	var errOnce sync.Once

	var wg sync.WaitGroup
	for w := 0; w < jobs; w++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for path := range paths {
				res, err := fixFile(cfg, path, write)
				if err != nil {
					errOnce.Do(func() { firstErr = err })
					continue
				}
				if res.Changed() {
					results <- res
				}
			}
		}()
	}

	go func() {
		for _, p := range files {
			paths <- p
		}
		close(paths)
		wg.Wait()
		close(results)
	}()

	var collected []FileResult
	for r := range results {
		collected = append(collected, r)
	}
	if firstErr != nil {
		return nil, firstErr
	}

	sort.Slice(collected, func(i, j int) bool { return collected[i].Path < collected[j].Path })
	return collected, nil
}

func fixFile(cfg *config.Config, path string, write bool) (FileResult, error) {
	src, err := os.ReadFile(path)
	if err != nil {
		return FileResult{}, err
	}
	original := string(src)

	stream := tokens.New(lexer.Lex(original))
	res := FileResult{Path: path}

	for _, rule := range cfg.Rules {
		if rule.Fix(stream) {
			res.AppliedRules = append(res.AppliedRules, rule.Name())
		}
	}

	res.After = stream.Render()
	if !res.Changed() {
		return res, nil
	}
	res.Diff = diff.Unified(original, res.After)

	if write {
		info, statErr := os.Stat(path)
		mode := os.FileMode(0o644)
		if statErr == nil {
			mode = info.Mode()
		}
		if err := os.WriteFile(path, []byte(res.After), mode); err != nil {
			return FileResult{}, err
		}
	}
	return res, nil
}
