#![allow(dead_code)]
pub struct BlitzEnv {
  is_stopped: bool,
  driver_tmp_dir: Option<String>,
}

impl BlitzEnv {
  pub fn new() {}
  pub fn stop() {}

  pub fn create_pythin_worker() {}
  pub fn destroy_python_worker() {}
  pub fn release_python_worker() {}
}