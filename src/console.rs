use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Image, Point2, Rect};

pub struct Console {
    font: Image,
    cell_size: i32,
}

impl Console {
    pub fn new(font: Image) -> Console {
        let cell_size = (font.width() / 16) as i32;

        Console { font, cell_size }
    }

    pub fn clear(&self, ctx: &mut Context) {
        graphics::clear(ctx);
        graphics::set_background_color(ctx, graphics::BLACK);
    }

    pub fn set_char(
        &self,
        ctx: &mut Context,
        c: char,
        x: i32,
        y: i32,
        color: Color,
    ) -> GameResult<()> {
        let codepoint = c as u8;
        let sprite_y = codepoint % 16;
        let sprite_x = (codepoint - sprite_y) / 16;

        graphics::set_color(ctx, color)?;
        graphics::draw_ex(
            ctx,
            &self.font,
            DrawParam {
                src: Rect::new(
                    sprite_x as f32 * 0.0625,
                    sprite_y as f32 * 0.0625,
                    0.0625,
                    0.0625,
                ),
                dest: Point2::new(
                    (x * self.cell_size + 4) as f32,
                    (y * self.cell_size + 4) as f32,
                ),

                ..Default::default()
            },
        )
    }

    pub fn set_bg(&self, ctx: &mut Context, x: i32, y: i32, color: Color) -> GameResult<()> {
        graphics::set_color(ctx, color)?;
        graphics::rectangle(
            ctx,
            DrawMode::Fill,
            Rect::new(
                (x * self.cell_size + 4) as f32,
                (y * self.cell_size + 4) as f32,
                self.cell_size as f32,
                self.cell_size as f32,
            ),
        )
    }

    pub fn present(&self, ctx: &mut Context) {
        graphics::present(ctx);
    }
}
