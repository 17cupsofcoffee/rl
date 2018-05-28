use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::{self, Color, DrawParam, Image, Point2, Rect};
use ggez::{Context, GameResult};

#[derive(Clone)]
struct ConsoleCell {
    glyph: char,
    foreground: Color,
    background: Color,
}

pub struct Console {
    spritebatch: SpriteBatch,
    cells: Vec<ConsoleCell>,
    cell_size: f32,
}

impl Console {
    pub fn new(font: Image) -> Console {
        let cell_size = (font.width() / 16) as f32;

        Console {
            spritebatch: SpriteBatch::new(font),
            cells: vec![
                ConsoleCell {
                    glyph: ' ',
                    foreground: Color::new(1.0, 1.0, 1.0, 1.0),
                    background: Color::new(0.0, 0.0, 0.0, 1.0),
                };
                80 * 50
            ],
            cell_size,
        }
    }

    pub fn clear(&mut self) {
        self.cells = vec![
            ConsoleCell {
                glyph: ' ',
                foreground: Color::new(1.0, 1.0, 1.0, 1.0),
                background: Color::new(0.0, 0.0, 0.0, 1.0),
            };
            80 * 50
        ];
    }

    pub fn set_char(&mut self, x: i32, y: i32, glyph: char, color: Color) {
        let cell = &mut self.cells[(x + 80 * y) as usize];
        cell.glyph = glyph;
        cell.foreground = color;
    }

    pub fn set_bg(&mut self, x: i32, y: i32, color: Color) {
        self.cells[(x + 80 * y) as usize].background = color;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for (i, cell) in self.cells.iter().enumerate() {
            let (x, y) = (i % 80, i / 80);

            let sprite_x = 219 / 16;
            let sprite_y = 219 % 16;

            self.spritebatch.add(DrawParam {
                src: Rect::new(
                    sprite_x as f32 * 0.0625,
                    sprite_y as f32 * 0.0625,
                    0.0625,
                    0.0625,
                ),
                dest: Point2::new(self.cell_size * x as f32, self.cell_size * y as f32),
                color: Some(cell.background),

                ..Default::default()
            });

            let codepoint = cell.glyph as u8;
            let sprite_x = codepoint / 16;
            let sprite_y = codepoint % 16;

            self.spritebatch.add(DrawParam {
                src: Rect::new(
                    f32::from(sprite_x) * 0.0625,
                    f32::from(sprite_y) * 0.0625,
                    0.0625,
                    0.0625,
                ),
                dest: Point2::new(self.cell_size * x as f32, self.cell_size * y as f32),
                color: Some(cell.foreground),

                ..Default::default()
            });
        }

        graphics::draw(ctx, &self.spritebatch, Point2::new(0.0, 0.0), 0.0)?;
        self.spritebatch.clear();
        Ok(())
    }
}
