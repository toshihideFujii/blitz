#![allow(dead_code)]

pub struct BufferValue {
  id: i64,
  is_array: bool,
  is_tuple: bool,
}

impl BufferValue {
  pub fn new() {}

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn instruction() {}
  pub fn index() {}
  pub fn shape() {}
  pub fn is_top_level() {}

  pub fn is_tuple(&self) -> bool {
    self.is_tuple
  }

  pub fn is_array(&self) -> bool {
    self.is_array
  }
  
  pub fn to_string() {}
}