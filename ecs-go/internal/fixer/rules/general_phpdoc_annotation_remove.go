package rules

import (
	"strings"

	"ecs-go/internal/tokens"
)

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/GeneralPhpdocAnnotationRemoveFixer.php
//
// GeneralPhpdocAnnotationRemove removes the configured phpdoc annotations. The
// annotation list is configuration-driven and empty by default (as ECS's
// psr12+common leaves it), so with the default set it is a no-op.
type GeneralPhpdocAnnotationRemove struct{}

func (GeneralPhpdocAnnotationRemove) Name() string {
	return `PhpCsFixer\Fixer\Phpdoc\GeneralPhpdocAnnotationRemoveFixer`
}

func (GeneralPhpdocAnnotationRemove) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/GeneralPhpdocAnnotationRemoveFixer.php"
}

// generalPhpdocAnnotationsToRemove is the configured annotation set. ECS's
// psr12+common configures none, so it stays empty (no-op).
var generalPhpdocAnnotationsToRemove []string

func (GeneralPhpdocAnnotationRemove) Fix(s *tokens.Stream) bool {
	if len(generalPhpdocAnnotationsToRemove) == 0 {
		return false
	}
	return applyToDocblocks(s, func(d *docblock) bool {
		kept := d.inner[:0:0]
		removed := false
		for _, l := range d.inner {
			if generalPhpdocLineHasTag(l.content, generalPhpdocAnnotationsToRemove) {
				removed = true
				continue
			}
			kept = append(kept, l)
		}
		if removed {
			d.inner = kept
		}
		return removed
	})
}

func generalPhpdocLineHasTag(content string, tags []string) bool {
	trimmed := strings.TrimLeft(content, " ")
	name := ""
	if strings.HasPrefix(trimmed, "@") {
		j := 1
		for j < len(trimmed) && (isIdentByte(trimmed[j]) || trimmed[j] == '-') {
			j++
		}
		name = trimmed[1:j]
	}
	if name == "" {
		return false
	}
	for _, t := range tags {
		if strings.EqualFold(name, t) {
			return true
		}
	}
	return false
}
