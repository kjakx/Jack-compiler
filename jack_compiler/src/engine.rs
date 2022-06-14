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
        // 'class' className '{'
        self.compile_keyword_expect("class");
        self.compile_identifier();
        self.compile_symbol_expect('{');
        // classVarDec*
        'classVarDec: loop {
            match self.tokenizer.peek_next_token() {
                &Token::Keyword(&_ @ "static" | "field") => {
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
                &Token::Keyword(&_ @ "constructor" | "function" | "method") => {
                    self.compile_subroutine();
                },
                _ => {
                    break 'subroutineDec;
                }
            }
        }
        // '}'
        self.compile_symbol_expect('}');
        // finish parsing class
        writeln!(self.writer, "</class>").unwrap();
    }

    pub fn compile_class_var_dec(&mut self) {
        writeln!(self.writer, "<classVarDec>").unwrap();
        // 'static' | 'field'
        match self.tokenizer.peek_next_token() {
            &Token::Keyword(&_ @ "static" | "field") => {
                self.compile_keyword();
            },
            t => {
                panic!("'static' or 'field' expected, found {}", t);
            }
        }
        // type
        match self.tokenizer.peek_next_token() {
            &Token::Keyword(&_ @ "int" | "char" | "boolean") => {
                self.compile_keyword();
            },
            &Token::Identifier => {
                self.compile_identifier();
            },
            t => {
                panic!("type expected, found {}", t);
            }
        }
        // varName (',' varName)*
        'varName: loop {
            // varName
            self.compile_identifier();
            // ','
            match self.tokenizer.peek_next_token() {
                &Token::Symbol(_ @ ',') => {
                    self.compile_symbol();
                },
                _ => { break 'varName; }
            }
        }
        // ';'
        self.compile_symbol_expect(';');
        writeln!(self.writer, "</classVarDec>").unwrap();
    }
    
    pub fn compile_subroutine_dec(&mut self) {
        writeln!(self.writer, "<subroutineDec>").unwrap();
        // 'constructor' | 'function' | 'method'
        match self.tokenizer.peek_next_token() {
            Token::Keyword(&_ @ "constructor" | "function" | "method") => {
                self.compile_keyword();
            },
            t => {
                panic!("'constructor', 'function' or 'method' expected, found {}", t);
            }
        }
        // 'void' | type
        match self.tokenizer.peek_next_token() {
            Token::Keyword(&_ @ "void") => {
                self.compile_keyword();
            },
            Token::Keyword(&_ @ "int" | "char" | "boolean") => {
                self.compile_keyword();
            },
            Token::Identifier => {
                self.compile_identifier();
            },
            t => {
                panic!("'void' or type expected, found {}", t);
            }
        }
        // subroutineName '(' parameterList ')'
        self.compile_identifier();
        self.compile_symbol_expect('(');
        self.compile_parameter_list();
        self.compile_symbol_expect(')');
        // subroutineBody
        self.compile_subroutine_body();
        writeln!(self.writer, "</subroutineDec>").unwrap();
    }

    pub fn compile_subroutine_body(&mut self) {
        writeln!(self.writer, "<subroutineBody>").unwrap();
        // '{'
        self.compile_keyword_expect('{');
        // varDec*
        'varDec: loop {
            match self.tokenizer.peek_next_token() {
                Token::Keyword(_ @ "var") => {
                    self.compile_var_dec();
                },
                _ => { break 'varDec; }
            }
        }
        // statements
        self.compile_statements();
        // '}'
        self.compile_keyword_expect('}');
        writeln!(self.writer, "</subroutineBody>").unwrap();
    }

    pub fn compile_parameter_list(&mut self) {
        writeln!(self.writer, "<parameterList>").unwrap();
        // (type varName (',' type varName)*)?
        'parameterList: loop {
            // type
            match self.tokenizer.peek_next_token() {
                &Token::Keyword(&_ @ "int" | "char" | "boolean") => {
                    self.compile_keyword();
                },
                &Token::Identifier => {
                    self.compile_identifier();
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
                    self.compile_symbol();
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
        self.compile_keyword_expect("var");
        // type
        match self.tokenizer.peek_next_token() {
            &Token::Keyword(&type_name @ "int" | "char" | "boolean") => {
                self.compile_keyword();
            },
            &Token::Identifier => {
                self.compile_identifier();
            },
            t => {
                panic!("type expected, found {}", t);
            }
        }
        // varName (',' varName)*
        'varName: loop {
            // varName
            self.compile_identifier();
            // ','
            match self.tokenizer.peek_next_token() {
                &Token::Symbol(_ @ ',') => {
                    self.compile_symbol();
                },
                _ => { break 'varName; }
            }
        }
        // ';'
        self.compile_symbol_expect(';');
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
                        s => {
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
        // 'do' subroutineCall
        self.compile_keyword_expect("do");
        self.compile_subroutine_call();
        writeln!(self.writer, "</doStatements>").unwrap();
    }

    pub fn compile_let(&mut self) {
        writeln!(self.writer, "<letStatement>").unwrap();
        // 'let' varName 
        self.compile_keyword_expect("let");
        self.compile_identifier();
        // ('[' expression ']')?
        if let &Token::Symbol(_ @ '[') = self.tokenizer.peek_next_token() {
            // '[' expression ']'
            self.compile_symbol_expect('[');
            self.compile_expression();
            self.compile_symbol_expect(']');
        }
        // '=' expression ';'
        self.compile_symbol_expect('=');
        self.compile_expression();
        self.compile_symbol_expect(';');
        writeln!(self.writer, "</letStatement>").unwrap();
    }

    pub fn compile_while(&mut self) {
        writeln!(self.writer, "<whileStatement>").unwrap();
        // 'while' '(' expression ')'
        self.compile_keyword_expect("while")
        self.compile_symbol_expect('(');
        self.compile_expression();
        self.compile_symbol_expect(')');
        // '{' statements '}'
        self.compile_symbol_expect('{');
        self.compile_statements();
        self.compile_symbol_expect('}');
        writeln!(self.writer, "</whileStatement>").unwrap();
    }

    pub fn compile_return(&mut self) {
        writeln!(self.writer, "<returnStatement>").unwrap();
        // 'return'
        self.compile_keyword_expect("return");
        // expression?
        match self.tokenizer.peek_next_token() {
            &Token::Symbol(_ @ ';') => (),
            _ => {
                self.compile_expression();
            }
        }
        // ';'
        self.compile_symbol_expect(';');
        writeln!(self.writer, "</returnStatements>").unwrap();
    }

    pub fn compile_if(&mut self) {
        writeln!(self.writer, "<ifStatements>").unwrap();
        // 'if' '(' expression ')'
        self.compile_keyword_expect("if");
        self.compile_symbol_expect('(');
        self.compile_expression();
        self.compile_symbol_expect(')');
        // '{' statements '}'
        self.compile_symbol_expect('{');
        self.compile_statements();
        self.compile_symbol_expect('}');
        // ('else' '{' statements '}')?
        if let &Token::Keyword(_ @ "else") = self.tokenizer.peek_next_token() {
            // 'else' '{' statements '}'
            self.compile_keyword_expect("else");
            self.compile_symbol_expect('{');
            self.compile_statements();
            self.compile_symbol_expect('}');
        }
        writeln!(self.writer, "</ifStatements>").unwrap();
    }

    pub fn compile_expression(&mut self) {
        writeln!(self.writer, "<expression>").unwrap();
        // term
        self.compile_term();
        // (op term)*
        'term loop {
            match self.tokenizer.peek_next_token() {
                Token::Symbol(_ @ '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '=') => {
                    self.compile_symbol();
                },
                _ => {
                    break 'term;
                }
            }
            self.compile_term();
        }
        writeln!(self.writer, "</expression>").unwrap();
    }

    pub fn compile_term(&mut self) {
        writeln!(self.writer, "<term>").unwrap();
        match self.tokenizer.peek_next_token() {
            &Token::IntConst => {
                self.compile_integer_constant();
            },
            &Token::StringConst => {
                self.compile_string_constant();
            },
            &Token::Keyword(kw_const @ "true" | "false" | "null" | "this") => {
                self.compile_keyword();
            },
            &Token::Identifier => {
                // varName | subroutineCall | (className | varName)
                self.compile_identifier();
                match self.tokenizer.peek_next_token() {
                    &Token::Symbol(_ @ '[') => {
                        // '[' expression ']'
                        self.compile_symbol_expect('[');
                        self.compile_expression();
                        self.compile_symbol_expect(']');
                    },
                    &Token::Symbol(_ @ '(' | '.') => {
                        self.compile_subroutine_call();
                    },
                    _ => ()
                }
            },
            &Token::Symbol(sym @ '-' | '~' | '(') => {
                match sym {
                    '-' | '~' => {
                        // unaryOp term
                        self.compile_symbol();
                        self.compile_term();
                    },
                    '(' => {
                        // '(' expressionList ')'
                        self.compile_symbol_expect('(');
                        self.compile_expression_list();
                        self.compile_symbol_expect(')');
                    }
                }
            },
            t => {
                panic!("unexpected token while parsing term: {}", t);
            }
        }
        writeln!(self.writer, "</term>").unwrap();
    }

    pub fn compile_expression_list(&mut self) {
        writeln!(self.writer, "<expressionList>").unwrap();
        // (expression (',' expression)* )?
        match self.tokenizer.peek_next_token() {
            &Token::Symbol(_ @ ')') => (),
            _ => {
                // expression (',' expression)*
                self.compile_expression();
                'expression loop {
                    match self.tokenizer.peek_next_token() {
                        &Token::Symbol(_ @ ',') => {
                            self.compile_symbol();
                        },
                        _ => { break 'expression; }
                    }
                    self.compile_expression();
                }
            }
        }
        writeln!(self.writer, "</expressionList>").unwrap();
    }

    pub fn compile_subroutine_call(&mut self) {
        writeln!(self.writer, "<subroutineCall>").unwrap();
        // (className | varName) | subroutineName
        self.compile_identifier();
        // ('.' subroutineName)?
        match self.tokenizer.peek_next_token() {
            &Token::Symbol(_ @ '.') => {
                self.compile_symbol();
                // subroutineName
                self.compile_identifier();
            },
            _ => ()
        } 
        // '(' expressionList ')'
        self.compile_symbol_expect('(');
        self.compile_expression_list();
        self.compile_symbol_expect(')');
        writeln!(self.writer, "</subroutineCall>").unwrap();
    }

    fn compile_keyword_expect(&mut self, kw: &str) {
        match self.tokenizer.peek_next_token() {
            &Token::Keyword(_ @ kw) {
                self.compile_keyword();
            },
            t => {
                panic!("{} expected, found {}", kw, t);
            }
        }
    }

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

    fn compile_symbol_expect(&mut self, sym: char) {
        match self.tokenizer.peek_next_token() {
            &Token::Symbol(_ @ sym) {
                self.compile_symbol();
            },
            t => {
                panic!("{} expected, found {}", sym, t);
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