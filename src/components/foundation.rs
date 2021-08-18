use crate::card::{Card, CARD_HEIGHT, CARD_WIDTH};
use crate::game::Collision;
use crate::tileset::{TileParams, TileSet};
use ggez::event::EventHandler;
use ggez::{Context, GameResult};
use nalgebra::Vector2;
use std::sync::{Arc, Mutex};

pub struct Foundation {
    pos: Vector2<i32>,
    cards: Vec<Card>,
    tileset: Arc<Mutex<TileSet<Option<Card>>>>,
}

impl Foundation {
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

    pub fn take(&mut self) -> Option<Card> {
        self.cards.pop()
    }
    pub fn put(&mut self, card: Card) {
        self.cards.push(card);
    }
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
    pub fn top_card(&self) -> Option<&Card> {
        self.cards.last()
    }
    pub fn can_stack(&self, card: &Card) -> bool {
        match self.top_card() {
            Some(stack_top_card) => card.follows(stack_top_card),
            None => card.is_ace(),
        }
    }
}

impl EventHandler<ggez::GameError> for Foundation {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.is_empty() {
            self.tileset
                .lock()
                .unwrap()
                .queue_tile(None, self.pos, None::<TileParams>)
                .unwrap();
        } else {
            self.tileset
                .lock()
                .unwrap()
                .queue_tile(
                    Some(self.cards.last().unwrap().clone()),
                    self.pos,
                    None::<TileParams>,
                )
                .unwrap();
        }
        Ok(())
    }
}

impl Collision for Foundation {
    fn inside(&self, pos: Vector2<i32>) -> bool {
        pos[0] >= self.pos[0]
            && pos[0] <= self.pos[0] + CARD_WIDTH
            && pos[1] >= self.pos[1]
            && pos[1] <= self.pos[1] + CARD_HEIGHT
    }
}
