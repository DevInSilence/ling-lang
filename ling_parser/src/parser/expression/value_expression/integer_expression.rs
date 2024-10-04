use crate::parser::expression::Expression;

use super::ValueExpression;

pub struct IntegerExpession {
  pub val: i32,
}

impl IntegerExpession {
  pub fn new(val: i32) -> Expression {
    Expression::ValueExpression(ValueExpression::IntegerExpression(Self { val }))
  }
}
