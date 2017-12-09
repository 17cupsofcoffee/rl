use specs::{Fetch, FetchMut, Join, ReadStorage, System, WriteStorage};
use components::{Enemy, MoveAction, Movement, Player, Position, Solid};
use resources::{Input, Map, TurnState};

pub struct GrantEnergy;

impl<'a> System<'a> for GrantEnergy {
    type SystemData = (Fetch<'a, TurnState>, WriteStorage<'a, Movement>);

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
    type SystemData = (
        FetchMut<'a, TurnState>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movement>,
    );

    fn run(&mut self, (mut turn_state, player_flags, movements): Self::SystemData) {
        if !turn_state.waiting {
            for (_, movement) in (&player_flags, &movements).join() {
                if movement.ready() {
                    turn_state.waiting = true;
                }
            }
        }
    }
}

pub struct PlayerMovement;

impl<'a> System<'a> for PlayerMovement {
    type SystemData = (
        Fetch<'a, Input>,
        FetchMut<'a, TurnState>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Movement>,
    );

    fn run(&mut self, (input, mut turn_state, player_flags, mut movements): Self::SystemData) {
        if turn_state.waiting {
            for (_, movement) in (&player_flags, &mut movements).join() {
                if input.up {
                    movement.move_queue.push_back(MoveAction::Up);
                    turn_state.waiting = false;
                }

                if input.down {
                    movement.move_queue.push_back(MoveAction::Down);
                    turn_state.waiting = false;
                }

                if input.left {
                    movement.move_queue.push_back(MoveAction::Left);
                    turn_state.waiting = false;
                }

                if input.right {
                    movement.move_queue.push_back(MoveAction::Right);
                    turn_state.waiting = false;
                }
            }
        }
    }
}

pub struct BasicEnemyMovement;

impl<'a> System<'a> for BasicEnemyMovement {
    type SystemData = (
        Fetch<'a, TurnState>,
        ReadStorage<'a, Enemy>,
        WriteStorage<'a, Movement>,
    );

    fn run(&mut self, (turn_state, enemy_flags, mut movements): Self::SystemData) {
        if !turn_state.waiting {
            for (_, movement) in (&enemy_flags, &mut movements).join() {
                if movement.ready() {
                    movement.move_queue.push_back(MoveAction::Right);
                }
            }
        }
    }
}

pub struct ProcessMovement;

impl<'a> System<'a> for ProcessMovement {
    type SystemData = (
        Fetch<'a, TurnState>,
        Fetch<'a, Map>,
        WriteStorage<'a, Movement>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Solid>,
    );

    fn run(
        &mut self,
        (turn_state, map, mut movements, mut positions, solid_tiles): Self::SystemData,
    ) {
        if !turn_state.waiting {
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
                    if let Some(_) = solid_tiles.get(*tile_id) {
                        // move blocked
                        continue;
                    }
                }

                position.x = target_x;
                position.y = target_y;
                movement.energy = 0;
            }
        }
    }
}
