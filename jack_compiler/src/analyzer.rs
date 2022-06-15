use std::path::Path;
use std::fs::File;
use crate::tokenizer::*;
use crate::engine::*;

pub struct Analyzer {
    engine: Engine,
}

impl Analyzer {
    pub fn new(source: &Path) -> Self {
        let fin = File::open(source).expect("cannot open source file");
        let t = Tokenizer::new(fin);
        let fout = File::create(source.with_extension("xml")).expect("cannot create output file");
        Analyzer {
            engine: Engine::new(t, fout)
        }
    }

    pub fn run(&mut self) {
        self.engine.compile();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_analyzer() {
        use super::*;
        use std::path::Path;
        use std::process::Command;

        let source = Path::new("/workspace/Jack-compiler/jack_compiler/jack/Square/Main.jack");
        let mut a = Analyzer::new(source);
        a.run();
        let fout = source.with_extension("xml").to_string_lossy().into_owned();
        let forg = source.with_extension("xml.org").to_string_lossy().into_owned();
        let diff_status = Command::new("diff").args(["-b", "-u", "-w", &fout, &forg]).status().expect("failed to execute process");
        assert!(diff_status.success());
    }
}