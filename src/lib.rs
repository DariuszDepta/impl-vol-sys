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
