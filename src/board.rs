// use std::collections::HashMap;
// use std::fmt;

use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

use game::GAME_SIZE;
use tile::{ Tile, TileState, variant_eq };

// const BOARD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BOARD_COLOR: Color = [(34.0/255.0), (41.0/255.0), (71.0/255.0), 1.0];
// const BOARD_COLOR: Color = [(15/255) as f32, (20/255) as f32, (45/255) as f32, 1.0];

pub struct Board {
  x: i32,
  y: i32,
  width: i32,
  height: i32,
  // rows: i32,
  // cols: i32,
  // tiles: HashMap<String, Tile>,
  tiles: Vec<Vec<Tile>>,
  tile_size: f32,
}

impl Board {
  pub fn new(x: i32, y: i32, width: i32, height: i32, rows: i32, cols: i32) -> Board {
    let border_count = rows + 1;
    let border_ratio = 0.1;
    let offsetsize = border_count as f32 * border_ratio;
    let tile_size = (width as f32 - (offsetsize)) / rows as f32;
    // println!("width {}, rows {}, tile_size {}, bordercount {}, offsetsize {}", width,  rows, tile_size, border_count, offsetsize);

    Board {
      x,
      y,
      width,
      height,
      // rows,
      // cols,
      tile_size,
      tiles: {
        // let mut tiles = HashMap::new();
        // let mut key = String::from("");
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        for row in 0 .. rows {
          let mut temp_tiles: Vec<Tile> = Vec::new();
          for col in 0 .. cols {
            temp_tiles.push(Tile::new(row, col, tile_size as f64, border_ratio as f64))
            // key = format!("{}:{}", row, col);
            // tiles.insert(key, Tile::new(row, col, tile_size as f64, border_ratio as f64));
          }
          tiles.push(temp_tiles);
        }
        tiles
      },
    }
  }

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

    // for (_key, tile) in &mut self.tiles {
    //   tile.draw(&con, g);
    // }
    for (_row_idx, rows) in &mut self.tiles.iter().enumerate() {
      for (_col_idx, col) in &mut rows.iter().enumerate() {
        col.draw(&con, g);
      }
    }
  }

  pub fn update(&mut self, x: f32, y: f32, clicked: bool, player: &mut TileState) {
    // let key = format!("{}:{}", ((x / self.tile_size) as f64 / GAME_SIZE) as i32, ((y / self.tile_size) as f64 / GAME_SIZE) as i32);
    // println!("{}", key);

    // if self.tiles.contains_key(&key) && clicked {
    //   let tile = self.tiles.get_mut(&key).unwrap();
    //   if 
    //     (x / self.tile_size) as f64 / GAME_SIZE > tile.row as f64
    //     && (x / self.tile_size) as f64 / GAME_SIZE < (tile.row + 1) as f64
    //     && (y / self.tile_size) as f64 / GAME_SIZE > tile.col as f64
    //     && (y / self.tile_size) as f64 / GAME_SIZE < (tile.col + 1) as f64
    //     {
    //     // let updated = tile.clicked(player);
    //     // self.check_win(player);
    //     // return updated;
    //     tile.clicked(player);
    //   }
    // }
    let (row, col) = ((((x / self.tile_size) as f64 / GAME_SIZE) as i32), (((y / self.tile_size) as f64 / GAME_SIZE) as i32));
    
    if row >= 0 && col >= 0 && self.tiles.len() > row as usize && self.tiles[row as usize].len() > col as usize && clicked {
      self.tiles[row as usize][col as usize].clicked(player);
    }
  }

  pub fn check_win(&self, player: &TileState) {
    match player {
      TileState::None => (),
      _ => {
        // col 1
        let mut a = &self.tiles[0][0].tile_state;
        let mut b = &self.tiles[0][1].tile_state;
        let mut c = &self.tiles[0][2].tile_state;
        // col 2
        let mut d = &self.tiles[1][0].tile_state;
        let mut e = &self.tiles[1][1].tile_state;
        let mut f = &self.tiles[1][2].tile_state;
        // col 3
        let mut g = &self.tiles[2][0].tile_state;
        let mut h = &self.tiles[2][1].tile_state;
        let mut i = &self.tiles[2][2].tile_state;

        // cols
        if variant_eq(a, b) && variant_eq(a, c) && !variant_eq(a, &TileState::None) {
          println!("{:?} COL 1", a);
        }
        if variant_eq(d, e) && variant_eq(d, f) && !variant_eq(d, &TileState::None) {
          println!("{:?} COL 2", d);
        }
        if variant_eq(g, h) && variant_eq(g, i) && !variant_eq(g, &TileState::None) {
          println!("{:?} COL 3", g);
        }
        // rows
        if variant_eq(a, d) && variant_eq(a, g) && !variant_eq(a, &TileState::None) {
          println!("{:?} ROW 1", a);
        }
        if variant_eq(b, e) && variant_eq(b, h) && !variant_eq(b, &TileState::None) {
          println!("{:?} ROW 2", d);
        }
        if variant_eq(c, f) && variant_eq(c, i) && !variant_eq(c, &TileState::None) {
          println!("{:?} ROW 3", g);
        }
        // Left top -> Right down
        if variant_eq(a, e) && variant_eq(a, f) && !variant_eq(a, &TileState::None) {
          println!("{:?} LEFT TO RIGHT 1", a);
        }
        // Right top -> Left down
        if variant_eq(c, e) && variant_eq(c, i) && !variant_eq(c, &TileState::None) {
          println!("{:?} RIGHT TO LEFT 1", a);
        }
      },
    }
    // match player {
    //   TileState::Player1 => {
    //     if self.tiles[0][0].tile_state == TileState::Player1 {

    //     }
    //   },
    //   TileState::Player2 => {
    //   },
    //   _ => (),
    // };
    // let mut key = String::from("");
    // for row in 0 .. self.rows {
    //   for col in 0 .. self.cols {
    //     key = format!("{}:{}", row, col);
    //     if row == 1 {
    //       if self.tiles[&key].tile_state == player {
    //         row1.push(row);
    //         if row1.len() == 3 {
    //           println!("WIN");
    //         }
    //       }
    //     }
    //   }
    // }
  }
}