use super::expression;

#[derive(Debug)]
pub enum Statement {
  Expression(expression::Expression),
}