mod tokenizer;
mod engine;
mod keyword;
mod symbol;
mod analyzer;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("usage: jackc <filename>.jack | <dirname>"); }
    let arg_path = Path::new(&args[1]);
    analyzer::Analyzer::run(arg_path);
}