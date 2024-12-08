use std::{
    env,
    ffi::OsString,
    fs::canonicalize,
    path::{Path, PathBuf},
    str::FromStr,
};

use chrono::{Datelike, Utc};

use crate::part::Part;

#[must_use]
pub fn default_data_dir() -> PathBuf {
    if let Ok(aoc_dir) = env::var("AOC_DATA_DIR") {
        canonicalize(PathBuf::from(aoc_dir)).expect("failed to canonicalize AOC_DATA_DIR")
    } else {
        let mut base = env::current_dir().expect("invalid current working directory");
        base.push("data");
        base
    }
}

#[must_use]
pub fn current_year() -> OsString {
    let current_date = Utc::now();
    let year = current_date.year();
    let month = current_date.month();
    let year_str = if month == 12 {
        year.to_string()
    } else {
        (year - 1).to_string()
    };
    OsString::from_str(&year_str).expect("invalid aoc year")
}

#[must_use]
pub fn example_dir_for_year_and_day(base_dir: &Path, year: i32, day: u16) -> PathBuf {
    base_dir.join(year.to_string()).join(format!("{day:02}"))
}

#[must_use]
pub fn file_path(example_dir: &Path, part: Part, example: bool) -> PathBuf {
    if example {
        example_dir.join(format!("example_{}.txt", part.value()))
    } else {
        example_dir.join(format!("part_{}.txt", part.value()))
    }
}
