# ARFF &emsp; [![Build Status]][travis] [![Latest Version]][crates.io] [![Rustc Version 1.23+]][rustc]

[Build Status]: https://api.travis-ci.org/mbillingr/arff.svg?branch=master
[travis]: https://travis-ci.org/mbillingr/arff
[Latest Version]: https://img.shields.io/crates/v/arff.svg
[crates.io]: https://crates.io/crates/arff
[Rustc Version 1.23+]: https://img.shields.io/badge/rustc-1.23+-lightgray.svg
[rustc]: https://blog.rust-lang.org/2018/01/04/Rust-1.23.html

ARFF file format serializer and deserializer

An [ARFF (Attribute-Relation File Format)](http://weka.wikispaces.com/ARFF) file is an ASCII text file
that describes a list of instances sharing a set of attributes. Its
main use is in data science to store tabular data: rows are 
instances and columns are attributes. Meta data such as attribute 
(column) names, data types, and comments are included in the file
format.

## Usage
- ARFF is used as an input file format by the machine-learning tool
  Weka.
- The [OpenML website](https://www.openml.org/) provides data sets in
  ARFF and CSV formats.

The ARFF crate utilizes the power of Serde to allow serialization and
deserialization of certain Rust types. The file format is relatively
simple, so not all rust types are supported. As a general rule of thumb,
data needs to be represented as a sequence of rows, and a row can be
either a `struct` with named columns or a sequence with static length.

## Example

```toml
[dependencies]
arff = "0.1"
```

```rust
extern crate arff;

#[macro_use]
extern crate serde_derive;

fn main() {
    let input = "
@RELATION Data
@ATTRIBUTE a NUMERIC
@ATTRIBUTE b NUMERIC

@DATA
42, 9
7, 5";

    #[derive(Debug, Deserialize)]
    struct NamedRow {
        b: i32,  // order of fields does not matter
        a: i32,
    }
    
    let named_data: Vec<NamedRow> = arff::from_str(input).unwrap();
    println!("{:?}", named_data);
    
    let unnamed_data: Vec<[i32; 2]> = arff::from_str(input).unwrap();
    println!("{:?}", unnamed_data);
}
```

## Supported Data Types

The tabular ARFF data is represented in Rust as a sequence of rows.
Columns may have different data types, but each row must be the same.

### Serialization

#### Data Set Types

If `Row` is a valid type that can be serialized into a data row, the
following  types can be serialized as ARFF data sets:

  - Vectors: `Vec<Row>`
  - Slices: `&[Row]`
  - Arrays: `[Row; N]`
  - Tuples: `(Row, Row, Row, ...)`
  
By default the data set is named `unnamed_data`. You can give the data
set a different name by wrapping it in a newtype struct. For example,
`MyData(Vec<Row>)` is represented in ARFF as

```arff
@RELATION MyData
...
```

A tuple struct is serialized like a tuple wrapped in a newtype struct.
`MoreData((Row, Row, Row))` is equivalent to `MoreData(Row, Row, Row)`.

#### Data Row Types

Valid types for the `Row` data format are

 - Structures: `#[derive(Serialize)] struct Row { ... }`
 - Tuples: `type Row = (i32, f64, bool, String, ...);`
 - Arrays: `type Row<T> = [T; N];`

#### Nested Columns

It is possible to have nested sequences in rows. These will be flattened
during serialization. For example, `[[i32; 2]; 2]`,
`(i32, [i32; 2], i32)`, and `[i32; 4]` result in equivalent
serializations.

Nested structs are currently not supported because they could result in
ambiguous column names.

#### Value Types

ARFF supports NUMURIC, STRING, and NOMINAL data types. The serializer
performs the following mappings from rust types to ARFF types:

  - NUMERIC <-- `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`, `f32`, `f64`
  - STRING <-- `String`, `&str`
  - NOMINAL <-- `enum`

Missing values are encoded as `?` in ARFF. `Option::None` is mapped to
`?`, while `Option::Some(T)` is unwrapped and serialized according to
the rules above.
 
### Deserialization

#### Data Set Types

If `Row` is a valid type that can be deserialized into a data row, the
following types can be deserialized from ARFF:

  - Vectors: `Vec<Row>`
  - Arrays: `[Row; N]`
  - Tuples: `(Row, Row, Row, ...)`

The data set name is ignored during deserialization. It is possible to
wrap above types in a newtype struct, but the name of the type is not
checked against the data set.

#### Data Row Types

Valid types for deserializing a data row are

  - Structures: `#[derive(Deserialize)] sruct Row { ... }`
  - Arrays: `type Row<T> = [T; N];`
  - Tuples: `type Row = (i32, f64, Sting, bool, ...);`
  - Vectors: `type Row<T> = Vec<T>;`

#### Nested Columns

Nested sequences will be flattened, similar to serialization.

#### Value Types

The deserializer ignores the ARFF type description and tries to
parse each value as the rust type expected by the targetted
data structure. If this is not possible an `Error` is returned.

Columns that can contain missing values need to be wrapped in an
`Option`, so that an encoded `?` is parsed as `None`.


## License

The ARFF crate is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally 
submitted for inclusion in the ARFF crate by you, as defined in the 
Apache-2.0 license, shall be dual licensed as above, without any 
additional terms or conditions.
