use std::fmt;

use crate::{Cell, Grid};

pub struct DumpGrid<'g>(pub &'static str, pub &'g Grid);

impl<'g> fmt::Display for DumpGrid<'g> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pfx = self.0;
        let g = self.1;
        write!(f, "{pfx}   ")?;
        for c in 0..g.cols() {
            write!(f, "{c:2} ")?;
        }
        for r in 0..g.rows() {
            write!(f, "\n{pfx}{r:2} ")?;
            for c in 0..g.cols() {
                match g[(r, c)] {
                    Cell::Empty => write!(f, " - ")?,
                    Cell::Black => write!(f, "( )")?,
                    Cell::White => write!(f, "(@)")?,
                }
            }
        }
        Ok(())
    }
}
