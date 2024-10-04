use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  Whitespace,
  Newline,
  Decimal,
  Number,
  Iden,
  Assign,
  Operator,
}

impl TokenKind {
  pub fn regex(&self) -> Regex {
    match self {
      TokenKind::Whitespace => Regex::new(r"^[\s\t\r]+").unwrap(),
      TokenKind::Newline => Regex::new(r"^\n").unwrap(),
      TokenKind::Decimal => Regex::new(r"^[+-]?(0|[1-9]\d*)\.\d+").unwrap(),
      TokenKind::Number => Regex::new(r"^[+-]?([1-9]\d*|0)").unwrap(),
      TokenKind::Iden => Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(),
      TokenKind::Assign => Regex::new(r"^=").unwrap(),
      TokenKind::Operator => Regex::new(r"^[+\-*/]").unwrap(),
    }
  }

  pub fn combine() -> Vec<(TokenKind, Regex)> {
    static TOKEN_KINDS: [TokenKind; 7] = [
      TokenKind::Whitespace,
      TokenKind::Newline,
      TokenKind::Decimal,
      TokenKind::Number,
      TokenKind::Iden,
      TokenKind::Assign,
      TokenKind::Operator,
    ];
    let mut token_kinds: Vec<(TokenKind, Regex)> = Vec::new();
    for token_type in TOKEN_KINDS.iter() {
      token_kinds.push((token_type.clone(), token_type.regex()));
    }
    token_kinds
  }
}
