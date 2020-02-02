#[derive(Clone, Copy)]
pub struct Number {
  pub sum: u32,
}

impl Number {
  pub fn new() -> Self {
    Number { sum: 0 }
  }

  pub fn add(&mut self, cell: &Number) {
    self.sum += cell.sum;
  }
}
