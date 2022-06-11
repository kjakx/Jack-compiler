use std::io::{BufWriter, Write};
use std::fs::File;
use crate::tokenizer::*;

pub struct Engine {
    tokenizer: tokenizer::Tokenizer;
    writer: BufWriter<File>,
}

impl Engine {
    pub fn new(t: Tokenizer, f: File) -> Self {
        Engine {
            tokenizer: t,
            writer: BufWriter::<File>::new(f),
        }
    }
    
    pub fn compile_class(&mut self) {
        writeln!(self.writer, "<class>").unwrap();
        // "class"
        self.tokenizer.advance();
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
        self.compile_class_var_dec();
        // subroutineDec*
        self.compile_subroutine();
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
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Keyword(&attribute @ 'static' | 'field') => {
                writeln!(self.writer, "<keyword> {} </keyword>", attribute).unwrap();
                /*
                    s => {
                        panic!("'static' or 'field' expected, found {}", s);
                    }
                }
                */
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
            self.tokenizer.advance();
            match self.tokenizer.token_type() {
                Token::Symbol(comma @ ',') => {
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
        // finish parsing classVarDec
        writeln!(self.writer, "</classVarDec>").unwrap();
    }
    
    pub fn compile_subroutine(&mut self) { unimplemented!(); }
    pub fn compile_parameter_list(&mut self) { unimplemented!(); }
    pub fn compile_var_dec(&mut self) { unimplemented!(); }
    pub fn compile_statements(&mut self) { unimplemented!(); }
    pub fn compile_do(&mut self) { unimplemented!(); }
    pub fn compile_let(&mut self) { unimplemented!(); }
    pub fn compile_while(&mut self) { unimplemented!(); }
    pub fn compile_return(&mut self) { unimplemented!(); }
    pub fn compile_if(&mut self) { unimplemented!(); }
    pub fn compile_expression(&mut self) { unimplemented!(); }
    pub fn compile_term(&mut self) { unimplemented!(); }
    pub fn compile_expression_list(&mut self) { unimplemented!(); }
}