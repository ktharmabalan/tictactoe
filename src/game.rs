// Import piston
use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;
// (34%, 21%, 0%, 73%)
const GAME_COLOR: Color = [0.17, 0.21, 0.27, 1.0];

// Moving period/restart time
const MOVING_PERIOD: f64 = 0.1; // lower for faster update, greater for slower update
const RESTART_TIME: f64 = 1.0;  // wait time in seconds

pub const GAME_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
  (game_coord as f64) * GAME_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
  to_coord(game_coord) as u32
}

pub struct Game {
  width: i32,
  height: i32,
  game_over: bool,
  wait_time: f64,
}

impl Game {
  pub fn new(width: i32, height: i32) -> Game {
    Game {
      wait_time: 0.0,
      game_over: false,
      width,
      height,
    }
  }

  pub fn draw(&self, con: &Context, g: &mut G2d) {
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
  }
}