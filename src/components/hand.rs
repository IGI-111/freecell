use super::CARD_STACK_INCREMENT;
use crate::card::{Card, CARD_HEIGHT, CARD_WIDTH};
use crate::tileset::{TileParams, TileSet};
use ggez::audio::{SoundData, SoundSource, Source};
use ggez::event::EventHandler;
use ggez::input;
use ggez::{Context, GameResult};
use nalgebra::{point, vector, Vector2};
use std::sync::{Arc, Mutex};

pub struct Hand {
    pos: Vector2<i32>,
    cards: Vec<Card>,
    tileset: Arc<Mutex<TileSet<Option<Card>>>>,
    drop_audio: SoundData,
    take_audio: SoundData,
}

impl Hand {
    pub fn new(ctx: &mut Context, tileset: Arc<Mutex<TileSet<Option<Card>>>>) -> Self {
        let drop_audio = SoundData::new(ctx, "/drop.wav").unwrap();
        let take_audio = SoundData::new(ctx, "/take.wav").unwrap();
        Self {
            pos: vector![0, 0],
            cards: Vec::new(),
            tileset,
            drop_audio,
            take_audio,
        }
    }
    fn play_drop(&self, ctx: &mut Context) {
        let mut source = Source::from_data(ctx, self.drop_audio.clone()).unwrap();
        source.set_volume(0.1);
        source.set_pitch(1.7);
        source.play_detached(ctx).unwrap();
    }
    fn play_take(&self, ctx: &mut Context) {
        let mut source = Source::from_data(ctx, self.take_audio.clone()).unwrap();
        source.set_volume(0.1);
        source.set_pitch(1.7);
        source.play_detached(ctx).unwrap();
    }

    pub fn take(&mut self, ctx: &mut Context) -> Vec<Card> {
        let mut v = Vec::new();
        std::mem::swap(&mut v, &mut self.cards);
        self.play_drop(ctx);
        v
    }
    pub fn put(&mut self, ctx: &mut Context, mut cards: Vec<Card>) {
        self.play_take(ctx);
        self.cards.append(&mut cards);
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn is_single_card(&self) -> bool {
        self.cards.len() == 1
    }

    pub fn top_card(&self) -> Option<&Card> {
        self.cards.first()
    }
}

impl EventHandler<ggez::GameError> for Hand {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let cursor_pos = input::mouse::position(ctx);
        self.pos = vector![
            cursor_pos.x as i32 - CARD_WIDTH / 2,
            cursor_pos.y as i32 - CARD_HEIGHT / 3
        ];
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
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
        Ok(())
    }
}
