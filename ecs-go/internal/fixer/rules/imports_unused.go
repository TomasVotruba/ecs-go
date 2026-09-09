package rules

import (
	"slices"
	"strings"

	"ecs-go/internal/token"
	"ecs-go/internal/tokens"
)

type importInfo struct {
	start, semi int
	shortLower  string
}

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Import/NoUnusedImportsFixer.php
//
// NoUnusedImports removes a top-level "use" import whose short name is never
// referenced. To stay safe it keeps any import whose name appears in a comment
// or doc comment, and skips group imports.
type NoUnusedImports struct{}

func (NoUnusedImports) Name() string {
	return `PhpCsFixer\Fixer\Import\NoUnusedImportsFixer`
}

func (NoUnusedImports) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Import/NoUnusedImportsFixer.php"
}

func (NoUnusedImports) Fix(s *tokens.Stream) bool {
	var imports []importInfo
	for i := 0; i < s.Len(); i++ {
		t := s.At(i)
		if t.Kind != token.Keyword || strings.ToLower(t.Value) != "use" || inClassLikeBody(s, i) {
			continue
		}
		j := skipWhitespace(s, i+1)
		if j < s.Len() && s.At(j).Kind == token.Punct && s.At(j).Value == "(" {
			continue // closure use
		}
		if j < s.Len() && s.At(j).Kind == token.Keyword {
			if lw := strings.ToLower(s.At(j).Value); lw == "function" || lw == "const" {
				j = skipWhitespace(s, j+1)
			}
		}

		semi, group := -1, false
		for k := i + 1; k < s.Len(); k++ {
			if s.At(k).Kind != token.Punct {
				continue
			}
			if s.At(k).Value == "{" {
				group = true
				break
			}
			if s.At(k).Value == ";" {
				semi = k
				break
			}
		}
		if group || semi < 0 {
			continue
		}

		short := importShortName(s, j, semi)
		if short == "" {
			continue
		}
		imports = append(imports, importInfo{start: i, semi: semi, shortLower: strings.ToLower(short)})
		i = semi
	}
	if len(imports) == 0 {
		return false
	}

	inImport := func(idx int) bool {
		for _, im := range imports {
			if idx >= im.start && idx <= im.semi {
				return true
			}
		}
		return false
	}
	used := map[string]bool{}
	var comments strings.Builder
	for k := 0; k < s.Len(); k++ {
		t := s.At(k)
		if t.Kind == token.Ident && !inImport(k) {
			used[strings.ToLower(t.Value)] = true
		}
		if t.Kind == token.Comment || t.Kind == token.DocComment {
			comments.WriteString(strings.ToLower(t.Value))
		}
	}
	commentText := comments.String()

	changed := false
	for _, im := range slices.Backward(imports) {
		if used[im.shortLower] || strings.Contains(commentText, im.shortLower) {
			continue
		}
		for r := im.semi; r >= im.start; r-- {
			s.RemoveAt(r)
		}
		// drop just the statement's own line break, keeping any blank line
		if im.start < s.Len() && s.At(im.start).Kind == token.Whitespace {
			if v := s.At(im.start).Value; strings.HasPrefix(v, "\n") {
				s.SetValue(im.start, v[1:])
			}
		}
		changed = true
	}
	return changed
}

// importShortName returns the imported name (the alias after "as", else the last
// path segment) of a use statement spanning (from, semi).
func importShortName(s *tokens.Stream, from, semi int) string {
	for k := from; k < semi; k++ {
		if s.At(k).Kind == token.Keyword && strings.ToLower(s.At(k).Value) == "as" {
			a := skipWhitespace(s, k+1)
			if a < semi && s.At(a).Kind == token.Ident {
				return s.At(a).Value
			}
		}
	}
	for k := semi - 1; k >= from; k-- {
		if s.At(k).Kind == token.Ident {
			return s.At(k).Value
		}
	}
	return ""
}
