use card::{Card, CARD_HEIGHT, CARD_WIDTH};
use components::cell::Cell;
use components::column::Column;
use components::stack::Stack;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::input::mouse::MouseButton;
use ggez::{Context, ContextBuilder, GameResult};
use nalgebra::{point, vector, Vector2};
use rand::prelude::*;
use std::sync::{Arc, Mutex};
use tileset::TileSet;

mod card;
mod components;
mod tileset;

const MARGIN_LENGTH: i32 = 20;

trait Collision {
    fn inside(&self, pos: Vector2<i32>) -> bool;
}

enum CardSource {
    Cell(usize),
    Column(usize),
    Stack(usize),
}

fn main() {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("freecell", "Freecell")
        .add_resource_path(resource_dir)
        .build()
        .unwrap();
    graphics::set_window_title(&mut ctx, "Freecell");
    let my_game = Game::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct Game {
    tileset: Arc<Mutex<tileset::TileSet<Option<Card>>>>,
    columns: [Column; 8],
    open_cells: [Cell; 4],
    stacks: [Stack; 4],
    cursor_column: Column,
    cursor_card_source: Option<CardSource>,
}

impl Game {
    fn init_tileset(ctx: &mut Context) -> tileset::TileSet<Option<Card>> {
        let image = graphics::Image::new(ctx, "/cards.png").unwrap();
        let mut tileset = tileset::TileSet::new(image, vector![CARD_WIDTH, CARD_HEIGHT]);
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

    fn init_columns(tileset: Arc<Mutex<TileSet<Option<Card>>>>) -> [Column; 8] {
        let mut rng = rand::thread_rng();
        let mut deck = Card::deck();
        deck.shuffle(&mut rng);

        let mut columns = vec![
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
            columns[selected_column].push(card);
            selected_column = (selected_column + 1) % columns.len();
        }
        let mut it = columns.into_iter().enumerate().map(|(i, cards)| {
            Column::new(
                vector![
                    MARGIN_LENGTH + (i as i32 * (CARD_WIDTH + MARGIN_LENGTH)),
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
    fn init_cursor_column(tileset: Arc<Mutex<TileSet<Option<Card>>>>) -> Column {
        Column::new(vector![0, 0], vec![], tileset.clone(), true)
    }
    fn init_open_cells(tileset: Arc<Mutex<TileSet<Option<Card>>>>) -> [Cell; 4] {
        [
            Cell::new(
                vector![
                    MARGIN_LENGTH + (4 * (CARD_WIDTH + MARGIN_LENGTH)),
                    MARGIN_LENGTH
                ],
                None,
                tileset.clone(),
            ),
            Cell::new(
                vector![
                    MARGIN_LENGTH + (5 * (CARD_WIDTH + MARGIN_LENGTH)),
                    MARGIN_LENGTH
                ],
                None,
                tileset.clone(),
            ),
            Cell::new(
                vector![
                    MARGIN_LENGTH + (6 * (CARD_WIDTH + MARGIN_LENGTH)),
                    MARGIN_LENGTH
                ],
                None,
                tileset.clone(),
            ),
            Cell::new(
                vector![
                    MARGIN_LENGTH + (7 * (CARD_WIDTH + MARGIN_LENGTH)),
                    MARGIN_LENGTH
                ],
                None,
                tileset.clone(),
            ),
        ]
    }
    fn init_stacks(tileset: Arc<Mutex<TileSet<Option<Card>>>>) -> [Stack; 4] {
        [
            Stack::new(
                vector![
                    MARGIN_LENGTH + (0 * (CARD_WIDTH + MARGIN_LENGTH)),
                    MARGIN_LENGTH
                ],
                vec![],
                tileset.clone(),
            ),
            Stack::new(
                vector![
                    MARGIN_LENGTH + (1 * (CARD_WIDTH + MARGIN_LENGTH)),
                    MARGIN_LENGTH
                ],
                vec![],
                tileset.clone(),
            ),
            Stack::new(
                vector![
                    MARGIN_LENGTH + (2 * (CARD_WIDTH + MARGIN_LENGTH)),
                    MARGIN_LENGTH
                ],
                vec![],
                tileset.clone(),
            ),
            Stack::new(
                vector![
                    MARGIN_LENGTH + (3 * (CARD_WIDTH + MARGIN_LENGTH)),
                    MARGIN_LENGTH
                ],
                vec![],
                tileset.clone(),
            ),
        ]
    }

    pub fn new(ctx: &mut Context) -> Self {
        let tileset = Arc::new(Mutex::new(Self::init_tileset(ctx)));
        let columns = Self::init_columns(tileset.clone());
        let open_cells = Self::init_open_cells(tileset.clone());
        let stacks = Self::init_stacks(tileset.clone());
        let cursor_column = Self::init_cursor_column(tileset.clone());
        Self {
            columns,
            tileset,
            open_cells,
            stacks,
            cursor_column,
            cursor_card_source: None,
        }
    }
}

impl EventHandler<ggez::GameError> for Game {
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            let pos = vector![x as i32, y as i32];
            if !self.cursor_column.is_empty() {
                for c in self.columns.iter_mut() {
                    if c.inside(pos) && c.can_stack(self.cursor_column.top_card().unwrap()) {
                        c.put(self.cursor_column.take_all());
                        return;
                    }
                }

                for c in self.open_cells.iter_mut() {
                    if c.inside(pos) && self.cursor_column.is_single_card() && c.is_empty() {
                        c.put(self.cursor_column.take(1).pop().unwrap());
                        return;
                    }
                }
                for s in self.stacks.iter_mut() {
                    if s.inside(pos)
                        && self.cursor_column.is_single_card()
                        && s.can_stack(self.cursor_column.top_card().unwrap())
                    {
                        s.put(self.cursor_column.take(1).pop().unwrap());
                        return;
                    }
                }
                // return cards back if they're not put anywhere
                match self.cursor_card_source {
                    Some(CardSource::Cell(n)) => {
                        self.open_cells[n].put(self.cursor_column.take(1).pop().unwrap());
                    }
                    Some(CardSource::Column(n)) => {
                        self.columns[n].put(self.cursor_column.take_all());
                    }
                    Some(CardSource::Stack(n)) => {
                        self.stacks[n].put(self.cursor_column.take(1).pop().unwrap());
                    }
                    None => {
                        panic!("No card source");
                    }
                }
                self.cursor_card_source = None;
            }
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match button {
            MouseButton::Right => {
                let pos = vector![x as i32, y as i32];
                for c in self.columns.iter_mut() {
                    if c.inside(pos) && c.cards_to_take(pos) == 1 {
                        if let Some(card_to_stack) = c.bottom_card() {
                            for s in self.stacks.iter_mut() {
                                if s.can_stack(card_to_stack) {
                                    s.put(c.take(1).pop().unwrap());
                                    self.cursor_card_source = None;
                                    return;
                                }
                            }
                        }
                    }
                }
                for c in self.open_cells.iter_mut() {
                    if c.inside(pos) {
                        if let Some(card_to_stack) = c.card() {
                            for s in self.stacks.iter_mut() {
                                if s.can_stack(card_to_stack) {
                                    s.put(c.take().unwrap());
                                    self.cursor_card_source = None;
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
                    for (i, c) in self.columns.iter_mut().enumerate() {
                        if c.inside(pos) {
                            let cards_to_take = c.cards_to_take(pos);
                            if c.has_alternating_color_cards(cards_to_take) {
                                self.cursor_column.put(c.take(cards_to_take));
                                self.cursor_card_source = Some(CardSource::Column(i));
                            }
                        }
                    }
                    for (i, c) in self.open_cells.iter_mut().enumerate() {
                        if c.inside(pos) {
                            if let Some(card) = c.take() {
                                self.cursor_column.put(vec![card]);
                                self.cursor_card_source = Some(CardSource::Cell(i));
                            }
                        }
                    }
                    for (i, s) in self.stacks.iter_mut().enumerate() {
                        if s.inside(pos) {
                            if let Some(card) = s.take() {
                                self.cursor_column.put(vec![card]);
                                self.cursor_card_source = Some(CardSource::Stack(i));
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
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(19, 147, 40));
        self.tileset.lock().unwrap().clear_queue();

        for c in self.stacks.iter_mut() {
            c.draw(ctx)?;
        }
        for c in self.open_cells.iter_mut() {
            c.draw(ctx)?;
        }
        for c in self.columns.iter_mut() {
            c.draw(ctx)?;
        }
        self.cursor_column.draw(ctx)?;
        self.tileset.lock().unwrap().draw(ctx)?;
        graphics::present(ctx)
    }
}
