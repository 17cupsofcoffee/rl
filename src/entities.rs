use hecs::{Entity, World};
use tetra::graphics::Color;

use crate::components::*;

pub fn create_player(world: &mut World, x: usize, y: usize) -> Entity {
    world.spawn((
        Position::new(x, y),
        Sprite::new('@', Color::WHITE),
        Movement::new(true, 2, 2),
    ))
}

pub fn create_snake(world: &mut World, x: usize, y: usize) -> Entity {
    world.spawn((
        Position::new(x, y),
        Sprite::new('S', Color::rgb(0.0, 1.0, 0.0)),
        Movement::new(false, 0, 1),
    ))
}

pub fn create_rat(world: &mut World, x: usize, y: usize) -> Entity {
    world.spawn((
        Position::new(x, y),
        Sprite::new('r', Color::rgb(0.59, 0.41, 0.31)),
        Movement::new(false, 0, 4),
    ))
}
