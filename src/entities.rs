use components::*;
use ggez::graphics::{self, Color};
use specs::{Entity, World};

pub fn create_player(world: &mut World, x: i32, y: i32) -> Entity {
    world
        .create_entity()
        .with(Position::new(x, y))
        .with(Sprite::new('@', graphics::WHITE))
        .with(Movement::new(true, 2, 2))
        .build()
}

pub fn create_snake(world: &mut World, x: i32, y: i32) -> Entity {
    world
        .create_entity()
        .with(Position::new(x, y))
        .with(Sprite::new('S', Color::new(0.0, 1.0, 0.0, 1.0)))
        .with(Movement::new(false, 0, 1))
        .build()
}

pub fn create_rat(world: &mut World, x: i32, y: i32) -> Entity {
    world
        .create_entity()
        .with(Position::new(x, y))
        .with(Sprite::new('r', Color::new(0.59, 0.41, 0.31, 1.0)))
        .with(Movement::new(false, 0, 4))
        .build()
}

pub fn create_wall(world: &mut World, x: i32, y: i32) -> Entity {
    world
        .create_entity()
        .with(Position::new(x, y))
        .with(Tile::new(Color::new(0.2, 0.2, 0.2, 1.0)))
        .with(Solid)
        .build()
}
