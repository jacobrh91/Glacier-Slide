use serde::Serialize;

use super::point::Point;

#[derive(Debug, Clone, Serialize)]
pub struct Player(pub Point);

#[derive(Debug, Clone, Serialize)]
pub struct Start(pub Point);

#[derive(Debug, Clone, Serialize)]
pub struct End(pub Point);

#[derive(Debug, Serialize)]
pub struct Rock(pub Point);

#[derive(Debug, Clone)]
pub enum Tile {
    Wall,
    Rock,
    Start,
    End,
    Player,
    Ice,
}

impl Tile {
    pub fn as_char(&self) -> char {
        match self {
            Tile::Wall => 'W',
            Tile::Rock => 'R',
            Tile::Start => 'S',
            Tile::End => 'E',
            Tile::Player => 'P',
            Tile::Ice => ' ', // So when the board is serialized to JSON, there is a gap.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_as_char_maps_correctly() {
        assert_eq!(Tile::Wall.as_char(), 'W');
        assert_eq!(Tile::Rock.as_char(), 'R');
        assert_eq!(Tile::Start.as_char(), 'S');
        assert_eq!(Tile::End.as_char(), 'E');
        assert_eq!(Tile::Player.as_char(), 'P');
        assert_eq!(Tile::Ice.as_char(), ' ');
    }
}
