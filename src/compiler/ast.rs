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
        Tokens::Dot => {
          if self.nodes.is_empty() {
            Some(Nodes::Dot)
          } else if self.nodes[self.nodes.len() - 1] == Nodes::Dot {
            Some(Nodes::Dots(2))
          } else if matches!(self.nodes[self.nodes.len() - 1], Nodes::Dots(_)) {
            if let Some(Nodes::Dots(value)) = self.nodes.last() {
              Some(Nodes::Dots(value + 1))
            } else {
              panic!("Expected value inside of Nodes::Dots")
            }
          } else {
            Some(Nodes::Dot)
          }
        },
        _ => Some(Nodes::No),
      } {
        let index = self.nodes.len();
        if matches!(nodes, Nodes::Dots(_)) {
          self.nodes[index - 1] = nodes;
        } else {
          self.nodes.push(nodes);
        }
      }
    }
    &self.nodes
  }
}
