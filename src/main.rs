mod components;
mod console;
mod entities;
mod resources;
mod systems;

use specs::{Dispatcher, DispatcherBuilder, Join, World};
use tetra::graphics::color;
use tetra::graphics::{self, Texture};
use tetra::input::{self, Key};
use tetra::{Context, ContextBuilder, State};

use crate::console::Console;

struct GameState<'a> {
    world: World,
    dispatcher: Dispatcher<'a, 'a>,
    console: Console,
}

impl<'a> GameState<'a> {
    fn new(ctx: &mut Context) -> tetra::Result<GameState<'a>> {
        let mut world = World::new();

        world.register::<components::Position>();
        world.register::<components::Sprite>();
        world.register::<components::Tile>();
        world.register::<components::Movement>();
        world.register::<components::Solid>();

        world.add_resource(resources::Input::new());
        world.add_resource(resources::TurnState::new());

        let dispatcher = DispatcherBuilder::new()
            .with(systems::WaitForInput, "WaitForInput", &[])
            .with(systems::GrantEnergy, "GrantEnergy", &["WaitForInput"])
            .with(systems::PlayerMovement, "PlayerMovement", &["GrantEnergy"])
            .with(
                systems::BasicEnemyMovement,
                "BasicEnemyMovement",
                &["WaitForInput"],
            )
            .with(
                systems::ProcessMovement,
                "ProcessMovement",
                &["PlayerMovement", "BasicEnemyMovement"],
            )
            .build();

        let font = Texture::new(ctx, "./resources/terminal.png")?;
        let console = Console::new(font);

        Ok(GameState {
            world,
            dispatcher,
            console,
        })
    }

    fn generate_map(&mut self) {
        self.world.delete_all();

        let mut map = resources::Map::new();

        for x in 0..80 {
            for y in 0..50 {
                if x == 0 || x == 79 || y == 0 || y == 49 {
                    let tile = entities::create_wall(&mut self.world, x, y);
                    map.tiles.insert((x, y), tile);
                }
            }
        }

        entities::create_player(&mut self.world, 2, 2);
        entities::create_snake(&mut self.world, 2, 16);
        entities::create_rat(&mut self.world, 2, 18);

        self.world.add_resource(map);
    }
}

impl<'a> State for GameState<'a> {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        {
            let mut input_state = self.world.write_resource::<resources::Input>();

            input_state.up = input::is_key_down(ctx, Key::Up) || input::is_key_down(ctx, Key::W);
            input_state.down =
                input::is_key_down(ctx, Key::Down) || input::is_key_down(ctx, Key::S);
            input_state.left =
                input::is_key_down(ctx, Key::Left) || input::is_key_down(ctx, Key::A);
            input_state.right =
                input::is_key_down(ctx, Key::Right) || input::is_key_down(ctx, Key::D);
        }

        self.dispatcher.dispatch(&self.world.res);
        self.world.maintain();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result {
        graphics::clear(ctx, color::BLACK);

        self.console.clear();

        let positions = self.world.read_storage::<components::Position>();
        let sprites = self.world.read_storage::<components::Sprite>();
        let tiles = self.world.read_storage::<components::Tile>();

        for (position, tile) in (&positions, &tiles).join() {
            self.console.set_bg(position.x, position.y, tile.color);
        }

        for (position, sprite) in (&positions, &sprites).join() {
            self.console
                .set_char(position.x, position.y, sprite.character, sprite.color);
        }

        self.console.draw(ctx);

        Ok(())
    }
}

fn main() -> tetra::Result {
    let ctx = &mut ContextBuilder::new("Generic Roguelike #7026", 80 * 8, 50 * 8)
        .tick_rate(30.0)
        .maximized(true)
        .resizable(true)
        .quit_on_escape(true)
        .build()?;

    let state = &mut GameState::new(ctx)?;

    state.generate_map();

    ctx.run(state)
}
