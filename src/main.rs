extern crate ggez;
extern crate specs;

mod components;
mod console;
mod entities;
mod resources;
mod systems;

use std::env;
use std::path::PathBuf;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf::{WindowSetup, WindowMode};
use ggez::event::{self, EventHandler, Keycode, Mod};
use ggez::graphics::Image;
use ggez::timer;
use specs::{Dispatcher, DispatcherBuilder, Join, World};
use console::Console;

struct State<'a> {
    world: World,
    dispatcher: Dispatcher<'a, 'a>,
    console: Console,
}

impl<'a> State<'a> {
    fn new(ctx: &mut Context) -> GameResult<State<'a>> {
        let mut world = World::new();

        world.register::<components::Position>();
        world.register::<components::Sprite>();
        world.register::<components::Tile>();
        world.register::<components::Player>();
        world.register::<components::Movement>();
        world.register::<components::Enemy>();

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

        for x in 0..80 {
            for y in 0..50 {
                if x == 0 || x == 79 || y == 0 || y == 49 {
                    entities::create_wall(&mut world, x, y);
                }
            }
        }

        let font = Image::new(ctx, "/terminal.png")?;
        let console = Console::new(font);

        Ok(State {
            world,
            dispatcher,
            console,
        })
    }
}

impl<'a> EventHandler for State<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.dispatcher.dispatch(&self.world.res);
            self.world.maintain();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.console.clear(ctx);

        let positions = self.world.read::<components::Position>();
        let sprites = self.world.read::<components::Sprite>();
        let tiles = self.world.read::<components::Tile>();

        for (position, tile) in (&positions, &tiles).join() {
            self.console
                .set_bg(ctx, position.x, position.y, tile.color)?;
        }

        for (position, sprite) in (&positions, &sprites).join() {
            self.console
                .set_char(ctx, sprite.character, position.x, position.y, sprite.color)?;
        }

        self.console.present(ctx);

        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let mut input = self.world.write_resource::<resources::Input>();

        match keycode {
            Keycode::Up => input.up = true,
            Keycode::Down => input.down = true,
            Keycode::Left => input.left = true,
            Keycode::Right => input.right = true,
            Keycode::Escape => ctx.quit().unwrap(),
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let mut input = self.world.write_resource::<resources::Input>();

        match keycode {
            Keycode::Up => input.up = false,
            Keycode::Down => input.down = false,
            Keycode::Left => input.left = false,
            Keycode::Right => input.right = false,
            _ => (),
        }
    }
}

fn run() -> GameResult<()> {
    let mut cb = ContextBuilder::new("rl", "17cupsofcoffee")
        .window_setup(WindowSetup::default().title("Generic Roguelike #7026"))
        .window_mode(WindowMode::default().dimensions(80 * 8, 50 * 8));

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }

    let ctx = &mut cb.build()?;
    let state = &mut State::new(ctx)?;

    event::run(ctx, state)
}

fn main() {
    match run() {
        Ok(_) => println!("Game exited cleanly."),
        Err(e) => println!("Error: {}", e),
    }
}
