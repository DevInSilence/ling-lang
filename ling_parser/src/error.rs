use std::rc::Rc;

use crate::{context::Context, lexer::Location};
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

#[derive(Debug)]
pub struct LangError<'a> {
  pub message: String,
  location: Location,
  context: Rc<Context<'a>>,
}

impl<'a> LangError<'a> {
  pub fn new(message: String, location: Location, context: Rc<Context<'a>>) -> Self {
    Self {
      message,
      location,
      context,
    }
  }
}

impl<'a> std::fmt::Display for LangError<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}:{}:{}: {}error{}: {}",
      self.context.filename, self.location.line, self.location.col, RED, RESET, self.message
    )
  }
}
