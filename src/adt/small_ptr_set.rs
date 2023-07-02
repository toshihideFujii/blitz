#![allow(dead_code)]

// This class implements a set which is optimized for holding
// SmallSize or less elements.
#[derive(Debug, Clone, PartialEq)]
pub struct SmallPtrSet<PtrType> {
  dummy: Option<PtrType>
}
impl<PtrType> SmallPtrSet<PtrType> {
  pub fn new() -> Self {
    SmallPtrSet { dummy: None }
  }
  pub fn empty() {}
  pub fn size() {}
  pub fn clear() {}

  pub fn insert(&self, _ptr: PtrType) {}
  pub fn erase(&self, _ptr: PtrType) {}
  
  pub fn count() {}
  pub fn find() {}
  pub fn contains() {}
}