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
