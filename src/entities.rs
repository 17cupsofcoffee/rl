use ggez::graphics::{self, Color};
use specs::{Entity, World};
use components::*;

pub fn create_player(world: &mut World, x: i32, y: i32) -> Entity {
    world
        .create_entity()
        .with(Position::new(x, y))
        .with(Sprite::new('@', graphics::WHITE))
        .with(Energy::new(2, 2))
        .with(Player)
        .build()
}

pub fn create_snake(world: &mut World, x: i32, y: i32) -> Entity {
    world
        .create_entity()
        .with(Position::new(x, y))
        .with(Sprite::new('S', Color::new(0.0, 1.0, 0.0, 1.0)))
        .with(Energy::new(0, 1))
        .with(Enemy)
        .build()
}

pub fn create_wall(world: &mut World, x: i32, y: i32) -> Entity {
    world
        .create_entity()
        .with(Position::new(x, y))
        .with(Tile::new(Color::new(0.1, 0.1, 0.1, 1.0)))
        .build()
}
