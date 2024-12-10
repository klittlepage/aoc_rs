pub mod matrix;
pub mod table;

mod array_2d_core;

use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IterationInstruction {
    Continue,
    AbortDirection,
    Abort,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coordinate2d(usize, usize);

impl Coordinate2d {
    #[must_use]
    pub fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Step2d(isize, isize);

impl Step2d {
    #[must_use]
    pub fn new(x: isize, y: isize) -> Self {
        Self(x, y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
pub enum Direction2d {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction2d {
    #[must_use]
    pub fn step(&self) -> Step2d {
        match self {
            Self::North => Step2d::new(0, -1),
            Self::NorthEast => Step2d::new(1, -1),
            Self::East => Step2d::new(1, 0),
            Self::SouthEast => Step2d::new(1, 1),
            Self::South => Step2d::new(0, 1),
            Self::SouthWest => Step2d::new(-1, 1),
            Self::West => Step2d::new(-1, 0),
            Self::NorthWest => Step2d::new(-1, -1),
        }
    }
}

pub trait TwoDimensionalArray<T>
where
    T: Clone,
{
    fn new(values: Vec<Vec<T>>) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn n_rows(&self) -> usize;

    fn row(&self, index: usize) -> Option<&[T]>;

    fn row_mut(&mut self, index: usize) -> Option<&mut [T]>;

    fn col(&self, index: usize) -> Option<Vec<&T>>;

    fn get(&self, row: usize, col: usize) -> Option<&T>;

    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T>;

    fn iter_cardnal<F: FnMut(Direction2d, Coordinate2d, &T) -> IterationInstruction>(
        &self,
        starting_coord: Coordinate2d,
        cb: &mut F,
    ) {
        self.iter_directions(
            starting_coord,
            &[
                Direction2d::North,
                Direction2d::East,
                Direction2d::South,
                Direction2d::West,
            ],
            cb,
        );
    }

    fn iter_all_directions<F: FnMut(Direction2d, Coordinate2d, &T) -> IterationInstruction>(
        &self,
        starting_coord: Coordinate2d,
        cb: &mut F,
    ) {
        let directions: Vec<Direction2d> = Direction2d::iter().collect();
        self.iter_directions(starting_coord, &directions, cb);
    }

    #[allow(clippy::cast_sign_loss)]
    fn iter_directions<F>(
        &self,
        starting_coord: Coordinate2d,
        directions: &[Direction2d],
        cb: &mut F,
    ) where
        F: FnMut(Direction2d, Coordinate2d, &T) -> IterationInstruction,
    {
        for direction in directions {
            let Step2d(delta_x, delta_y) = direction.step();
            let mut iter_x = starting_coord.0 as i128;
            let mut iter_y = starting_coord.1 as i128;

            while let Some(v) = self.get(iter_y as usize, iter_x as usize) {
                match cb(
                    *direction,
                    Coordinate2d::new(iter_x as usize, iter_y as usize),
                    v,
                ) {
                    IterationInstruction::Abort => return,
                    IterationInstruction::AbortDirection => break,
                    IterationInstruction::Continue => {}
                }
                iter_x += delta_x as i128;
                iter_y += delta_y as i128;
            }
        }
    }
}

#[must_use]
pub fn nth_difference_i64(input: &[i64], n: usize) -> Vec<i64> {
    let vals = input.iter();
    let next_vals = input.iter().skip(n);
    vals.zip(next_vals).map(|(cur, next)| next - cur).collect()
}

#[must_use]
pub fn all_positive_i64(input: &[i64]) -> bool {
    input.iter().all(|x| x.is_positive())
}

#[must_use]
pub fn all_negative_i64(input: &[i64]) -> bool {
    input.iter().all(|x| x.is_negative())
}

#[allow(clippy::unwrap_used, clippy::panic)]
#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    pub(crate) fn iter_direction<T>(arr: &T)
    where
        T: TwoDimensionalArray<u32>,
    {
        let mut iter_idx = 0;

        arr.iter_all_directions(Coordinate2d::new(1, 1), &mut |dir, coord, value| {
            match iter_idx {
                0 => {
                    assert_eq!(dir, Direction2d::North);
                    assert_eq!(coord, Coordinate2d::new(1, 1));
                    assert_eq!(*value, 5);
                }
                1 => {
                    assert_eq!(dir, Direction2d::North);
                    assert_eq!(coord, Coordinate2d::new(1, 0));
                    assert_eq!(*value, 2);
                }
                2 => {
                    assert_eq!(dir, Direction2d::NorthEast);
                    assert_eq!(coord, Coordinate2d::new(1, 1));
                }
                3 => {
                    assert_eq!(dir, Direction2d::NorthEast);
                    assert_eq!(coord, Coordinate2d::new(2, 0));
                    assert_eq!(*value, 3);
                }
                4 => {
                    assert_eq!(dir, Direction2d::East);
                    assert_eq!(coord, Coordinate2d::new(1, 1));
                }
                5 => {
                    assert_eq!(dir, Direction2d::East);
                    assert_eq!(coord, Coordinate2d::new(2, 1));
                    assert_eq!(*value, 6);
                }
                6 => {
                    assert_eq!(dir, Direction2d::SouthEast);
                    assert_eq!(coord, Coordinate2d::new(1, 1));
                }
                7 => {
                    assert_eq!(dir, Direction2d::SouthEast);
                    assert_eq!(coord, Coordinate2d::new(2, 2));
                    assert_eq!(*value, 9);
                }
                8 => {
                    assert_eq!(dir, Direction2d::South);
                    assert_eq!(coord, Coordinate2d::new(1, 1));
                }
                9 => {
                    assert_eq!(dir, Direction2d::South);
                    assert_eq!(coord, Coordinate2d::new(1, 2));
                    assert_eq!(*value, 8);
                }
                10 => {
                    assert_eq!(dir, Direction2d::SouthWest);
                    assert_eq!(coord, Coordinate2d::new(1, 1));
                }
                11 => {
                    assert_eq!(dir, Direction2d::SouthWest);
                    assert_eq!(coord, Coordinate2d::new(0, 2));
                    assert_eq!(*value, 7);
                }
                12 => {
                    assert_eq!(dir, Direction2d::West);
                    assert_eq!(coord, Coordinate2d::new(1, 1));
                }
                13 => {
                    assert_eq!(dir, Direction2d::West);
                    assert_eq!(coord, Coordinate2d::new(0, 1));
                    assert_eq!(*value, 4);
                }
                14 => {
                    assert_eq!(dir, Direction2d::NorthWest);
                    assert_eq!(coord, Coordinate2d::new(1, 1));
                }
                15 => {
                    assert_eq!(dir, Direction2d::NorthWest);
                    assert_eq!(coord, Coordinate2d::new(0, 0));
                    assert_eq!(*value, 1);
                }
                _ => {
                    panic!("bad state")
                }
            }

            iter_idx += 1;
            IterationInstruction::Continue
        });
    }
}
