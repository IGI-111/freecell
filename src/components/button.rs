use crate::game::Collision;
use ggez::event::EventHandler;
use ggez::graphics::{DrawParam, Drawable, Image};
use ggez::{Context, GameResult};
use nalgebra::{point, Vector2};

pub struct Button {
    pos: Vector2<i32>,
    image: Image,
}

impl Button {
    pub fn new(ctx: &mut Context, pos: Vector2<i32>) -> Self {
        let image = Image::new(ctx, "/icon.png").unwrap();
        Self { image, pos }
    }
}

impl EventHandler<ggez::GameError> for Button {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let pos = point![self.pos[0] as f32, self.pos[1] as f32];
        self.image.draw(ctx, DrawParam::default().dest(pos))
    }
}

impl Collision for Button {
    fn inside(&self, pos: Vector2<i32>) -> bool {
        let dim = self.image.dimensions();

        pos[0] >= self.pos[0]
            && pos[0] <= self.pos[0] + dim.w as i32
            && pos[1] >= self.pos[1]
            && pos[1] <= self.pos[1] + dim.h as i32
    }
}
