#[derive(Debug, Clone, PartialEq)]
pub enum KW {
    Function,
    If,
    Else,
    For,
    Return,
    Break,
    Continue,
    Print,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    KW(KW),
    Ident(String),
    Str(String),
    Num(i64),
    Boolean(bool),
    Null,

    Plus,
    Minus,
    Mul,
    Div,
    Mod,

    LessThan,   // <
    LessThanEq, // <=
    GreatThan,
    GreatThanEq,
    Eq,    // ==
    NotEq, // !=
    Bang,  // !

    And, // &&
    Or,  // ||

    /// =
    Assign,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Comma, // ,
    Semi,  //;

    Whitespace,
    Eof,
}
