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

// Token value is kept as raw bytes so the stream is byte-for-byte lossless,
// matching Go's use of string (bytes) rather than assuming valid UTF-8.
#[derive(Clone)]
pub struct Token {
    pub kind: Kind,
    pub value: Vec<u8>,
}

impl Token {
    pub fn new(kind: Kind, value: &[u8]) -> Self {
        Token {
            kind,
            value: value.to_vec(),
        }
    }
}
