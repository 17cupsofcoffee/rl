use std::collections::HashMap;

use hecs::{Entity, World};

use crate::entities;

pub struct Resources {
    pub input: Input,
    pub turn_state: TurnState,
    pub map: Map,
}

impl Resources {
    pub fn new(world: &mut World) -> Resources {
        Resources {
            input: Input::new(),
            turn_state: TurnState::new(),
            map: Map::generate(world),
        }
    }
}

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

pub struct TurnState {
    pub waiting: bool,
}

impl TurnState {
    pub fn new() -> TurnState {
        TurnState { waiting: false }
    }
}

pub struct Map {
    pub tiles: HashMap<(i32, i32), Entity>,
}

impl Map {
    pub fn generate(world: &mut World) -> Map {
        let mut tiles = HashMap::new();

        for x in 0..80 {
            for y in 0..50 {
                if x == 0 || x == 79 || y == 0 || y == 49 {
                    let tile = entities::create_wall(world, x, y);
                    tiles.insert((x, y), tile);
                }
            }
        }

        entities::create_player(world, 2, 2);
        entities::create_snake(world, 2, 16);
        entities::create_rat(world, 2, 18);

        Map { tiles }
    }
}
