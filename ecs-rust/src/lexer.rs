// A small, self-contained PHP tokenizer producing a flat, lossless token stream,
// ported from ecs-go's internal/lexer. Concatenating the value of each returned
// token reproduces the source byte for byte.

use crate::token::{Kind, Token, Value};

// Multi-char operators, longest first, emitted as a single Punct each.
const OPERATORS: &[&[u8]] = &[
    b"<=>", b"===", b"!==", b"**=", b"...", b"<<=", b">>=", b"??=", b"?->", b"->", b"=>", b"==",
    b"!=", b"<>", b"<=", b">=", b"&&", b"||", b"++", b"--", b"+=", b"-=", b"*=", b"/=", b".=",
    b"%=", b"&=", b"|=", b"^=", b"::", b"??", b"**", b"<<", b">>",
];

const KEYWORDS: &[&str] = &[
    "abstract", "and", "array", "as", "break", "callable", "case", "catch", "class", "clone",
    "const", "continue", "declare", "default", "do", "echo", "else", "elseif", "empty",
    "enddeclare", "endfor", "endforeach", "endif", "endswitch", "endwhile", "enum", "extends",
    "final", "finally", "fn", "for", "foreach", "function", "global", "goto", "if", "implements",
    "include", "include_once", "instanceof", "insteadof", "interface", "isset", "list", "match",
    "namespace", "new", "or", "print", "private", "protected", "public", "readonly", "require",
    "require_once", "return", "static", "switch", "throw", "trait", "try", "unset", "use", "var",
    "while", "xor", "yield",
];

fn is_keyword(word_lower: &str) -> bool {
    KEYWORDS.contains(&word_lower)
}

pub fn lex(src: &[u8]) -> Vec<Token> {
    let mut l = Lexer {
        src,
        pos: 0,
        in_php: false,
        toks: Vec::new(),
    };
    l.run();
    l.toks
}

struct Lexer<'a> {
    src: &'a [u8],
    pos: usize,
    in_php: bool,
    toks: Vec<Token>,
}

impl<'a> Lexer<'a> {
    fn emit(&mut self, k: Kind, start: usize) {
        self.toks.push(Token::span(k, start, self.pos));
    }

    fn has_prefix(&self, s: &[u8]) -> bool {
        self.src[self.pos..].starts_with(s)
    }

    fn run(&mut self) {
        while self.pos < self.src.len() {
            if self.in_php {
                self.lex_php();
            } else {
                self.lex_html();
            }
        }
    }

    fn lex_html(&mut self) {
        let start = self.pos;
        loop {
            match find(&self.src[self.pos..], b"<?") {
                None => {
                    self.pos = self.src.len();
                    break;
                }
                Some(idx) => {
                    self.pos += idx;
                    if self.is_open_tag_here() {
                        break;
                    }
                    self.pos += 2; // a "<?" that is not a PHP tag (e.g. "<?xml")
                }
            }
        }
        if self.pos > start {
            self.toks.push(Token::span(Kind::InlineHtml, start, self.pos));
        }
        if self.pos < self.src.len() {
            self.lex_open_tag();
            self.in_php = true;
        }
    }

    fn is_open_tag_here(&self) -> bool {
        if self.has_prefix(b"<?php") || self.has_prefix(b"<?=") {
            return true;
        }
        let rest = &self.src[self.pos + 2..];
        if rest.len() >= 3 && rest[..3].eq_ignore_ascii_case(b"xml") {
            return false;
        }
        true
    }

    fn lex_open_tag(&mut self) {
        let start = self.pos;
        if self.has_prefix(b"<?php") {
            self.pos += 5;
        } else if self.has_prefix(b"<?=") {
            self.pos += 3;
        } else {
            self.pos += 2;
        }
        self.emit(Kind::OpenTag, start);
    }

