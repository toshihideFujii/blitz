#![allow(dead_code)]

// This file defines the classes used to represent and build scalar expressions.

enum SCEVTypes {
  ScConstant,
  ScVscale,
  ScTruncate,
  ScZeroExtend,
  ScSignExtend
}

struct SCEVConstant {}
impl SCEVConstant {
  pub fn new() {}
  pub fn get_value() {}
  pub fn get_apint() {}
  pub fn get_type() {}
  pub fn class_of() {}
}