use puzzle_grid::array::{Array, ArrayBuffer, ArrayVec};

use crate::Cell;

pub trait CellExt {
    fn inv(self) -> Cell;
    fn empty(self) -> bool;
}

impl CellExt for Cell {
    fn inv(self) -> Cell {
        match self {
            Cell::Empty => Cell::Empty,
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
        }
    }

    fn empty(self) -> bool {
        self == Cell::Empty
    }
}

pub trait ArrayExt {
    fn dup_default<T: Default>(&self) -> ArrayVec<T>;
    fn dup_filled<T: Clone>(&self, value: T) -> ArrayVec<T>;
    fn adj(&self, rc: (usize, usize)) -> Adjacent;
}

impl<B: ArrayBuffer> ArrayExt for Array<B> {
    fn dup_default<T: Default>(&self) -> ArrayVec<T> {
        (0..self.len())
            .map(|_| T::default())
            .collect::<ArrayVec<T>>()
            .reshape(self.rows(), self.cols())
            .unwrap()
    }

    fn dup_filled<T: Clone>(&self, value: T) -> ArrayVec<T> {
        (0..self.len())
            .map(move |_| value.clone())
            .collect::<ArrayVec<T>>()
            .reshape(self.rows(), self.cols())
            .unwrap()
    }

    fn adj(&self, (r, c): (usize, usize)) -> Adjacent {
        Adjacent {
            rows: self.rows(),
            cols: self.cols(),
            row: r,
            col: c,
            next: 0,
        }
    }
}

pub struct Adjacent {
    rows: usize,
    cols: usize,
    row: usize,
    col: usize,
    next: usize,
}

impl Iterator for Adjacent {
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
