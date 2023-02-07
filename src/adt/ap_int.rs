#![allow(dead_code)]
#![allow(unused_variables)]

/*
This file implements a class to represent arbitrary precision
integral constant values and operations on them.
*/

use crate::support::{math_extras::*};
use std::{ops::*};

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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct APInt {
  val_: i64, // Used to store the <= 64 bits integer value.
  pval_: i64, // Used to store the > 64 bits integer value.
  bit_width_: u32 // The number of bits in this APInt.
}

impl APInt {
  // Default constructor that creates an APInt with a 1-bit zero value.
  pub fn new_zero() -> Self {
    APInt { val_: 0, pval_: 0, bit_width_: 1 }
  }

  // Create a new APInt of num_bits width, initialized as val.
  // If is_signed is true then val is treated as if it were a signed value
  // and the appropriate sign extension to the bit width will be done.
  pub fn new(num_bits: u32, val: i64, is_signed: bool) -> Self {
    let mut instance = APInt::new_zero();
    instance.bit_width_ = num_bits;
    if instance.is_single_word() {
      instance.val_ = val;
      instance.clear_unused_bits();
    } else {
      instance.init_slow_case(val, is_signed);
    }
    instance
  }

  // Determine if this APInt just has one word to store value.
  // Returns true if the number of bits <= 64, false otherwise.
  pub fn is_single_word(&self) -> bool {
    self.bit_width_ <= APINT_BITS_PER_WORD
  }

  // Determine sign of this APInt.
  // This tests the high bit of APInt to determine if it is set.
  // Returns true if this APint is negative, false otherwise.
  pub fn is_negative(&self) -> bool {
    self.at(self.bit_width_ - 1)
  }

  // Determine if this APInt value is non-negative (>=0).
  pub fn is_non_negative(&self) -> bool {
    !self.is_negative()
  }

