#![allow(dead_code)]
#![allow(unused_variables)]

/*
This file implements a class to represent arbitrary precision
integral constant values and operations on them.
*/

// Byte size of a word.
const APINT_WORD_SIZE: u32 = 8;

// Bits in a word.
const APINT_BITS_PER_WORD: u32 = APINT_WORD_SIZE * 8; // TODO char_bit?

const WORD_TYPE_MAX: u64 = std::u64::MAX;

enum Rounding {
  Down,
  TowardZero,
  Up
}

pub struct APInt {
  val_: u64, // Used to store the <= 64 bits integer value.
  pval_: u64, // Used to store the > 64 bits integer value.
  bit_width: u32 // The number of bits in this APInt.
}

impl APInt {
  // Create a new APInt of num_bits width, initialized as val.
  pub fn new(num_bits: u32, val: u64, is_signed: bool) -> Self {
    if num_bits <= APINT_BITS_PER_WORD {
      APInt { // TODO
        val_: val,
        pval_: 0,
        bit_width: num_bits
      }
    } else { // TODO
      APInt {
        val_: val,
        pval_: 0,
        bit_width: 1
      }
    }
  }

  // Determine if this APInt just has one word to store value.
  pub fn is_single_word(&self) -> bool {
    self.bit_width <= APINT_BITS_PER_WORD
  }

  // Determine sign of this APInt.
  pub fn is_negative(&self) -> bool {
    false // TODO
  }

  // Determine if this APInt value is non-negative (>=0).
  pub fn is_non_negative(&self) -> bool {
    !self.is_negative()
  }

  // Determine if sign bit of this APInt is set.
  pub fn is_sign_bit_set(&self) -> bool {
    false // TODO
  }

  // Determine if sign bit of this APInt is clear.
  pub fn is_sign_bit_clear(&self) -> bool {
    !self.is_sign_bit_set()
  }

  // Determine if this APInt value is positive.
  pub fn is_strictly_positive(&self) -> bool {
    self.is_non_negative() && !self.is_zero()
  }

  // Determine if this APInt value is non-positive (<= 0).
  pub fn is_non_positive(&self) -> bool {
    !self.is_strictly_positive()
  }

  // Determine if all bits are set. This is true for zero-width values.
  pub fn is_all_ones(&self) -> bool {
    false // TODO
  }

  // Determine if this value is zero, i.e. all bits are clear.
  pub fn is_zero(&self) -> bool {
    false // TODO
  }

  // Determine if this is a value of 1.
  pub fn is_one(&self) -> bool {
    false // TODO
  }

  // Determine if this is the largest signed value.
  pub fn is_max_value(&self) -> bool {
    self.is_all_ones()
  }

  pub fn is_max_signed_value() {}

  // Determine if this is the smallest unsigned value.
  pub fn is_min_value(&self) -> bool {
    self.is_zero()
  }

  // Determine ifthis is the smallest signed value.
  pub fn is_min_signed_value(&self) -> bool {
    if self.is_single_word() {
      return false // TODO
    }
    return self.is_negative() &&
      self.count_trailing_zeros_slow_case() == self.bit_width - 1;
  }

  // Check if this APInt has an N-bits unsigned integer value.
  pub fn is_int_n(&self, n: u32) -> bool {
    self.get_active_bits() <= n
  }

  // Check if this APInt has an N-bits signed integer value.
  pub fn is_signed_int_n(&self, n: u32) -> bool {
    self.get_significant_bits() <= n
  }

  // Check if this APInt's negated value is a power of two greater
  // than zero.
  pub fn is_power_of_2(&self) -> bool {
    if self.is_single_word() {
      return false; // TODO
    }
    return self.count_population_slow_case() == 1;
  }

  pub fn is_negated_power_of_2() {}

  // Check if the APInt's value is returned by get_sign_mask.
  pub fn is_sign_mask(&self) -> bool {
    self.is_min_signed_value()
  }

  // Convert APInt to a boolean value.
  pub fn get_bool_value(&self) -> bool {
    !self.is_zero()
  }

  pub fn get_limited_value() {}

  pub fn is_splat() {}

  pub fn is_mask() {}

  pub fn is_shifted_mask() {}

  pub fn get_hi_bits() {}

