use std::io::{BufWriter, Write};
use std::fs::File;
use crate::tokenizer::*;

pub struct Engine {
    tokenizer: Tokenizer;
    writer: BufWriter<File>,
}

impl Engine {
    pub fn new(t: Tokenizer, f: File) -> Self {
        if t.token_type() == Token::Empty {
            t.advance();
        }
        Engine {
            tokenizer: t,
            writer: BufWriter::<File>::new(f),
        }
    }
    
    pub fn compile_class(&mut self) {
        writeln!(self.writer, "<class>").unwrap();
        // "class"
        match self.tokenizer.token_type() {
            Token::Keyword(class) => {
                writeln!(self.writer, "<keyword> {} </keyword>", class_name).unwrap();
            },
            t => {
                panic!("'class' expected, found {}", t);
            }
        }
        // className
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Identifier(class_name) => {
                writeln!(self.writer, "<identifier> {} </identifier>", class_name).unwrap();
            },
            t => {
                panic!("className expected, found {}", t);
            }
        }
        // '{'
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Symbol(left_brace @ '{') => {
                writeln!(self.writer, "<symbol> {} </symbol>", left_brace).unwrap();
            },
            t => {
                panic!("'{{' expected, found {}", t);
            }
        }
        // classVarDec*
        'classVarDec: loop {
            match self.tokenizer.peek_next_token() {
                &Token::Keyword(&attribute @ "static" | "field") => {
                    self.tokenizer.advance();
                    self.compile_class_var_dec();
                },
                _ => {
                    break 'classVarDec;
                }
            }
        }
        // subroutineDec*
        'subroutineDec: loop {
            match self.tokenizer.peek_next_token() {
                &Token::Keyword(&attribute @ "constructor" | "function" | "method") => {
                    self.tokenizer.advance();
                    self.compile_subroutine();
                },
                _ => {
                    break 'subroutineDec;
                }
            }
        }
        // '}'
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Symbol(right_brace @ '}') => {
                writeln!(self.writer, "<symbol> {} </symbol>", right_brace).unwrap();
            },
            t => {
                panic!("'}}' expected, found {}", t);
            }
        }
        // finish parsing class
        writeln!(self.writer, "</class>").unwrap();
    }

    pub fn compile_class_var_dec(&mut self) {
        writeln!(self.writer, "<classVarDec>").unwrap();
        // 'static' | 'field'
        match self.tokenizer.token_type() {
            Token::Keyword(&attribute @ "static" | "field") => {
                writeln!(self.writer, "<keyword> {} </keyword>", attribute).unwrap();
            },
            t => {
                panic!("'static' or 'field' expected, found {}", t);
            }
        }
        // type
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Keyword(&type_name @ "int" | "char" | "boolean") => {
                writeln!(self.writer, "<keyword> {} </keyword>", type_name).unwrap();
            },
            Token::Identifier(class_name) => {
                writeln!(self.writer, "<identifier> {} </identifier>", class_name).unwrap();
            },
            t => {
                panic!("type expected, found {}", t);
            }
        }
        // varName (',' varName)*
        'varName: loop {
            // varName
            self.tokenizer.advance();
            match self.tokenizer.token_type() {
                Token::Identifier(var_name) => {
                    writeln!(self.writer, "<identifier> {} </identifier>", var_name).unwrap();
                },
                t => {
                    panic!("varName expected, found {}", t);
                }
            }
            // ','
            match self.tokenizer.peek_next_token() {
                &Token::Symbol(comma @ ',') => {
                    self.tokenizer.advance();
                    writeln!(self.writer, "<symbol> {} </symbol>", comma).unwrap();
                },
                _ => { break 'varName; }
            }
        }
        // ';'
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Symbol(semicolon @ ';') => {
                writeln!(self.writer, "<symbol> {} </symbol>", semicolon).unwrap();
            },
            _ => {
                panic!("';' expected, found {}", t);
            }
        }
        // finish parsing classVarDec
        writeln!(self.writer, "</classVarDec>").unwrap();
    }
    
    pub fn compile_subroutine(&mut self) {
        writeln!(self.writer, "<subroutineDec>").unwrap();
        // 'constructor' | 'function' | 'method'
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Keyword(&attribute @ "constructor" | "function" | "method") => {
                writeln!(self.writer, "<keyword> {} </keyword>", attribute).unwrap();
            },
            t => {
                panic!("'constructor', 'function' or 'method' expected, found {}", t);
            }
        }
        // 'void' | type
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Keyword(&void @ "void") => {
                writeln!(self.writer, "<keyword> {} </keyword>", void).unwrap();
            },
            Token::Keyword(&type_name @ "int" | "char" | "boolean") => {
                writeln!(self.writer, "<keyword> {} </keyword>", type_name).unwrap();
            },
            Token::Identifier(class_name) => {
                writeln!(self.writer, "<identifier> {} </identifier>", class_name).unwrap();
            },
            t => {
                panic!("'void' or type expected, found {}", t);
            }
        }
        // subroutineName
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Identifier(subroutine_name) => {
                writeln!(self.writer, "<identifier> {} </identifier>", subroutine_name).unwrap();
            },
            t => {
                panic!("subroutineName expected, found {}", t);
            }
        }
        // '('
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Symbol(left_parenthesis @ '(') => {
                writeln!(self.writer, "<symbol> {} </symbol>", left_parenthesis).unwrap();
            },
            t => {
                panic!("'(' expected, found {}", t);
            }
        }
        // parameterList
        self.compile_parameter_list();
        // ')'
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Symbol(left_parenthesis @ ')') => {
                writeln!(self.writer, "<symbol> {} </symbol>", left_parenthesis).unwrap();
            },
            t => {
                panic!("')' expected, found {}", t);
            }
        }
        // subroutineBody
        writeln!(self.writer, "<subroutineBody>").unwrap();
        // '{'
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Symbol(left_brace @ '{') => {
                writeln!(self.writer, "<symbol> {} </symbol>", left_brace).unwrap();
            },
            t => {
                panic!("'{{' expected, found {}", t);
            }
        }
        // varDec*
        'varDec: loop {
            match self.tokenizer.peek_next_token() {
                Token::Keyword(var @ 'var') => {
                    self.compile_var_dec();
                },
                _ => { break 'varDec; }
            }
        }
        // statements
        self.compile_statements();
        // '}'
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Symbol(right_brace @ '}') => {
                writeln!(self.writer, "<symbol> {} </symbol>", right_brace).unwrap();
            },
            t => {
                panic!("'}}' expected, found {}", t);
            }
        }
        writeln!(self.writer, "</subroutineBody>").unwrap();
        writeln!(self.writer, "</subroutineDec>").unwrap();
    }

    pub fn compile_parameter_list(&mut self) { // this method looks ahead a ')' symbol token.
        writeln!(self.writer, "<parameterList>").unwrap();
        // (type varName (',' type varName)*)?
        'parameterList: loop {
            // type
            match self.tokenizer.peek_next_token() {
                &Token::Keyword(&type_name @ "int" | "char" | "boolean") => {
                    self.tokenizer.advance();
                    writeln!(self.writer, "<keyword> {} </keyword>", type_name).unwrap();
                },
                &Token::Identifier(class_name) => {
                    self.tokenizer.advance();
                    writeln!(self.writer, "<identifier> {} </identifier>", class_name).unwrap();
                },
                _ => {
                    break 'parameterList;
                }
            }
            // varName
            self.tokenizer.advance();
            match self.tokenizer.token_type() {
                Token::Identifier(var_name) => {
                    writeln!(self.writer, "<identifier> {} </identifier>", var_name).unwrap();
                },
                t => {
                    panic!("varName expected, found {}", t);
                }
            }
            // ','
            match self.tokenizer.peek_next_token() {
                &Token::Symbol(comma @ ',') => {
                    self.tokenizer.advance();
                    writeln!(self.writer, "<symbol> {} </symbol>", comma).unwrap();
                },
                _ => {
                    break 'parameterList;
                }
            }
        }
        writeln!(self.writer, "</parameterList>").unwrap();
    }

    pub fn compile_var_dec(&mut self) {
        writeln!(self.writer, "<varDec>").unwrap();
        // 'var'
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Keyword(&var @ "var") => {
                writeln!(self.writer, "<keyword> {} </keyword>", var).unwrap();
                /*
                    s => {
                        panic!("'static' or 'field' expected, found {}", s);
                    }
                }
                */
            },
            t => {
                panic!("'var' expected, found {}", t);
            }
        }
        // type
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Keyword(&type_name @ "int" | "char" | "boolean") => {
                writeln!(self.writer, "<keyword> {} </keyword>", type_name).unwrap();
                /*
                match type_name.as_str() => {
                    'int' | 'char' | 'boolean' => {
                        writeln!(self.writer, "<keyword> {} </keyword>", type_name).unwrap();
                    },
                    s => {
                        panic!("'int', 'char', or 'boolean' expected, found {}", s);
                    }
                }
                */
            },
            Token::Identifier(class_name) => {
                writeln!(self.writer, "<identifier> {} </identifier>", class_name).unwrap();
            },
            t => {
                panic!("type expected, found {}", t);
            }
        }
        // varName (',' varName)*
        'varName: loop {
            // varName
            self.tokenizer.advance();
            match self.tokenizer.token_type() {
                Token::Identifier(var_name) => {
                    writeln!(self.writer, "<identifier> {} </identifier>", var_name).unwrap();
                },
                t => {
                    panic!("varName expected, found {}", t);
                }
            }
            // ','
            match self.tokenizer.peek_next_token() {
                &Token::Symbol(comma @ ',') => {
                    self.tokenizer.advance();
                    writeln!(self.writer, "<symbol> {} </symbol>", comma).unwrap();
                },
                _ => { break 'varName; }
            }
        }
        // ';'
        match self.tokenizer.token_type() {
            Token::Symbol(semicolon @ ';') => {
                writeln!(self.writer, "<symbol> {} </symbol>", semicolon).unwrap();
            },
            _ => {
                panic!("';' expected, found {}", t);
            }
        }
        writeln!(self.writer, "</varDec>").unwrap();
    }

    pub fn compile_statements(&mut self) { unimplemented!(); }
    pub fn compile_do(&mut self) { unimplemented!(); }
    pub fn compile_let(&mut self) { unimplemented!(); }
    pub fn compile_while(&mut self) { unimplemented!(); }
    pub fn compile_return(&mut self) { unimplemented!(); }
    pub fn compile_if(&mut self) { unimplemented!(); }
    pub fn compile_expression(&mut self) { unimplemented!(); }
    pub fn compile_term(&mut self) { unimplemented!(); }
    pub fn compile_expression_lis          t(&mut self) { unimplemented!(); }
}