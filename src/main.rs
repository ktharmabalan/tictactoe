extern crate piston_window;

mod game;
mod board;
mod tile;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use game::to_coord_u32;

use board::Board;

const CLEAR_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

fn main() {
  let (width, height) = (24, 24);
  let (cols, rows) = (6, 6);

  let mut window: PistonWindow = WindowSettings::new(
    "Tic Tac Toe",
    [to_coord_u32(width), to_coord_u32(height)],
  )
  .exit_on_esc(true)
  .build()
  .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

  let game = Game::new(width, height);
  let board = Board::new(
    0,
    0,
    width,
    height,
    cols,
    rows,
  );

  while let Some(e) = window.next() {
    window.draw_2d(&e, |c, g| {
      clear(CLEAR_COLOR, g);
      game.draw(&c, g);
      board.draw(&c, g);
    });
  }
}
