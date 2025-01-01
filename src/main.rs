use num_complex::Complex;
use sphrs::{Coordinates, ComplexSH, SHEval};

fn main() {
  let r = 1.; 
  let theta = std::f64::consts::FRAC_PI_2; 
//  let phi = std::f64::consts::FRAC_PI_2; // pi / 2
//  let theta = 0.;
  let phi = 0.;                                        //
  let coords = Coordinates::spherical(r, theta, phi);
  let l = 1;
  let m = -1;

  let condon_shortley_phase_factor = f64::powf(-1., -m as f64);
  
  // Does NOT apply Condon-Shortley phase factor (i think)
  let harmonic_value = ComplexSH::Spherical.eval(l, m, &coords);
  // so here i am manually applying the Condon-Shortley phase factor
  let harmonic_value = harmonic_value * condon_shortley_phase_factor;

  println!("l: {} m: {} Harmonic Value @ orientation [in radians] (phi = {}, theta = {}): {}", l, m, phi, theta, harmonic_value);
}
