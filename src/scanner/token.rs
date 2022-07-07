pub enum TokenType {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
    Not,

    LeftParen,
    RightParen,

    RightBracket,
    LeftBracket,

    LeftBrace,
    RightBrace,

    EqualsEquals,
    NotEquals,

    Let,
    Const,
    Fun,

    EndOfFile,
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String
}