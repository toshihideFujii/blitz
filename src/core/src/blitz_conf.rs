#![allow(dead_code)]

use std::{sync::Mutex, collections::HashMap};

pub struct BlitzConf {
  settings: Mutex<HashMap<String, String>>
}

impl BlitzConf {
  fn set(&mut self, k: String, v: String) {
    self.set_silent(k, v, false);
  }

  fn set_silent(&mut self, k: String, v: String, silent: bool) {
    if !silent {
      // log
    }
    self.settings.lock().unwrap().insert(k, v);
  }

  pub fn set_master(&mut self, master: String) {
    self.set("blitz.master".to_string(), master);
  }

  pub fn set_app_name(&mut self, name: String) {
    self.set("blitz.app.name".to_string(), name);
  }

  pub fn set_executor_env(&mut self, variable: String, value: String) {
    self.set("blitz.exeutor_env".to_string() + variable.as_str(), value);
  }

  pub fn set_blitz_home(&mut self, home: String) {
    self.set("blitz.home".to_string(), home);
  }

  pub fn remove(&mut self, k: String) {
    self.settings.lock().unwrap().remove(&k);
  }

  pub fn get(&self, _k: String) -> Option<&String> {
    //self.settings.lock().unwrap().get(&k)
    None
  }

  pub fn get_time_as_seconds() {}
  pub fn get_time_as_ms() {}
  pub fn get_size_as_bytes() {}
  pub fn get_size_as_kb() {}
  pub fn get_size_as_mb() {}
  pub fn get_size_as_gb() {}

  pub fn get_all() {}
  pub fn get_app_id() {}
  pub fn contains(&self) -> bool {
    false
  }
  pub fn validate_settings() {}
}