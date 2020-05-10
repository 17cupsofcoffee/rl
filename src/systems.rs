use hecs::World;

use crate::components::{MoveAction, Movement, Position, Solid};
use crate::resources::Resources;

pub fn grant_energy(world: &mut World, resources: &mut Resources) {
    if !resources.turn_state.waiting {
        let mut player_ready = false;

        for (_, movement) in world.query::<&mut Movement>().iter() {
            movement.energy = i32::min(movement.energy + 1, movement.speed);

            if movement.player_input && movement.ready() {
                player_ready = true;
            }
        }

        resources.turn_state.waiting = player_ready;
    }
}

pub fn player_movement(world: &mut World, resources: &mut Resources) {
    for (_, movement) in world.query::<&mut Movement>().iter() {
        if movement.player_input && movement.ready() {
            if resources.input.up {
                movement.move_queue.push_back(MoveAction::Up);
            }

            if resources.input.down {
                movement.move_queue.push_back(MoveAction::Down);
            }

            if resources.input.left {
                movement.move_queue.push_back(MoveAction::Left);
            }

            if resources.input.right {
                movement.move_queue.push_back(MoveAction::Right);
            }
        }
    }
}

pub fn basic_enemy_movement(world: &mut World, _: &mut Resources) {
    for (_, movement) in world.query::<&mut Movement>().iter() {
        if !movement.player_input && movement.ready() {
            movement.move_queue.push_back(MoveAction::Right);
        }
    }
}

pub fn process_movement(world: &mut World, resources: &mut Resources) {
    for (_, (movement, position)) in world.query::<(&mut Movement, &mut Position)>().iter() {
        let (target_x, target_y) = match movement.move_queue.pop_front() {
            Some(MoveAction::Up) => (position.x, position.y - 1),
            Some(MoveAction::Down) => (position.x, position.y + 1),
            Some(MoveAction::Left) => (position.x - 1, position.y),
            Some(MoveAction::Right) => (position.x + 1, position.y),
            _ => continue,
        };

        if let Some(tile_id) = resources.map.tiles.get(&(target_x, target_y)) {
            if world.get::<Solid>(*tile_id).is_ok() {
                // move blocked
                continue;
            }
        }

        position.x = target_x;
        position.y = target_y;
        movement.energy = 0;

        if movement.player_input {
            resources.turn_state.waiting = false;
        }
    }
}
