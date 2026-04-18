use puzzle_grid::array::ArrayIterator;

use crate::{
    Grid,
    ext::CellExt,
    solve_path::{SolvePath, SolveStep},
};

pub fn apply(grid: &mut Grid, mut path: Option<&mut SolvePath>) -> bool {
    let mut made_progress = false;
    let mut next = grid.clone();

    let path = &mut path;
    let mut report = |r, c, color, solid| {
        next[(r, c)] = color;
        made_progress = true;
        if let Some(path) = path {
            path.push(if solid {
                SolveStep::ApplySolid2x2(r, c, color)
            } else {
                SolveStep::ApplyCheckerboard2x2(r, c, color)
            })
        }
    };

    for (r, c, view) in grid.iter_views(2, 2).with_positions() {
        let c0 = view[0];
        let c1 = view[1];
        let c2 = view[2];
        let c3 = view[3];

        if c1 == c2 && !c1.empty() {
            if c0.empty() && !c3.empty() {
                report(r, c, c3.inv(), c1 == c3);
            } else if c3.empty() && !c0.empty() {
                report(r + 1, c + 1, c0.inv(), c1 == c0);
            }
        } else if c0 == c3 && !c0.empty() {
            if c1.empty() && !c2.empty() {
                report(r, c + 1, c2.inv(), c0 == c2);
            } else if c2.empty() && !c1.empty() {
                report(r + 1, c, c1.inv(), c0 == c1);
            }
        }
    }

    grid.assign_from(next.iter().copied());

    made_progress
}
