use std::io::{BufWriter, Write};
use std::fs::File;
use crate::tokenizer::*;
use crate::keyword::*;
use crate::symbol::*;
use crate::symbol_table::*;
use crate::vm_writer::*;

pub struct Engine {
    tokenizer: Tokenizer,
    sym_tbl: SymbolTable,
    //writer: BufWriter<File>,
    vm_writer: VMWriter,
    class_name: String,
    if_count: usize,
    while_count: usize,
}

impl Engine {
    pub fn new(t: Tokenizer, f: File, cn: String) -> Self {
        Engine {
            tokenizer: t,
            sym_tbl: SymbolTable::new(),
            //writer: BufWriter::<File>::new(f),
            vm_writer: VMWriter::new(f),
            class_name: cn,
            if_count: 0,
            while_count: 0,
        }
    }
    
    pub fn compile(&mut self) {
        self.compile_class();
        //self.writer.flush().unwrap();
    }

    pub fn compile_class(&mut self) {
        // 'class' className '{'
        self.compile_keyword_expect(Keyword::Class);
        let cls_name = self.compile_class_name();
        self.compile_symbol_expect(Symbol::BraceL);
        self.sym_tbl.define("this", VarKind::Field, VarType::ClassName(cls_name));
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
    }

    pub fn compile_class_var_dec(&mut self) {
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
            self.compile_var_name_defined(varkind, vartype.clone());
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
    }
    
    pub fn compile_subroutine_dec(&mut self) {
        self.sym_tbl.start_subroutine();
        // 'constructor' | 'function' | 'method'
        let subroutine_type = match self.tokenizer.peek_next_token().unwrap() {
            Token::Keyword(Keyword::Constructor | Keyword::Function | Keyword::Method) => {
                self.compile_keyword()
            },
            t => {
                panic!("'constructor', 'function' or 'method' expected, found {:?}", t);
            }
        };
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
        let fname = self.class_name.clone() + "." + &self.compile_subroutine_name();
        self.compile_symbol_expect(Symbol::ParenL);
        let count = self.compile_parameter_list();
        self.compile_symbol_expect(Symbol::ParenR);
        // subroutineBody
        self.compile_subroutine_body(&fname, subroutine_type);
        //writeln!(self.writer, "</subroutineDec>").unwrap();
    }

    pub fn compile_subroutine_body(&mut self, fun_name: &str, subroutine_type: Keyword) {
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
        self.vm_writer.write_function(fun_name, self.sym_tbl.var_count(VarKind::Var) as i16);
        match subroutine_type {
            Keyword::Constructor => {
                let size = self.sym_tbl.var_count(VarKind::Field) + 1;
                self.vm_writer.write_push(Segment::Const, size as i16);
                self.vm_writer.write_call("Memory.alloc", 1);
                self.vm_writer.write_pop(Segment::Pointer, 0);
            },
            Keyword::Method => {
                self.vm_writer.write_push(Segment::Arg, 0);
                self.vm_writer.write_pop(Segment::Pointer, 0);
            }
            _ => (),
        }
        // statements
        self.compile_statements();
        // '}'
        self.compile_symbol_expect(Symbol::BraceR);
    }

