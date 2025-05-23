#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BufferValue {
  id: i64,
  is_array: bool,
  is_tuple: bool,
  color: i64
}

impl BufferValue {
  pub fn new() -> Self {
    BufferValue {
      id: 0,
      is_array: false,
      is_tuple: false,
      color: -1
    }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn instruction() {}
  pub fn index() {}

  // Return the color of the BufferValue.
  pub fn color(&self) -> i64 {
    debug_assert!(self.color != -1);
    self.color
  }

  pub fn set_color(&mut self, color: i64) {
    self.color = color;
  }

  pub fn has_color(&self) -> bool {
    self.color != -1
  }

  pub fn shape() {}
  pub fn is_top_level() {}

  // Whether this buffer contains a tuple.
  pub fn is_tuple(&self) -> bool {
    self.is_tuple
  }

  // Whether this buffer contains an array.
  pub fn is_array(&self) -> bool {
    self.is_array
  }
  
  pub fn to_string() {}
}