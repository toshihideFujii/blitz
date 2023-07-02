#![allow(dead_code)]

// This file defines the fast math flags.

use std::ops::{BitAndAssign, BitOrAssign};

// This is how the bits are used in Value::sub_class_optional_data
// so they should fit there too.
enum FMFBits {
  AllowReassoc = (1 << 0),
  NoNans = (1 << 1),
  NoInfs = (1 << 2),
  NoSignedZeros = (1 << 3),
  AllowReciprocal = (1 << 4),
  AllowContract = (1 << 5),
  ApproxFunc = (1 << 6)
}

// Convenience struct for specifying and reasoning about fast-math flags.
struct FastMathFlags {
  flags: u32
}

impl FastMathFlags {
  pub fn new(f: u32) -> Self {
    let mut fmf = FastMathFlags { flags: f };
    // If all 7 bits are set, turn this into -1.
    // If the number of bits grows, this must be updated.
    if f == 0x7F {
      fmf.flags = !0;
    }
    fmf
  }
  pub fn get_fast() -> Self {
    let mut fmf = FastMathFlags::new(0);
    fmf.set_fast(true);
    fmf
  }

  pub fn any(&self) -> bool {
    self.flags != 0
  }

  pub fn none(&self) -> bool {
    self.flags == 0
  }

  pub fn all(&self) -> bool {
    self.flags == !0
  }

  pub fn clear(&mut self) {
    self.flags = 0;
  }

  pub fn set(&mut self) {
    self.flags = !0;
  }

  // Flag queries.
  pub fn allow_reassoc(&self) -> bool {
    self.flags & FMFBits::AllowReassoc as u32 != 0
  }

  pub fn no_nans(&self) -> bool {
    self.flags & FMFBits::NoNans as u32 != 0
  }

  pub fn no_infs(&self) -> bool {
    self.flags & FMFBits::NoInfs as u32 != 0
  }

  pub fn no_signed_zeros(&self) -> bool {
    self.flags & FMFBits::NoSignedZeros as u32 != 0
  }

  pub fn allow_reciprocal(&self) -> bool {
    self.flags & FMFBits::AllowReciprocal as u32 != 0
  }

  pub fn allow_contract(&self) -> bool {
    self.flags & FMFBits::AllowContract as u32 != 0
  }

  pub fn approx_func(&self) -> bool {
    self.flags & FMFBits::ApproxFunc as u32 != 0
  }

  pub fn is_fast(&self) -> bool {
    self.all()
  }

  // Flag setters.
  pub fn set_allow_reassoc(&mut self, b: bool) {
    self.flags = (self.flags & !(FMFBits::AllowReassoc as u32)) |
      b as u32 * FMFBits::AllowReassoc as u32;
  }

  pub fn set_no_nans(&mut self, b: bool) {
    self.flags = (self.flags & !(FMFBits::NoNans as u32)) |
      b as u32 * FMFBits::NoNans as u32;
  }

  pub fn set_no_infs(&mut self, b: bool) {
    self.flags = (self.flags & !(FMFBits::NoInfs as u32)) |
      b as u32 * FMFBits::NoInfs as u32;
  }

  pub fn set_no_signed_zeros(&mut self, b: bool) {
    self.flags = (self.flags & !(FMFBits::NoSignedZeros as u32)) |
      b as u32 * FMFBits::NoSignedZeros as u32;
  }

  pub fn set_allow_reciprocal(&mut self, b: bool) {
    self.flags = (self.flags & !(FMFBits::AllowReciprocal as u32)) |
      b as u32 * FMFBits::AllowReciprocal as u32;
  }

  pub fn set_allow_contract(&mut self, b: bool) {
    self.flags = (self.flags & !(FMFBits::AllowContract as u32)) |
      b as u32 * FMFBits::AllowContract as u32;
  }

  pub fn set_approx_func(&mut self, b: bool) {
    self.flags = (self.flags & !(FMFBits::ApproxFunc as u32)) |
      b as u32 * FMFBits::ApproxFunc as u32;
  }

  pub fn set_fast(&mut self, b: bool) {
    if b { self.set(); } else { self.clear(); }
  }

  // Print fast math flags.
  pub fn print(&self) {
    if self.all() {
      println!(" fast");
    } else {
      if self.allow_reassoc() {
        println!(" reassoc");
      }
      if self.no_nans() {
        println!(" nnan");
      }
      if self.no_infs() {
        println!(" ninf");
      }
      if self.no_signed_zeros() {
        println!(" nsz");
      }
      if self.allow_reciprocal() {
        println!(" arcp")
      }
      if self.allow_contract() {
        println!(" contract")
      }
      if self.approx_func() {
        println!(" afn")
      }
    }
  }
}

impl BitAndAssign for FastMathFlags {
  fn bitand_assign(&mut self, rhs: Self) {
    self.flags &= rhs.flags;
  }
}

impl BitOrAssign for FastMathFlags {
  fn bitor_assign(&mut self, rhs: Self) {
    self.flags |= rhs.flags;
  }
}