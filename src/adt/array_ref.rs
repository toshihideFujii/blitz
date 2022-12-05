#![allow(dead_code)]

struct ArrayRef {}

impl ArrayRef {
  pub fn new() {}

  pub fn begin() {}
  pub fn end() {}
  pub fn rbegin() {}
  pub fn rend() {}

  pub fn empty() {}
  pub fn data() {}
  pub fn size() {}
  pub fn front() {}
  pub fn back() {}

  pub fn copy() {}
  pub fn equals() {}
  pub fn slice() {}

  pub fn drop_front() {}
  pub fn drop_back() {}
  pub fn drop_while() {}
  pub fn drop_until() {}

  pub fn take_front() {}
  pub fn take_back() {}
  pub fn take_while() {}
  pub fn take_until() {}
}

struct MutableArrayRef {}

impl MutableArrayRef {
  pub fn new() {}

  pub fn begin() {}
  pub fn end() {}
  pub fn rbegin() {}
  pub fn rend() {}

  pub fn empty() {}
  pub fn data() {}
  pub fn size() {}
  pub fn front() {}
  pub fn back() {}

  pub fn copy() {}
  pub fn equals() {}
  pub fn slice() {}

  pub fn drop_front() {}
  pub fn drop_back() {}
  pub fn drop_while() {}
  pub fn drop_until() {}

  pub fn take_front() {}
  pub fn take_back() {}
  pub fn take_while() {}
  pub fn take_until() {}
}

struct OwningArrayRef {}