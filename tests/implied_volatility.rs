use impl_vol_sys::{
  binary_flag, iv_implied_volatility_from_a_transformed_rational_guess,
};

#[test]
fn test_implied_volatility() {
  // option price
  let option_price: f64 = 1.0;

  // underlying price
  let underlying_price: f64 = 100.0;

  // strike price
  let strike_price: f64 = 100.0;

  // interest rate
  let r: f64 = 0.05;

  // dividend rate
  let q: f64 = 0.01;

  // time to expiration in years
  let t: f64 = 0.1123;

  // calculate deflator
  let opt_deflator = (-r * t).exp();

  // calculate forward price
  let forward_deflator = ((r - q) * t).exp();
  let forward_price = underlying_price * forward_deflator;
  println!("      forward price: {}", forward_price);

  let iv = iv_implied_volatility_from_a_transformed_rational_guess(
    option_price / opt_deflator,
    forward_price,
    strike_price,
    t,
    binary_flag('c'),
  );
  println!("implied volatility (CALL): {}", iv);

  // let iv = iv_implied_volatility_from_a_transformed_rational_guess(
  //   adjusted_price,
  //   forward_price,
  //   strike_price,
  //   t,
  //   binary_flag('p'),
  // );
  // println!(" implied volatility (PUT): {}", iv);
}

//0.056242807414176824
