use ggez::graphics::Color;
use specs::{Component, NullStorage, VecStorage};

pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

pub struct Sprite {
    pub character: char,
    pub color: Color,
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}

pub struct Tile {
    pub color: Color,
}

impl Component for Tile {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}
