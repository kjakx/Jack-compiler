use std::io::{BufWriter, Write};
use std::fs::File;

pub struct Engine {
    writer: BufWriter<File>,
}

impl Engine {
    pub fn new(f: File) -> Self { unimplemented!(); }
    pub fn compile_class(&mut self) { unimplemented!(); }
    pub fn compile_class_var_dec(&mut self) { unimplemented!(); }
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