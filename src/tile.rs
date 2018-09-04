use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

use game::GAME_SIZE;

const TILE_COLOR: Color = [(15.0/255.0), (20.0/255.0), (45.0/255.0), 1.0];
const CLICKED_TILE_COLOR: Color = [1.0, (20.0/255.0), (45.0/255.0), 0.5];

pub struct Tile {
  pub row: i32,
  pub col: i32,
  pub tile_size: f64,
  pub offset_size: f64,
  pub clicked: bool,
}

impl Tile {
  pub fn new(row: i32, col: i32, tile_size: f64, offset_size: f64) -> Tile {
    Tile {
      row,
      col,
      tile_size,
      offset_size,
      clicked: false,
    }
  }

  pub fn clicked(&mut self) {
    self.clicked = true;
  }

  pub fn draw(&self, con: &Context, g: &mut G2d) {
    let mut color = TILE_COLOR;
    if self.clicked {
      color = CLICKED_TILE_COLOR;
    }
    rectangle(
      color,
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