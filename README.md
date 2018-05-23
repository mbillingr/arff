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

This crate is not yet published at [crates.io](https://crates.io/), so 
your `Cargo.toml` needs to link to the repository directly.
```toml
[dependencies]
arff = { git = "https://github.com/mbillingr/arff" }
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
