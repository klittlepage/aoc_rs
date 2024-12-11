#![deny(clippy::all, clippy::pedantic, clippy::panic, clippy::unwrap_used)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

mod d1;
mod d2;
mod d3;
mod d4;

use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use clap::Parser;

use cli::{part::Part, show_result, util::example_dir_for_year_and_day, Args};

pub(crate) fn example_dir_for_day(base_dir: &Path, day: u16) -> PathBuf {
    example_dir_for_year_and_day(base_dir, 2024, day)
}

fn dispatch(data_dir: &Path, day: u16, problem_part: Part, example: bool) -> Result<String> {
    match day {
        1 => d1::run(data_dir, problem_part, example),
        2 => d2::run(data_dir, problem_part, example),
        3 => d3::run(data_dir, problem_part, example),
        4 => d4::run(data_dir, problem_part, example),
        other => Err(anyhow!("invalid day {}", other)),
    }
}

pub fn main() -> Result<()> {
    let Args {
        data_dir: base_dir,
        example,
        day,
        problem_part,
    } = Args::parse();
    show_result(
        day,
        problem_part,
        example,
        dispatch(&base_dir, day, problem_part, example),
    )
}
