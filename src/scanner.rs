use crate::scanner::token::Token;

mod token;
mod error;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    current_line: usize,
    current_char: usize,
}

impl Scanner {
    fn scan_tokens() -> Vec<Token> {

    }
}