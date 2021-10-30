/*
 * lexer.rs
 * 
 * Rooney Malladi
 */

use std::str::Chars;
use crate::defs::{Token, TokenKind};

pub struct Cursor<'a> {
    chars: Chars<'a>,   // iterator over chars
    prev: char,
    putback: Option<char>,
    line: usize,
}

impl<'a> Cursor<'a> {
    pub fn from_str<'b>(s: &'b str) -> Cursor<'b> {
        Cursor {
            chars: s.chars(),
            prev: '\0',
            putback: None,
            line: 1,
        }
    }

    pub fn get_next_token(&mut self) -> Token {
        let c: Option<char> = self.skip();

        match c {
            None => Token::new(TokenKind::EOF),
            Some('+') => Token::new(TokenKind::TPlus),
            Some('-') => Token::new(TokenKind::TMinus),
            Some('*') => Token::new(TokenKind::TStar),
            Some('/') => Token::new(TokenKind::TSlash),
            _ => {
                if c.unwrap().is_ascii_digit() {
                    let int_value = self.scan_int();
                    return Token::new(TokenKind::TIntLit(int_value));
                } else { 
                    panic!("Unrecognized character {} on line {}.", c.unwrap(), self.line); 
                }
            }
        }
    }

    pub fn putback_char(&mut self, c: char) {
        self.putback = Some(c);
    }

    // ---- Advance iterator until next important character. The first nontrivial character is in self.prev, ----
    // ---- so the iterator is actually on the first nontrivial character (the next character is the second -----
    // ---- nontrivial character) -------------------------------------------------------------------------------
    fn skip(&mut self) -> Option<char> {
        loop {
            let maybe_c: Option<char> = self.advance();
            match maybe_c {
                Some(c) => { if !(is_whitespace(c)) { return Some(c); }}
                None => { return None; },
            }
        }
    }

    // ---- Get the next character by consuming the character iterator ----
    // ---- Returns None if there is nothing left in the iterator ---------
    fn advance(&mut self) -> Option<char> {
        match self.putback {
            Some(c) => {
                self.putback = None;
                Some(c)
            }
            None => {
                let c = self.chars.next()?;     // None returned if nothing is left
                self.prev = c;
                if c == '\n' { self.line += 1; }
                Some(c)
            }
        }
    }

    fn scan_int(&mut self) -> u32 {
        let mut val = 0;
    
        loop {
            let c = self.prev;
            match c.to_digit(10) {
                Some(d) => { 
                    val = val*10 + d; 
                    self.advance();
                }
                None => { 
                    self.putback_char(c); 
                    break;
                }
            }
        }
    
        return val;
    }
}

fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        
        ' '
        | '\n'
        | '\t'
        | '\r'
    )
}

