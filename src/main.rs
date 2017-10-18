extern crate tictactoe;

fn main() {
  let mut board = tictactoe::Board::new(3);
  board.set(0, 1, tictactoe::Mark::X);
  println!("{:?}", board);
}
