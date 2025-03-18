use super::point::Point;

#[derive(Debug, Clone)]
pub struct Player {
    pub pos: Point,
}

#[derive(Debug, Clone)]
pub struct Start {
    pub pos: Point,
}

#[derive(Debug, Clone)]
pub struct End {
    pub pos: Point,
}

#[derive(Debug, Clone)]
pub struct Rock {
    pub pos: Point,
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
