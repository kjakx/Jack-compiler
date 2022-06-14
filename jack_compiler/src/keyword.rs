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

impl FromStr for Keyword {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kw = match s {
            "class" => Ok(Keyword::Class),
            "method" => Ok(Keyword::Method),
            "function" => Ok(Keyword::Function),
            "constructor" => Ok(Keyword::Constructor),
            "int" => Ok(Keyword::Int),
            "boolean" => Ok(Keyword::Boolean),
            "char" => Ok(Keyword::Char),
            "void" => Ok(Keyword::Void),
            "var" => Ok(Keyword::Var),
            "static" => Ok(Keyword::Static),
            "field" => Ok(Keyword::Field),
            "let" => Ok(Keyword::Let),
            "do" => Ok(Keyword::Do),
            "if" => Ok(Keyword::If),
            "else" => Ok(Keyword::Else),
            "while" => Ok(Keyword::While),
            "return" => Ok(Keyword::Return),
            "true" => Ok(Keyword::True),
            "false" => Ok(Keyword::False),
            "null" => Ok(Keyword::Null),
            "this" => Ok(Keyword::This),
            _ => Err("bad keyword");
        }
    }
}