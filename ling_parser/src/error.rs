use std::rc::Rc;

use crate::{context::Context, lexer::Location};

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

  fn within_boundary(&self) -> bool {
    let source_len = self.context.source.len();
    self.location.start_idx <= self.location.stop_idx
      && self.location.start_idx < source_len
      && self.location.stop_idx <= source_len
      && self
        .context
        .source
        .is_char_boundary(self.location.start_idx)
      && self.context.source.is_char_boundary(self.location.stop_idx)
  }
}

impl<'a> std::fmt::Display for LangError<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if !self.within_boundary() {
      return write!(f, "{}", self.message);
    }

    let source = self.context.source;
    let span = &source[self.location.start_idx..self.location.stop_idx];
    write!(
      f,
      "Error: {} in {} here:\n{}:{}| {}\n",
      self.message, self.context.filename, self.location.line, self.location.col, span
    )
  }
}
