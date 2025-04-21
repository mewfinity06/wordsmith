mod board;

use board::*;

fn main() -> Result<(), String> {
    let mut board = Board::new();

    board.place_word("hello", Direction::Horizontal, 0x7, 0x7)?;

    board.display();

    Ok(())
}
