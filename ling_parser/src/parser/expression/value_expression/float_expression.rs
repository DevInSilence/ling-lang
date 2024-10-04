use crate::parser::expression::Expression;

use super::ValueExpression;

pub struct FloatExpession {
  pub val: f32,
}

impl FloatExpession {
  pub fn new(val: f32) -> Expression {
    Expression::ValueExpression(ValueExpression::FloatExpression(Self { val }))
  }
}

