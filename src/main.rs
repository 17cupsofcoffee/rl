mod components;
mod console;
mod entities;
mod resources;
mod systems;

use hecs::World;
use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::time::Timestep;
use tetra::{Context, ContextBuilder, State};

use crate::console::Console;
use crate::resources::Resources;

struct GameState {
    world: World,
    resources: Resources,
    console: Console,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let mut world = World::new();
        let resources = Resources::new(&mut world);

        let font = Texture::new(ctx, "./resources/terminal.png")?;
        let console = Console::new(font);

        Ok(GameState {
            world,
            resources,
            console,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.resources.input.up =
            input::is_key_down(ctx, Key::Up) || input::is_key_down(ctx, Key::W);
        self.resources.input.down =
            input::is_key_down(ctx, Key::Down) || input::is_key_down(ctx, Key::S);
        self.resources.input.left =
            input::is_key_down(ctx, Key::Left) || input::is_key_down(ctx, Key::A);
        self.resources.input.right =
            input::is_key_down(ctx, Key::Right) || input::is_key_down(ctx, Key::D);

        systems::grant_energy(&mut self.world, &mut self.resources);
        systems::player_movement(&mut self.world, &mut self.resources);
        systems::basic_enemy_movement(&mut self.world, &mut self.resources);
        systems::process_movement(&mut self.world, &mut self.resources);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);

        self.console.clear();

        let mut tile_query = self
            .world
            .query::<(&components::Position, &components::Tile)>();

        for (_, (position, tile)) in tile_query.iter() {
            self.console.set_bg(position.x, position.y, tile.color);
        }

        let mut sprite_query = self
            .world
            .query::<(&components::Position, &components::Sprite)>();

        for (_, (position, sprite)) in sprite_query.iter() {
            self.console
                .set_char(position.x, position.y, sprite.character, sprite.color);
        }

        self.console.draw(ctx);

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Generic Roguelike #7026", 80 * 8, 50 * 8)
        .timestep(Timestep::Fixed(30.0))
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
