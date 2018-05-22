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
}

