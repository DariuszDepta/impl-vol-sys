use impl_vol_sys::iv_normalised_vega;

#[test]
fn test_normalised_vega() {
  assert_eq!(0.000000004293512152292861_f64, iv_normalised_vega(10.0, 12.0));
}