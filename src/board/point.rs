#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub col: usize,
    pub row: usize,
}

impl serde::Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize as [col, row]
        (self.col, self.row).serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn point_serializes_as_col_row_array() {
        let p = Point { col: 2, row: 5 };
        let json = serde_json::to_string(&p).unwrap();
        assert_eq!(json, "[2,5]");
    }
}
