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
  tile_size: f32,
}

impl Board {
  pub fn new(x: i32, y: i32, width: i32, height: i32, rows: i32, cols: i32) -> Board {
    let border_count = rows + 1;
    let border_ratio = 0.1;
    let offsetsize = border_count as f32 * border_ratio;
    let tile_size = (width as f32 - (offsetsize)) / rows as f32;
    println!("width {}, rows {}, tile_size {}, bordercount {}, offsetsize {}", width,  rows, tile_size, border_count, offsetsize);

    let mut tiles = HashMap::new();
    let mut key = String::from("");
    for row in 0 .. rows {
      for col in 0 .. cols {
        key = format!("{}:{}", row, col);
        tiles.insert(key, Tile::new(row, col, tile_size as f64, border_ratio as f64));
      }
    }

    Board {
      x,
      y,
      width,
      height,
      tiles,
      tile_size,
    }
  }

  pub fn draw(&mut self, con: &Context, g: &mut G2d, x: f32, y: f32, clicked: bool) {
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

    for (_key, tile) in &mut self.tiles {
      // if tile.row  >= x && x <= (tile.row * tile.tile_size) * GAME_SIZE {
      if clicked {
        // println!("{}, {}", x as f64 / GAME_SIZE, y as f64 / GAME_SIZE);
        // if (tile.row as f64 * tile.tile_size * GAME_SIZE) + (tile.offset_size * GAME_SIZE * ((tile.row + 1) as f64)) >= x as f64
        //   && ((tile.row + 2) as f64 * tile.tile_size * GAME_SIZE) + (tile.offset_size * GAME_SIZE * ((tile.row + 2) as f64)) <= x as f64
        //   // && (tile.tile_size * GAME_SIZE) <= x as f64
        //   && (tile.col as f64 * tile.tile_size * GAME_SIZE) + (tile.offset_size * GAME_SIZE * ((tile.col + 1) as f64)) >= y as f64
        //   && ((tile.col + 2) as f64 * tile.tile_size * GAME_SIZE) + (tile.offset_size * GAME_SIZE * ((tile.col + 2) as f64)) >= y as f64 {
        //   // && (tile.tile_size * GAME_SIZE) <= y as f64 {
        if 
          (x / self.tile_size) as f64 / GAME_SIZE > tile.row as f64
          && (x / self.tile_size) as f64 / GAME_SIZE < (tile.row + 1) as f64
          && (y / self.tile_size) as f64 / GAME_SIZE > tile.col as f64
          && (y / self.tile_size) as f64 / GAME_SIZE < (tile.col + 1) as f64
          {
          tile.clicked();
        }
        // let updateTile = &self.tiles.get(key).unwrap();
        // updateTile.clicked();
      }
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