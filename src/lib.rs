extern crate cxx;

use libc::c_double;

#[cxx::bridge]
mod ffi {}

extern "C" {
  fn implied_volatility_from_a_transformed_rational_guess(
    price: c_double,
    f: c_double,
    k: c_double,
    t: c_double,
    q: c_double,
  ) -> c_double;
  fn black(
    f: c_double,
    k: c_double,
    sigma: c_double,
    t: c_double,
    q: c_double,
  ) -> c_double;
}

pub fn iv_implied_volatility_from_a_transformed_rational_guess(
  price: f64,
  forward: f64,
  strike: f64,
  time: f64,
  quote: f64,
) -> f64 {
  unsafe {
    implied_volatility_from_a_transformed_rational_guess(
      price, forward, strike, time, quote,
    )
  }
}

pub fn iv_black(
  forward: f64,
  strike: f64,
  sigma: f64,
  time: f64,
  quote: f64,
) -> f64 {
  unsafe { black(forward, strike, sigma, time, quote) }
}

/// Returns Black-Scholes option price.
pub fn black_scholes(
  price: f64,
  strike: f64,
  time: f64,
  rate: f64,
  sigma: f64,
  quote: f64,
) -> f64 {
  let deflator = (-rate * time).exp();
  let forward = price / deflator;
  iv_black(forward, strike, sigma, time, quote) * deflator
}

/// Returns flag value for different kind of options.
pub fn binary_flag(ch: char) -> f64 {
  match ch {
    'c' => 1.0,
    'p' => -1.0,
    _ => panic!("only 'c' (CALL) or 'p' (PUT) accepted"),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_black() {
    let price = iv_black(100.0, 100.0, 0.2, 0.5, binary_flag('c'));
    assert!((5.637197779701664 - price).abs() < f64::EPSILON);
  }

  #[test]
  fn test_black_scholes() {
    let price = black_scholes(100.0, 90.0, 0.5, 0.01, 0.2, binary_flag('c'));
    assert!((12.11158143496968 - price).abs() < f64::EPSILON);
  }

  #[test]
  fn test_black_scholes_1() {
    let price =
      black_scholes(100.0, 100.0, 0.1123, 0.05, 0.04444, binary_flag('c'));
    println!("{}", price);
    //assert!((12.11158143496968 - price).abs() < f64::EPSILON);
  }
}
