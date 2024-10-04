use crate::{lexer::Location, parser::expression::Expression};

use super::ValueExpression;

#[derive(Debug)]
pub struct IntegerExpession {
  pub location: Location,
  pub val: i32,
}

impl IntegerExpession {
  pub fn new(val: i32, location: Location) -> Expression {
    Expression::ValueExpression(ValueExpression::IntegerExpression(Self { val, location }))
  }
}
