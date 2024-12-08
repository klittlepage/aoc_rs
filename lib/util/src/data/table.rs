use std::{fmt, path::Path};

use anyhow::{Context, Result};

use crate::io::read_with_callback;

#[derive(Debug, Clone)]
pub struct Table<T: Clone> {
    values: Vec<Vec<T>>,
    n_rows: usize,
}

impl<T: Clone> Table<T> {
    pub fn read_from_path<F: FnMut(&str) -> Result<T>>(
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

    #[must_use]
    pub fn n_rows(&self) -> usize {
        self.n_rows
    }

    pub fn row(&self, index: usize) -> Option<&[T]> {
        self.values.get(index).map(Vec::as_slice)
    }

    pub fn row_mut(&mut self, index: usize) -> Option<&mut [T]> {
        self.values.get_mut(index).map(Vec::as_mut_slice)
    }

    #[must_use]
    pub fn col(&self, index: usize) -> Option<Vec<&T>> {
        let mut col: Vec<&T> = Vec::with_capacity(self.n_rows);
        for row in &self.values {
            let col_value = row.get(index)?;
            col.push(col_value);
        }
        Some(col)
    }

    #[must_use]
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        let row = self.values.get(row)?;
        row.get(col)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        let row = self.values.get_mut(row)?;
        row.get_mut(col)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec<T>> {
        self.values.iter()
    }
}

impl<T: Clone + fmt::Display> fmt::Display for Table<T> {
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

pub fn read_i64_table(path: &Path) -> Result<Table<i64>> {
    Table::read_from_path(path, " ", |x| x.parse::<i64>().context("invalid i64"))
        .context("failed to parse input file")
}

pub fn read_u64_table(path: &Path) -> Result<Table<u64>> {
    Table::read_from_path(path, " ", |x| x.parse::<u64>().context("invalid u64"))
        .context("failed to parse input file")
}
