mod board;

use board::Board;

fn main() {
    let board = Board::new();
    board.display();
}
