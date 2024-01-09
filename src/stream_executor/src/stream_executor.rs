#![allow(dead_code)]

use crate::platform::Platform;

pub struct StreamExecutor {
  platform: Box<dyn Platform>,
  device_ordinal: i64,
}

impl StreamExecutor {
  pub fn new() {}

  pub fn platform(&self) -> &dyn Platform {
    self.platform.as_ref()
  }

  pub fn device_ordinal(&self) -> i64 {
    self.device_ordinal
  }
}