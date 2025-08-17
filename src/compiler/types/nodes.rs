use crate::types::Char;

#[derive(Debug, Clone, PartialEq)]
pub enum Nodes {
  Plus,
  Pluses(u64),
  Minus,
  Minuses(u64),
  Dot,
  Dots(u64),
  Comma,
  Commas(u64),
  MoveRight,
  MoveRights(u64),
  MoveLeft,
  MoveLefts(u64),
  OpenBracket,
  OpenBrackets(u64),
  CloseBracket,
  CloseBrackets(u64),
  Special(Char),
  Unknown,
}
