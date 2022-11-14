#![allow(dead_code)]

/*
This header enumerates high-level optimization levels.
Each level has a specific goal and rationale.
*/

#[derive(PartialEq, Eq)]
struct OptimizationLevel {
  speed_level: u32,
  size_level: u32
}

impl OptimizationLevel {
  pub fn new(speed_level: u32, size_level: u32) -> Self {
    OptimizationLevel { speed_level: speed_level, size_level: size_level }
  }

  pub fn is_optimizing_for_speed(&self) -> bool {
    self.size_level == 0 && self.speed_level > 0
  }

  pub fn is_optimizing_for_size(&self) -> bool {
    self.size_level > 0
  }

  pub fn get_speed_up_level(&self) -> u32 {
    self.speed_level
  }

  pub fn get_size_level(&self) -> u32 {
    self.size_level
  }
}