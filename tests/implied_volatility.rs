use impl_vol_sys::*;

fn eq(expected: f64, actual: f64) {
  assert!(
    (expected - actual).abs() < f64::EPSILON,
    "expected: {}\n  actual: {}",
    expected,
    actual
  );
}

fn iv(
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

fn binary_flag(ch: char) -> f64 {
  match ch {
    'c' => 1.0,
    'p' => -1.0,
    _ => 1.0,
  }
}

#[test]
fn test_implied_volatility() {
  eq(
    0.056675076377579556,
    iv(1.0, 100.0, 100.0, 0.05, 0.01, 0.1123, 'c'),
  );
  eq(
    0.7354556444164313,
    iv(10.0, 100.0, 100.0, 0.05, 0.01, 0.1123, 'c'),
  );
}
