use crate::card::{Card, CARD_HEIGHT, CARD_WIDTH};
use crate::components::{Button, Cascade, Cell, Foundation};
use crate::tileset::TileSet;
use ggez::graphics;
use ggez::Context;
use nalgebra::{point, vector};
use rand::prelude::*;
use std::sync::{Arc, Mutex};

const MARGIN_LENGTH: i32 = 20;
const ICON_SIDE: i32 = 38;

pub fn tileset(ctx: &mut Context) -> TileSet<Option<Card>> {
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

pub fn button(ctx: &mut Context) -> Button {
    Button::new(
        ctx,
        vector![
            3 * MARGIN_LENGTH / 2 - ICON_SIDE / 2 + (4 * (CARD_WIDTH + MARGIN_LENGTH)),
            MARGIN_LENGTH
        ],
    )
}

pub fn cascades(tileset: Arc<Mutex<TileSet<Option<Card>>>>) -> [Cascade; 8] {
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
pub fn open_cells(tileset: Arc<Mutex<TileSet<Option<Card>>>>) -> [Cell; 4] {
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
            tileset,
        ),
    ]
}
pub fn foundations(tileset: Arc<Mutex<TileSet<Option<Card>>>>) -> [Foundation; 4] {
    [
        Foundation::new(
            vector![MARGIN_LENGTH, MARGIN_LENGTH],
            vec![],
            tileset.clone(),
        ),
        Foundation::new(
            vector![MARGIN_LENGTH + (CARD_WIDTH + MARGIN_LENGTH), MARGIN_LENGTH],
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
            tileset,
        ),
    ]
}
