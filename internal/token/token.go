package token

// Kind is the lexical category of a token.
type Kind int

const (
	Unknown Kind = iota
	OpenTag      // <?php <?= <?
	CloseTag     // ?>
	InlineHTML   // text outside PHP tags
	Whitespace   // spaces, tabs, newlines
	Comment      // // # /* */
	DocComment   // /** */
	Variable     // $foo
	Ident        // keywords and names
	Number
	String // '...' "..."
	Punct  // operators, braces, ; , etc.
	EOF
)

var kindNames = map[Kind]string{
	Unknown:    "Unknown",
	OpenTag:    "OpenTag",
	CloseTag:   "CloseTag",
	InlineHTML: "InlineHTML",
	Whitespace: "Whitespace",
	Comment:    "Comment",
	DocComment: "DocComment",
	Variable:   "Variable",
	Ident:      "Ident",
	Number:     "Number",
	String:     "String",
	Punct:      "Punct",
	EOF:        "EOF",
}

func (k Kind) String() string {
	if n, ok := kindNames[k]; ok {
		return n
	}
	return "Kind(?)"
}

// Token is a single lexical unit. The stream is lossless: concatenating every
// Value in order reproduces the original source byte for byte.
type Token struct {
	Kind  Kind
	Value string
	Pos   int // byte offset in source
}
