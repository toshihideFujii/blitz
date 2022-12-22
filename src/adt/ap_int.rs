#![allow(dead_code)]
#![allow(unused_variables)]

/*
This file implements a class to represent arbitrary precision
integral constant values and operations on them.
*/

use crate::support::math_extras::*;
use std::ops;

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

#[derive(Debug, PartialEq, Eq)]
pub struct APInt {
  val_: u64, // Used to store the <= 64 bits integer value.
  pval_: u64, // Used to store the > 64 bits integer value.
  bit_width_: u32 // The number of bits in this APInt.
}

impl APInt {
  // Default constructor that creates an APInt with a 1-bit zero value.
  pub fn new_zero() -> Self {
    APInt { val_: 0, pval_: 0, bit_width_: 1 }
  }

  // Create a new APInt of num_bits width, initialized as val.
  pub fn new_from_val(num_bits: u32, val: u64, is_signed: bool) -> Self {
    if num_bits <= APINT_BITS_PER_WORD {
      APInt { // TODO
        val_: val,
        pval_: 0,
        bit_width_: num_bits
      }
    } else { // TODO
      APInt {
        val_: val,
        pval_: 0,
        bit_width_: 1
      }
    }
  }

  // Determine if this APInt just has one word to store value.
  pub fn is_single_word(&self) -> bool {
    self.bit_width_ <= APINT_BITS_PER_WORD
  }

  // Determine sign of this APInt.
  pub fn is_negative(&self) -> bool {
    false
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
    if self.bit_width_ == 0 {
      return true;
    }
    if self.is_single_word() {
      return self.val_ == WORD_TYPE_MAX >> (APINT_BITS_PER_WORD - self.bit_width_);
    }
    self.count_trailing_ones_slow_case() == self.bit_width_
  }

  // Determine if this value is zero, i.e. all bits are clear.
  pub fn is_zero(&self) -> bool {
    if self.is_single_word() {
      return self.val_ == 0;
    }
    self.count_leading_zeros_slow_case() == self.bit_width_
  }

  // Determine if this is a value of 1.
  pub fn is_one(&self) -> bool {
    if self.is_single_word() {
      return self.val_ == 1;
    }
    self.count_leading_zeros_slow_case() == self.bit_width_ - 1
  }

  // Determine if this is the largest signed value.
  pub fn is_max_value(&self) -> bool {
    self.is_all_ones()
  }

  // Determine if this is the largest signed value.
  pub fn is_max_signed_value(&self) -> bool {
    if self.is_single_word() {
      return self.val_ == ((1 << self.bit_width_ - 1) - 1);
    }
    self.is_negative() == false &&
      self.count_trailing_ones_slow_case() == self.bit_width_ - 1
  }

  // Determine if this is the smallest unsigned value.
  pub fn is_min_value(&self) -> bool {
    self.is_zero()
  }