    fn lex_php(&mut self) {
        let start = self.pos;
        let c = self.src[self.pos];

        if self.has_prefix(b"?>") {
            self.pos += 2;
            self.emit(Kind::CloseTag, start);
            self.in_php = false;
        } else if self.has_prefix(b"<<<") {
            let end = self.scan_heredoc();
            if end > self.pos {
                self.pos = end;
                self.emit(Kind::String, start);
            } else {
                let op = self.match_operator();
                self.pos += if op > 0 { op } else { 1 };
                self.emit(Kind::Punct, start);
            }
        } else if is_space(c) {
            while self.pos < self.src.len() && is_space(self.src[self.pos]) {
                self.pos += 1;
            }
            self.emit(Kind::Whitespace, start);
        } else if c == b'$' {
            self.pos += 1;
            while self.pos < self.src.len() && is_ident(self.src[self.pos]) {
                self.pos += 1;
            }
            self.emit(Kind::Variable, start);
        } else if self.has_prefix(b"#[") {
            self.lex_attribute(start);
        } else if self.has_prefix(b"//") || c == b'#' {
            self.lex_line_comment(start);
        } else if self.has_prefix(b"/*") {
            self.lex_block_comment(start);
        } else if c == b'\'' {
            self.lex_string(start, b'\'');
        } else if c == b'"' {
            self.lex_string(start, b'"');
        } else if is_digit(c) {
            while self.pos < self.src.len() && is_number(self.src[self.pos]) {
                self.pos += 1;
            }
            self.emit(Kind::Number, start);
        } else if is_ident_start(c) {
            while self.pos < self.src.len() && is_ident(self.src[self.pos]) {
                self.pos += 1;
            }
            let kind = self.ident_kind(&self.src[start..self.pos]);
            self.emit(kind, start);
        } else {
            let op = self.match_operator();
            self.pos += if op > 0 { op } else { 1 };
            self.emit(Kind::Punct, start);
        }
    }

    fn ident_kind(&self, word: &[u8]) -> Kind {
        let prev = self.last_significant();
        if prev == b"->" || prev == b"?->" || prev == b"::" || prev == b"\\" {
            return Kind::Ident;
        }
        let lp = ascii_lower(prev);
        if lp == b"function" || lp == b"const" {
            return Kind::Ident;
        }
        if self.pos < self.src.len() && self.src[self.pos] == b'\\' {
            return Kind::Ident;
        }
        if let Ok(w) = std::str::from_utf8(word) {
            if is_keyword(&w.to_ascii_lowercase()) {
                return Kind::Keyword;
            }
        }
        Kind::Ident
    }

    fn last_significant(&self) -> &[u8] {
        for tk in self.toks.iter().rev() {
            match tk.kind {
                Kind::Whitespace | Kind::Comment | Kind::DocComment => continue,
                _ => {
                    return match &tk.value {
                        Value::Span(a, b) => &self.src[*a as usize..*b as usize],
                        Value::Owned(v) => v,
                    }
                }
            }
        }
        b""
    }

    fn match_operator(&self) -> usize {
        for op in OPERATORS {
            if self.has_prefix(op) {
                return op.len();
            }
        }
        0
    }

    fn lex_line_comment(&mut self, start: usize) {
        while self.pos < self.src.len() && self.src[self.pos] != b'\n' {
            if self.has_prefix(b"?>") {
                break;
            }
            self.pos += 1;
        }
        self.emit(Kind::Comment, start);
    }

    fn lex_block_comment(&mut self, start: usize) {
        self.pos += 2; // consume /*
        while self.pos < self.src.len() && !self.has_prefix(b"*/") {
            self.pos += 1;
        }
        if self.has_prefix(b"*/") {
            self.pos += 2;
        }
        let val = &self.src[start..self.pos];
        let mut kind = Kind::Comment;
        if val.starts_with(b"/**") && val != b"/**/" {
            kind = Kind::DocComment;
        }
        self.toks.push(Token::span(kind, start, self.pos));
    }

