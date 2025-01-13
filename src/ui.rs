use macroquad::prelude::*;
use std::any::Any;

pub struct Mosaic {
  pub eigenstates: Vec<Box<dyn UIElement>>,
  pub position: (f32, f32),
  pub zoom: f32,
}
impl UIElement for Mosaic {
  fn draw(&self) {
    for i in 0..self.eigenstates.len() {
      self.eigenstates[i].draw();
    }
  }
  fn update(&mut self) {
    // move and zoom the image based on mouse input
    // (this means applying an offset to each eigenstate's
    // position vector)
    
    let mouse_pos = mouse_position();
    if mouse_pos.0 > screen_width() * 0.9 {
      self.position.0 -= 3.;
    } else if mouse_pos.0 < screen_width() * 0.1 {
      self.position.0 += 3.;
    }
    if mouse_pos.1 > screen_height() * 0.9 {
      self.position.1 -= 3.;
    } else if mouse_pos.1 < screen_height() * 0.1 {
      self.position.1 += 3.;
    }
     
    let portrait = if screen_width() > screen_height() { false } else { true };
    let total_eigenstates = self.eigenstates.len();
/*
    let rows = (total_eigenstates as f32).sqrt().floor() as usize;
    let cols = if rows * rows == total_eigenstates { rows } else { rows + 1 };
*/
    let rows = (total_eigenstates as f32).sqrt().ceil() as usize; // Always round up for rows
    let cols = ((total_eigenstates as f32) / rows as f32).ceil() as usize; // Compute cols directly
    let x: f32 = if let Some(eigenstate) = self.eigenstates[0].as_any().downcast_ref::<Eigenstate>() {
      eigenstate.texture.width()
    } else {
      0.
    };
    for i in 0..rows {
      for k in 0..cols {
        let idx = (i * cols) + k;
        if idx < self.eigenstates.len(){
          let xpos = (k as f32 * x) + self.position.0;
          let ypos = (i as f32 * x) + self.position.1;
          self.eigenstates[idx].set_pos((xpos, ypos));
          if xpos == 0. && ypos == 0. {
            println!("UH OH set i: {}, k: {} to 0", i, k);
          }
        }
      }
    }
  }
  fn as_any(&self) -> &dyn Any { self }
  fn set_pos(&mut self, pos: (f32,f32)) {
    self.position = pos;
  }
}

pub struct Eigenstate {
  pub texture: Texture2D,
  pub hovered: bool,
  pub position: (f32, f32),
  pub info: String,
}
impl UIElement for Eigenstate {
  fn draw(&self) {
    if !self.hovered {
      draw_texture(&self.texture, self.position.0, self.position.1, WHITE); 
      draw_text(&self.info, self.position.0 + 3., self.position.1 + 10., self.texture.width() / 7., WHITE);

    } else {

    }
  }
  fn update(&mut self) {

  }
  fn set_pos(&mut self, pos: (f32,f32)) {
    self.position = pos;
  }
  fn as_any(&self) -> &dyn Any  { self }
}

pub trait UIElement: Any {
  fn draw(&self);
  fn update(&mut self);
  fn as_any(&self) -> &dyn Any;
  fn set_pos(&mut self, pos: (f32, f32));
}
