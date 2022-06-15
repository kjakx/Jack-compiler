use std::io::{Read, BufRead, BufReader};
use std::io::ErrorKind;
use std::fs::File;
use std::str::FromStr;
use crate::keyword::*;
use crate::symbol::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Empty(),
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(String),
    IntConst(i16),
    StringConst(String),
}

pub struct Tokenizer {
    pub tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(f: File) -> Self {
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
                        // If a number, it is an integerConstant. Read until the end of the number.
                        b'0'..=b'9' => {
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

                            let word = std::str::from_utf8(&chars).unwrap();
                            match Keyword::from_str(word) {
                                Ok(kw) => {
                                    tokens.push(Token::Keyword(kw));
                                },
                                Err(_) => {
                                    tokens.push(Token::Identifier(word.to_string()));
                                }
                            }
                        },
                        // if a symbol, it is a symbol token or a comment.
                        c => {
                            match Symbol::from_u8(c) {
                                Ok(sym) => {
                                    match sym {
                                        // If c is a /(slash), the next byte should be checked.
                                        Symbol::Slash => {
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
                                                        tokens.push(Token::Symbol(Symbol::from_u8(c).unwrap()));
                                                    }
                                                }
                                            }
                                        },
                                        // If the other symbol, it can immediately be added to tokens as a symbol.
                                        _ => {
                                            tokens.push(Token::Symbol(Symbol::from_u8(c).unwrap()));
                                        }
                                    }
                                },
                                Err(e) => {
                                    panic!("unexpected error occurred while tokenizing: {}", e);
                                }
                            }
                        }
                    }
                },
                Err(e) => {
                    match e.kind() {
                        ErrorKind::UnexpectedEof => { break 'tokenize; }, // reached EOF
                        _ => panic!("unexpected error occurred while tokenizing: {}", e),
                    }
                }
            }
        }

        let tokens = tokens.into_iter().rev().collect();

        Tokenizer {
            tokens: tokens,
        }
    }

    pub fn has_more_tokens(&self) -> bool {
        !self.tokens.is_empty()
    }

    pub fn get_next_token(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Empty())
    }

    pub fn peek_next_token(&self) -> Option<&Token> {
        self.tokens.last()
    }

    pub fn peek_2nd_next_token(&self) -> Option<&Token> {
        match self.tokens.len() {
            n if n >= 2 => Some(&self.tokens[n-2]),
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
                match t.get_next_token() {
                    Token::Keyword(kw) => {
                        writeln!(w, "<keyword> {} </keyword>", kw).unwrap();
                    },
                    Token::Symbol(sym) => {
                        writeln!(w, "<symbol> {} </symbol>", sym).unwrap();
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
