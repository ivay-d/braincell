use crate::types::{Tokens, Nodes, Char};
use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Ast {
  nodes: Vec<Nodes>,
}

impl Display for Ast {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self.nodes)
  }
}

impl Ast {
  pub fn new() -> Self {
    Self {
      nodes: Vec::new(),
    }
  }

  pub fn parse(&mut self, tokens: &Vec<Tokens>) -> &Vec<Nodes> {
    for word in tokens.iter() {
      if let Some(nodes) = match word {
        Tokens::Plus => match self.nodes.last() {
          Some(Nodes::Plus) => Some(Nodes::Pluses(2)),
          Some(Nodes::Pluses(value)) => Some(Nodes::Pluses(value + 1)),
          _  => Some(Nodes::Plus),
        },
        Tokens::Minus => match self.nodes.last() {
          Some(Nodes::Minus) => Some(Nodes::Minuses(2)),
          Some(Nodes::Minuses(value)) => Some(Nodes::Minuses(value + 1)),
          _ => Some(Nodes::Minus)
        },
        Tokens::Dot => match self.nodes.last() {
          Some(Nodes::Dot) => Some(Nodes::Dots(2)),
          Some(Nodes::Dots(value)) => Some(Nodes::Dots(value + 1)),
          _ => Some(Nodes::Dot),
        },
        Tokens::Comma => match self.nodes.last() {
          Some(Nodes::Comma) => Some(Nodes::Commas(2)),
          Some(Nodes::Commas(value)) => Some(Nodes::Commas(value + 1)),
          _ => Some(Nodes::Comma),
        },
        Tokens::MoveRight => match self.nodes.last() {
          Some(Nodes::MoveRight) => Some(Nodes::MoveRights(2)),
          Some(Nodes::MoveRights(value)) => Some(Nodes::MoveRights(value + 1)),
          _ => Some(Nodes::MoveRight),
        },
        Tokens::MoveLeft => match self.nodes.last() {
          Some(Nodes::MoveLeft) => Some(Nodes::MoveLefts(2)),
          Some(Nodes::MoveLefts(value)) => Some(Nodes::MoveLefts(value + 1)),
          _ => Some(Nodes::MoveLeft),
        },
        Tokens::OpenBracket => match self.nodes.last() {
          Some(Nodes::OpenBracket) => Some(Nodes::OpenBrackets(2)),
          Some(Nodes::OpenBrackets(value)) => Some(Nodes::OpenBrackets(value + 1)),
          _ => Some(Nodes::OpenBracket),
        },
        Tokens::CloseBracket => match self.nodes.last() {
          Some(Nodes::CloseBracket) => Some(Nodes::CloseBracket),
          Some(Nodes::CloseBrackets(value)) => Some(Nodes::CloseBrackets(value + 1)),
          _ => Some(Nodes::CloseBracket),
        },
        Tokens::Special(Char::Space) => match self.nodes.last() {
          Some(Nodes::Special(Char::Space)) => Some(Nodes::Special(Char::Spaces(2))),
          Some(Nodes::Special(Char::Spaces(value))) => Some(Nodes::Special(Char::Spaces(value + 1))),
          _ => Some(Nodes::Special(Char::Space)),
        },
        _ => Some(Nodes::Unknown),
      } {
        let index = self.nodes.len();
        if matches!(nodes, Nodes::Pluses(_)) || matches!(nodes, Nodes::Minuses(_)) || matches!(nodes, Nodes::Dots(_)) || matches!(nodes, Nodes::Commas(_)) || matches!(nodes, Nodes::MoveRights(_)) || matches!(nodes, Nodes::MoveLefts(_)) || matches!(nodes, Nodes::OpenBrackets(_)) || matches!(nodes, Nodes::CloseBrackets(_)) || matches!(nodes, Nodes::Special(Char::Spaces(_))) {
          self.nodes[index - 1] = nodes;
        } else {
          self.nodes.push(nodes);
        }
      }
    }
    &self.nodes
  }
}
