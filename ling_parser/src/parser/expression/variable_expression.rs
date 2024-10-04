use crate::lexer::Location;

use super::Expression;

#[derive(Debug)]
pub struct VariableExpression {
  pub location: Location,
  pub name: String,
}

impl VariableExpression {
  pub fn new(name: String, location: Location) -> Expression {
    Expression::VariableExpression(VariableExpression { name, location })
  }
}

impl From<Expression> for VariableExpression {
  fn from(value: Expression) -> Self {
    match value {
      Expression::VariableExpression(v) => v,
      _ => panic!("Cannot convert {:?} to VariableExpression", value),
    }
  }
}
