use core::num::flt2dec::strategy::grisu::max_pow10_no_more_than;
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
    fn scan_tokens(&mut self) -> Vec<Token> {
        match
    }

    fn next_char(&mut self) -> char {
        self.source.chars().collect();
    }
}