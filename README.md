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
cd ecs-go/ecs-go   # the Go tool lives here; the Rust port is in ecs-rust/
make build         # produces ./ecs-go
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

- `sets` - enable a prepared set: `spaces`, `casing`, `psr12`, `per-cs`, `common`.
  `per-cs` mirrors PHP-CS-Fixer's `@PER-CS` (as ECS's `SetList::PER_CS`) and adds
  `single_line_empty_body` on top of the full set.
- `level` - gradual adoption: `{"spaces": N}` enables the first N rules of the
  spaces set (safest first), so you can raise coverage one step at a time.
- `rules` - enable individual fixers by name.
- `paths` / `skip` - files to scan and glob patterns to ignore.

With no config file, every fixer runs. CLI path arguments override `paths`.

## Performance

The `Performance` CI workflow runs the original PHP ECS, ecs-go, and a Rust port
(in `ecs-rust/`) over the same PSR-12 rule subset (the 35 fixers both ports
implement) on real codebases and compares wall time. All three run `--fix` in
parallel across every core; Go and Rust also produce byte-for-byte identical
output. Mean of 10 runs on a 24-core Linux box:

| codebase | .php files | ecs (PHP) | ecs-go | ecs-rust |
|---|---:|---:|---:|---:|
| laravel/framework (src) | 1696 | 4.356s | 0.085s | 0.084s |
| symfony/symfony (src) | 11581 | 5.951s | 0.560s | 0.352s |

Both compiled tools are far faster than the original PHP ECS - roughly 10-50x - and
run close to each other: ecs-go is level with the Rust port on the small tree, and
ecs-rust edges ahead on the large one. All three fix in place with diff rendering
off (ECS via `--no-diffs`; ecs-go and ecs-rust skip it in `--fix` mode), so each
does the same work: lex, fix, write.
ecs-go relaxes the GC for a batch run, presizes the lexer's token slice, and avoids
allocations in its hottest fixers; ecs-rust lexes into copy-on-write span tokens,
fixes files in parallel with rayon, and holds bytes as slices. Exact figures for
each change land in that workflow's job summary.

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
(`strict_types = 1` -> `strict_types=1`), `no_multiple_statements_per_line`
(`$a; $b;` -> two lines), `switch_case_space` + `switch_case_semicolon_to_colon`
(`case 1 ;` -> `case 1:`).

### Functions and operators

`method_argument_space` (single space after a comma in call/signature args),
`return_type_declaration` (`) : int` -> `): int`), `ternary_operator_spaces`
(`$a?$b:$c` -> `$a ? $b : $c`, nullable types left alone).

### Arrays and strings

`array_syntax` (`array(...)` -> `[...]`), `list_syntax` (`list(...)` -> `[...]`),
`whitespace_after_comma_in_array` + `no_whitespace_before_comma_in_array`,
`trailing_comma_in_multiline` (arrays), `no_trailing_comma_in_singleline`,
`no_spaces_around_offset` (`$a[ 0 ]` -> `$a[0]`), `single_quote` (double ->
single when safe), `standardize_not_equals` (`<>` -> `!=`), `no_empty_statement`
(`$a = 1;;` -> `$a = 1;`), `line_ending` (CRLF -> LF).

### More casing / operators / comments / imports

`magic_constant_casing` (`__line__` -> `__LINE__`), `magic_method_casing`,
`native_function_casing` (curated), `integer_literal_case` (`0XFF` -> `0xff`),
`object_operator_without_whitespace` (`$a -> b` -> `$a->b`), `no_empty_comment`,
`single_line_comment_spacing` (`//x` -> `// x`), `no_unused_imports`.

### Comments and blank lines

`no_trailing_whitespace_in_comment`, `no_extra_blank_lines` (collapse 2+ blank
lines to one).

### PHPDoc

`phpdoc_trim` (drop blank lines at the ends of a docblock), `phpdoc_no_empty_return`
(`@return void`/`@return null` removed), `phpdoc_scalar` (`@param integer` ->
`@param int`, `boolean` -> `bool`, `double`/`real` -> `float`). Built on a small
docblock parser.

### Imports

`single_import_per_statement`, `ordered_imports` (group class / function / const),
`blank_line_between_import_groups`, `single_line_after_imports`.

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
their opening brace on the next line (kept inline for multi-line signatures, per
PSR-12 4.5); control structures keep it on the same line after one space.
`statement_indentation` - reindents statement lines to four spaces per brace
level, leaving continuation lines (multi-line arguments, arrays, method chains)
untouched. Both build on a small statement/scope layer.

### Class members

`visibility_required` - adds an explicit `public` to methods, properties and
constants that declare none (`var $x` -> `public $x`); `single_trait_insert_per_statement`
- splits `use A, B;` inside a class into one per line; `single_class_element_per_statement`
- splits `public $a, $b;` into one per line. All walk the class body via the
scope layer, skipping trait use, enum cases and promoted constructor parameters.
`class_definition` normalizes spacing in a class header (`class  A  extends B`
-> `class A extends B`); `ordered_class_elements` groups members (trait use,
constants, properties, methods) - conservatively, skipping any class with
comments or attributes.

## License

MIT
