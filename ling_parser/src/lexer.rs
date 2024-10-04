pub mod kind;
pub mod location;
pub mod token;

use std::rc::Rc;

use crate::{context::Context, error::LangError};

pub use kind::TokenKind;
pub use location::Location;
pub use token::Token;

pub struct Lexer;

impl Lexer {
  pub fn tokenize(ctx: Rc<Context>) -> Result<Vec<Token>, Vec<LangError>> {
    let token_kinds = TokenKind::combine();
    let content_length = ctx.source_len;

    let mut errors = vec![];
    let mut tokens = Vec::new();
    let mut index = 0;
    let mut loc = Location::zero();

    let mut buf = String::new();
    let mut err = false;

    'outer: while index < content_length {
      let content = &ctx.source[index..];

      for (kind, regex) in token_kinds.iter() {
        if let Some(captures) = regex.captures(content) {
          let lexeme = captures.get(0).unwrap().as_str();

          if err {
            let error = LangError::new(
              format!("Unexpected character: {}", buf),
              Location::new(index - buf.len(), index, loc.col - buf.len(), loc.line),
              Rc::clone(&ctx),
            );
            errors.push(error);
            err = false;
            buf.clear();
          }

          let start_idx = index;
          let stop_idx = start_idx + lexeme.len();
          let line = loc.line;
          let col = loc.col;

          match kind {
            TokenKind::Whitespace => {
              loc.col += lexeme.len();
              index += lexeme.len();
              continue 'outer;
            }
            TokenKind::Newline => {
              loc.line += 1;
              loc.col = 0;
              index += lexeme.len();
              continue 'outer;
            }
            _ => {}
          }

          let token = Token::new(
            kind.clone(),
            Some(lexeme.to_string()),
            Location::new(start_idx, stop_idx, col, line),
          );

          tokens.push(token);
          index += lexeme.len();
          loc.col += lexeme.len();
          continue 'outer;
        }
      }

      err = true;
      buf.push(ctx.source[index..].chars().next().unwrap());
      index += 1;
      loc.col += 1;
    }

    if err && !buf.is_empty() {
      let error = LangError::new(
        format!("Unexpected characters found"),
        Location::new(index - buf.len(), index, loc.col - buf.len(), loc.line),
        Rc::clone(&ctx),
      );
      errors.push(error);
    }

    if errors.is_empty() {
      Ok(tokens)
    } else {
      Err(errors)
    }
  }
}
