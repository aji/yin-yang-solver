use puzzle_grid::array::{ArrayVec, ArrayView, ArrayViewMut};

pub use yin_yang_extractor::Cell;

pub type Grid = ArrayVec<Cell>;
pub type GridView<'a> = ArrayView<'a, Cell>;
pub type GridViewMut<'a> = ArrayViewMut<'a, Cell>;

pub(crate) mod apply_2x2;
pub(crate) mod apply_border;
pub(crate) mod apply_connectivity;
pub(crate) mod apply_pbc;
pub(crate) mod dump;
pub(crate) mod ext;
pub(crate) mod iter;
pub(crate) mod rules;
pub(crate) mod solve;

pub use solve::{SolveResult, solve};
