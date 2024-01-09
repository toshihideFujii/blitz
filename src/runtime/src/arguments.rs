#![allow(dead_code)]

pub trait Argument {
  fn new() {}
  fn verify() {}
  fn pack() {}
  fn to_string() {}
}

pub struct OpaqueArg {}

pub struct ScalarArg {}

pub struct MemrefDesc {}

pub struct AsyncTokenArg {}

pub struct AsyncScalarArg {}

pub struct AsyncMemrefArg {}

pub struct Arguments {
  num_args: usize,
}

impl Arguments {
  pub fn new() {}
  pub fn push_back() {}
  pub fn emplace_back() {}
  pub fn size() {}
}