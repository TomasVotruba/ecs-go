package rules

import (
	"regexp"
	"strings"

	"ecs-go/internal/token"
	"ecs-go/internal/tokens"
)

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/NoEmptyPhpdocFixer.php
//
// NoEmptyPhpdoc removes a docblock whose body is empty.
type NoEmptyPhpdoc struct{}

func (NoEmptyPhpdoc) Name() string {
	return `PhpCsFixer\Fixer\Phpdoc\NoEmptyPhpdocFixer`
}

func (NoEmptyPhpdoc) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/NoEmptyPhpdocFixer.php"
}

func (NoEmptyPhpdoc) Fix(s *tokens.Stream) bool {
	changed := false
	for i := 0; i < s.Len(); i++ {
		t := s.At(i)
		if t.Kind != token.DocComment {
			continue
		}
		if !docblockIsEmpty(t.Value) {
			continue
		}
		s.RemoveAt(i)
		changed = true
		// Drop one newline of the following whitespace, like removing a blank line.
		if i < s.Len() && s.At(i).Kind == token.Whitespace && strings.HasPrefix(s.At(i).Value, "\n") {
			if v := s.At(i).Value[1:]; v == "" {
				s.RemoveAt(i)
			} else {
				s.SetValue(i, v)
			}
		}
		i--
	}
	return changed
}

// docblockIsEmpty reports whether a docblock has no textual content, ignoring
// the delimiters, "*" line markers and whitespace.
func docblockIsEmpty(v string) bool {
	if !strings.HasPrefix(v, "/**") || !strings.HasSuffix(v, "*/") || len(v) < 5 {
		return false
	}
	inner := strings.Map(func(r rune) rune {
		switch r {
		case '*', ' ', '\t', '\n', '\r':
			return -1
		}
		return r
	}, v[3:len(v)-2])
	return inner == ""
}

var phpdocTypeKeywords = map[string]bool{
	"array": true, "bool": true, "callable": true, "false": true, "float": true,
	"int": true, "iterable": true, "mixed": true, "null": true, "object": true,
	"parent": true, "self": true, "static": true, "string": true, "true": true,
	"void": true, "never": true, "$this": true,
}

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/PhpdocTypesFixer.php
//
// PhpdocTypes lowercases known phpdoc type keywords to their canonical form.
type PhpdocTypes struct{}

func (PhpdocTypes) Name() string {
	return `PhpCsFixer\Fixer\Phpdoc\PhpdocTypesFixer`
}

func (PhpdocTypes) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/PhpdocTypesFixer.php"
}

func (PhpdocTypes) Fix(s *tokens.Stream) bool {
	return applyToDocblocks(s, func(d *docblock) bool {
		changed := false
		for i, l := range d.inner {
			trimmed := strings.TrimLeft(l.content, " ")
			m := phpdocTypeTagRe.FindStringSubmatch(trimmed)
			if m == nil {
				continue
			}
			newType := normalizePhpdocTypeCase(m[2])
			if newType == m[2] {
				continue
			}
			lead := l.content[:len(l.content)-len(trimmed)]
			d.inner[i].content = lead + m[1] + newType + m[3]
			changed = true
		}
		return changed
	})
}

// normalizePhpdocTypeCase lowercases keyword bases in a phpdoc type, handling
// nullables, unions and array suffixes. Non-keyword class names are left as-is.
func normalizePhpdocTypeCase(typ string) string {
	nullable := strings.HasPrefix(typ, "?")
	body := strings.TrimPrefix(typ, "?")
	parts := strings.Split(body, "|")
	for i, p := range parts {
		base, suffix := p, ""
		for strings.HasSuffix(base, "[]") {
			base = base[:len(base)-2]
			suffix = "[]" + suffix
		}
		low := strings.ToLower(base)
		if base != low && phpdocTypeKeywords[low] {
			parts[i] = low + suffix
		}
	}
	out := strings.Join(parts, "|")
	if nullable {
		out = "?" + out
	}
	return out
}

var aliasTagRe = regexp.MustCompile(`^@(type|link)\b`)

var aliasTagMap = map[string]string{"type": "var", "link": "see"}

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/PhpdocNoAliasTagFixer.php
//
// PhpdocNoAliasTag rewrites alias tags: @type -> @var, @link -> @see.
type PhpdocNoAliasTag struct{}

func (PhpdocNoAliasTag) Name() string {
	return `PhpCsFixer\Fixer\Phpdoc\PhpdocNoAliasTagFixer`
}

func (PhpdocNoAliasTag) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/PhpdocNoAliasTagFixer.php"
}

func (PhpdocNoAliasTag) Fix(s *tokens.Stream) bool {
	return applyToDocblocks(s, func(d *docblock) bool {
		changed := false
		for i, l := range d.inner {
			trimmed := strings.TrimLeft(l.content, " ")
			m := aliasTagRe.FindStringSubmatch(trimmed)
			if m == nil {
				continue
			}
			lead := l.content[:len(l.content)-len(trimmed)]
			d.inner[i].content = lead + "@" + aliasTagMap[m[1]] + trimmed[len(m[0]):]
			changed = true
		}
		return changed
	})
}

var noPackageRe = regexp.MustCompile(`^@(?:package|subpackage)\b`)

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/PhpdocNoPackageFixer.php
//
// PhpdocNoPackage removes @package and @subpackage tag lines.
type PhpdocNoPackage struct{}

