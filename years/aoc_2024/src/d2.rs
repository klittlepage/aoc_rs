use std::path::Path;

use anyhow::Result;

use cli::{part::Part, util::file_path};
use util::data::{
    all_negative_i64, all_positive_i64, nth_difference_i64,
    table::{read_i64_table, Table},
};

use crate::example_dir_for_day;

pub(crate) fn run(base_dir: &Path, part: Part, example: bool) -> Result<String> {
    let path = file_path(&example_dir_for_day(base_dir, 2), part, example);
    let table = read_i64_table(&path)?;
    let solution = match part {
        Part::P1 => solve_p1(&table),
        Part::P2 => solve_p2(&table),
    };
    Ok(solution.to_string())
}

fn is_safe(row: &[i64]) -> bool {
    let diffs = nth_difference_i64(row, 1);
    if all_positive_i64(&diffs) {
        diffs.iter().all(|x| (1..=3).contains(x))
    } else if all_negative_i64(&diffs) {
        diffs.iter().all(|x| (-3..0).contains(x))
    } else {
        false
    }
}

fn solve_p1(table: &Table<i64>) -> usize {
    table
        .iter()
        .map(|x| is_safe(x.as_slice()))
        .map(usize::from)
        .sum()
}

fn solve_p2(table: &Table<i64>) -> usize {
    fn try_solve(row: &[i64]) -> bool {
        if is_safe(row) {
            return true;
        }
        for idx in 0..row.len() {
            let mut dampened_row = row.to_owned();
            dampened_row.remove(idx);
            if is_safe(&dampened_row) {
                return true;
            }
        }
        false
    }
    table
        .iter()
        .map(|x| try_solve(x.as_slice()))
        .map(usize::from)
        .sum()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use cli::util::default_data_dir;

    #[test]
    fn test_example_1() {
        let path = file_path(&example_dir_for_day(&default_data_dir(), 2), Part::P1, true);
        let table = read_i64_table(&path).expect("valid table");
        assert_eq!(2, solve_p1(&table));
    }

    #[test]
    fn test_example_2() {
        let path = file_path(&example_dir_for_day(&default_data_dir(), 2), Part::P2, true);
        let table = read_i64_table(&path).expect("valid table");
        assert_eq!(4, solve_p2(&table));
    }

    #[test]
    fn test_part_1() {
        let path = file_path(
            &example_dir_for_day(&default_data_dir(), 2),
            Part::P1,
            false,
        );
        let table = read_i64_table(&path).expect("valid table");
        assert_eq!(326, solve_p1(&table));
    }

    #[test]
    fn test_part_2() {
        let path = file_path(
            &example_dir_for_day(&default_data_dir(), 2),
            Part::P2,
            false,
        );
        let table = read_i64_table(&path).expect("valid table");
        assert_eq!(381, solve_p2(&table));
    }
}
