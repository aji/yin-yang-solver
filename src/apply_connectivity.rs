use crate::{
    Cell, Grid,
    ext::{ArrayExt, CellExt},
    solve_path::{SolvePath, SolveStep},
};
use puzzle_grid::array::ArrayVec;

struct Connectivity<'g> {
    grid: &'g mut Grid,
    color: Cell,
    made_progress: bool,
    visited: ArrayVec<bool>,
    tin: ArrayVec<isize>,
    tin_next: isize,
    low: ArrayVec<isize>,
    has: ArrayVec<bool>,
}

impl<'g> Connectivity<'g> {
    fn new(grid: &'g mut Grid, color: Cell) -> Connectivity<'g> {
        let visited = grid.dup_default();
        let tin = grid.dup_default();
        let lim = grid.dup_filled(grid.len() as isize);
        let has = grid.dup_default();
        Connectivity {
            grid,
            color,
            made_progress: false,
            visited,
            tin,
            tin_next: 1,
            low: lim,
            has,
        }
    }

    fn dfs(&mut self, x0: (usize, usize), up: Option<(usize, usize)>) {
        self.tin[x0] = self.tin_next;
        self.tin_next += 1;
        self.visited[x0] = true;
        self.has[x0] = self.grid[x0] == self.color;
        for x1 in self.grid.adj(x0) {
            if self.grid[x1] == self.color.inv() {
                // not traversable in dfs
                continue;
            }
            if Some(x1) == up {
                // dfs tree parent
                continue;
            }
            if self.visited[x1] {
                self.low[x0] = self.low[x0].min(self.tin[x1]);
            } else {
                self.dfs(x1, Some(x0));
                self.has[x0] = self.has[x0] || self.has[x1];
                self.low[x0] = self.low[x0].min(self.low[x1]);
                if self.low[x1] >= self.tin[x0] && up.is_some() && self.has[x1] {
                    if self.grid[x0] != self.color {
                        self.grid[x0] = self.color;
                        self.made_progress = true;
                    }
                }
            }
        }
    }

    fn dfs_all(&mut self) {
        for r in 0..self.grid.rows() {
            for c in 0..self.grid.cols() {
                let x = (r, c);
                if !self.visited[x] && self.grid[x] == self.color {
                    self.dfs(x, None);
                }
            }
        }
    }

    #[allow(unused)]
    fn dump(&self) {
        println!("dfs dump {:?}", self.color);
        print!("    ");
        for c in 0..self.grid.cols() {
            print!(" (  {c:2}   ) ");
        }
        println!();
        for r in 0..self.grid.rows() {
            print!("{r:3} ");
            for c in 0..self.grid.cols() {
                print!(
                    "{}({}{:2},{:3}) ",
                    match self.grid[(r, c)] {
                        Cell::Empty => " ",
                        Cell::Black => "B",
                        Cell::White => "W",
                    },
                    if self.has[(r, c)] { '*' } else { ' ' },
                    self.tin[(r, c)],
                    self.low[(r, c)],
                );
            }
            println!();
        }
    }
}

fn apply_connectivity_for(grid: &mut Grid, path: Option<&mut SolvePath>, color: Cell) -> bool {
    let mut conn = Connectivity::new(grid, color);
    conn.dfs_all();
    if conn.made_progress
        && let Some(path) = path
    {
        path.push(SolveStep::ApplyConnectivity);
    }
    conn.made_progress
}

pub fn apply(grid: &mut Grid, mut path: Option<&mut SolvePath>) -> bool {
    let did_black = apply_connectivity_for(grid, path.as_deref_mut(), Cell::Black);
    let did_white = apply_connectivity_for(grid, path.as_deref_mut(), Cell::White);
    did_black || did_white
}
