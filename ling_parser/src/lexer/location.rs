#[derive(Debug, Clone, Default)]
pub struct Location {
  pub start_idx: usize,
  pub stop_idx: usize,
  pub col: usize,
  pub line: usize,
}

impl Location {
  pub fn new(start_idx: usize, stop_idx: usize, col: usize, line: usize) -> Self {
    Self {
      start_idx,
      stop_idx,
      col,
      line,
    }
  }

  pub fn zero() -> Self {
    Self {
      start_idx: 0,
      stop_idx: 0,
      col: 0,
      line: 0,
    }
  }
}
