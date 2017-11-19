use ggez::graphics::Color;
use specs::{Component, NullStorage, VecStorage};

pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

pub struct Sprite {
    pub character: char,
    pub color: Color,
}

impl Sprite {
    pub fn new(character: char, color: Color) -> Sprite {
        Sprite { character, color }
    }
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}

pub struct Tile {
    pub color: Color,
}

impl Tile {
    pub fn new(color: Color) -> Tile {
        Tile { color }
    }
}

impl Component for Tile {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}
