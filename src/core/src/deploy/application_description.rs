#![allow(dead_code)]

use std::fmt::{Formatter, Display, Result};

pub struct ApplicationDescription {
  name: String,
  max_cores: Option<u64>,
  app_ui_url: String,
  event_log_codec: Option<String>,
  initial_executor_limit: Option<u64>,
  user: String,
}

impl ApplicationDescription {
  pub fn new() {}
  pub fn memory_per_executor_mb() {}
  pub fn cores_per_executor() {}
  pub fn resource_reqs_per_executor() {}
}

impl Display for ApplicationDescription {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "ApplicationDescription('{}')", self.name)
  }
}