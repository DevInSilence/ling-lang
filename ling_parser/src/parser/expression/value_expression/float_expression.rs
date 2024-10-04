use crate::parser::expression::Expression;

use super::ValueExpression;

#[derive(Debug)]
pub struct FloatExpession {
  pub val: f32,
}

impl FloatExpession {
  pub fn new(val: f32) -> Expression {
    Expression::ValueExpression(ValueExpression::FloatExpression(Self { val }))
  }
}
