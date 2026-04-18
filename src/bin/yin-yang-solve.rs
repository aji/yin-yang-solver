use std::fmt;

use clap::{Parser, ValueEnum};
use yin_yang_solver::{Cell, Grid, GridView, SolveResult, SolveStep};

#[derive(Parser)]
struct Cli {
    #[arg(value_name = "PUZZLE")]
    puzzle: String,

    #[arg(short = 'f', long)]
    output_format: Option<OutputFormat>,

    #[arg(short, long)]
    output: Option<String>,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum InputFormat {
    Image,
    Url,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum OutputFormat {
    Ascii,
    Colors,
    Url,
    Html,
}

const HTML_TEMPLATE: &'static str = include_str!("template.html");

const DEFAULT_PUZZLE_SITE: &'static str = "https://puzz.link/p";
const KNOWN_PUZZLE_SITES: &'static [&'static str] =
    &["//puzz.link/p", "//pzprxs.vercel.app/p", "//pzv.jp/p.html"];

impl InputFormat {
    fn guess(arg: &str) -> Option<Self> {
        if arg.starts_with("https://") {
            Some(Self::Url)
        } else if arg.ends_with(".png") {
            Some(Self::Image)
        } else {
            None
        }
    }

    fn load(&self, arg: &str) -> Result<Grid, String> {
        match self {
            Self::Image => yin_yang_extractor::extract_from_image_file(arg)
                .map_err(|e| format!("failed to load image: {e}")),
            Self::Url => get_puzzle_from_url(arg),
        }
    }
}

impl OutputFormat {
    fn save(
        &self,
        arg: Option<&str>,
        initial_grid: GridView,
        grid: GridView,
        path: &[SolveStep],
    ) -> Result<(), String> {
        let out = match self {
            Self::Ascii => format!("{}", FormatAscii(grid)),
            Self::Colors => format!("{}", FormatColors(grid)),
            Self::Url => format!("{}\n", FormatUrl(grid)),
            Self::Html => HTML_TEMPLATE.replace(
                "{PATH}",
                path_to_html_list(initial_grid, grid, path).as_str(),
            ),
        };
        match arg {
            Some(path) => std::fs::write(path, out).map_err(|e| format!("{e}")),
            None => Ok(print!("{out}")),
        }
    }
}

fn step_to_html_list_item(grid: &mut Grid, step: &SolveStep) -> String {
    match step {
        SolveStep::ApplySolid2x2(r, c, cell) => {
            grid[(*r, *c)] = *cell;
            GridListItem(grid.as_ref(), format!("avoid 2x2 solid"), Vec::new()).to_string()
        }
        SolveStep::ApplyCheckerboard2x2(r, c, cell) => {
            grid[(*r, *c)] = *cell;
            GridListItem(grid.as_ref(), format!("avoid 2x2 checkered"), Vec::new()).to_string()
        }
        SolveStep::ApplyBorder(items) => {
            for (r, c, cell) in items.iter() {
                grid[(*r, *c)] = *cell;
            }
            GridListItem(grid.as_ref(), format!("apply border logic"), Vec::new()).to_string()
        }
        SolveStep::ApplyConnectivity(items) => {
            for (r, c, cell) in items.iter() {
                grid[(*r, *c)] = *cell;
            }
            GridListItem(grid.as_ref(), format!("apply connectivity"), Vec::new()).to_string()
        }
        SolveStep::ApplyPbc(r, c, cell, hyppath) => {
            let mut hyp = grid.clone();
            hyp[(*r, *c)] = match cell {
                Cell::Empty => Cell::Empty,
                Cell::Black => Cell::White,
                Cell::White => Cell::Black,
            };
            let mut hypitems = Vec::new();
            hypitems.push(GridListItem(hyp.as_ref(), "hypothesis".into(), Vec::new()).to_string());
            for step in hyppath.iter() {
                hypitems.push(step_to_html_list_item(&mut hyp, step));
            }
            grid[(*r, *c)] = *cell;
            GridListItem(grid.as_ref(), format!("proof by contradiction"), hypitems).to_string()
        }
    }
}

fn path_to_html_list_items(grid: GridView, path: &[SolveStep]) -> String {
    let mut grid = grid.into_owned();
    let mut lis: Vec<String> = Vec::new();

    for step in path.iter() {
        lis.push(step_to_html_list_item(&mut grid, step));
    }

    lis.join("")
}

fn path_to_html_list(grid: GridView, final_grid: GridView, path: &[SolveStep]) -> String {
    let lis = path_to_html_list_items(grid, path);
    return format!(
        r#"<ul id="path">{}{lis}{}</ul>"#,
        GridListItem(grid, "start".into(), Vec::new()),
        GridListItem(final_grid, "done".into(), Vec::new())
    );
}

struct GridListItem<'g>(GridView<'g>, String, Vec<String>);

impl<'g> fmt::Display for GridListItem<'g> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let g = self.0;
        write!(f, "<li data-puz=\"")?;
        for r in 0..g.rows() {
            if r != 0 {
                write!(f, "/")?;
            }
            for c in 0..g.cols() {
                match g[(r, c)] {
                    Cell::Empty => write!(f, ".")?,
                    Cell::Black => write!(f, "B")?,
                    Cell::White => write!(f, "W")?,
                }
            }
        }
        write!(f, "\">")?;
        write!(f, "{}", self.1)?;
        if self.2.len() > 0 {
            write!(f, "<ul>")?;
            for item in self.2.iter() {
                write!(f, "{item}")?;
            }
            write!(f, "</ul>")?;
        }
        write!(f, "</li>")?;
        Ok(())
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
        let url_query = pzpr_codec::yinyang::encode(&self.0).unwrap();
        write!(f, "{DEFAULT_PUZZLE_SITE}?{url_query}")
    }
}

pub fn main() {
    let cli = Cli::parse();

    env_logger::init();

    log::debug!("loading puzzle");
    let input = InputFormat::guess(&cli.puzzle)
        .expect("could not determine input puzzle format")
        .load(&cli.puzzle)
        .expect("could not load input puzzle");

    log::debug!("solving puzzle");
    let mut path = Vec::new();
    let output = match yin_yang_solver::solve(input.clone(), Some(&mut path)) {
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
        .unwrap_or(OutputFormat::Ascii)
        .save(
            cli.output.as_ref().map(String::as_str),
            input.as_ref(),
            output.as_ref(),
            &path,
        )
        .expect("could not save output");
}
