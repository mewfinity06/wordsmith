use super::*;

#[test]
fn test_conflict_pass() {
    let mut board = Board::new();

    assert!(board.place_word("hello", 0x5, 0x7, Direction::Vertical).is_ok());
    assert!(board.place_word("world", 0x7, 0x4, Direction::Horizontal).is_ok());
}

#[test]
fn test_conflict_fail() {
    let mut board = Board::new();

    assert!(board.place_word("hello", 0x5, 0x7, Direction::Vertical).is_ok());
    assert!(board.place_word("world", 0x7, 0x5, Direction::Horizontal).is_err());
}
