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
            .map(|steps| steps.iter().map(|d| d.as_char()).collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Direction;

    #[test]
    fn solution_with_no_steps_returns_none() {
        let s = Solution {
            steps: None,
            edges_traversed: 5,
        };

        assert_eq!(s.get_solution_string(), None);
    }

    #[test]
    fn solution_to_string_converts_directions_in_order() {
        let s = Solution {
            steps: Some(vec![
                Direction::Up,
                Direction::Right,
                Direction::Up,
                Direction::Down,
                Direction::Down,
                Direction::Left,
            ]),
            edges_traversed: 6,
        };

        assert_eq!(s.get_solution_string(), Some("URUDDL".to_string()));
    }
}
