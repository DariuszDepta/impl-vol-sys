use impl_vol_sys::iv;

fn eq(expected: f64, actual: f64) {
  assert!(
    (expected - actual).abs() < f64::EPSILON,
    "expected: {}\n  actual: {}",
    expected,
    actual
  );
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
