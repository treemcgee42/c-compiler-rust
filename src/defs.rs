/*
 * defs.rs
 * 
 * Varun Malladi
 */

pub struct Token {
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Token {
        Token { kind }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    EOF,
    TPlus,
    TMinus,
    TStar,
    TSlash,
    TIntLit(u32),
}