    fn lex_string(&mut self, start: usize, quote: u8) {
        self.pos += 1; // opening quote
        while self.pos < self.src.len() {
            let c = self.src[self.pos];
            if c == b'\\' && self.pos + 1 < self.src.len() {
                self.pos += 2;
                continue;
            }
            if c == quote {
                self.pos += 1;
                break;
            }
            self.pos += 1;
        }
        self.emit(Kind::String, start);
    }

    fn lex_attribute(&mut self, start: usize) {
        self.pos += 2; // consume "#["
        let mut depth = 1;
        while self.pos < self.src.len() && depth > 0 {
            let c = self.src[self.pos];
            if c == b'[' {
                depth += 1;
                self.pos += 1;
            } else if c == b']' {
                depth -= 1;
                self.pos += 1;
            } else if c == b'\'' || c == b'"' {
                self.skip_string(c);
            } else if self.has_prefix(b"<<<") {
                let end = self.scan_heredoc();
                if end > self.pos {
                    self.pos = end;
                } else {
                    self.pos += 1;
                }
            } else {
                self.pos += 1;
            }
        }
        self.emit(Kind::Comment, start);
    }

    fn skip_string(&mut self, quote: u8) {
        self.pos += 1; // opening quote
        while self.pos < self.src.len() {
            let c = self.src[self.pos];
            if c == b'\\' && self.pos + 1 < self.src.len() {
                self.pos += 2;
                continue;
            }
            self.pos += 1;
            if c == quote {
                return;
            }
        }
    }

    // Returns the end offset of a heredoc/nowdoc starting at "<<<", or 0 when the
    // "<<<" is not a valid heredoc opener (Go returns -1; here 0 means "not one",
    // since a real heredoc always ends past self.pos).
    fn scan_heredoc(&self) -> usize {
        let src = self.src;
        let mut p = self.pos + 3;
        while p < src.len() && (src[p] == b' ' || src[p] == b'\t') {
            p += 1;
        }
        let mut quote = 0u8;
        if p < src.len() && (src[p] == b'\'' || src[p] == b'"') {
            quote = src[p];
            p += 1;
        }
        let label_start = p;
        while p < src.len() && is_ident(src[p]) {
            p += 1;
        }
        if p == label_start {
            return 0; // no label -> not a heredoc
        }
        let label = &src[label_start..p];
        if quote != 0 {
            if p >= src.len() || src[p] != quote {
                return 0;
            }
            p += 1;
        }
        while p < src.len() && src[p] != b'\n' {
            p += 1;
        }
        if p >= src.len() {
            return 0;
        }
        while p < src.len() {
            p += 1; // step past the newline to the start of the next line
            let mut line_start = p;
            while line_start < src.len() && (src[line_start] == b' ' || src[line_start] == b'\t') {
                line_start += 1;
            }
            if src[line_start..].starts_with(label) {
                let after = line_start + label.len();
                if after >= src.len() || !is_ident(src[after]) {
                    return after;
                }
            }
            while p < src.len() && src[p] != b'\n' {
                p += 1;
            }
            if p >= src.len() {
                return src.len();
            }
        }
        src.len()
    }
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|w| w == needle)
}

fn ascii_lower(b: &[u8]) -> Vec<u8> {
    b.to_ascii_lowercase()
}

fn is_space(c: u8) -> bool {
    c == b' ' || c == b'\t' || c == b'\n' || c == b'\r'
}
fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
}
fn is_number(c: u8) -> bool {
    is_digit(c)
        || c == b'.'
        || c == b'_'
        || c == b'x'
        || c == b'X'
        || (c >= b'a' && c <= b'f')
        || (c >= b'A' && c <= b'F')
}
fn is_ident_start(c: u8) -> bool {
    c == b'_' || (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || c >= 0x80
}
fn is_ident(c: u8) -> bool {
    is_ident_start(c) || is_digit(c)
}
