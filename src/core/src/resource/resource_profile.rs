#![allow(dead_code)]

pub struct ResourceProfile {
  id: u64
}

impl ResourceProfile {
  pub fn new() {}
  
  pub fn id(&self) -> u64 {
    self.id
  }
}