// Token kinds and the Token struct, mirroring ecs-go's internal/token.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    OpenTag,    // <?php <?= <?
    CloseTag,   // ?>
    InlineHtml, // text outside PHP tags
    Whitespace, // spaces, tabs, newlines
    Comment,    // // # /* */
    DocComment, // /** */
    Variable,   // $foo
    Ident,      // names (T_STRING): function/class names, constants, types
    Keyword,    // reserved words
    Number,
    String, // '...' "..."
    Punct,  // operators, braces, ; , etc.
}

// A token's bytes are either a span into the original source (the common case,
// zero-allocation) or owned bytes produced when a fixer rewrites the token. This
// copy-on-write model keeps lexing allocation-free while staying byte-lossless.
#[derive(Clone)]
pub enum Value {
    Span(u32, u32), // [start, end) byte offsets into the source
    Owned(Vec<u8>),
}

#[derive(Clone)]
pub struct Token {
    pub kind: Kind,
    pub value: Value,
}

impl Token {
    pub fn span(kind: Kind, start: usize, end: usize) -> Self {
        Token {
            kind,
            value: Value::Span(start as u32, end as u32),
        }
    }

    pub fn owned(kind: Kind, bytes: Vec<u8>) -> Self {
        Token {
            kind,
            value: Value::Owned(bytes),
        }
    }
}
