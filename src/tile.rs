use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

use game::GAME_SIZE;

const TILE_COLOR: Color = [(15.0/255.0), (20.0/255.0), (45.0/255.0), 1.0];
const PLAYER1_TILE_COLOR: Color = [(20.0/255.0), 0.75, (45.0/255.0), 1.0];
const PLAYER2_TILE_COLOR: Color = [(15.0/255.0), (45.0/255.0), 0.75, 1.0];

#[derive(Debug)]
pub enum TileState {
  None,
  Player1,
  Player2,
}

pub struct Tile {
  pub row: i32,
  pub col: i32,
  pub tile_size: f64,
  pub offset_size: f64,
  pub clicked: bool,
  pub tile_state: TileState,
}

impl Tile {
  pub fn new(row: i32, col: i32, tile_size: f64, offset_size: f64) -> Tile {
    Tile {
      row,
      col,
      tile_size,
      offset_size,
      tile_state: TileState::None,
      clicked: false,
    }
  }

  pub fn clicked(&mut self, player: &TileState) {
    println!("UPDATED");
    match self.tile_state {
      TileState::None => {
        self.clicked = true;
        self.tile_state = match player {
          TileState::Player1 => TileState::Player1,
          TileState::Player2 => TileState::Player2,
          _ => TileState::None,
        };
      },
      _ => (),
    }
  }

  pub fn draw(&self, con: &Context, g: &mut G2d) {
    let color = match self.tile_state {
        TileState::Player1 => PLAYER1_TILE_COLOR,
      TileState::Player2 => PLAYER2_TILE_COLOR,
      _ => TILE_COLOR,
    };
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

  // pub fn update(&mut self) {
  // }
}