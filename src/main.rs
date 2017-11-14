extern crate ggez;
extern crate specs;

mod components;
mod console;
mod resources;
mod systems;

use std::env;
use std::path::PathBuf;
use ggez::{Context, GameResult};
use ggez::conf::Conf;
use ggez::event::{self, EventHandler, Keycode, Mod};
use ggez::graphics::{self, Color, Image};
use ggez::timer;
use specs::{Dispatcher, DispatcherBuilder, Entity, Join, World};
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

        world.add_resource(resources::Input {
            x_axis: 0.0,
            y_axis: 0.0,
        });

        let dispatcher = DispatcherBuilder::new()
            .add(systems::MovementSystem, "movement", &[])
            .build();

        world
            .create_entity()
            .with(components::Position { x: 0, y: 0 })
            .with(components::Sprite {
                character: '@',
                color: graphics::WHITE,
            })
            .with(components::Player)
            .build();

        world
            .create_entity()
            .with(components::Position { x: 16, y: 16 })
            .with(components::Sprite {
                character: 'S',
                color: Color::new(0.0, 1.0, 0.0, 1.0),
            })
            .build();

        // This is fairly dire
        State::create_wall(&mut world, 1, 1);

        let font = Image::new(ctx, "/terminal.png")?;
        let console = Console::new(font);

        Ok(State {
            world,
            dispatcher,
            console,
        })
    }

    fn create_wall(world: &mut World, x: i32, y: i32) -> Entity {
        world
            .create_entity()
            .with(components::Position { x, y })
            .with(components::Tile {
                color: Color::new(0.1, 0.1, 0.1, 1.0),
            })
            .build()
    }
}

impl<'a> EventHandler for State<'a> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.dispatcher.dispatch(&self.world.res);
        self.world.maintain();

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
            Keycode::Up => input.y_axis = -1.0,
            Keycode::Down => input.y_axis = 1.0,
            Keycode::Left => input.x_axis = -1.0,
            Keycode::Right => input.x_axis = 1.0,
            Keycode::Escape => ctx.quit().unwrap(),
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let mut input = self.world.write_resource::<resources::Input>();

        match keycode {
            Keycode::Up | Keycode::Down => input.y_axis = 0.0,
            Keycode::Left | Keycode::Right => input.x_axis = 0.0,
            _ => (),
        }
    }
}

fn run() -> GameResult<()> {
    let conf = Conf {
        window_title: "Generic Roguelike #7026".to_string(),
        window_width: 80 * 8,
        window_height: 50 * 8,

        ..Default::default()
    };

    let ctx = &mut Context::load_from_conf("rl", "17cupsofcoffee", conf)?;

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
        println!("Adding path {:?}", path);
    }

    let state = &mut State::new(ctx)?;

    event::run(ctx, state)
}

fn main() {
    match run() {
        Ok(_) => println!("Game exited cleanly."),
        Err(e) => println!("Error: {}", e),
    }
}