  // Determine ifthis is the smallest signed value.
  pub fn is_min_signed_value(&self) -> bool {
    if self.is_single_word() {
      return self.val_ == (1 << (self.bit_width_ - 1));
    }
    return self.is_negative() &&
      self.count_trailing_zeros_slow_case() == self.bit_width_ - 1;
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
      return is_power_of_2_64(self.val_);
    }
    return self.count_population_slow_case() == 1;
  }

  // Check if this negated value is a power of 2 greater than zero.
  pub fn is_negated_power_of_2(&self) -> bool {
    if self.is_non_negative() {
      return false;
    }
    let lo = self.count_leading_ones();
    let tz = self.count_trailing_zeros();
    (lo + tz) == self.bit_width_
  }

  // Check if the APInt's value is returned by get_sign_mask.
  pub fn is_sign_mask(&self) -> bool {
    self.is_min_signed_value()
  }

  // Convert APInt to a boolean value.
  pub fn get_bool_value(&self) -> bool {
    !self.is_zero()
  }

  // If this value is smaller than the specified limit, return it,
  // otherwise return the limit value.
  pub fn get_limited_value(&self, limit: u64) -> u64 {
    if self.ugt(limit) {
      return limit;
    } else {
      return self.get_zext_value();
    }
  }

  pub fn is_splat() {}

  // Return true if this is a non-empty sequence of ones starting
  // at the least significant bit with the remainder zero.
  pub fn is_mask(&self) -> bool {
    if self.is_single_word() {
      return is_mask_64(self.val_);
    }
    let ones = self.count_trailing_ones_slow_case();
    ones > 0 && ((ones + self.count_leading_zeros_slow_case()) == self.bit_width_)
  }

  // Return true if this value contains a non-empty sequence os ones 
  // with the remainder zero.
  pub fn is_shifted_mask(&self) -> bool {
    if self.is_single_word() {
      return is_shifted_mask_64(self.val_);
    }
    let ones = self.count_population_slow_case();
    let leadz = self.count_leading_zeros_slow_case();
    (ones + leadz + self.count_trailing_zeros()) == self.bit_width_
  }

  // Compute an APInt containing num_bits highbits from this.
  pub fn get_hi_bits(&self, num_bits: u32) -> Self {
    self.lshr(self.bit_width_ - num_bits)
  }

  pub fn get_lo_bits() {}

  // Determine if two APInts have the same value, after zero-extending
  // one of them to ensure that the bit-widths match.
  pub fn is_same_value(i1: &APInt, i2: &APInt) -> bool {
    if i1.get_bit_width() == i2.get_bit_width() {
      return i1 == i2;
    }
    if i1.get_bit_width() > i2.get_bit_width() {
      return i1 == &i2.zext(i1.get_bit_width());
    }
    &i1.zext(i2.get_bit_width()) == i2
  }

  pub fn hash_value() {}

  // This function returns an internal storage of the APInt.
  pub fn get_raw_data(&self) -> u64 {
    if self.is_single_word() {
      return self.val_;
    }
    self.pval_
  }

  // Arithmetic right-shift function.
  pub fn ashr(&self, shift_amt: u32) -> Self {
    let mut r = APInt { val_: self.val_, pval_: self.pval_, bit_width_: self.bit_width_ };
    r.ashr_in_place(shift_amt);
    r
  }

  // Arithmetic right-shift this APInt by shift_amt in place.
  pub fn ashr_in_place(&mut self, shift_amt: u32) {
    debug_assert!(shift_amt <= self.bit_width_, "Invalid shift amount");
    if self.is_single_word() {
      let s_ext_val = sign_extend_64(self.val_, self.bit_width_);
      if shift_amt == self.bit_width_ {
        self.val_ = (s_ext_val >> (APINT_BITS_PER_WORD - 1)) as u64;
      } else {
        self.val_ = (s_ext_val >> shift_amt) as u64;
      }
      self.clear_unused_bits();
      return;
    }
    self.ashr_slow_case(shift_amt)
  }

  // Logical right-shift this APInt by shift_amt.
  pub fn lshr(&self, shift_amt: u32) -> Self {
    let mut r = APInt { val_: self.val_, pval_: self.pval_, bit_width_: self.bit_width_ };
    r.lshr_in_place(shift_amt);
    r
  }

  // Logical right-shift this APInt by shift_amt in place.
  pub fn lshr_in_place(&mut self, shift_amt: u32) {
    debug_assert!(shift_amt <= self.bit_width_, "Invalid shift amount");
    if self.is_single_word() {
      if shift_amt == self.bit_width_ {
        self.val_ = 0
      } else {
        self.val_ >>= shift_amt;
      }
      return;
    }
    self.lshr_slow_case(shift_amt)
  }

  // Left-shift this APInt by shift_amt.
  pub fn shl(&self, shift_amt: u32) -> Self {
    let mut r = APInt { val_: self.val_, pval_: self.pval_, bit_width_: self.bit_width_ };
    r <<= shift_amt;
    r
  }

  // relative logical shift right
  pub fn relative_lshr(&self, relative_shift: i32) -> Self {
    if relative_shift > 0 {
      return self.lshr(relative_shift as u32);
    } else {
      return self.shl((-relative_shift) as u32);
    }
  }

  // relative logical shift left
  pub fn relative_lshl(&self, relative_shift: i32) -> Self {
    self.relative_lshr(-relative_shift)
  }

  // relative arithmetic shift right
  pub fn relative_ashr(&self, relative_shift: i32) -> Self {
    if relative_shift > 0 {
      return self.ashr(relative_shift as u32);
    } else {
      return self.shl((-relative_shift) as u32);
    }
  }

  // relative arithmetic shift left
  pub fn relative_ashl(&self, relative_shift: i32) -> Self {
    self.relative_ashr(-relative_shift)
  }

  /*
  pub fn rotl(&self, mut rotate_amt: u32) -> Self {
    if self.bit_width_ == 0 {
      return self;
    }
    rotate_amt %= self.bit_width_;
    if rotate_amt == 0 {
      return self;
    }
    self.shl(rotate_amt) | self.lshr(self.bit_width_ - rotate_amt)
  }
  */

  pub fn rotr() {}

  // Concatanate the bits from new_lsb onto the bottom of this.
  pub fn concat(&self, new_lsb: APInt) -> Self {
    let new_width = self.bit_width_ + new_lsb.bit_width_;
    if new_width <= APINT_BITS_PER_WORD {
      return APInt::new_from_val(new_width,
        (self.val_ << new_lsb.bit_width_) | new_lsb.val_, false);
    }
    self.concat_slow_case(new_lsb)
  }

  // TODO
  // Unsigned division operation.
  pub fn udiv(&self, rhs: u64) -> Self {
    debug_assert!(rhs != 0, "Divide by zero ?");

    // First, deal with easy case
    if self.is_single_word() {
      return APInt::new_from_val(self.bit_width_, self.val_ / rhs, false);
    }

    // Get some facts about the lhs words.
    let lhs_words = self.get_num_words(self.get_active_bits());

    // Deal with some degenerate cases
    if lhs_words == 0 { // 0 / x => 0
      return APInt::new_from_val(self.bit_width_, 0, false);
    }
    if rhs == 1 { // x / 1 => x
      return APInt::new_from_val(self.bit_width_, self.val_, self.is_sign_bit_set());
    }
    if self.val_ == rhs { // x / x => 1 
      return APInt::new_from_val(self.bit_width_, 1, false);
    }
    if lhs_words == 1 {
      return APInt::new_from_val(self.bit_width_, self.pval_ / rhs, false);
    }

    let quotient = APInt::new_from_val(self.bit_width_, 0, false);
    quotient
  }

  // TODO
  // Signed division function for APInt.
  pub fn sdiv(&self, rhs: i64) -> Self {
    if self.is_negative() {
      if rhs < 0 {
        //return APInt::new_from_val(self.bit_width_, -self.val_, false).udiv(-rhs as u64);
      }
      //return APInt::new_from_val(self.bit_width_, -self.val_, false).udiv(rhs as u64);
    }
    if rhs < 0 {
      return self.udiv(-rhs as u64);
    }
    self.udiv(rhs as u64)
  }

  pub fn urem() {}

  pub fn srem() {}

  pub fn udivrem() {}

  pub fn sdiv_rem() {}

  pub fn sadd_ov(&self, rhs: &APInt) -> (APInt, bool) {
    let res = APInt::new_from_val(self.bit_width_, self.val_ + rhs.val_, false);
    let overflow = self.is_non_negative() == rhs.is_non_negative() && res.is_non_negative() != self.is_non_negative();
    (res, overflow)
  }

  pub fn uadd_ov(&self, rhs: &APInt) -> (APInt, bool) {
    let res = APInt::new_from_val(self.bit_width_, self.val_ + rhs.val_, false);
    let overflow = res.ult(rhs.val_);
    (res, overflow)
  }

  pub fn ssub_ov(&self, rhs: &APInt) -> (APInt, bool) {
    let res = APInt::new_from_val(self.bit_width_, self.val_ - rhs.val_, false);
    let overflow = self.is_non_negative() != rhs.is_non_negative() && res.is_non_negative() != self.is_non_negative();
    (res, overflow)
  }

  pub fn usub_ov(&self, rhs: &APInt) -> (APInt, bool) {
    let res = APInt::new_from_val(self.bit_width_, self.val_ - rhs.val_, false);
    let overflow = res.ugt(self.val_);
    (res, overflow)
  }

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

  // Unsigned less than comparison
  pub fn ult(&self, rhs: u64) -> bool {
    (self.is_single_word() || self.get_active_bits() <= 64) && self.get_zext_value() < rhs
  }

  pub fn slt() {}

  pub fn ule() {}

  pub fn sle() {}

  // Unsigned greater than comparison.
  pub fn ugt(&self, rhs: u64) -> bool {
    false
  }

  pub fn sgt() {}

  pub fn uge() {}

  pub fn sge() {}

  pub fn intersects() {}

  pub fn is_subset_of() {}

  pub fn trunc() {}

  pub fn trunc_usat() {}

  pub fn trunc_ssat() {}

  pub fn sext() {}

  pub fn zext(&self, width: u32) -> APInt {
    APInt::new_zero()
  }

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

  // Return the number of bits in the APInt.
  pub fn get_bit_width(&self) -> u32 {
    self.bit_width_
  }

  // Get the number of words.
  pub fn get_num_words(&self, bit_width: u32) -> u32 {
    (self.bit_width_ + APINT_BITS_PER_WORD - 1) / APINT_BITS_PER_WORD
  }

  // Compute the number of active bits in the value.
  pub fn get_active_bits(&self) -> u32 {
    self.bit_width_ - self.count_leading_zeros()
  }

  pub fn get_active_words() {}

  // Get the minimum bit size for this signed APInt.
  pub fn get_significant_bits(&self) -> u32 {
    self.bit_width_ - self.get_num_sign_bits() + 1
  }

  // Get zero extended value.
  pub fn get_zext_value(&self) -> u64 {
    if self.is_single_word() {
      return self.val_;
    }
    if self.get_active_bits() > 64 {
      panic!("Too many bits for u64.");
    }
    self.pval_ // TODO
  }

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

  fn clear_unused_bits(&self) {}

  fn get_word() {}

  fn reallocate() {}

  fn from_string() {}

  fn divide() {}

  fn init_slow_case() {}

  fn init_from_array() {}

  fn shl_slow_case() {}

  fn lshr_slow_case(&self, _shift_amt: u32) {}

  fn ashr_slow_case(&self, _shift_amt: u32) {}

  fn assign_slow_case() {}

  fn equal_slow_case() {}

  // out-of-line slow case for count_leading_zeros.
  fn count_leading_zeros_slow_case(&self) -> u32 {
    0
  }

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

  // out-of-line slow case for concat.
  fn concat_slow_case(&self, new_lsb: APInt) -> Self {
    APInt::new_zero() // TODO
  }

  fn and_assign_slow_case() {}

  fn or_assing_slow_case() {}

  fn xor_assign_slow_case() {}

  fn compare() {}

  fn compare_signed() {}

  // Get the '0' value for the specified bit-width.
