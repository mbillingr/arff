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

/// The dynamically typed data of a column
#[derive(Debug, PartialEq)]
enum ColumnData {
    Invalid,

    U8 {
        values: Vec<Option<u8>>
    },

    U16 {
        values: Vec<Option<u16>>
    },

    U32 {
        values: Vec<Option<u32>>
    },

    U64 {
        values: Vec<Option<u64>>
    },

    I8 {
        values: Vec<Option<i8>>
    },

    I16 {
        values: Vec<Option<i16>>
    },

    I32 {
        values: Vec<Option<i32>>
    },

    I64 {
        values: Vec<Option<i64>>
    },

    F64 {
        values: Vec<Option<f64>>
    },

    String {
        values: Vec<Option<String>>
    },

    Nominal {
        categories: Vec<String>,
        values: Vec<Option<usize>>,
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
    F64,
    String,
    Nominal {
        categories: Vec<String>,
    },
}

macro_rules! def_columndata_into {
    ($name:ident, $variant:ident, $typ:ident) => (
        fn $name(self) -> Self {
            let values = match self {
                ColumnData::U8{values} => values.into_iter().map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::U16{values} => values.into_iter().map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::U32{values} => values.into_iter().map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::U64{values} => values.into_iter().map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::I8{values} => values.into_iter().map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::I16{values} => values.into_iter().map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::I32{values} => values.into_iter().map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::I64{values} => values.into_iter().map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::F64{values} => values.into_iter().map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::String{values} => values.into_iter().map(|x| x.map(|v| v.parse().unwrap())).collect(),
                ColumnData::Nominal{values, ..} => values.into_iter().map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::Invalid => panic!("invalid column state"),
            };
            ColumnData::$variant{values}
        }
    )
}

macro_rules! def_columndata_pushed {
    ($name:ident, $variant:ident, $typ:ident) => (
        fn $name(mut self, v: Option<$typ>) -> Self {
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
            ColumnData::F64{..} => ColumnType::F64,
            ColumnData::String{..} => ColumnType::String,
            ColumnData::Nominal{categories, ..} => ColumnType::Nominal {categories: categories.clone()},
            &ColumnData::Invalid => panic!("invalid column state")
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            ColumnData::U8{values} => values.is_empty(),
            ColumnData::U16{values} => values.is_empty(),
            ColumnData::U32{values} => values.is_empty(),
            ColumnData::U64{values} => values.is_empty(),
            ColumnData::I8{values} => values.is_empty(),
            ColumnData::I16{values} => values.is_empty(),
            ColumnData::I32{values} => values.is_empty(),
            ColumnData::I64{values} => values.is_empty(),
            ColumnData::F64{values} => values.is_empty(),
            ColumnData::String{values} => values.is_empty(),
            ColumnData::Nominal{values, ..} => values.is_empty(),
            &ColumnData::Invalid => panic!("invalid column state")
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
    def_columndata_pushed!(pushed_f64, F64, f64);

    def_columndata_into!(into_u16, U16, u16);
    def_columndata_into!(into_u32, U32, u32);
    def_columndata_into!(into_u64, U64, u64);
    def_columndata_into!(into_i8, I8, i8);
    def_columndata_into!(into_i16, I16, i16);
    def_columndata_into!(into_i32, I32, i32);
    def_columndata_into!(into_i64, I64, i64);
    def_columndata_into!(into_f64, F64, f64);
}

/// a dynamically typed ARFF value
#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Missing,
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

impl<'a> From<Option<u8>> for Value<'a> { fn from(x: Option<u8>) -> Self { x.map_or(Value::Missing, |v| Value::U8(v)) } }
impl<'a> From<Option<u16>> for Value<'a> { fn from(x: Option<u16>) -> Self { x.map_or(Value::Missing, |v| Value::U16(v)) } }
impl<'a> From<Option<u32>> for Value<'a> { fn from(x: Option<u32>) -> Self { x.map_or(Value::Missing, |v| Value::U32(v)) } }
impl<'a> From<Option<u64>> for Value<'a> { fn from(x: Option<u64>) -> Self { x.map_or(Value::Missing, |v| Value::U64(v)) } }
impl<'a> From<Option<i8>> for Value<'a> { fn from(x: Option<i8>) -> Self { x.map_or(Value::Missing, |v| Value::I8(v)) } }
impl<'a> From<Option<i16>> for Value<'a> { fn from(x: Option<i16>) -> Self { x.map_or(Value::Missing, |v| Value::I16(v)) } }
impl<'a> From<Option<i32>> for Value<'a> { fn from(x: Option<i32>) -> Self { x.map_or(Value::Missing, |v| Value::I32(v)) } }
impl<'a> From<Option<i64>> for Value<'a> { fn from(x: Option<i64>) -> Self { x.map_or(Value::Missing, |v| Value::I64(v)) } }
impl<'a> From<Option<f32>> for Value<'a> { fn from(x: Option<f32>) -> Self { x.map_or(Value::Missing, |v| Value::F64(v as f64)) } }
impl<'a> From<Option<f64>> for Value<'a> { fn from(x: Option<f64>) -> Self { x.map_or(Value::Missing, |v| Value::F64(v)) } }
impl<'a> From<Option<&'a str>> for Value<'a> { fn from(s: Option<&'a str>) -> Self { s.map_or(Value::Missing, |v| Value::String(v)) } }

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
            ColumnData::String {ref mut values} => {
                if parser.parse_is_missing() {
                    values.push(None);
                } else {
                    values.push(Some(parser.parse_string()?));
                }
            },
            ColumnData::Nominal {ref mut values, ref categories} => {
                let pos = parser.pos();
                if parser.parse_is_missing() {
                    values.push(None);
                }
                let value = parser.parse_unquoted_string()?;
                match categories
                    .iter()
                    .position(|item| item == &value)
                    {
                        Some(i) => values.push(Some(i)),
                        None => return Err(Error::WrongNominalValue(pos, value)),
                    }
            }
            _ => self.push(parser.parse_dynamic()?)
            //ColumnData::Date {..} => unimplemented!(),
        }
        Ok(())
    }

    fn push(&mut self, value: Option<DynamicValue>) {

        let data = std::mem::replace(&mut self.data, ColumnData::Invalid);

        match (data.get_type(), value) {
            (ColumnType::U8, None) => self.data = data.pushed_u8(None),
            (ColumnType::U8, Some(DynamicValue::U8(v))) => self.data = data.pushed_u8(Some(v)),
            (ColumnType::U8, Some(DynamicValue::U16(v))) => self.data = data.into_u16().pushed_u16(Some(v)),
            (ColumnType::U8, Some(DynamicValue::U32(v))) => self.data = data.into_u32().pushed_u32(Some(v)),
            (ColumnType::U8, Some(DynamicValue::U64(v))) => self.data = data.into_u64().pushed_u64(Some(v)),
            (ColumnType::U8, Some(DynamicValue::I8(v))) => {
                if data.is_empty() {
                    self.data = data.into_i8().pushed_i8(Some(v))
                } else {
                    self.data = data.into_i16().pushed_i16(Some(v as i16))
                }
            },
            (ColumnType::U8, Some(DynamicValue::I16(v))) => self.data = data.into_i16().pushed_i16(Some(v)),
            (ColumnType::U8, Some(DynamicValue::I32(v))) => self.data = data.into_i32().pushed_i32(Some(v)),
            (ColumnType::U8, Some(DynamicValue::I64(v))) => self.data = data.into_i64().pushed_i64(Some(v)),
            (ColumnType::U8, Some(DynamicValue::F64(v))) => self.data = data.into_f64().pushed_f64(Some(v)),

            (ColumnType::U16, None) => self.data = data.pushed_u16(None),
            (ColumnType::U16, Some(DynamicValue::U8(v))) => self.data = data.pushed_u16(Some(v as u16)),
            (ColumnType::U16, Some(DynamicValue::U16(v))) => self.data = data.pushed_u16(Some(v)),
            (ColumnType::U16, Some(DynamicValue::U32(v))) => self.data = data.into_u32().pushed_u32(Some(v)),
            (ColumnType::U16, Some(DynamicValue::U64(v))) => self.data = data.into_u64().pushed_u64(Some(v)),
            (ColumnType::U16, Some(DynamicValue::I8(v))) => self.data = data.into_i32().pushed_i32(Some(v as i32)),
            (ColumnType::U16, Some(DynamicValue::I16(v))) => self.data = data.into_i32().pushed_i32(Some(v as i32)),
            (ColumnType::U16, Some(DynamicValue::I32(v))) => self.data = data.into_i32().pushed_i32(Some(v)),
            (ColumnType::U16, Some(DynamicValue::I64(v))) => self.data = data.into_i64().pushed_i64(Some(v)),
            (ColumnType::U16, Some(DynamicValue::F64(v))) => self.data = data.into_f64().pushed_f64(Some(v)),

            (ColumnType::U32, None) => self.data = data.pushed_u32(None),
            (ColumnType::U32, Some(DynamicValue::U8(v))) => self.data = data.pushed_u32(Some(v as u32)),
            (ColumnType::U32, Some(DynamicValue::U16(v))) => self.data = data.pushed_u32(Some(v as u32)),
            (ColumnType::U32, Some(DynamicValue::U32(v))) => self.data = data.pushed_u32(Some(v)),
            (ColumnType::U32, Some(DynamicValue::U64(v))) => self.data = data.into_u64().pushed_u64(Some(v)),
            (ColumnType::U32, Some(DynamicValue::I8(v))) => self.data = data.into_i64().pushed_i64(Some(v as i64)),
            (ColumnType::U32, Some(DynamicValue::I16(v))) => self.data = data.into_i64().pushed_i64(Some(v as i64)),
            (ColumnType::U32, Some(DynamicValue::I32(v))) => self.data = data.into_i64().pushed_i64(Some(v as i64)),
            (ColumnType::U32, Some(DynamicValue::I64(v))) => self.data = data.into_i64().pushed_i64(Some(v)),
            (ColumnType::U32, Some(DynamicValue::F64(v))) => self.data = data.into_f64().pushed_f64(Some(v)),

            (ColumnType::U64, None) => self.data = data.pushed_u64(None),
            (ColumnType::U64, Some(DynamicValue::U8(v))) => self.data = data.pushed_u64(Some(v as u64)),
            (ColumnType::U64, Some(DynamicValue::U16(v))) => self.data = data.pushed_u64(Some(v as u64)),
            (ColumnType::U64, Some(DynamicValue::U32(v))) => self.data = data.pushed_u64(Some(v as u64)),
            (ColumnType::U64, Some(DynamicValue::U64(v))) => self.data = data.pushed_u64(Some(v)),
            (ColumnType::U64, Some(DynamicValue::I8(v))) => self.data = data.into_f64().pushed_f64(Some(v as f64)),
            (ColumnType::U64, Some(DynamicValue::I16(v))) => self.data = data.into_f64().pushed_f64(Some(v as f64)),
            (ColumnType::U64, Some(DynamicValue::I32(v))) => self.data = data.into_f64().pushed_f64(Some(v as f64)),
            (ColumnType::U64, Some(DynamicValue::I64(v))) => self.data = data.into_f64().pushed_f64(Some(v as f64)),
            (ColumnType::U64, Some(DynamicValue::F64(v))) => self.data = data.into_f64().pushed_f64(Some(v)),

            (ColumnType::I8, None) => self.data = data.pushed_i8(None),
            (ColumnType::I8, Some(DynamicValue::U8(v))) => self.data = data.into_i16().pushed_i16(Some(v as i16)),
            (ColumnType::I8, Some(DynamicValue::U16(v))) => self.data = data.into_i32().pushed_i32(Some(v as i32)),
            (ColumnType::I8, Some(DynamicValue::U32(v))) => self.data = data.into_i64().pushed_i64(Some(v as i64)),
            (ColumnType::I8, Some(DynamicValue::U64(v))) => self.data = data.into_f64().pushed_f64(Some(v as f64)),
            (ColumnType::I8, Some(DynamicValue::I8(v))) => self.data = data.pushed_i8(Some(v)),
            (ColumnType::I8, Some(DynamicValue::I16(v))) => self.data = data.into_i16().pushed_i16(Some(v)),
            (ColumnType::I8, Some(DynamicValue::I32(v))) => self.data = data.into_i32().pushed_i32(Some(v)),
            (ColumnType::I8, Some(DynamicValue::I64(v))) => self.data = data.into_i64().pushed_i64(Some(v)),
            (ColumnType::I8, Some(DynamicValue::F64(v))) => self.data = data.into_f64().pushed_f64(Some(v)),

            (ColumnType::I16, None) => self.data = data.pushed_i16(None),
            (ColumnType::I16, Some(DynamicValue::U8(v))) => self.data = data.pushed_i16(Some(v as i16)),
            (ColumnType::I16, Some(DynamicValue::U16(v))) => self.data = data.into_i32().pushed_i32(Some(v as i32)),
            (ColumnType::I16, Some(DynamicValue::U32(v))) => self.data = data.into_i64().pushed_i64(Some(v as i64)),
            (ColumnType::I16, Some(DynamicValue::U64(v))) => self.data = data.into_f64().pushed_f64(Some(v as f64)),
            (ColumnType::I16, Some(DynamicValue::I8(v))) => self.data = data.pushed_i16(Some(v as i16)),
            (ColumnType::I16, Some(DynamicValue::I16(v))) => self.data = data.pushed_i16(Some(v)),
            (ColumnType::I16, Some(DynamicValue::I32(v))) => self.data = data.into_i32().pushed_i32(Some(v)),
            (ColumnType::I16, Some(DynamicValue::I64(v))) => self.data = data.into_i64().pushed_i64(Some(v)),
            (ColumnType::I16, Some(DynamicValue::F64(v))) => self.data = data.into_f64().pushed_f64(Some(v)),

            (ColumnType::I32, None) => self.data = data.pushed_i32(None),
            (ColumnType::I32, Some(DynamicValue::U8(v))) => self.data = data.pushed_i32(Some(v as i32)),
            (ColumnType::I32, Some(DynamicValue::U16(v))) => self.data = data.pushed_i32(Some(v as i32)),
            (ColumnType::I32, Some(DynamicValue::U32(v))) => self.data = data.into_i64().pushed_i64(Some(v as i64)),
            (ColumnType::I32, Some(DynamicValue::U64(v))) => self.data = data.into_f64().pushed_f64(Some(v as f64)),
            (ColumnType::I32, Some(DynamicValue::I8(v))) => self.data = data.pushed_i32(Some(v as i32)),
            (ColumnType::I32, Some(DynamicValue::I16(v))) => self.data = data.pushed_i32(Some(v as i32)),
            (ColumnType::I32, Some(DynamicValue::I32(v))) => self.data = data.pushed_i32(Some(v)),
            (ColumnType::I32, Some(DynamicValue::I64(v))) => self.data = data.into_i64().pushed_i64(Some(v)),
            (ColumnType::I32, Some(DynamicValue::F64(v))) => self.data = data.into_f64().pushed_f64(Some(v)),

            (ColumnType::I64, None) => self.data = data.pushed_i64(None),
            (ColumnType::I64, Some(DynamicValue::U8(v))) => self.data = data.pushed_i64(Some(v as i64)),
            (ColumnType::I64, Some(DynamicValue::U16(v))) => self.data = data.pushed_i64(Some(v as i64)),
            (ColumnType::I64, Some(DynamicValue::U32(v))) => self.data = data.pushed_i64(Some(v as i64)),
            (ColumnType::I64, Some(DynamicValue::U64(v))) => self.data = data.into_f64().pushed_f64(Some(v as f64)),
            (ColumnType::I64, Some(DynamicValue::I8(v))) => self.data = data.pushed_i64(Some(v as i64)),
            (ColumnType::I64, Some(DynamicValue::I16(v))) => self.data = data.pushed_i64(Some(v as i64)),
            (ColumnType::I64, Some(DynamicValue::I32(v))) => self.data = data.pushed_i64(Some(v as i64)),
            (ColumnType::I64, Some(DynamicValue::I64(v))) => self.data = data.pushed_i64(Some(v)),
            (ColumnType::I64, Some(DynamicValue::F64(v))) => self.data = data.into_f64().pushed_f64(Some(v)),

            (ColumnType::F64, None) => self.data = data.pushed_f64(None),
            (ColumnType::F64, Some(DynamicValue::U8(v))) => self.data = data.pushed_f64(Some(v as f64)),
            (ColumnType::F64, Some(DynamicValue::U16(v))) => self.data = data.pushed_f64(Some(v as f64)),
            (ColumnType::F64, Some(DynamicValue::U32(v))) => self.data = data.pushed_f64(Some(v as f64)),
            (ColumnType::F64, Some(DynamicValue::U64(v))) => self.data = data.pushed_f64(Some(v as f64)),
            (ColumnType::F64, Some(DynamicValue::I8(v))) => self.data = data.pushed_f64(Some(v as f64)),
            (ColumnType::F64, Some(DynamicValue::I16(v))) => self.data = data.pushed_f64(Some(v as f64)),
            (ColumnType::F64, Some(DynamicValue::I32(v))) => self.data = data.pushed_f64(Some(v as f64)),
            (ColumnType::F64, Some(DynamicValue::I64(v))) => self.data = data.pushed_f64(Some(v as f64)),
            (ColumnType::F64, Some(DynamicValue::F64(v))) => self.data = data.pushed_f64(Some(v)),

            (ColumnType::String, _) => unreachable!(),
            (ColumnType::Nominal{..}, _) => unreachable!(),
            (_, Some(DynamicValue::String(_))) => unimplemented!()
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
            ColumnData::F64 {ref values} => values[idx].into(),
            ColumnData::String {ref values} => values[idx].as_ref().map(|x| x.as_str()).into(),
            ColumnData::Nominal {ref categories, ref values} => {
                match values[idx] {
                    Some(v) => Value::Nominal(v, &categories),
                    None => Value::Missing,
                }
            },
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
4, ?, '7', red
";

    let dset: DataSet = DataSet::from_str(input).unwrap();

    assert_eq!(dset, DataSet {
        relation: "Test data".to_owned(),
        n_rows: 2,
        columns: vec![
            Column { name: "int".to_owned(), data: ColumnData::U8 {values: vec![Some(1), Some(4)]}},
            Column { name: "float".to_owned(), data: ColumnData::F64 {values: vec![Some(2.0), None]}},
            Column { name: "text".to_owned(), data: ColumnData::String {
                values: vec![Some("three".to_owned()), Some("7".to_owned())]
            }},
            Column { name: "color".to_owned(), data: ColumnData::Nominal {
                values: vec![Some(2), Some(0)], categories: vec!["red".to_owned(),
                                                                 "green".to_owned(),
                                                                 "blue".to_owned()]
            }}
        ]
    });
}
