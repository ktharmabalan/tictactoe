use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

use game::GAME_SIZE;

const TILE_COLOR: Color = [(15.0/255.0), (20.0/255.0), (45.0/255.0), 1.0];

pub struct Tile {
  row: i32,
  col: i32,
  tile_size: f64,
  offset_size: f64,
}

impl Tile {
  pub fn new(row: i32, col: i32, tile_size: f64, offset_size: f64) -> Tile {
    Tile {
      row,
      col,
      tile_size,
      offset_size,
    }
  }

  pub fn draw(&self, con: &Context, g: &mut G2d) {
    rectangle(
      TILE_COLOR,
      [
        (self.row as f64 * self.tile_size * GAME_SIZE) + (self.offset_size * GAME_SIZE * ((self.row + 1) as f64)),
        (self.col as f64 * self.tile_size * GAME_SIZE) + (self.offset_size * GAME_SIZE * ((self.col + 1) as f64)),
        (self.tile_size * GAME_SIZE),
        (self.tile_size * GAME_SIZE),
      ],
      con.transform,
      g,
    );
  }
}