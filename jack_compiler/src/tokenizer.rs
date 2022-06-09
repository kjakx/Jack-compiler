use std::io::{Read, BufRead, BufReader};
use std::io::ErrorKind;
use std::fs::File;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Empty(),
    Keyword(String),
    Symbol(char),
    Identifier(String),
    IntConst(i16),
    StringConst(String),
}

#[derive(Debug, PartialEq)]
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
    pub tokens: Vec<Token>,
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
        let keyword_set = HashSet::from([
            "class", "constructor", "function", "method", "field", "static",
            "var", "int", "char", "boolean", "void", "true", "false", "null",
            "this", "let", "do", "if", "else", "while", "return"
        ]);
        let mut reader = BufReader::new(f);
        let mut tokens = vec![];
        'tokenize: loop {
            let mut ch = [0; 1];
            match reader.read_exact(&mut ch) {
                Ok(_) => {
                    match ch[0] {
                        // skip newline and ascii whitespace
                        b'\n' => { continue 'tokenize; },
                        c if c.is_ascii_whitespace() => { continue 'tokenize; },
                        c if symbol_set.contains(&char::from_u32(c as u32).unwrap()) => {
                            match c {
                                // If c is a /(slash), the next byte should be checked.
                                b'/' => {
                                    if let Ok(buf) = reader.fill_buf() { // TODO: I need more efficient way to look ahead a buffer...
                                        match buf[0] {
                                            // If / or *, it is followed by a comment, so skip it.
                                            b'/' => {
                                                let mut comment = vec![];
                                                reader.read_until(b'\n', &mut comment).unwrap();
                                            },
                                            b'*' => {
                                                // skip until next "*/".
                                                let mut comment = vec![];
                                                while let Ok(_) = reader.read_until(b'/', &mut comment) {
                                                    if comment.len() >= 2 && comment[comment.len()-2] == b'*' {
                                                        break;
                                                    }
                                                }
                                            }
                                            _ => {
                                                // If not a comment, the slash is a symbol token.
                                                tokens.push(Token::Symbol(char::from_u32(ch[0] as u32).unwrap()));
                                            }
                                        }
                                    }
                                },
                                // If the other symbol, it can immediately be added to tokens as a symbol.
                                _ => {
                                    tokens.push(Token::Symbol(char::from_u32(c as u32).unwrap()));
                                }
                            }
                        },
                        // If a number, it is an integerConstant. Read until the end of the number.
                        b'0' => {
                            tokens.push(Token::IntConst((ch[0] - b'0') as i16));
                        },
                        b'1'..=b'9' => {
                            let mut digits = vec![ch[0]];
                            if let Ok(buf) = reader.fill_buf() {
                                for d in buf.iter() {
                                    match d {
                                        b'0'..=b'9' => {
                                            digits.push(*d);
                                        },
                                        _ => {
                                            break;
                                        }
                                    }
                                }
                            }
                            reader.consume(digits.len()-1);

                            let int_const: i16 = digits
                                .into_iter()
                                .rev()
                                .map(|d| d - b'0')
                                .fold(0, |acc, d| 10*acc + d as i16);
                            tokens.push(Token::IntConst(int_const));
                        },
                        // If a doublequote, it is beginning of a stringConstant. Read until the next doublequote appears.
                        b'"' => {
                            let mut string_const = vec![];
                            reader.read_until(b'"', &mut string_const).expect("reached unexpected EOF while parsing StringConst");
                            let string_const = string_const[..string_const.len()-1].to_vec();
                            tokens.push(Token::StringConst(String::from_utf8(string_const).unwrap()));
                        },
                        // If an alphabet or underscore, it is a keyword or identifier.
                        b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                            let mut chars = vec![ch[0]];
                            if let Ok(buf) = reader.fill_buf() {
                                for c in buf.iter() {
                                    match c {
                                        b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                                            chars.push(*c);
                                        },
                                        _ => {
                                            break;
                                        }
                                    }
                                }
                            }
                            reader.consume(chars.len()-1);

                            let word: String = chars
                                .iter()
                                .map(|c| char::from_u32(*c as u32).unwrap().to_string())
                                .collect::<Vec<String>>()
                                .join("");
                            if keyword_set.contains(word.as_str()) {
                                tokens.push(Token::Keyword(word));
                            } else {
                                tokens.push(Token::Identifier(word));
                            }
                        },
                        _ => { panic!("invalid byte appeared while tokenizing"); }
                    }
                },
                Err(e) => {
                    match e.kind() {
                        ErrorKind::UnexpectedEof => { break 'tokenize; }, // reached EOF
                        _ => { panic!("unexpected error occurred while tokenizing"); }
                    }
                }
            }
        }

        Tokenizer {
            tokens: tokens,
            current_token: Token::Empty(),
        }
    }

    pub fn has_more_tokens(&self) -> bool {
        !self.tokens.is_empty()
    }

    pub fn advance(&mut self) {
        self.current_token = self.tokens.pop().unwrap_or(Token::Empty());
    }

    pub fn token_type(&self) -> Token {
        self.current_token.clone()
    }

    pub fn keyword(&self) -> Option<String> {
        match &self.current_token {
            Token::Keyword(kw) => Some(kw.to_string()),
            _ => None
        }
    }

    pub fn symbol(&self) -> Option<char> {
        match &self.current_token {
            Token::Symbol(sym) => Some(*sym),
            _ => None
        }
    }

    pub fn identifier(&self) -> Option<String> {
        match &self.current_token {
            Token::Identifier(ident) => Some(ident.to_string()),
            _ => None
        }
    }

    pub fn int_val(&self) -> Option<i16> {
        match &self.current_token {
            Token::IntConst(int_const) => Some(*int_const),
            _ => None
        }
    }

    pub fn string_val(&self) -> Option<String> {
        match &self.current_token {
            Token::StringConst(str_const) => Some(str_const.to_string()),
            _ => None
        }
    }
}
