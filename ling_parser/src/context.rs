#[derive(Debug)]
pub struct Context<'a> {
  pub source: &'a str,
  pub source_len: usize,
  pub filename: &'a str,
}

impl<'a> Context<'a> {
  pub fn new(source: &'a str, filename: &'a str) -> Self {
    Self {
      source,
      source_len: source.len(),
      filename,
    }
  }
}