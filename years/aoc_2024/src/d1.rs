use std::{
    collections::{hash_map::Entry, HashMap},
    ops::AddAssign,
    path::Path,
};

use anyhow::{bail, Context, Result};
use cli::util::file_path;
use rug::{Complete, Integer};

use cli::part::Part;
use util::io::read_lines;

use crate::example_dir_for_day;

pub(crate) fn run(base_dir: &Path, part: Part, example: bool) -> Result<String> {
    let path = file_path(&example_dir_for_day(base_dir, 1), part, example);
    let (lhs, rhs) = read(&path)?;
    let solution = match part {
        Part::P1 => solve_p1(lhs, rhs),
        Part::P2 => solve_p2(&lhs, &rhs),
    };
    Ok(solution.to_string())
}

fn read(path: &Path) -> Result<(Vec<Integer>, Vec<Integer>)> {
    let mut lhs: Vec<Integer> = vec![];
    let mut rhs: Vec<Integer> = vec![];

    if let Ok(lines) = read_lines(path) {
        for line in lines.map_while(Result::ok) {
            let mut split_iter = line.split("   ");
            match (split_iter.next(), split_iter.next()) {
                (Some(l), Some(r)) => {
                    lhs.push(Integer::parse(l).context("not an integer")?.complete());
                    rhs.push(Integer::parse(r).context("not an integer")?.complete());
                }
                (_, _) => bail!("unexpected file format"),
            }
        }
    }

    Ok((lhs, rhs))
}

fn solve_p1(mut lhs: Vec<Integer>, mut rhs: Vec<Integer>) -> Integer {
    lhs.sort();
    rhs.sort();
    lhs.iter()
        .zip(rhs.iter())
        .map(|(x, y)| (x - y).complete().abs())
        .sum()
}

fn solve_p2(lhs: &Vec<Integer>, rhs: &Vec<Integer>) -> Integer {
    let mut rhs_counts: HashMap<Integer, usize> = HashMap::new();

    for entry in rhs {
        match rhs_counts.entry(entry.clone()) {
            Entry::Occupied(mut x) => {
                x.get_mut().add_assign(1);
            }
            Entry::Vacant(x) => {
                x.insert(1);
            }
        }
    }

    let mut total = Integer::new();
    for entry in lhs {
        if let Some(count) = rhs_counts.get(entry) {
            total += Integer::from(*count) * entry;
        }
    }

    total
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use cli::util::default_data_dir;

    #[test]
    fn test_example_1() {
        let path = file_path(&example_dir_for_day(&default_data_dir(), 1), Part::P1, true);
        let (lhs, rhs) = read(&path).unwrap();
        assert_eq!(Integer::from(11), solve_p1(lhs, rhs));
    }

    #[test]
    fn test_example_2() {
        let path = file_path(&example_dir_for_day(&default_data_dir(), 1), Part::P2, true);
        let (lhs, rhs) = read(&path).unwrap();
        assert_eq!(Integer::from(31), solve_p2(&lhs, &rhs));
    }

    #[test]
    fn test_problem_1() {
        let path = file_path(
            &example_dir_for_day(&default_data_dir(), 1),
            Part::P1,
            false,
        );
        let (lhs, rhs) = read(&path).unwrap();
        assert_eq!(Integer::from(1_506_483), solve_p1(lhs, rhs));
    }

    #[test]
    fn test_problem_2() {
        let path = file_path(
            &example_dir_for_day(&default_data_dir(), 1),
            Part::P2,
            false,
        );
        let (lhs, rhs) = read(&path).unwrap();
        assert_eq!(Integer::from(23_126_924), solve_p2(&lhs, &rhs));
    }
}
