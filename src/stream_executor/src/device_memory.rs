#![allow(dead_code)]

use std::mem::size_of;

pub struct DeviceMemoryBase {
  size: usize,
  payload: u64
}

impl DeviceMemoryBase {
  pub fn new(size: usize) -> Self {
    DeviceMemoryBase { size: size, payload: 0 }
  }

  // Returns whether the backing memory is the null pointer.
  pub fn is_null(&self) -> bool {
    unimplemented!()
  }

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

  // Returns whether the two DeviceMemoryBase segments are identical.
  pub fn is_same_as(&self, _other: &DeviceMemoryBase) -> bool {
    unimplemented!()
  }

  pub fn get_byte_slice(&self, _offset_bytes: u64, _size_bytes: u64) -> Self {
    unimplemented!()
  }

  pub fn reset() {}
}

// Typed wrapper around "void *"-like DeviceMemoryBase.
pub struct Devicememory<T> {
  type_: T,
  base: DeviceMemoryBase
}

impl<T> Devicememory<T> {
  pub fn new() {}

  // Returns the number of elements of type T that constitute this allocation.
  pub fn element_count(&self) -> usize {
    self.base.size() / size_of::<T>()
  }

  // Returns whether this is a single-element allocation.
  pub fn is_scalar(&self) -> bool {
    self.element_count() == 1
  }

  pub fn make_form_byte_size() {}
  pub fn get_slice() {}
  pub fn reset_from_byte_size() {}
}