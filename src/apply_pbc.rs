use crate::{
    Cell, Grid, apply_2x2, apply_border, apply_connectivity,
    dump::DumpGrid,
    ext::CellExt,
    rules::{self, RuleCheck},
};

fn apply_pbc_deductions_once(grid: &mut Grid) -> bool {
    if apply_2x2::apply(grid) {
        return true;
    }
    if apply_border::apply(grid) {
        return true;
    }
    if apply_connectivity::apply(grid) {
        return true;
    }
    false
}

fn apply_pbc_deductions(grid: &mut Grid, depth: usize) {
    for _ in 0..depth {
        if !apply_pbc_deductions_once(grid) {
            break;
        }
    }
}

pub fn apply(grid: &mut Grid, depth: usize) -> bool {
    log::debug!("attempting proof by contradiction");
    for i in 0..grid.len() {
        let r = i / grid.cols();
        let c = i % grid.cols();
        let x = (r, c);

        if !grid[x].empty() {
            continue;
        }

        let mut hyp = grid.clone();
        hyp[x] = Cell::Black;
        apply_pbc_deductions(&mut hyp, depth);
        if rules::check(&hyp) == RuleCheck::Contradiction {
            log::debug!("pbc found contradiction in\n{}", DumpGrid("?....", &hyp));
            log::debug!("by pbc cell at {x:?} is white",);
            grid[i] = Cell::White;
            return true;
        }

        let mut hyp = grid.clone();
        hyp[x] = Cell::White;
        apply_pbc_deductions(&mut hyp, depth);
        if rules::check(&hyp) == RuleCheck::Contradiction {
            log::debug!("pbc found contradiction in {}", DumpGrid("?....", &hyp));
            log::debug!("by pbc cell at {x:?} is black",);
            grid[x] = Cell::Black;
            return true;
        }
    }
    false
}
