pub mod ttt {
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
    dr: usize,
    dc: usize,
  }

  impl SliceIter {
    fn new<'a>(&board: Board<'a>) -> SliceIter<'a> {
      SliceIter{
        board,
        sr: 0, sc: 0,
        dr: 0, dc: 1,
      }
    }

    fn advance(&mut self) -> bool {
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
      }
      true
    }

    fn attempt_next(&mut self) -> Option<Vec<Mark>> {
      let mut v = Vec::with_capacity(self.board.size);
      for dist in 0..self.board.size {
        let rr = self.sr + dist*self.dr;
        let cc = self.sc + dist*self.dc;
        if let Some(val) = self.board.get(rr, cc) {
          v.push(val.clone()); // @TODO: rm clone()
        } else {
          return None;
        }
      }
      v
    }
  }

  impl<'a> Iterator<Vec<Mark>> for SliceIter<'a> {
    fn next(&mut self) -> Option<Vec<Mark>> {
      loop {
        let has_more = self.advance();
        if !has_more { return None; }
        if let Some(slice) = self.attempt_next {
          return slice;
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

    pub fn get(&self, rr: usize, cc: usize) -> Option<Mark> {
      // @TODO: rm x.clone on the next line; seems like it should be implicit
      self.vals
        .get(rr)
        .and_then(|row| row.get(cc))
        .map(|x| x.clone())
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

    pub fn slices(&'a self) -> SliceIter<'a> {
      SliceIter::new(&self)
    }

    pub fn winner(&self) -> Option<Mark> {
      for slice in self.slices {
        match slice.get(0) {
          Some(target) if target != Mark::Empty => {
            for elem in slice.iter() {
              if elem != target {
                continue;
              }
            }
            return target;
          },
          Some(Mark::Empty) => { continue; },
          None => { continue; },
        }
      }
      None
    }
  }
}
