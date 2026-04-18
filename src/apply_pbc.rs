use crate::{
    Cell, Grid, apply_2x2, apply_border, apply_connectivity,
    dump::DumpGrid,
    ext::CellExt,
    rules::{self, RuleCheck},
    solve_path::{SolvePath, SolveStep},
};

fn apply_pbc_deductions_once(grid: &mut Grid, mut path: Option<&mut SolvePath>) -> bool {
    if apply_2x2::apply(grid, path.as_deref_mut()) {
        return true;
    }
    if apply_border::apply(grid, path.as_deref_mut()) {
        return true;
    }
    if apply_connectivity::apply(grid, path.as_deref_mut()) {
        return true;
    }
    false
}

fn apply_pbc_deductions(grid: &mut Grid, mut path: Option<&mut SolvePath>, depth: usize) {
    for _ in 0..depth {
        if !apply_pbc_deductions_once(grid, path.as_deref_mut()) {
            break;
        }
    }
}

pub fn apply(grid: &mut Grid, path: Option<&mut SolvePath>, depth: usize) -> bool {
    log::debug!("attempting proof by contradiction");
    for i in 0..grid.len() {
        let r = i / grid.cols();
        let c = i % grid.cols();
        let x = (r, c);

        if !grid[x].empty() {
            continue;
        }

        let mut hyppath = Vec::new();
        let mut hyp = grid.clone();
        hyp[x] = Cell::Black;
        apply_pbc_deductions(&mut hyp, Some(&mut hyppath), depth);
        if rules::check(&hyp) == RuleCheck::Contradiction {
            log::debug!("pbc found contradiction in\n{}", DumpGrid("?....", &hyp));
            log::debug!("by pbc cell at {x:?} is white",);
            if let Some(path) = path {
                path.push(SolveStep::ApplyPbc(r, c, Cell::White, hyppath));
            }
            grid[i] = Cell::White;
            return true;
        }

        let mut hyppath = Vec::new();
        let mut hyp = grid.clone();
        hyp[x] = Cell::White;
        apply_pbc_deductions(&mut hyp, Some(&mut hyppath), depth);
        if rules::check(&hyp) == RuleCheck::Contradiction {
            log::debug!("pbc found contradiction in {}", DumpGrid("?....", &hyp));
            log::debug!("by pbc cell at {x:?} is black",);
            if let Some(path) = path {
                path.push(SolveStep::ApplyPbc(r, c, Cell::White, hyppath));
            }
            grid[x] = Cell::Black;
            return true;
        }
    }
    false
}
