use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::fmt::Display;

#[derive(PartialEq, Hash, Eq, Clone, Serialize, Deserialize)]
pub struct Point {
    pub row: usize,
    pub col: usize,
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