pub fn get_zero(num_bits: u32) -> APInt {
  APInt::new_from_val(num_bits, 0, false)
}

// Return an APInt zero bits wide.
pub fn get_zero_width() -> APInt {
  APInt::get_zero(0)
}

// Gets maximum unsigned value of APInt for specific bit width.
pub fn get_max_value(num_bits: u32) -> APInt {
  APInt::get_all_ones(num_bits)
}

pub fn get_signed_max_value() {}

// Gets minimum unsigned value of APInt for specific bit width.
pub fn get_min_value(num_bits: u32) -> APInt {
  APInt::new_from_val(num_bits, 0, false)
}

pub fn get_signed_min_value() {}

pub fn get_sign_mask() {}

// Return an APInt of a specified width with all bits set.
pub fn get_all_ones(num_bits: u32) -> APInt {
  APInt::new_from_val(num_bits, WORD_TYPE_MAX, true)
}

pub fn get_all_ones_value() {}

pub fn get_one_bit_set() {}

pub fn get_bits_set() {}

pub fn get_bits_set_with_wrap() {}

pub fn get_bits_set_from() {}

pub fn get_high_bits_set() {}

pub fn get_low_bits_set() {}

pub fn get_splat() {}

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
}

impl ops::BitAndAssign<APInt> for APInt {
  fn bitand_assign(&mut self, rhs: APInt) {
    if self.is_single_word() {
      self.val_ &= rhs.val_;
    } else {
      // and_assign_slow_case()
    }
  }
}

