use super::Expression;

pub mod float_expression;
pub mod integer_expression;
pub enum ValueExpression {
  IntegerExpression(integer_expression::IntegerExpession),
  FloatExpression(float_expression::FloatExpession),
}

impl From<Expression> for ValueExpression {
  fn from(expression: Expression) -> Self {
    match expression {
      Expression::ValueExpression(value_expression) => value_expression,
      _ => panic!("Cannot convert non-ValueExpression to ValueExpression"),
    }
  }
}
