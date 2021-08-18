use ggez::graphics::{self, spritebatch::SpriteBatch, Color, DrawParam, Image, Rect};
use nalgebra::{point, vector, Point2, Vector2};
use std::collections::HashMap;
use std::hash::Hash;

/// A set of tiles made from a tilesheet image.
pub struct TileSet<Key: Hash + Eq> {
    // tile_size: Vector2<i32>,
    tile_cache: HashMap<Key, Point2<i32>>,
    sheet_dimensions: Vector2<i32>,
    spritebatch: SpriteBatch,
}

impl<Key: Hash + Eq> TileSet<Key> {
    /// Create a new `TileSet` from an image and tile size.
    pub fn new(sheet: Image, tile_size: impl Into<Vector2<i32>>) -> Self {
        let tile_size = tile_size.into();
        let sheet_dimensions = vector![
            sheet.width() as i32 / tile_size.x,
            sheet.height() as i32 / tile_size.y
        ];

        Self {
            // tile_size,
            tile_cache: HashMap::new(),
            sheet_dimensions,
            spritebatch: SpriteBatch::new(sheet),
        }
    }

    /// Register a tile from the tilesheet to the `TileSet` with the lookup
    /// value of `key`.
    pub fn register_tile(
        &mut self,
        key: Key,
        index: impl Into<Point2<i32>>,
    ) -> Result<(), TileSetError> {
        let index = index.into();

        if index.x > self.sheet_dimensions.x || index.y > self.sheet_dimensions.y {
            return Err(TileSetError::OutOfRange);
        }

        self.tile_cache.insert(key, index);

        Ok(())
    }

    /// Queue a tile with the lookup value `key` to be drawn at `draw_location`,
    /// with extra drawing options.
    pub fn queue_tile(
        &mut self,
        key: Key,
        draw_location: impl Into<Point2<i32>>,
        options: Option<impl Into<TileParams>>,
    ) -> Result<(), TileSetError> {
        let tile = self
            .tile_cache
            .get(&key)
            .ok_or(TileSetError::TileNotFound)?;

        let options = options.map(|tp| tp.into()).unwrap_or(TileParams {
            color: None,
            scale: None,
        });

        let coords = draw_location.into();
        let normal_x = 1.0 / self.sheet_dimensions.x as f32;
        let normal_y = 1.0 / self.sheet_dimensions.y as f32;

        let d = DrawParam::default()
            .dest(point![(coords.x) as f32, (coords.y) as f32])
            .src(Rect::new(
                normal_x * tile.x as f32,
                normal_y * tile.y as f32,
                normal_x,
                normal_y,
            ))
            .color(options.color.unwrap_or(Color::WHITE))
            .scale(options.scale.unwrap_or(vector![1.0, 1.0]));

        self.spritebatch.add(d);

        Ok(())
    }

    /// Clear the tile queue.
    #[allow(dead_code)]
    pub fn clear_queue(&mut self) {
        self.spritebatch.clear();
    }

    /// Draw the tiles using `ctx` && 'spritebatch'. Default parameters
    /// are given to the batch.
    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::draw(ctx, &self.spritebatch, DrawParam::default())
    }
}

/// Additional parameters for drawing tiles.
pub struct TileParams {
    /// The optional color to draw the tile with.
    pub color: Option<Color>,
    /// Scale factor for drawing. Default is `1.0` (no scaling).
    pub scale: Option<Vector2<f32>>,
}

impl From<(Option<Color>, Option<Vector2<f32>>)> for TileParams {
    fn from((color, scale): (Option<Color>, Option<Vector2<f32>>)) -> TileParams {
        TileParams { color, scale }
    }
}

impl From<(Option<Color>, Vector2<f32>)> for TileParams {
    fn from((color, scale): (Option<Color>, Vector2<f32>)) -> TileParams {
        TileParams {
            color,
            scale: Some(scale),
        }
    }
}

impl From<(Color, Option<Vector2<f32>>)> for TileParams {
    fn from((color, scale): (Color, Option<Vector2<f32>>)) -> TileParams {
        TileParams {
            color: Some(color),
            scale,
        }
    }
}

impl From<(Color, Vector2<f32>)> for TileParams {
    fn from((color, scale): (Color, Vector2<f32>)) -> TileParams {
        TileParams {
            color: Some(color),
            scale: Some(scale),
        }
    }
}

/// Possible errors from `TileSet` operations.
#[derive(Debug, Clone, Copy)]
pub enum TileSetError {
    /// The tile position to register was outside the tilesheet bounds.
    OutOfRange,
    /// Tile not found.
    TileNotFound,
}

impl std::fmt::Display for TileSetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TileSetError::OutOfRange => "Position out of range of tilesheet dimensions",
                TileSetError::TileNotFound => "Tile not found during lookup",
            }
        )
    }
}

impl std::error::Error for TileSetError {}
