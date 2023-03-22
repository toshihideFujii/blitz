#![allow(dead_code)]

// This file defines the fast math flags.

struct FastMathFlags {}
impl FastMathFlags {
  pub fn new() {}
  pub fn get_fast() {}
  pub fn any() {}
  pub fn none() {}
  pub fn all() {}
  pub fn clear() {}
  pub fn set() {}
  pub fn allow_reassoc() {}
  pub fn no_nans() {}
  pub fn no_infs() {}
  pub fn no_signed_zeros() {}
  pub fn allow_reciprocal() {}
  pub fn allow_contract() {}
  pub fn approx_func() {}
  pub fn is_fast() {}
}