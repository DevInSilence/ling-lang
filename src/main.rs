use std::rc::Rc;

use ling_parser::{context::Context, lexer::Lexer};

fn main() {
  let ctx = Rc::new(Context::new("variableName = 10 + 12", "main.ling"));

  let ret = Lexer::tokenize(ctx);
  match ret {
    Ok(tokens) => {
      for token in tokens.iter() {
        println!("{:?}", token);
      }
    }
    Err(errors) => {
      for error in errors.iter() {
        println!("{}", error);
      }
    }
  }
}
