#![allow(dead_code)]

// This file contains types to represent alignments.
// They are instrumented to guarantee some invariants are preserved
// and prevent invalid manipulations.

use super::math_extras::{is_power_of_2_64,log2_64, min_align};

// This struct is a compact representation of a valid (non-zero
// power of two) alignment.
pub struct Align {
  shift_value: u8
}

impl Align {
  pub fn new(value: u64) -> Self {
    debug_assert!(value > 0, "Value must not be 0.");
    debug_assert!(is_power_of_2_64(value), "Alignment is not a power of 2.");
    Align { shift_value: log2_64(value) as u8 }
  }

  pub fn value(&self) -> u64 {
    1 << self.shift_value
  }

  // Returns the previous alignment.
  pub fn previous(&self) -> Align {
    debug_assert!(self.shift_value != 0, "Undefined operation.");
    Align { shift_value: self.shift_value - 1 }
  }
}

// This struct is a compact representation of a valid (power of two)
// or undefined (0) alignment.
pub struct MaybeAlign {
  shift_value: u8
}

impl MaybeAlign {
  pub fn new(value: u64) -> Self {
    MaybeAlign { shift_value: value as u8 }
  }

  pub fn value_or_one() {}

  pub fn value(&self) -> u64 {
    1 << self.shift_value
  }
}

// Checks that size_in_bytes is a multiple of the alignment.
pub fn is_aligned(lhs: &Align, size_in_bytes: u64) -> bool {
  size_in_bytes % lhs.value() == 0
}

pub fn is_addr_aligned() {}

// Returns a multiple of 'a' needed to store 'size' bytes.
pub fn align_to(size: u64, a: &Align) -> u64 {
  let value = a.value();
  // The following line is equivalent to '(size + va;ue - 1) / value * value'.
  (size + value - 1) & !(value - 1)
}

pub fn align_addr() {}

// Returns the offset to the next integer (mod 2**64) that is greater
// than or equal to value and is a multiple of align.
pub fn offset_to_alignment(value: u64, alignment: &Align) -> u64 {
  align_to(value, alignment) - value
}

pub fn offset_to_aligned_addr() {}

// Returns the log2 of the alignment.
pub fn log2(a: &Align) -> u32 {
  a.shift_value as u32
}

// Return the alignment that satisfies both alignments.
// Same semantic as min_align.
pub fn common_alignment(a: &Align, offset: u64) -> Align {
  Align::new(min_align(a.value(), offset))
}

// Return a representation of the alignment that encodes undefined as 0.
pub fn encode(a: Option<MaybeAlign>) -> u32 {
  if a.is_some() {
    return a.unwrap().shift_value as u32 + 1;
  } else {
    return 0;
  }
}

// Dual operation of the encode function above.
pub fn decode_maybe_align(value: u32) -> MaybeAlign {
  if value == 0 {
    return MaybeAlign::new(0);
  } else {
    return MaybeAlign::new(value as u64 - 1);
  }
}