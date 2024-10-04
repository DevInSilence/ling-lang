use super::Expression;

#[derive(Debug)]
pub struct VariableExpression {
  pub name: String,
}

impl VariableExpression {
  pub fn new(name: String) -> Expression {
    Expression::VariableExpression(VariableExpression { name })
  }
}

impl From<Expression> for VariableExpression {
  fn from(value: Expression) -> Self {
    match value {
      Expression::VariableExpression(v) => v,
      _ => panic!("Cannot convert {:?} to VariableExpression", value)
    }
  }
}
