use crate::Cell;
use std::io;

pub type SolvePath = Vec<SolveStep>;

pub enum SolveStep {
    ApplySolid2x2(usize, usize, Cell),
    ApplyCheckerboard2x2(usize, usize, Cell),
    ApplyBorder,
    ApplyConnectivity,
    ApplyPbc(usize, usize, Cell, Vec<SolveStep>),
}

fn fmt_step<W: io::Write>(f: &mut W, indent: usize, step: &SolveStep) -> io::Result<()> {
    for _ in 0..indent {
        write!(f, "  ")?;
    }

    let cell = match step {
        SolveStep::ApplySolid2x2(_, _, cell) => *cell,
        SolveStep::ApplyCheckerboard2x2(_, _, cell) => *cell,
        SolveStep::ApplyPbc(_, _, cell, _) => *cell,
        _ => Cell::Empty,
    };
    let (color, color_inv) = match cell {
        Cell::Empty => ("empty", "empty"),
        Cell::Black => ("black", "white"),
        Cell::White => ("white", "black"),
    };

    match step {
        SolveStep::ApplySolid2x2(r, c, _) => write!(f, "2x2 forces {r},{c} to {color} (solid)\n"),
        SolveStep::ApplyCheckerboard2x2(r, c, _) => {
            write!(f, "2x2 forces {r},{c} to {color} (checkerboard)\n")
        }
        SolveStep::ApplyBorder => write!(f, "apply border logic\n"),
        SolveStep::ApplyConnectivity => write!(f, "apply connectivity logic\n"),
        SolveStep::ApplyPbc(r, c, _, steps) => {
            write!(f, "pbc at {r},{c} forces {color}; if {color_inv}:\n")?;
            fmt_path_inner(f, indent + 1, steps)
        }
    }
}

fn fmt_path_inner<W: io::Write>(f: &mut W, indent: usize, path: &[SolveStep]) -> io::Result<()> {
    for step in path {
        fmt_step(f, indent, step)?;
    }
    Ok(())
}

pub fn fmt_path<W: io::Write>(f: &mut W, path: &[SolveStep]) -> io::Result<()> {
    fmt_path_inner(f, 0, path)
}
