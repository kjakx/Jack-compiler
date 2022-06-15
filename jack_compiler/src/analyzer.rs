use std::path::Path;
use std::fs::File;
use crate::tokenizer::*;
use crate::engine::*;

pub struct Analyzer;

impl Analyzer {
    pub fn run(source: &Path) {
        if source.is_dir() {
            for f in source.read_dir().expect("read_dir call failed") {
                if let Ok(f) = f {
                    if f.path().extension().unwrap() == "jack" {
                        let fin = File::open(f.path()).expect("cannot create source file");
                        let fout = File::create(f.path().with_extension("xml")).expect("cannot create output file");
                        let t = Tokenizer::new(fin);
                        let mut e = Engine::new(t, fout);
                        e.compile();
                    }
                }
            }
        } else {
            let fin = File::open(source).expect("cannot create source file");
            let fout = File::create(source.with_extension("xml")).expect("cannot create output file");
            let t = Tokenizer::new(fin);
            let mut e = Engine::new(t, fout);
            e.compile();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_analyze_from_file() {
        use super::*;
        use std::path::Path;
        use std::process::Command;

        let source = Path::new("/workspace/Jack-compiler/jack_compiler/jack/Square/Main.jack");
        Analyzer::run(source);
        let fout = source.with_extension("xml").to_string_lossy().into_owned();
        let forg = source.with_extension("xml.org").to_string_lossy().into_owned();
        let diff_status = Command::new("diff").args(["-b", "-u", "-w", &fout, &forg]).status().expect("failed to execute process");
        assert!(diff_status.success());
    }

    #[test]
    fn test_analyze_from_dir() {
        use super::*;
        use std::path::Path;
        use std::process::Command;

        let source_dir = Path::new("/workspace/Jack-compiler/jack_compiler/jack/Square/");
        for f in source_dir.read_dir().expect("read_dir call failed") {
            if let Ok(f) = f {
                if f.path().extension().unwrap() == "jack" {
                    Analyzer::run(&f.path());
                    let fout = f.path().with_extension("xml").to_string_lossy().into_owned();
                    let forg = f.path().with_extension("xml.org").to_string_lossy().into_owned();
                    let diff_status = Command::new("diff").args(["-b", "-u", "-w", &fout, &forg]).status().expect("failed to execute process");
                    assert!(diff_status.success());
                }
            }
        }
    }
}