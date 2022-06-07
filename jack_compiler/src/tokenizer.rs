use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(PartialEq)]
pub enum TokenType {
    Keyword,
    Symbol,
    Identifier,
    IntConst,
    StringConst,
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
    tokens: Vec<String>,
    current_token: String,
}

impl Tokenizer {
    pub fn new(f: File) -> Self { unimplemented!(); }
    pub fn has_more_tokens(&self) -> bool { unimplemented!(); }
    pub fn advance(&mut self) { unimplemented!(); }
    pub fn token_type(&self) -> TokenType { unimplemented!(); }
    pub fn keyword(&self) -> Keyword { unimplemented!(); }
    pub fn symbol(&self) -> char { unimplemented!(); }
    pub fn identifier(&self) -> String { unimplemented!(); }
    pub fn int_val(&self) -> i16 { unimplemented!(); }
    pub fn string_val(&self) -> String { unimplemented!(); }
}