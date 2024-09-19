use std::iter::Peekable;
use std::vec::IntoIter;


pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Sin,
    Number(f32),
    Ident(String),
    EOF,
}


pub type TokenIter = Peekable<IntoIter<Token>>;