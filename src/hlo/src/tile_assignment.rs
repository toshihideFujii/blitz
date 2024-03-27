#![allow(dead_code)]

pub struct IotaTileAssignment {
  ndims: i64,
  reshape_ndims: i32,
  storage: Vec<u8>,
}

impl IotaTileAssignment {
  pub fn new() {}

  pub fn value_at() {}

  pub fn ndims(&self) -> i64 {
    self.ndims
  }
  
  pub fn dims() {}
  pub fn dim() {}
  pub fn reshape_dims() {}
  pub fn transpose_perm() {}
  pub fn num_elements() {}
  pub fn transpose() {}
  pub fn print() {}
  pub fn to_string() {}
}


#[derive(Clone, PartialEq)]
pub struct TileAssignment {}

impl TileAssignment {
  pub fn new() -> Self {
    TileAssignment {  }
  }

  pub fn dimensions(&self) -> &Vec<i64> {
    unimplemented!()
  }

  pub fn num_dimensions(&self) -> usize { 0 }

  pub fn dim(&self, _n: i64) -> i64 {
    0
  }

  pub fn num_elements() {}
  pub fn first() {}
  pub fn each() {}
  pub fn each_status() {}
  pub fn reshape() {}
  pub fn transpose() {}
  pub fn print() {}
  pub fn to_string() {}

  pub fn uses_device(&self, _device: i64) -> bool { false }

  pub fn iota() {}
  pub fn array(&self) -> &Vec<i64> {
    unimplemented!()
  }
  pub fn shared_array() {}
  pub fn shared_array_clone() {}
}