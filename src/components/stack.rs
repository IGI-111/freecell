use crate::card::Card;
use crate::card::{CARD_HEIGHT, CARD_WIDTH};
use crate::tileset::TileSet;
use crate::Collision;
use ggez::event::EventHandler;
use ggez::{Context, GameResult};
use nalgebra::Vector2;
use std::sync::{Arc, Mutex};

pub struct Stack {
    pos: Vector2<i32>,
    cards: Vec<Card>,
    tileset: Arc<Mutex<TileSet<Option<Card>>>>,
}

impl Stack {
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
}

impl EventHandler<ggez::GameError> for Stack {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.is_empty() {
            self.tileset
                .lock()
                .unwrap()
                .queue_tile(None, self.pos, None::<crate::tileset::TileParams>)
                .unwrap();
        } else {
            self.tileset
                .lock()
                .unwrap()
                .queue_tile(
                    Some(self.cards.last().unwrap().clone()),
                    self.pos,
                    None::<crate::tileset::TileParams>,
                )
                .unwrap();
        }
        Ok(())
    }
}

impl Collision for Stack {
    fn inside(&self, pos: Vector2<i32>) -> bool {
        pos[0] >= self.pos[0]
            && pos[0] <= self.pos[0] + CARD_WIDTH
            && pos[1] >= self.pos[1]
            && pos[1] <= self.pos[1] + CARD_HEIGHT
    }
}
