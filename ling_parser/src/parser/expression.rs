use super::statement::Statement;

pub mod value_expression;
pub mod variable_expression;

#[derive(Debug)]
pub enum Expression {
  ValueExpression(value_expression::ValueExpression),
  VariableExpression(variable_expression::VariableExpression),
}

impl From<Statement> for Expression {
  fn from(value: Statement) -> Self {
    match value {
      Statement::Expression(expr) => expr,
      _ => panic!("Cannot convert {:?} to Expression", value),
    }
  }
}
