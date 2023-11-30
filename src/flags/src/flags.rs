#![allow(dead_code)]

pub struct FlagValue<T> {
  value: T
}

impl<T> FlagValue<T> {
  pub fn new(value: T) -> Self {
    FlagValue { value: value }
  }

  pub fn value(&self) -> &T {
    &self.value
  }
}

pub struct FlagValues {
  pub minor_ms: bool,
  pub minor_ms_max_new_space_capacity_mb: usize,
  pub scavenger_max_new_space_capacity_mb: usize,
}

pub static BLITZ_FLAGS: FlagValues =
  FlagValues {
    minor_ms: false,
    minor_ms_max_new_space_capacity_mb: 72,
    scavenger_max_new_space_capacity_mb: 8,
  };