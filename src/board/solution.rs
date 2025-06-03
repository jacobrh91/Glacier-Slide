pub struct Solution {
    pub solutions: Vec<String>,
}

impl Solution {
    pub fn is_solvable(self: &Self) -> bool {
        self.solutions.len() > 0
    }
}
