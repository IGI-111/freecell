use crate::card::{Card, CARD_HEIGHT, CARD_WIDTH};
use crate::game::Collision;
use crate::tileset::{TileParams, TileSet};
use ggez::event::EventHandler;
use ggez::input;
use ggez::{Context, GameResult};
use nalgebra::{point, vector, Vector2};
use std::sync::{Arc, Mutex};

const CARD_STACK_INCREMENT: i32 = CARD_HEIGHT / 4;

pub struct Cascade {
    pos: Vector2<i32>,
    cards: Vec<Card>,
    tileset: Arc<Mutex<TileSet<Option<Card>>>>,
    follow_cursor: bool,
}

impl Cascade {
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
    pub fn take_all(&mut self) -> Vec<Card> {
        let mut v = Vec::new();
        std::mem::swap(&mut v, &mut self.cards);
        v
    }
    pub fn put(&mut self, mut cards: Vec<Card>) {
        self.cards.append(&mut cards);
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn is_single_card(&self) -> bool {
        self.cards.len() == 1
    }

    pub fn cards_to_take(&self, pos: Vector2<i32>) -> usize {
        if !self.inside(pos) {
            0
        } else {
            for i in 0..(self.cards.len()) {
                if pos[1] <= (self.pos[1] + ((1 + i as i32) * CARD_STACK_INCREMENT)) {
                    return self.cards.len() - i as usize;
                }
            }
            1
        }
    }
    pub fn has_alternating_color_cards(&self, n: usize) -> bool {
        let mut it = (self.cards[(self.cards.len() - n)..(self.cards.len())]).iter();
        let mut prev_card = if let Some(first_card) = it.next() {
            first_card
        } else {
            return true;
        };
        while let Some(current_card) = it.next() {
            if !(prev_card.follows_alternating(current_card)) {
                return false;
            }
            prev_card = current_card;
        }
        true
    }
    pub fn top_card(&self) -> Option<&Card> {
        self.cards.first()
    }
    pub fn bottom_card(&self) -> Option<&Card> {
        self.cards.last()
    }

    pub fn can_stack(&self, card: &Card) -> bool {
        match self.bottom_card() {
            Some(column_bottom_card) => column_bottom_card.follows_alternating(card),
            None => true,
        }
    }
}

impl EventHandler<ggez::GameError> for Cascade {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.follow_cursor {
            let cursor_pos = input::mouse::position(ctx);
            self.pos = vector![
                cursor_pos.x as i32 - CARD_WIDTH / 2,
                cursor_pos.y as i32 - CARD_HEIGHT / 3
            ];
        }
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.is_empty() && !self.follow_cursor {
            self.tileset
                .lock()
                .unwrap()
                .queue_tile(None, self.pos, None::<TileParams>)
                .unwrap();
        } else {
            for (y, card) in self.cards.iter().cloned().enumerate() {
                self.tileset
                    .lock()
                    .unwrap()
                    .queue_tile(
                        Some(card),
                        point![0, (y as i32 * CARD_STACK_INCREMENT)] + self.pos,
                        None::<TileParams>,
                    )
                    .unwrap();
            }
        }
        Ok(())
    }
}

impl Collision for Cascade {
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
