use std::{error::Error, fmt};

use crate::board::point::Point;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }

    pub fn from_string(s: &str) -> Result<Vec<Direction>, Box<dyn Error>> {
        let mut results = Vec::new();
        for c in s.chars() {
            match c.to_ascii_uppercase() {
                'U' => results.push(Direction::Up),
                'D' => results.push(Direction::Down),
                'L' => results.push(Direction::Left),
                'R' => results.push(Direction::Right),
                _ => return Err(format!("Unknown direction {}", c).into()),
            }
        }
        Ok(results)
    }

    pub fn to_string(directions: Vec<Direction>) -> String {
        let mut result = String::with_capacity(directions.len());
        for direction in directions {
            result.push_str(&direction.to_string());
        }
        result
    }
}
impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Up => String::from("U"),
            Direction::Down => String::from("D"),
            Direction::Left => String::from("L"),
            Direction::Right => String::from("R"),
        }
    }
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Slide {
    pub steps: u8,
    pub direction: Direction,
}

impl Slide {
    pub fn new(steps: u8, direction: Direction) -> Self {
        Slide { steps, direction }
    }
}

#[derive(Debug, Clone)]
pub enum Move {
    Reset,
    MovePlayer(Slide),
    ChangeView,
    Exit,
}
