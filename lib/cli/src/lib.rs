#![deny(clippy::all, clippy::pedantic, clippy::panic, clippy::unwrap_used)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions
)]

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use part::Part;
use util::default_data_dir;

pub mod part;
pub mod util;

/// AOC challenge runner
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Data directory
    #[arg(short, long, default_value=default_data_dir().into_os_string())]
    pub data_dir: PathBuf,
    /// Run example
    #[arg(short, long, default_value_t = false)]
    pub example: bool,
    /// Problem day
    #[arg(value_parser=clap::value_parser!(u16).range(1..=25))]
    pub day: u16,
    /// Problem part
    #[arg()]
    pub problem_part: Part,
}

pub fn show_result(
    day: u16,
    problem_part: Part,
    example: bool,
    result: Result<String>,
) -> Result<()> {
    match result {
        Ok(value) => {
            let part_idx = problem_part.value();
            if example {
                println!("the solution to day {day}, example {part_idx} is: {value}");
            } else {
                println!("the solution to day {day}, {part_idx} is: {value}");
            }
            Ok(())
        }
        Err(err) => {
            println!("solving failed with error: {err}");
            Err(err)
        }
    }
}
