#![allow(dead_code)]

use std::i64;

pub fn multiply_without_overflow(x: i64, y: i64) -> i64 {
  let ux: u64 = x as u64;
  let uy: u64 = y as u64;
  let uxy = ux * uy;

  if (ux | uy) >> 32 != 0 {
    assert!(x >= 0 && y >= 0);
    if ux != 0 && uxy / ux != uy {
      return -1;
    }
  }
  uxy as i64
}

#[inline]
pub fn overflow_safe_multiply(x: i64, y: i64) -> (i64, bool) {
  let ux: u64 = x as u64;
  let uy: u64 = y as u64;
  let uxy = ux * uy;

  let result = uxy as i64;
  let mut bad = if result < 0 { true } else { false };

  if (ux | uy) >> 32 != 0 {
    if x < 0 || y < 0 {
      bad = true;
    } else if ux != 0 && uxy / ux != uy {
      bad = true;
    }
  }
  (result, bad)
}