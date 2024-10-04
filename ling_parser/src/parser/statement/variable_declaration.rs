use crate::{lexer::Location, parser::expression::Expression};

use super::Statement;

#[derive(Debug)]
pub struct VariableDeclaration {
  pub name: String,
  pub value: Expression,
  pub location: Location,
}

impl VariableDeclaration {
  pub fn new(name: String, value: Expression, location: Location) -> Self {
    Self {
      name,
      value,
      location,
    }
  }
}

impl From<Statement> for VariableDeclaration {
  fn from(value: Statement) -> Self {
    match value {
      Statement::VariableDeclaration(v) => v,
      _ => panic!("Cannot convert {:?} to VariableDeclaration", value),
    }
  }
}
