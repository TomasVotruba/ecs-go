// A flat, mutable, index-addressable token stream, ported from ecs-go's
// internal/tokens. Fixers read tokens by index and insert, replace or remove
// them in place; render rebuilds the source.

use crate::token::Token;

pub struct Stream {
    pub toks: Vec<Token>,
}

impl Stream {
    pub fn new(toks: Vec<Token>) -> Self {
        Stream { toks }
    }

    pub fn len(&self) -> usize {
        self.toks.len()
    }

    pub fn at(&self, i: usize) -> &Token {
        &self.toks[i]
    }

    pub fn set_value(&mut self, i: usize, v: &[u8]) {
        self.toks[i].value = v.to_vec();
    }

    pub fn remove_at(&mut self, i: usize) {
        self.toks.remove(i);
    }

    pub fn insert_at(&mut self, i: usize, t: Token) {
        self.toks.insert(i, t);
    }

    pub fn render(&self) -> Vec<u8> {
        let cap = self.toks.iter().map(|t| t.value.len()).sum();
        let mut out = Vec::with_capacity(cap);
        for t in &self.toks {
            out.extend_from_slice(&t.value);
        }
        out
    }
}
