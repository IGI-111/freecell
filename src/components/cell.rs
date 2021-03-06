use crate::card::{Card, CARD_HEIGHT, CARD_WIDTH};
use crate::game::Collision;
use crate::tileset::{TileParams, TileSet};
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

    pub fn take(&mut self) -> Option<Card> {
        self.card.take()
    }
    pub fn card(&self) -> Option<&Card> {
        self.card.as_ref()
    }
    pub fn put(&mut self, card: Card) {
        self.card = Some(card)
    }
    pub fn is_empty(&self) -> bool {
        self.card.is_none()
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
            .queue_tile(self.card.clone(), self.pos, None::<TileParams>)
            .unwrap();
        Ok(())
    }
}

impl Collision for Cell {
    fn inside(&self, pos: Vector2<i32>) -> bool {
        pos[0] >= self.pos[0]
            && pos[0] <= self.pos[0] + CARD_WIDTH
            && pos[1] >= self.pos[1]
            && pos[1] <= self.pos[1] + CARD_HEIGHT
    }
}
