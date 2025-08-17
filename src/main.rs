use braincell::{Ast, Lexer};

fn main() {
  let mut lexer = Lexer::new();
  let tokens = lexer.tokenize(".+".to_string());
  let mut ast = Ast::new();
  let nodes = ast.parse(tokens);
  println!("{lexer}\n");
  println!("{ast}");
}
