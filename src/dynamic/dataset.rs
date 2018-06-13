use error::Result;
use parser::Parser;

use super::FlatIter;
use super::column::Column;
use super::value::Value;

/// A dynamically typed representation of an ARFF data set
#[derive(Debug, PartialEq)]
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
}
