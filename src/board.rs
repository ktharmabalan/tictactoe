use std::collections::HashMap;
// use std::fmt;

use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

// mod tile;

use game::GAME_SIZE;
use tile::{ Tile };

// const BOARD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BOARD_COLOR: Color = [(34.0/255.0), (41.0/255.0), (71.0/255.0), 1.0];
// const BOARD_COLOR: Color = [(15/255) as f32, (20/255) as f32, (45/255) as f32, 1.0];

pub struct Board {
  x: i32,
  y: i32,
  width: i32,
  height: i32,
  tiles: HashMap<String, Tile>,
}

impl Board {
  pub fn new(x: i32, y: i32, width: i32, height: i32, rows: i32, cols: i32) -> Board {
    let border_count = rows + 1;
    let border_ratio = 0.1;
    let offsetsize = border_count as f32 * border_ratio;
    let tilesize = (width as f32 - (offsetsize)) / rows as f32;
    // println!("width {}, rows {}, tilesize {}, bordercount {}, offsetsize {}", width,  rows, tilesize, border_count, offsetsize);

    let mut tiles = HashMap::new();
    let mut key = String::from("");
    for row in 0 .. rows {
      for col in 0 .. cols {
        key = format!("{}:{}", row, col);
        tiles.insert(key, Tile::new(row, col, tilesize as f64, border_ratio as f64));
      }
    }

    Board {
      x,
      y,
      width,
      height,
      tiles,
    }
  }

  pub fn draw(&self, con: &Context, g: &mut G2d) {
    rectangle(
      BOARD_COLOR,
      [
        self.x as f64 * GAME_SIZE,
        self.y as f64 * GAME_SIZE,
        self.width as f64 * GAME_SIZE,
        self.height as f64 * GAME_SIZE
      ],
      con.transform,
      g,
    );
    
    for (_key, tile) in &self.tiles {
      tile.draw(&con, g);
    }

    // let offset = (self.width as f64) * GAME_SIZE / 3.0;
    // for i in 1..3 {
    //   line(
    //     // [(34/255) as f32, (41/255) as f32, (71/255) as f32, 1.0],
    //     [(15/255) as f32, (20/255) as f32, (45/255) as f32, 1.0],
    //     0.1 * GAME_SIZE,
    //     [
    //       offset * i as f64,
    //       0.0,
    //       offset * i as f64,
    //       self.height as f64 * GAME_SIZE,
    //     ],
    //     con.transform,
    //     g,
    //   );

    //   line(
    //     [(34/255) as f32, (41/255) as f32, (71/255) as f32, 1.0],
    //     // [(15/255) as f32, (20/255) as f32, (45/255) as f32, 1.0],
    //     0.1 * GAME_SIZE,
    //     [
    //       0.0,
    //       offset * i as f64,
    //       self.width as f64 * GAME_SIZE,
    //       offset * i as f64,
    //     ],
    //     con.transform,
    //     g,
    //   );
    // }
  }
}