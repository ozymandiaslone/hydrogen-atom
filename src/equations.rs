use macroquad::prelude::*;
use rug::ops::Pow;
use rug::{Complex, Float, Integer, Assign};
use std::io::{self, Write};
use sphrs::{Coordinates, ComplexSH, SHEval};


pub fn factorial(num: &Integer) -> Integer {
  let mut result = Integer::from(1);
  let mut i = Integer::from(2);
  while &i <= num {
    result *= &i;
    i += 1;
  }
  result
}


pub fn associated_laguerre_recursive(k: u32, alpha: &Float, x: &Float) -> Float {
  if k == 0 {
    Float::with_val(x.prec(), 1)
  } else if k == 1 {
    alpha + Float::with_val(x.prec(), 1) - x
  } else {
    let minus_two = associated_laguerre_recursive(k - 2, alpha, x);
    let minus_one = associated_laguerre_recursive(k - 1, alpha, x);
    ((Float::with_val(x.prec(), 2 * (k - 1)) + Float::with_val(x.prec(), 1) + alpha - x) * &minus_one
      - (Float::with_val(x.prec(), k - 1) + alpha) * &minus_two)
      / Float::with_val(x.prec(), k)
  }
}

pub fn normalization_constant(n: &Integer, l: &Integer, prec: u32) -> Float {
  let two = Float::with_val(prec, 2);
  let n_minus_l = Integer::from(n - l);
  let n_minus_l_minus_one = Integer::from(&n_minus_l - 1);
  let n_plus_l = Integer::from(n + l);
  let factorial_n_minus_l_minus_one = factorial(&n_minus_l_minus_one);
  let factorial_n_plus_l = factorial(&n_plus_l);
  let numerator = two.pow(3) * factorial_n_minus_l_minus_one;
  let two_n = Integer::from(n*2);
  let denominator = Float::with_val(prec, &two_n * factorial_n_plus_l);
  let norm_const = Float::with_val(prec, numerator) / Float::with_val(prec, denominator);
  norm_const.sqrt()
}

pub fn probability_amplitude(
  n: i64,
  l: i64,
  m: i64,
  radius: f64,
  theta: f64,
  phi: f64,
) -> Complex {
  // Precision of these calculations (in bits) 
  let prec = 256;

  let n = Integer::from(n);
  let l = Integer::from(l);
  let m = Integer::from(m);
  let radius = Float::with_val(prec, radius);
  let theta = Float::with_val(prec, theta);
  let phi = Float::with_val(prec, phi);

//  use the below bohr constant if you want to 
//  deal with meters for length values in this
//  calculation such as radius (r)
//
//  let reduced_bohr = Float::with_val(prec, 5.29177e-11);
//
//  otherwise, using a reduced bohr of 1 puts
//  us in length domain of ~ a hydrogen atom,
//  meaning that instead of passing in very small
//  values of r, we can pass in values of r near 1.
//
  // Now we can go ahead and calculate rho
  let reduced_bohr = Float::with_val(prec, 1);
  let two = Float::with_val(prec, 2);
  let numerator = Float::with_val(prec, &radius * &two);
  let n_float = Float::with_val(prec, &n);
  let denominator = Float::with_val(prec, &n_float * &reduced_bohr);
  let rho_val = Float::with_val(prec, &numerator / &denominator);
  // clone rho for later use :)
  let rho_clone = rho_val.clone();
  
  let norm_const = normalization_constant(&n, &l, prec);

  let half_rho = Float::with_val(prec, &rho_val / 2.);
  let negative_half_rho = Float::with_val(prec, -&half_rho);
  let e_term = Float::with_val(prec, negative_half_rho.exp());

  let rho_term = Float::with_val(prec, rho_val.pow(&(l.to_u32().unwrap())));

  let l_float = Float::with_val(prec, &l);
  let alpha = &two * &l_float + Float::with_val(prec, 1);

  // Compute the associated Laguerre polynomial term
  let k = n.to_u32().unwrap() - l.to_u32().unwrap() - 1;
  let laguerre_term = associated_laguerre_recursive(k, &alpha, &rho_clone);

  // Compute the spherical harmonic term
  let spherical_term = {
    // we gotta cast these coords to f64 because sphrs crate expects that
    // maybe in the future we can calculate our own spherical
    // harmonics using associated legendre functions. we'll see.
    let coordinates = Coordinates::spherical(
        radius.to_f64(),
        theta.to_f64(),
        phi.to_f64(),
    );
    let sh_value = ComplexSH::Spherical.eval(
        l.to_i64().unwrap(),
        m.to_i64().unwrap(),
        &coordinates,
    );
    let sh_real = Float::with_val(prec, sh_value.re);
    let sh_imag = Float::with_val(prec, sh_value.im);
    let sh_rug = Complex::with_val(prec, (sh_real, sh_imag));

    let cs_phase = Float::with_val(prec, (-1.0f64).powf(-m.to_f64()));
    sh_rug * cs_phase
  };

  // combine terms
  let norm_plus_e_term = Float::with_val(prec,&norm_const * &e_term);
  let norm_plus_e_times_rho = Float::with_val(prec, &norm_plus_e_term * &rho_term);
  let radial_term = Float::with_val(prec, &norm_plus_e_times_rho * &laguerre_term);
  let result = Complex::with_val(prec, &spherical_term * &radial_term);

  result
}

pub fn map_pixel_to_continuous_range(x: u32, y: u32, width: u32, height: u32, scale: f64) -> (f64, f64) {
  let max_dim = width.max(height) as f64;
  let scale = scale / max_dim;
  let mapped_x = (x as f64 - width as f64 / 2.0) * scale;
  let mapped_y = (y as f64 - height as f64 / 2.0) * scale;
  (mapped_x, mapped_y)
}

