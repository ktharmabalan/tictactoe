use std::collections::HashMap;
// use std::fmt;

use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

use game::GAME_SIZE;
use tile::{ Tile, TileState };

// const BOARD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BOARD_COLOR: Color = [(34.0/255.0), (41.0/255.0), (71.0/255.0), 1.0];
// const BOARD_COLOR: Color = [(15/255) as f32, (20/255) as f32, (45/255) as f32, 1.0];

pub struct Board {
  x: i32,
  y: i32,
  width: i32,
  height: i32,
  rows: i32,
  cols: i32,
  tiles: HashMap<String, Tile>,
  // tiles: [Tile; 4],
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
      rows,
      cols,
      tiles,
      tile_size,
    }
  }

  // fn check_win(&self, player: &mut TileState) {
  //   let mut row1: Vec<i32> = Vec::new();
  //   // let row2: Vec<i32> = Vec::new();
  //   // let row3: Vec<i32> = Vec::new();
  //   // let col1: Vec<i32> = Vec::new();
  //   // let col2: Vec<i32> = Vec::new();
  //   // let col3: Vec<i32> = Vec::new();
  //   let mut key = String::from("");
  //   for row in 0 .. self.rows {
  //     for col in 0 .. self.cols {
  //       key = format!("{}:{}", row, col);
  //       if row == 1 {
  //         if self.tiles[&key].tile_state == player {
  //           row1.push(row);
  //           if row1.len() == 3 {
  //             println!("WIN");
  //           }
  //         }
  //       }
  //     }
  //   }
  // }

  pub fn draw(&mut self, con: &Context, g: &mut G2d) {
    // , x: f32, y: f32, clicked: bool, player: &mut TileState
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
      tile.draw(&con, g);
    }
  }

  pub fn update(&mut self, x: f32, y: f32, clicked: bool, player: &mut TileState) {
    let key = format!("{}:{}", ((x / self.tile_size) as f64 / GAME_SIZE) as i32, ((y / self.tile_size) as f64 / GAME_SIZE) as i32);
    println!("{}", key);


    if self.tiles.contains_key(&key) && clicked {
      let tile = self.tiles.get_mut(&key).unwrap();
      if 
        (x / self.tile_size) as f64 / GAME_SIZE > tile.row as f64
        && (x / self.tile_size) as f64 / GAME_SIZE < (tile.row + 1) as f64
        && (y / self.tile_size) as f64 / GAME_SIZE > tile.col as f64
        && (y / self.tile_size) as f64 / GAME_SIZE < (tile.col + 1) as f64
        {
        // let updated = tile.clicked(player);
        // self.check_win(player);
        // return updated;
        tile.clicked(player);
      }
    }
  }
}