use crate::card::Card;
use crate::tileset::TileSet;
use ggez::event::EventHandler;
use ggez::{Context, GameResult};
use nalgebra::Vector2;
use std::sync::{Arc, Mutex};

pub struct Cell {
    pos: Vector2<i32>,
    card: Option<Card>,
    tileset: Arc<Mutex<TileSet<Option<Card>>>>,
}

impl Cell {
    pub fn new(
        pos: Vector2<i32>,
        card: Option<Card>,
        tileset: Arc<Mutex<TileSet<Option<Card>>>>,
    ) -> Self {
        Self { pos, card, tileset }
    }
}

impl EventHandler<ggez::GameError> for Cell {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.tileset
            .lock()
            .unwrap()
            .queue_tile(
                self.card.clone(),
                self.pos,
                None::<crate::tileset::TileParams>,
            )
            .unwrap();
        Ok(())
    }
}
