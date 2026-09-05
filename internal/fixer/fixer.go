// Package fixer defines the Fixer contract. A fixer inspects a token stream and
// mutates it in place, mirroring ECS/PHP-CS-Fixer fixers.
package fixer

import "ecs-go/internal/tokens"

type Fixer interface {
	// Name is the checker identifier shown in reports and list-checkers.
	Name() string
	// Fix mutates the stream and reports whether it changed anything.
	Fix(s *tokens.Stream) bool
}
