use crate::{lexer::Location, parser::expression::Expression};

use super::ValueExpression;

#[derive(Debug)]
pub struct FloatExpession {
  pub location: Location,
  pub val: f32,
}

impl FloatExpession {
  pub fn new(val: f32, location: Location) -> Expression {
    Expression::ValueExpression(ValueExpression::FloatExpression(Self { val, location }))
  }
}
