extern crate tzfe;

use tzfe::board::*;

fn main() {
    {
        println!("==================================================");
        let mut b = Board::new();
        Board::print(&b);
        println!();
        b.move_cell(Point { i: 3, j: 2 }, Direction::RIGHT);
        Board::print(&b);
        println!();
        b.move_all_cell(Direction::UP);
        Board::print(&b);
        println!();
        b.pop_cell_number();
        Board::print(&b);
        println!();
    }

    {
        println!("==================================================");
        let mut b = Board::new();
        Board::print(&b);
        println!();
        b.move_cell(Point { i: 3, j: 3 }, Direction::LEFT);
        b.pop_cell_number();
        Board::print(&b);
        println!();
    }

    {
        println!("==================================================");
        let mut b = Board::new();
        Board::print(&b);
        println!();
        b.move_all_cell(Direction::LEFT);
        Board::print(&b);
        println!();
        b.pop_cell_number();
        Board::print(&b);
        println!();
        b.move_all_cell(Direction::UP);
        Board::print(&b);
        println!();
        b.pop_cell_number();
        Board::print(&b);
        println!();
        b.move_all_cell(Direction::LEFT);
        Board::print(&b);
        println!();
    }
}
