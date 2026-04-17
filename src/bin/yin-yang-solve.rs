use std::fmt;

use clap::{Parser, ValueEnum};
use yin_yang_solver::{Cell, Grid, GridView, SolveResult};

#[derive(Parser)]
struct Cli {
    #[arg(value_name = "PUZZLE")]
    puzzle: String,

    #[arg(short = 'f', long)]
    output_format: Option<PuzzleFormat>,

    #[arg(short, long)]
    output: Option<String>,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum PuzzleFormat {
    Image,
    Ascii,
    Colors,
    Url,
}

const DEFAULT_PUZZLE_SITE: &'static str = "https://puzz.link/p";
const KNOWN_PUZZLE_SITES: &'static [&'static str] =
    &["//puzz.link/p", "//pzprxs.vercel.app/p", "//pzv.jp/p.html"];

impl PuzzleFormat {
    fn guess(arg: &str) -> Option<Self> {
        if arg.starts_with("https://") {
            Some(PuzzleFormat::Url)
        } else if arg.ends_with(".png") {
            Some(PuzzleFormat::Image)
        } else {
            None
        }
    }

    fn load(&self, arg: &str) -> Result<Grid, String> {
        match self {
            PuzzleFormat::Image => yin_yang_extractor::extract_from_image_file(arg)
                .map_err(|e| format!("failed to load image: {e}")),
            PuzzleFormat::Ascii => Err("ascii format cannot be used for load".into()),
            PuzzleFormat::Colors => Err("colors format cannot be used for load".into()),
            PuzzleFormat::Url => get_puzzle_from_url(arg),
        }
    }

    fn save(&self, arg: Option<&str>, grid: GridView) -> Result<(), String> {
        match self {
            PuzzleFormat::Image => Err("image format cannot be used for save".into()),
            PuzzleFormat::Ascii => match arg {
                Some(_) => Err("ascii output to file not implemented yet".into()),
                None => Ok(print!("{}", FormatAscii(grid))),
            },
            PuzzleFormat::Colors => match arg {
                Some(_) => Err("colors output to file not implemented yet".into()),
                None => Ok(print!("{}", FormatColors(grid))),
            },
            PuzzleFormat::Url => match arg {
                Some(_) => Err("url output to file not implemented yet".into()),
                None => Ok(println!("{DEFAULT_PUZZLE_SITE}?{}", FormatUrl(grid))),
            },
        }
    }
}

fn get_puzzle_from_url(url: &str) -> Result<Grid, String> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("input does not appear to be a url".into());
    }
    let Some((path, query)) = url.split_once('?') else {
        return Err("url has no query delimiter ('?')".into());
    };
    if !KNOWN_PUZZLE_SITES.iter().any(|p| path.ends_with(p)) {
        log::warn!("parse may fail, unknown puzzle site: {path}");
    }
    pzpr_codec::yinyang::decode(query)
}

struct FormatAscii<'g>(GridView<'g>);

impl<'g> fmt::Display for FormatAscii<'g> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let g = self.0;
        for row in 0..g.rows() {
            for col in 0..g.cols() {
                if col != 0 {
                    write!(f, " ")?;
                }
                match g[(row, col)] {
                    Cell::Empty => write!(f, ".")?,
                    Cell::Black => write!(f, "B")?,
                    Cell::White => write!(f, "W")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

struct FormatColors<'g>(GridView<'g>);

impl<'g> fmt::Display for FormatColors<'g> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let g = self.0;
        for row in 0..g.rows() {
            for col in 0..g.cols() {
                match g[(row, col)] {
                    Cell::Empty => write!(f, "  ")?,
                    Cell::Black => write!(f, "⬜")?,
                    Cell::White => write!(f, "⬛")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

struct FormatUrl<'g>(GridView<'g>);

impl<'g> fmt::Display for FormatUrl<'g> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let url = pzpr_codec::yinyang::encode(&self.0).unwrap();
        write!(f, "{url}")
    }
}

pub fn main() {
    let cli = Cli::parse();

    env_logger::init();

    log::debug!("loading puzzle");
    let input = PuzzleFormat::guess(&cli.puzzle)
        .expect("could not determine input puzzle format")
        .load(&cli.puzzle)
        .expect("could not load input puzzle");

    log::debug!("solving puzzle");
    let output = match yin_yang_solver::solve(input.clone()) {
        SolveResult::NoSolutions => {
            log::error!("puzzle has no solutions");
            std::process::exit(1);
        }
        SolveResult::Partial(grid) => {
            log::warn!("partial solve");
            grid
        }
        SolveResult::Solved(grid) => grid,
    };

    log::debug!("saving solved puzzle");
    cli.output_format
        .unwrap_or(PuzzleFormat::Ascii)
        .save(cli.output.as_ref().map(String::as_str), output.as_ref())
        .expect("could not save output");
}
