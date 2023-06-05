#![allow(dead_code)]

// This file defines the class that knows how to divide SCEV's.

struct SCEVDivision {}
impl SCEVDivision {
  pub fn new() {}
  pub fn divide() {}
  pub fn visit_ptr_to_int_expr() {}
  pub fn visit_truncate_expr() {}
  pub fn visit_zero_extend_expr() {}
  pub fn visit_sign_extend_expr() {}
  pub fn visit_udiv_expr() {}
  pub fn visit_smax_expr() {}
  pub fn visit_umax_expr() {}
  pub fn visit_smin_expr() {}
  pub fn visit_umin_expr() {}
  pub fn visit_sequential_umin_expr() {}
  pub fn visit_unknown() {}
  pub fn visit_could_not_compute() {}
  pub fn visit_constant() {}
  pub fn visit_vscale() {}
  pub fn visit_add_rec_expr() {}
  pub fn visit_add_expr() {}
  pub fn visit_mul_expr() {}
  fn cannot_divide() {}
}