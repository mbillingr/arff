// Copyright 2018 Martin Billinger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Serialize a Rust data structure to ARFF formatted text.

use std::borrow::Cow;
use std::collections::BTreeSet;

use serde::ser::{self, Serialize};

use super::error::{Error, Result};


#[derive(Debug)]
struct Header {
    name: &'static str,
    attr_names: Vec<Cow<'static, str>>,
    attr_types: Vec<DType>,
}

impl Header {
    fn to_string(&self) -> String {
        let mut s = format!("@RELATION {}\n\n", self.name);

        for (aname, atype) in self.attr_names.iter().zip(&self.attr_types) {
            s += &format!("@ATTRIBUTE {} {}\n", aname, atype.to_string());
        }

        s + "\n@DATA\n"
    }
}

#[derive(Debug)]
enum DType {
    Numeric,
    Nominal(BTreeSet<&'static str>),
    String,
    //Date(String),
}

impl DType {
    fn to_string(&self) -> String {
        match self {
            DType::Numeric => "NUMERIC".to_owned(),
            DType::Nominal(names) => {
                let mut s = "{".to_owned();
                for (i, n) in names.iter().enumerate() {
                    if i > 0 {
                        s += ", ";
                    }
                    s += n;
                }
                s += "}";
                s
            },
            DType::String => "STRING".to_owned(),
            //DType::Date(_) => unimplemented!(),
        }
    }
}

/// Serialize an instance of type `T` into an ARFF formatted string.
pub fn to_string<T>(value: &T) -> Result<String>
    where
        T: Serialize,
{
    let mut serializer = Serializer {
        header: Header {
            name: "unnamed_data",
            attr_names: Vec::new(),
            attr_types: Vec::new(),
        },
        output: String::new(),
    };
    value.serialize(&mut serializer)?;

    let header = serializer.header.to_string();

    Ok(header + &serializer.output)
}

pub struct Serializer {
    header: Header,
    output: String,
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _: bool) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i8(self, _: i8) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i16(self, _: i16) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i32(self, _: i32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i64(self, _: i64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u8(self, _: u8) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u16(self, _: u16) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u32(self, _: u32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u64(self, _: u64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f32(self, _: f32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f64(self, _: f64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_char(self, _: char) -> Result<()> {
        unimplemented!()
    }

    fn serialize_str(self, _: &str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        self.header.name = name;
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        unimplemented!()
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize
    {
        {
            let mut ser = RowSerializer::new(self);
            value.serialize(&mut ser)?;
        }
        self.output += "\n";
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize
    {
        {
            let mut ser = RowSerializer::new(self);
            value.serialize(&mut ser)?;
        }
        self.output += "\n";
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
        where
            T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

pub struct RowSerializer<'a> {
    header: &'a mut Header,
    output: &'a mut String,
    current_column: usize,
    current_key: Option<&'static str>,
}

impl<'a> RowSerializer<'a> {
    fn new(ser: &'a mut Serializer) -> Self {
        RowSerializer {
            header: &mut ser.header,
            output: &mut ser.output,
            current_column: 0,
            current_key: None,
        }
    }

    fn get_current_dtype(&mut self) -> Option<&mut DType> {
        self.header.attr_types.get_mut(self.current_column)
    }

    fn set_current_dtype(&mut self, dt: DType) {
        if self.current_column > self.header.attr_types.len() {
            panic!("col_idx is too far ahead")
        }

        if self.current_column == self.header.attr_types.len() {
            self.header.attr_types.push(dt);
        } else {
            self.header.attr_types[self.current_column] = dt;
        }
    }

    fn get_current_name(&self) -> Option<&str> {
        self.header.attr_names.get(self.current_column).map(|s|&s[..])
    }

    fn set_current_name(&mut self, n: Cow<'static, str>) {
        if self.current_column > self.header.attr_names.len() {
            panic!("col_idx is too far ahead")
        }

        if self.current_column == self.header.attr_names.len() {
            self.header.attr_names.push(n);
        } else {
            self.header.attr_names[self.current_column] = n;
        }
    }
}

impl<'a, 'b> ser::Serializer for &'b mut RowSerializer<'a> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        match self.get_current_dtype() {
            None => self.set_current_dtype(DType::Nominal(["f", "t"].iter().cloned().collect())),
            Some(DType::Nominal(_)) => {}
            Some(_) => return Err(Error::InconsistentDataType),
        }
        *self.output += if v {"t"} else {"f"};
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        match self.get_current_dtype() {
            None => self.set_current_dtype(DType::Numeric),
            Some(DType::Numeric) => {}
            Some(_) => return Err(Error::InconsistentDataType),
        }
        *self.output += &v.to_string();
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        match self.get_current_dtype() {
            None => self.set_current_dtype(DType::Numeric),
            Some(DType::Numeric) => {}
            Some(_) => return Err(Error::InconsistentDataType),
        }
        *self.output += &v.to_string();
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        match self.get_current_dtype() {
            None => self.set_current_dtype(DType::Numeric),
            Some(DType::Numeric) => {}
            Some(_) => return Err(Error::InconsistentDataType),
        }
        *self.output += &v.to_string();
        Ok(())
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        match self.get_current_dtype() {
            None => self.set_current_dtype(DType::String),
            Some(DType::String) => {}
            Some(_) => return Err(Error::InconsistentDataType),
        }
        *self.output += "'";
        *self.output += v;
        *self.output += "'";
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<()> {
        if self.get_current_dtype().is_none() {
            self.set_current_dtype(DType::Nominal(BTreeSet::new()));
        }
        if let Some(DType::Nominal(variants)) = self.get_current_dtype() {
            variants.insert(variant);
        } else {
            return Err(Error::InconsistentDataType)
        }
        *self.output += variant;
        Ok(())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        unimplemented!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

impl<'a, 'b> ser::SerializeSeq for &'b mut RowSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, 'b> ser::SerializeTuple for &'b mut RowSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize
    {
        if self.get_current_name().is_none() {
            let name = match self.current_key {
                Some(key) => key.to_owned() + &(self.current_column + 1).to_string(),
                None => "col".to_owned() + &(self.current_column + 1).to_string(),
            };
            self.set_current_name(name.into());
        }

        if self.current_column > 0 && ! self.output.ends_with(", ") {
            *self.output += ", ";
        }

        let last_idx = self.current_column;
        value.serialize(&mut **self)?;
        if last_idx == self.current_column {
            self.current_column += 1;
        }
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'b> ser::SerializeTupleStruct for &'b mut RowSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, 'b> ser::SerializeTupleVariant for &'b mut RowSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, 'b> ser::SerializeMap for &'b mut RowSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
        where
            T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, 'b> ser::SerializeStruct for &'b mut RowSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize
    {
        if self.current_column > 0 {
            *self.output += ", ";
        }
        self.current_key = Some(key);
        let last_idx = self.current_column;
        value.serialize(&mut **self)?;
        if last_idx == self.current_column {
            self.header.attr_names.push(key.into());
            self.current_column += 1;
        }
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'b> ser::SerializeStructVariant for &'b mut RowSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}


#[test]
fn test_struct_data() {
    #[derive(Serialize)]
    enum Color {
        Red,
        Blue,
    }

    #[derive(Serialize)]
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
        k: &'static str,
        l: bool,
        m: Color,
    }

    #[derive(Serialize)]
    struct Data(Vec<Row>);

    let test = Data(vec![
        Row {a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, g: 0, h: 0, i: 0.0, j: 0.0, k: "", l: false, m: Color::Red},
        Row {a: 1, b: 2, c: 3, d: 4, e: -4, f: -3, g: -2, h: -1, i: 1.0/3.0, j: 2.0/3.0, k: "abc", l: true, m: Color::Blue},
    ]);

    let expected = "@RELATION Data

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
1, 2, 3, 4, -4, -3, -2, -1, 0.3333333432674408, 0.6666666666666666, 'abc', t, Blue
";

    let res = to_string(&test).unwrap();

    assert_eq!(res, expected);
}

#[test]
fn test_primitive() {

    let expected = "@RELATION unnamed_data

@ATTRIBUTE b NUMERIC
@ATTRIBUTE a NUMERIC

@DATA
9, 42
5, 7
";

    #[derive(Debug, Serialize)]
    struct Row {
        b: u8,
        a: u8,
    }

    let data = vec![
        Row{a: 42, b: 9},
        Row{b: 5, a: 7}
    ];

    let output = to_string(&data).unwrap();

    assert_eq!(output, expected);
}

#[test]
fn test_newtype_data() {
    #[derive(Serialize)]
    struct Row([u8; 5]);

    #[derive(Serialize)]
    struct Data(Vec<Row>);

    let test = Data(vec![
        Row([1, 2, 3, 4, 5]),
        Row([6, 7, 8, 9, 0]),
    ]);

    let expected = "@RELATION Data

@ATTRIBUTE col1 NUMERIC
@ATTRIBUTE col2 NUMERIC
@ATTRIBUTE col3 NUMERIC
@ATTRIBUTE col4 NUMERIC
@ATTRIBUTE col5 NUMERIC

@DATA
1, 2, 3, 4, 5
6, 7, 8, 9, 0
";

    let res = to_string(&test).unwrap();

    assert_eq!(res, expected);
}

#[test]
fn test_array_data() {

    #[derive(Serialize)]
    struct Row {
        rgb: [u8; 3],
        name: String,
    }

    #[derive(Serialize)]
    struct Data(Vec<Row>);

    let test = Data(vec![
        Row {rgb: [255, 0, 0], name: "red".to_owned()},
        Row {rgb: [0, 255, 0], name: "green".to_owned()},
        Row {rgb: [0, 0, 255], name: "blue".to_owned()},
    ]);

    let expected = "@RELATION Data

@ATTRIBUTE rgb1 NUMERIC
@ATTRIBUTE rgb2 NUMERIC
@ATTRIBUTE rgb3 NUMERIC
@ATTRIBUTE name STRING

@DATA
255, 0, 0, 'red'
0, 255, 0, 'green'
0, 0, 255, 'blue'
";

    let res = to_string(&test).unwrap();

    assert_eq!(res, expected);
}

#[test]
fn test_2darray() {
    let expected = "@RELATION unnamed_data

@ATTRIBUTE col1 NUMERIC
@ATTRIBUTE col2 NUMERIC

@DATA
42, 9
7, 5
";

    let output = to_string(&[[42, 9], [7, 5]]).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn test_mixed() {
    let expected = "@RELATION unnamed_data

@ATTRIBUTE col1 NUMERIC
@ATTRIBUTE col2 NUMERIC
@ATTRIBUTE col3 NUMERIC
@ATTRIBUTE col4 NUMERIC

@DATA
42, 9, 8, 7
7, 5, 3, 2
";

    let data = vec![
        (42, [9, 8], 7),
        (7, [5, 3], 2)
    ];

    let output = to_string(&data).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn test_2dtuple() {
    let expected = "@RELATION unnamed_data

@ATTRIBUTE col1 NUMERIC
@ATTRIBUTE col2 NUMERIC

@DATA
1, 2
3, 4
";

    let data = ((1u8, 2u16), (3u32, 4u64));

    let output = to_string(&data).unwrap();
    assert_eq!(output, expected);
}
