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

fn grab_stdin() -> String {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  input
}

fn rho(n: u32, r: &Float) -> Float {
//    let reduced_bohr = Float::with_val(r.prec(), 5.29177e-11);
  let reduced_bohr = Float::with_val(r.prec(), 1);
    (Float::with_val(r.prec(), 2) * r) / (Float::with_val(r.prec(), n) * reduced_bohr)
}

fn associated_laguerre_recursive(k: u32, alpha: &Float, x: &Float) -> Float {
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
fn _oldassociated_laguerre_recursive(k: u32, alpha: f64, x: f64) -> f64 {
  // Base case numero uno
    if k == 0 {
        1.0
  // And numero dos
    } else if k == 1 {
        1.0 + alpha - x

  // Recursion
    } else {
        let minus_two = _oldassociated_laguerre_recursive(k - 2, alpha, x);
        let minus_one = _oldassociated_laguerre_recursive(k - 1, alpha, x);
        
        // The Wikipedia definion of the recurrence relation is: 
        //
        // L_alpha_k+1(x) = ((2k + 1 + alpha - x)(L_alpha_k(x)) - (k + alpha)(L_alpha_k-1(x))) / (k + 1)
        //
        // However, this is defining L_alpha_k+1(x) in terms of L_alpha_k(x) and L_alpha_k-1(x). We would rather
        // compute L_alpha_k(x) directly in terms of L_alpha_k-1 and L_alpha_k-2
        //
        ((2.0 * (k - 1) as f64 + 1.0 + alpha - x) * minus_one - ((k - 1) as f64 + alpha) * minus_two) / (k as f64)
    }
}

//fn _oldnormalization_constant(n: i64, l: i64) -> f64 {
//  let reduced_bohr: f64 = 5.29177e-11;
//  return ((2./reduced_bohr).powi(3) * (factorial(n - l - 1)/((2*n*factorial(n+l))))as f64).sqrt()
//}

fn normalization_constant(n: &Integer, l: &Integer, prec: u32) -> Float {
  let reduced_bohr = Float::with_val(prec, 1);
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
/*
fn probability_amplitude(n: i64, l: i64, m: i64, radius: f64, theta: f64, phi: f64) -> Complex<f64> {
  let coordinates = Coordinates::spherical(radius, theta, phi);
  let condon_shortley_phase_factor = f64::powf(-1., -m as f64);
  let norm_const = normalization_constant(n, l);
  let e_term = std::f64::consts::E.powf(-rho(n,radius) / 2.);
  let rho_term = rho(n,radius).powi(l as i32);
  let laguerre_term = associated_laguerre_recursive(n as u32 - l as u32 -1, 2.*l as f64+1., rho(n,radius));
  let spherical_term = ComplexSH::Spherical.eval(l,m,&coordinates) * condon_shortley_phase_factor;
  println!("RHO: {}", rho(n,radius));
  println!("Normalization constant: {}", norm_const);
  println!("e term: {}", e_term);
  println!("rho term: {}", rho_term);
  println!("Laguerre_term: {}", laguerre_term);
  println!("Spherical_term: {}", spherical_term);
  
  return norm_const * e_term * rho_term * laguerre_term * spherical_term

  //return 
  //  normalization_constant(n, l)
  //  * std::f64::consts::E.powf(-rho(n,radius) / 2.)
  //  * rho(n,radius).powi(l as i32)
  //  * associated_laguerre_recursive(n as u32 -l as u32 -1, 2.*l as f64+1., rho(n, radius))
  //  * ComplexSH::Spherical.eval(l, m, &coordinates) * condon_shortley_phase_factor
}
*/
fn probability_amplitude(
  n: i64,
  l: i64,
  m: i64,
  radius: f64,
  theta: f64,
  phi: f64,
) -> Complex {
  // Set the desired precision (e.g., 128 bits)
  let prec = 64;

  let n = Integer::from(n);
  let l = Integer::from(l);
  let m = Integer::from(m);
  let radius = Float::with_val(prec, radius);
  let theta = Float::with_val(prec, theta);
  let phi = Float::with_val(prec, phi);
//  meters
//  let reduced_bohr = Float::with_val(prec, 5.29177e-11);

  let reduced_bohr = Float::with_val(prec, 1);
  let two = Float::with_val(prec, 2);
  // Calculate numerator: radius * 2
  let numerator = Float::with_val(prec, &radius * &two);
  // Convert n to a Float
  let n_float = Float::with_val(prec, &n);
  // Calculate denominator: n_float * reduced_bohr
  let denominator = Float::with_val(prec, &n_float * &reduced_bohr);
  // Now compute rho_val: numerator / denominator
  let rho_val = Float::with_val(prec, &numerator / &denominator);
  let rho_clone = rho_val.clone();
  
  let norm_const = normalization_constant(&n, &l, prec);

  // Compute the exponential term
  let half_rho = Float::with_val(prec, &rho_val / 2.);
  let negative_half_rho = Float::with_val(prec, (-&half_rho));
  let e_term = Float::with_val(prec, negative_half_rho.exp());

  // Compute the rho term
  let rho_term = Float::with_val(prec, rho_val.pow(&(l.to_u32().unwrap())));

  // Compute alpha
  let l_float = Float::with_val(prec, &l);
  let alpha = &two * &l_float + Float::with_val(prec, 1);

  // Compute the associated Laguerre polynomial term
  let k = n.to_u32().unwrap() - l.to_u32().unwrap() - 1;
  let laguerre_term = associated_laguerre_recursive(k, &alpha, &rho_clone);

  // Compute the spherical harmonic term
  let spherical_term = {
    // Use f64 for the coordinates as sphrs crate expects f64
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

  // Combine all terms
  let norm_plus_e_term = Float::with_val(prec,&norm_const * &e_term);
  let norm_plus_e_times_rho = Float::with_val(prec, &norm_plus_e_term * &rho_term);
  let radial_term = Float::with_val(prec, &norm_plus_e_times_rho * &laguerre_term);
  let result = Complex::with_val(prec, &spherical_term * &radial_term);

  result
}

fn main() {
  print!("Enter a value n: ");
  io::stdout().flush().unwrap();
  let mut nstr = grab_stdin();
  nstr = nstr.trim().to_string();
  let mut n: i64 = nstr.parse().unwrap();
  println!("n: {}", n);

  print!("Enter a value l: ");
  io::stdout().flush().unwrap();
  let mut lstr = grab_stdin();
  lstr = lstr.trim().to_string();
  let mut l: i64 = lstr.parse().unwrap();
  println!("l: {}", l);

  print!("Enter a value m: ");
  io::stdout().flush().unwrap();
  let mut mstr = grab_stdin();
  mstr = mstr.trim().to_string();
  let mut m: i64 = mstr.parse().unwrap();
  println!("m: {}", m);

  print!("Enter a value for the radius: ");
  io::stdout().flush().unwrap();
  let mut rstr = grab_stdin();
  rstr = rstr.trim().to_string();
  let mut radius: f64 = rstr.parse().unwrap();
  println!("Radius: {}", radius);

  print!("Enter a value for Theta: ");
  io::stdout().flush().unwrap();
  let mut thetastr = grab_stdin();
  thetastr = thetastr.trim().to_string();
  let mut theta: f64 = thetastr.parse().unwrap();
  println!("Theta: {}", theta);

  print!("Enter a value for Phi: ");
  io::stdout().flush().unwrap();
  let mut phistr = grab_stdin();
  phistr = phistr.trim().to_string();
  let mut phi: f64 = phistr.parse().unwrap();
  println!("Phi: {}", phi);
  
  let prob_amplitude = probability_amplitude(n, l, m, radius, theta, phi);
  
  println!("PROBABILIY AMPLITUDE FOR THESE VALUES == REAL: {} || IMAGINARY: {}", prob_amplitude.real(), prob_amplitude.imag());
}
