use super::direction::Direction;

pub struct Solution {
    pub steps: Option<Vec<Direction>>,
    pub edges_traversed: u32,
}

impl Solution {
    pub fn new() -> Self {
        Solution {
            steps: None,
            edges_traversed: 0,
        }
    }

    pub fn get_solution_string(&self) -> Option<String> {
        self.steps
            .as_ref()
            .map(|x| Direction::to_string(x.to_vec()))
    }
}
