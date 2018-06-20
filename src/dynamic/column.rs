use std;

use error::{Error, Result};
use parser::{self, DType, DynamicValue, Parser};

use super::value::Value;

/// A dynamically typed column of an ARFF data set
#[derive(Debug, PartialEq, Clone)]
pub struct Column {
    name: String,
    data: ColumnData,
}

/// The type of a column
#[derive(Debug, PartialEq)]
pub enum ColumnType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F64,
    String,
    Nominal { categories: Vec<String> },
}

/// The dynamically typed data of a column
#[derive(Debug, PartialEq, Clone)]
pub enum ColumnData {
    Invalid,

    U8 {
        values: Vec<Option<u8>>,
    },

    U16 {
        values: Vec<Option<u16>>,
    },

    U32 {
        values: Vec<Option<u32>>,
    },

    U64 {
        values: Vec<Option<u64>>,
    },

    I8 {
        values: Vec<Option<i8>>,
    },

    I16 {
        values: Vec<Option<i16>>,
    },

    I32 {
        values: Vec<Option<i32>>,
    },

    I64 {
        values: Vec<Option<i64>>,
    },

    F64 {
        values: Vec<Option<f64>>,
    },

    String {
        values: Vec<Option<String>>,
    },

    Nominal {
        categories: Vec<String>,
        values: Vec<Option<usize>>,
    },
}

