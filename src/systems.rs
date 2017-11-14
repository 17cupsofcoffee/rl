use specs::{Fetch, Join, ReadStorage, System, WriteStorage};
use components::{Player, Position};
use resources::Input;

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Fetch<'a, Input>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (input, player_flags, mut positions): Self::SystemData) {
        for (_, position) in (&player_flags, &mut positions).join() {
            if input.up {
                position.y = (position.y - 1).max(0);
            }

            if input.down {
                position.y = (position.y + 1).min(49);
            }

            if input.left {
                position.x = (position.x - 1).max(0);
            }

            if input.right {
                position.x = (position.x + 1).min(79);
            }
        }
    }
}
