use std::collections::BTreeSet;

use serde::ser::{self, Serialize};

use super::error::{Error, Result};


#[derive(Debug)]
struct Header {
    name: &'static str,
    attr_names: Vec<String>,
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

enum State {
    Init,
    Data,
}


pub struct Serializer {
    header: Header,
    output: String,
    state: State,
    col_idx: usize,
    current_key: Option<&'static str>,
}


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
        state: State::Init,
        col_idx: 0,
        current_key: None,
    };
    value.serialize(&mut serializer)?;

    let header = serializer.header.to_string();

    Ok(header + &serializer.output)
}

impl Serializer {
    fn get_current_dtype(&mut self) -> Option<&mut DType> {
        if self.col_idx < self.header.attr_types.len() {
            Some(&mut self.header.attr_types[self.col_idx])
        } else {
            None
        }
    }

    fn set_current_dtype(&mut self, dt: DType) {
        if self.col_idx > self.header.attr_types.len() {
            panic!("col_idx is too far ahead")
        }

        if self.col_idx == self.header.attr_types.len() {
            self.header.attr_types.push(dt);
        } else {
            self.header.attr_types[self.col_idx] = dt;
        }
    }
    fn get_current_name(&self) -> Option<&str> {
        if self.col_idx < self.header.attr_names.len() {
            Some(&self.header.attr_names[self.col_idx])
        } else {
            None
        }
    }

    fn set_current_name(&mut self, n: &str) {
        if self.col_idx > self.header.attr_names.len() {
            panic!("col_idx is too far ahead")
        }

        if self.col_idx == self.header.attr_names.len() {
            self.header.attr_names.push(n.to_owned());
        } else {
            self.header.attr_names[self.col_idx] = n.to_owned();
        }
    }
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

    fn serialize_bool(self, v: bool) -> Result<()> {
        match self.state {
            State::Init => panic!("Attempting to serialize value during Init state"),
            State::Data => {
                match self.get_current_dtype() {
                    None => self.set_current_dtype(DType::Nominal(["f", "t"].iter().cloned().collect())),
                    Some(DType::Nominal(_)) => {}
                    Some(_) => return Err(Error::InconsistentDataType),
                }
            }
        }
        self.output += if v {"t"} else {"f"};
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
        match self.state {
            State::Init => panic!("Attempting to serialize value during Init state"),
            State::Data => {
                match self.get_current_dtype() {
                    None => self.set_current_dtype(DType::Numeric),
                    Some(DType::Numeric) => {}
                    Some(_) => return Err(Error::InconsistentDataType),
                }
            }
        }
        self.output += &v.to_string();
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
        match self.state {
            State::Init => panic!("Attempting to serialize value during Init state"),
            State::Data => {
                match self.get_current_dtype() {
                    None => self.set_current_dtype(DType::Numeric),
                    Some(DType::Numeric) => {}
                    Some(_) => return Err(Error::InconsistentDataType),
                }
            }
        }
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        match self.state {
            State::Init => panic!("Attempting to serialize value during Init state"),
            State::Data => {
                match self.get_current_dtype() {
                    None => self.set_current_dtype(DType::Numeric),
                    Some(DType::Numeric) => {}
                    Some(_) => return Err(Error::InconsistentDataType),
                }
            }
        }
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        match self.state {
            State::Init => panic!("Attempting to serialize value during Init state"),
            State::Data => {
                match self.get_current_dtype() {
                    None => self.set_current_dtype(DType::String),
                    Some(DType::String) => {}
                    Some(_) => return Err(Error::InconsistentDataType),
                }
            }
        }
        self.output += "'";
        self.output += v;
        self.output += "'";
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
        match self.state {
            State::Init => panic!("Attempting to serialize value during Init state"),
            State::Data => {
                if self.get_current_dtype().is_none() {
                    self.set_current_dtype(DType::Nominal(BTreeSet::new()));
                }
                if let Some(DType::Nominal(variants)) = self.get_current_dtype() {
                    variants.insert(variant);
                } else {
                    return Err(Error::InconsistentDataType)
                }
            }
        }
        self.output += variant;
        Ok(())
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        match self.state {
            State::Init => {
                self.header.name = name;
            }
            State::Data => {
                self.col_idx = 0;
                self.current_key = None;
            }
        }
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        match self.state {
            State::Init => {
                self.state = State::Data;
            }
            _ => unimplemented!()
        }
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

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        match self.state {
            State::Init => {
                if len != 1 {
                    return Err(Error::NotOneField)
                }
                self.header.name = name;
            }
            State::Data => {
                self.col_idx = 0;
                self.current_key = None;
            }
        }
        Ok(self)
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
        match self.state {
            State::Init => panic!("Should never enter `SerializeSeq` in Init state"),
            State::Data => value.serialize(&mut **self)?,
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
        match self.state {
            State::Init => unimplemented!(),
            State::Data => {
                if self.get_current_name().is_none() {
                    let name = match self.current_key {
                        Some(key) => key.to_owned() + &(self.col_idx + 1).to_string(),
                        None => "col".to_owned() + &(self.col_idx + 1).to_string(),
                    };
                    self.set_current_name(&name);
                }
                if self.col_idx > 0 {
                    self.output += ", ";
                }
                value.serialize(&mut **self)?;
                self.col_idx += 1;
            }
        }
        Ok(())
    }

    fn end(self) -> Result<()> {
        match self.state {
            State::Init => panic!("Should never leave `SerializeTuple` in Init state"),
            State::Data => {}
        }
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

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize
    {
        match self.state {
            State::Init => {
                value.serialize(&mut **self)?;
            }
            State::Data => {
                if self.col_idx > 0 {
                    self.output += ", ";
                }
                self.current_key = Some(key);
                let last_idx = self.col_idx;
                value.serialize(&mut **self)?;
                if last_idx == self.col_idx {
                    self.header.attr_names.push(key.to_owned());
                    self.col_idx += 1;
                }
            }
        }
        Ok(())
    }

    fn end(self) -> Result<()> {
        match self.state {
            State::Init => panic!("Should never leave `SerializeStruct` in Init state"),
            State::Data => {}
        }
        Ok(())
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
    struct Data {
        data: Vec<Row>,
    }

    let test = Data{ data: vec![
        Row {a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, g: 0, h: 0, i: 0.0, j: 0.0, k: "", l: false, m: Color::Red},
        Row {a: 1, b: 2, c: 3, d: 4, e: -4, f: -3, g: -2, h: -1, i: 1.0/3.0, j: 2.0/3.0, k: "abc", l: true, m: Color::Blue},
    ]};

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
    struct Data {
        data: Vec<Row>,
    }

    let test = Data{ data: vec![
        Row {rgb: [255, 0, 0], name: "red".to_owned()},
        Row {rgb: [0, 255, 0], name: "green".to_owned()},
        Row {rgb: [0, 0, 255], name: "blue".to_owned()},
    ]};

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