  // Determine if sign bit of this APInt is set.
  // This tests the high bit of this APInt to determine if it is set.
  // Return true if this APInt has its sign bit set, false otherwise.
  pub fn is_sign_bit_set(&self) -> bool {
    self.at(self.bit_width_ - 1)
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
      return self.val_ as u64 == WORD_TYPE_MAX >> (APINT_BITS_PER_WORD - self.bit_width_);
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

  // Determine if this is the smallest signed value.
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
      return is_power_of_2_64(self.val_ as u64);
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
      return self.get_z_ext_value();
    }
  }

  pub fn is_splat() {}

  // Return true if this is a non-empty sequence of ones starting
  // at the least significant bit with the remainder zero.
  pub fn is_mask(&self) -> bool {
    if self.is_single_word() {
      return is_mask_64(self.val_ as u64);
    }
    let ones = self.count_trailing_ones_slow_case();
    ones > 0 && ((ones + self.count_leading_zeros_slow_case()) == self.bit_width_)
  }

  // Return true if this value contains a non-empty sequence os ones 
  // with the remainder zero.
  pub fn is_shifted_mask(&self) -> bool {
    if self.is_single_word() {
      return is_shifted_mask_64(self.val_ as u64);
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
  pub fn get_raw_data(&self) -> i64 {
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
      let s_ext_val = sign_extend_64(self.val_ as u64, self.bit_width_);
      if shift_amt == self.bit_width_ {
        self.val_ = s_ext_val >> (APINT_BITS_PER_WORD - 1);
      } else {
        self.val_ = s_ext_val >> shift_amt;
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
      return APInt::new(new_width,
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
      return APInt::new(self.bit_width_, self.val_ / rhs as i64, false);
    }

    // Get some facts about the lhs words.
    let lhs_words = self.get_num_words(self.get_active_bits());

    // Deal with some degenerate cases
    if lhs_words == 0 { // 0 / x => 0
      return APInt::new(self.bit_width_, 0, false);
    }
    if rhs == 1 { // x / 1 => x
      return APInt::new(self.bit_width_, self.val_, self.is_sign_bit_set());
    }
    if self.val_ as u64 == rhs { // x / x => 1 
      return APInt::new(self.bit_width_, 1, false);
    }
    if lhs_words == 1 {
      return APInt::new(self.bit_width_, self.pval_ / rhs as i64, false);
    }

    let quotient = APInt::new(self.bit_width_, 0, false);
    quotient
  }

  // TODO
  // Signed division function for APInt.
  pub fn sdiv(&self, rhs: i64) -> Self {
    if self.is_negative() {
      if rhs < 0 {
        //return APInt::new(self.bit_width_, -self.val_, false).udiv(-rhs as u64);
      }
      //return APInt::new(self.bit_width_, -self.val_, false).udiv(rhs as u64);
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
    let res = APInt::new(self.bit_width_, self.val_ + rhs.val_, false);
    let overflow = self.is_non_negative() == rhs.is_non_negative() && res.is_non_negative() != self.is_non_negative();
    (res, overflow)
  }

  pub fn uadd_ov(&self, rhs: &APInt) -> (APInt, bool) {
    let res = APInt::new(self.bit_width_, self.val_ + rhs.val_, false);
    let overflow = res.ult(rhs.val_ as u64);
    (res, overflow)
  }

  pub fn ssub_ov(&self, rhs: &APInt) -> (APInt, bool) {
    let res = APInt::new(self.bit_width_, self.val_ - rhs.val_, false);
    let overflow = self.is_non_negative() != rhs.is_non_negative() && res.is_non_negative() != self.is_non_negative();
    (res, overflow)
  }

  pub fn usub_ov(&self, rhs: &APInt) -> (APInt, bool) {
    let res = APInt::new(self.bit_width_, self.val_ - rhs.val_, false);
    let overflow = res.ugt(self.val_ as u64);
    (res, overflow)
  }

  pub fn sdiv_ov(&self, rhs: &APInt) -> (APInt, bool) {
    let overflow = self.is_min_signed_value() && rhs.is_all_ones();
    (self.sdiv(rhs.val_ as i64), overflow)
  }

  pub fn smul_ov(&self, rhs: &APInt) -> (APInt, bool) {
    let val = self.val_ * rhs.val_;
    let res = APInt::new(self.bit_width_, val, false);
    let mut overflow = false;
    if rhs.val_ != 0{
      overflow = res.sdiv(rhs.val_ as i64) != *self ||
        (self.is_min_signed_value() && rhs.is_all_ones())
    }
    (res, overflow)
  }

  pub fn umul_ov(&self, rhs: &APInt) -> (APInt, bool) {
    if self.count_leading_zeros() + rhs.count_leading_zeros() + 2 <= self.bit_width_ {
      let val = self.val_ * rhs.val_;
      let res = APInt::new(self.bit_width_, val, false);
      return (res, true);
    }
    let rhs2 = rhs.clone();
    let val = self.lshr(1); // TODO * (*rhs2);
    let mut res = APInt::new(self.bit_width_, val.val_, false);
    let mut overflow = res.is_negative();
    res.val_ <<= 1;
    if self.val_ != 0 { // TODO
      res.val_ += rhs.val_;
      if res.ult(rhs.val_ as u64) {
        overflow = true;
      }
    }
    (res, overflow)
  }

  pub fn sshl_ov(&self, shamt: &APInt) -> (APInt, bool) {
    let mut overflow = shamt.uge(self.get_bit_width() as u64);
    if overflow {
      let res = APInt::new(self.bit_width_, 0, false);
      return (res, overflow);
    }
    if self.is_non_negative() {
      overflow = shamt.uge(self.count_leading_zeros() as u64);
    } else {
      overflow = shamt.uge(self.count_leading_ones() as u64);
    }
    let val = self.val_ << shamt.val_;
    let res = APInt::new(self.bit_width_, val, false);
    (res, overflow)
  }

  pub fn ushl_ov(&self, shamt: &APInt) -> (APInt, bool) {
    let mut overflow = shamt.uge(self.get_bit_width() as u64);
    if overflow {
      let res = APInt::new(self.bit_width_, 0, false);
      return (res, overflow);
    }
    overflow = shamt.ugt(self.count_leading_zeros() as u64);
    let val = self.val_ << shamt.val_;
    let res = APInt::new(self.bit_width_, val, false);
    (res, overflow)
  }

  pub fn sadd_sat(&self, _rhs: &APInt) {}

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
    (self.is_single_word() || self.get_active_bits() <= 64) && self.get_z_ext_value() < rhs
  }

  // Signed less than comparison
  pub fn slt(&self, rhs: i64) -> bool {
    if !self.is_single_word() && self.get_significant_bits() > 64 {
      self.is_negative()
    } else {
      self.get_s_ext_value() < rhs
    }
  }

  // Unsigned less or equal comparison
  pub fn ule(&self, rhs: u64) -> bool {
    !self.ugt(rhs)
  }

  // Signed less or equal comparison
  pub fn sle(&self, rhs: u64) -> bool {
    !self.sgt(rhs as i64)
  }

  // Unsigned greater than comparison.
  pub fn ugt(&self, rhs: u64) -> bool {
    (!self.is_single_word() && self.get_active_bits() > 64) || self.get_z_ext_value() > rhs
  }

  // Signed greater than comparison
  pub fn sgt(&self, rhs: i64) -> bool {
    if !self.is_single_word() && self.get_significant_bits() > 64 {
      !self.is_negative()
    } else {
      self.get_s_ext_value() > rhs
    }
  }

  // Unsigned greater or equal comparison
  pub fn uge(&self, rhs: u64) -> bool {
    !self.ult(rhs)
  }

  // Signed greater or equal comparison
  pub fn sge(&self, rhs: i64) -> bool {
    !self.slt(rhs)
  }

  // This operation tests if there are any pairs of corresponding
  // bits between this APInt and RHS that are both set.
  pub fn intersects(&self, rhs: &APInt) -> bool {
    debug_assert!(self.bit_width_ == rhs.bit_width_, "Bit widths must be the same");
    if self.is_single_word() {
      return (self.val_ & rhs.val_) != 0;
    }
    self.intersects_slow_case(rhs)
  }

  // This operation checks that all bits sest in this APInt are also set in RHS.
  pub fn is_subset_of(&self, rhs: &APInt) -> bool {
    debug_assert!(self.bit_width_ == rhs.bit_width_, "Bit widths must be the same");
    if self.is_single_word() {
      return (self.val_ & !rhs.val_) == 0;
    }
    self.is_subset_of_slow_case(rhs)
  }

  pub fn trunc() {}

  pub fn trunc_usat() {}

  pub fn trunc_ssat() {}

  pub fn sext() {}

  // Zero extend to a new width.
  pub fn zext(&self, width: u32) -> APInt {
    debug_assert!(width >= self.bit_width_, "Invalid APInt zero extend request");

    if width <= APINT_BITS_PER_WORD || width == self.bit_width_ {
      return APInt::new(width, self.val_, false);
    }

    // TODO memcpy
    APInt::new_zero()
  }

  pub fn sext_or_trunc() {}

  pub fn zext_or_trunc() {}

  // Set every bit to 1.
  pub fn set_all_bits(&mut self) {
    if self.is_single_word() {
      self.val_ = WORD_TYPE_MAX as i64;
    } // TODO else case
    self.clear_unused_bits();
  }

  // Set the given bit to 1 whose position is given as "bit_position".
  pub fn set_bit(&mut self, bit_position: u32) {
    debug_assert!(bit_position < self.bit_width_, "bit_position out of range");
    let mask = APInt::mask_bit(bit_position);
    if self.is_single_word() {
      self.val_ |= mask as i64;
    } // TODO else case
  }

  // Set the sign bit to 1.
  pub fn set_sign_bit(&mut self) {
    self.set_bit(self.bit_width_ - 1)
  }

  // Set a given bit to a given value.
  pub fn set_bit_val(&mut self, bit_position: u32, bit_value: bool) {
    if bit_value {
      self.set_bit(bit_position);
    } else {
      self.clear_bit(bit_position);
    }
  }

  // Set the bits from lo_bit to hi_bit to 1.
  pub fn set_bits_with_wrap(&mut self, lo_bit: u32, hi_bit: u32) {
    debug_assert!(hi_bit <= self.bit_width_, "hi_bit out of range");
    debug_assert!(lo_bit <= self.bit_width_, "lo_bit out of range");
    if lo_bit < hi_bit {
      self.set_bits(lo_bit, hi_bit);
      return;
    }
    self.set_low_bits(hi_bit);
    self.set_high_bits(self.bit_width_ - lo_bit);
  }

  // Set the bits from lo_bit to hi_bit to 1.
  pub fn set_bits(&mut self, lo_bit: u32, hi_bit: u32) {
    debug_assert!(hi_bit <= self.bit_width_, "hi_bit out of range");
    debug_assert!(lo_bit <= self.bit_width_, "lo_bit out of range");
    debug_assert!(lo_bit <= hi_bit, "lo_bit greater than hi_bit");
    if lo_bit == hi_bit {
      return;
    }
    if lo_bit < APINT_BITS_PER_WORD && hi_bit <= APINT_BITS_PER_WORD {
      let mut mask = WORD_TYPE_MAX >> (APINT_BITS_PER_WORD - (hi_bit - lo_bit));
      mask <<= lo_bit;
      if self.is_single_word() {
        self.val_ |= mask as i64;
      } // TODO else case
    } else {
      self.set_bits_slow_case(lo_bit, hi_bit);
    }
  }

  // Set the top bits starting from lo_bit.
  pub fn set_bits_from(&mut self, lo_bit: u32) {
    self.set_bits(lo_bit, self.bit_width_)
  }

  // Set the bottom lo_bits bits.
  pub fn set_low_bits(&mut self, lo_bits: u32) {
    self.set_bits(0, lo_bits)
  }

  // Set the top hi_bits bits.
  pub fn set_high_bits(&mut self, hi_bits: u32) {
    self.set_bits(self.bit_width_ - hi_bits, self.bit_width_)
  }

  // Set every bit to 0.
  pub fn clear_all_bits(&mut self) {
    if self.is_single_word() {
      self.val_ = 0;
    } // TODO else case
  }

  // Set a given bit to 0.
  pub fn clear_bit(&mut self, bit_position: u32) {
    debug_assert!(bit_position < self.bit_width_, "bit_position out of range");
    let mask = !APInt::mask_bit(bit_position);
    if self.is_single_word() {
      self.val_ &= mask as i64;
    } // TODO else case
  }

  // Set bottom lo_bits bits to 0.
  pub fn clear_low_bits(&mut self, lo_bits: u32) {
    debug_assert!(lo_bits <= self.bit_width_, "More bits than bit_width");
    let keep = APInt::get_high_bits_set(self.bit_width_,
      self.bit_width_ - lo_bits);
    self.val_ &= keep.val_;
  }

  // Set the sign bit to 0.
  pub fn clear_sign_bit(&mut self) {
    self.clear_bit(self.bit_width_ - 1)
  }

  // Toggle every bit to its opposite value.
  pub fn flip_all_bits(&mut self) {
    if self.is_single_word() {
      self.val_ ^= WORD_TYPE_MAX as i64;
      self.clear_unused_bits();
    } else {
      self.flip_all_bits_slow_case();
    }
  }

  pub fn flip_bit() {}

  // Negate this APInt in place.
  pub fn negate(&mut self) {
    self.flip_all_bits();
    self.val_ += 1;
  }

  pub fn insert_bits() {}

  pub fn extract_bits() {}

  pub fn extract_bits_as_zext_value() {}

  // Return the number of bits in the APInt.
  pub fn get_bit_width(&self) -> u32 {
    self.bit_width_
  }

  // Get the number of words.
  pub fn get_num_words_own(&self) -> u32 {
    self.get_num_words(self.bit_width_)
  }

  // Get the number of words.
  pub fn get_num_words(&self, bit_width: u32) -> u32 {
    (bit_width + APINT_BITS_PER_WORD - 1) / APINT_BITS_PER_WORD
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
  pub fn get_z_ext_value(&self) -> u64 {
    if self.is_single_word() {
      return self.val_ as u64;
    }
    if self.get_active_bits() > 64 {
      panic!("Too many bits for u64.");
    }
    self.pval_ as u64// TODO
  }

  // Get sign extended value
  pub fn get_s_ext_value(&self) -> i64 {
    if self.is_single_word() {
      return sign_extend_64(self.val_ as u64, self.bit_width_);
    }
    debug_assert!(self.get_significant_bits() <= 64, "Too meny bits for i64");
    0
  }

  // It counts the number of zeros from the significant bit to
  // the first one bit. 
  pub fn count_leading_zeros(&self) -> u32 {
    if self.is_single_word() {
      let unused_bits = APINT_BITS_PER_WORD - self.bit_width_;
      return count_leading_zeros(self.val_) - unused_bits;
    }
    self.count_trailing_zeros_slow_case()
  }

  // Count the number of leading one bits.
  pub fn count_leading_ones(&self) -> u32 {
    if self.is_single_word() {
      if self.bit_width_ == 0 {
        return 0;
      }
      return count_leading_ones((self.val_ << APINT_BITS_PER_WORD - self.bit_width_) as u64);
    }
    self.count_leading_ones_slow_case()
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
  // This function is an APInt version of the count_trailing_zeros functions
  // in math_extra.
  // It counts the number of zeros from the least significant bit to the
  // first set bit.
  // Returns bit_width if the value is zero, otherwise returns the number of
  // zeros from the least significant bit to the first one bit.
  pub fn count_trailing_zeros(&self) -> u32 {
    if self.is_single_word() {
      let trailing_zeros = count_trailing_zeros(self.val_ as u64);
      if trailing_zeros > self.bit_width_ {
        return self.bit_width_;
      } else {
        return trailing_zeros;
      }
    }
    self.count_trailing_zeros_slow_case()
  }

  // Count the number of trailing one bits.
  // This function is an APInt version of the count_trailing_ones functions
  // in math_extra.
  // It counts the number of ones from the least significant bit to the
  // first zero bit.
  pub fn count_trailing_ones(&self) -> u32 {
    if self.is_single_word() {
      return count_trailing_ones(self.val_ as u64);
    }
    self.count_trailing_ones_slow_case()
  }

  // Count the number of bits set.
  pub fn count_population(&self) -> u32 {
    if self.is_single_word() {
      return count_population(self.val_ as u64);
    }
    self.count_population_slow_case()
  }

  pub fn print() {}

  pub fn to_string() {}

  pub fn to_string_unsined() {}

  pub fn to_string_signed() {}

  pub fn byte_swap() {}

  // Returns the value with the bit representation reversed of this
  // APInt vlaue.
  pub fn reverse_bits(&self) -> APInt {
    let val = reverse_bits(self.val_ as u64);
    APInt::new(self.bit_width_, val as i64, false)
  }

  pub fn round_to_double() {}

  pub fn signed_round_to_double() {}

  pub fn bits_to_double() {}

  pub fn bits_to_float() {}

  pub fn double_to_bits() {}

  pub fn float_to_bits() {}

  // Returns the floor log base 2 of this APInt.
  pub fn log_base_2(&self) -> u32 {
    self.get_active_bits() - 1
  }

  // Returns the ceil log base 2 of this APInt.
  pub fn ceil_log_base_2(&self) -> u32 {
    let mut temp = APInt::new(self.bit_width_, self.val_, false);
    temp.val_ -= 1;
    temp.get_active_bits()
  }

  pub fn nearest_log_base_2() {}

  // Returns the log base 2 of this APInt if its an exact power of two,
  // -1 otherwise.
  pub fn exact_log_base_2(&self) -> i32 {
    if !self.is_power_of_2() {
      return -1;
    }
    self.log_base_2() as i32
  }

  pub fn sqrt() {}

  // Get the absolute value.
  pub fn abs(&self) /*-> APInt*/ {
    /*
    if self.is_negative() {
      let val = self.val_ as i64 * (-1);
      let res = APInt::new(self.bit_width_, val, false);
      return res;
    }
    APInt::new(self.bit_width_, self.val_, false)
    */
  }

  pub fn multiplicative_inverse() {}

  pub fn profile() {}

  pub fn dump() {}

  // Returns whether this instance allocated memory.
  pub fn needs_cleanup(&self) -> bool{
    !self.is_single_word()
  }

  // Clear unused high order bits.
  fn clear_unused_bits(&mut self) -> &APInt {
    // Compute how many bits are used in the final word.
    let word_bits = ((self.bit_width_ - 1) % APINT_BITS_PER_WORD) + 1;

    // Mask out the high bits.
    let mut mask = WORD_TYPE_MAX >> (APINT_BITS_PER_WORD - word_bits);
    if self.bit_width_ == 0 {
      mask = 0;
    }

    if self.is_single_word() {
      self.val_ &= mask as i64;
    } else {
      self.pval_ &= mask as i64;
    }

    self
  }

  // Get the word corresponding to a bit position.
  fn get_word(&self, bit_position: u32) -> u64 {
    if self.is_single_word() {
      return self.val_ as u64;
    } // TODO else case
    0
  }

  // Utility method to change the bit width of this ApInt to a
  // nyew bit width.
  fn reallocate(&mut self, new_bit_width: u32) {
    if self.get_num_words_own() == self.get_num_words(new_bit_width) {
      self.bit_width_ = new_bit_width;
      return;
    }
    // TODO !is_single_word
    self.bit_width_ = new_bit_width
  }

  fn from_string() {}

  fn divide() {}

  // TODO
  // out-of-line slow case for inline constructor
  fn init_slow_case(&mut self, val: i64, is_signed: bool) {
    self.pval_ = val;
    self.clear_unused_bits();
  }

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

  fn count_leading_ones_slow_case(&self) -> u32 {
    self.pval_.leading_ones()
  }

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

  fn intersects_slow_case(&self, rhs: &APInt) -> bool {
    false
  }

  fn is_subset_of_slow_case(&self, rhs: &APInt) -> bool {
    false
  }

  fn set_bits_slow_case(&self, lo_bit: u32, hi_bit: u32) {}

  fn flip_all_bits_slow_case(&self) {}

  // out-of-line slow case for concat.
  fn concat_slow_case(&self, new_lsb: APInt) -> Self {
    APInt::new_zero() // TODO
  }

  fn and_assign_slow_case(&self) {}

  fn or_assing_slow_case(&self) {}

  fn xor_assign_slow_case(&self) {}

  // Unsigned comparizon.
  fn compare(&self, rhs: &APInt) -> i64 {
    debug_assert!(self.bit_width_ == rhs.bit_width_, "Bit widths must be same for comparizon");
    if self.is_single_word() {
      if self.val_ < rhs.val_ {
        return -1;
      } else if self.val_ > rhs.val_ {
        return 1;
      } else {
        return 0;
      }
    }
    self.tc_compare(rhs.val_ as u64, self.bit_width_)
  }

  fn compare_signed() {}

  // Get the '0' value for the specified bit-width.
  pub fn get_zero(num_bits: u32) -> APInt {
    APInt::new(num_bits, 0, false)
  }

  // Return an APInt zero bits wide.
  pub fn get_zero_width() -> APInt {
    APInt::get_zero(0)
  }

  // Gets maximum unsigned value of APInt for specific bit width.
  pub fn get_max_value(num_bits: u32) -> APInt {
    APInt::get_all_ones(num_bits)
  }

  // Gets maximum signed value of APInt for a specific bit width
  pub fn get_signed_max_value(num_bits: u32) -> APInt {
    let mut api = APInt::get_all_ones(num_bits);
    api.clear_bit(num_bits - 1);
    api
  }

  // Gets minimum unsigned value of APInt for specific bit width.
  pub fn get_min_value(num_bits: u32) -> APInt {
    APInt::new(num_bits, 0, false)
  }

  // Get minimum signed value of APInt for a specific bit width.
  pub fn get_signed_min_value(num_bits: u32) -> APInt {
    let mut api = APInt::new(num_bits, 0, false);
    api.set_bit(num_bits - 1);
    api
  }

  // Get the SignMask for a specific bit width.
  pub fn get_sign_mask(bit_width: u32) -> APInt {
    APInt::get_signed_min_value(bit_width)
  }

  // Return an APInt of a specified width with all bits set.
  pub fn get_all_ones(num_bits: u32) -> APInt {
    APInt::new(num_bits, WORD_TYPE_MAX as i64, true)
  }

  // Return an APInt with exactly one bit set in the result.
  pub fn get_one_bit_set(num_bits: u32, bit_no: u32) -> APInt {
    let mut res = APInt::new(num_bits, 0, false);
    res.set_bit(bit_no);
    res
  }

  // Get a value with a block of bits set.
  pub fn get_bits_set(num_bits: u32, lo_bit: u32, hi_bit: u32) -> APInt {
    let mut res = APInt::new(num_bits, 0, false);
    res.set_bits(lo_bit, hi_bit);
    res
  }

  // Wrap version of get_bits_set.
  pub fn get_bits_set_with_wrap(num_bits: u32, lo_bit: u32, hi_bit: u32) -> APInt {
    let mut res = APInt::new(num_bits, 0, false);
    res.set_bits_with_wrap(lo_bit, hi_bit);
    res
  }

  // Constructs an APInt value that has a contiguous range of bits set.
  pub fn get_bits_set_from(num_bits: u32, lo_bit: u32) -> APInt {
    let mut res = APInt::new(num_bits, 0, false);
    res.set_bits_from(lo_bit);
    res
  }

  // Constructs an APInt value that has the top hi_bits_set bits set.
  pub fn get_high_bits_set(num_bits: u32, hi_bits_set: u32) -> APInt {
    let mut res = APInt::new(num_bits, 0, false);
    res.set_high_bits(hi_bits_set);
    res
  }

  // Constructs an APInt value that has the bottom lo_bits_set bits set.
  pub fn get_low_bits_set(num_bits: u32, lo_bits_set: u32) -> APInt {
    let mut res = APInt::new(num_bits, 0, false);
    res.set_low_bits(lo_bits_set);
    res
  }

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

  // Comparison (unsigned) of two bignums.
  pub fn tc_compare(&self, rhs: u64, num_words: u32) -> i64 {
    0
  }

  pub fn tc_increment() {}

  pub fn tc_decrement() {}

  // Determine which word a bit is in.
  fn which_word(bit_position: u32) -> u32 {
    bit_position / APINT_BITS_PER_WORD
  }

  // Determine which bit in a word the specified bit position is in.
  fn which_bit(bit_position: u32) -> u32 {
    bit_position % APINT_BITS_PER_WORD
  }

  // Get a single bit mask.
  fn mask_bit(bit_position: u32) -> u64 {
    let bit = APInt::which_bit(bit_position) as u64;
    1 << bit
  }

  // Array-indexing support.
  // Returns the bit value at bit_position.
  fn at(&self, bit_position: u32) -> bool {
    debug_assert!(bit_position < self.bit_width_, "Bit position out of bounds!");
    APInt::mask_bit(bit_position) & self.get_word(bit_position) != 0
  }
}

// Array-indexing support.
// Returns the bit value at bit position.
impl Index<u32> for APInt {
  type Output = bool;
  fn index(&self, bit_position: u32) -> &Self::Output {
    debug_assert!(bit_position < self.get_bit_width(), "Bit position out of bounds!");
    if (APInt::mask_bit(bit_position) & self.get_word(bit_position)) != 0 {
      return &true;
    } else {
      return &false;
    }
  }
}

impl BitAnd<APInt> for APInt {
  type Output = APInt;
  fn bitand(self, rhs: APInt) -> Self::Output {
    if self.is_single_word() && rhs.is_single_word() {
      APInt::new(self.bit_width_ , self.val_ & rhs.val_, false)
    } else { // TODO
      APInt::new(self.bit_width_ , self.val_ & rhs.val_, false)
    }
  }
}

impl BitAndAssign<APInt> for APInt {
  fn bitand_assign(&mut self, rhs: APInt) {
    if self.is_single_word() {
      self.val_ &= rhs.val_;
    } else {
      // and_assign_slow_case()
    }
  }
}

impl BitAndAssign<u64> for APInt {
  fn bitand_assign(&mut self, rhs: u64) {
    if self.is_single_word() {
      self.val_ &= rhs as i64;
    } else {
      // self.pval_ = rhs
    }
  }
}

impl BitOr<APInt> for APInt {
  type Output = APInt;
  fn bitor(self, rhs: APInt) -> Self::Output {
    if self.is_single_word() && rhs.is_single_word() {
      APInt::new(self.bit_width_ , self.val_ | rhs.val_, false)
    } else { // TODO
      APInt::new(self.bit_width_ , self.val_ | rhs.val_, false)
    }
  }
}

impl BitOrAssign<APInt> for APInt {
  fn bitor_assign(&mut self, rhs: APInt) {
    if self.is_single_word() {
      self.val_ |= rhs.val_;
    } else {
      // or_assign_slow_case()
    }
  }
}

impl BitOrAssign<u64> for APInt {
  fn bitor_assign(&mut self, rhs: u64) {
    if self.is_single_word() {
      self.val_ |= rhs as i64;
    } else {
      // self.pval_ |= rhs
    }
  }
}

impl BitXor<APInt> for APInt {
  type Output = APInt;
  fn bitxor(self, rhs: APInt) -> Self::Output {
    if self.is_single_word() && rhs.is_single_word() {
      APInt::new(self.bit_width_ , self.val_ ^ rhs.val_, false)
    } else { // TODO
      APInt::new(self.bit_width_ , self.val_ ^ rhs.val_, false)
    }
  }
}

impl BitXorAssign<APInt> for APInt {
  fn bitxor_assign(&mut self, rhs: APInt) {
    if self.is_single_word() {
      self.val_ ^= rhs.val_;
    } else {
      // xor_assign_slow_case()
    }
  }
}

impl BitXorAssign<u64> for APInt {
  fn bitxor_assign(&mut self, rhs: u64) {
    if self.is_single_word() {
      self.val_ ^= rhs as i64;
    } else {
      // TODO
    }
  }
}

impl Mul<APInt> for APInt {
  type Output = APInt;
  fn mul(self, rhs: APInt) -> Self::Output {
    if self.is_single_word() {
      APInt::new(self.bit_width_, self.val_*rhs.val_, false)
    } else {
      // TODO
      APInt { val_: self.val_ * rhs.val_, pval_: self.pval_, bit_width_: self.bit_width_ }
    }
  }
}

impl MulAssign<APInt> for APInt {
  fn mul_assign(&mut self, rhs: APInt) {
    if self.is_single_word() {
      self.val_ *= rhs.val_;
    } else {
      // TODO
    }
  }
}

impl MulAssign<u64> for APInt {
  fn mul_assign(&mut self, rhs: u64) {
    if self.is_single_word() {
      self.val_ *= rhs as i64;
    } else {
      // TODO
    }
  }
}

impl Add<APInt> for APInt {
  type Output = APInt;
  fn add(self, rhs: APInt) -> Self::Output {
    if self.is_single_word() && rhs.is_single_word() {
      APInt::new(self.bit_width_ , self.val_ + rhs.val_, false)
    } else { // TODO
      APInt::new(self.bit_width_ , self.val_ + rhs.val_, false)
    }
  }
}

impl AddAssign<APInt> for APInt {
  fn add_assign(&mut self, rhs: APInt) {
    if self.is_single_word() && rhs.is_single_word() {
      self.val_ = rhs.val_;
      self.bit_width_ = rhs.bit_width_;
    }
    // TODO
    // assign_slow_case()
  }
}

impl AddAssign<u64> for APInt {
  fn add_assign(&mut self, rhs: u64) {
    if self.is_single_word() {
      self.val_ = rhs as i64;
      self.clear_unused_bits();
      return;
    }
    // TODO
    // self.pval_ = rhs;
  }
}

impl Sub<APInt> for APInt {
  type Output = APInt;
  fn sub(self, rhs: APInt) -> Self::Output {
    if self.is_single_word() && rhs.is_single_word() {
      APInt::new(self.bit_width_ , self.val_ - rhs.val_, false)
    } else { // TODO
      APInt::new(self.bit_width_ , self.val_ - rhs.val_, false)
    }
  }
}

impl SubAssign<APInt> for APInt {
  fn sub_assign(&mut self, rhs: APInt) {
    if self.is_single_word() {
      self.val_ -= rhs.val_;
    } else {
      // TODO
    }
  }
}

impl SubAssign<u64> for APInt {
  fn sub_assign(&mut self, rhs: u64) {
    if self.is_single_word() {
      self.val_ -= rhs as i64;
    } else {
      // TODO
    }
  }
}

impl Shl<APInt> for APInt {
  type Output = APInt;
  fn shl(self, rhs: APInt) -> Self::Output {
    if self.is_single_word() && rhs.is_single_word() {
      APInt::new(self.bit_width_ , self.val_ << rhs.val_, false)
    } else { // TODO
      APInt::new(self.bit_width_ , self.val_ << rhs.val_, false)
    }
  }
}

impl ShlAssign<APInt> for APInt {
  fn shl_assign(&mut self, rhs: APInt) {
    // TODO
  }
}

impl ShlAssign<u32> for APInt {
  fn shl_assign(&mut self, rhs: u32) {
    // TODO
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /*
  //#[test]
  fn test_value_init() {
    let zero = APInt::new_zero();
    assert_eq!(zero.val_, 0);
    assert_eq!(zero.zext(64).val_, 0);
    // TODO zero.sext
  }
  */

  //#[test]
  //fn test_shift_left_by_zero() {
    //let one = APInt::get_zero(65) + 
      //APInt::new(65, 1, false);
    //let shl = one.shl(0);
    //assert_eq!(shl.at(0), true);
    //assert_eq!(shl.at(1), false);
  //}

  #[test]
  fn test_i64_arithmetic_right_shift_negative() {
    let neg_one = APInt::new(64, -1, true);
    assert_eq!(neg_one.ashr(7), neg_one);
  }

  /*
  #[test]
  fn test_i128_negative_count() {
    let minus3 = APInt::new(128, -3, true);
    assert_eq!(minus3.count_leading_ones(), 126);
    assert_eq!(minus3.get_s_ext_value(), -3);
  }
  */

  #[test]
  fn test_i33_count() {
    let i33minus2= APInt::new(33, -2, true);
    assert_eq!(i33minus2.count_leading_zeros(), 0);
    assert_eq!(i33minus2.count_leading_ones(), 32);
    assert_eq!(i33minus2.get_active_bits(), 33);
    assert_eq!(i33minus2.count_trailing_zeros(), 1);
    assert_eq!(i33minus2.count_population(), 32);
    assert_eq!(i33minus2.get_s_ext_value(), -2);
    //assert_eq!(i33minus2.get_z_ext_value(), -2 & 1 << 33 - 1);
  }

  #[test]
  fn test_i61_count() {
    let mut i61 = APInt::new(61, 1 << 15, false);
    assert_eq!(i61.count_leading_zeros(), 45);
    assert_eq!(i61.count_leading_ones(), 0);
    assert_eq!(i61.get_active_bits(), 16);
    assert_eq!(i61.count_trailing_zeros(), 15);
    assert_eq!(i61.count_population(), 1);
    assert_eq!(i61.get_s_ext_value(), 1 << 15 as i64);
    assert_eq!(i61.get_z_ext_value(), 1 << 15);

    i61.set_bits(8, 19);
    assert_eq!(i61.count_leading_zeros(), 42);
    assert_eq!(i61.count_leading_ones(), 0);
    assert_eq!(i61.get_active_bits(), 19);
    assert_eq!(i61.count_trailing_zeros(), 8);
    assert_eq!(i61.count_population(), 11);
    assert_eq!(i61.get_s_ext_value(), ((1 << 19) - (1 << 8)) as i64);
    assert_eq!(i61.get_z_ext_value(), ((1 << 19) - (1 << 8)));
  }

  #[test]
  fn test_i1() {
    let neg_two = APInt::new(1, -2, true);
    let neg_one = APInt::new(1, -1, true);
    let zero = APInt::new(1, 0, false);
    let one = APInt::new(1, 1, false);
    let two = APInt::new(1, 2, false);

    assert_eq!(neg_two.get_s_ext_value(), 0);
    assert_eq!(neg_one.get_s_ext_value(), -1);
    assert_eq!(neg_one.get_z_ext_value(), 1);
    assert_eq!(zero.get_z_ext_value(), 0);
    assert_eq!(one.get_s_ext_value(), -1);
    assert_eq!(one.get_z_ext_value(), 1);
    assert_eq!(two.get_z_ext_value(), 0);
    assert_eq!(two.get_s_ext_value(), 0);

    // Basic equalities for 1-bit values.
    assert_eq!(zero, two);
    assert_eq!(zero, neg_two);
    assert_eq!(one, neg_one);
    assert_eq!(two, neg_two);

    // Min/max signed values.
    assert_eq!(zero.is_max_signed_value(), true);
    assert_eq!(one.is_max_signed_value(), false);
    assert_eq!(zero.is_min_signed_value(), false);
    assert_eq!(one.is_min_signed_value(), true);

    // Additions.
    assert_eq!(one.clone() + one.clone(), two);
    assert_eq!(neg_one.clone() + one.clone(), zero);
    assert_eq!(neg_one.clone() + neg_one.clone(), neg_two);

    // Subtractions.
    assert_eq!(neg_one.clone() - one.clone(), neg_two);
    assert_eq!(one.clone() - neg_one.clone(), two);
    assert_eq!(one.clone() - one.clone(), zero);

    // And
    assert_eq!(zero.clone() & zero.clone(), zero);
    assert_eq!(one.clone() & zero.clone(), zero);
    assert_eq!(zero.clone() & one.clone(), zero);
    assert_eq!(one.clone() & one.clone(), one);
    assert_eq!(neg_one.clone() & zero.clone(), zero);
    assert_eq!(zero.clone() & neg_one.clone(), zero);
    assert_eq!(neg_one.clone() & neg_one.clone(), neg_one);

    // Or
    assert_eq!(zero.clone() | zero.clone(), zero);
    assert_eq!(one.clone() | zero.clone(), one);
    assert_eq!(zero.clone() | one.clone(), one);
    assert_eq!(one.clone() | one.clone(), one);
    assert_eq!(neg_one.clone() | zero.clone(), neg_one);
    assert_eq!(zero.clone() | neg_one.clone(), neg_one);
    assert_eq!(neg_one.clone() | neg_one.clone(), neg_one);

    // Xor
    assert_eq!(zero.clone() ^ zero.clone(), zero);
    assert_eq!(one.clone() ^ zero.clone(), one);
    assert_eq!(zero.clone() ^ one.clone(), one);
    assert_eq!(one.clone() ^ one.clone(), zero);
    assert_eq!(neg_one.clone() ^ zero.clone(), neg_one);
    assert_eq!(zero.clone() ^ neg_one.clone(), neg_one);
    assert_eq!(neg_one.clone() ^ neg_one.clone(), zero);

    // Shifts.
    assert_eq!(one.clone() << one.clone(), zero);
    assert_eq!(one.clone() << zero.clone(), one);

  }
}