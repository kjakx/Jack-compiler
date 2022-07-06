use std::io::{BufWriter, Write};
use std::fs::File;
use std::fmt;

pub enum Segment {
    Const,
    Arg,
    Local,
    Static,
    This,
    That,
    Pointer,
    Temp,
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Segment::Const => {
                write!(f, "const")
            },
            Segment::Arg => {
                write!(f, "argument")
            },
            Segment::Local => {
                write!(f, "local")
            },
            Segment::Static => {
                write!(f, "static")
            },
            Segment::This => {
                write!(f, "this")
            },
            Segment::That => {
                write!(f, "that")
            },
            Segment::Pointer => {
                write!(f, "pointer")
            },
            Segment::Temp => {
                write!(f, "temp")
            },
        }
    }
}

pub enum Command {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Add => {
                write!(f, "add")
            },
            Command::Sub => {
                write!(f, "sub")
            },
            Command::Neg => {
                write!(f, "neg")
            },
            Command::Eq => {
                write!(f, "eq")
            },
            Command::Gt => {
                write!(f, "gt")
            },
            Command::Lt => {
                write!(f, "lt")
            },
            Command::And => {
                write!(f, "and")
            },
            Command::Or => {
                write!(f, "or")
            },
            Command::Not => {
                write!(f, "not")
            },
        }
    }
}

pub struct VMWriter {
    writer: BufWriter<File>,
}

impl VMWriter {
    pub fn new(f: File) -> Self {
        VMWriter {
            writer: BufWriter::<File>::new(f),
        }
    }

    pub fn write_push(&mut self, segment: Segment, index: i16) {
        writeln!(self.writer, "push {} {}", segment.to_string(), index).unwrap();
    }

    pub fn write_pop(&mut self, segment: Segment, index: i16) {
        writeln!(self.writer, "pop {} {}", segment.to_string(), index).unwrap();
    }

    pub fn write_arithmetic(&mut self, command: Command) {
       writeln!(self.writer, "{}", command.to_string()).unwrap();
    }

    pub fn write_label(&mut self, label: &str) {
        writeln!(self.writer, "label {}", label).unwrap();
    }

    pub fn write_goto(&mut self, label: &str) {
        writeln!(self.writer, "goto {}", label).unwrap();
    }

    pub fn write_if(&mut self, label: &str) {
        writeln!(self.writer, "if-goto {}", label).unwrap();
    }

    pub fn write_call(&mut self, name: &str, n_args: i16) {
        writeln!(self.writer, "call {} {}", name, n_args).unwrap();
    }

    pub fn write_function(&mut self, name: &str, n_locals: i16) {
        writeln!(self.writer, "function {} {}", name, n_locals).unwrap();
    }

    pub fn write_return(&mut self) {
        writeln!(self.writer, "return").unwrap();
    }

    pub fn close(&mut self) {
        self.writer.flush().unwrap();
    }
}