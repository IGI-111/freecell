use card::{Card, CARD_HEIGHT, CARD_WIDTH};
use cell::Cell;
use column::Column;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use nalgebra::{point, vector, Vector2};
use rand::prelude::*;
use std::sync::{Arc, Mutex};
use tileset::TileSet;

mod card;
mod cell;
mod column;
mod tileset;

trait Collision {
    fn inside(pos: Vector2<i32>) -> bool;
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
    let my_game = Game::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct Game {
    tileset: Arc<Mutex<tileset::TileSet<Option<Card>>>>,
    columns: [Column; 8],
    open_cells: [Cell; 4],
    foundations: [Cell; 4],
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
            column::Column::new(
                vector![10 + (i as i32 * (CARD_WIDTH + 10)), 10 + CARD_HEIGHT + 10],
                cards,
                tileset.clone(),
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
    fn init_open_cells(tileset: Arc<Mutex<TileSet<Option<Card>>>>) -> [Cell; 4] {
        [
            Cell::new(
                vector![10 + (4 * (CARD_WIDTH + 10)), 10],
                None,
                tileset.clone(),
            ),
            Cell::new(
                vector![10 + (5 * (CARD_WIDTH + 10)), 10],
                None,
                tileset.clone(),
            ),
            Cell::new(
                vector![10 + (6 * (CARD_WIDTH + 10)), 10],
                None,
                tileset.clone(),
            ),
            Cell::new(
                vector![10 + (7 * (CARD_WIDTH + 10)), 10],
                None,
                tileset.clone(),
            ),
        ]
    }
    fn init_foundations(tileset: Arc<Mutex<TileSet<Option<Card>>>>) -> [Cell; 4] {
        [
            Cell::new(
                vector![10 + (0 * (CARD_WIDTH + 10)), 10],
                None,
                tileset.clone(),
            ),
            Cell::new(
                vector![10 + (1 * (CARD_WIDTH + 10)), 10],
                None,
                tileset.clone(),
            ),
            Cell::new(
                vector![10 + (2 * (CARD_WIDTH + 10)), 10],
                None,
                tileset.clone(),
            ),
            Cell::new(
                vector![10 + (3 * (CARD_WIDTH + 10)), 10],
                None,
                tileset.clone(),
            ),
        ]
    }

    pub fn new(ctx: &mut Context) -> Game {
        let tileset = Arc::new(Mutex::new(Self::init_tileset(ctx)));
        let columns = Self::init_columns(tileset.clone());
        let open_cells = Self::init_open_cells(tileset.clone());
        let foundations = Self::init_foundations(tileset.clone());
        Game {
            columns,
            tileset,
            open_cells,
            foundations,
        }
    }
}

impl EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::GREEN);
        for c in self.foundations.iter_mut() {
            c.draw(ctx)?;
        }
        for c in self.open_cells.iter_mut() {
            c.draw(ctx)?;
        }
        for c in self.columns.iter_mut() {
            c.draw(ctx)?;
        }
        self.tileset.lock().unwrap().draw(ctx)?;
        graphics::present(ctx)
    }
}
