mod board;

use board::*;

fn main() -> anyhow::Result<()> {

    let mut board = Board::new();

    board.place_word_horizontal("hello", 0x4, 0x6)?;

    board.place_word_vertical("world", 0x3, 0x3)?;

    board.display();

    Ok(())
}
