pub mod ttt {
  #[derive(Debug, Clone, Copy)]
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

    pub fn winner(&self) -> Option<Mark> {
      for sr in 0..self.size {
        for sc in 0..self.size {
          let target = self.get(sr, sc);
          match target {
            Mark::Empty => (),
            Mark::X | Mark::O => {
              for dr in -1..2 {
                for dc in -1..2 {
                  for dist in 0..self.size {
                    let rr = sr + dist*dr;
                    let cc = sc + dist*dc;
                    if self.get(rr, cc) != target {
                      // @TODO
                      // ugh I don't really like this func sig anyway...
                      // seems like this should be split into a slice iterator
                      // and a function that tells if a slice is a winner
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
