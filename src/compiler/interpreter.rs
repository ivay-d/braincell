use crate::types::Nodes;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Interpreter {
  cell_limit: u128,
  current_cell: u128,
  cell_values: Vec<u128>,
}

impl Display for Interpreter {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self.cell_values)
  }
}

impl Interpreter {
  pub fn new(cell_limit: u128) -> Self {
    Self {
      cell_limit,
      current_cell: 0,
      cell_values: Vec::new(),
    }
  }

  pub fn change_cell_limit(&mut self, new_limit: u128) {
    self.cell_limit = new_limit;
  }

  pub fn run(&mut self, nodes: &Vec<Nodes>) {
    self.cell_values.push(0);
    for word in nodes.iter() {
      match word {
        Nodes::MoveRight => self.cell_values.push(0),
        Nodes::MoveRights(value) => for _ in 0..*value {
          self.cell_values.push(0);
        },
        _ => self.cell_values.push(0),
      }
    }
  }
}
