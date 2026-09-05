package rules

import (
	"strings"

	"ecs-go/internal/token"
	"ecs-go/internal/tokens"
)

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Import/NoLeadingImportSlashFixer.php
//
// NoLeadingImportSlash removes a leading backslash from an import:
// "use \Foo\Bar;" -> "use Foo\Bar;", including "use function" / "use const".
type NoLeadingImportSlash struct{}

func (NoLeadingImportSlash) Name() string {
	return `PhpCsFixer\Fixer\Import\NoLeadingImportSlashFixer`
}

func (NoLeadingImportSlash) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Import/NoLeadingImportSlashFixer.php"
}

func (NoLeadingImportSlash) Fix(s *tokens.Stream) bool {
	changed := false
	for i := 0; i < s.Len(); i++ {
		t := s.At(i)
		if t.Kind != token.Keyword || strings.ToLower(t.Value) != "use" {
			continue
		}
		j := skipWhitespace(s, i+1)
		if j < s.Len() && s.At(j).Kind == token.Keyword {
			if lw := strings.ToLower(s.At(j).Value); lw == "function" || lw == "const" {
				j = skipWhitespace(s, j+1)
			}
		}
		if j < s.Len() && s.At(j).Kind == token.Punct && s.At(j).Value == `\` {
			s.RemoveAt(j)
			changed = true
		}
	}
	return changed
}

func skipWhitespace(s *tokens.Stream, i int) int {
	for i < s.Len() && s.At(i).Kind == token.Whitespace {
		i++
	}
	return i
}