  pub fn get_lo_bits() {}

  pub fn hash_value() {}

  pub fn get_raw_data() {}

  pub fn ashr() {}

  pub fn ashr_in_place() {}

  pub fn lshr() {}

  pub fn lshr_in_place() {}

  pub fn shl() {}

  pub fn rotl() {}

  pub fn rotr() {}

  pub fn concat() {}

  pub fn udiv() {}

  pub fn sdiv() {}

  pub fn urem() {}

  pub fn srem() {}

  pub fn udivrem() {}

  pub fn sdiv_rem() {}

  pub fn sadd_ov() {}

  pub fn uadd_ov() {}

  pub fn ssub_ov() {}

  pub fn usub_ov() {}

  pub fn sdiv_ov() {}

  pub fn smul_ov() {}

  pub fn umul_ov() {}

  pub fn sshl_ov() {}

  pub fn ushl_ov() {}

  pub fn sadd_sat() {}

  pub fn uadd_sat() {}

  pub fn ssub_sat() {}

  pub fn usub_sat() {}

  pub fn smul_sat() {}

  pub fn umul_sat() {}

  pub fn sshl_sat() {}

  pub fn ushl_sat() {}

  pub fn ne() {}

  pub fn ult() {}

  pub fn slt() {}

  pub fn ule() {}

  pub fn sle() {}

  pub fn ugt() {}

  pub fn sgt() {}

  pub fn uge() {}

  pub fn sge() {}

  pub fn intersects() {}

  pub fn is_subset_of() {}

  pub fn trunc() {}

  pub fn trunc_usat() {}

  pub fn trunc_ssat() {}

  pub fn sext() {}

  pub fn zext() {}

  pub fn sext_or_trunc() {}

  pub fn zext_or_trunc() {}

  pub fn set_all_bits() {}

  pub fn set_bit() {}

  pub fn set_sign_bit() {}

  pub fn set_bit_val() {}

  pub fn set_bits_with_wrap() {}

  pub fn set_bits() {}

  pub fn set_bits_from() {}

  pub fn set_low_bits() {}

  pub fn set_high_bits() {}

  pub fn clear_all_bits() {}

  pub fn clear_bit() {}

  pub fn clear_low_bits() {}

  pub fn clear_sign_bit() {}

  pub fn flip_all_bits() {}

  pub fn flip_bit() {}

  pub fn negate() {}

  pub fn insert_bits() {}

  pub fn extract_bits() {}

  pub fn extract_bits_as_zext_value() {}

  pub fn get_bit_width() {}

  pub fn get_num_words() {}

  // Compute the number of active bits in the value.
  pub fn get_active_bits(&self) -> u32 {
    self.bit_width - self.count_leading_zeros()
  }

  pub fn get_active_words() {}

  // Get the minimum bit size for this signed APInt.
  pub fn get_significant_bits(&self) -> u32 {
    self.bit_width - self.get_num_sign_bits() + 1
  }

  pub fn get_zext_value() {}

  pub fn get_sext_value() {}

  // It counts the number of zeros from the significant bit to
  // the first one bit. 
  pub fn count_leading_zeros(&self) -> u32 {
    if self.is_single_word() {
      return 0; // TODO
    }
    return self.count_trailing_zeros_slow_case();
  }

  // Count the number of leading one bits.
  pub fn count_leading_ones(&self) -> u32 {
    if self.is_single_word() {
      return 0; // TODO
    }
    return self.count_trailing_ones_slow_case();
  }

  // Computes the number of leading bits of this APInt that are equal
  // to its sign bits.
  pub fn get_num_sign_bits(&self) -> u32 {
    if self.is_negative() {
      return self.count_leading_ones();
    } else {
      return self.count_leading_zeros();
    }
  }

  // Count the number of trailing zero bits.
  pub fn count_trailing_zeros(&self) -> u32 {
    if self.is_single_word() {
      return 0; // TODO
    }
    return self.count_trailing_zeros_slow_case();
  }

  // Count the number of trailing one bits.
  pub fn count_trailing_ones(&self) -> u32 {
    if self.is_single_word() {
      return 0; // TODO
    }
    return self.count_trailing_ones_slow_case();
  }

  pub fn count_population() {}

  pub fn print() {}

  pub fn to_string() {}

