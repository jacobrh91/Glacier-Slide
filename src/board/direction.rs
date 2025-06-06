use std::fmt;

#[derive(Clone, Copy, PartialEq)]
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
    ShowSolution,
    Exit,
}
