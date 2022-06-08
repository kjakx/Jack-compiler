use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;

#[derive(PartialEq, Default)]
pub enum Token {
    Empty(None),
    Keyword(String),
    Symbol(char),
    Identifier(String), // may contain: alphabet, number, _(underscore)
    IntConst(i16), // 0-32767
    StringConst(String),
}

#[derive(PartialEq)]
pub enum Keyword {
    Class,
    Method,
    Function,
    Constructor,
    Int,
    Boolean,
    Char,
    Void,
    Var,
    Static,
    Field,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    This,
}

pub struct Tokenizer {
    tokens: Vec<Token>,
    current_token: Token,
}

impl Tokenizer {
    pub fn new(f: File) -> Self {
        let symbol_set = HashSet::from([
            '{', '}', '(', ')', '[', ']',
            '.', ',', ';',
            '+', '-', '*', '/',
            '&', '|', '<', '>', '=', '~'
        ]);
        let reader = BufReader::new(f); // should I use BufPeekReader?
        let mut tokens = vec![];
        loop {
            let mut buffer = [0; 1];
            match reader.read_exact(&buffer) {
                Ok(_) => {
                    /* implementation plan */
                    // If buffer[0] is a /(slash), the next byte should be checked. If * or /, it is followed by a comment, so skip it.
                    // If an other symbol, and not a comment, it can immediately be added to tokens as a symbol.
                    // If a number, it is integerConstant. Read until the end of the number.
                    // If a doublequote, it is stringConstant. Read until the next doublequote appears.
                    // If an alphabet or underscore, read string as an identifier. It may be a keyword or identifier.
                },
                Err(UnexpectedEof) => { break; }, // EOF
                _ => { panic!("unexpected error"); }
            }

        }
        Tokenizer {
            tokens: tokens,
            current_token: Token::default(),
        }
    }
    pub fn has_more_tokens(&self) -> bool { unimplemented!(); }
    pub fn advance(&mut self) { unimplemented!(); }
    pub fn token_type(&self) -> Token { unimplemented!(); }
    pub fn keyword(&self) -> Keyword { unimplemented!(); }
    pub fn symbol(&self) -> char { unimplemented!(); }
    pub fn identifier(&self) -> String { unimplemented!(); }
    pub fn int_val(&self) -> i16 { unimplemented!(); }
    pub fn string_val(&self) -> String { unimplemented!(); }
}