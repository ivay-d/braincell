use crate::types::{Tokens, Nodes};
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
    for (_i, word) in tokens.iter().enumerate() {
      if let Some(nodes) = match word {
        Tokens::Plus => match self.nodes.last() {
          None => Some(Nodes::Plus),
          Some(Nodes::Plus) => Some(Nodes::Pluses(2)),
          Some(Nodes::Pluses(value)) => Some(Nodes::Pluses(value + 1)),
          _ => Some(Nodes::Plus),
        },
        Tokens::Dot => match self.nodes.last() {
          None => Some(Nodes::Dot),
          Some(Nodes::Dot) => Some(Nodes::Dots(2)),
          Some(Nodes::Dots(value)) => Some(Nodes::Dots(value + 1)),
          _ => Some(Nodes::Dot),
        },
        _ => Some(Nodes::No),
      } {
        let index = self.nodes.len();
        if matches!(nodes, Nodes::Dots(_)) || matches!(nodes, Nodes::Pluses(_)) {
          self.nodes[index - 1] = nodes;
        } else {
          self.nodes.push(nodes);
        }
      }
    }
    &self.nodes
  }
}
