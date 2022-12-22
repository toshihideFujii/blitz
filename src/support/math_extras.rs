#![allow(dead_code)]
#![allow(unused_variables)]

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

// Return true if the argument is a power of 2 > 0
// (64 bit edition).
pub fn is_power_of_2_64(value: u64) -> bool {
  false
}

// Sign-extend the number in the bottom b bits of x to a 32-bit integer.
// Requires 0 < b <= 32.
pub fn sign_extend_32(x: u32, b: u32) -> i32 {
  debug_assert!(b > 0, "Bit width can't be 0.");
  debug_assert!(b <= 32, "Bit width out of range.");
  let val = ((x << (32 - b)) >> (32 - b)) as i32;
  val
}

// Sign-extend the number in the bottom b bits of x to a 64-bit integer.
// Requires 0 < b <= 64.
pub fn sign_extend_64(x: u64, b: u32) -> i64 {
  debug_assert!(b > 0, "Bit width can't be 0.");
  debug_assert!(b <= 64, "Bit width out of range.");
  let val = ((x << (64 - b)) >> (64 - b)) as i64;
  val
}