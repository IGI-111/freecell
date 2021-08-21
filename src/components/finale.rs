use crate::card::Card;
use crate::tileset::{TileParams, TileSet};
use ggez::audio::{SoundSource, Source};
use ggez::event::EventHandler;
use ggez::timer::check_update_time;
use ggez::{Context, GameResult};
use nalgebra::vector;
use nalgebra::Vector2;
use rand::prelude::*;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct Finale {
    tileset: Arc<Mutex<TileSet<Option<Card>>>>,
    finale_card_positions: VecDeque<(Card, Vector2<i32>)>,
    audio: Source,
}

impl Finale {
    pub fn new(ctx: &mut Context, tileset: Arc<Mutex<TileSet<Option<Card>>>>) -> Self {
        let mut audio = Source::new(ctx, "/take.wav").unwrap();
        audio.set_repeat(true);
        audio.set_pitch(4.);
        audio.set_volume(0.6);

        Self {
            finale_card_positions: VecDeque::new(),
            tileset,
            audio,
        }
    }
}

impl EventHandler<ggez::GameError> for Finale {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if check_update_time(ctx, 35) {
            let mut rng = rand::thread_rng();
            if self.finale_card_positions.len() >= 300 {
                self.finale_card_positions.pop_front();
            }
            self.finale_card_positions.push_back((
                Card::deck().into_iter().choose(&mut rng).unwrap(),
                vector![rng.gen_range(-10..810), rng.gen_range(-10..610)],
            ));

            if !self.audio.playing() {
                self.audio.play(ctx)?;
            } else {
                self.audio.set_volume(self.audio.volume() * 0.99);
            }
        }
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for (card, pos) in self.finale_card_positions.iter() {
            self.tileset
                .lock()
                .unwrap()
                .queue_tile(Some(card.clone()), pos.clone(), None::<TileParams>)
                .unwrap();
        }
        Ok(())
    }
}
