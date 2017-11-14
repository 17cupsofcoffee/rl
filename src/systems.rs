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
            position.x = (position.x + input.x_axis as i32).min(79).max(0);
            position.y = (position.y + input.y_axis as i32).min(49).max(0);
        }
    }
}
