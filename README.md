# ecs-go

A thin, token-based PHP coding-standard checker and fixer in Go, modeled on
[symplify/easy-coding-standard](https://github.com/symplify/easy-coding-standard).

This is a **skeleton**, not a full port: it ships a small standalone PHP lexer
and three fixers to prove the architecture end to end.

## Why a custom lexer

ECS (via PHP-CS-Fixer / PHP_CodeSniffer) works on a **flat, indexed, mutable
token stream** — fixers walk tokens by index and insert/replace/remove them in
place, then re-render.

[`rectorphp/php-parser-in-go`](https://github.com/rectorphp/php-parser-in-go)
does tokenize (`pkg/token`, with `T_WHITESPACE` / `T_COMMENT` / `T_DOC_COMMENT`),
but it is a port of `z7zmey/php-parser`: `parser.Parse` returns an **AST**, and
whitespace/comments hang off nodes as **`FreeFloating`** trivia — not a flat
stream. That model is the opposite of what a fixer engine wants, so this project
uses its own flat tokenizer instead. Swapping in php-parser-in-go later would
mean flattening its AST + FreeFloating back into a stream.

## Architecture

```
lexer   -> flat, lossless token slice (concat of values == source)
tokens  -> mutable index-addressable Stream (Insert/Remove/Set/Render)
fixer   -> Fixer interface: Fix(*Stream) bool
rules   -> the built-in fixers
finder  -> collect .php files, honor skips
runner  -> read -> lex -> fix -> (write) per file
reporter-> ECS-like summary
```

## Built-in rules

- `no_space_before_semicolon` — removes single-line whitespace before `;`
- `no_trailing_whitespace` — trims spaces/tabs at line ends
- `single_blank_line_at_eof` — exactly one trailing newline

## Usage

```
go build -o ecs-go .

./ecs-go testdata            # check (exit 1 if issues)
./ecs-go --fix testdata      # fix in place
./ecs-go list-checkers       # list rules
```

## Test

```
go test ./...
```

## Limits

The lexer is a pragmatic subset of PHP: string interpolation is not split,
heredoc/nowdoc are treated as generic content, and operators are emitted as
single-char `Punct` tokens. Enough for line/whitespace fixers; extend the lexer
before adding rules that need real operator or keyword semantics.