impl ops::BitAndAssign<u64> for APInt {
  fn bitand_assign(&mut self, rhs: u64) {
    if self.is_single_word() {
      self.val_ &= rhs;
    } else {
      // self.pval_ = rhs
    }
  }
}

impl ops::BitOrAssign<APInt> for APInt {
  fn bitor_assign(&mut self, rhs: APInt) {
    if self.is_single_word() {
      self.val_ |= rhs.val_;
    } else {
      // or_assign_slow_case()
    }
  }
}

impl ops::BitOrAssign<u64> for APInt {
  fn bitor_assign(&mut self, rhs: u64) {
    if self.is_single_word() {
      self.val_ |= rhs;
    } else {
      // self.pval_ |= rhs
    }
  }
}

impl ops::BitXorAssign<APInt> for APInt {
  fn bitxor_assign(&mut self, rhs: APInt) {
    if self.is_single_word() {
      self.val_ ^= rhs.val_;
    } else {
      // xor_assign_slow_case()
    }
  }
}

impl ops::BitXorAssign<u64> for APInt {
  fn bitxor_assign(&mut self, rhs: u64) {
    if self.is_single_word() {
      self.val_ ^= rhs;
    } else {
      // TODO
    }
  }
}

impl ops::MulAssign<APInt> for APInt {
  fn mul_assign(&mut self, rhs: APInt) {
    if self.is_single_word() {
      self.val_ *= rhs.val_;
    } else {
      // TODO
    }
  }
}

