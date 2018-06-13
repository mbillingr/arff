use std::iter;

use serde::de::{self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess,
                SeqAccess, VariantAccess, Visitor};

use error::{Error, Result};

use super::DataSet;
use super::FlatIter;
use super::Value;

pub fn from_dataset<'a, T>(dset: &'a DataSet) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_dataset(dset);
    T::deserialize(&mut deserializer)
}

/// Deserialize from a data set
pub struct Deserializer<'de> {
    input: iter::Peekable<FlatIter<'de>>,
}

impl<'de> Deserializer<'de> {
    pub fn from_dataset(input: &'de DataSet) -> Self {
        Deserializer {
            input: input.flat_iter().peekable(),
        }
    }

    fn next(&mut self) -> Result<(&str, Value)> {
        let n = self.input.next().ok_or(Error::Eof);
        n
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.next()?.1.as_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.next()?.1.as_i8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.next()?.1.as_i16()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.next()?.1.as_i32()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.next()?.1.as_i64()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.next()?.1.as_u8()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.next()?.1.as_u16()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.next()?.1.as_u32()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.next()?.1.as_u64()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.next()?.1.as_f64()? as f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.next()?.1.as_f64()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.next()?.1.as_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.next()?.1.as_string()?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input.peek() {
            None => return Err(Error::Eof),
            Some(&(_, Value::Missing)) => {
                self.next()?;
                visitor.visit_none()
            }
            Some(_) => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    // Tuple structs look just like sequences in JSON.
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self.next()?.1.as_str()?.into_deserializer())
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (name, _) = self.input.peek().unwrap();
        visitor.visit_str(name)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

impl<'de> SeqAccess<'de> for Deserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.input.peek().is_none() {
            return Ok(None)
        }

        seed.deserialize(self).map(Some)
    }
}

impl<'de> MapAccess<'de> for Deserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.input.peek().is_none() {
            return Ok(None)
        }
        seed.deserialize(self).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }
}

#[cfg(test)]
use super::column::{Column, ColumnData};

#[test]
fn simple() {
    let dset = DataSet::new(
        "Test data",
        vec![
            Column::new(
                "int",
                ColumnData::U8 {
                    values: vec![Some(1), Some(4)],
                },
            ),
            Column::new(
                "float",
                ColumnData::F64 {
                    values: vec![Some(2.0), None],
                },
            ),
            Column::new(
                "text",
                ColumnData::String {
                    values: vec![Some("three".to_owned()), Some("7".to_owned())],
                },
            ),
            Column::new(
                "color",
                ColumnData::Nominal {
                    values: vec![Some(2), Some(0)],
                    categories: vec!["red".to_owned(), "green".to_owned(), "blue".to_owned()],
                },
            ),
        ],
    );


    let x: Vec<(u8, Option<f64>, String, String)> = from_dataset(&dset).unwrap();

    assert_eq!(x, vec![(1, Some(2.0), "three".to_owned(), "blue".to_owned()),
                       (4, None, "7".to_owned(), "red".to_owned())]);
}

#[test]
fn named() {
    let dset = DataSet::new(
        "Test data",
        vec![
            Column::new(
                "int",
                ColumnData::U8 {
                    values: vec![Some(1), Some(4)],
                },
            ),
            Column::new(
                "float",
                ColumnData::F64 {
                    values: vec![Some(2.0), None],
                },
            ),
            Column::new(
                "text",
                ColumnData::String {
                    values: vec![Some("three".to_owned()), Some("7".to_owned())],
                },
            ),
            Column::new(
                "color",
                ColumnData::Nominal {
                    values: vec![Some(2), Some(0)],
                    categories: vec!["Red".to_owned(), "Green".to_owned(), "Blue".to_owned()],
                },
            ),
        ],
    );

    #[derive(Debug, Deserialize)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    #[derive(Debug, Deserialize)]
    struct Row {
        int: i16,
        float: Option<f32>,
        text: String,
        color: Color,
    }


    let x: Vec<Row> = from_dataset(&dset).unwrap();

    println!("{:?}", x);

    /*assert_eq!(x, vec![(1, Some(2.0), "three".to_owned(), "blue".to_owned()),
                       (4, None, "7".to_owned(), "red".to_owned())]);*/
}