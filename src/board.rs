extern crate rand;

use crate::number::Number;
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
  Empty,
  Fill,
}

pub enum Direction {
  UP,
  DOWN,
  LEFT,
  RIGHT,
}

#[derive(Clone, Copy)]
struct Cell {
  pub state: CellState,
  pub number: Number,
}

pub struct Point {
  pub i: usize,
  pub j: usize,
}

impl Cell {
  fn activate(&mut self) {
    self.state = CellState::Fill;
    self.number.sum = 2;
  }

  fn activate_with_value(&mut self, sum: u32) {
    self.state = CellState::Fill;
    self.number.sum = sum;
  }

  fn deactivate(&mut self) {
    self.state = CellState::Empty;
    self.number.sum = 0;
  }
}

#[derive(Clone, Copy)]
pub struct Board {
  cells: [[Cell; 4]; 4],
}

impl Board {
  pub fn new() -> Self {
    let cells: [[Cell; 4]; 4] = [[Cell {
      state: CellState::Empty,
      number: Number::new(),
    }; 4]; 4];
    let mut board = Board { cells: cells };
    board.activate_cell(3, 2);
    board.activate_cell(3, 3);
    board
  }

  pub fn get_cell_state(&self, i: usize, j: usize) -> CellState {
    assert!(i <= 3);
    assert!(j <= 3);
    self.cells[i][j].state
  }

  pub fn get_cell_number_as_mut_ref(&mut self, i: usize, j: usize) -> &mut Number {
    assert!(i <= 3);
    assert!(j <= 3);
    &mut self.cells[i][j].number
  }

  pub fn get_cell_number(&self, i: usize, j: usize) -> Number {
    assert!(i <= 3);
    assert!(j <= 3);
    self.cells[i][j].number
  }

  pub fn update_cell(&mut self, point: Point, num: &Number) {
    assert!(point.i <= 3);
    assert!(point.j <= 3);
    if self.get_cell_state(point.i, point.j) == CellState::Fill {
      self.get_cell_number_as_mut_ref(point.i, point.j).add(num);
    }
  }

  pub fn activate_cell(&mut self, i: usize, j: usize) {
    assert!(i <= 3);
    assert!(j <= 3);
    if self.get_cell_state(i, j) == CellState::Empty {
      self.cells[i][j].activate();
    }
  }

  pub fn activate_cell_with_value(&mut self, i: usize, j: usize, sum: u32) {
    assert!(i <= 3);
    assert!(j <= 3);
    if self.get_cell_state(i, j) == CellState::Empty {
      self.cells[i][j].activate_with_value(sum);
    }
  }

  pub fn deactivate_cell(&mut self, i: usize, j: usize) {
    assert!(i <= 3);
    assert!(j <= 3);
    if self.get_cell_state(i, j) == CellState::Fill {
      self.cells[i][j].deactivate();
    }
  }

  pub fn pop_cell_number(&mut self) {
    loop {
      let rand_x = rand::thread_rng().gen_range(0, 4);
      let rand_y = rand::thread_rng().gen_range(0, 4);

      if self.get_cell_state(rand_x, rand_y) == CellState::Empty {
        let helper_rand = rand::thread_rng().gen_range(0, 10);
        let init_sum = if helper_rand > 8 { 4 } else { 2 };
        self.activate_cell_with_value(rand_x, rand_y, init_sum);
        break;
      }
    }
  }

