use crate::{Cell, Grid, ext::CellExt, iter::IterBorderMut};

pub fn apply(grid: &mut Grid) -> bool {
    let mut made_progress = false;
    let mut border = {
        let mut border: Vec<Cell> = Vec::new();
        let mut it = IterBorderMut::new();
        while let Some(cell) = it.next(grid) {
            border.push(*cell);
        }
        border
    };

    let border_filled = border
        .iter()
        .copied()
        .filter(|x| !x.empty())
        .collect::<Vec<_>>();
    if border_filled.len() < 3 {
        return false;
    }
    let mut has_black = false;
    let mut has_white = false;
    for c in border_filled.into_iter() {
        match c {
            Cell::Black => has_black = true,
            Cell::White => has_white = true,
            _ => {}
        }
    }
    if !has_black || !has_white {
        return false;
    }

    let n = border.len();
    let mut i = 0;
    while i < n {
        if border[i].empty() {
            i += 1;
        } else {
            let mut j = i + 1;
            while j < i + n - 1 && border[j % n].empty() {
                j += 1;
            }
            if i + 2 <= j && border[j % n] == border[i] {
                for k in i + 1..j {
                    border[k % n] = border[i];
                }
                made_progress = true;
            }
            i = j;
        }
    }

    if made_progress {
        let mut i = 0;
        let mut it = IterBorderMut::new();
        while let Some(cell) = it.next(grid) {
            *cell = border[i];
            i += 1;
        }
    }

    made_progress
}
