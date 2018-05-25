# ARFF

ARFF file format serializer and deserializer

An ARFF (Attribute-Relation File Format) file is an ASCII text file
that describes a list of instances sharing a set of attributes. Its
main use is in data science to store tabular data: rows are 
instances and columns are attributes. Meta data such as attribute 
(column) names, data types, and comments are included in the file
format.

## Usage
- ARFF is used as an input file format by the machine-learning tool Weka.
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

The tabular ARFF data is represented in Rust as a sequence of rows. Columns
may have different data types, but each row must be the same.

### Serialization

If `Row` is a valid type that can be serialized into a data row, the following 
types can be serialized as ARFF data sets:

  - Vectors: `Vec<Row>`
  - Slices: `&[Row]`
  - Arrays: `[Row; N]`
  - Tuples: `(Row, Row, Row, ...)`
  
By default the data set is named `unnamed_data`. You can give the data set
a different name by wrapping it in a newtype struct. For example, 
`MyData(Vec<Row>)` is represented in ARFF as

```arff
@RELATION MyData
...
```

A tuple struct is serialized like a tuple wrapped in a newtype struct.
`MoreData((Row, Row, Row))` is equivalent to `MoreData(Row, Row, Row)`.

Valid types for the `Row` data format are

 - Structures: `#[derive(Serialize)] struct Row { ... }`
 - Tuples: `type Row = (i32, f64, bool, String, ...);`
 - Arrays: `type Row<T> = [T; N];`
 
TODO: 
- Nested sequences
- Allowed column types
 
### Deserialization

TODO

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
