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
