// A flat, mutable, index-addressable token stream, ported from ecs-go's
// internal/tokens. Fixers read tokens by index and insert, replace or remove
// them in place; render rebuilds the source. Tokens hold spans into the borrowed
// source until a fixer rewrites one, so reading a token allocates nothing.

use crate::token::{Kind, Token, Value};

pub struct Stream<'a> {
    src: &'a [u8],
    toks: Vec<Token>,
}

impl<'a> Stream<'a> {
    pub fn new(src: &'a [u8], toks: Vec<Token>) -> Self {
        Stream { src, toks }
    }

    pub fn len(&self) -> usize {
        self.toks.len()
    }

    pub fn kind(&self, i: usize) -> Kind {
        self.toks[i].kind
    }

    // The token's bytes, resolved from the source span or the owned buffer.
    pub fn bytes(&self, i: usize) -> &[u8] {
        match &self.toks[i].value {
            Value::Span(a, b) => &self.src[*a as usize..*b as usize],
            Value::Owned(v) => v,
        }
    }

    pub fn set_owned(&mut self, i: usize, v: Vec<u8>) {
        self.toks[i].value = Value::Owned(v);
    }

    pub fn remove_at(&mut self, i: usize) {
        self.toks.remove(i);
    }

    pub fn insert_owned(&mut self, i: usize, kind: Kind, v: Vec<u8>) {
        self.toks.insert(i, Token::owned(kind, v));
    }

    pub fn render(&self) -> Vec<u8> {
        let cap: usize = (0..self.toks.len()).map(|i| self.bytes(i).len()).sum();
        let mut out = Vec::with_capacity(cap);
        for i in 0..self.toks.len() {
            out.extend_from_slice(self.bytes(i));
        }
        out
    }
}
