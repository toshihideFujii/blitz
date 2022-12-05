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