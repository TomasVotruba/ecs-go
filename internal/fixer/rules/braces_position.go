package rules

import (
	"ecs-go/internal/token"
	"ecs-go/internal/tokens"
)

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Basic/BracesPositionFixer.php
//
// BracesPosition places opening braces per PSR-12: classes/interfaces/traits/
// enums and named functions/methods get their "{" on the next line, aligned
// with the declaration; control structures keep it on the same line after a
// single space. Closures and free blocks are left alone.
type BracesPosition struct{}

func (BracesPosition) Name() string {
	return `PhpCsFixer\Fixer\Basic\BracesPositionFixer`
}

func (BracesPosition) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Basic/BracesPositionFixer.php"
}

func (BracesPosition) Fix(s *tokens.Stream) bool {
	changed := false
	for i := 0; i < s.Len(); i++ {
		if s.At(i).Kind != token.Punct || s.At(i).Value != "{" {
			continue
		}
		kind, keyword := classifyBrace(s, i)
		switch kind {
		case braceClassLike, braceFunctionDecl:
			want := "\n" + lineIndent(s, keyword)
			if i > 0 && s.At(i-1).Kind == token.Whitespace {
				if s.At(i-1).Value != want {
					s.SetValue(i-1, want)
					changed = true
				}
			} else {
				s.InsertAt(i, token.Token{Kind: token.Whitespace, Value: want})
				i++
				changed = true
			}
		case braceControl:
			if i > 0 && s.At(i-1).Kind == token.Whitespace {
				if s.At(i-1).Value != " " {
					s.SetValue(i-1, " ")
					changed = true
				}
			} else {
				s.InsertAt(i, token.Token{Kind: token.Whitespace, Value: " "})
				i++
				changed = true
			}
		}
	}
	return changed
}
