use std::io::{BufWriter, Write};
use std::fs::File;
use crate::tokenizer::*;
use crate::keyword::*;
use crate::symbol::*;
use crate::symbol_table::*;

pub struct Engine {
    tokenizer: Tokenizer,
    sym_tbl: SymbolTable,
    writer: BufWriter<File>,
}

impl Engine {
    pub fn new(t: Tokenizer, f: File) -> Self {
        Engine {
            tokenizer: t,
            sym_tbl: SymbolTable::new(),
            writer: BufWriter::<File>::new(f),
        }
    }
    
    pub fn compile(&mut self) {
        self.compile_class();
        self.writer.flush().unwrap();
    }

    pub fn compile_class(&mut self) {
        writeln!(self.writer, "<class>").unwrap();
        // 'class' className '{'
        self.compile_keyword_expect(Keyword::Class);
        self.compile_class_name();
        self.compile_symbol_expect(Symbol::BraceL);
        // classVarDec*
        'classVarDec: loop {
            match self.tokenizer.peek_next_token().unwrap() {
                &Token::Keyword(Keyword::Static | Keyword::Field) => {
                    self.compile_class_var_dec();
                },
                _ => {
                    break 'classVarDec;
                }
            }
        }
        // subroutineDec*
        'subroutineDec: loop {
            match self.tokenizer.peek_next_token().unwrap() {
                &Token::Keyword(Keyword::Constructor | Keyword::Function | Keyword::Method) => {
                    self.compile_subroutine_dec();
                },
                _ => {
                    break 'subroutineDec;
                }
            }
        }
        // '}'
        self.compile_symbol_expect(Symbol::BraceR);
        // finish parsing class
        writeln!(self.writer, "</class>").unwrap();
    }

    pub fn compile_class_var_dec(&mut self) {
        writeln!(self.writer, "<classVarDec>").unwrap();
        // 'static' | 'field'
        let varkind = match self.tokenizer.peek_next_token().unwrap() {
            &Token::Keyword(Keyword::Static) => {
                VarKind::Static
            },
            &Token::Keyword(Keyword::Field) => {
                VarKind::Field
            },
            t => {
                panic!("'static' or 'field' expected, found {:?}", t);
            }
        };
        self.compile_keyword();
        // type
        let vartype = self.compile_type().unwrap();
        // varName (',' varName)*
        'varName: loop {
            // varName
            self.compile_var_name_defined(varkind, vartype);
            // ','
            match self.tokenizer.peek_next_token().unwrap() {
                &Token::Symbol(Symbol::Comma) => {
                    self.compile_symbol();
                },
                _ => { break 'varName; }
            }
        }
        // ';'
        self.compile_symbol_expect(Symbol::SemiColon);
        writeln!(self.writer, "</classVarDec>").unwrap();
    }
    
    pub fn compile_subroutine_dec(&mut self) {
        self.sym_tbl.start_subroutine();
        writeln!(self.writer, "<subroutineDec>").unwrap();
        // 'constructor' | 'function' | 'method'
        match self.tokenizer.peek_next_token().unwrap() {
            Token::Keyword(Keyword::Constructor | Keyword::Function | Keyword::Method) => {
                self.compile_keyword();
            },
            t => {
                panic!("'constructor', 'function' or 'method' expected, found {:?}", t);
            }
        }
        // 'void' | type
        match self.tokenizer.peek_next_token().unwrap() {
            Token::Keyword(Keyword::Void) => {
                self.compile_keyword();
            },
            Token::Keyword(Keyword::Int | Keyword::Char | Keyword::Boolean) => {
                self.compile_keyword();
            },
            Token::Identifier(_) => {
                self.compile_class_name();
            },
            t => {
                panic!("'void' or type expected, found {:?}", t);
            }
        }
        // subroutineName '(' parameterList ')'
        self.compile_subroutine_name();
        self.compile_symbol_expect(Symbol::ParenL);
        self.compile_parameter_list();
        self.compile_symbol_expect(Symbol::ParenR);
        // subroutineBody
        self.compile_subroutine_body();
        writeln!(self.writer, "</subroutineDec>").unwrap();
    }

    pub fn compile_subroutine_body(&mut self) {
        writeln!(self.writer, "<subroutineBody>").unwrap();
        // '{'
        self.compile_symbol_expect(Symbol::BraceL);
        // varDec*
        'varDec: loop {
            match self.tokenizer.peek_next_token().unwrap() {
                Token::Keyword(Keyword::Var) => {
                    self.compile_var_dec();
                },
                _ => { break 'varDec; }
            }
        }
        // statements
        self.compile_statements();
        // '}'
        self.compile_symbol_expect(Symbol::BraceR);
        writeln!(self.writer, "</subroutineBody>").unwrap();
    }

    pub fn compile_parameter_list(&mut self) {
        writeln!(self.writer, "<parameterList>").unwrap();
        // (type varName (',' type varName)*)?
        'parameterList: loop {
            // type
            if let Ok(vartype) = self.compile_type() {
                // varName
                self.compile_var_name_defined(VarKind::Arg, vartype);
            } else {
                break 'parameterList;
            }
            // ','
            match self.tokenizer.peek_next_token().unwrap() {
                &Token::Symbol(Symbol::Comma) => {
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
        self.compile_keyword_expect(Keyword::Var);
        // type
        let vartype = self.compile_type().unwrap();
        // varName (',' varName)*
        'varName: loop {
            // varName
            self.compile_var_name_defined(VarKind::Var, vartype);
            // ','
            match self.tokenizer.peek_next_token().unwrap() {
                &Token::Symbol(Symbol::Comma) => {
                    self.compile_symbol();
                },
                _ => { break 'varName; }
            }
        };
        // ';'
        self.compile_symbol_expect(Symbol::SemiColon);
        writeln!(self.writer, "</varDec>").unwrap();
    }

    pub fn compile_statements(&mut self) {
        writeln!(self.writer, "<statements>").unwrap();
        // statement*
        'statement: loop {
            match self.tokenizer.peek_next_token().unwrap() {
                Token::Keyword(stat) => {
                    match stat {
                        Keyword::Let => {
                            self.compile_let();
                        },
                        Keyword::If => {
                            self.compile_if();
                        },
                        Keyword::While => {
                            self.compile_while();
                        },
                        Keyword::Do => {
                            self.compile_do();
                        },
                        Keyword::Return => {
                            self.compile_return();
                        },
                        s => {
                            panic!("'let', 'if', 'while', 'do', or 'return' expected, found {:?}", s);
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
        // 'do' subroutineCall ';'
        self.compile_keyword_expect(Keyword::Do);
        self.compile_subroutine_call();
        self.compile_symbol_expect(Symbol::SemiColon);
        writeln!(self.writer, "</doStatement>").unwrap();
    }

    pub fn compile_let(&mut self) {
        writeln!(self.writer, "<letStatement>").unwrap();
        // 'let' varName 
        self.compile_keyword_expect(Keyword::Let);
        self.compile_var_name_used();
        // ('[' expression ']')?
        if let &Token::Symbol(Symbol::SqParL) = self.tokenizer.peek_next_token().unwrap() {
            // '[' expression ']'
            self.compile_symbol_expect(Symbol::SqParL);
            self.compile_expression();
            self.compile_symbol_expect(Symbol::SqParR);
        }
        // '=' expression ';'
        self.compile_symbol_expect(Symbol::Equal);
        self.compile_expression();
        self.compile_symbol_expect(Symbol::SemiColon);
        writeln!(self.writer, "</letStatement>").unwrap();
    }

    pub fn compile_while(&mut self) {
        writeln!(self.writer, "<whileStatement>").unwrap();
        // 'while' '(' expression ')'
        self.compile_keyword_expect(Keyword::While);
        self.compile_symbol_expect(Symbol::ParenL);
        self.compile_expression();
        self.compile_symbol_expect(Symbol::ParenR);
        // '{' statements '}'
        self.compile_symbol_expect(Symbol::BraceL);
        self.compile_statements();
        self.compile_symbol_expect(Symbol::BraceR);
        writeln!(self.writer, "</whileStatement>").unwrap();
    }

    pub fn compile_return(&mut self) {
        writeln!(self.writer, "<returnStatement>").unwrap();
        // 'return'
        self.compile_keyword_expect(Keyword::Return);
        // expression?
        match self.tokenizer.peek_next_token().unwrap() {
            &Token::Symbol(Symbol::SemiColon) => (),
            _ => {
                self.compile_expression();
            }
        }
        // ';'
        self.compile_symbol_expect(Symbol::SemiColon);
        writeln!(self.writer, "</returnStatement>").unwrap();
    }

    pub fn compile_if(&mut self) {
        writeln!(self.writer, "<ifStatement>").unwrap();
        // 'if' '(' expression ')'
        self.compile_keyword_expect(Keyword::If);
        self.compile_symbol_expect(Symbol::ParenL);
        self.compile_expression();
        self.compile_symbol_expect(Symbol::ParenR);
        // '{' statements '}'
        self.compile_symbol_expect(Symbol::BraceL);
        self.compile_statements();
        self.compile_symbol_expect(Symbol::BraceR);
        // ('else' '{' statements '}')?
        if let &Token::Keyword(Keyword::Else) = self.tokenizer.peek_next_token().unwrap() {
            // 'else' '{' statements '}'
            self.compile_keyword_expect(Keyword::Else);
            self.compile_symbol_expect(Symbol::BraceL);
            self.compile_statements();
            self.compile_symbol_expect(Symbol::BraceR);
        }
        writeln!(self.writer, "</ifStatement>").unwrap();
    }

    pub fn compile_expression(&mut self) {
        writeln!(self.writer, "<expression>").unwrap();
        // term
        self.compile_term();
        // (op term)*
        'term: loop {
            match self.tokenizer.peek_next_token().unwrap() {
                Token::Symbol(
                    Symbol::Plus | Symbol::Minus | Symbol::Asterisk | Symbol::Slash |
                    Symbol::And  | Symbol::Or    | Symbol::LessThan | Symbol::GreaterThan | Symbol::Equal
                ) => {
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
        match self.tokenizer.peek_next_token().unwrap() {
            &Token::IntConst(_) => {
                self.compile_integer_constant();
            },
            &Token::StringConst(_) => {
                self.compile_string_constant();
            },
            &Token::Keyword(Keyword::True | Keyword::False | Keyword::Null | Keyword::This) => {
                self.compile_keyword();
            },
            &Token::Identifier(_) => {
                match self.tokenizer.peek_2nd_next_token().unwrap() {
                    &Token::Symbol(Symbol::SqParL) => {
                        // varName '[' expression ']'
                        self.compile_var_name_used();
                        self.compile_symbol_expect(Symbol::SqParL);
                        self.compile_expression();
                        self.compile_symbol_expect(Symbol::SqParR);
                    },
                    &Token::Symbol(Symbol::ParenL | Symbol::Dot) => {
                        // subroutineCall
                        self.compile_subroutine_call();
                    },
                    _ => {
                        // varName
                        self.compile_var_name_used();
                    }
                }
            },
            &Token::Symbol(Symbol::Minus | Symbol::Not) => {
                // unaryOp term
                self.compile_symbol();
                self.compile_term();
            },
            &Token::Symbol(Symbol::ParenL) => {
                // '(' expression ')'
                self.compile_symbol_expect(Symbol::ParenL);
                self.compile_expression();
                self.compile_symbol_expect(Symbol::ParenR);
            },
            t => {
                panic!("unexpected token while parsing term: {:?}", t);
            }
        }
        writeln!(self.writer, "</term>").unwrap();
    }

    pub fn compile_expression_list(&mut self) {
        writeln!(self.writer, "<expressionList>").unwrap();
        // (expression (',' expression)* )?
        match self.tokenizer.peek_next_token().unwrap() {
            &Token::Symbol(Symbol::ParenR) => (),
            _ => {
                // expression (',' expression)*
                self.compile_expression();
                'expression: loop {
                    match self.tokenizer.peek_next_token().unwrap() {
                        &Token::Symbol(Symbol::Comma) => {
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
        // subroutine or method?
        match self.tokenizer.peek_2nd_next_token().unwrap() {
            &Token::Symbol(Symbol::Dot) => {
                // (className | varName) '.' subroutineName
                self.compile_var_name_used();
                self.compile_symbol_expect(Symbol::Dot);
                self.compile_subroutine_name();
            },
            _ => {
                // subroutineName
                self.compile_subroutine_name();
            }
        } 
        // '(' expressionList ')'
        self.compile_symbol_expect(Symbol::ParenL);
        self.compile_expression_list();
        self.compile_symbol_expect(Symbol::ParenR);
    }

    fn compile_keyword_expect(&mut self, kw_expect: Keyword) {
        match self.tokenizer.peek_next_token().unwrap() {
            Token::Keyword(kw_next) => {
                if *kw_next == kw_expect {
                    self.compile_keyword();
                }
                else {
                    panic!("{} expected, found {:?}", kw_expect, kw_next);
                }
            },
            t => {
                panic!("Token::keyword expected, found {:?}", t);
            }
        }
    }

    fn compile_keyword(&mut self) {
        match self.tokenizer.get_next_token() {
            Token::Keyword(kw) => {
                writeln!(self.writer, "<keyword> {} </keyword>", kw).unwrap();
            },
            t => {
                panic!("keyword expected, found {:?}", t);
            }
        }
    }

    fn compile_symbol_expect(&mut self, sym_expect: Symbol) {
        match self.tokenizer.peek_next_token().unwrap() {
            Token::Symbol(sym_next) => {
                if *sym_next == sym_expect {
                    self.compile_symbol();
                }
                else {
                    panic!("{} expected, found {:?}", sym_expect, sym_next);
                }
            },
            t => {
                panic!("Token::Symbol expected, found {:?}", t);
            }
        }
    }

    fn compile_symbol(&mut self) {
        match self.tokenizer.get_next_token() {
            Token::Symbol(sym) => {
                writeln!(self.writer, "<symbol> {} </symbol>", sym).unwrap();
            },
            t => {
                panic!("symbol expected, found {:?}", t);
            }
        }
    }

    fn compile_identifier(&mut self) {
        match self.tokenizer.get_next_token() {
            Token::Identifier(ident) => {
                writeln!(self.writer, "<identifier> {} </identifier>", ident).unwrap();
            },
            t => {
                panic!("identifier expected, found {:?}", t);
            }
        }
    }


    fn compile_integer_constant(&mut self) {
        match self.tokenizer.get_next_token() {
            Token::IntConst(int_const) => {
                writeln!(self.writer, "<integerConstant> {} </integerConstant>", int_const).unwrap();
            },
            t => {
                panic!("integerConstant expected, found {:?}", t);
            }
        }
    }

    fn compile_string_constant(&mut self) {
        match self.tokenizer.get_next_token() {
            Token::StringConst(str_const) => {
                writeln!(self.writer, "<stringConstant> {} </stringConstant>", str_const).unwrap();
            },
            t => {
                panic!("stringConstant expected, found {:?}", t);
            }
        }
    }

    fn compile_class_name(&mut self) {
        match self.tokenizer.get_next_token() {
            Token::Identifier(ident) => {
                writeln!(self.writer, "<className> {} </className>", ident).unwrap();
            },
            t => {
                panic!("identifier expected, found {:?}", t);
            }
        }
    }

    fn compile_subroutine_name(&mut self) {
        match self.tokenizer.get_next_token() {
            Token::Identifier(ident) => {
                writeln!(self.writer, "<subroutineName> {} </subroutineName>", ident).unwrap();
            },
            t => {
                panic!("identifier expected, found {:?}", t);
            }
        }
    }

    fn compile_var_name_defined(&mut self, var_kind: VarKind, var_type: VarType) {
        match self.tokenizer.get_next_token() {
            Token::Identifier(ident) => {
                self.sym_tbl.define(&ident, var_kind, var_type);
                let var_index = self.sym_tbl.index_of(&ident).expect(format!("unknown identifier {}", ident).as_str());
                writeln!(self.writer, "<varName(defined)> {}[{}] {} {} </varName(defined)>", var_kind, *var_index, var_type, ident).unwrap();
            },
            t => {
                panic!("identifier expected, found {:?}", t);
            }
        }
    }

    fn compile_var_name_used(&mut self) {
        match self.tokenizer.get_next_token() {
            Token::Identifier(ident) => {
                if self.sym_tbl.contains(&ident) {
                    let var_kind = self.sym_tbl.kind_of(&ident).expect(format!("unknown identifier {}", ident).as_str());
                    let var_type = self.sym_tbl.type_of(&ident).expect(format!("unknown identifier {}", ident).as_str());
                    let var_index = self.sym_tbl.index_of(&ident).expect(format!("unknown identifier {}", ident).as_str());
                    writeln!(self.writer, "<varName(used)> {}[{}] {} {} </varName(used)>", var_kind, *var_index, var_type, ident).unwrap();
                } else {
                    writeln!(self.writer, "<className> {} </className>", ident).unwrap();

                }
            },
            t => {
                panic!("identifier expected, found {:?}", t);
            }
        }
    }

    fn compile_type(&mut self) -> Result<VarType, String> {
        match self.tokenizer.peek_next_token().unwrap() {
            Token::Keyword(t) => {
                let vartype = match t {
                    Keyword::Int     => Ok(VarType::Int),
                    Keyword::Char    => Ok(VarType::Char),
                    Keyword::Boolean => Ok(VarType::Boolean),
                    _ => {
                        Err(format!("type expected, found {:?}", t))
                    }
                };
                self.compile_keyword();
                vartype
            },
            Token::Identifier(_) => {
                self.compile_class_name();
                Ok(VarType::ClassName)
            },
            t => {
                Err(format!("type expected, found {:?}", t))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_no_expression_case() {
        use super::*;
        use std::path::Path;
        use std::fs::File;
        use std::io::{BufWriter, Write};
        use std::process::Command;
        use crate::tokenizer::*;

        // pair list of full path of *.jack and *.xml files
        let mut filename_pairs_in_out = vec![]; 
        let jack_src_path = Path::new("./jack/ExpressionLessSquare");
        for f in jack_src_path.read_dir().expect("read_dir call failed") {
            if let Ok(f) = f {
                if f.path().extension().unwrap() == "jack" {
                    let input_filename = f.path().to_string_lossy().into_owned();
                    let output_filename = f.path().with_extension("detailed.xml").to_string_lossy().into_owned();
                    filename_pairs_in_out.push((input_filename, output_filename));
                }
            }
        }

        // compile *.jack, export *.xml, and compare with *.xml.org
        for (fin, fout) in filename_pairs_in_out.iter() {
            // tokenize
            let input_file = File::open(fin).expect("cannot open input file");
            let mut t = Tokenizer::new(input_file);
            
            // compile
            let output_file = File::create(fout).expect("cannot open output file");
            let mut e = Engine::new(t, output_file);
            e.compile();

            // compare two files
            //let forg = Path::new(fout).with_extension("xml.org").to_string_lossy().into_owned();
            //let diff_status = Command::new("diff").args(["-b", "-u", "-w", &fout, &forg]).status().expect("failed to execute process");
            //assert!(diff_status.success());
        }
    }

    #[test]
    fn test_expression_case() {
        use super::*;
        use std::path::Path;
        use std::fs::File;
        use std::io::{BufWriter, Write};
        use std::process::Command;
        use crate::tokenizer::*;

        // pair list of full path of *.jack and *.xml files
        let mut filename_pairs_in_out = vec![]; 
        let square_path = Path::new("./jack/Square");
        let array_test_path = Path::new("./jack/ArrayTest");
        for d in [square_path, array_test_path].into_iter() {
            for f in d.read_dir().expect("read_dir call failed") {
                if let Ok(f) = f {
                    if f.path().extension().unwrap() == "jack" {
                        let input_filename = f.path().to_string_lossy().into_owned();
                        let output_filename = f.path().with_extension("xml").to_string_lossy().into_owned();
                        filename_pairs_in_out.push((input_filename, output_filename));
                    }
                }
            }
        }

        // compile *.jack, export *.xml, and compare with *.xml.org
        for (fin, fout) in filename_pairs_in_out.iter() {
            // tokenize
            let input_file = File::open(fin).expect("cannot open input file");
            let mut t = Tokenizer::new(input_file);
            
            // compile
            let output_file = File::create(fout).expect("cannot open output file");
            let mut e = Engine::new(t, output_file);
            e.compile();

            // compare two files
            //let forg = Path::new(fout).with_extension("xml.org").to_string_lossy().into_owned();
            //let diff_status = Command::new("diff").args(["-b", "-u", "-w", &fout, &forg]).status().expect("failed to execute process");
            //assert!(diff_status.success());
        }
    }
}
