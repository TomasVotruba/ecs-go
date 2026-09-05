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

- `sets` - enable a prepared set (`spaces`).
- `level` - gradual adoption: `{"spaces": N}` enables the first N rules of the
  spaces set (safest first), so you can raise coverage one step at a time.
- `rules` - enable individual fixers by name.
- `paths` / `skip` - files to scan and glob patterns to ignore.

With no config file, every fixer runs. CLI path arguments override `paths`.

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

## License

MIT
