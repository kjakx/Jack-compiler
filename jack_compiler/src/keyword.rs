use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Class,
    Method,
    Function,
    Constructor,
    Int,
    Boolean,
    Char,
    Void,
    Var,
    Static,
    Field,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    This,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::Class       => write!(f, "class"),
            Keyword::Method      => write!(f, "method"), 
            Keyword::Function    => write!(f, "function"),
            Keyword::Constructor => write!(f, "constructor"),
            Keyword::Int         => write!(f, "int"),
            Keyword::Boolean     => write!(f, "boolean"),
            Keyword::Char        => write!(f, "char"),
            Keyword::Void        => write!(f, "void"),
            Keyword::Var         => write!(f, "var"),
            Keyword::Static      => write!(f, "static"),
            Keyword::Field       => write!(f, "field"),
            Keyword::Let         => write!(f, "let"),
            Keyword::Do          => write!(f, "do"),
            Keyword::If          => write!(f, "if"),
            Keyword::Else        => write!(f, "else"),
            Keyword::While       => write!(f, "while"),
            Keyword::Return      => write!(f, "return"),
            Keyword::True        => write!(f, "true"),
            Keyword::False       => write!(f, "false"),
            Keyword::Null        => write!(f, "null"),
            Keyword::This        => write!(f, "this"),
        }
    }
}