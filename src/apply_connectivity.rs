use crate::{
    Cell, Grid,
    ext::{ArrayExt, CellExt},
    solve_path::{SolvePath, SolveStep},
};
use puzzle_grid::array::ArrayVec;

struct Connectivity<'g> {
    grid: &'g mut Grid,
    color: Cell,
    changed: Vec<(usize, usize, Cell)>,
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
            changed: Vec::new(),
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
        for x1 in self.grid.iter_adj(x0) {
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
                        self.changed.push((x0.0, x0.1, self.color));
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

    let changed = conn.changed;
    let made_progress = changed.len() > 0;
    if let Some(path) = path {
        if made_progress {
            path.push(SolveStep::ApplyConnectivity(changed));
        }
    }
    made_progress
}

struct Isolation<'g> {
    grid: &'g mut Grid,
    changed: Vec<(usize, usize, Cell)>,
    visited: ArrayVec<bool>,
    reaches: ArrayVec<u8>,
    roots: ArrayVec<(usize, usize)>,
}

impl<'g> Isolation<'g> {
    fn new(grid: &'g mut Grid) -> Isolation<'g> {
        let visited = grid.dup_default();
        let reaches = grid.dup_default();
        let roots = grid.dup_default();
        Isolation {
            grid,
            changed: Vec::new(),
            visited,
            reaches,
            roots,
        }
    }

    fn dfs(&mut self, x0: (usize, usize), root: (usize, usize)) {
        self.visited[x0] = true;
        self.roots[x0] = root;
        for x1 in self.grid.iter_adj(x0) {
            match self.grid[x1] {
                Cell::Empty => {
                    if !self.visited[x1] {
                        self.dfs(x1, root);
                    }
                }
                Cell::Black => self.reaches[root] |= 0x01,
                Cell::White => self.reaches[root] |= 0x02,
            }
        }
    }

    fn dfs_all(&mut self) {
        for r in 0..self.grid.rows() {
            for c in 0..self.grid.cols() {
                let x = (r, c);
                if !self.visited[x] && self.grid[x] == Cell::Empty {
                    self.dfs(x, x);
                    self.apply(x);
                }
            }
        }
    }

    fn apply(&mut self, root: (usize, usize)) {
        let color = match self.reaches[root] {
            0x01 => Cell::Black,
            0x02 => Cell::White,
            _ => return,
        };
        for r in 0..self.grid.rows() {
            for c in 0..self.grid.cols() {
                let x = (r, c);
                if self.grid[x] == Cell::Empty && self.roots[x] == root {
                    self.grid[x] = color;
                    self.changed.push((r, c, color));
                }
            }
        }
    }
}

fn apply_isolation(grid: &mut Grid, path: Option<&mut SolvePath>) -> bool {
    let mut iso = Isolation::new(grid);
    iso.dfs_all();

    let changed = iso.changed;
    let made_progress = changed.len() > 0;
    if let Some(path) = path {
        if made_progress {
            path.push(SolveStep::ApplyConnectivity(changed));
        }
    }
    made_progress
}

pub fn apply(grid: &mut Grid, mut path: Option<&mut SolvePath>) -> bool {
    let did_black = apply_connectivity_for(grid, path.as_deref_mut(), Cell::Black);
    let did_white = apply_connectivity_for(grid, path.as_deref_mut(), Cell::White);
    let did_empty = apply_isolation(grid, path.as_deref_mut());
    did_black || did_white || did_empty
}
