/*
MIT License

Copyright (c) 2025 Ivaylo D. Y.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use braincell::{Ast, Lexer, Interpreter};

fn main() {
  let mut lexer = Lexer::new();
  let tokens = lexer.tokenize("+ ++ +++ - -- --- > >> >>> < << <<< , ,, ,,, . .. ... [ [[ [[[ ] ]] ]]]".to_string());
  let mut ast = Ast::new();
  let nodes = ast.parse(tokens);
  println!("{lexer}\n");
  println!("{ast}\n");
  let tokens = lexer.tokenize(">>>".to_string());
  let nodes = ast.parse(tokens);
  let mut interpreter = Interpreter::new(30000);
  let _ = interpreter.run(nodes);
  println!("{lexer}\n");
  println!("{ast}\n");
  println!("{interpreter}");
}
