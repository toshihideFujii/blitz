#![allow(dead_code)]

// This file implements methods to test, set and extract
//typed bits from packed unsigned integers.

// Compressor is used to manipulate the bits of a (possibly signed)
// integer type so it can be packed and unpacked into a 'bits'
// sized integer, Compressor is specialized on signed-ness so no
// runtime cost is incurred.
struct Compressor {}
impl Compressor {
  pub fn pack() {}
  pub fn unpack() {}
}

struct Element {}

struct Bitfield {}
impl Bitfield {
  pub fn get() {}
  pub fn test() {}
  pub fn set() {}
  pub fn is_over_lapping() {}
  pub fn are_contiguous() {}
}