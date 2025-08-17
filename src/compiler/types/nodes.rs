#[derive(Debug, Clone, PartialEq)]
pub enum Nodes {
  Plus,
  Pluses(u64),
  Dot,
  Dots(u64),
  No,
}
