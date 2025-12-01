use std::fmt::{self, Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    pub fn as_char(self) -> char {
        match self {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_char().to_string())
    }
}

impl Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
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
    SlidePlayer(Slide),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_as_char_is_correct() {
        assert_eq!(Direction::Up.as_char(), 'U');
        assert_eq!(Direction::Down.as_char(), 'D');
        assert_eq!(Direction::Left.as_char(), 'L');
        assert_eq!(Direction::Right.as_char(), 'R');
    }
}
