#[cfg(test)]
mod tests {
  use braincell::{Lexer, types::{Tokens, Char}};

  #[test]
  fn run_lexer() {
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize("+-><,. []".to_string());
    assert_eq!(tokens, &vec![Tokens::Plus, Tokens::Minus, Tokens::MoveRight, Tokens::MoveLeft, Tokens::Comma, Tokens::Dot, Tokens::Special(Char::Space),Tokens::OpenBracket, Tokens::CloseBracket])
  }
}
