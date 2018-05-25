// Copyright 2018 Martin Billinger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! # ARFF
//!
//! An ARFF (Attribute-Relation File Format) file is an ASCII text file
//! that describes a list of instances sharing a set of attributes. Its
//! main use is in data science to store tabular data: each row is an
//! instance and each column is an attribute. In addition it contains
//! meta-data such as attribute (column) names, data types, and comments.
//!
//! ## Usage
//! - ARFF is used as an input file format by the machine-learning tool Weka.
//! - The [OpenML website](https://www.openml.org/) provides data sets in
//!   ARFF and CSV formats.
//!
//! The ARFF crate utilizes the power of Serde to allow serialization and
//! deserialization of certain Rust types. The file format is relatively
//! simple, so not all rust types are supported. As a general rule of thumb,
//! data needs to be represented as a sequence of rows, and a row can be
//! either a `struct` with named columns or a sequence with static length.
//!
//! ## Example
//!
//! ```rust
//! extern crate arff;
//!
//! #[macro_use]
//! extern crate serde_derive;
//!
//! fn main() {
//!     let input = "
//! @RELATION Data
//! @ATTRIBUTE a NUMERIC
//! @ATTRIBUTE b NUMERIC
//!
//! @DATA
//! 42, 9
//! 7, 5";
//!
//!     #[derive(Debug, Deserialize)]
//!     struct NamedRow {
//!         b: i32,  // order of fields does not matter
//!         a: i32,
//!     }
//!
//!     let named_data: Vec<NamedRow> = arff::from_str(input).unwrap();
//!     println!("{:?}", named_data);
//!
//!     let unnamed_data: Vec<[i32; 2]> = arff::from_str(input).unwrap();
//!     println!("{:?}", unnamed_data);
//! }
//! ```

extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

mod error;
mod ser;
mod de;
mod parser;

pub use error::{Error, Result};
pub use ser::{to_string, Serializer};
pub use de::{from_str, Deserializer};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_1() {

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Row {
            a: i16,
            b: f32,
            c: String,
        }

        let orig = vec![
            Row{a: 0, b: 0.0, c: String::new()},
            Row{a: 1, b: 2.0, c: "123".to_owned()},
            Row{a: -1726, b: 3.1415, c: "pie".to_owned()},
        ];

        let arff = to_string(&orig).unwrap();
        let deser: Vec<Row> = from_str(&arff).unwrap();

        assert_eq!(deser, orig);
    }

    #[test]
    fn roundtrip_2() {

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Row {
            a: i16,
            b: f32,
            c: String,
        }

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct MyData(Vec<Row>);

        let input = "@RELATION MyData

@ATTRIBUTE a NUMERIC
@ATTRIBUTE b NUMERIC
@ATTRIBUTE c STRING

@DATA
0, 0, ''
1, 2, '123'
-1726, 3.1414999961853027, 'pie'
";

        let data: MyData = from_str(input).unwrap();
        let output = to_string(&data).unwrap();

        assert_eq!(input, output);
    }

    #[test]
    fn roundtrip_3() {

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        enum Answer {
            Yes,
            No,
            Maybe,
            Dunno,
        }

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Row {
            x: f32,
            class: Answer,
        }

        let orig = vec![
            Row{x: -1.0, class: Answer::No},
            Row{x: 0.0, class: Answer::Maybe},
            Row{x: 1.0, class: Answer::Yes},
        ];

        let arff = to_string(&orig).unwrap();
        let deser: Vec<Row> = from_str(&arff).unwrap();

        assert_eq!(deser, orig);
    }

    #[test]
    fn roundtrip_4() {

        type Row = [[i32; 2]; 2];

        let orig = vec![
            [[1, 2], [3, 4]],
            [[1, 3], [2, 4]],
        ];

        let arff = to_string(&orig).unwrap();
        let deser: Vec<Row> = from_str(&arff).unwrap();

        assert_eq!(deser, orig);
    }

    #[test]
    fn roundtrip_5() {

        type Row = (i32, [u8; 2], i32);

        let orig = vec![
            (1, [2, 3], 4),
            (5, [6, 7], 8),
        ];

        let arff = to_string(&orig).unwrap();
        let deser: Vec<Row> = from_str(&arff).unwrap();

        assert_eq!(deser, orig);
    }

    #[test]
    fn type_ser_support_outer() {
        type Row = [i32; 1];

        let d_tuple: (Row, Row) = ([1], [2]);
        let d_array: [Row; 2] = [[1], [2]];
        let d_vec: Vec<Row> = d_array.to_vec();
        let d_slice: &[Row] = d_array.as_ref();

        assert_eq!(to_string(&d_tuple).unwrap(), format!("@RELATION {}\n\n@ATTRIBUTE col1 NUMERIC\n\n@DATA\n1\n2\n", "unnamed_data"));
        assert_eq!(to_string(&d_array).unwrap(), format!("@RELATION {}\n\n@ATTRIBUTE col1 NUMERIC\n\n@DATA\n1\n2\n", "unnamed_data"));
        assert_eq!(to_string(&d_vec).unwrap(), format!("@RELATION {}\n\n@ATTRIBUTE col1 NUMERIC\n\n@DATA\n1\n2\n", "unnamed_data"));
        assert_eq!(to_string(&d_slice).unwrap(), format!("@RELATION {}\n\n@ATTRIBUTE col1 NUMERIC\n\n@DATA\n1\n2\n", "unnamed_data"));

        #[derive(Serialize, Deserialize)]
        struct NewtypeStruct(Vec<Row>);
        let d_newtype_struct = NewtypeStruct(vec![[1], [2]]);
        assert_eq!(to_string(&d_newtype_struct).unwrap(), format!("@RELATION {}\n\n@ATTRIBUTE col1 NUMERIC\n\n@DATA\n1\n2\n", "NewtypeStruct"));

        #[derive(Serialize, Deserialize)]
        struct TupleStruct(Row, Row);
        let d_tuple_struct = TupleStruct([1], [2]);
        assert_eq!(to_string(&d_tuple_struct).unwrap(), format!("@RELATION {}\n\n@ATTRIBUTE col1 NUMERIC\n\n@DATA\n1\n2\n", "TupleStruct"));
    }

    #[test]
    fn type_ser_support_inner() {
        #[derive(Serialize)]
        struct StructRow {
            x: f64,
            y: i32,
        };

        let d_struct = [StructRow{x: 1.1, y: 2}];
        let d_tuple: [(f64, i32); 1] = [(1.1, 2)];
        let d_array: [[f64; 2]; 1] = [[1.1, 2.0]];

        assert_eq!(to_string(&d_struct).unwrap(), "@RELATION unnamed_data\n\n@ATTRIBUTE x NUMERIC\n@ATTRIBUTE y NUMERIC\n\n@DATA\n1.1, 2\n");
        assert_eq!(to_string(&d_tuple).unwrap(), "@RELATION unnamed_data\n\n@ATTRIBUTE col1 NUMERIC\n@ATTRIBUTE col2 NUMERIC\n\n@DATA\n1.1, 2\n");
        assert_eq!(to_string(&d_array).unwrap(), "@RELATION unnamed_data\n\n@ATTRIBUTE col1 NUMERIC\n@ATTRIBUTE col2 NUMERIC\n\n@DATA\n1.1, 2\n");
    }
}
