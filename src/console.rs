use tetra::graphics::{Color, DrawParams, Rectangle, Texture};
use tetra::math::Vec2;
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
    width: usize,
    height: usize,
}

impl Console {
    pub fn new(font: Texture, width: usize, height: usize) -> Console {
        let cell_size = (font.width() / 16) as f32;

        Console {
            font,
            cells: vec![
                ConsoleCell {
                    glyph: ' ',
                    foreground: Color::rgb(1.0, 1.0, 1.0),
                    background: Color::rgb(0.0, 0.0, 0.0),
                };
                width * height
            ],
            cell_size,
            width,
            height,
        }
    }

    pub fn clear(&mut self) {
        self.cells = vec![
            ConsoleCell {
                glyph: ' ',
                foreground: Color::rgb(1.0, 1.0, 1.0),
                background: Color::rgb(0.0, 0.0, 0.0),
            };
            self.width * self.height
        ];
    }

    pub fn set_char(&mut self, x: usize, y: usize, glyph: char) {
        self.cells[x + y * self.width].glyph = glyph;
    }

    pub fn set_fg(&mut self, x: usize, y: usize, color: Color) {
        self.cells[x + y * self.width].foreground = color;
    }

    pub fn set_bg(&mut self, x: usize, y: usize, color: Color) {
        self.cells[x + y * self.width].background = color;
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        for (i, cell) in self.cells.iter().enumerate() {
            let (x, y) = (i % self.width, i / self.width);
            let sprite_x = (219 / 16) as f32 * 8.0;
            let sprite_y = (219 % 16) as f32 * 8.0;

            if cell.background != Color::BLACK {
                self.font.draw_region(
                    ctx,
                    Rectangle::new(sprite_x, sprite_y, 8.0, 8.0),
                    DrawParams::new()
                        .position(Vec2::new(
                            self.cell_size * x as f32,
                            self.cell_size * y as f32,
                        ))
                        .color(cell.background),
                );
            }

            if cell.glyph != ' ' {
                let codepoint = cell.glyph as u8;
                let sprite_x = f32::from(codepoint / 16) * 8.0;
                let sprite_y = f32::from(codepoint % 16) * 8.0;

                self.font.draw_region(
                    ctx,
                    Rectangle::new(sprite_x, sprite_y, 8.0, 8.0),
                    DrawParams::new()
                        .position(Vec2::new(
                            self.cell_size * x as f32,
                            self.cell_size * y as f32,
                        ))
                        .color(cell.foreground),
                );
            }
        }
    }
}
