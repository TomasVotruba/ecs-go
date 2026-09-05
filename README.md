# ecs-go

Fast, token-based PHP coding-standard checker and fixer - an [ECS](https://github.com/symplify/easy-coding-standard)-style
tool written in Go. Runs across all CPU cores by default.

## Install

Via Composer (exposes `vendor/bin/ecs-go`):

```bash
composer require tomasvotruba/ecs-go --dev
```

Or clone and build (requires Go):

```bash
git clone https://github.com/TomasVotruba/ecs-go.git
cd ecs-go
make build      # produces ./ecs-go
```

## Usage

Check your code (reports a diff of what would change, exit code 1 if issues):

```bash
vendor/bin/ecs-go src tests
```

Fix in place:

```bash
vendor/bin/ecs-go --fix src tests
```

List the active fixers:

```bash
vendor/bin/ecs-go list-checkers
```

## Configuration

Drop an `ecs-go.json` in your project root (auto-loaded, or point at one with
`--config`):

```json
{
    "paths": ["src", "tests"],
    "skip": ["*/Fixture/*"],
    "sets": ["spaces"],
    "level": {"spaces": 6}
}
```

- `sets` - enable a prepared set: `spaces`, `casing`, `psr12`, `common`.
- `level` - gradual adoption: `{"spaces": N}` enables the first N rules of the
  spaces set (safest first), so you can raise coverage one step at a time.
- `rules` - enable individual fixers by name.
- `paths` / `skip` - files to scan and glob patterns to ignore.

With no config file, every fixer runs. CLI path arguments override `paths`.

## PSR-12

The `psr12` set implements the token-safe part of PHP-CS-Fixer's `@PSR-12`:
casing (keywords, constants, static references, casts), operator spacing
(assignment, arrow, comparison and logical operators), parenthesis and
language-construct spacing, `else if` -> `elseif`, `declare` normalization,
leading import slash removal, one import per statement, blank lines before a
namespace, no blank lines after a class opening, and tab-to-space indentation.

Rules that need full statement/AST analysis are still pending and are the
natural next step (adopting an AST such as php-parser-in-go): `braces_position`,
`method_argument_space` (multiline), `ordered_imports`, `visibility_required`,
`ordered_class_elements`, and blank-line rules around namespaces and imports.

## What it looks like

```
1) src/Foo.php

    ---------- begin diff ----------
@@ @@
 <?php
-    namespace App;
+namespace App;
-    $count=1;$total=2;
+    $count=1; $total=2;
    ----------- end diff -----------

Applied checkers:

 * PhpCsFixer\Fixer\NamespaceNotation\NoLeadingNamespaceWhitespaceFixer
 * PhpCsFixer\Fixer\Semicolon\SpaceAfterSemicolonFixer

 [WARNING] 1 error is fixable! Just add "--fix" to console command and rerun to apply.
```

## Fixers

**No leading namespace whitespace**

```diff
-    namespace App;
+namespace App;
```

**Blank line after opening tag**

```diff
 <?php
+
 declare(strict_types=1);
```

**No single-line whitespace before semicolons**

```diff
-$name = 'Rector' ;
+$name = 'Rector';
```

**Space after semicolon**

```diff
-$a = 1;$b = 2;
+$a = 1; $b = 2;
```

**Binary operator spaces** (single space around `=` and the `=>` arrow)

```diff
-$map = ['a'=>1, 'b'=>2];
+$map = ['a' => 1, 'b' => 2];
```

**Concat space** (single space around `.`)

```diff
-$name = $first.' '.$last;
+$name = $first . ' ' . $last;
```

**Cast spaces** (no inner space, single space after a cast)

```diff
-$id = (int)$value;
+$id = (int) $value;
```

**No whitespace in blank line** (a blank line full of spaces becomes truly empty)

```diff
 $a = 1;
-····
+
 $b = 2;
```

**No trailing whitespace**

```diff
-$a = 1;····
+$a = 1;
```

**Single blank line at end of file** (collapses trailing blank lines to exactly one newline)

### Casing

`lowercase_keywords`, `constant_case` (`TRUE` -> `true`), `lowercase_static_reference`
(`SELF` -> `self`), `lowercase_cast`, `short_scalar_cast` (`(integer)` -> `(int)`).

### Language constructs

`single_space_around_construct` (`if(` -> `if (`), `no_spaces_after_function_name`
(`foo ()` -> `foo()`), `spaces_inside_parentheses` (`( $a )` -> `($a)`),
`unary_operator_spaces` (`$i ++` -> `$i++`), `elseif` (`else if` -> `elseif`),
`no_leading_import_slash` (`use \Foo` -> `use Foo`), `declare_equal_normalize`
(`strict_types = 1` -> `strict_types=1`).

### Structural

`single_import_per_statement` (`use A, B;` -> `use A;` / `use B;`),
`single_line_after_imports`, `blank_lines_before_namespace`,
`blank_line_after_namespace`, `no_blank_lines_after_class_opening`,
`indentation_type` (leading tabs -> four spaces).

### PHP tags

`full_opening_tag` (`<?` -> `<?php`), `no_closing_tag` (strips a trailing `?>`
from a pure-PHP file).

### Braces

`braces_position` - PSR-12 brace placement: classes and named functions get
their opening brace on the next line; control structures keep it on the same
line after one space. Built on a small statement/scope layer that classifies
each brace by its construct.

### Class members

`visibility_required` - adds an explicit `public` to methods, properties and
constants that declare none (`var $x` -> `public $x`); `single_trait_insert_per_statement`
- splits `use A, B;` inside a class into one per line. Both walk the class body
via the scope layer, skipping trait use, enum cases and promoted constructor
parameters.

## License

MIT
