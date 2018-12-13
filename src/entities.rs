use specs::{Builder, Entity, World};
use tetra::graphics::color;
use tetra::graphics::Color;

use components::*;

pub fn create_player(world: &mut World, x: i32, y: i32) -> Entity {
        world.create_entity()
                .with(Position::new(x, y))
                .with(Sprite::new('@', color::WHITE))
                .with(Movement::new(true, 2, 2))
                .build()
}

pub fn create_snake(world: &mut World, x: i32, y: i32) -> Entity {
        world.create_entity()
                .with(Position::new(x, y))
                .with(Sprite::new('S', Color::rgb(0.0, 1.0, 0.0)))
                .with(Movement::new(false, 0, 1))
                .build()
}

pub fn create_rat(world: &mut World, x: i32, y: i32) -> Entity {
        world.create_entity()
                .with(Position::new(x, y))
                .with(Sprite::new('r', Color::rgb(0.59, 0.41, 0.31)))
                .with(Movement::new(false, 0, 4))
                .build()
}

pub fn create_wall(world: &mut World, x: i32, y: i32) -> Entity {
        world.create_entity()
                .with(Position::new(x, y))
                .with(Tile::new(Color::rgb(0.2, 0.2, 0.2)))
                .with(Solid)
                .build()
}