    pub fn compile_parameter_list(&mut self) -> i16 {
        let mut count = 0;
        // (type varName (',' type varName)*)?
        'parameterList: loop {
            // type
            if let Ok(vartype) = self.compile_type() {
                // varName
                self.compile_var_name_defined(VarKind::Arg, vartype);
                count += 1;
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
        count
    }

    pub fn compile_var_dec(&mut self) {
        // 'var'
        self.compile_keyword_expect(Keyword::Var);
        // type
        let vartype = self.compile_type().unwrap();
        // varName (',' varName)*
        'varName: loop {
            // varName
            self.compile_var_name_defined(VarKind::Var, vartype.clone());
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
    }

    pub fn compile_statements(&mut self) {
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
    }

    pub fn compile_do(&mut self) {
        // 'do' subroutineCall ';'
        self.compile_keyword_expect(Keyword::Do);
        self.compile_subroutine_call();
        self.compile_symbol_expect(Symbol::SemiColon);
        self.vm_writer.write_pop(Segment::Temp, 0);
    }

    pub fn compile_let(&mut self) {
        // 'let' varName 
        self.compile_keyword_expect(Keyword::Let);
        let var_name = self.compile_var_name_used();
        let mut var_seg = self._seg_of(&var_name);
        /*
        if var_seg == Segment::This {
            self.vm_writer.write_push(Segment::Pointer, 0);
            self.vm_writer.write_pop(Segment::This, 0);
        }
        */
        let mut var_index = *self.sym_tbl.index_of(&var_name).unwrap() as i16;
        // ('[' expression ']')?
        if let &Token::Symbol(Symbol::SqParL) = self.tokenizer.peek_next_token().unwrap() {
            // '[' expression ']'
            self.vm_writer.write_push(var_seg, var_index);
            self.compile_symbol_expect(Symbol::SqParL);
            self.compile_expression(); // array index
            self.compile_symbol_expect(Symbol::SqParR);
            self.vm_writer.write_arithmetic(Command::Add);
            self.vm_writer.write_pop(Segment::Pointer, 1);
            var_seg = Segment::That;
            var_index = 0;
        }

        // '=' expression ';'
        self.compile_symbol_expect(Symbol::Equal);
        self.compile_expression();
        self.compile_symbol_expect(Symbol::SemiColon);

        self.vm_writer.write_pop(var_seg, var_index);
    }

    pub fn compile_while(&mut self) {
        // 'while' '(' expression ')'
        let w_cnt = self.while_count;
        self.while_count += 1;
        self.compile_keyword_expect(Keyword::While);
        let while_label = format!("WHILE_EXP{}", w_cnt);
        self.vm_writer.write_label(&while_label);
        self.compile_symbol_expect(Symbol::ParenL);
        self.compile_expression();
        self.compile_symbol_expect(Symbol::ParenR);
        self.vm_writer.write_arithmetic(Command::Not);
        let while_end_label = format!("WHILE_END{}", w_cnt);
        self.vm_writer.write_if(&while_end_label);

        // '{' statements '}'
        self.compile_symbol_expect(Symbol::BraceL);
        self.compile_statements();
        self.compile_symbol_expect(Symbol::BraceR);
        self.vm_writer.write_goto(&while_label);
        self.vm_writer.write_label(&while_end_label);
    }

    pub fn compile_return(&mut self) {
        // 'return'
        self.compile_keyword_expect(Keyword::Return);
        // expression?
        match self.tokenizer.peek_next_token().unwrap() {
            &Token::Symbol(Symbol::SemiColon) => {
                self.vm_writer.write_push(Segment::Const, 0);
            },
            _ => {
                self.compile_expression();
            }
        }
        // ';'
        self.compile_symbol_expect(Symbol::SemiColon);
        self.vm_writer.write_return();
    }

    pub fn compile_if(&mut self) {
        let i_cnt = self.if_count;
        self.if_count += 1;
        // 'if' '(' expression ')'
        self.compile_keyword_expect(Keyword::If);
        self.compile_symbol_expect(Symbol::ParenL);
        self.compile_expression();
        self.compile_symbol_expect(Symbol::ParenR);
        let if_true_label = format!("IF_TRUE{}", i_cnt);
        let if_false_label = format!("IF_FALSE{}", i_cnt);
        let if_end_label = format!("IF_END{}", i_cnt);
        self.vm_writer.write_if(&if_true_label);
        self.vm_writer.write_goto(&if_false_label);
        // '{' statements '}'
        self.vm_writer.write_label(&if_true_label);
        self.compile_symbol_expect(Symbol::BraceL);
        self.compile_statements();
        self.compile_symbol_expect(Symbol::BraceR);
        self.vm_writer.write_goto(&if_end_label);
        // ('else' '{' statements '}')?
        self.vm_writer.write_label(&if_false_label);
        if let &Token::Keyword(Keyword::Else) = self.tokenizer.peek_next_token().unwrap() {
            // 'else' '{' statements '}'
            self.compile_keyword_expect(Keyword::Else);
            self.compile_symbol_expect(Symbol::BraceL);
            self.compile_statements();
            self.compile_symbol_expect(Symbol::BraceR);
        }
        self.vm_writer.write_label(&if_end_label);
    }

    pub fn compile_expression(&mut self) {
        // term
        self.compile_term();
        // (op term)*
        'term: loop {
            match self.tokenizer.peek_next_token().unwrap() {
                Token::Symbol(
                    Symbol::Plus | Symbol::Minus | Symbol::Asterisk | Symbol::Slash |
                    Symbol::And  | Symbol::Or    | Symbol::LessThan | Symbol::GreaterThan | Symbol::Equal
                ) => {
                    let sym = self.compile_symbol();
                    self.compile_term();
                    match sym {
                        Symbol::Plus => {
                            self.vm_writer.write_arithmetic(Command::Add);
                        },
                        Symbol::Minus => {
                            self.vm_writer.write_arithmetic(Command::Sub);
                        },
                        Symbol::Asterisk => {
                            self.vm_writer.write_call("Math.multiply", 2);
                        },
                        Symbol::Slash => {
                            self.vm_writer.write_call("Math.divide", 2);
                        },
                        Symbol::And => {
                            self.vm_writer.write_arithmetic(Command::And);
                        },
                        Symbol::Or => {
                            self.vm_writer.write_arithmetic(Command::Or);
                        },
                        Symbol::LessThan => {
                            self.vm_writer.write_arithmetic(Command::Lt);
                        },
                        Symbol::GreaterThan => {
                            self.vm_writer.write_arithmetic(Command::Gt);
                        },
                        Symbol::Equal => {
                            self.vm_writer.write_arithmetic(Command::Eq);
                        }
                        _ => { unreachable!(); }
                    };
                    //self.compile_term();
                },
                _ => {
                    break 'term;
                }
            }
            //self.compile_term();
        }
    }

