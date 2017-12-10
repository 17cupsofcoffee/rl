use tcod::colors;
use specs::{Entity, World};
use components::*;

pub fn create_player(world: &mut World, x: i32, y: i32) -> Entity {
    world
        .create_entity()
        .with(Position::new(x, y))
        .with(Sprite::new('@', colors::WHITE))
        .with(Movement::new(true, 2, 2))
        .build()
}

pub fn create_snake(world: &mut World, x: i32, y: i32) -> Entity {
    world
        .create_entity()
        .with(Position::new(x, y))
        .with(Sprite::new('S', colors::GREEN))
        .with(Movement::new(false, 0, 1))
        .build()
}

pub fn create_rat(world: &mut World, x: i32, y: i32) -> Entity {
    world
        .create_entity()
        .with(Position::new(x, y))
        .with(Sprite::new('r', colors::SEPIA))
        .with(Movement::new(false, 0, 4))
        .build()
}

pub fn create_wall(world: &mut World, x: i32, y: i32) -> Entity {
    world
        .create_entity()
        .with(Position::new(x, y))
        .with(Tile::new(colors::GREY))
        .with(Solid)
        .build()
}
