use std;
use super::{Error, Result};
use parser::{self, DType, DynamicValue, Parser};


/// A dynamically typed representation of an ARFF data set
#[derive(Debug, PartialEq)]
pub struct DataSet {
    relation: String,
    columns: Vec<Column>,
    n_rows: usize,
}

/// A dynamically typed column of an ARFF data set
#[derive(Debug, PartialEq)]
pub struct Column {
    name: String,
    data: ColumnData,
}

#[derive(Debug, PartialEq)]
enum ColumnData {
    Invalid,

    U8 {
        values: Vec<u8>
    },

    U16 {
        values: Vec<u16>
    },

    U32 {
        values: Vec<u32>
    },

    U64 {
        values: Vec<u64>
    },

    I8 {
        values: Vec<i8>
    },

    I16 {
        values: Vec<i16>
    },

    I32 {
        values: Vec<i32>
    },

    I64 {
        values: Vec<i64>
    },

    F32 {
        values: Vec<f32>
    },

    F64 {
        values: Vec<f64>
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

#[derive(Debug, PartialEq)]
enum ColumnType {
    U8, U16, U32, U64,
    I8, I16, I32, I64,
    F32, F64,
    String,
    Nominal {
        categories: Vec<String>,
    },
}

macro_rules! def_columndata_into {
    ($name:ident, $variant:ident, $typ:ident) => (
        fn $name(self) -> Self {
            let values = match self {
                ColumnData::U8{values} => values.into_iter().map(|x| x as $typ).collect(),
                ColumnData::U16{values} => values.into_iter().map(|x| x as $typ).collect(),
                ColumnData::U32{values} => values.into_iter().map(|x| x as $typ).collect(),
                ColumnData::U64{values} => values.into_iter().map(|x| x as $typ).collect(),
                ColumnData::I8{values} => values.into_iter().map(|x| x as $typ).collect(),
                ColumnData::I16{values} => values.into_iter().map(|x| x as $typ).collect(),
                ColumnData::I32{values} => values.into_iter().map(|x| x as $typ).collect(),
                ColumnData::I64{values} => values.into_iter().map(|x| x as $typ).collect(),
                ColumnData::F32{values} => values.into_iter().map(|x| x as $typ).collect(),
                ColumnData::F64{values} => values.into_iter().map(|x| x as $typ).collect(),
                ColumnData::String{values} => values.into_iter().map(|x| x.parse().unwrap()).collect(),
                ColumnData::Nominal{values, ..} => values.into_iter().map(|x| x as $typ).collect(),
                ColumnData::Invalid => panic!("invalid column state"),
            };
            ColumnData::$variant{values}
        }
    )
}

macro_rules! def_columndata_pushed {
    ($name:ident, $variant:ident, $typ:ident) => (
        fn $name(mut self, v: $typ) -> Self {
            match self {
                ColumnData::$variant{ref mut values} => values.push(v),
                ColumnData::Invalid => panic!("invalid column state"),
                _ => panic!("unexpected type: {:?} (expected {:?}", self, ColumnType::$variant)
            };
            self
        }
    )
}

impl ColumnData {
    fn get_type(&self) -> ColumnType {
        match self {
            ColumnData::U8{..} => ColumnType::U8,
            ColumnData::U16{..} => ColumnType::U16,
            ColumnData::U32{..} => ColumnType::U32,
            ColumnData::U64{..} => ColumnType::U64,
            ColumnData::I8{..} => ColumnType::I8,
            ColumnData::I16{..} => ColumnType::I16,
            ColumnData::I32{..} => ColumnType::I32,
            ColumnData::I64{..} => ColumnType::I64,
            ColumnData::F32{..} => ColumnType::F32,
            ColumnData::F64{..} => ColumnType::F64,
            ColumnData::String{..} => ColumnType::String,
            ColumnData::Nominal{categories, ..} => ColumnType::Nominal {categories: categories.clone()},
            ColumnData::Invalid => panic!("invalid column state")
        }
    }

    def_columndata_pushed!(pushed_u8, U8, u8);
    def_columndata_pushed!(pushed_u16, U16, u16);
    def_columndata_pushed!(pushed_u32, U32, u32);
    def_columndata_pushed!(pushed_u64, U64, u64);
    def_columndata_pushed!(pushed_i8, I8, i8);
    def_columndata_pushed!(pushed_i16, I16, i16);
    def_columndata_pushed!(pushed_i32, I32, i32);
    def_columndata_pushed!(pushed_i64, I64, i64);
    def_columndata_pushed!(pushed_f32, F32, f32);
    def_columndata_pushed!(pushed_f64, F64, f64);

    def_columndata_into!(into_u8, U8, u8);
    def_columndata_into!(into_u16, U16, u16);
    def_columndata_into!(into_u32, U32, u32);
    def_columndata_into!(into_u64, U64, u64);
    def_columndata_into!(into_i8, I8, i8);
    def_columndata_into!(into_i16, I16, i16);
    def_columndata_into!(into_i32, I32, i32);
    def_columndata_into!(into_i64, I64, i64);
    def_columndata_into!(into_f32, F32, f32);
    def_columndata_into!(into_f64, F64, f64);
}

/// a dynamically typed ARFF value
#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    U8(u8), U16(u16), U32(u32), U64(u64),
    I8(i8), I16(i16), I32(i32), I64(i64),
    F64(f64),
    String(&'a str),
    Nominal(usize, &'a Vec<String>),
}

impl<'a> From<u8> for Value<'a> { fn from(x: u8) -> Self { Value::U8(x) } }
impl<'a> From<u16> for Value<'a> { fn from(x: u16) -> Self { Value::U16(x) } }
impl<'a> From<u32> for Value<'a> { fn from(x: u32) -> Self { Value::U32(x) } }
impl<'a> From<u64> for Value<'a> { fn from(x: u64) -> Self { Value::U64(x) } }
impl<'a> From<i8> for Value<'a> { fn from(x: i8) -> Self { Value::I8(x) } }
impl<'a> From<i16> for Value<'a> { fn from(x: i16) -> Self { Value::I16(x) } }
impl<'a> From<i32> for Value<'a> { fn from(x: i32) -> Self { Value::I32(x) } }
impl<'a> From<i64> for Value<'a> { fn from(x: i64) -> Self { Value::I64(x) } }
impl<'a> From<f32> for Value<'a> { fn from(x: f32) -> Self { Value::F64(x as f64) } }
impl<'a> From<f64> for Value<'a> { fn from(x: f64) -> Self { Value::F64(x) } }
impl<'a> From<&'a str> for Value<'a> { fn from(s: &'a str) -> Self { Value::String(s) } }

impl DataSet {

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
    pub fn row<T>(&self, idx: usize) -> Vec<Value> {
        self.columns
            .iter()
            .map(|c| c.item(idx))
            .collect()
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
            if c.name == col {
                return c
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
}

impl Column {
    fn from_attr(attr: parser::Attribute) -> Result<Self> {
        Ok(Column {
            name: attr.name,
            data: ColumnData::new_from_dtype(attr.dtype),
        })
    }

    fn parse_value(&mut self, parser: &mut Parser) -> Result<()> {
        match self.data {
            ColumnData::String {ref mut values} => values.push(parser.parse_string()?),
            ColumnData::Nominal {ref mut values, ref categories} => {
                let pos = parser.pos();
                let value = parser.parse_unquoted_string()?;
                match categories
                    .iter()
                    .position(|item| item == &value)
                    {
                        Some(i) => values.push(i),
                        None => return Err(Error::WrongNominalValue(pos, value)),
                    }
            }
            _ => self.push(parser.parse_dynamic()?)
            //ColumnData::Date {..} => unimplemented!(),
        }
        Ok(())
    }

    fn push(&mut self, value: DynamicValue) {

        let data = std::mem::replace(&mut self.data, ColumnData::Invalid);

        match (data.get_type(), value) {
            (ColumnType::U8, DynamicValue::U8(v)) => self.data = data.pushed_u8(v),
            (ColumnType::U8, DynamicValue::U16(v)) => self.data = data.into_u16().pushed_u16(v),
            (ColumnType::U8, DynamicValue::U32(v)) => self.data = data.into_u32().pushed_u32(v),
            (ColumnType::U8, DynamicValue::U64(v)) => self.data = data.into_u64().pushed_u64(v),
            (ColumnType::U8, DynamicValue::I8(v)) => self.data = data.into_i16().pushed_i16(v as i16),
            (ColumnType::U8, DynamicValue::I16(v)) => self.data = data.into_i16().pushed_i16(v),
            (ColumnType::U8, DynamicValue::I32(v)) => self.data = data.into_i32().pushed_i32(v),
            (ColumnType::U8, DynamicValue::I64(v)) => self.data = data.into_i64().pushed_i64(v),
            (ColumnType::U8, DynamicValue::F64(v)) => self.data = data.into_f64().pushed_f64(v),

            (ColumnType::U16, DynamicValue::U8(v)) => self.data = data.pushed_u16(v as u16),
            (ColumnType::U16, DynamicValue::U16(v)) => self.data = data.pushed_u16(v),
            (ColumnType::U16, DynamicValue::U32(v)) => self.data = data.into_u32().pushed_u32(v),
            (ColumnType::U16, DynamicValue::U64(v)) => self.data = data.into_u64().pushed_u64(v),
            (ColumnType::U16, DynamicValue::I8(v)) => self.data = data.into_i32().pushed_i32(v as i32),
            (ColumnType::U16, DynamicValue::I16(v)) => self.data = data.into_i32().pushed_i32(v as i32),
            (ColumnType::U16, DynamicValue::I32(v)) => self.data = data.into_i32().pushed_i32(v),
            (ColumnType::U16, DynamicValue::I64(v)) => self.data = data.into_i64().pushed_i64(v),
            (ColumnType::U16, DynamicValue::F64(v)) => self.data = data.into_f64().pushed_f64(v),

            (ColumnType::U32, DynamicValue::U8(v)) => self.data = data.pushed_u32(v as u32),
            (ColumnType::U32, DynamicValue::U16(v)) => self.data = data.pushed_u32(v as u32),
            (ColumnType::U32, DynamicValue::U32(v)) => self.data = data.pushed_u32(v),
            (ColumnType::U32, DynamicValue::U64(v)) => self.data = data.into_u64().pushed_u64(v),
            (ColumnType::U32, DynamicValue::I8(v)) => self.data = data.into_i64().pushed_i64(v as i64),
            (ColumnType::U32, DynamicValue::I16(v)) => self.data = data.into_i64().pushed_i64(v as i64),
            (ColumnType::U32, DynamicValue::I32(v)) => self.data = data.into_i64().pushed_i64(v as i64),
            (ColumnType::U32, DynamicValue::I64(v)) => self.data = data.into_i64().pushed_i64(v),
            (ColumnType::U32, DynamicValue::F64(v)) => self.data = data.into_f64().pushed_f64(v),

            (ColumnType::U64, DynamicValue::U8(v)) => self.data = data.pushed_u64(v as u64),
            (ColumnType::U64, DynamicValue::U16(v)) => self.data = data.pushed_u64(v as u64),
            (ColumnType::U64, DynamicValue::U32(v)) => self.data = data.pushed_u64(v as u64),
            (ColumnType::U64, DynamicValue::U64(v)) => self.data = data.pushed_u64(v),
            (ColumnType::U64, DynamicValue::I8(v)) => self.data = data.into_f64().pushed_f64(v as f64),
            (ColumnType::U64, DynamicValue::I16(v)) => self.data = data.into_f64().pushed_f64(v as f64),
            (ColumnType::U64, DynamicValue::I32(v)) => self.data = data.into_f64().pushed_f64(v as f64),
            (ColumnType::U64, DynamicValue::I64(v)) => self.data = data.into_f64().pushed_f64(v as f64),
            (ColumnType::U64, DynamicValue::F64(v)) => self.data = data.into_f64().pushed_f64(v),

            (ColumnType::I8, DynamicValue::U8(v)) => self.data = data.into_i16().pushed_i16(v as i16),
            (ColumnType::I8, DynamicValue::U16(v)) => self.data = data.into_i32().pushed_i32(v as i32),
            (ColumnType::I8, DynamicValue::U32(v)) => self.data = data.into_i64().pushed_i64(v as i64),
            (ColumnType::I8, DynamicValue::U64(v)) => self.data = data.into_f64().pushed_f64(v as f64),
            (ColumnType::I8, DynamicValue::I8(v)) => self.data = data.pushed_i8(v),
            (ColumnType::I8, DynamicValue::I16(v)) => self.data = data.into_i16().pushed_i16(v),
            (ColumnType::I8, DynamicValue::I32(v)) => self.data = data.into_i32().pushed_i32(v),
            (ColumnType::I8, DynamicValue::I64(v)) => self.data = data.into_i64().pushed_i64(v),
            (ColumnType::I8, DynamicValue::F64(v)) => self.data = data.into_f64().pushed_f64(v),

            (ColumnType::I16, DynamicValue::U8(v)) => self.data = data.pushed_i16(v as i16),
            (ColumnType::I16, DynamicValue::U16(v)) => self.data = data.into_i32().pushed_i32(v as i32),
            (ColumnType::I16, DynamicValue::U32(v)) => self.data = data.into_i64().pushed_i64(v as i64),
            (ColumnType::I16, DynamicValue::U64(v)) => self.data = data.into_f64().pushed_f64(v as f64),
            (ColumnType::I16, DynamicValue::I8(v)) => self.data = data.pushed_i16(v as i16),
            (ColumnType::I16, DynamicValue::I16(v)) => self.data = data.pushed_i16(v),
            (ColumnType::I16, DynamicValue::I32(v)) => self.data = data.into_i32().pushed_i32(v),
            (ColumnType::I16, DynamicValue::I64(v)) => self.data = data.into_i64().pushed_i64(v),
            (ColumnType::I16, DynamicValue::F64(v)) => self.data = data.into_f64().pushed_f64(v),

            (ColumnType::I32, DynamicValue::U8(v)) => self.data = data.pushed_i32(v as i32),
            (ColumnType::I32, DynamicValue::U16(v)) => self.data = data.pushed_i32(v as i32),
            (ColumnType::I32, DynamicValue::U32(v)) => self.data = data.into_i64().pushed_i64(v as i64),
            (ColumnType::I32, DynamicValue::U64(v)) => self.data = data.into_f64().pushed_f64(v as f64),
            (ColumnType::I32, DynamicValue::I8(v)) => self.data = data.pushed_i32(v as i32),
            (ColumnType::I32, DynamicValue::I16(v)) => self.data = data.pushed_i32(v as i32),
            (ColumnType::I32, DynamicValue::I32(v)) => self.data = data.pushed_i32(v),
            (ColumnType::I32, DynamicValue::I64(v)) => self.data = data.into_i64().pushed_i64(v),
            (ColumnType::I32, DynamicValue::F64(v)) => self.data = data.into_f64().pushed_f64(v),

            (ColumnType::I64, DynamicValue::U8(v)) => self.data = data.pushed_i64(v as i64),
            (ColumnType::I64, DynamicValue::U16(v)) => self.data = data.pushed_i64(v as i64),
            (ColumnType::I64, DynamicValue::U32(v)) => self.data = data.pushed_i64(v as i64),
            (ColumnType::I64, DynamicValue::U64(v)) => self.data = data.into_f64().pushed_f64(v as f64),
            (ColumnType::I64, DynamicValue::I8(v)) => self.data = data.pushed_i64(v as i64),
            (ColumnType::I64, DynamicValue::I16(v)) => self.data = data.pushed_i64(v as i64),
            (ColumnType::I64, DynamicValue::I32(v)) => self.data = data.pushed_i64(v as i64),
            (ColumnType::I64, DynamicValue::I64(v)) => self.data = data.pushed_i64(v),
            (ColumnType::I64, DynamicValue::F64(v)) => self.data = data.into_f64().pushed_f64(v),

            (ColumnType::F32, DynamicValue::U8(v)) => self.data = data.pushed_f32(v as f32),
            (ColumnType::F32, DynamicValue::U16(v)) => self.data = data.pushed_f32(v as f32),
            (ColumnType::F32, DynamicValue::U32(v)) => self.data = data.into_f64().pushed_f64(v as f64),
            (ColumnType::F32, DynamicValue::U64(v)) => self.data = data.into_f64().pushed_f64(v as f64),
            (ColumnType::F32, DynamicValue::I8(v)) => self.data = data.pushed_f32(v as f32),
            (ColumnType::F32, DynamicValue::I16(v)) => self.data = data.pushed_f32(v as f32),
            (ColumnType::F32, DynamicValue::I32(v)) => self.data = data.into_f64().pushed_f64(v as f64),
            (ColumnType::F32, DynamicValue::I64(v)) => self.data = data.into_f64().pushed_f64(v as f64),
            (ColumnType::F32, DynamicValue::F64(v)) => self.data = data.into_f64().pushed_f64(v),

            (ColumnType::F64, DynamicValue::U8(v)) => self.data = data.pushed_f64(v as f64),
            (ColumnType::F64, DynamicValue::U16(v)) => self.data = data.pushed_f64(v as f64),
            (ColumnType::F64, DynamicValue::U32(v)) => self.data = data.pushed_f64(v as f64),
            (ColumnType::F64, DynamicValue::U64(v)) => self.data = data.pushed_f64(v as f64),
            (ColumnType::F64, DynamicValue::I8(v)) => self.data = data.pushed_f64(v as f64),
            (ColumnType::F64, DynamicValue::I16(v)) => self.data = data.pushed_f64(v as f64),
            (ColumnType::F64, DynamicValue::I32(v)) => self.data = data.pushed_f64(v as f64),
            (ColumnType::F64, DynamicValue::I64(v)) => self.data = data.pushed_f64(v as f64),
            (ColumnType::F64, DynamicValue::F64(v)) => self.data = data.pushed_f64(v),

            (ColumnType::String, _) => unreachable!(),
            (ColumnType::Nominal{..}, _) => unreachable!(),
            (_, DynamicValue::String(_)) => unimplemented!()
        }
    }

    /// get item by index
    pub fn item(&self, idx: usize) -> Value {
        match self.data {
            ColumnData::U8 {ref values} => values[idx].into(),
            ColumnData::U16 {ref values} => values[idx].into(),
            ColumnData::U32 {ref values} => values[idx].into(),
            ColumnData::U64 {ref values} => values[idx].into(),
            ColumnData::I8 {ref values} => values[idx].into(),
            ColumnData::I16 {ref values} => values[idx].into(),
            ColumnData::I32 {ref values} => values[idx].into(),
            ColumnData::I64 {ref values} => values[idx].into(),
            ColumnData::F32 {ref values} => values[idx].into(),
            ColumnData::F64 {ref values} => values[idx].into(),
            ColumnData::String {ref values} => values[idx].as_str().into(),
            ColumnData::Nominal {ref categories, ref values} => Value::Nominal(values[idx], categories),
            ColumnData::Invalid => panic!("invalid column state"),
        }
    }
}

impl ColumnData {
    fn new_from_dtype(dt: DType) -> Self {
        match dt {
            DType::Numeric => ColumnData::new_numeric(),
            DType::String => ColumnData::new_string(),
            DType::Nominal(names) => ColumnData::new_nominal(names),
        }
    }

    fn new_numeric() -> Self {
        ColumnData::U8 {
            values: Vec::new()
        }
    }

    fn new_string() -> Self {
        ColumnData::String {
            values: Vec::new()
        }
    }

    fn new_nominal(categories: Vec<String>) -> Self {
        ColumnData::Nominal {
            categories,
            values: Vec::new(),
        }
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
            Column { name: "int".to_owned(), data: ColumnData::U8 {values: vec![1, 4]}},
            Column { name: "float".to_owned(), data: ColumnData::F64 {values: vec![2.0, 5.6]}},
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
