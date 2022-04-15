extern crate cxx;

use libc::c_double;

#[cxx::bridge]
mod ffi {}

extern "C" {
  fn normalised_vega(x: c_double, s: c_double) -> c_double;
  fn implied_volatility_from_a_transformed_rational_guess(p: c_double, f: c_double, k: c_double, t: c_double, q: c_double) -> c_double;
}

pub fn iv_normalised_vega(x: f64, s: f64) -> f64 {
  unsafe { normalised_vega(x, s) }
}

pub fn iv_implied_volatility(price: f64, forward: f64, strike: f64, time: f64, quote: f64) -> f64 {
  unsafe {
    implied_volatility_from_a_transformed_rational_guess(
      price,
      forward,
      strike,
      time,
      quote,
    )
  }
}

#[cfg(test)]
mod tests {
  use crate::iv_normalised_vega;

  #[test]
  fn test_normalised_vega() {
    assert_eq!(0.000000004293512152292861_f64, iv_normalised_vega(10.0, 12.0));
  }
}
