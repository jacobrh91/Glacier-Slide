use std::fmt::Debug;
use std::fmt::Display;

use serde::ser::SerializeTuple;
use serde::Serialize;
use serde::Serializer;

#[derive(PartialEq, Hash, Eq, Clone, Copy)]

pub struct Point {
    pub col: usize,
    pub row: usize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.col, self.row)
    }
}
impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.col, self.row)
    }
}

impl Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(&self.col)?;
        tup.serialize_element(&self.row)?;
        tup.end()
    }
}
