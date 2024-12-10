use std::{fmt, path::Path};

use anyhow::{Context, Result};

use super::{array_2d_core::Array2dCore, TwoDimensionalArray};

#[derive(Debug, Clone)]
pub struct Table<T: Clone> {
    inner: Array2dCore<T>,
}

impl<T: Clone> Table<T> {
    pub(crate) fn read_generic_from_path<F: FnMut(&str) -> Result<T>>(
        path: &Path,
        sep: &str,
        map: F,
    ) -> Result<Self> {
        Ok(Self {
            inner: Array2dCore::read_generic_from_path(path, sep, map)?,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec<T>> {
        self.inner.iter()
    }
}

impl Table<char> {
    pub fn read_from_path(path: &Path) -> Result<Self> {
        Ok(Self {
            inner: Array2dCore::<char>::read_chars(path)?,
        })
    }
}

crate::data::array_2d_core::typed_readers!(Table);
crate::data::array_2d_core::show_wrapper!(Table);

impl<T> TwoDimensionalArray<T> for Table<T>
where
    T: Clone,
{
    fn new(values: Vec<Vec<T>>) -> Result<Self> {
        let inner = Array2dCore::new(values)?;
        Ok(Self { inner })
    }

    crate::data::array_2d_core::wrapper_methods!();
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use crate::data::test_support::iter_direction;

    use super::*;

    #[test]
    fn test_iter_direction() {
        let row_1 = vec![1, 2, 3];
        let row_2 = vec![4, 5, 6];
        let row_3 = vec![7, 8, 9];
        let values = vec![row_1, row_2, row_3];

        let mat = Table::new(values).unwrap();
        iter_direction(&mat);
    }
}
