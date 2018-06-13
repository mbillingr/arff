use super::dataset::DataSet;
use super::value::Value;

// Implemented using indices. Performance could probably be improved by using an array of column
// iterators.
pub struct FlatIter<'a> {
    dset: &'a DataSet,
    row_idx: usize,
    col_idx: usize,
}

impl<'a> FlatIter<'a> {
    pub(crate) fn new(dset: &'a DataSet) -> Self {
        FlatIter {
            dset,
            row_idx: 0,
            col_idx: 0,
        }
    }
}

impl<'a> Iterator for FlatIter<'a> {
    type Item = (&'a str, Value<'a>);

    fn next(&mut self) -> Option<(&'a str, Value<'a>)> {
        if self.row_idx >= self.dset.n_rows() {
            return None;
        }

        let value = self.dset.item(self.row_idx, self.col_idx);
        let name = self.dset.col_name(self.col_idx);

        self.col_idx += 1;
        if self.col_idx >= self.dset.n_cols() {
            self.col_idx = 0;
            self.row_idx += 1;
        }

        Some((name, value))
    }
}
