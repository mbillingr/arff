use serde::de::{self, Deserialize, DeserializeSeed, Visitor, SeqAccess,
                MapAccess, IntoDeserializer};

use super::error::{Error, Result};
use super::parser::*;


pub struct Deserializer<'de> {
    parser: Parser<'de>,
    header: Header,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Result<Self> {
        let mut parser = Parser::new(input);
        let header = parser.parse_header()?;

        Ok(Deserializer {
            parser,
            header,
        })
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
    where
        T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s)?;

    let t = T::deserialize(&mut deserializer)?;
    if deserializer.parser.is_eof() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
        where V:
        Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        let value = visitor.visit_seq(DataRows::new(self))?;
        Ok(value)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::WrongDatasetType)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        panic!("We should not be here... this must be a bug!")
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }
}


pub struct RowDeserializer<'de: 'a, 'a> {
    parser: &'a mut Parser<'de>,
    header: &'a Header,
    current_column: usize,
}

impl<'de, 'a> RowDeserializer<'de, 'a> {
    fn new(parser: &'a mut Parser<'de>, header: &'a Header) -> Self {
        RowDeserializer {
            parser,
            header,
            current_column: 0,
        }
    }
}

impl<'de, 'a, 'b> de::Deserializer<'de> for &'b mut RowDeserializer<'de, 'a> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_bool(self.parser.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        let value = self.parser.parse_signed()?;
        match value {
            -128...127 => visitor.visit_i8(value as i8),
            _ => Err(Error::NumericRange),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        let value = self.parser.parse_signed()?;
        match value {
            I16_MIN...I16_MAX => visitor.visit_i16(value as i16),
            _ => Err(Error::NumericRange),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        let value = self.parser.parse_signed()?;
        match value {
            I32_MIN...I32_MAX => visitor.visit_i32(value as i32),
            _ => Err(Error::NumericRange),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_i64(self.parser.parse_signed()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        let value = self.parser.parse_unsigned()?;
        match value {
            0...255 => visitor.visit_u8(value as u8),
            _ => Err(Error::NumericRange),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        let value = self.parser.parse_unsigned()?;
        match value {
            0...U16_MAX => visitor.visit_u16(value as u16),
            _ => Err(Error::NumericRange),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        let value = self.parser.parse_unsigned()?;
        match value {
            0...U32_MAX => visitor.visit_u32(value as u32),
            _ => Err(Error::NumericRange),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_u64(self.parser.parse_unsigned()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_f32(self.parser.parse_float()? as f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_f64(self.parser.parse_float()?)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_str(&self.parser.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
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

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_seq(DataCols::new(self))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_map(DataCols::new(self))
    }

    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_enum(self.parser.parse_string()?.into_deserializer())
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_str(&self.header.attrs[self.current_column].name)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }
}


struct DataRows<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> DataRows<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        DataRows { de }
    }
}

impl<'de, 'a> SeqAccess<'de> for DataRows<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
        where
            T: DeserializeSeed<'de>,
    {
        if self.de.parser.is_eof() {
            Ok(None)
        } else {
            let value = {
                let mut de = RowDeserializer::new(&mut self.de.parser, &self.de.header);
                Some(seed.deserialize(&mut de)?)
            };
            match self.de.parser.parse_newline() {
                Ok(_) => Ok(value),
                Err(Error::Eof) => Ok(value),
                Err(e) => Err(e),
            }
        }
    }
}

struct DataCols<'a, 'b: 'a, 'de: 'b> {
    de: &'a mut RowDeserializer<'de, 'b>,
}

impl<'a, 'b, 'de> DataCols<'a, 'b, 'de> {
    fn new(de: &'a mut RowDeserializer<'de, 'b>) -> Self {
        DataCols { de }
    }
}

impl<'a, 'b, 'de> MapAccess<'de> for DataCols<'a, 'b, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
        where
            K: DeserializeSeed<'de>,
    {
        if self.de.current_column >= self.de.header.attrs.len() {
            Ok(None)
        } else {
            seed.deserialize(&mut *self.de).map(Some)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
        where
            V: DeserializeSeed<'de>,
    {
        let value = seed.deserialize(&mut *self.de)?;
        self.de.current_column += 1;
        if self.de.current_column < self.de.header.attrs.len() {
            self.de.parser.parse_token(",")?;
        }
        Ok(value)
    }
}

impl<'de, 'a, 'b> SeqAccess<'de> for DataCols<'a, 'b, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
        where
            T: DeserializeSeed<'de>,
    {
        let value = seed.deserialize(&mut *self.de)?;
        //self.de.parser.parse_token(",")?;
        self.de.parser.parse_optional(',').unwrap();
        Ok(Some(value))
    }
}


#[test]
fn test_struct_data() {
    #[derive(Debug, Deserialize, PartialEq)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Row {
        a: u8,
        b: u16,
        c: u32,
        d: u64,
        e: i8,
        f: i16,
        g: i32,
        h: i64,
        i: f32,
        j: f64,
        k: String,
        l: bool,
        m: Color,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Data(Vec<Row>);

    let input = "@RELATION Data

@ATTRIBUTE a NUMERIC
@ATTRIBUTE b NUMERIC
@ATTRIBUTE c NUMERIC
@ATTRIBUTE d NUMERIC
@ATTRIBUTE e NUMERIC
@ATTRIBUTE f NUMERIC
@ATTRIBUTE g NUMERIC
@ATTRIBUTE h NUMERIC
@ATTRIBUTE i NUMERIC
@ATTRIBUTE j NUMERIC
@ATTRIBUTE k STRING
@ATTRIBUTE l {f, t}
@ATTRIBUTE m {Blue, Red}

@DATA
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', f, Red
1, 2, 3, 4, -4, -3, -2, -1, 0.3333333333333333, 0.6666666666666666, 'abc', t, Blue
";

    let res: Data = from_str(input).unwrap();
    assert_eq!(res, Data(vec![
        Row {a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, g: 0, h: 0, i: 0.0, j: 0.0, k: "".to_owned(), l: false, m: Color::Red},
        Row {a: 1, b: 2, c: 3, d: 4, e: -4, f: -3, g: -2, h: -1, i: 1.0/3.0, j: 2.0/3.0, k: "abc".to_owned(), l: true, m: Color::Blue},
    ]));
}

#[test]
fn test_primitive() {

    let input = "@RELATION Data

@ATTRIBUTE a NUMERIC
@ATTRIBUTE b NUMERIC

@DATA
42, 9
7, 5";

    #[derive(Debug, Deserialize, PartialEq)]
    struct Data {
        b: u8,
        a: u8,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct DataSet(Vec<Data>);

    let res: DataSet = from_str(input).unwrap();
    assert_eq!(res, DataSet(vec![Data{a: 42, b: 9},
                                 Data{b: 5, a: 7}]));
}

#[test]
fn test_2darray() {

    let input = "@RELATION Data

@ATTRIBUTE a NUMERIC
@ATTRIBUTE b NUMERIC

@DATA
42, 9
7, 5";

    let res: [[u8; 2]; 2] = from_str(input).unwrap();
    assert_eq!(res, [[42, 9], [7, 5]]);
}

#[test]
fn test_mixed() {

    let input = "@RELATION Data

@ATTRIBUTE a NUMERIC
@ATTRIBUTE b NUMERIC
@ATTRIBUTE c NUMERIC
@ATTRIBUTE d NUMERIC

@DATA
42, 9, 8, 7
7, 5, 3, 2";

    let res: Vec<(u8, [u8; 2], u8)> = from_str(input).unwrap();
    assert_eq!(res, vec![(42, [9, 8], 7), (7, [5, 3], 2)]);
}