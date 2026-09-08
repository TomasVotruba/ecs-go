// Package fixer defines the Fixer contract. A fixer inspects a token stream and
// mutates it in place, mirroring ECS/PHP-CS-Fixer fixers.
package fixer

import (
	"strings"

	"ecs-go/internal/tokens"
)

// SourceBase is the GitHub location of the original PHP-CS-Fixer fixers.
const SourceBase = "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/"

type Fixer interface {
	// Name is the checker identifier (the PHP-CS-Fixer FQCN) shown in reports.
	Name() string
	// SourceURL links to the original PHP-CS-Fixer rule on GitHub. Required for
	// every fixer and verified by the source checker.
	SourceURL() string
	// Fix mutates the stream and reports whether it changed anything.
	Fix(s *tokens.Stream) bool
}

// SourceURLFor derives the canonical PHP-CS-Fixer source URL from a fixer Name
// (an FQCN like `PhpCsFixer\Fixer\Semicolon\SpaceAfterSemicolonFixer`).
func SourceURLFor(name string) string {
	parts := strings.Split(name, `\`)
	if len(parts) < 2 {
		return ""
	}
	category := parts[len(parts)-2]
	class := parts[len(parts)-1]
	return SourceBase + category + "/" + class + ".php"
}