  pub fn to_string_unsined() {}

  pub fn to_string_signed() {}

  pub fn byte_swap() {}

  pub fn reverse_bits() {}

  pub fn round_to_double() {}

  pub fn signed_round_to_double() {}

  pub fn bits_to_double() {}

  pub fn bits_to_float() {}

  pub fn double_to_bits() {}

  pub fn float_to_bits() {}

  pub fn log_base_2() {}

  pub fn ceil_log_base_2() {}

  pub fn nearest_log_base_2() {}

  pub fn exact_log_base_2() {}

  pub fn sqrt() {}

  pub fn abs() {}

  pub fn multiplicative_inverse() {}

  pub fn profile() {}

  pub fn dump() {}

  pub fn needs_cleanup() {}

  fn clear_unused_bits() {}

  fn get_word() {}

  fn reallocate() {}

  fn from_string() {}

  fn divide() {}

  fn init_slow_case() {}

  fn init_from_array() {}

  fn shl_slow_case() {}

  fn lshr_slow_case() {}

  fn ashr_slow_case() {}

  fn assign_slow_case() {}

  fn equal_slow_case() {}

  fn count_leading_zeros_slow_case() {}

  fn count_leading_ones_slow_case() {}

  // out-of-line slow case for count_trailing_zeros.
  fn count_trailing_zeros_slow_case(&self) -> u32 {
    0 // TODO
  }

  // out-of-line slow case for count_trailing_ones.
  fn count_trailing_ones_slow_case(&self) -> u32 {
    0 // TODO
  }

  fn count_population_slow_case(&self) -> u32 {
    0 // TODO
  }

  fn intersects_slow_case() {}

  fn is_subset_of_slow_case() {}

  fn set_bits_slow_case() {}

  fn flip_all_bits_slow_case() {}

  fn concat_slow_case() {}

  fn and_assign_slow_case() {}

  fn or_assing_slow_case() {}

  fn xor_assign_slow_case() {}

  fn compare() {}

  fn compare_signed() {}
}

// Get the '0' value for the specified bit-width.
pub fn get_zero(num_bits: u32) -> APInt {
  APInt::new(num_bits, 0, false)
}

// Return an APInt zero bits wide.
pub fn get_zero_width() -> APInt {
  get_zero(0)
}

// Gets maximum unsigned value of APInt for specific bit width.
pub fn get_max_value(num_bits: u32) -> APInt {
  get_all_ones(num_bits)
}

pub fn get_signed_max_value() {}

// Gets minimum unsigned value of APInt for specific bit width.
pub fn get_min_value(num_bits: u32) -> APInt {
  APInt::new(num_bits, 0, false)
}

pub fn get_signed_min_value() {}

pub fn get_sign_mask() {}

// Return an APInt of a specified width with all bits set.
pub fn get_all_ones(num_bits: u32) -> APInt {
  APInt::new(num_bits, WORD_TYPE_MAX, true)
}

pub fn get_all_ones_value() {}

pub fn get_one_bit_set() {}

pub fn get_bits_set() {}

pub fn get_bits_set_with_wrap() {}

pub fn get_bits_set_from() {}

pub fn get_high_bits_set() {}

pub fn get_low_bits_set() {}

pub fn get_splat() {}

pub fn is_same_value() {}

pub fn get_bits_needed() {}

pub fn get_sufficient_bits_needed() {}

pub fn tc_set() {}

pub fn tc_assign() {}

pub fn tc_is_zero() {}

pub fn tc_extract_bit() {}

pub fn tc_extract() {}

pub fn ts_set_bit() {}

pub fn tc_clear_bit() {}

pub fn tc_lsb() {}

pub fn tc_msb() {}

pub fn tc_negate() {}

pub fn tc_add() {}

pub fn tc_add_part() {}

pub fn tc_subtract() {}

pub fn tc_subtract_part() {}

pub fn tc_multiply_part() {}

pub fn tc_multiply() {}

pub fn tc_full_multiply() {}

pub fn tc_divide() {}

pub fn tc_shift_left() {}

pub fn tc_shift_right() {}

pub fn tc_compare() {}

pub fn tc_increment() {}

pub fn tc_decrement() {}

fn which_word() {}

fn which_bit() {}

fn mask_bit() {}