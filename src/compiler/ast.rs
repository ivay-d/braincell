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
        Tokens::Dot => Some(Nodes::Nice),
        _ => Some(Nodes::No),
      } {
        self.nodes.push(nodes);
      }
    }
    &self.nodes
  }
}
