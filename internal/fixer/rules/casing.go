package rules

import (
	"strings"

	"ecs-go/internal/token"
	"ecs-go/internal/tokens"
)

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Casing/LowercaseKeywordsFixer.php
//
// LowercaseKeywords lowercases PHP keywords (FUNCTION -> function).
type LowercaseKeywords struct{}

func (LowercaseKeywords) Name() string {
	return `PhpCsFixer\Fixer\Casing\LowercaseKeywordsFixer`
}

func (LowercaseKeywords) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Casing/LowercaseKeywordsFixer.php"
}

func (LowercaseKeywords) Fix(s *tokens.Stream) bool {
	changed := false
	for i := range s.Len() {
		t := s.At(i)
		if t.Kind != token.Keyword {
			continue
		}
		if lower := strings.ToLower(t.Value); lower != t.Value {
			s.SetValue(i, lower)
			changed = true
		}
	}
	return changed
}

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Casing/ConstantCaseFixer.php
//
// ConstantCase lowercases the true, false and null constants.
type ConstantCase struct{}

func (ConstantCase) Name() string {
	return `PhpCsFixer\Fixer\Casing\ConstantCaseFixer`
}

func (ConstantCase) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Casing/ConstantCaseFixer.php"
}

func (ConstantCase) Fix(s *tokens.Stream) bool {
	changed := false
	for i := range s.Len() {
		t := s.At(i)
		if t.Kind != token.Ident {
			continue
		}
		lower := strings.ToLower(t.Value)
		if lower != "true" && lower != "false" && lower != "null" {
			continue
		}
		if memberPrev(s, i) {
			continue // $obj->true, Foo::null - a member name, not the constant
		}
		if lower != t.Value {
			s.SetValue(i, lower)
			changed = true
		}
	}
	return changed
}

// PHP-CS-Fixer: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Casing/LowercaseStaticReferenceFixer.php
//
// LowercaseStaticReference lowercases self, static and parent.
type LowercaseStaticReference struct{}

func (LowercaseStaticReference) Name() string {
	return `PhpCsFixer\Fixer\Casing\LowercaseStaticReferenceFixer`
}

func (LowercaseStaticReference) SourceURL() string {
	return "https://github.com/PHP-CS-Fixer/PHP-CS-Fixer/blob/master/src/Fixer/Casing/LowercaseStaticReferenceFixer.php"
}

func (LowercaseStaticReference) Fix(s *tokens.Stream) bool {
	changed := false
	for i := range s.Len() {
		t := s.At(i)
		if t.Kind != token.Ident && t.Kind != token.Keyword {
			continue
		}
		lower := strings.ToLower(t.Value)
		if lower != "self" && lower != "static" && lower != "parent" {
			continue
		}
		if lower != t.Value {
			s.SetValue(i, lower)
			changed = true
		}
	}
	return changed
}
