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
}

fn iv_implied_volatility_from_a_transformed_rational_guess(
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

pub fn iv(
  price: f64,
  underlying: f64,
  strike: f64,
  r: f64,
  q: f64,
  t: f64,
  flag: char,
) -> f64 {
  iv_implied_volatility_from_a_transformed_rational_guess(
    price / (-r * t).exp(),
    underlying * ((r - q) * t).exp(),
    strike,
    t,
    binary_flag(flag),
  )
}

/// Returns flag value for different kind of options.
fn binary_flag(ch: char) -> f64 {
  match ch {
    'c' => 1.0,
    'p' => -1.0,
    _ => 1.0,
  }
}
