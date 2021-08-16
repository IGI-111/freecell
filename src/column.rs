use crate::card::Card;
use crate::card::CARD_WIDTH;
use crate::tileset::TileSet;
use ggez::event::EventHandler;
use ggez::{Context, GameResult};
use nalgebra::{point, Vector2};
use std::sync::{Arc, Mutex};

pub struct Column {
    pos: Vector2<i32>,
    cards: Vec<Card>,
    tileset: Arc<Mutex<TileSet<Option<Card>>>>,
}

impl Column {
    pub fn new(
        pos: Vector2<i32>,
        cards: Vec<Card>,
        tileset: Arc<Mutex<TileSet<Option<Card>>>>,
    ) -> Self {
        Self {
            pos,
            cards,
            tileset,
        }
    }
}

impl EventHandler<ggez::GameError> for Column {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for (y, card) in self.cards.iter().cloned().enumerate() {
            self.tileset
                .lock()
                .unwrap()
                .queue_tile(
                    Some(card),
                    point![0, (y as i32 * (CARD_WIDTH / 3))] + self.pos,
                    None::<crate::tileset::TileParams>,
                )
                .unwrap();
        }
        Ok(())
    }
}
