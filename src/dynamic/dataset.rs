use std::collections::HashSet;

use arff_array::Array;
use error::Result;
use parser::{Attribute, DType, Parser};

use super::FlatIter;
use super::column::{Column, ColumnType};
use super::value::{CastValue, Value};

/// A dynamically typed representation of an ARFF data set
#[derive(Debug, Clone, PartialEq)]
pub struct DataSet {
    relation: String,
    columns: Vec<Column>,
    n_rows: usize,
}

impl DataSet {
    pub fn new(relation: &str, columns: Vec<Column>) -> DataSet {
        let n_rows = {
            let mut it = columns.iter().map(Column::len);
            let n_rows = it.next().unwrap_or(0);
            assert!(it.all(|l| l == n_rows));
            n_rows
        };

        DataSet {
            relation: relation.to_owned(),
            columns,
            n_rows,
        }
    }

    pub fn name(&self) -> &str {
        &self.relation
    }

    /// Deserialize an instance of type `DataSet` from an ARFF formatted string.
    pub fn from_str(input: &str) -> Result<Self> {
        let mut parser = Parser::new(input);
        let header = parser.parse_header()?;

        let mut columns = Vec::new();

        for attr in header.attrs.into_iter() {
            columns.push(Column::from_attr(attr)?);
        }

        let mut n_rows = 0;

        parser.skip_empty();
        while !parser.is_eof() {
            let mut cit = columns.iter_mut();

            match cit.next() {
                None => {}
                Some(col) => {
                    col.parse_value(&mut parser)?;
                }
            }

            for col in cit {
                parser.parse_column_delimiter()?;
                col.parse_value(&mut parser)?;
            }
            parser.parse_row_delimiter()?;
            parser.skip_empty();

            n_rows += 1;
        }

        Ok(DataSet {
            relation: header.name,
            columns,
            n_rows,
        })
    }

    /// number of rows
    pub fn n_rows(&self) -> usize {
        self.n_rows
    }

    /// number of columns
    pub fn n_cols(&self) -> usize {
        self.columns.len()
    }

    /// column names
    pub fn col_names<'a>(&'a self) -> Box<'a + Iterator<Item = &'a str>> {
        let iter = self.columns.iter().map(|col| col.name());
        Box::new(iter)
    }

    /// column name by index
    pub fn col_name<'a>(&'a self, idx: usize) -> &str {
        self.columns[idx].name()
    }

    /// get data row by index
    pub fn row(&self, idx: usize) -> Vec<Value> {
        self.columns.iter().map(|c| c.item(idx)).collect()
    }

    /// get data column by index
    pub fn col(&self, idx: usize) -> &Column {
        &self.columns[idx]
    }

    /// get column by name
    ///
    /// panics if there is no such column.
    pub fn col_by_name(&self, col: &str) -> &Column {
        for c in &self.columns {
            if c.name() == col {
                return c;
            }
        }
        panic!("unknown column: {}", col);
    }

    /// get item by row/column index
    pub fn item(&self, row: usize, col: usize) -> Value {
        self.col(col).item(row)
    }

    /// get item by row index and column name
    ///
    /// panics if there is no such column.
    pub fn item_by_name<T>(&self, row: usize, col: &str) -> Value {
        self.col_by_name(col).item(row)
    }

    /// returns an iterator over the flattened dataset in row major.
    pub fn flat_iter(&self) -> FlatIter {
        FlatIter::new(self)
    }

    /// move given columns into a separate data set
    pub fn split(self, names: HashSet<&str>) -> (Self, Self) {
        let mut a = DataSet {
            relation: self.relation.clone(),
            columns: Vec::new(),
            n_rows: self.n_rows,
        };

        let mut b = DataSet {
            relation: self.relation.clone(),
            columns: Vec::new(),
            n_rows: self.n_rows,
        };

        for col in self.columns {
            if names.contains(col.name()) {
                b.columns.push(col);
            } else {
                a.columns.push(col);
            }
        }

        (a, b)
    }

    /// move given columns into a separate data set
    pub fn split_one(self, column: &str) -> (Self, Self) {
        let mut a = DataSet {
            relation: self.relation.clone(),
            columns: Vec::new(),
            n_rows: self.n_rows,
        };

        let mut b = DataSet {
            relation: self.relation.clone(),
            columns: Vec::new(),
            n_rows: self.n_rows,
        };

        for col in self.columns {
            if column == col.name() {
                b.columns.push(col);
            } else {
                a.columns.push(col);
            }
        }

        (a, b)
    }

    pub fn to_array<T>(&self) -> Result<Array<T>>
    where
        T: CastValue,
    {
        let mut columns = Vec::with_capacity(self.columns.len());
        let mut data = Vec::with_capacity(self.columns.len());

        for col in self.columns.iter() {
            let name = col.name().to_owned();
            let dtype = match col.data().get_type() {
                ColumnType::U8
                | ColumnType::U16
                | ColumnType::U32
                | ColumnType::U64
                | ColumnType::I8
                | ColumnType::I16
                | ColumnType::I32
                | ColumnType::I64
                | ColumnType::F64 => DType::Numeric,
                ColumnType::String => DType::String,
                ColumnType::Nominal { categories } => DType::Nominal(categories),
            };
            columns.push(Attribute { name, dtype });
        }

        for i in 0..self.n_rows() {
            for col in self.columns.iter() {
                data.push(T::from_value(col.item(i))?);
            }
        }

        Ok(Array::new(columns, data))
    }
}
