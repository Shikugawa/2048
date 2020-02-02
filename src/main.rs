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
    }

    {
        println!("==================================================");
        let mut b = Board::new();
        Board::print(&b);
        println!();
        b.move_cell(Point { i: 3, j: 3 }, Direction::LEFT);
        Board::print(&b);
        println!();
    }

    {
        println!("==================================================");
        let mut b = Board::new();
        b.activate_cell(3, 1);
        Board::print(&b);
        println!();
        b.move_cell(Point { i: 3, j: 1 }, Direction::RIGHT);
        Board::print(&b);
        println!();
    }
}
