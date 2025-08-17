use crate::types::Char;

#[derive(Debug, Clone)]
pub enum Tokens {
  Plus,
  Minus,
  Dot,
  Comma,
  MoveRight,
  MoveLeft,
  OpenBracket,
  CloseBracket,
  Special(Char),
  Unknown(char),
}
