use crate::audio::Audio;
use crate::card::{Card, CARD_HEIGHT, CARD_WIDTH};
use crate::components::{Cascade, Cell, Finale, Foundation};
use crate::tileset::TileSet;
use ggez::event::EventHandler;
use ggez::graphics::{self, Color};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};
use nalgebra::{point, vector, Vector2};
use rand::prelude::*;
use std::sync::{Arc, Mutex};

const MARGIN_LENGTH: i32 = 20;

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
    cursor_column: Cascade,
    cursor_card_source: Option<CardSource>,
    audio: Audio,
    finale: Finale,
}

impl Game {
    fn init_tileset(ctx: &mut Context) -> TileSet<Option<Card>> {
        let image = graphics::Image::new(ctx, "/cards.png").unwrap();
        let mut tileset = TileSet::new(image, vector![CARD_WIDTH, CARD_HEIGHT]);
        for suit in 0..4 {
            for value in 0..13 {
                tileset
                    .register_tile(
                        Some(Card { suit, value }),
                        point![value as i32, suit as i32],
                    )
                    .unwrap();
            }
        }
        tileset.register_tile(None, point![12, 4]).unwrap();
        tileset
    }

    fn init_cascades(tileset: Arc<Mutex<TileSet<Option<Card>>>>) -> [Cascade; 8] {
        let mut rng = rand::thread_rng();
        let mut deck = Card::deck();
        deck.shuffle(&mut rng);

        let mut cascades = vec![
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];
        let mut selected_column = 0;
        while let Some(card) = deck.pop() {
            cascades[selected_column].push(card);
            selected_column = (selected_column + 1) % cascades.len();
        }
        let mut it = cascades.into_iter().enumerate().map(|(i, cards)| {
            Cascade::new(
                vector![
                    2 * MARGIN_LENGTH + (i as i32 * (CARD_WIDTH + MARGIN_LENGTH)),
                    MARGIN_LENGTH + CARD_HEIGHT + MARGIN_LENGTH
                ],
                cards,
                tileset.clone(),
                false,
            )
        });
        [
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
        ]
    }
    fn init_cursor_column(tileset: Arc<Mutex<TileSet<Option<Card>>>>) -> Cascade {
        Cascade::new(vector![0, 0], vec![], tileset.clone(), true)
    }
    fn init_open_cells(tileset: Arc<Mutex<TileSet<Option<Card>>>>) -> [Cell; 4] {
        [
            Cell::new(
                vector![
                    3 * MARGIN_LENGTH + (4 * (CARD_WIDTH + MARGIN_LENGTH)),
                    MARGIN_LENGTH
                ],
                None,
                tileset.clone(),
            ),
            Cell::new(
                vector![
                    3 * MARGIN_LENGTH + (5 * (CARD_WIDTH + MARGIN_LENGTH)),
                    MARGIN_LENGTH
                ],
                None,
                tileset.clone(),
            ),
            Cell::new(
                vector![
                    3 * MARGIN_LENGTH + (6 * (CARD_WIDTH + MARGIN_LENGTH)),
                    MARGIN_LENGTH
                ],
                None,
                tileset.clone(),
            ),
            Cell::new(
                vector![
                    3 * MARGIN_LENGTH + (7 * (CARD_WIDTH + MARGIN_LENGTH)),
                    MARGIN_LENGTH
                ],
                None,
                tileset.clone(),
            ),
        ]
    }
    fn init_foundations(tileset: Arc<Mutex<TileSet<Option<Card>>>>) -> [Foundation; 4] {
        [
            Foundation::new(
                vector![
                    MARGIN_LENGTH + (0 * (CARD_WIDTH + MARGIN_LENGTH)),
                    MARGIN_LENGTH
                ],
                vec![],
                tileset.clone(),
            ),
            Foundation::new(
                vector![
                    MARGIN_LENGTH + (1 * (CARD_WIDTH + MARGIN_LENGTH)),
                    MARGIN_LENGTH
                ],
                vec![],
                tileset.clone(),
            ),
            Foundation::new(
                vector![
                    MARGIN_LENGTH + (2 * (CARD_WIDTH + MARGIN_LENGTH)),
                    MARGIN_LENGTH
                ],
                vec![],
                tileset.clone(),
            ),
            Foundation::new(
                vector![
                    MARGIN_LENGTH + (3 * (CARD_WIDTH + MARGIN_LENGTH)),
                    MARGIN_LENGTH
                ],
                vec![],
                tileset.clone(),
            ),
        ]
    }

