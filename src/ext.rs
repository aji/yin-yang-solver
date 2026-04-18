use puzzle_grid::array::{Array, ArrayBuffer, ArrayVec};

use crate::{
    Cell,
    iter::{IterAdjacent, IterBorderPositions},
};

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
    fn iter_adj(&self, rc: (usize, usize)) -> IterAdjacent;
    fn iter_border_positions(&self) -> IterBorderPositions;
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

    fn iter_adj(&self, (r, c): (usize, usize)) -> IterAdjacent {
        IterAdjacent::new(self.rows(), self.cols(), r, c)
    }

    fn iter_border_positions(&self) -> IterBorderPositions {
        IterBorderPositions::new(self.rows(), self.cols())
    }
}
