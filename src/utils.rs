/*
*  utils.rs is intended to remove math calculations and other details from
*  main.rs.  This is particularly important in no-std programming. For example,
*  the pow() operation is normally implemented inline with standard-lib integer
*  and float methods, but in no-std it takes a number of lines that can clutter
*  up a block of code.
*/

// Round a float to 0 or more decimals (base 10 only). 
pub fn round_to_decimal(humidity: f32, rounding: u32) -> f32 {
  // no-std approach to rounding; push digits left past the decimal, subtract 
  // everything to the right of the decimal, which is the remainder in the
  // calculations below, then push the number back rightward.

  // Shift left by 'rounding' digits
  let temp_num = humidity * pow(10.0, rounding);

  // Subtract everything left to the right of the decimal
  let rem = (humidity * pow(10.0, rounding)) % 1.0;
  let mut result = temp_num - rem;

  // round up one if needed
  if rem >= 0.5 { result += 1.0 };

  // return the result after shifting back to the lefty by 'rounding' digits
  result / pow(10.0, rounding)
}

// Generalized exponent function raising any base to any exponent >= 0
// To make it more generalized, this could be improved by:
//    - allowing a negative exponent
//    - error handling, such as testing to ensure there is no overflow or
//      incaccuracy from wraparound behavior with too high an exponent
fn pow(base: f32, exp: u32) -> f32 {
  // if exp is 1, result will drop past the if..else to return
  let mut result: f32 = base;

  // base^0 = 1
  if exp == 0 {
    result = 1.0;
  } else {
    for _ in 1..exp {
      result *= base;
    }
  }

  result
}