
use super::{Error, Result};
use parser::{self, Parser};


/// A dynamically typed representation of an ARFF data set
#[derive(Debug, PartialEq)]
pub struct DataSet<T=f64> {
    relation: String,
    columns: Vec<Column<T>>,
    n_rows: usize,
}

/// A dynamically typed column of an ARFF data set
#[derive(Debug, PartialEq)]
pub struct Column<T> {
    name: String,
    data: ColumnData<T>,
}

#[derive(Debug, PartialEq)]
enum ColumnData<T> {
    Numeric {
        values: Vec<T>
    },

    String {
        values: Vec<String>
    },

    Nominal {
        categories: Vec<String>,
        values: Vec<usize>,
    },

    /*Date {
        format: String,
        values: Vec<String>,
    },*/
}

/// a dynamically typed ARFF value
#[derive(Debug, PartialEq)]
pub enum Value<'a, T=f64> {
    Numeric(T),
    String(&'a str),
    Nominal(usize, &'a Vec<String>),
}

impl<T: Numeric> DataSet<T> {

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
                None => {},
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
            n_rows
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
    pub fn col_names<'a>(&'a self) -> Box<'a + Iterator<Item=&'a str>> {
        let iter = self.columns.iter().map(|col| col.name.as_ref());
        Box::new(iter)
    }

    /// get data row by index
    pub fn row(&self, idx: usize) -> Vec<Value<T>> {
        let mut row = Vec::with_capacity(self.columns.len());
        for col in self.columns.iter() {
            match col.data {
                ColumnData::Numeric {ref values} => row.push(Value::Numeric(values[idx])),
                ColumnData::String {ref values} => row.push(Value::String(&values[idx])),
                ColumnData::Nominal {ref categories, ref values} => row.push(Value::Nominal(values[idx], categories)),
            }
        }
        row
    }

    /// get data column by index
    pub fn col(&self, idx: usize) -> &Column<T> {
        &self.columns[idx]
    }

    /// get column by name
    ///
    /// panics if there is no such column.
    pub fn col_by_name(&self, col: &str) -> &Column<T> {
        for c in &self.columns {
            if c.name == col {
                return c
            }
        }
        panic!("unknown column: {}", col);
    }

    /// get item by row/column index
    pub fn item(&self, row: usize, col: usize) -> Value<T> {
        self.col(col).item(row)
    }

    /// get item by row index and column name
    ///
    /// panics if there is no such column.
    pub fn item_by_name(&self, row: usize, col: &str) -> Value<T> {
        self.col_by_name(col).item(row)
    }
}

impl<T: Numeric> Column<T> {
    fn from_attr(attr: parser::Attribute) -> Result<Self> {
        Ok(Column {
            name: attr.name,
            data: ColumnData::new_from_string(attr.dtype)?,
        })
    }

    fn parse_value(&mut self, parser: &mut Parser) -> Result<()> {
        match self.data {
            ColumnData::Numeric {ref mut values} => values.push(T::parse(parser)?),
            ColumnData::String {ref mut values} => values.push(parser.parse_string()?),
            ColumnData::Nominal {ref mut values, ref categories} => {
                let value = parser.parse_unquoted_string()?;
                match categories
                    .iter()
                    .position(|item| item == &value)
                    {
                        Some(i) => values.push(i),
                        None => return Err(Error::WrongNominalValue(value)),
                    }
            }
            //ColumnData::Date {..} => unimplemented!(),
        }
        Ok(())
    }

    /// get item by index
    pub fn item(&self, idx: usize) -> Value<T> {
        match self.data {
            ColumnData::Numeric {ref values} => Value::Numeric(values[idx]),
            ColumnData::String {ref values} => Value::String(&values[idx]),
            ColumnData::Nominal {ref categories, ref values} => Value::Nominal(values[idx], categories),
        }
    }
}

impl<T> ColumnData<T> {
    fn new_from_string(mut s: String) -> Result<Self>{
        if s.starts_with('{') && s.ends_with('}') {
            return ColumnData::new_nominal(&s[1..s.len()-1])
        }

        s.make_ascii_uppercase();

        match &s[..4] {
            "NUME" => Ok(ColumnData::new_numeric()),
            "STRI" => Ok(ColumnData::new_string()),
            "DATE" => ColumnData::new_date(&s[4..]),
            _ => Err(Error::InvalidColumnType)
        }
    }

    fn new_numeric() -> Self {
        ColumnData::Numeric {
            values: Vec::new()
        }
    }

    fn new_string() -> Self {
        ColumnData::String {
            values: Vec::new()
        }
    }

    fn new_date(_fmt: &str) -> Result<Self> {
        unimplemented!()
    }

    fn new_nominal(s: &str) -> Result<Self> {
        let categories = s.split(',').map(|s| s.trim().to_owned()).collect();
        Ok(ColumnData::Nominal {
            categories,
            values: Vec::new(),
        })
    }
}

/// trait that marks primitives that "numeric" columns can represent
pub trait Numeric: Sized + Copy + Clone {
    fn parse(parser: &mut Parser) -> Result<Self>;
}

impl Numeric for i8 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_i8()
    }
}

impl Numeric for i16 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_i16()
    }
}

impl Numeric for i32 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_i32()
    }
}

impl Numeric for i64 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_i64()
    }
}

impl Numeric for u8 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_u8()
    }
}

impl Numeric for u16 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_u16()
    }
}

impl Numeric for u32 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_u32()
    }
}

impl Numeric for u64 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_u64()
    }
}

impl Numeric for f32 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_float().map(|f| f as f32)
    }
}

impl Numeric for f64 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_float()
    }
}


#[test]
fn dynamic_loader() {
    let input = "\
@Relation 'Test data'
@Attribute int NUMERIC
@Attribute float NUMERIC
@Attribute text String
@Attribute color {red, green, blue}
@Data
1, 2.0, 'three', blue
4, 5.6, '7', red
";

    let dset: DataSet = DataSet::from_str(input).unwrap();

    assert_eq!(dset, DataSet {
        relation: "Test data".to_owned(),
        n_rows: 2,
        columns: vec![
            Column { name: "int".to_owned(), data: ColumnData::Numeric {values: vec![1.0, 4.0]}},
            Column { name: "float".to_owned(), data: ColumnData::Numeric {values: vec![2.0, 5.6]}},
            Column { name: "text".to_owned(), data: ColumnData::String {
                values: vec!["three".to_owned(), "7".to_owned()]
            }},
            Column { name: "color".to_owned(), data: ColumnData::Nominal {
                values: vec![2, 0], categories: vec!["red".to_owned(),
                                                     "green".to_owned(),
                                                     "blue".to_owned()]
            }}
        ]
    });
}
