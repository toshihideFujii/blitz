#![allow(dead_code)]

use crate::blitz_data::PrimitiveType;

// Structure describing permissible absolute and relative error bounds.
pub struct ErrorSpec {
  // Absolute error bound.
  abs: f64,
  // Relative error bound.
  rel: f64,
  // If relaxed_nans is true then any result is valid if we are expecting NaNs.
  // In effect, this allows the tested operation to produce incorrect results
  // for inputs outside its mathematical domain.
  relaxed_nans: bool,
  // If true, then we don't check for bitwise equality of NaNs.  All NaNs are
  // considered equivalent.
  all_nans_are_equivalent: bool,
  // If this is true, then we treat each +/-inf in the actual result as
  // equivalent to our choice of either +/-inf or the min/max floating-point
  // value.
  //
  // If the expected result is +/-inf, the actual result must still be +/-inf.
  //
  // In effect, this allows the tested operation to overflow, so long as it's
  // overflowing on "large" values.
  //
  // (We could have a symmetric more_infs_ok flag if necessary; right now it
  // appears not to be.)
  fewer_infs_ok: bool,
  // If the computation uses low precision floating point (e.g. FP8), this field
  // specifies the error bounds to be used. This allows us to have a per element
  // error bound measured in floats vs relying on the default relative/absolute
  // error bounds. We need this for FP8 since it's very sparse and we'd like to
  // avoid unnecessarily large error bounds. This overrides abserr/relerr when
  // specified.
  low_precision_fp_error_spec: LowPrecisionFPErrorSpec,
}

impl ErrorSpec {
  pub fn new(abs: f64, rel: f64, relaxed_nans: bool) -> Self {
    ErrorSpec {
      abs: abs,
      rel: rel,
      relaxed_nans: relaxed_nans,
      all_nans_are_equivalent: true,
      fewer_infs_ok: false,
      low_precision_fp_error_spec: LowPrecisionFPErrorSpec::new(),
    }
  }
}

struct  LowPrecisionFPErrorSpec {
  // Type of low precision floating point to use for error bound calculations.
  // We can't infer this type from the result because the lower precision
  // could have been used for intermediate calculations.
  type_: PrimitiveType,
  // Allowable distance in number of representable floats between the expected
  // and actual once they're converted to the PrimitiveType specified above.
  // Note:
  // - this is only valid if the expected value is outside the error bound.
  // - +/-0 are considered equivalent.
  within_n_values: i64,
}

impl LowPrecisionFPErrorSpec {
  pub fn new() -> Self {
    LowPrecisionFPErrorSpec { type_: PrimitiveType::Invalid, within_n_values: -1 }
  }
}