    pub fn compile_term(&mut self) {
        match self.tokenizer.peek_next_token().unwrap() {
            &Token::IntConst(_) => {
                let i = self.compile_integer_constant();
                self.vm_writer.write_push(Segment::Const, i);
            },
            &Token::StringConst(_) => {
                let s = self.compile_string_constant();
                let length = s.len() as i16;
                self.vm_writer.write_push(Segment::Const, length);
                self.vm_writer.write_call("String.new", 1);
                for i in 0..s.len() {
                    self.vm_writer.write_push(Segment::Const, s.chars().nth(i).unwrap() as i16);
                    self.vm_writer.write_call("String.appendChar", 1);
                }
            },
            &Token::Keyword(Keyword::True | Keyword::False | Keyword::Null | Keyword::This) => {
                let kw = self.compile_keyword();
                match kw {
                    Keyword::True => {
                        self.vm_writer.write_push(Segment::Const, 1);
                        self.vm_writer.write_arithmetic(Command::Neg);
                    },
                    Keyword::False | Keyword::Null => {
                        self.vm_writer.write_push(Segment::Const, 0);
                    },
                    Keyword::This => {
                        self.vm_writer.write_push(Segment::Pointer, 0);
                    },
                    _ => { unreachable!() }
                }
            },
            &Token::Identifier(_) => {
                match self.tokenizer.peek_2nd_next_token().unwrap() {
                    &Token::Symbol(Symbol::SqParL) => {
                        // varName '[' expression ']'
                        let var_name = self.compile_var_name_used();
                        let var_seg = self._seg_of(&var_name);
                        let var_index = *self.sym_tbl.index_of(&var_name).unwrap() as i16;
                        self.vm_writer.write_push(var_seg, var_index);
                        self.compile_symbol_expect(Symbol::SqParL);
                        self.compile_expression(); // array index
                        self.compile_symbol_expect(Symbol::SqParR);
                        self.vm_writer.write_arithmetic(Command::Add);
                        self.vm_writer.write_pop(Segment::Pointer, 1);
                        self.vm_writer.write_push(Segment::That, 0);
                    },
                    &Token::Symbol(Symbol::ParenL | Symbol::Dot) => {
                        // subroutineCall
                        self.compile_subroutine_call();
                    },
                    _ => {
                        // varName
                        let var_name = self.compile_var_name_used();
                        let var_seg = self._seg_of(&var_name);
                        let var_index = *self.sym_tbl.index_of(&var_name).unwrap() as i16;
                        self.vm_writer.write_push(var_seg, var_index);
                    }
                }
            },
            &Token::Symbol(Symbol::Minus | Symbol::Not) => {
                // unaryOp term
                let sym = self.compile_symbol();
                self.compile_term();
                match sym {
                    Symbol::Minus => {
                        self.vm_writer.write_arithmetic(Command::Neg);
                    },
                    Symbol::Not => {
                        self.vm_writer.write_arithmetic(Command::Not);
                    },
                    _ => { unreachable!(); }
                }
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
    }

    pub fn compile_expression_list(&mut self) -> i16 {
        let mut count = 0;
        // (expression (',' expression)* )?
        match self.tokenizer.peek_next_token().unwrap() {
            &Token::Symbol(Symbol::ParenR) => (),
            _ => {
                // expression (',' expression)*
                self.compile_expression();
                count += 1;
                'expression: loop {
                    match self.tokenizer.peek_next_token().unwrap() {
                        &Token::Symbol(Symbol::Comma) => {
                            self.compile_symbol();
                        },
                        _ => { break 'expression; }
                    }
                    self.compile_expression();
                    count += 1;
                }
            }
        }
        count
    }

    pub fn compile_subroutine_call(&mut self) {
        let mut is_method = false;
        // function | method | constructor?
        let sym = match self.tokenizer.peek_2nd_next_token().unwrap() {
            Token::Symbol(sym) => {
                sym
            },
            t => { panic!("symbol expected, found {:?}", t); }
        };
        let cls_name = if *sym == Symbol::Dot {
            // (className | varName) '.' subroutineName
            let i = match self.tokenizer.peek_next_token().unwrap() {
                Token::Identifier(i) => i,
                t => { panic!("identifier expected, found {:?}", t); }
            };
            let cls_name = if self.sym_tbl.contains(&i) { // method
                is_method = true;
                let var_name = self.compile_var_name_used();
                let var_seg = self._seg_of(&var_name);
                let var_index = *self.sym_tbl.index_of(&var_name).unwrap() as i16;
                self.vm_writer.write_push(var_seg, var_index);
                let cn = match self.sym_tbl.type_of(&var_name).unwrap() {
                    VarType::ClassName(cn) => cn,
                    vt => { panic!("class name expected, found {:?}", vt); } 
                };
                cn.clone()
            } else { // function or constructor
                self.compile_class_name()
            };
            self.compile_symbol_expect(Symbol::Dot);
            cls_name
        } else { // method call within its belonging class
            is_method = true;
            self.vm_writer.write_push(Segment::Pointer, 0);
            self.class_name.clone()
        };
        let fun_name = self.compile_subroutine_name();
        let fname = format!("{}.{}", cls_name, fun_name);
        // '(' expressionList ')'
        self.compile_symbol_expect(Symbol::ParenL);
        let mut num_exp = self.compile_expression_list();
        self.compile_symbol_expect(Symbol::ParenR);
        if is_method {
            num_exp += 1;
        }
        self.vm_writer.write_call(&fname, num_exp);
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

    fn compile_keyword(&mut self) -> Keyword {
        match self.tokenizer.get_next_token() {
            Token::Keyword(kw) => {
                //writeln!(self.writer, "<keyword> {} </keyword>", kw).unwrap();
                kw
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

    fn compile_symbol(&mut self) -> Symbol {
        match self.tokenizer.get_next_token() {
            Token::Symbol(sym) => {
                //writeln!(self.writer, "<symbol> {} </symbol>", sym).unwrap();
                sym
            },
            t => {
                panic!("symbol expected, found {:?}", t);
            }
        }
    }

    fn compile_identifier(&mut self) -> String {
        match self.tokenizer.get_next_token() {
            Token::Identifier(ident) => {
                //writeln!(self.writer, "<identifier> {} </identifier>", ident).unwrap();
                ident
            },
            t => {
                panic!("identifier expected, found {:?}", t);
            }
        }
    }

    fn compile_integer_constant(&mut self) -> i16 {
        match self.tokenizer.get_next_token() {
            Token::IntConst(int_const) => {
                //writeln!(self.writer, "<integerConstant> {} </integerConstant>", int_const).unwrap();
                int_const
            },
            t => {
                panic!("integerConstant expected, found {:?}", t);
            }
        }
    }

    fn compile_string_constant(&mut self) -> String {
        match self.tokenizer.get_next_token() {
            Token::StringConst(str_const) => {
                //writeln!(self.writer, "<stringConstant> {} </stringConstant>", str_const).unwrap();
                str_const
            },
            t => {
                panic!("stringConstant expected, found {:?}", t);
            }
        }
    }

    fn compile_class_name(&mut self) -> String {
        self.compile_identifier()
    }

    fn compile_subroutine_name(&mut self) -> String {
        self.compile_identifier()
    }

    fn compile_var_name_defined(&mut self, var_kind: VarKind, var_type: VarType) -> String {
        match self.tokenizer.get_next_token() {
            Token::Identifier(ident) => {
                self.sym_tbl.define(&ident, var_kind, var_type);
                // let var_index = self.sym_tbl.index_of(&ident).expect(format!("unknown identifier {}", ident).as_str());
                //writeln!(self.writer, "<varName(defined)> {}[{}] {} {} </varName(defined)>", var_kind, *var_index, var_type, ident).unwrap();
                ident
            },
            t => {
                panic!("identifier expected, found {:?}", t);
            }
        }
    }

    fn compile_var_name_used(&mut self) -> String {
        match self.tokenizer.get_next_token() {
            Token::Identifier(ident) => {
                if self.sym_tbl.contains(&ident) {
                    let var_kind = self.sym_tbl.kind_of(&ident).expect(format!("unknown identifier {}", ident).as_str());
                    let var_type = self.sym_tbl.type_of(&ident).expect(format!("unknown identifier {}", ident).as_str());
                    let var_index = self.sym_tbl.index_of(&ident).expect(format!("unknown identifier {}", ident).as_str());
                    //writeln!(self.writer, "<varName(used)> {}[{}] {} {} </varName(used)>", var_kind, *var_index, var_type, ident).unwrap();
                } else {
                    unreachable!();
                    //writeln!(self.writer, "<className> {} </className>", ident).unwrap();
                }
                ident
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
                let class_name = self.compile_class_name();
                Ok(VarType::ClassName(class_name))
            },
            t => {
                Err(format!("type expected, found {:?}", t))
            }
        }
    }

    fn _seg_of(&self, var_name: &str) -> Segment {
        match self.sym_tbl.kind_of(&var_name) {
            Some(k) => {
                //let index = *self.sym_tbl.index_of(&var_name).unwrap()
                match k {
                    VarKind::Static => {
                        Segment::Static
                    },
                    VarKind::Field => {
                        Segment::This
                    },
                    VarKind::Arg => {
                        Segment::Arg
                    },
                    VarKind::Var => {
                        Segment::Local
                    },
                }
            },
            None => {
                panic!("variable is not declared");
            }
        }
    }
}

/*
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
*/