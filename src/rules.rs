use puzzle_grid::array::{ArrayIterator, ArrayVec};

use crate::{
    Cell, Grid,
    ext::{ArrayExt, CellExt},
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RuleCheck {
    Unsolved,
    Solved,
    Contradiction,
}

struct RuleChecker<'g> {
    grid: &'g Grid,
    visited: ArrayVec<bool>,
    visited_black: bool,
    visited_white: bool,
}

impl<'g> RuleChecker<'g> {
    fn new(grid: &'g Grid) -> RuleChecker<'g> {
        let visited = grid.dup_default();
        RuleChecker {
            grid,
            visited,
            visited_black: false,
            visited_white: false,
        }
    }

    fn dfs(&mut self, x0: (usize, usize), color: Cell) {
        self.visited[x0] = true;
        for x1 in self.grid.adj(x0) {
            if !self.visited[x1] && self.grid[x1] == color {
                self.dfs(x1, color);
            }
        }
    }

    fn dfs_all(&mut self) -> RuleCheck {
        for r in 0..self.grid.rows() {
            for c in 0..self.grid.cols() {
                let x = (r, c);
                if self.visited[x] {
                    continue;
                }
                let color = self.grid[x];
                let visited_color = match color {
                    Cell::Black => self.visited_black,
                    Cell::White => self.visited_white,
                    Cell::Empty => return RuleCheck::Unsolved,
                };
                if !visited_color {
                    match color {
                        Cell::Black => self.visited_black = true,
                        Cell::White => self.visited_white = true,
                        _ => panic!(),
                    }
                    self.dfs(x, color);
                }
            }
        }
        let unvisited = self
            .visited
            .iter()
            .with_positions()
            .skip_while(|(_, _, x)| **x)
            .next();
        match unvisited {
            Some((r, c, _)) => {
                log::debug!("found dfs contradiction at {r},{c}");
                RuleCheck::Contradiction
            }
            None => RuleCheck::Solved,
        }
    }

    fn check_2x2(&self) -> Option<RuleCheck> {
        for (r, c, view) in self.grid.iter_views(2, 2).with_positions() {
            let c0 = view[(0, 0)];
            let c1 = view[(0, 1)];
            let c2 = view[(1, 0)];
            let c3 = view[(1, 1)];
            if c0.empty() || c1.empty() {
                continue;
            }
            // matches both solid 2x2 and checkerboard
            if c0 == c3 && c1 == c2 {
                log::debug!("found 2x2 contradiction at {r},{c}");
                return Some(RuleCheck::Contradiction);
            }
        }
        None
    }

    fn check(mut self) -> RuleCheck {
        self.check_2x2().unwrap_or_else(|| self.dfs_all())
    }
}

pub fn check(grid: &Grid) -> RuleCheck {
    RuleChecker::new(grid).check()
}
