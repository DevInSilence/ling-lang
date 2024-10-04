use super::expression;
pub mod variable_declaration;

#[derive(Debug)]
pub enum Statement {
  Expression(expression::Expression),
  VariableDeclaration(variable_declaration::VariableDeclaration),
}

#[cfg(test)]
mod tests {
  use std::panic;

  use crate::parser::expression::{self, value_expression::float_expression::FloatExpession};

  use super::{variable_declaration, Statement};

  #[test]
  fn print_test() {
    let stmt = Statement::VariableDeclaration(variable_declaration::VariableDeclaration::new(
      "x".to_string(),
      FloatExpession::new(3.14, Default::default()),
      Default::default(),
    ));

    println!("{:#?}", stmt);
  }

  #[test]
  fn invalid_conversion_test() {
    let result = panic::catch_unwind(|| {
      let stmt = Statement::VariableDeclaration(variable_declaration::VariableDeclaration::new(
        "x".to_string(),
        FloatExpession::new(3.14, Default::default()),
        Default::default(),
      ));

      let expr: expression::Expression = stmt.into();
      assert!(matches!(
        expr,
        expression::Expression::VariableExpression(_)
      ));
    });

    assert!(result.is_err());
  }
}
