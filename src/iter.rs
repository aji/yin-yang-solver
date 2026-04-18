pub struct IterBorderPositions {
    rows: usize,
    cols: usize,
    next: usize,
}

impl IterBorderPositions {
    pub fn new(rows: usize, cols: usize) -> IterBorderPositions {
        IterBorderPositions {
            rows,
            cols,
            next: 0,
        }
    }

    fn peek(&mut self) -> Option<(usize, usize)> {
        let mut idx = self.next;
        let nr = self.rows - 1;
        let nc = self.cols - 1;
        // top edge
        match idx < nc {
            true => return Some((0, idx)),
            false => idx -= nc,
        }
        // right edge
        match idx < nr {
            true => return Some((idx, nc)),
            false => idx -= nr,
        }
        // bottom edge
        match idx < nc {
            true => return Some((nr, nc - idx)),
            false => idx -= nc,
        }
        // left edge
        match idx < nr {
            true => return Some((nr - idx, 0)),
            false => return None,
        }
    }
}

impl Iterator for IterBorderPositions {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.peek().inspect(|_| self.next += 1)
    }
}

pub struct IterAdjacent {
    rows: usize,
    cols: usize,
    row: usize,
    col: usize,
    next: usize,
}

impl IterAdjacent {
    pub fn new(rows: usize, cols: usize, row: usize, col: usize) -> IterAdjacent {
        IterAdjacent {
            rows,
            cols,
            row,
            col,
            next: 0,
        }
    }
}

impl Iterator for IterAdjacent {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.row;
        let c = self.col;
        loop {
            let res = match self.next {
                0 => (r + 1 < self.rows).then_some((r + 1, c)),
                1 => (c + 1 < self.cols).then_some((r, c + 1)),
                2 => r.checked_sub(1).map(|rm1| (rm1, c)),
                3 => c.checked_sub(1).map(|cm1| (r, cm1)),
                _ => return None,
            };
            self.next += 1;
            if let Some(res) = res {
                return Some(res);
            }
        }
    }
}
