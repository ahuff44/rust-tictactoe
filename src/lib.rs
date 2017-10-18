#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mark {
  X,
  O,
  Empty,
}

#[derive(Debug)]
pub struct Board {
  size: usize,
  vals: Vec<Vec<Mark>>,
}

struct SliceIter<'a> {
  board: &'a Board,
  sr: usize,
  sc: usize,
  dr: i32,
  dc: i32,
}

impl<'b> SliceIter<'b> {
  fn new<'a>(board: &'a Board) -> SliceIter<'a> {
    SliceIter{
      board: &board, // @TODO: why is the `&` optional here?
      sr: 0, sc: 0,
      dr: 0, dc: 1,
    }
  }

  fn advance(&mut self) -> bool {
    // returns: whether all possible (sr, sc, dr, dc) combos have been tried yet
    //    (i.e. should we keep going)
    match (self.dr, self.dc) {
      (0, 1) => {self.dr = 1},
      (1, 1) => {self.dc = 0},
      (1, 0) => {self.dc = -1},
      (1, -1) => {
        self.dr = 0;
        self.dc = 1;

        self.sc += 1;
        if self.sc >= self.board.size {
          self.sc = 0;
          self.sr += 1;
          if self.sr >= self.board.size {
            return false;
          }
        }
      },
      (_, _) => panic!("invalid (dr, dc)"),
    }
    true
  }

  fn attempt_slice(&mut self) -> Option<Vec<Mark>> {
    let mut v = Vec::with_capacity(self.board.size);
    for dist in 0..self.board.size {
      let rr: i32 = (self.sr as i32) + (dist as i32)*self.dr;
      let cc: i32 = (self.sc as i32) + (dist as i32)*self.dc;
      if let Some(val) = self.board.get(rr, cc) {
        v.push(val);
      } else {
        return None;
      }
    }
    Some(v)
  }
}

impl<'a> Iterator for SliceIter<'a> {
  type Item = Vec<Mark>;

  fn next(&mut self) -> Option<Vec<Mark>> {
    loop {
      let has_more = self.advance();
      if !has_more { return None; }
      if let Some(slice) = self.attempt_slice() {
        return Some(slice);
      }
    }
  }
}

impl Board {
  pub fn new(size: usize) -> Board {
    let mut vals = Vec::with_capacity(size);
    for _ in 0..size {
      let mut row = Vec::with_capacity(size);
      for _ in 0..size {
        row.push(Mark::Empty);
      }
      vals.push(row);
    }
    Board{size, vals}
  }

  pub fn get(&self, rr: i32, cc: i32) -> Option<Mark> {
    if rr < 0 || cc < 0 {
      return None
    }
    let rr = rr as usize;
    let cc = cc as usize;
    self.vals
      .get(rr)
      .and_then(|row| row.get(cc))
      .map(|&val| val)
  }

  pub fn set(&mut self, rr: usize, cc: usize, val: Mark) -> bool {
    // @TODO: s/bool/Result/ in type sig?
    match self.vals
        .get_mut(rr)
        .and_then(|row| row.get_mut(cc))
        .map(|x| *x = val) {
      Some(_) => true,
      None => false,
    }
  }

  fn slices(&self) -> SliceIter {
    SliceIter::new(&self)
  }

  pub fn winner(&self) -> Option<Mark> {
    for slice in self.slices() {
      // let x:Option<Mark>=slice[0]; @TODO something's up...
      match slice.get(0) {
        Some(&target) if target != Mark::Empty => {
          for &elem in slice.iter() {
            if elem != target {
              continue;
            }
          }
          return Some(target);
        },
        Some(_) => { continue; },
        None => { continue; },
      }
    }
    None
  }
}





#[cfg(test)]
mod tests {
  use super::{Board, Mark};

  #[test]
  fn board_set() {
    let mut board = Board::new(3);

    board.set(0, 1, Mark::O);
    board.set(0, 1, Mark::Empty);
    board.set(0, 2, Mark::X);

    assert_eq!(Some(Mark::Empty), board.get(0, 0));
    assert_eq!(Some(Mark::Empty), board.get(0, 1));
    assert_eq!(Some(Mark::X), board.get(0, 2));
    assert_eq!(Some(Mark::Empty), board.get(1, 0));
    assert_eq!(Some(Mark::Empty), board.get(1, 1));
    assert_eq!(Some(Mark::Empty), board.get(1, 2));
    assert_eq!(Some(Mark::Empty), board.get(2, 0));
    assert_eq!(Some(Mark::Empty), board.get(2, 1));
    assert_eq!(Some(Mark::Empty), board.get(2, 2));
  }

  #[test]
  fn board_winner() {
    let mut board = Board::new(3);

    board.set(2, 0, Mark::O);
    board.set(1, 1, Mark::O);
    assert_eq!(None, board.winner());
    board.set(0, 2, Mark::O);
    assert_eq!(Some(O), board.winner());
  }
}
