#![allow(dead_code)]

pub struct DeviceMemoryBase {
  size: usize,
  payload: u64
}

impl DeviceMemoryBase {
  pub fn new() {}
  pub fn is_null() {}

  // Returns the size, in bytes, for the backing memory.
  pub fn size(&self) -> usize {
    self.size
  }

  pub fn opaque() {}

  // Returns the payload of this memory region.
  pub fn payload(&self) -> u64 {
    self.payload
  }

  // Sets payload to given value.
  pub fn set_payload(&mut self, payload :u64) {
    self.payload = payload;
  }

  pub fn is_same_as() {}
  pub fn get_byte_slice() {}
  pub fn reset() {}
}