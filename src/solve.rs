use crate::{
    Grid, apply_2x2, apply_border, apply_connectivity, apply_pbc,
    dump::DumpGrid,
    rules::{self, RuleCheck},
    solve_path::SolvePath,
};

fn apply_solve(grid: &mut Grid, mut path: Option<&mut SolvePath>) -> bool {
    if apply_2x2::apply(grid, path.as_deref_mut()) {
        log::debug!("applied 2x2 logic");
        return true;
    }
    if apply_border::apply(grid, path.as_deref_mut()) {
        log::debug!("applied border logic");
        return true;
    }
    if apply_connectivity::apply(grid, path.as_deref_mut()) {
        log::debug!("applied connectivity logic");
        return true;
    }
    if apply_pbc::apply(grid, path.as_deref_mut(), 5) {
        return true;
    }
    false
}

pub fn solve(mut grid: Grid, mut path: Option<&mut SolvePath>) -> SolveResult {
    loop {
        log::debug!("solver progress:\n{}", DumpGrid("", &grid));
        let path_rb = path.as_deref_mut();
        match rules::check(&grid) {
            RuleCheck::Unsolved => match apply_solve(&mut grid, path_rb) {
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
