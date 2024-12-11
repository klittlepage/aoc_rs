use std::path::Path;

use anyhow::Result;

use cli::{part::Part, util::file_path};
use util::data::{
    table::Table, Coordinate2d, Direction2d, IterationInstruction, TwoDimensionalArray,
};

use crate::example_dir_for_day;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum State {
    #[default]
    AwaitingX,
    AwaitingM,
    AwaitingA,
    AwaitingS,
    Xmas,
    NoMatch,
}

impl State {
    fn on_value(&mut self, c: char) {
        match (&self, c) {
            (Self::AwaitingX, 'X') => *self = Self::AwaitingM,
            (Self::AwaitingM, 'M') => *self = Self::AwaitingA,
            (Self::AwaitingA, 'A') => *self = Self::AwaitingS,
            (Self::AwaitingS, 'S') | (Self::Xmas, _) => *self = Self::Xmas,
            (
                Self::AwaitingX
                | Self::AwaitingM
                | Self::AwaitingA
                | Self::AwaitingS
                | Self::NoMatch,
                _,
            ) => *self = Self::NoMatch,
        }
    }
}

pub(crate) fn run(base_dir: &Path, part: Part, example: bool) -> Result<String> {
    let path = file_path(&example_dir_for_day(base_dir, 4), part, example);
    let table = Table::<char>::read_from_path(&path)?;
    let solution = match part {
        Part::P1 => solve_p1(&table),
        Part::P2 => solve_p2(&table),
    };
    Ok(solution.to_string())
}

fn solve_p1(table: &Table<char>) -> usize {
    let mut prev_direction: Option<Direction2d> = None;
    let mut state = State::default();
    let mut count = 0;

    for (row_idx, row) in table.iter().enumerate() {
        for col_idx in 0..row.len() {
            let starting_position = Coordinate2d::new(row_idx, col_idx);
            table.iter_all_directions(starting_position, &mut |direction, _coord, value| {
                if Some(direction) != prev_direction {
                    prev_direction = Some(direction);
                    state = State::default();
                }
                state.on_value(*value);
                match state {
                    State::NoMatch => IterationInstruction::AbortDirection,
                    State::Xmas => {
                        count += 1;
                        IterationInstruction::AbortDirection
                    }
                    _ => IterationInstruction::Continue,
                }
            });
        }
    }
    count
}

fn solve_p2(table: &Table<char>) -> usize {
    fn table_value(table: &Table<char>, coordinate: Option<Coordinate2d>) -> Option<&char> {
        let coordinate = coordinate?;
        table.get(coordinate.1, coordinate.0)
    }

    fn xmas(c_1: Option<&char>, c_2: Option<&char>) -> bool {
        match (c_1, c_2) {
            (Some('M'), Some('S')) | (Some('S'), Some('M')) => true,
            (_, _) => false,
        }
    }

    let mut count = 0;

    for (row_idx, row) in table.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            if *value != 'A' {
                continue;
            }
            let coord = Coordinate2d::new(col_idx, row_idx);
            let ne = table_value(table, coord.step(Direction2d::NorthEast));
            let se = table_value(table, coord.step(Direction2d::SouthEast));
            let sw = table_value(table, coord.step(Direction2d::SouthWest));
            let nw = table_value(table, coord.step(Direction2d::NorthWest));
            if xmas(ne, sw) && xmas(se, nw) {
                count += 1;
            }
        }
    }

    count
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use cli::util::default_data_dir;

    use super::*;

    #[test]
    fn test_example_1() {
        let path = file_path(&example_dir_for_day(&default_data_dir(), 4), Part::P1, true);
        let table = Table::<char>::read_from_path(&path).unwrap();
        assert_eq!(18, solve_p1(&table));
    }

    #[test]
    fn test_example_2() {
        let path = file_path(&example_dir_for_day(&default_data_dir(), 4), Part::P2, true);
        let table = Table::<char>::read_from_path(&path).unwrap();
        assert_eq!(9, solve_p2(&table));
    }

    #[test]
    fn test_part_1() {
        let path = file_path(
            &example_dir_for_day(&default_data_dir(), 4),
            Part::P1,
            false,
        );
        let table = Table::<char>::read_from_path(&path).unwrap();
        assert_eq!(2662, solve_p1(&table));
    }

    #[test]
    fn test_part_2() {
        let path = file_path(
            &example_dir_for_day(&default_data_dir(), 4),
            Part::P2,
            false,
        );
        let table = Table::<char>::read_from_path(&path).unwrap();
        assert_eq!(2034, solve_p2(&table));
    }
}
