
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
