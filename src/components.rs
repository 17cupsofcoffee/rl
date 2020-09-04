use std::collections::VecDeque;

use tetra::graphics::Color;

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
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

pub enum MoveAction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Movement {
    pub player_input: bool,
    pub energy: i32,
    pub speed: i32,
    pub move_queue: VecDeque<MoveAction>,
}

impl Movement {
    pub fn new(player_input: bool, energy: i32, speed: i32) -> Movement {
        Movement {
            player_input,
            energy,
            speed,
            move_queue: VecDeque::new(),
        }
    }

    pub fn ready(&self) -> bool {
        self.energy >= self.speed
    }
}
