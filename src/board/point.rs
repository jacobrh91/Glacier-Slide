use std::fmt::Debug;
use std::fmt::Display;

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
