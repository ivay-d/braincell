#[cfg(test)]
mod tests {
  use braincell::{Lexer, Ast, types::{Tokens, Char, Nodes}};

  #[test]
  fn run_lexer() {
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize("+-><,. []".to_string());
    assert_eq!(tokens, &vec![Tokens::Plus, Tokens::Minus, Tokens::MoveRight, Tokens::MoveLeft, Tokens::Comma, Tokens::Dot, Tokens::Special(Char::Space),Tokens::OpenBracket, Tokens::CloseBracket])
  }

  #[test]
  fn run_ast() {
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize("+ ++ +++ - -- --- > >> >>> < << <<< , ,, ,,, . .. ... [ [[ [[[ ] ]] ]]]".to_string());
    let mut ast = Ast::new();
    let nodes = ast.parse(tokens);
    assert_eq!(nodes, &vec![Nodes::Plus, Nodes::Special(Char::Space), Nodes::Pluses(2), Nodes::Special(Char::Space), Nodes::Pluses(3), Nodes::Special(Char::Space), Nodes::Minus, Nodes::Special(Char::Space), Nodes::Minuses(2), Nodes::Special(Char::Space), Nodes::Minuses(3), Nodes::Special(Char::Space), Nodes::MoveRight, Nodes::Special(Char::Space), Nodes::MoveRights(2), Nodes::Special(Char::Space), Nodes::MoveRights(3), Nodes::Special(Char::Space), Nodes::MoveLeft, Nodes::Special(Char::Space), Nodes::MoveLefts(2), Nodes::Special(Char::Space), Nodes::MoveLefts(3), Nodes::Special(Char::Space), Nodes::Comma, Nodes::Special(Char::Space), Nodes::Commas(2), Nodes::Special(Char::Space), Nodes::Commas(3), Nodes::Special(Char::Space), Nodes::Dot, Nodes::Special(Char::Space), Nodes::Dots(2), Nodes::Special(Char::Space), Nodes::Dots(3), Nodes::Special(Char::Space), Nodes::OpenBracket, Nodes::Special(Char::Space), Nodes::OpenBrackets(2), Nodes::Special(Char::Space), Nodes::OpenBrackets(3), Nodes::Special(Char::Space), Nodes::CloseBracket, Nodes::Special(Char::Space), Nodes::CloseBrackets(2), Nodes::Special(Char::Space), Nodes::CloseBrackets(3)])
  }
}
