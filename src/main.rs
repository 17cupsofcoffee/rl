extern crate specs;
extern crate tcod;

mod components;
mod entities;
mod resources;
mod systems;

use tcod::{BackgroundFlag, Console, RootConsole};
use tcod::input::{self, Event, KeyCode};
use tcod::system;
use specs::{Dispatcher, DispatcherBuilder, Join, World};

struct Game<'a> {
    world: World,
    dispatcher: Dispatcher<'a, 'a>,
    root_console: RootConsole,
}

impl<'a> Game<'a> {
    fn new() -> Game<'a> {
        let mut world = World::new();

        world.register::<components::Position>();
        world.register::<components::Sprite>();
        world.register::<components::Tile>();
        world.register::<components::Movement>();
        world.register::<components::Solid>();

        world.add_resource(resources::Input::new());
        world.add_resource(resources::TurnState::new());

        let dispatcher = DispatcherBuilder::new()
            .add(systems::GrantEnergy, "GrantEnergy", &[])
            .add(systems::WaitForInput, "WaitForInput", &["GrantEnergy"])
            .add(systems::PlayerMovement, "PlayerMovement", &["WaitForInput"])
            .add(
                systems::BasicEnemyMovement,
                "BasicEnemyMovement",
                &["WaitForInput"],
            )
            .add(
                systems::ProcessMovement,
                "ProcessMovement",
                &["PlayerMovement", "BasicEnemyMovement"],
            )
            .build();

        entities::create_player(&mut world, 2, 2);
        entities::create_snake(&mut world, 16, 16);

        let mut map = resources::Map::new();

        for x in 0..80 {
            for y in 0..50 {
                if x == 0 || x == 79 || y == 0 || y == 49 {
                    let tile = entities::create_wall(&mut world, x, y);
                    map.tiles.insert((x, y), tile);
                }
            }
        }

        world.add_resource(map);

        let root_console = RootConsole::initializer()
            .size(80, 50)
            .title("Generic Roguelike #7026")
            .init();

        Game {
            world,
            dispatcher,
            root_console,
        }
    }

    fn update(&mut self) {
        while let Some((_, Event::Key(key))) = input::check_for_event(input::KEY) {
            let mut input_state = self.world.write_resource::<resources::Input>();

            match key.code {
                KeyCode::Up => input_state.up = key.pressed,
                KeyCode::Down => input_state.down = key.pressed,
                KeyCode::Left => input_state.left = key.pressed,
                KeyCode::Right => input_state.right = key.pressed,
                _ => {}
            }
        }

        self.dispatcher.dispatch(&self.world.res);
        self.world.maintain();
    }

    fn draw(&mut self) {
        self.root_console.clear();

        let positions = self.world.read::<components::Position>();
        let sprites = self.world.read::<components::Sprite>();
        let tiles = self.world.read::<components::Tile>();

        for (position, tile) in (&positions, &tiles).join() {
            self.root_console.set_char_background(
                position.x,
                position.y,
                tile.color,
                BackgroundFlag::Set,
            );
        }

        for (position, sprite) in (&positions, &sprites).join() {
            self.root_console
                .set_char(position.x, position.y, sprite.character);
            self.root_console
                .set_char_foreground(position.x, position.y, sprite.color);
        }

        self.root_console.flush();
    }
}

fn main() {
    // TODO: Uncouple frame rate from world tick rate
    system::set_fps(30);

    let mut game = Game::new();

    while !game.root_console.window_closed() {
        game.draw();
        game.update();
    }
}
