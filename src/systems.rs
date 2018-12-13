use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

use components::{MoveAction, Movement, Position, Solid};
use resources::{Input, Map, TurnState};

pub struct GrantEnergy;

impl<'a> System<'a> for GrantEnergy {
    type SystemData = (Read<'a, TurnState>, WriteStorage<'a, Movement>);

    fn run(&mut self, (turn_state, mut movements): Self::SystemData) {
        if !turn_state.waiting {
            for movement in (&mut movements).join() {
                movement.energy = (movement.energy + 1).min(movement.speed);
            }
        }
    }
}

pub struct WaitForInput;

impl<'a> System<'a> for WaitForInput {
    type SystemData = (Write<'a, TurnState>, ReadStorage<'a, Movement>);

    fn run(&mut self, (mut turn_state, movements): Self::SystemData) {
        turn_state.waiting = (&movements).join().any(|m| m.player_input && m.ready());
    }
}

pub struct PlayerMovement;

impl<'a> System<'a> for PlayerMovement {
    type SystemData = (Read<'a, Input>, WriteStorage<'a, Movement>);

    fn run(&mut self, (input, mut movements): Self::SystemData) {
        for movement in (&mut movements).join() {
            if movement.player_input && movement.ready() {
                if input.up {
                    movement.move_queue.push_back(MoveAction::Up);
                }

                if input.down {
                    movement.move_queue.push_back(MoveAction::Down);
                }

                if input.left {
                    movement.move_queue.push_back(MoveAction::Left);
                }

                if input.right {
                    movement.move_queue.push_back(MoveAction::Right);
                }
            }
        }
    }
}

pub struct BasicEnemyMovement;

impl<'a> System<'a> for BasicEnemyMovement {
    type SystemData = WriteStorage<'a, Movement>;

    fn run(&mut self, mut movements: Self::SystemData) {
        for movement in (&mut movements).join() {
            if !movement.player_input && movement.ready() {
                movement.move_queue.push_back(MoveAction::Right);
            }
        }
    }
}

pub struct ProcessMovement;

impl<'a> System<'a> for ProcessMovement {
    type SystemData = (
        Write<'a, TurnState>,
        Read<'a, Map>,
        WriteStorage<'a, Movement>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Solid>,
    );

    fn run(
        &mut self,
        (mut turn_state, map, mut movements, mut positions, solid_tiles): Self::SystemData,
    ) {
        for (movement, position) in (&mut movements, &mut positions).join() {
            let (target_x, target_y) = match movement.move_queue.front() {
                Some(&MoveAction::Up) => (position.x, position.y - 1),
                Some(&MoveAction::Down) => (position.x, position.y + 1),
                Some(&MoveAction::Left) => (position.x - 1, position.y),
                Some(&MoveAction::Right) => (position.x + 1, position.y),
                _ => continue,
            };

            movement.move_queue.pop_front();

            if let Some(tile_id) = map.tiles.get(&(target_x, target_y)) {
                if solid_tiles.get(*tile_id).is_some() {
                    // move blocked
                    continue;
                }
            }

            position.x = target_x;
            position.y = target_y;
            movement.energy = 0;

            if movement.player_input {
                turn_state.waiting = false;
            }
        }
    }
}
