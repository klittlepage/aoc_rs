use std::{fmt, path::Path};

use anyhow::{anyhow, Context, Result};

use super::{array_2d_core::Array2dCore, TwoDimensionalArray};

#[derive(Debug, Clone)]
pub struct Matrix<T: Clone> {
    inner: Array2dCore<T>,
    n_cols: usize,
}

impl Matrix<char> {
    pub fn read_from_path(path: &Path) -> Result<Self> {
        let inner = Array2dCore::<char>::read_chars(path)?;
        let n_cols = inner
            .n_cols_if_uniform()
            .ok_or(anyhow!("rows are not of uniform length"))?;
        Ok(Self { inner, n_cols })
    }
}

crate::data::array_2d_core::typed_readers!(Matrix);
crate::data::array_2d_core::show_wrapper!(Matrix);

impl<T: Clone> Matrix<T> {
    pub fn read_generic_from_path<F: FnMut(&str) -> Result<T>>(
        path: &Path,
        sep: &str,
        map: F,
    ) -> Result<Self> {
        let inner = Array2dCore::read_generic_from_path(path, sep, map)?;
        let n_cols = inner
            .n_cols_if_uniform()
            .ok_or(anyhow!("rows are not of uniform length"))?;
        Ok(Self { inner, n_cols })
    }

    #[must_use]
    pub fn n_cols(&self) -> usize {
        self.n_cols
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec<T>> {
        self.inner.iter()
    }
}

impl<T> TwoDimensionalArray<T> for Matrix<T>
where
    T: Clone,
{
    fn new(values: Vec<Vec<T>>) -> Result<Self> {
        let inner = Array2dCore::new(values)?;
        let n_cols = inner
            .n_cols_if_uniform()
            .ok_or(anyhow!("rows are not of uniform length"))?;
        Ok(Self { inner, n_cols })
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

        let mat = Matrix::new(values).unwrap();
        iter_direction(&mat);
    }
}
