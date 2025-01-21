use macroquad::prelude::*;
use rug::ops::Pow;
use rug::{Complex, Float, Integer, Assign};
use std::io::{self, Write};
use sphrs::{Coordinates, ComplexSH, SHEval};

mod equations;
mod ui;

use equations::*;
use ui::*;

#[macroquad::main("Hydrogen Atom :)")]
async fn main() {
  let mut mosiac = Mosaic {
    // TODO we are gonna want to instead send a query
    // to the server to get the probability amplitudes
    // and then perhaps on the client side (here) we use 
    // that data to construct images -> Texture2D
    eigenstates: calculate_n_states(6),
    position: (0., 0.),
    zoom: 0.
  };
  loop {
    if screen_width() < 500. {
      set_fullscreen(true);
      next_frame().await
    } else {
      clear_background(GRAY);
      // update mosiac
      mosiac.update();
      mosiac.draw();
      }
      next_frame().await
    }
}
