extern crate piston_window;

mod game;
mod board;
mod tile;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use game::to_coord_u32;

const CLEAR_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

fn main() {
  let (width, height) = (24, 24);

  let mut window: PistonWindow = WindowSettings::new(
    "Tic Tac Toe",
    [to_coord_u32(width), to_coord_u32(height)],
  )
  .exit_on_esc(true)
  .build()
  .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

  let mut game = Game::new(width, height);
  let mut clicked = false;
  let mut x = 0.0;
  let mut y = 0.0;

  while let Some(e) = window.next() {
    if let Some(pos) = e.mouse_cursor_args() {
      // println!("{:?}", pos);
      x = pos[0] as f32;
      y = pos[1] as f32;
    };

    if let Some(button) = e.press_args() {
      if button == Button::Mouse(MouseButton::Left) {
        clicked = true;
        // println!("{}, {}", x, y);
      }
    };

    window.draw_2d(&e, |c, g| {
      clear(CLEAR_COLOR, g);
      game.draw(&c, g, x, y, clicked);
      if clicked {
        clicked = false;
      }
    });

  }
}
