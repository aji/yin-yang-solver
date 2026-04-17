use crate::{Cell, Grid};

pub struct IterBorderMut {
    next: usize,
}

impl IterBorderMut {
    pub fn new() -> IterBorderMut {
        IterBorderMut { next: 0 }
    }

    pub fn peek<'a>(&mut self, grid: &'a mut Grid) -> Option<&'a mut Cell> {
        let mut idx = self.next;
        let nr = grid.rows() - 1;
        let nc = grid.cols() - 1;
        // top edge
        match idx < nc {
            true => return Some(&mut grid[(0, idx)]),
            false => idx -= nc,
        }
        // right edge
        match idx < nr {
            true => return Some(&mut grid[(idx, nc)]),
            false => idx -= nr,
        }
        // bottom edge
        match idx < nc {
            true => return Some(&mut grid[(nr, nc - idx)]),
            false => idx -= nc,
        }
        // left edge
        match idx < nr {
            true => return Some(&mut grid[(nr - idx, 0)]),
            false => return None,
        }
    }

    pub fn next<'a>(&mut self, grid: &'a mut Grid) -> Option<&'a mut Cell> {
        self.peek(grid).inspect(|_| self.next += 1)
    }
}
