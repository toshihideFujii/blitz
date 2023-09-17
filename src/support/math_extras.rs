#![allow(dead_code)]
#![allow(unused_variables)]

// Count number of 0's from the least significant bit to the most
// stopping at the first 1.
pub fn count_trailing_zeros(val: u64) -> u32 {
  val.trailing_zeros()
}

// Count the number of ones from the least significant bit to
// the first zero bit.
pub fn count_trailing_ones(val: u64) -> u32 {
  val.trailing_ones()
}

// Count number of 0's from the most significant bit to the 
// least stopping at the first 1.
pub fn count_leading_zeros(val: i64) -> u32 {
  val.leading_zeros()
}

// Count the number of ones from the most significant bit to
// the first zero bit.
pub fn count_leading_ones(val: u64) -> u32 {
  val.leading_ones()
}

// Count the number of set bits in a value.
pub fn count_population(val: u64) -> u32 {
  val.count_ones()
}

// Reverse the bits in val.
pub fn reverse_bits(val: u64) -> u64 {
  val.reverse_bits()
}

// Return true if the argument is a non-empty sequence of ones
// starting at the least significant bit with the remainder zero
// (64 bit verion).
pub fn is_mask_64(value: u64) -> bool {
  value != 0 && ((value + 1) & value) == 0
}

// Return true if the argument contains a non-empty sequence of
// ones with the remainder zero (64 bit version).
pub fn is_shifted_mask_64(value: u64) -> bool {
  value != 0 && is_mask_64((value - 1) | value)
}

// Return true if the argument is a power of 2 > 0 (32 bit edition).
pub fn is_power_of_2_32(value: u32) -> bool {
  value.is_power_of_two()
}

// Return true if the argument is a power of 2 > 0 (64 bit edition).
pub fn is_power_of_2_64(value: u64) -> bool {
  value.is_power_of_two()
}

// Return the floor log base 2 of the specified value, -1 if the
// value is zero (32 bit edition).
pub fn log2_32(value: u32) -> u32 {
  value.ilog2()
}

// Return the floor log base 2 of the specified value, -1 if the
// value is zero (64 bit edition).
pub fn log2_64(value: u64) -> u32 {
  value.ilog2()
}

// a and b are either alignments or offsets.
// Return the minimum alignment that may be assumed after adding the
// two together.
pub fn min_align(a: u64, b: u64) -> u64 {
  (a | b) & (1 + !(a | b))
}

// Returns the next power of two (in 64-bits) that is strictly greater
// than a. Returns zero on overflow.
pub fn next_power_of_2(mut a: u64) -> u64 {
  a |= a >> 1;
  a |= a >> 2;
  a |= a >> 4;
  a |= a >> 8;
  a |= a >> 16;
  a |= a >> 32;
  a + 1
}

// Sign-extend the number in the bottom b bits of x to a 32-bit integer.
// Requires 0 < b <= 32.
pub fn sign_extend_32(x: u32, b: u32) -> i32 {
  debug_assert!(b > 0, "Bit width can't be 0.");
  debug_assert!(b <= 32, "Bit width out of range.");
  let val = (x << (32 - b)) as i32;
  val >> (32 - b)
}

// Sign-extend the number in the bottom b bits of x to a 64-bit integer.
// Requires 0 < b <= 64.
pub fn sign_extend_64(x: u64, b: u32) -> i64 {
  debug_assert!(b > 0, "Bit width can't be 0.");
  debug_assert!(b <= 64, "Bit width out of range.");
  let val = (x << (64 - b)) as i64;
  val >> (64 - b)
}