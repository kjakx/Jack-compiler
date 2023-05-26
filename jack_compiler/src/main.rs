mod tokenizer;
mod engine;
mod keyword;
mod symbol;
//mod analyzer;
mod compiler;
mod symbol_table;
mod vm_writer;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("usage: jackc <filename>.jack | <dirname>"); }
    let arg_path = Path::new(&args[1]);
    compiler::Compiler::run(arg_path);
}