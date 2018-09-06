// Import piston
use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

use board::Board;
use tile::TileState;

const GAME_COLOR: Color = [0.17, 0.21, 0.27, 1.0];

// Moving period/restart time
#[allow(dead_code)]
const MOVING_PERIOD: f64 = 0.1; // lower for faster update, greater for slower update
#[allow(dead_code)]
const RESTART_TIME: f64 = 1.0;  // wait time in seconds

pub const GAME_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
  (game_coord as f64) * GAME_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
  to_coord(game_coord) as u32
}

#[allow(dead_code)]
pub struct Game {
  width: i32,
  height: i32,
  game_over: bool,
  wait_time: f64,
  board: Board,
}

impl Game {
  pub fn new(width: i32, height: i32) -> Game {
    let (cols, rows) = (3, 3);

    let board = Board::new(
      0,
      0,
      width,
      height,
      cols,
      rows,
    );

    Game {
      wait_time: 0.0,
      game_over: false,
      width,
      height,
      board,
    }
  }

  pub fn draw(&mut self, con: &Context, g: &mut G2d) {
    rectangle(
      GAME_COLOR,
      [
        0.0,
        0.0,
        GAME_SIZE * (self.width as f64),
        GAME_SIZE * (self.height as f64),
        // GAME_SIZE / 2.0 * (self.width as f64),
        // GAME_SIZE / 2.0 * (self.height as f64),
      ],
      con.transform,
      g,
    );
    self.board.draw(&con, g)
  }

  pub fn update(&mut self, delta_time: f64, x: f32, y: f32, clicked: bool, player: &mut TileState) -> bool {
    self.wait_time += delta_time;

    if self.wait_time > MOVING_PERIOD {
      if clicked {
        self.update_board(x, y, clicked, player);
        return true
      }
    }
    false
  }

  fn update_board(&mut self, x: f32, y: f32, clicked: bool, player: &mut TileState) {
    self.board.update(x, y, clicked, player)
  }
}