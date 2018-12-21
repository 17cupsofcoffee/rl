use tetra::glm::Vec2;
use tetra::graphics::{self, Color, DrawParams, Rectangle, Texture};
use tetra::graphics::color;
use tetra::Context;

#[derive(Clone)]
struct ConsoleCell {
    glyph: char,
    foreground: Color,
    background: Color,
}

pub struct Console {
    font: Texture,
    cells: Vec<ConsoleCell>,
    cell_size: f32,
}

impl Console {
    pub fn new(font: Texture) -> Console {
        let cell_size = (font.width() / 16) as f32;

        Console {
            font,
            cells: vec![
                ConsoleCell {
                    glyph: ' ',
                    foreground: Color::rgb(1.0, 1.0, 1.0),
                    background: Color::rgb(0.0, 0.0, 0.0),
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
                foreground: Color::rgb(1.0, 1.0, 1.0),
                background: Color::rgb(0.0, 0.0, 0.0),
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

    pub fn draw(&mut self, ctx: &mut Context) {
        for (i, cell) in self.cells.iter().enumerate() {
            let (x, y) = (i % 80, i / 80);
            let sprite_x = (219 / 16) as f32 * 8.0;
            let sprite_y = (219 % 16) as f32 * 8.0;

            if cell.background != color::BLACK {
                graphics::draw(
                    ctx,
                    &self.font,
                    DrawParams::new()
                        .position(Vec2::new(
                            self.cell_size * x as f32,
                            self.cell_size * y as f32,
                        ))
                        .color(cell.background)
                        .clip(Rectangle::new(sprite_x, sprite_y, 8.0, 8.0)),
                );
            }

            if cell.glyph != ' ' {
                let codepoint = cell.glyph as u8;
                let sprite_x = f32::from(codepoint / 16) * 8.0;
                let sprite_y = f32::from(codepoint % 16) * 8.0;

                graphics::draw(
                    ctx,
                    &self.font,
                    DrawParams::new()
                        .position(Vec2::new(
                            self.cell_size * x as f32,
                            self.cell_size * y as f32,
                        ))
                        .color(cell.foreground)
                        .clip(Rectangle::new(sprite_x, sprite_y, 8.0, 8.0)),
                );
            }
        }
    }
}