  pub fn move_cell(&mut self, point: Point, direction: Direction) {
    assert!(point.i <= 3);
    assert!(point.j <= 3);
    match direction {
      Direction::UP => {
        if point.i == 0 {
          return;
        }
        match self.get_cell_state(point.i - 1, point.j) {
          CellState::Fill => {
            if self.get_cell_number(point.i, point.j).sum
              == self.get_cell_number(point.i - 1, point.j).sum
            {
              let x = self.get_cell_number(point.i, point.j);
              self.deactivate_cell(point.i, point.j);
              self.update_cell(
                Point {
                  i: point.i - 1,
                  j: point.j,
                },
                &x,
              );
            }
          }
          CellState::Empty => {
            let x = self.get_cell_number(point.i, point.j);
            self.deactivate_cell(point.i, point.j);
            self.activate_cell_with_value(point.i - 1, point.j, x.sum);
          }
        }
        self.move_cell(
          Point {
            i: point.i - 1,
            j: point.j,
          },
          Direction::UP,
        )
      }
      Direction::DOWN => {
        if point.i == 3 {
          return;
        }
        match self.get_cell_state(point.i + 1, point.j) {
          CellState::Fill => {
            if self.get_cell_number(point.i, point.j).sum
              == self.get_cell_number(point.i + 1, point.j).sum
            {
              let x = self.get_cell_number(point.i, point.j);
              self.deactivate_cell(point.i, point.j);
              self.update_cell(
                Point {
                  i: point.i + 1,
                  j: point.j,
                },
                &x,
              );
            }
          }
          CellState::Empty => {
            let x = self.get_cell_number(point.i, point.j);
            self.deactivate_cell(point.i, point.j);
            self.activate_cell_with_value(point.i + 1, point.j, x.sum);
          }
        }
        self.move_cell(
          Point {
            i: point.i + 1,
            j: point.j,
          },
          Direction::DOWN,
        )
      }
      Direction::LEFT => {
        if point.j == 0 {
          return;
        }
        match self.get_cell_state(point.i, point.j - 1) {
          CellState::Fill => {
            if self.get_cell_number(point.i, point.j).sum
              == self.get_cell_number(point.i, point.j - 1).sum
            {
              let x = self.get_cell_number(point.i, point.j);
              self.deactivate_cell(point.i, point.j);
              self.update_cell(
                Point {
                  i: point.i,
                  j: point.j - 1,
                },
                &x,
              );
            }
          }
          CellState::Empty => {
            let x = self.get_cell_number(point.i, point.j);
            self.deactivate_cell(point.i, point.j);
            self.activate_cell_with_value(point.i, point.j - 1, x.sum);
          }
        }
        self.move_cell(
          Point {
            i: point.i,
            j: point.j - 1,
          },
          Direction::LEFT,
        )
      }
      Direction::RIGHT => {
        if point.j == 3 {
          return;
        }
        match self.get_cell_state(point.i, point.j + 1) {
          CellState::Fill => {
            if self.get_cell_number(point.i, point.j).sum
              == self.get_cell_number(point.i, point.j + 1).sum
            {
              let x = self.get_cell_number(point.i, point.j);
              self.deactivate_cell(point.i, point.j);
              self.update_cell(
                Point {
                  i: point.i,
                  j: point.j + 1,
                },
                &x,
              );
            }
          }
          CellState::Empty => {
            let x = self.get_cell_number(point.i, point.j);
            self.deactivate_cell(point.i, point.j);
            self.activate_cell_with_value(point.i, point.j + 1, x.sum);
          }
        }
        self.move_cell(
          Point {
            i: point.i,
            j: point.j + 1,
          },
          Direction::RIGHT,
        )
      }
    }
  }
  // Move all of number cells to specified direction
  pub fn move_all_cell(&mut self, direction: Direction) {
    match direction {
      Direction::UP => {
        for _i in 0..4 {
          for _j in 0..4 {
            if self.get_cell_state(_j, _i) == CellState::Fill {
              self.move_cell(Point { i: _j, j: _i }, Direction::UP);
            }
          }
        }
      }
      Direction::DOWN => {
        for _i in 0..4 {
          for _j in (0..4).rev() {
            if self.get_cell_state(_j, _i) == CellState::Fill {
              self.move_cell(Point { i: _j, j: _i }, Direction::DOWN);
            }
          }
        }
      }
      Direction::LEFT => {
        for _i in 0..4 {
          for _j in 0..4 {
            if self.get_cell_state(_j, _i) == CellState::Fill {
              self.move_cell(Point { i: _j, j: _i }, Direction::LEFT);
            }
          }
        }
      }
      Direction::RIGHT => {
        for _i in 0..4 {
          for _j in (0..4).rev() {
            if self.get_cell_state(_j, _i) == CellState::Fill {
              self.move_cell(Point { i: _j, j: _i }, Direction::RIGHT);
            }
          }
        }
      }
    }
  }

  pub fn print(b: &Board) {
    for i in 0..4 {
      for j in 0..4 {
        let x = b.get_cell_number(i, j);
        if x.sum != 0 {
          print!(" {} ", x.sum);
        } else {
          print!(" {} ", x.sum);
        }
        if j == 3 {
          println!();
        }
      }
    }
  }
}
