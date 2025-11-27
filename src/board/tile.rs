use serde::{Serialize, Serializer};

use super::point::Point;

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    pub pos: Point,
}

#[derive(Debug, Clone)]
pub struct Start {
    pub pos: Point,
}

impl Serialize for Start {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Delegate to Point's serialization: [col, row]
        self.pos.serialize(serializer)
    }
}

#[derive(Debug, Clone)]
pub struct End {
    pub pos: Point,
}

impl Serialize for End {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Delegate to Point's serialization: [col, row]
        self.pos.serialize(serializer)
    }
}

#[derive(Debug)]
pub struct Rock {
    pub pos: Point,
}

impl Serialize for Rock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Delegate to Point's serialization: [col, row]
        self.pos.serialize(serializer)
    }
}

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
            Tile::Ice => 'I',
        }
    }
}
