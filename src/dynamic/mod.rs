mod column;
mod dataset;
mod iter;
mod value;

pub mod de;

pub use self::column::Column;
pub use self::dataset::DataSet;
pub use self::iter::FlatIter;
pub use self::value::Value;

#[cfg(test)]
use self::column::ColumnData;

#[test]
fn dynamic_loader() {
    let input = "\
@Relation 'Test data'
@Attribute int NUMERIC
@Attribute float NUMERIC
@Attribute text String
@Attribute color {red, green, blue}
@Data
1, 2.0, 'three', blue
4, ?, '7', red
";

    let dset: DataSet = DataSet::from_str(input).unwrap();

    assert_eq!(
        dset,
        DataSet::new(
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
            ]
        )
    );
}
