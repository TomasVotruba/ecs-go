// Package set groups fixers into named, ordered sets, mirroring ECS prepared
// sets and gradual levels.
package set

import (
	"sort"

	"ecs-go/internal/fixer"
	"ecs-go/internal/fixer/rules"
)

// Spaces is the ordered spaces set (safest first), matching ECS SpacesLevel.
func Spaces() []fixer.Fixer { return rules.All() }

var byName = map[string]func() []fixer.Fixer{
	"spaces": Spaces,
	"common": Spaces,
}

// Get returns the fixers of a named set.
func Get(name string) ([]fixer.Fixer, bool) {
	build, ok := byName[name]
	if !ok {
		return nil, false
	}
	return build(), true
}

// SpacesLevel returns the first n rules of the spaces set, for gradual adoption
// (withSpacesLevel in ECS). n is clamped to the set size.
func SpacesLevel(n int) []fixer.Fixer {
	all := Spaces()
	if n < 0 {
		n = 0
	}
	if n > len(all) {
		n = len(all)
	}
	return all[:n]
}

// Names lists the available set names, sorted.
func Names() []string {
	names := make([]string, 0, len(byName))
	for name := range byName {
		names = append(names, name)
	}
	sort.Strings(names)
	return names
}
