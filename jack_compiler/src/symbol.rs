use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Symbol {
    BraceL,
    BraceR,
    ParenL,
    ParenR,
    SqParL,
    SqParR,
    Plus,
    Minus,
    Asterisk,
    Slash,
    And,
    Or,
    Not,
    LessThan,
    GreaterThan,
    Equal,
    Dot,
    Comma,
    SemiColon,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::BraceL      => write!(f, "{"),
            Symbol::BraceR      => write!(f, "}"), 
            Symbol::ParenL      => write!(f, "("),
            Symbol::ParenR      => write!(f, ")"),
            Symbol::SqParL      => write!(f, "["),
            Symbol::SqParR      => write!(f, "]"),
            Symbol::Plus        => write!(f, "+"),
            Symbol::Minus       => write!(f, "-"),
            Symbol::Asterisk    => write!(f, "*"),
            Symbol::Slash       => write!(f, "/"),
            Symbol::And         => write!(f, "&amp;"),
            Symbol::Or          => write!(f, "|"),
            Symbol::Not         => write!(f, "~"),
            Symbol::LessThan    => write!(f, "&lt;"),
            Symbol::GreaterThan => write!(f, "&gt;"),
            Symbol::Equal       => write!(f, "="),
            Symbol::Dot         => write!(f, "."),
            Symbol::Comma       => write!(f, ","),
            Symbol::SemiColon   => write!(f, ";"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UndefinedSymbol;

impl fmt::Display for UndefinedSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Undefined symbol")
    }
}

impl Symbol {
    type Err = UndefinedSymbol;

    fn from_byte(b: u8) -> Result<Self, Self::Err> {
        match b {
            b"{" => Ok(Symbol::BraceL),
            b"}" => Ok(Symbol::BraceR),
            b"(" => Ok(Symbol::ParenL),
            b")" => Ok(Symbol::ParenR),
            b"[" => Ok(Symbol::SqParL),
            b"]" => Ok(Symbol::SqParR),
            b"+" => Ok(Symbol::Plus),
            b"-" => Ok(Symbol::Minus),
            b"*" => Ok(Symbol::Asterisk),
            b"/" => Ok(Symbol::Slash),
            b"&" => Ok(Symbol::And),
            b"|" => Ok(Symbol::Or),
            b"~" => Ok(Symbol::Not),
            b"<" => Ok(Symbol::LessThan),
            b">" => Ok(Symbol::GreaterThan),
            b"=" => Ok(Symbol::Equal),
            b"." => Ok(Symbol::Dot),
            b"," => Ok(Symbol::Comma),
            b";" => Ok(Symbol::SemiColon),
              _  => Err(BadSymbolError),
        }
    }
}