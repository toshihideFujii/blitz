#![allow(dead_code)]

// This class implements a set which is optimized for holding
// SmallSize or less elements.
#[derive(Debug, Clone, PartialEq)]
pub struct SmallPtrSet<PtrType> {
  dummy: PtrType
}
impl<PtrType> SmallPtrSet<PtrType> {
  pub fn new() {}
  pub fn empty() {}
  pub fn size() {}
  pub fn clear() {}
  pub fn insert() {}
  pub fn erase() {}
  pub fn count() {}
  pub fn find() {}
  pub fn contains() {}
}