#![allow(dead_code)]

// Wrapper class for abstracting references to option IDs.
#[derive(Debug, Clone)]
pub struct OptSpecifier {
  id: u32
}

impl OptSpecifier {
  pub fn new(id: u32) -> Self {
    OptSpecifier { id: id }
  }

  pub fn is_valid(&self) -> bool {
    self.id != 0
  }

  pub fn get_id(&self) -> u32 {
    self.id
  }
}