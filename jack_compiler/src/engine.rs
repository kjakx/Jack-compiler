use std::io::{BufWriter, Write};
use std::fs::File;
use crate::tokenizer::*;

pub struct Engine {
    tokenizer: Tokenizer;
    writer: BufWriter<File>,
}

impl Engine {
    pub fn new(t: Tokenizer, f: File) -> Self {
        Engine {
            tokenizer: t,
            writer: BufWriter::<File>::new(f),
        }
    }
    
    pub fn compile(&mut self) {
        self.compile_class();
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
        self.compile_identifier();
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
        self.compile_type();
        // varName (',' varName)*
        'varName: loop {
            // varName
            self.compile_identifier();
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
    
    pub fn compile_subroutine_dec(&mut self) {
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
        self.compile_identifier();
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
        self.compile_subroutine_body();
        writeln!(self.writer, "</subroutineDec>").unwrap();
    }

    pub fn compile_subroutine_body(&mut self) {
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
    }

    pub fn compile_parameter_list(&mut self) {
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
            self.compile_identifier();
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
            },
            t => {
                panic!("'var' expected, found {}", t);
            }
        }
        // type
        self.compile_type();
        // varName (',' varName)*
        'varName: loop {
            // varName
            compile_identifier();
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

    pub fn compile_statements(&mut self) {
        writeln!(self.writer, "<statements>").unwrap();
        // statement*
        'statement: loop {
            match self.tokenizer.peek_next_token() {
                Token::Keyword(stat) => {
                    match stat.as_str() {
                        "let" => {
                            self.compile_let();
                        },
                        "if" => {
                            self.compile_if();
                        },
                        "while" => {
                            self.compile_while();
                        },
                        "do" => {
                            self.compile_do();
                        },
                        "return" => {
                            self.compile_return();
                        },
                        _ => {
                            panic!("'let', 'if', 'while', 'do', or 'return' expected, found {}", s);
                        }
                    }
                },
                _ => { break 'statement; }
            }
        }
        writeln!(self.writer, "</statements>").unwrap();
    }

    pub fn compile_do(&mut self) {
        writeln!(self.writer, "<doStatement>").unwrap();
        // 'do'
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Keyword(do_stat @ "do") => {
                writeln!(self.writer, "<keyword> {} </keyword>", do_stat).unwrap();
            },
            t => {
                panic!("'do' expected, found {}", t);
            }
        }
        // subroutineCall
        self.compile_subroutine_call();
        writeln!(self.writer, "</doStatements>").unwrap();
    }

    pub fn compile_type(&mut self) {
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
    }

    pub fn compile_let(&mut self) { unimplemented!(); }
    pub fn compile_while(&mut self) { unimplemented!(); }
    pub fn compile_return(&mut self) { unimplemented!(); }
    pub fn compile_if(&mut self) { unimplemented!(); }
    pub fn compile_expression(&mut self) { unimplemented!(); }
    pub fn compile_term(&mut self) { unimplemented!(); }
    pub fn compile_expression_list(&mut self) { unimplemented!(); }
    pub fn compile_subroutine_call(&mut self) { unimplemented!(); }

    fn compile_keyword(&mut self) {
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Keyword(kw) => {
                writeln!(self.writer, "<keyword> {} </keyword>", kw).unwrap();
            },
            t => {
                panic!("keyword expected, found {}", t);
            }
        }
    }

    fn compile_symbol(&mut self) {
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Symbol(sym) => {
                writeln!(self.writer, "<symbol> {} </symbol>", sym).unwrap();
            },
            t => {
                panic!("symbol expected, found {}", t);
            }
        }
    }

    fn compile_identifier(&mut self) {
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::Identifier(ident) => {
                writeln!(self.writer, "<identifier> {} </identifier>", ident).unwrap();
            },
            t => {
                panic!("identifier expected, found {}", t);
            }
        }
    }

    fn compile_integer_constant(&mut self) {
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::IntConst(int_const) => {
                writeln!(self.writer, "<integerConstant> {} </integerConstant>", int_const).unwrap();
            },
            t => {
                panic!("integerConstant expected, found {}", t);
            }
        }
    }

    fn compile_string_constant(&mut self) {
        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            Token::StringConst(str_const) => {
                writeln!(self.writer, "<stringConstant> {} </stringConstant>", str_const).unwrap();
            },
            t => {
                panic!("stringConstant expected, found {}", t);
            }
        }
    }
}