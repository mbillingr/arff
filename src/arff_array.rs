//! This module provides a flat homogenous representation of an Arff data set. The purpose of this
//! representation is to allow easy access to the data in form of a `Vec<T>`, where the user
//! specifies `T`. It includes additional type information about columns, that specialized
//! algoritms may use. The contiguous and homogenous representation can be easily converted into an
//! ndarray for flexibility.

use num_traits::ToPrimitive;

use error::{Result};
use parser::{Attribute, DType, Header};

/// A contiguos and homogenous representation of an Arff data set with additional column meta
/// information.
#[derive(Debug)]
pub struct ArffArray<T> {
    columns: Vec<Attribute>,
    data: Vec<T>,
}

impl<T> ArffArray<T> {
    pub fn new(header: Header, data: Vec<T>) -> Result<Self> {
        Ok(ArffArray {
            columns: header.attrs,
            data
        })
    }

    pub fn at(&self, row: usize, col: usize) -> &T {
        &self.data[row * self.n_cols() + col]
    }

    #[inline(always)]
    pub fn n_cols(&self) -> usize {
        self.columns.len()
    }

    #[inline(always)]
    pub fn n_rows(&self) -> usize {
        self.data.len() / self.n_cols()
    }

    #[inline(always)]
    pub fn raw_data(&self) -> &[T] {
        self.data.as_ref()
    }

    #[inline(always)]
    pub fn raw_attributes(&self) -> &[Attribute] {
        self.columns.as_ref()
    }

    pub fn consume(self) -> (Vec<Attribute>, Vec<T>) {
        (self.columns, self.data)
    }
}

impl<T: Clone> ArffArray<T> {
    pub fn clone_rows(&self, indices: &[usize]) -> ArffArray<T> {
        let n_cols = self.n_cols();

        let mut data = Vec::with_capacity(indices.len() * n_cols);

        for row in indices {
            let col_data = &self.data[row * n_cols..(1+row) * n_cols];
            data.extend_from_slice(col_data);
        }

        ArffArray {
            columns: self.columns.clone(),
            data,
        }
    }

    pub fn clone_cols(&self, indices: &[usize]) -> ArffArray<T> {
        let n_cols = self.n_cols();
        let n_rows = self.n_rows();

        let columns = indices
            .iter()
            .map(|&i| self.columns[i].clone())
            .collect();

        let mut data = Vec::with_capacity(n_rows * indices.len());

        for row in 0..n_rows {
            let row_offset = row * n_cols;
            for col in indices {
                data.push(self.data[row_offset + col].clone());
            }
        }

        ArffArray {
            columns,
            data,
        }
    }

    pub fn clone_cols_by_name(&self, col_names: &[&str]) -> ArffArray<T> {
        let indices: Vec<_> = col_names
            .iter()
            .map(|&n|
                self.columns
                    .iter()
                    .position(|c| c.name == n)
                    .unwrap()
            )
            .collect();

        self.clone_cols(&indices)
    }
}

impl<T: Copy + ToPrimitive> ArffArray<T>
{
    /// Convert the numeric representation of a Nominal value at `row`/`col` into its
    /// corresponding name. Returns None if the value is Numeric.
    pub fn str_at(&self, row: usize, col: usize) -> Option<&str> {
        match self.columns[col].dtype {
            DType::Numeric => None,
            DType::Nominal(ref names) => {
                let value: usize = (self.at(row, col)).to_usize().unwrap();
                Some(&names[value])
            }
            DType::String => unreachable!()
        }
    }
}

#[test]
fn test_array() {
    let array:  ArffArray<f64> = ArffArray {
        columns: vec![
            Attribute {
                name: "a".to_owned(),
                dtype: DType::Numeric,
            },
            Attribute {
                name: "b".to_owned(),
                dtype: DType::Nominal(vec!["here".to_owned(), "there".to_owned()]),
            },
            Attribute {
                name: "c".to_owned(),
                dtype: DType::Nominal(vec!["maybe".to_owned(), "perhaps".to_owned()]),
            }
        ],
        data: vec![1.0, 0.0, 1.0, 3.1, 1.0, 0.0, 9.9, 0.0, 0.0, 5.2, 1.0, 1.0],
    };

    assert_eq!(array.n_cols(), 3);
    assert_eq!(array.n_rows(), 4);

    assert_eq!(array.str_at(0, 0), None);
    assert_eq!(array.str_at(0, 1), Some("here"));
    assert_eq!(array.str_at(0, 2), Some("perhaps"));
    assert_eq!(array.str_at(1, 0), None);
    assert_eq!(array.str_at(1, 1), Some("there"));
    assert_eq!(array.str_at(1, 2), Some("maybe"));

    let middle = array.clone_rows(&[1, 2]);
    assert_eq!(middle.n_cols(), 3);
    assert_eq!(middle.n_rows(), 2);
    assert_eq!(middle.columns, array.columns);
    assert_eq!(middle.data[..], array.data[3..9]);

    let ab = array.clone_cols(&[0, 1]);
    assert_eq!(ab.n_cols(), 2);
    assert_eq!(ab.n_rows(), 4);
    assert_eq!(ab.columns[..], array.columns[..2]);
    assert_eq!(ab.data, [1.0, 0.0, 3.1, 1.0, 9.9, 0.0, 5.2, 1.0]);

    let bc = array.clone_cols_by_name(&["b", "c"]);
    assert_eq!(bc.n_cols(), 2);
    assert_eq!(bc.n_rows(), 4);
    assert_eq!(bc.columns[..], array.columns[1..]);
    assert_eq!(bc.data, [0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0]);
}