impl Column {
    pub fn new(name: &str, data: ColumnData) -> Self {
        Column {
            name: name.to_owned(),
            data,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data(&self) -> &ColumnData {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub(crate) fn from_attr(attr: parser::Attribute) -> Result<Self> {
        Ok(Column {
            name: attr.name,
            data: ColumnData::new_from_dtype(attr.dtype),
        })
    }

    pub(crate) fn parse_value(&mut self, parser: &mut Parser) -> Result<()> {
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
                } else {
                    let value = parser.parse_unquoted_string()?;
                    match categories
                        .iter()
                        .position(|item| item == &value)
                        {
                            Some(i) => values.push(Some(i)),
                            None => return Err(Error::WrongNominalValue(pos, value)),
                        }
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
            (ColumnType::U8, Some(DynamicValue::U16(v))) => {
                self.data = data.into_u16().pushed_u16(Some(v))
            }
            (ColumnType::U8, Some(DynamicValue::U32(v))) => {
                self.data = data.into_u32().pushed_u32(Some(v))
            }
            (ColumnType::U8, Some(DynamicValue::U64(v))) => {
                self.data = data.into_u64().pushed_u64(Some(v))
            }
            (ColumnType::U8, Some(DynamicValue::I8(v))) => {
                if data.is_empty() {
                    self.data = data.into_i8().pushed_i8(Some(v))
                } else {
                    self.data = data.into_i16().pushed_i16(Some(v as i16))
                }
            }
            (ColumnType::U8, Some(DynamicValue::I16(v))) => {
                self.data = data.into_i16().pushed_i16(Some(v))
            }
            (ColumnType::U8, Some(DynamicValue::I32(v))) => {
                self.data = data.into_i32().pushed_i32(Some(v))
            }
            (ColumnType::U8, Some(DynamicValue::I64(v))) => {
                self.data = data.into_i64().pushed_i64(Some(v))
            }
            (ColumnType::U8, Some(DynamicValue::F64(v))) => {
                self.data = data.into_f64().pushed_f64(Some(v))
            }

            (ColumnType::U16, None) => self.data = data.pushed_u16(None),
            (ColumnType::U16, Some(DynamicValue::U8(v))) => {
                self.data = data.pushed_u16(Some(v as u16))
            }
            (ColumnType::U16, Some(DynamicValue::U16(v))) => self.data = data.pushed_u16(Some(v)),
            (ColumnType::U16, Some(DynamicValue::U32(v))) => {
                self.data = data.into_u32().pushed_u32(Some(v))
            }
            (ColumnType::U16, Some(DynamicValue::U64(v))) => {
                self.data = data.into_u64().pushed_u64(Some(v))
            }
            (ColumnType::U16, Some(DynamicValue::I8(v))) => {
                self.data = data.into_i32().pushed_i32(Some(v as i32))
            }
            (ColumnType::U16, Some(DynamicValue::I16(v))) => {
                self.data = data.into_i32().pushed_i32(Some(v as i32))
            }
            (ColumnType::U16, Some(DynamicValue::I32(v))) => {
                self.data = data.into_i32().pushed_i32(Some(v))
            }
            (ColumnType::U16, Some(DynamicValue::I64(v))) => {
                self.data = data.into_i64().pushed_i64(Some(v))
            }
            (ColumnType::U16, Some(DynamicValue::F64(v))) => {
                self.data = data.into_f64().pushed_f64(Some(v))
            }

            (ColumnType::U32, None) => self.data = data.pushed_u32(None),
            (ColumnType::U32, Some(DynamicValue::U8(v))) => {
                self.data = data.pushed_u32(Some(v as u32))
            }
            (ColumnType::U32, Some(DynamicValue::U16(v))) => {
                self.data = data.pushed_u32(Some(v as u32))
            }
            (ColumnType::U32, Some(DynamicValue::U32(v))) => self.data = data.pushed_u32(Some(v)),
            (ColumnType::U32, Some(DynamicValue::U64(v))) => {
                self.data = data.into_u64().pushed_u64(Some(v))
            }
            (ColumnType::U32, Some(DynamicValue::I8(v))) => {
                self.data = data.into_i64().pushed_i64(Some(v as i64))
            }
            (ColumnType::U32, Some(DynamicValue::I16(v))) => {
                self.data = data.into_i64().pushed_i64(Some(v as i64))
            }
            (ColumnType::U32, Some(DynamicValue::I32(v))) => {
                self.data = data.into_i64().pushed_i64(Some(v as i64))
            }
            (ColumnType::U32, Some(DynamicValue::I64(v))) => {
                self.data = data.into_i64().pushed_i64(Some(v))
            }
            (ColumnType::U32, Some(DynamicValue::F64(v))) => {
                self.data = data.into_f64().pushed_f64(Some(v))
            }

            (ColumnType::U64, None) => self.data = data.pushed_u64(None),
            (ColumnType::U64, Some(DynamicValue::U8(v))) => {
                self.data = data.pushed_u64(Some(v as u64))
            }
            (ColumnType::U64, Some(DynamicValue::U16(v))) => {
                self.data = data.pushed_u64(Some(v as u64))
            }
            (ColumnType::U64, Some(DynamicValue::U32(v))) => {
                self.data = data.pushed_u64(Some(v as u64))
            }
            (ColumnType::U64, Some(DynamicValue::U64(v))) => self.data = data.pushed_u64(Some(v)),
            (ColumnType::U64, Some(DynamicValue::I8(v))) => {
                self.data = data.into_f64().pushed_f64(Some(v as f64))
            }
            (ColumnType::U64, Some(DynamicValue::I16(v))) => {
                self.data = data.into_f64().pushed_f64(Some(v as f64))
            }
            (ColumnType::U64, Some(DynamicValue::I32(v))) => {
                self.data = data.into_f64().pushed_f64(Some(v as f64))
            }
            (ColumnType::U64, Some(DynamicValue::I64(v))) => {
                self.data = data.into_f64().pushed_f64(Some(v as f64))
            }
            (ColumnType::U64, Some(DynamicValue::F64(v))) => {
                self.data = data.into_f64().pushed_f64(Some(v))
            }

            (ColumnType::I8, None) => self.data = data.pushed_i8(None),
            (ColumnType::I8, Some(DynamicValue::U8(v))) => {
                self.data = data.into_i16().pushed_i16(Some(v as i16))
            }
            (ColumnType::I8, Some(DynamicValue::U16(v))) => {
                self.data = data.into_i32().pushed_i32(Some(v as i32))
            }
            (ColumnType::I8, Some(DynamicValue::U32(v))) => {
                self.data = data.into_i64().pushed_i64(Some(v as i64))
            }
            (ColumnType::I8, Some(DynamicValue::U64(v))) => {
                self.data = data.into_f64().pushed_f64(Some(v as f64))
            }
            (ColumnType::I8, Some(DynamicValue::I8(v))) => self.data = data.pushed_i8(Some(v)),
            (ColumnType::I8, Some(DynamicValue::I16(v))) => {
                self.data = data.into_i16().pushed_i16(Some(v))
            }
            (ColumnType::I8, Some(DynamicValue::I32(v))) => {
                self.data = data.into_i32().pushed_i32(Some(v))
            }
            (ColumnType::I8, Some(DynamicValue::I64(v))) => {
                self.data = data.into_i64().pushed_i64(Some(v))
            }
            (ColumnType::I8, Some(DynamicValue::F64(v))) => {
                self.data = data.into_f64().pushed_f64(Some(v))
            }

            (ColumnType::I16, None) => self.data = data.pushed_i16(None),
            (ColumnType::I16, Some(DynamicValue::U8(v))) => {
                self.data = data.pushed_i16(Some(v as i16))
            }
            (ColumnType::I16, Some(DynamicValue::U16(v))) => {
                self.data = data.into_i32().pushed_i32(Some(v as i32))
            }
            (ColumnType::I16, Some(DynamicValue::U32(v))) => {
                self.data = data.into_i64().pushed_i64(Some(v as i64))
            }
            (ColumnType::I16, Some(DynamicValue::U64(v))) => {
                self.data = data.into_f64().pushed_f64(Some(v as f64))
            }
            (ColumnType::I16, Some(DynamicValue::I8(v))) => {
                self.data = data.pushed_i16(Some(v as i16))
            }
            (ColumnType::I16, Some(DynamicValue::I16(v))) => self.data = data.pushed_i16(Some(v)),
            (ColumnType::I16, Some(DynamicValue::I32(v))) => {
                self.data = data.into_i32().pushed_i32(Some(v))
            }
            (ColumnType::I16, Some(DynamicValue::I64(v))) => {
                self.data = data.into_i64().pushed_i64(Some(v))
            }
            (ColumnType::I16, Some(DynamicValue::F64(v))) => {
                self.data = data.into_f64().pushed_f64(Some(v))
            }

            (ColumnType::I32, None) => self.data = data.pushed_i32(None),
            (ColumnType::I32, Some(DynamicValue::U8(v))) => {
                self.data = data.pushed_i32(Some(v as i32))
            }
            (ColumnType::I32, Some(DynamicValue::U16(v))) => {
                self.data = data.pushed_i32(Some(v as i32))
            }
            (ColumnType::I32, Some(DynamicValue::U32(v))) => {
                self.data = data.into_i64().pushed_i64(Some(v as i64))
            }
            (ColumnType::I32, Some(DynamicValue::U64(v))) => {
                self.data = data.into_f64().pushed_f64(Some(v as f64))
            }
            (ColumnType::I32, Some(DynamicValue::I8(v))) => {
                self.data = data.pushed_i32(Some(v as i32))
            }
            (ColumnType::I32, Some(DynamicValue::I16(v))) => {
                self.data = data.pushed_i32(Some(v as i32))
            }
            (ColumnType::I32, Some(DynamicValue::I32(v))) => self.data = data.pushed_i32(Some(v)),
            (ColumnType::I32, Some(DynamicValue::I64(v))) => {
                self.data = data.into_i64().pushed_i64(Some(v))
            }
            (ColumnType::I32, Some(DynamicValue::F64(v))) => {
                self.data = data.into_f64().pushed_f64(Some(v))
            }

            (ColumnType::I64, None) => self.data = data.pushed_i64(None),
            (ColumnType::I64, Some(DynamicValue::U8(v))) => {
                self.data = data.pushed_i64(Some(v as i64))
            }
            (ColumnType::I64, Some(DynamicValue::U16(v))) => {
                self.data = data.pushed_i64(Some(v as i64))
            }
            (ColumnType::I64, Some(DynamicValue::U32(v))) => {
                self.data = data.pushed_i64(Some(v as i64))
            }
            (ColumnType::I64, Some(DynamicValue::U64(v))) => {
                self.data = data.into_f64().pushed_f64(Some(v as f64))
            }
            (ColumnType::I64, Some(DynamicValue::I8(v))) => {
                self.data = data.pushed_i64(Some(v as i64))
            }
            (ColumnType::I64, Some(DynamicValue::I16(v))) => {
                self.data = data.pushed_i64(Some(v as i64))
            }
            (ColumnType::I64, Some(DynamicValue::I32(v))) => {
                self.data = data.pushed_i64(Some(v as i64))
            }
            (ColumnType::I64, Some(DynamicValue::I64(v))) => self.data = data.pushed_i64(Some(v)),
            (ColumnType::I64, Some(DynamicValue::F64(v))) => {
                self.data = data.into_f64().pushed_f64(Some(v))
            }

            (ColumnType::F64, None) => self.data = data.pushed_f64(None),
            (ColumnType::F64, Some(DynamicValue::U8(v))) => {
                self.data = data.pushed_f64(Some(v as f64))
            }
            (ColumnType::F64, Some(DynamicValue::U16(v))) => {
                self.data = data.pushed_f64(Some(v as f64))
            }
            (ColumnType::F64, Some(DynamicValue::U32(v))) => {
                self.data = data.pushed_f64(Some(v as f64))
            }
            (ColumnType::F64, Some(DynamicValue::U64(v))) => {
                self.data = data.pushed_f64(Some(v as f64))
            }
            (ColumnType::F64, Some(DynamicValue::I8(v))) => {
                self.data = data.pushed_f64(Some(v as f64))
            }
            (ColumnType::F64, Some(DynamicValue::I16(v))) => {
                self.data = data.pushed_f64(Some(v as f64))
            }
            (ColumnType::F64, Some(DynamicValue::I32(v))) => {
                self.data = data.pushed_f64(Some(v as f64))
            }
            (ColumnType::F64, Some(DynamicValue::I64(v))) => {
                self.data = data.pushed_f64(Some(v as f64))
            }
            (ColumnType::F64, Some(DynamicValue::F64(v))) => self.data = data.pushed_f64(Some(v)),

            (ColumnType::String, _) => unreachable!(),
            (ColumnType::Nominal { .. }, _) => unreachable!(),
            (_, Some(DynamicValue::String(_))) => unimplemented!(),
        }
    }

    /// get item by index
    pub fn item(&self, idx: usize) -> Value {
        match self.data {
            ColumnData::U8 { ref values } => values[idx].into(),
            ColumnData::U16 { ref values } => values[idx].into(),
            ColumnData::U32 { ref values } => values[idx].into(),
            ColumnData::U64 { ref values } => values[idx].into(),
            ColumnData::I8 { ref values } => values[idx].into(),
            ColumnData::I16 { ref values } => values[idx].into(),
            ColumnData::I32 { ref values } => values[idx].into(),
            ColumnData::I64 { ref values } => values[idx].into(),
            ColumnData::F64 { ref values } => values[idx].into(),
            ColumnData::String { ref values } => values[idx].as_ref().map(|x| x.as_str()).into(),
            ColumnData::Nominal {
                ref categories,
                ref values,
            } => match values[idx] {
                Some(v) => Value::Nominal(v, &categories),
                None => Value::Missing,
            },
            ColumnData::Invalid => panic!("invalid column state"),
        }
    }
}

macro_rules! def_columndata_into {
    ($name:ident, $variant:ident, $typ:ident) => (
        fn $name(self) -> Self {
            let values = match self {
                ColumnData::U8{values} => values.into_iter()
                                                .map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::U16{values} => values.into_iter()
                                                 .map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::U32{values} => values.into_iter()
                                                 .map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::U64{values} => values.into_iter()
                                                 .map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::I8{values} => values.into_iter()
                                                 .map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::I16{values} => values.into_iter()
                                                 .map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::I32{values} => values.into_iter()
                                                 .map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::I64{values} => values.into_iter()
                                                 .map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::F64{values} => values.into_iter()
                                                 .map(|x| x.map(|v| v as $typ)).collect(),
                ColumnData::String{values} => values.into_iter()
                                                    .map(|x| x.map(|v| v.parse().unwrap()))
                                                    .collect(),
                ColumnData::Nominal{values, ..} => values.into_iter()
                                                         .map(|x| x.map(|v| v as $typ)).collect(),
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
    fn new_from_dtype(dt: DType) -> Self {
        match dt {
            DType::Numeric => ColumnData::new_numeric(),
            DType::String => ColumnData::new_string(),
            DType::Nominal(names) => ColumnData::new_nominal(names),
        }
    }

    fn new_numeric() -> Self {
        ColumnData::U8 { values: Vec::new() }
    }

    fn new_string() -> Self {
        ColumnData::String { values: Vec::new() }
    }

    fn new_nominal(categories: Vec<String>) -> Self {
        ColumnData::Nominal {
            categories,
            values: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        match *self {
            ColumnData::U8 { ref values } => values.len(),
            ColumnData::U16 { ref values } => values.len(),
            ColumnData::U32 { ref values } => values.len(),
            ColumnData::U64 { ref values } => values.len(),
            ColumnData::I8 { ref values } => values.len(),
            ColumnData::I16 { ref values } => values.len(),
            ColumnData::I32 { ref values } => values.len(),
            ColumnData::I64 { ref values } => values.len(),
            ColumnData::F64 { ref values } => values.len(),
            ColumnData::String { ref values } => values.len(),
            ColumnData::Nominal { ref values, .. } => values.len(),
            ColumnData::Invalid => panic!("invalid column state"),
        }
    }

    pub fn get_type(&self) -> ColumnType {
        match *self {
            ColumnData::U8 { .. } => ColumnType::U8,
            ColumnData::U16 { .. } => ColumnType::U16,
            ColumnData::U32 { .. } => ColumnType::U32,
            ColumnData::U64 { .. } => ColumnType::U64,
            ColumnData::I8 { .. } => ColumnType::I8,
            ColumnData::I16 { .. } => ColumnType::I16,
            ColumnData::I32 { .. } => ColumnType::I32,
            ColumnData::I64 { .. } => ColumnType::I64,
            ColumnData::F64 { .. } => ColumnType::F64,
            ColumnData::String { .. } => ColumnType::String,
            ColumnData::Nominal { ref categories, .. } => ColumnType::Nominal {
                categories: categories.clone(),
            },
            ColumnData::Invalid => panic!("invalid column state"),
        }
    }

    fn is_empty(&self) -> bool {
        match *self {
            ColumnData::U8 { ref values } => values.is_empty(),
            ColumnData::U16 { ref values } => values.is_empty(),
            ColumnData::U32 { ref values } => values.is_empty(),
            ColumnData::U64 { ref values } => values.is_empty(),
            ColumnData::I8 { ref values } => values.is_empty(),
            ColumnData::I16 { ref values } => values.is_empty(),
            ColumnData::I32 { ref values } => values.is_empty(),
            ColumnData::I64 { ref values } => values.is_empty(),
            ColumnData::F64 { ref values } => values.is_empty(),
            ColumnData::String { ref values } => values.is_empty(),
            ColumnData::Nominal { ref values, .. } => values.is_empty(),
            ColumnData::Invalid => panic!("invalid column state"),
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
