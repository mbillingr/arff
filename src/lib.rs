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
