use puzzle_grid::array::ArrayIterator;

use crate::{Grid, ext::CellExt};

pub fn apply(grid: &mut Grid) -> bool {
    let mut made_progress = false;

    let mut next = grid.clone();

    for (r, c, view) in grid.iter_views(2, 2).with_positions() {
        let c0 = view[0];
        let c1 = view[1];
        let c2 = view[2];
        let c3 = view[3];

        if c1 == c2 && !c1.empty() {
            if c0.empty() && !c3.empty() {
                next[(r, c)] = c3.inv();
                made_progress = true;
            } else if c3.empty() && !c0.empty() {
                next[(r + 1, c + 1)] = c0.inv();
                made_progress = true;
            }
        } else if c0 == c3 && !c0.empty() {
            if c1.empty() && !c2.empty() {
                next[(r, c + 1)] = c2.inv();
                made_progress = true;
            } else if c2.empty() && !c1.empty() {
                next[(r + 1, c)] = c1.inv();
                made_progress = true;
            }
        }
    }

    grid.assign_from(next.iter().copied());

    made_progress
}