func (PhpdocNoPackage) Name() string {
	return `PhpCsFixer\Fixer\Phpdoc\PhpdocNoPackageFixer`
}

func (PhpdocNoPackage) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/PhpdocNoPackageFixer.php"
}

func (PhpdocNoPackage) Fix(s *tokens.Stream) bool {
	return removeDocLines(s, noPackageRe)
}

var noAccessRe = regexp.MustCompile(`^@access\b`)

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/PhpdocNoAccessFixer.php
//
// PhpdocNoAccess removes @access tag lines.
type PhpdocNoAccess struct{}

func (PhpdocNoAccess) Name() string {
	return `PhpCsFixer\Fixer\Phpdoc\PhpdocNoAccessFixer`
}

func (PhpdocNoAccess) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/PhpdocNoAccessFixer.php"
}

func (PhpdocNoAccess) Fix(s *tokens.Stream) bool {
	return removeDocLines(s, noAccessRe)
}

// removeDocLines drops every inner line whose tag matches re (checked at the
// start of the content, after any leading spaces).
func removeDocLines(s *tokens.Stream, re *regexp.Regexp) bool {
	return applyToDocblocks(s, func(d *docblock) bool {
		kept := d.inner[:0:0]
		removed := false
		for _, l := range d.inner {
			if re.MatchString(strings.TrimLeft(l.content, " ")) {
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

var singleLineVarRe = regexp.MustCompile(`^(@(?:var|type|param))\s+([^\s$]+)\s*(\$\S+)?\s*(.*)$`)

var phpdocWsRe = regexp.MustCompile(`\s+`)

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/PhpdocSingleLineVarSpacingFixer.php
//
// PhpdocSingleLineVarSpacing normalizes spacing in single-line @var/@type/@param docblocks.
type PhpdocSingleLineVarSpacing struct{}

func (PhpdocSingleLineVarSpacing) Name() string {
	return `PhpCsFixer\Fixer\Phpdoc\PhpdocSingleLineVarSpacingFixer`
}

func (PhpdocSingleLineVarSpacing) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/PhpdocSingleLineVarSpacingFixer.php"
}

func (PhpdocSingleLineVarSpacing) Fix(s *tokens.Stream) bool {
	return applyToDocblocks(s, func(d *docblock) bool {
		if !d.single || len(d.inner) == 0 {
			return false
		}
		content := d.inner[0].content
		m := singleLineVarRe.FindStringSubmatch(content)
		if m == nil {
			return false
		}
		out := []string{m[1], m[2]}
		if m[3] != "" {
			out = append(out, m[3])
		}
		normalized := strings.Join(out, " ")
		if rest := strings.TrimSpace(phpdocWsRe.ReplaceAllString(m[4], " ")); rest != "" {
			normalized += " " + rest
		}
		if normalized == content {
			return false
		}
		d.inner[0].content = normalized
		return true
	})
}

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/PhpdocTrimConsecutiveBlankLineSeparationFixer.php
//
// PhpdocTrimConsecutiveBlankLineSeparation collapses runs of blank inner lines into one.
type PhpdocTrimConsecutiveBlankLineSeparation struct{}

func (PhpdocTrimConsecutiveBlankLineSeparation) Name() string {
	return `PhpCsFixer\Fixer\Phpdoc\PhpdocTrimConsecutiveBlankLineSeparationFixer`
}

func (PhpdocTrimConsecutiveBlankLineSeparation) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/PhpdocTrimConsecutiveBlankLineSeparationFixer.php"
}

func (PhpdocTrimConsecutiveBlankLineSeparation) Fix(s *tokens.Stream) bool {
	return applyToDocblocks(s, func(d *docblock) bool {
		kept := d.inner[:0:0]
		prevBlank := false
		changed := false
		for _, l := range d.inner {
			blank := strings.TrimSpace(l.content) == ""
			if blank && prevBlank {
				changed = true
				continue
			}
			kept = append(kept, l)
			prevBlank = blank
		}
		if changed {
			d.inner = kept
		}
		return changed
	})
}

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/NoBlankLinesAfterPhpdocFixer.php
//
// NoBlankLinesAfterPhpdoc removes blank lines between a docblock and the code it documents.
type NoBlankLinesAfterPhpdoc struct{}

func (NoBlankLinesAfterPhpdoc) Name() string {
	return `PhpCsFixer\Fixer\Phpdoc\NoBlankLinesAfterPhpdocFixer`
}

func (NoBlankLinesAfterPhpdoc) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Phpdoc/NoBlankLinesAfterPhpdocFixer.php"
}

func (NoBlankLinesAfterPhpdoc) Fix(s *tokens.Stream) bool {
	changed := false
	for i := 0; i < s.Len(); i++ {
		if s.At(i).Kind != token.DocComment {
			continue
		}
		j := i + 1
		if j >= s.Len() || s.At(j).Kind != token.Whitespace {
			continue
		}
		v := s.At(j).Value
		if strings.Count(v, "\n") < 2 {
			continue
		}
		// Keep the last newline plus the following statement's indentation.
		nv := v[strings.LastIndexByte(v, '\n'):]
		if nv != v {
			s.SetValue(j, nv)
			changed = true
		}
	}
	return changed
}
