use std::{fmt, path::Path};

use anyhow::{anyhow, bail, Context, Result};

use crate::io::read_with_callback;

use super::TwoDimensionalArray;

#[derive(Debug, Clone)]
pub(crate) struct Array2dCore<T: Clone> {
    pub(crate) values: Vec<Vec<T>>,
    pub(crate) n_rows: usize,
}

impl Array2dCore<char> {
    pub(crate) fn read_chars(path: &Path) -> Result<Self> {
        let mut values: Vec<Vec<char>> = vec![];

        read_with_callback(path, &mut |line| {
            let row: Vec<char> = line.as_str().chars().collect();
            values.push(row);
            Ok(())
        })?;

        let n_rows = values.len();

        Ok(Self { values, n_rows })
    }
}

impl<T: Clone> Array2dCore<T> {
    pub(crate) fn read_generic_from_path<F: FnMut(&str) -> Result<T>>(
        path: &Path,
        sep: &str,
        mut map: F,
    ) -> Result<Self> {
        let mut values: Vec<Vec<T>> = vec![];

        read_with_callback(path, &mut |line| {
            let row: Result<Vec<T>> = line.split(sep).map(&mut map).collect();
            let row = row.context(format!("invalid row {line}"))?;
            values.push(row);
            Ok(())
        })?;

        let n_rows = values.len();

        Ok(Self { values, n_rows })
    }

    pub(crate) fn n_cols_if_uniform(&self) -> Option<usize> {
        let n_cols = self.values.first()?.len();
        if self.values.iter().all(|x| x.len() == n_cols) {
            Some(n_cols)
        } else {
            None
        }
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Vec<T>> {
        self.values.iter()
    }
}

impl<T> TwoDimensionalArray<T> for Array2dCore<T>
where
    T: Clone,
{
    fn new(values: Vec<Vec<T>>) -> Result<Self> {
        let n_rows = values.len();
        let n_cols = values.first().ok_or(anyhow!("no rows"))?.len();
        if n_cols == 0 {
            bail!("zero-length rows are not allowed");
        }
        Ok(Self { values, n_rows })
    }

    #[must_use]
    fn n_rows(&self) -> usize {
        self.n_rows
    }

    fn row(&self, index: usize) -> Option<&[T]> {
        self.values.get(index).map(Vec::as_slice)
    }

    fn row_mut(&mut self, index: usize) -> Option<&mut [T]> {
        self.values.get_mut(index).map(Vec::as_mut_slice)
    }

    #[must_use]
    fn col(&self, index: usize) -> Option<Vec<&T>> {
        let mut col: Vec<&T> = Vec::with_capacity(self.n_rows);
        for row in &self.values {
            let col_value = row.get(index)?;
            col.push(col_value);
        }
        Some(col)
    }

    #[must_use]
    fn get(&self, row: usize, col: usize) -> Option<&T> {
        let row = self.values.get(row)?;
        row.get(col)
    }

    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        let row = self.values.get_mut(row)?;
        row.get_mut(col)
    }
}

impl<T: Clone + fmt::Display> fmt::Display for Array2dCore<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut largest_element = 0usize;

        for row in &self.values {
            for value in row {
                let elem_length = value.to_string().len();
                largest_element = std::cmp::max(largest_element, elem_length);
            }
        }

        for (row_idx, row) in self.values.iter().enumerate() {
            let n_cols = row.len();
            for (col_idx, value) in row.iter().enumerate() {
                let elem = value.to_string();
                write!(f, "{elem:^largest_element$}")?;
                if col_idx != n_cols - 1 {
                    write!(f, " ")?;
                }
            }
            if row_idx != self.n_rows - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

macro_rules! typed_readers {
    ($ident:ident) => {
        impl $ident<i64> {
            pub fn read_from_path(path: &Path) -> Result<$ident<i64>> {
                $ident::read_generic_from_path(path, " ", |x| {
                    x.parse::<i64>().context("invalid i64")
                })
                .context("failed to parse input file")
            }
        }

        impl $ident<u64> {
            pub fn read_from_path(path: &Path) -> Result<$ident<u64>> {
                $ident::read_generic_from_path(path, " ", |x| {
                    x.parse::<u64>().context("invalid i64")
                })
                .context("failed to parse input file")
            }
        }
    };
}

macro_rules! wrapper_methods {
    () => {
        #[must_use]
        fn n_rows(&self) -> usize {
            self.inner.n_rows
        }

        fn row(&self, index: usize) -> Option<&[T]> {
            self.inner.row(index)
        }

        fn row_mut(&mut self, index: usize) -> Option<&mut [T]> {
            self.inner.row_mut(index)
        }

        #[must_use]
        fn col(&self, index: usize) -> Option<Vec<&T>> {
            self.inner.col(index)
        }

        #[must_use]
        fn get(&self, row: usize, col: usize) -> Option<&T> {
            self.inner.get(row, col)
        }

        fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
            self.inner.get_mut(row, col)
        }
    };
}

macro_rules! show_wrapper {
    ($ident:ident) => {
        impl<T: Clone + fmt::Display> fmt::Display for $ident<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.inner)
            }
        }
    };
}

pub(crate) use show_wrapper;
pub(crate) use typed_readers;
pub(crate) use wrapper_methods;
