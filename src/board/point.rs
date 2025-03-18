use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}