    fn is_victory(&self) -> bool {
        self.foundations.iter().all(|f| f.is_full())
    }

    pub fn new(ctx: &mut Context) -> Self {
        let tileset = Arc::new(Mutex::new(Self::init_tileset(ctx)));
        let cascades = Self::init_cascades(tileset.clone());
        let open_cells = Self::init_open_cells(tileset.clone());
        let foundations = Self::init_foundations(tileset.clone());
        let cursor_column = Self::init_cursor_column(tileset.clone());

        Self {
            cascades,
            open_cells,
            foundations,
            cursor_column,
            cursor_card_source: None,
            audio: Audio::new(ctx),
            finale: Finale::new(ctx, tileset.clone()),
            tileset,
        }
    }
}

impl EventHandler<ggez::GameError> for Game {
    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if self.is_victory() {
            return;
        }
        if button == MouseButton::Left {
            let pos = vector![x as i32, y as i32];
            if !self.cursor_column.is_empty() {
                for c in self.cascades.iter_mut() {
                    if c.inside(pos) && c.can_stack(self.cursor_column.top_card().unwrap()) {
                        c.put(self.cursor_column.take_all());
                        self.audio.play_drop(ctx);
                        return;
                    }
                }

                for c in self.open_cells.iter_mut() {
                    if c.inside(pos) && self.cursor_column.is_single_card() && c.is_empty() {
                        c.put(self.cursor_column.take(1).pop().unwrap());
                        self.audio.play_drop(ctx);
                        return;
                    }
                }
                for f in self.foundations.iter_mut() {
                    if f.inside(pos)
                        && self.cursor_column.is_single_card()
                        && f.can_stack(self.cursor_column.top_card().unwrap())
                    {
                        f.put(self.cursor_column.take(1).pop().unwrap());
                        self.audio.play_drop(ctx);
                        return;
                    }
                }
                // return cards back if they're not put anywhere
                match self.cursor_card_source {
                    Some(CardSource::Cell(n)) => {
                        self.open_cells[n].put(self.cursor_column.take(1).pop().unwrap());
                    }
                    Some(CardSource::Cascade(n)) => {
                        self.cascades[n].put(self.cursor_column.take_all());
                    }
                    Some(CardSource::Foundation(n)) => {
                        self.foundations[n].put(self.cursor_column.take(1).pop().unwrap());
                    }
                    None => {
                        panic!("No card source");
                    }
                }
                self.cursor_card_source = None;
                self.audio.play_drop(ctx);
            }
        }
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if self.is_victory() {
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
                                    self.cursor_card_source = None;
                                    self.audio.play_deal(ctx);
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
                                    self.cursor_card_source = None;
                                    self.audio.play_deal(ctx);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
            MouseButton::Left => {
                let pos = vector![x as i32, y as i32];
                if self.cursor_column.is_empty() {
                    for (i, c) in self.cascades.iter_mut().enumerate() {
                        if c.inside(pos) {
                            let cards_to_take = c.cards_to_take(pos);
                            if c.has_alternating_color_cards(cards_to_take) {
                                self.cursor_column.put(c.take(cards_to_take));
                                self.cursor_card_source = Some(CardSource::Cascade(i));
                                self.audio.play_take(ctx);
                                return;
                            }
                        }
                    }
                    for (i, c) in self.open_cells.iter_mut().enumerate() {
                        if c.inside(pos) {
                            if let Some(card) = c.take() {
                                self.cursor_column.put(vec![card]);
                                self.cursor_card_source = Some(CardSource::Cell(i));
                                self.audio.play_take(ctx);
                                return;
                            }
                        }
                    }
                    for (i, f) in self.foundations.iter_mut().enumerate() {
                        if f.inside(pos) {
                            if let Some(card) = f.take() {
                                self.cursor_column.put(vec![card]);
                                self.cursor_card_source = Some(CardSource::Foundation(i));
                                self.audio.play_take(ctx);
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
        self.cursor_column.update(ctx)?;

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
        self.cursor_column.draw(ctx)?;

        if self.is_victory() {
            self.finale.draw(ctx)?;
        }

        self.tileset.lock().unwrap().draw(ctx)?;
        graphics::present(ctx)?;
        std::thread::yield_now();
        Ok(())
    }
}
