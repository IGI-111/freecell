use crate::card::Card;
use crate::components::{Button, Cascade, Cell, Finale, Foundation, Hand};
use crate::tileset::TileSet;
use ggez::audio::{SoundData, SoundSource, Source};
use ggez::event::EventHandler;
use ggez::graphics::{self, Color};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};
use nalgebra::{vector, Vector2};
use std::sync::{Arc, Mutex};

mod init;

pub trait Collision {
    fn inside(&self, pos: Vector2<i32>) -> bool;
}

enum CardSource {
    Cell(usize),
    Cascade(usize),
    Foundation(usize),
}

pub struct Game {
    tileset: Arc<Mutex<TileSet<Option<Card>>>>,
    cascades: [Cascade; 8],
    open_cells: [Cell; 4],
    foundations: [Foundation; 4],
    hand: Hand,
    hand_card_source: Option<CardSource>,
    button: Button,
    finale: Finale,
    deal_audio: SoundData,
}

impl Game {
    fn is_victory(&self) -> bool {
        self.cascades.iter().all(|c| c.has_all_alternating())
    }

    pub fn new(ctx: &mut Context) -> Self {
        let tileset = Arc::new(Mutex::new(init::tileset(ctx)));
        let cascades = init::cascades(tileset.clone());
        let open_cells = init::open_cells(tileset.clone());
        let foundations = init::foundations(tileset.clone());
        let hand = Hand::new(ctx, tileset.clone());
        let deal_audio = SoundData::new(ctx, "/deal.wav").unwrap();
        let button = init::button(ctx);

        let game = Self {
            cascades,
            open_cells,
            foundations,
            hand,
            hand_card_source: None,
            finale: Finale::new(ctx, tileset.clone()),
            tileset,
            deal_audio,
            button,
        };
        game.play_deal(ctx);
        game
    }

    fn play_send(&self, ctx: &mut Context) {
        let mut source = Source::from_data(ctx, self.deal_audio.clone()).unwrap();
        source.set_volume(0.2);
        source.play_detached(ctx).unwrap();
    }

    fn play_deal(&self, ctx: &mut Context) {
        let mut source = Source::from_data(ctx, self.deal_audio.clone()).unwrap();
        source.set_volume(2.);
        source.set_pitch(2.);
        source.play_detached(ctx).unwrap();
    }

    fn reset(&mut self, ctx: &mut Context) {
        self.play_deal(ctx);
        self.cascades = init::cascades(self.tileset.clone());
        self.open_cells = init::open_cells(self.tileset.clone());
        self.foundations = init::foundations(self.tileset.clone());
        self.hand = Hand::new(ctx, self.tileset.clone());
        self.finale = Finale::new(ctx, self.tileset.clone());
    }
}

impl EventHandler<ggez::GameError> for Game {
    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if self.is_victory() {
            return;
        }
        if button == MouseButton::Left {
            let pos = vector![x as i32, y as i32];

            if self.button.inside(pos) {
                self.reset(ctx);
                return;
            }

            if !self.hand.is_empty() {
                for c in self.cascades.iter_mut() {
                    if c.inside(pos) && c.can_stack(self.hand.top_card().unwrap()) {
                        c.put(self.hand.take(ctx));
                        return;
                    }
                }

                for c in self.open_cells.iter_mut() {
                    if c.inside(pos) && self.hand.is_single_card() && c.is_empty() {
                        c.put(self.hand.take(ctx).pop().unwrap());
                        return;
                    }
                }
                for f in self.foundations.iter_mut() {
                    if f.inside(pos)
                        && self.hand.is_single_card()
                        && f.can_stack(self.hand.top_card().unwrap())
                    {
                        f.put(self.hand.take(ctx).pop().unwrap());
                        return;
                    }
                }
                // return cards back if they're not put anywhere
                match self.hand_card_source {
                    Some(CardSource::Cell(n)) => {
                        self.open_cells[n].put(self.hand.take(ctx).pop().unwrap());
                    }
                    Some(CardSource::Cascade(n)) => {
                        self.cascades[n].put(self.hand.take(ctx));
                    }
                    Some(CardSource::Foundation(n)) => {
                        self.foundations[n].put(self.hand.take(ctx).pop().unwrap());
                    }
                    None => {
                        panic!("No card source");
                    }
                }
                self.hand_card_source = None;
            }
        }
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if self.is_victory() {
            if self.finale.is_playing() {
                self.reset(ctx);
            }
            return;
        }
        match button {
            MouseButton::Right => {
                let pos = vector![x as i32, y as i32];
                for c in self.cascades.iter_mut() {
                    if c.inside(pos) && c.cards_to_take(pos) == 1 {
                        if let Some(card_to_stack) = c.bottom_card() {
                            for f in self.foundations.iter_mut() {
                                if f.can_stack(card_to_stack) {
                                    f.put(c.take(1).pop().unwrap());
                                    self.hand_card_source = None;
                                    self.play_send(ctx);
                                    return;
                                }
                            }
                        }
                    }
                }
                for c in self.open_cells.iter_mut() {
                    if c.inside(pos) {
                        if let Some(card_to_stack) = c.card() {
                            for f in self.foundations.iter_mut() {
                                if f.can_stack(card_to_stack) {
                                    f.put(c.take().unwrap());
                                    self.hand_card_source = None;
                                    self.play_send(ctx);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
            MouseButton::Left => {
                let pos = vector![x as i32, y as i32];
                if self.hand.is_empty() {
                    for (i, c) in self.cascades.iter_mut().enumerate() {
                        if c.inside(pos) {
                            let cards_to_take = c.cards_to_take(pos);
                            if c.has_alternating_color_cards(cards_to_take) {
                                self.hand.put(ctx, c.take(cards_to_take));
                                self.hand_card_source = Some(CardSource::Cascade(i));
                                return;
                            }
                        }
                    }
                    for (i, c) in self.open_cells.iter_mut().enumerate() {
                        if c.inside(pos) {
                            if let Some(card) = c.take() {
                                self.hand.put(ctx, vec![card]);
                                self.hand_card_source = Some(CardSource::Cell(i));
                                return;
                            }
                        }
                    }
                    for (i, f) in self.foundations.iter_mut().enumerate() {
                        if f.inside(pos) {
                            if let Some(card) = f.take() {
                                self.hand.put(ctx, vec![card]);
                                self.hand_card_source = Some(CardSource::Foundation(i));
                                return;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.hand.update(ctx)?;

        if self.is_victory() {
            self.finale.update(ctx)?;
        }
        std::thread::yield_now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(19, 147, 40));
        self.tileset.lock().unwrap().clear_queue();
        for f in self.foundations.iter_mut() {
            f.draw(ctx)?;
        }
        for c in self.open_cells.iter_mut() {
            c.draw(ctx)?;
        }
        for c in self.cascades.iter_mut() {
            c.draw(ctx)?;
        }
        self.hand.draw(ctx)?;
        self.button.draw(ctx)?;

        if self.is_victory() {
            self.finale.draw(ctx)?;
        }

        self.tileset.lock().unwrap().draw(ctx)?;
        graphics::present(ctx)?;
        std::thread::yield_now();
        Ok(())
    }
}
