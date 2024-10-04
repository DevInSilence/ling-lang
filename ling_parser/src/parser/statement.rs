use super::expression;

pub enum Statement {
  Expression(expression::Expression),
}