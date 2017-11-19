use specs::{Fetch, FetchMut, Join, ReadStorage, System, WriteStorage};
use components::{Energy, Player, Position};
use resources::{Input, TurnState};

pub struct GrantEnergy;

impl<'a> System<'a> for GrantEnergy {
    type SystemData = (Fetch<'a, TurnState>, WriteStorage<'a, Energy>);

    fn run(&mut self, (turn_state, mut energy): Self::SystemData) {
        if turn_state.waiting == false {
            for energy in (&mut energy).join() {
                energy.current = (energy.current + 1).min(energy.speed);
            }
        }
    }
}

pub struct WaitForInput;

impl<'a> System<'a> for WaitForInput {
    type SystemData = (
        FetchMut<'a, TurnState>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Energy>,
    );

    fn run(&mut self, (mut turn_state, player_flags, energy): Self::SystemData) {
        if turn_state.waiting == false {
            for (_, energy) in (&player_flags, &energy).join() {
                if energy.current >= energy.speed {
                    turn_state.waiting = true;
                }
            }
        }
    }
}

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        Fetch<'a, Input>,
        FetchMut<'a, TurnState>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Energy>,
    );

    fn run(
        &mut self,
        (input, mut turn_state, player_flags, mut positions, mut energy): Self::SystemData,
    ) {
        if turn_state.waiting == true {
            for (_, position, energy) in (&player_flags, &mut positions, &mut energy).join() {
                if input.up {
                    position.y = (position.y - 1).max(0);
                    energy.current = 0;
                    turn_state.waiting = false;
                }

                if input.down {
                    position.y = (position.y + 1).min(49);
                    energy.current = 0;
                    turn_state.waiting = false;
                }

                if input.left {
                    position.x = (position.x - 1).max(0);
                    energy.current = 0;
                    turn_state.waiting = false;
                }

                if input.right {
                    position.x = (position.x + 1).min(79);
                    energy.current = 0;
                    turn_state.waiting = false;
                }
            }
        }
    }
}
