extern crate tictactoe;

use tictactoe::ttt;

fn main() {
  let mut board = ttt::Board::new(3);
  board.set(0, 1, ttt::Mark::X);
  println!("{:?}", board);
}
