use macroquad::prelude::*;
use rug::ops::Pow;
use rug::{Complex, Float, Integer, Assign};
use std::io::{self, Write};
use sphrs::{Coordinates, ComplexSH, SHEval};

mod equations;
mod ui;

use equations::*;
use ui::*;

fn calculate_possible_eigenstates_recursive(n: i64) -> i64 {
  //base case
  if n == 1 {
    1
  } else {
    calculate_possible_eigenstates_recursive(n - 1) + n*n
  }
}

fn calculate_n_states(n: i64) -> Vec<Texture2D> {
  let portrait = if screen_width() > screen_height() { false } else { true };
  let total_eigenstates = calculate_possible_eigenstates_recursive(n);
//  let xsq = screen_width() * screen_height() * 0.66;
  let mut textures: Vec<Texture2D> = Vec::new();
  // dim of each image
  println!("CALCULATING {} EIGENSTATES", n);
  //let x = xsq.sqrt();
  let x = if portrait { screen_width() / 5.} else {screen_width() * 0.66 / 5. };
  for tn in 1..=n {
    for l in 0..tn {
      for m in -l..=l {
        println!("n: {}, l: {}, m: {}", tn, l, m);
        let mut image = Image::gen_image_color(x as u16, x as u16, WHITE);
        textures.push({get_phi_slice(
          tn,
          l,
          m,
          0.,
          &mut image
        )});
      }
    }
  }

  println!("DONE");
  let per_row = if portrait { x as i64 / screen_width() as i64 } else {
    ( x / screen_width() * 0.66) as i64 + 1
  }; 
  let num_cols = if portrait { (per_row as f32 / screen_height() * 0.66) as i64} else {
    x as i64 / screen_height() as i64 + 1
  };
  /*
  println!("x: {}", x);
  println!("PER ROW: {}", per_row);
  println!("NUM COLS: {}",num_cols);
  for i in 0..per_row {
    for k in 0..num_cols {
      let idx = ((i * per_row) + k) as usize;
      print!("idx: {}", idx);
      draw_texture(&textures[idx], 0. + (i as f32 * x), 0. + (k as f32 * x), WHITE); 
      println!("DREW TEXTURE @ {},{}", (0. + (i as f32 * x)), (0. + k as f32 * x));
    }
  }
  */
  textures
}

fn draw_textures_from_vec(textures: &Vec<Texture2D>) {
  let portrait = if screen_width() > screen_height() { false } else { true };
  let x = textures[0].width();
  let total_textures = textures.len();
  let rows = (total_textures as f32).sqrt().floor() as usize;
  let cols = if rows * rows == total_textures { rows } else { rows + 1 };
  for i in 0..rows{
    for k in 0..cols{
      let idx = ((i * cols) + k) as usize;
      if idx >= textures.len() {
        break
      }
      let draw_x = i as f32 * x;
      let draw_y = k as f32 * x;
      draw_texture(&textures[idx], draw_x, draw_y, WHITE);
    }
  }
}

#[macroquad::main("Hydrogen Atom :)")]
async fn main() {
  let mut initial_dims = 0.;
  let mut textures = Vec::new();
  loop {

    if screen_width() < 500. {
      set_fullscreen(true);
      next_frame().await
    } else {
      clear_background(GRAY);
      let portrait = if screen_width() > screen_height() { false } else { true };
      if portrait {
        // 
        //
      } else{
        if screen_width() * screen_height() != initial_dims {
          println!("UPDATING SCREEN");
          initial_dims = screen_width() * screen_height();
          textures = calculate_n_states(5);
        }
      }
      draw_textures_from_vec(&textures);
//      draw_texture(&textures[0], 0., 0., WHITE);
      next_frame().await

    }
      }
}
