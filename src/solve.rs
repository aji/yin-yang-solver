use crate::{
    Grid, apply_2x2, apply_border, apply_connectivity, apply_pbc,
    dump::DumpGrid,
    rules::{self, RuleCheck},
};

fn apply_solve(grid: &mut Grid) -> bool {
    if apply_2x2::apply(grid) {
        log::debug!("applied 2x2 logic");
        return true;
    }
    if apply_border::apply(grid) {
        log::debug!("applied border logic");
        return true;
    }
    if apply_connectivity::apply(grid) {
        log::debug!("applied connectivity logic");
        return true;
    }
    if apply_pbc::apply(grid, 2) {
        return true;
    }
    false
}

pub fn solve(mut grid: Grid) -> SolveResult {
    loop {
        log::debug!("solver progress:\n{}", DumpGrid("", &grid));
        match rules::check(&grid) {
            RuleCheck::Unsolved => match apply_solve(&mut grid) {
                true => continue,
                false => return SolveResult::Partial(grid),
            },
            RuleCheck::Solved => return SolveResult::Solved(grid),
            RuleCheck::Contradiction => return SolveResult::NoSolutions,
        }
    }
}

pub enum SolveResult {
    NoSolutions,
    Partial(Grid),
    Solved(Grid),
}