pub fn get_phi_slice(n: i64, l: i64, m: i64, theta: f64, image: &mut Image) -> Texture2D {
  /*
  let screen_width = screen_width();
    let screen_height = screen_height();
    let portrait = if screen_height > screen_width {
      true
    } else {
      false
    };
    let mut image_slice = if !portrait {
      Image::gen_image_color((screen_width * 0.70) as u16, screen_height as u16, WHITE)
    } else {
      Image::gen_image_color(screen_width as u16, (screen_height * 0.70) as u16, WHITE)
    };
    */
    let mut function_values: Vec<Vec<f32>> = vec![vec![0.0; image.height() as usize]; image.width() as usize];
    // draw image
    let mut max_amplitude: f32 = 0.;
    let mut min_amplitude: f32 = 999999999.;
    for x in 0..image.width() {
      for y in 0..image.height() {
        let (eq_x, eq_y) = map_pixel_to_continuous_range(x as u32, y as u32, image.width() as u32, image.height() as u32, (n*n) as f64 * 5.);        
          let r = (eq_x*eq_x + eq_y*eq_y).sqrt();
          let theta = eq_y.atan2(eq_x);
          let amplitude = probability_amplitude(n, l, m, r, theta, 0.);
          let amplitude: f32 = if *amplitude.real() == Float::with_val(amplitude.real().prec(), 0.) && *amplitude.imag() == Float::with_val(amplitude.imag().prec(), 0.) { 
            0.
          } else { 
            amplitude.real().to_f32()*amplitude.real().to_f32() + amplitude.imag().to_f32()*amplitude.imag().to_f32()
          };
          if amplitude > max_amplitude {
            max_amplitude = amplitude;
          }
          if amplitude < min_amplitude {
            min_amplitude = amplitude;
          }
          function_values[x][y] = amplitude;
      }
    }

    // normalize values
    for x in 0..function_values.len() {
      for y in 0..function_values[x].len() {
        if function_values[x][y] == 0. {
          image.set_pixel(x as u32, y as u32, Color::new(0., 0., 0., 1.));
        } else {
          let normalized = (function_values[x][y] - min_amplitude) / (max_amplitude - min_amplitude);
//          let col =  Color::new(normalized, normalized, normalized, 1.);
          let col = Color::new(normalized, normalized, normalized, 1.);
          image.set_pixel(x as u32, y as u32, col);
        }
      }
    }
    Texture2D::from_image(&image)
}
pub fn get_theta_slice(n: i64, l: i64, m: i64, theta: f64) -> Texture2D {
  let screen_width = screen_width();
    let screen_height = screen_height();
    let portrait = if screen_height > screen_width {
      true
    } else {
      false
    };
    let mut image_slice = if !portrait {
      Image::gen_image_color((screen_width * 0.70) as u16, screen_height as u16, WHITE)
    } else {
      Image::gen_image_color(screen_width as u16, (screen_height * 0.70) as u16, WHITE)
    };
    let mut function_values: Vec<Vec<f32>> = vec![vec![0.0; image_slice.height() as usize]; image_slice.width() as usize];
    // draw image
    let mut max_amplitude: f32 = 0.;
    let mut min_amplitude: f32 = 999999999.;
    for x in 0..image_slice.width() {
      for y in 0..image_slice.height() {
        let (eq_x, eq_y) = map_pixel_to_continuous_range(x as u32, y as u32, image_slice.width() as u32, image_slice.height() as u32, 50.);        
          let r = (eq_x*eq_x + eq_y*eq_y).sqrt();
          let phi = eq_y.atan2(eq_x);
          let amplitude = probability_amplitude(n, l, m, r, std::f64::consts::PI / 2., phi);
          let amplitude: f32 = if *amplitude.real() == Float::with_val(amplitude.real().prec(), 0.) && *amplitude.imag() == Float::with_val(amplitude.imag().prec(), 0.) { 
            0.
          } else { 
            amplitude.real().to_f32()*amplitude.real().to_f32() + amplitude.imag().to_f32()*amplitude.imag().to_f32()
          };
          if amplitude > max_amplitude {
            max_amplitude = amplitude;
          }
          if amplitude < min_amplitude {
            min_amplitude = amplitude;
          }
          function_values[x][y] = amplitude;
      }
    }

    // normalize values
    for x in 0..function_values.len() {
      for y in 0..function_values[x].len() {
        let normalized = (function_values[x][y] - min_amplitude) / (max_amplitude - min_amplitude);
        let col =  Color::new(normalized, normalized, normalized, 1.);
        image_slice.set_pixel(x as u32, y as u32, col);
      }
    }
    Texture2D::from_image(&image_slice)
}





struct PolarCamera {
  camera: Camera3D,
  x: f64,
  y: f64,
  z: f64,
  r: f64,
  theta: f64,
  phi: f64
}
impl PolarCamera {
  fn update(&mut self) {
    let d_theta = 0.1;
    // orbit via phi
    self.theta += d_theta; 
    if self.theta > 2.0 * std::f64::consts::PI {
      self.theta -= 2.0 * std::f64::consts::PI;
    }
    self.x = self.r * self.theta.sin() * self.phi.cos();
    self.y = self.r * self.theta.sin() * self.phi.sin();
    self.z = self.r * self.theta.cos();
    self.camera = Camera3D{
      position: vec3(self.x as f32, self.y as f32, self.z as f32),
      up: vec3(0., 1., 0.),
      target: vec3(0., 0., 0.,),
      ..Default::default()
    };
  } 

}


