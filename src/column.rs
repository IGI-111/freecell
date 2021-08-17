use crate::card::Card;
use crate::card::{CARD_HEIGHT, CARD_WIDTH};
use crate::tileset::TileSet;
use crate::Collision;
use ggez::event::EventHandler;
use ggez::input;
use ggez::{Context, GameResult};
use nalgebra::{point, vector, Vector2};
use std::sync::{Arc, Mutex};

const CARD_STACK_INCREMENT: i32 = CARD_HEIGHT / 4;

pub struct Column {
    pos: Vector2<i32>,
    cards: Vec<Card>,
    tileset: Arc<Mutex<TileSet<Option<Card>>>>,
    follow_cursor: bool,
}

impl Column {
    pub fn new(
        pos: Vector2<i32>,
        cards: Vec<Card>,
        tileset: Arc<Mutex<TileSet<Option<Card>>>>,
        follow_cursor: bool,
    ) -> Self {
        Self {
            pos,
            cards,
            tileset,
            follow_cursor,
        }
    }

    pub fn take(&mut self, n: usize) -> Vec<Card> {
        if self.is_empty() {
            vec![]
        } else {
            self.cards.split_off(self.cards.len() - n)
        }
    }
    pub fn put(&mut self, mut cards: Vec<Card>) {
        self.cards.append(&mut cards);
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

impl EventHandler<ggez::GameError> for Column {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.follow_cursor {
            let cursor_pos = input::mouse::position(ctx);
            self.pos = vector![
                cursor_pos.x as i32 - CARD_WIDTH / 2,
                cursor_pos.y as i32 - CARD_HEIGHT / 2
            ];
        }
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.is_empty() && !self.follow_cursor {
            self.tileset
                .lock()
                .unwrap()
                .queue_tile(None, self.pos, None::<crate::tileset::TileParams>)
                .unwrap();
        } else {
            for (y, card) in self.cards.iter().cloned().enumerate() {
                self.tileset
                    .lock()
                    .unwrap()
                    .queue_tile(
                        Some(card),
                        point![0, (y as i32 * CARD_STACK_INCREMENT)] + self.pos,
                        None::<crate::tileset::TileParams>,
                    )
                    .unwrap();
            }
        }
        Ok(())
    }
}

impl Collision for Column {
    fn inside(&self, pos: Vector2<i32>) -> bool {
        let height = if self.cards.is_empty() {
            CARD_HEIGHT
        } else {
            (self.cards.len() as i32 - 1) * CARD_STACK_INCREMENT + CARD_HEIGHT
        };

        pos[0] >= self.pos[0]
            && pos[0] <= self.pos[0] + CARD_WIDTH
            && pos[1] >= self.pos[1]
            && pos[1] <= self.pos[1] + height
    }
}
