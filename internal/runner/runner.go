// Package runner ties finder, lexer, token stream and fixers together, echoing
// ECS's application layer.
package runner

import (
	"os"

	"ecs-go/internal/config"
	"ecs-go/internal/finder"
	"ecs-go/internal/lexer"
	"ecs-go/internal/tokens"
)

// FileResult records what happened to one file.
type FileResult struct {
	Path         string
	AppliedRules []string // rules that changed this file
	After        string   // fixed content
}

// Changed reports whether any rule modified the file.
func (r FileResult) Changed() bool { return len(r.AppliedRules) > 0 }

// Run scans the config's paths and applies rules. When write is true, changed
// files are written back to disk.
func Run(cfg *config.Config, write bool) ([]FileResult, error) {
	files, err := finder.Find(cfg.Paths, cfg.Skip)
	if err != nil {
		return nil, err
	}

	var results []FileResult
	for _, path := range files {
		res, err := fixFile(cfg, path, write)
		if err != nil {
			return nil, err
		}
		if res.Changed() {
			results = append(results, res)
		}
	}
	return results, nil
}

func fixFile(cfg *config.Config, path string, write bool) (FileResult, error) {
	src, err := os.ReadFile(path)
	if err != nil {
		return FileResult{}, err
	}

	stream := tokens.New(lexer.Lex(string(src)))
	res := FileResult{Path: path}

	for _, rule := range cfg.Rules {
		if rule.Fix(stream) {
			res.AppliedRules = append(res.AppliedRules, rule.Name())
		}
	}

	res.After = stream.Render()
	if res.Changed() && write {
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
