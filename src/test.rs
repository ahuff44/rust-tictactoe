extern crate tictactoe;

use tictactoe::ttt::{Board, Mark};

#[cfg(test)]
mod tests {
  #[test]
  fn board_set() {
    let board = Board::new(3);
    board.set(0, 1, Mark::X);
    assert_eq!(Mark::Empty, board.get(0, 0));
    assert_eq!(Mark::X, board.get(0, 1));
    assert_eq!(Mark::Empty, board.get(0, 2));
    assert_eq!(Mark::Empty, board.get(1, 0));
    assert_eq!(Mark::Empty, board.get(1, 1));
    assert_eq!(Mark::Empty, board.get(1, 2));
    assert_eq!(Mark::Empty, board.get(2, 0));
    assert_eq!(Mark::Empty, board.get(2, 1));
    assert_eq!(Mark::Empty, board.get(2, 2));
  }
}
