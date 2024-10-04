use super::{kind::TokenKind, location::Location};

#[derive(Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub lexeme: Option<String>,
  pub loc: Location,
}

impl Token {
  pub fn new(kind: TokenKind, lexeme: Option<String>, loc: Location) -> Self {
    Self { kind, lexeme, loc }
  }
}