impl ops::MulAssign<u64> for APInt {
  fn mul_assign(&mut self, rhs: u64) {
    if self.is_single_word() {
      self.val_ *= rhs;
    } else {
      // TODO
    }
  }
}

impl ops::AddAssign<APInt> for APInt {
  fn add_assign(&mut self, rhs: APInt) {
    if self.is_single_word() && rhs.is_single_word() {
      self.val_ = rhs.val_;
      self.bit_width_ = rhs.bit_width_;
    }
    // TODO
    // assign_slow_case()
  }
}

impl ops::AddAssign<u64> for APInt {
  fn add_assign(&mut self, rhs: u64) {
    if self.is_single_word() {
      self.val_ = rhs;
      self.clear_unused_bits();
      return;
    }
    // TODO
    // self.pval_ = rhs;
  }
}

impl ops::SubAssign<APInt> for APInt {
  fn sub_assign(&mut self, rhs: APInt) {
    if self.is_single_word() {
      self.val_ -= rhs.val_;
    } else {
      // TODO
    }
  }
}

impl ops::SubAssign<u64> for APInt {
  fn sub_assign(&mut self, rhs: u64) {
    if self.is_single_word() {
      self.val_ -= rhs;
    } else {
      // TODO
    }
  }
}

impl ops::ShlAssign<APInt> for APInt {
  fn shl_assign(&mut self, rhs: APInt) {
    // TODO
  }
}

impl ops::ShlAssign<u32> for APInt {
  fn shl_assign(&mut self, rhs: u32) {
    // TODO
  }
}

impl ops::Mul<APInt> for APInt {
  type Output = APInt;
  fn mul(self, rhs: APInt) -> Self::Output {
    if self.is_single_word() {
      APInt::new_from_val(self.bit_width_, self.val_*rhs.val_, false)
    } else {
      // TODO
      APInt { val_: self.val_ * rhs.val_, pval_: self.pval_, bit_width_: self.bit_width_ }
    }
  }
}

impl ops::Shl<APInt> for APInt {
  type Output = APInt;
  fn shl(self, rhs: APInt) -> Self::Output {
    // TODO
    APInt::new_zero()
  }
}

impl ops::Shl<u32> for APInt {
  type Output = APInt;
  fn shl(self, rhs: u32) -> Self::Output {
    // TODO
    APInt::new_zero()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_value_init() {
    let zero = APInt::new_zero();
    assert_eq!(zero.val_, 0);
  }

  #[test]
  fn test_shift_left_by_zero() {
    //let one = APInt::get_zero(65) + 1;
    //let shl = one
  }
}