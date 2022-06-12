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
            '+', '-', '*', '/', '&', '|', '~',
            '<', '>', '=', '.', ',', ';'
        ]);
        let keyword_set = HashSet::from([
            "class", "constructor", "function", "method", "field", "static",
            "var", "int", "char", "boolean", "void", "true", "false", "null",
            "this", "let", "do", "if", "else", "while", "return"
        ]);

        let mut tokens = vec![];
        let mut reader = BufReader::new(f);
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
                                            b'/' => { // one line comment
                                                reader.consume(1);
                                                let mut comment = vec![];
                                                reader.read_until(b'\n', &mut comment).unwrap();
                                            },
                                            b'*' => {
                                                reader.consume(1);
                                                let mut comment = vec![];
                                                while let Ok(_) = reader.read_until(b'/', &mut comment) {
                                                    if comment.len() >= 2 && comment[comment.len()-2] == b'*' {
                                                        break;
                                                    }
                                                }
                                            },
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
                                .map(|d| (d - b'0') as i16)
                                .fold(0, |acc, d| 10*acc + d);
                            tokens.push(Token::IntConst(int_const));
                        },
                        // If a doublequote, it is beginning of a stringConstant. Read until the next doublequote appears.
                        b'"' => {
                            let mut string_const = vec![];
                            reader.read_until(b'"', &mut string_const).expect("reached unexpected EOF while parsing StringConst");
                            string_const.pop().unwrap(); // pop '"'
                            tokens.push(Token::StringConst(String::from_utf8(string_const).unwrap()));
                        },
                        // If an alphabet or underscore, it is a keyword or identifier.
                        b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                            let mut chars = vec![ch[0]];
                            if let Ok(buf) = reader.fill_buf() {
                                for c in buf.iter() {
                                    match c {
                                        b'0'..= b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                                            chars.push(*c);
                                        },
                                        _ => {
                                            break;
                                        }
                                    }
                                }
                            }
                            reader.consume(chars.len()-1);

                            let word = std::str::from_utf8(&chars).unwrap().to_string();
                            if keyword_set.contains(word.as_str()) {
                                tokens.push(Token::Keyword(word));
                            } else {
                                tokens.push(Token::Identifier(word));
                            }
                        },
                        _ => { panic!("invalid byte appeared while tokenizing: {:0x}", ch[0]); }
                    }
                },
                Err(e) => {
                    match e.kind() {
                        ErrorKind::UnexpectedEof => { break 'tokenize; }, // reached EOF
                        _ => { panic!("unexpected error occurred while tokenizing: {}", e); }
                    }
                }
            }
        }

        let tokens = tokens.into_iter().rev().collect();

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

    pub fn keyword(&self) -> Option<Keyword> {
        match &self.current_token {
            Token::Keyword(kw) => {
                match kw.as_str() {
                    "class"         => Some(Keyword::Class),
                    "constructor"   => Some(Keyword::Constructor),
                    "function"      => Some(Keyword::Function),
                    "method"        => Some(Keyword::Method),
                    "field"         => Some(Keyword::Field),
                    "static"        => Some(Keyword::Static),
                    "var"           => Some(Keyword::Var),
                    "int"           => Some(Keyword::Int),
                    "char"          => Some(Keyword::Char),
                    "boolean"       => Some(Keyword::Boolean),
                    "void"          => Some(Keyword::Void),
                    "true"          => Some(Keyword::True),
                    "false"         => Some(Keyword::False),
                    "null"          => Some(Keyword::Null),
                    "this"          => Some(Keyword::This),
                    "let"           => Some(Keyword::Let),
                    "do"            => Some(Keyword::Do),
                    "if"            => Some(Keyword::If),
                    "else"          => Some(Keyword::Else),
                    "while"         => Some(Keyword::While),
                    "return"        => Some(Keyword::Return),
                    _               => panic!("invalid keyword detected: {}", kw)
                }
            },
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_export_xml() {
        use super::*;
        use std::path::Path;
        use std::fs::File;
        use std::io::{BufWriter, Write};
        use std::process::Command;

        // pair list of full path of *.jack and *T.xml files
        let mut filename_pairs_in_out = vec![]; 
        let jack_src_path = Path::new("/workspace/Jack-compiler/jack_compiler/jack");
        for dir in jack_src_path.read_dir().expect("read_dir call failed") {
            if let Ok(dir) = dir {
                for f in dir.path().read_dir().expect("read_dir call failed") {
                    if let Ok(f) = f {
                        if f.path().extension().unwrap() == "jack" {
                            let input_filename = f.path().to_string_lossy().into_owned();
                            let output_filename = dir.path().join(f.path().file_stem().unwrap()).to_string_lossy().into_owned()+"T.xml";
                            filename_pairs_in_out.push((input_filename, output_filename));
                        }
                    }
                }
            }
        }

        // tokenize *.jack, export *T.xml, and compare with *T.xml.org
        for (fin, fout) in filename_pairs_in_out.iter() {
            let input_file = File::open(fin).expect("cannot open input file");
            let mut t = Tokenizer::new(input_file);

            let output_file = File::create(fout).expect("cannot open output file");
            let mut w = BufWriter::<File>::new(output_file);

            // export xml
            writeln!(w, "<tokens>").unwrap();
            while t.has_more_tokens() {
                t.advance();
                match t.token_type() {
                    Token::Keyword(s) => {
                        writeln!(w, "<keyword> {} </keyword>", s).unwrap();
                    },
                    Token::Symbol(c) => {
                        match c {
                            '&' => {
                                writeln!(w, "<symbol> &amp; </symbol>").unwrap();
                            },
                            '<' => {
                                writeln!(w, "<symbol> &lt; </symbol>").unwrap();
                            },
                            '>' => {
                                writeln!(w, "<symbol> &gt; </symbol>").unwrap();
                            },
                            c => {
                                writeln!(w, "<symbol> {} </symbol>", c).unwrap();
                            }
                        }
                    },
                    Token::Identifier(s) => {
                        writeln!(w, "<identifier> {} </identifier>", s).unwrap();
                    },
                    Token::IntConst(i) => {
                        writeln!(w, "<integerConstant> {} </integerConstant>", i).unwrap();
                    },
                    Token::StringConst(s) => {
                        writeln!(w, "<stringConstant> {} </stringConstant>", s).unwrap();
                    },
                    Token::Empty() => { unreachable!(); }
                }
            }
            writeln!(w, "</tokens>").unwrap();
            w.flush().unwrap();

            // compare two files
            let forg = Path::new(fout).with_extension("xml.org").to_string_lossy().into_owned();
            let diff_status = Command::new("diff").args(["-b", "-u", &fout, &forg]).status().expect("failed to execute process");
            assert!(diff_status.success());
        }
    }
}
