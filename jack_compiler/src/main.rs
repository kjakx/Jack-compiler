mod tokenizer;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_tokenizer_new() {
        use super::tokenizer;
        use std::fs::File;
        let t_els = tokenizer::Tokenizer::new(File::open("/workspace/Jack-compiler/jack_compiler/jack/ExpressionLessSquare/Main.jack").unwrap());
        println!("tokens in ExpressionLessSquare/Main.jack:");
        println!("{:?}", t_els.tokens);
        let t_s = tokenizer::Tokenizer::new(File::open("/workspace/Jack-compiler/jack_compiler/jack/Square/Main.jack").unwrap());
        println!("tokens in Square/Main.jack:");
        println!("{:?}", t_s.tokens);
        let t_at = tokenizer::Tokenizer::new(File::open("/workspace/Jack-compiler/jack_compiler/jack/ArrayTest/Main.jack").unwrap());
        println!("tokens in ArrayTest/Main.jack:");
        println!("{:?}", t_at.tokens);
    }
}
