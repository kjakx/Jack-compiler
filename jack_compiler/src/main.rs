mod tokenizer;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    /*
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
    */

    #[test]
    fn test_tokenizer_export_xml() {
        use super::tokenizer::*;
        use std::path::Path;
        use std::fs::File;
        use std::io::{BufWriter, Write};
        use std::process::Command;

        let jack_src_path = Path::new("/workspace/Jack-compiler/jack_compiler/jack");
        let mut filename_pairs_in_out = vec![]; 
        for dir in jack_src_path.read_dir().expect("read_dir call failed") {
            if let Ok(dir) = dir {
                for f in dir.path().read_dir().expect("read_dir call failed") {
                    if let Ok(f) = f {
                        if f.path().extension().unwrap() == "jack" {
                            let input_filename = f.path().to_string_lossy().into_owned();
                            let output_filename = dir.path().join(f.path().file_stem().unwrap()).to_string_lossy().into_owned()+"T.xml";
                            filename_pairs_in_out.push((input_filename, output_filename));
                        }
                    }
                }
            }
        }

        for (fin, fout) in filename_pairs_in_out.iter() {
            let input_file = File::open(fin).expect("cannot open input file");
            let output_file = File::create(fout).expect("cannot open output file");
            let mut t = Tokenizer::new(input_file);
            let mut w = BufWriter::<File>::new(output_file);

            // export xml
            writeln!(w, "<tokens>").unwrap();
            while t.has_more_tokens() {
                t.advance();
                match t.token_type() {
                    Token::Keyword(s) => {
                        writeln!(w, "<keyword> {} </keyword>", s).unwrap();
                    },
                    Token::Symbol(c) => {
                        match c {
                            '&' => {
                                writeln!(w, "<symbol> &amp; </symbol>").unwrap();
                            },
                            '<' => {
                                writeln!(w, "<symbol> &lt; </symbol>").unwrap();
                            },
                            '>' => {
                                writeln!(w, "<symbol> &gt; </symbol>").unwrap();
                            },
                            c => {
                                writeln!(w, "<symbol> {} </symbol>", c).unwrap();
                            }
                        }
                    },
                    Token::Identifier(s) => {
                        writeln!(w, "<identifier> {} </identifier>", s).unwrap();
                    },
                    Token::IntConst(i) => {
                        writeln!(w, "<integerConstant> {} </integerConstant>", i).unwrap();
                    },
                    Token::StringConst(s) => {
                        writeln!(w, "<stringConstant> {} </stringConstant>", s).unwrap();
                    },
                    Token::Empty() => { unreachable!(); }
                }
            }
            writeln!(w, "</tokens>").unwrap();
            w.flush().unwrap();

            // compare two files
            let forg = Path::new(fout).with_extension("xml.org").to_string_lossy().into_owned();
            let diff_status = Command::new("diff").args(["-b", "-u", &fout, &forg]).status().expect("failed to execute process");
            assert!(diff_status.success());
        }
    }
}
