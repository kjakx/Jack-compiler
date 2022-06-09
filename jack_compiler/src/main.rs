mod tokenizer;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_tokenizer() {
        use super::tokenizer;
        use std::fs::File;
        let t = tokenizer::Tokenizer::new(File::open("/workspace/Jack-compiler/jack_compiler/jack/ExpressionLessSquare/Main.jack").unwrap());
        println!("tokens: {:?}", t.tokens);
    }
}
