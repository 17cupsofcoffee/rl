use std::collections::HashMap;

use specs::Entity;

#[derive(Default)]
pub struct Input {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Input {
    pub fn new() -> Input {
        Input {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

#[derive(Default)]
pub struct TurnState {
    pub waiting: bool,
}

impl TurnState {
    pub fn new() -> TurnState {
        TurnState { waiting: false }
    }
}

#[derive(Default)]
pub struct Map {
    pub tiles: HashMap<(i32, i32), Entity>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            tiles: HashMap::new(),
        }
    }
}
