use crate::types::{Tokens, Char};
use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Lexer {
  tokens: Vec<Tokens>,
}

impl Display for Lexer {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self.tokens)
  }
}

impl Lexer {
  pub fn new() -> Self {
    Self {
      tokens: Vec::new()
    }
  }

  pub fn tokenize(&mut self, code: String) -> &Vec<Tokens> {
    for (_i, word) in code.chars().enumerate() {
      if let Some(tokens) = match word {
        '+' => Some(Tokens::Plus),
        '-' => Some(Tokens::Minus),
        '.' => Some(Tokens::Dot),
        ',' => Some(Tokens::Comma),
        '>' => Some(Tokens::MoveRight),
        '<' => Some(Tokens::MoveLeft),
        '[' => Some(Tokens::OpenBracket),
        ']' => Some(Tokens::CloseBracket),
        ' ' => Some(Tokens::Special(Char::Space)),
        _ => Some(Tokens::Unknown(word)),
      } {
        self.tokens.push(tokens);
      }
    }
    &self.tokens
  }
}
