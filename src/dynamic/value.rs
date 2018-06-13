use error::{Error, Result};

/// a dynamically typed ARFF value
#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Missing,
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F64(f64),
    String(&'a str),
    Nominal(usize, &'a Vec<String>),
}

impl<'a> From<u8> for Value<'a> {
    fn from(x: u8) -> Self {
        Value::U8(x)
    }
}
impl<'a> From<u16> for Value<'a> {
    fn from(x: u16) -> Self {
        Value::U16(x)
    }
}
impl<'a> From<u32> for Value<'a> {
    fn from(x: u32) -> Self {
        Value::U32(x)
    }
}
impl<'a> From<u64> for Value<'a> {
    fn from(x: u64) -> Self {
        Value::U64(x)
    }
}
impl<'a> From<i8> for Value<'a> {
    fn from(x: i8) -> Self {
        Value::I8(x)
    }
}
impl<'a> From<i16> for Value<'a> {
    fn from(x: i16) -> Self {
        Value::I16(x)
    }
}
impl<'a> From<i32> for Value<'a> {
    fn from(x: i32) -> Self {
        Value::I32(x)
    }
}
impl<'a> From<i64> for Value<'a> {
    fn from(x: i64) -> Self {
        Value::I64(x)
    }
}
impl<'a> From<f32> for Value<'a> {
    fn from(x: f32) -> Self {
        Value::F64(x as f64)
    }
}
impl<'a> From<f64> for Value<'a> {
    fn from(x: f64) -> Self {
        Value::F64(x)
    }
}
impl<'a> From<&'a str> for Value<'a> {
    fn from(s: &'a str) -> Self {
        Value::String(s)
    }
}

impl<'a> From<Option<u8>> for Value<'a> {
    fn from(x: Option<u8>) -> Self {
        x.map_or(Value::Missing, |v| Value::U8(v))
    }
}
impl<'a> From<Option<u16>> for Value<'a> {
    fn from(x: Option<u16>) -> Self {
        x.map_or(Value::Missing, |v| Value::U16(v))
    }
}
impl<'a> From<Option<u32>> for Value<'a> {
    fn from(x: Option<u32>) -> Self {
        x.map_or(Value::Missing, |v| Value::U32(v))
    }
}
impl<'a> From<Option<u64>> for Value<'a> {
    fn from(x: Option<u64>) -> Self {
        x.map_or(Value::Missing, |v| Value::U64(v))
    }
}
impl<'a> From<Option<i8>> for Value<'a> {
    fn from(x: Option<i8>) -> Self {
        x.map_or(Value::Missing, |v| Value::I8(v))
    }
}
impl<'a> From<Option<i16>> for Value<'a> {
    fn from(x: Option<i16>) -> Self {
        x.map_or(Value::Missing, |v| Value::I16(v))
    }
}
impl<'a> From<Option<i32>> for Value<'a> {
    fn from(x: Option<i32>) -> Self {
        x.map_or(Value::Missing, |v| Value::I32(v))
    }
}
impl<'a> From<Option<i64>> for Value<'a> {
    fn from(x: Option<i64>) -> Self {
        x.map_or(Value::Missing, |v| Value::I64(v))
    }
}
impl<'a> From<Option<f32>> for Value<'a> {
    fn from(x: Option<f32>) -> Self {
        x.map_or(Value::Missing, |v| Value::F64(v as f64))
    }
}
impl<'a> From<Option<f64>> for Value<'a> {
    fn from(x: Option<f64>) -> Self {
        x.map_or(Value::Missing, |v| Value::F64(v))
    }
}
impl<'a> From<Option<&'a str>> for Value<'a> {
    fn from(s: Option<&'a str>) -> Self {
        s.map_or(Value::Missing, |v| Value::String(v))
    }
}

impl<'a> Value<'a> {
    pub fn as_bool(&self) -> Result<bool> {
        match *self {
            Value::Missing => Err(Error::UnexpectedMissingValue),
            Value::U8(x) => Ok(x > 0),
            Value::U16(x) => Ok(x > 0),
            Value::U32(x) => Ok(x > 0),
            Value::U64(x) => Ok(x > 0),
            Value::I8(x) => Ok(x > 0),
            Value::I16(x) => Ok(x > 0),
            Value::I32(x) => Ok(x > 0),
            Value::I64(x) => Ok(x > 0),
            Value::F64(x) => Ok(x > 0.0),
            Value::String(s) => Ok(
                s.eq_ignore_ascii_case("true") || s.eq_ignore_ascii_case("yes")
                    || s.eq_ignore_ascii_case("y") || s.eq_ignore_ascii_case("t"),
            ),
            Value::Nominal(i, s) => Ok(s[i].eq_ignore_ascii_case("true")
                || s[i].eq_ignore_ascii_case("yes")
                || s[i].eq_ignore_ascii_case("y")
                || s[i].eq_ignore_ascii_case("t")),
        }
    }

    pub fn as_str(&self) -> Result<&'a str> {
        match *self {
            Value::Missing => Err(Error::UnexpectedMissingValue),
            Value::String(x) => Ok(x),
            Value::Nominal(i, s) => Ok(&s[i]),
            _ => Err(Error::UnexpectedType),
        }
    }

    pub fn as_string(&self) -> Result<String> {
        match *self {
            Value::Missing => Err(Error::UnexpectedMissingValue),
            Value::String(x) => Ok(x.to_owned()),
            Value::Nominal(i, s) => Ok(s[i].to_owned()),
            _ => Err(Error::UnexpectedType),
        }
    }

    pub fn as_u8(&self) -> Result<u8> {
        match *self {
            Value::Missing => Err(Error::UnexpectedMissingValue),
            Value::U8(x) => Ok(x),
            Value::Nominal(i @ 0...255, _) => Ok(i as u8),
            _ => Err(Error::UnexpectedType),
        }
    }

    pub fn as_u16(&self) -> Result<u16> {
        match *self {
            Value::Missing => Err(Error::UnexpectedMissingValue),
            Value::U8(x) => Ok(x as u16),
            Value::U16(x) => Ok(x),
            Value::Nominal(i @ 0...65535, _) => Ok(i as u16),
            _ => Err(Error::ConversionError),
        }
    }

    pub fn as_u32(&self) -> Result<u32> {
        match *self {
            Value::Missing => Err(Error::UnexpectedMissingValue),
            Value::U8(x) => Ok(x as u32),
            Value::U16(x) => Ok(x as u32),
            Value::U32(x) => Ok(x),
            Value::Nominal(i @ 0...4294967295, _) => Ok(i as u32),
            _ => Err(Error::ConversionError),
        }
    }

    pub fn as_u64(&self) -> Result<u64> {
        match *self {
            Value::Missing => Err(Error::UnexpectedMissingValue),
            Value::U8(x) => Ok(x as u64),
            Value::U16(x) => Ok(x as u64),
            Value::U32(x) => Ok(x as u64),
            Value::U64(x) => Ok(x),
            Value::Nominal(i, _) => Ok(i as u64),
            _ => Err(Error::ConversionError),
        }
    }

    pub fn as_i8(&self) -> Result<i8> {
        match *self {
            Value::Missing => return Err(Error::UnexpectedMissingValue),
            Value::I8(x) => Ok(x),
            Value::Nominal(i @ 0...127, _) => Ok(i as i8),
            _ => Err(Error::ConversionError),
        }
    }

    pub fn as_i16(&self) -> Result<i16> {
        match *self {
            Value::Missing => return Err(Error::UnexpectedMissingValue),
            Value::U8(x) => Ok(x as i16),
            Value::I8(x) => Ok(x as i16),
            Value::I16(x) => Ok(x),
            Value::Nominal(i @ 0...32767, _) => Ok(i as i16),
            _ => Err(Error::ConversionError),
        }
    }

    pub fn as_i32(&self) -> Result<i32> {
        match *self {
            Value::Missing => return Err(Error::UnexpectedMissingValue),
            Value::U8(x) => Ok(x as i32),
            Value::U16(x) => Ok(x as i32),
            Value::I8(x) => Ok(x as i32),
            Value::I16(x) => Ok(x as i32),
            Value::I32(x) => Ok(x),
            Value::Nominal(i @ 0...2147483647, _) => Ok(i as i32),
            _ => Err(Error::ConversionError),
        }
    }

    pub fn as_i64(&self) -> Result<i64> {
        match *self {
            Value::Missing => return Err(Error::UnexpectedMissingValue),
            Value::U8(x) => Ok(x as i64),
            Value::U16(x) => Ok(x as i64),
            Value::U32(x) => Ok(x as i64),
            Value::I8(x) => Ok(x as i64),
            Value::I16(x) => Ok(x as i64),
            Value::I32(x) => Ok(x as i64),
            Value::I64(x) => Ok(x),
            Value::Nominal(i @ 0...2147483647, _) => Ok(i as i64),
            _ => Err(Error::ConversionError),
        }
    }

    pub fn as_f64(&self) -> Result<f64> {
        match *self {
            Value::Missing => Err(Error::UnexpectedMissingValue),
            Value::U8(x) => Ok(x as f64),
            Value::U16(x) => Ok(x as f64),
            Value::U32(x) => Ok(x as f64),
            Value::U64(x) => Ok(x as f64),
            Value::I8(x) => Ok(x as f64),
            Value::I16(x) => Ok(x as f64),
            Value::I32(x) => Ok(x as f64),
            Value::I64(x) => Ok(x as f64),
            Value::F64(x) => Ok(x),
            Value::Nominal(i, _) => Ok(i as f64),
            _ => Err(Error::ConversionError),
        }
    }
}

pub trait CastValue: Sized {
    fn from_value(v: Value) -> Result<Self>;
}

impl CastValue for f64 {
    fn from_value(v: Value) -> Result<f64> {
        v.as_f64()
    }
}

impl CastValue for u8 {
    fn from_value(v: Value) -> Result<u8> {
        v.as_u8()
    }
}
