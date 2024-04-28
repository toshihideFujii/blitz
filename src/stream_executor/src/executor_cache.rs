#![allow(dead_code)]

use std::{
  collections::HashMap,
  sync::Mutex
};

use crate::{
  platform::StreamExecutorConfig,
  stream_executor::StreamExecutor
};

struct Entry {
  configurations: Mutex<Vec<(StreamExecutorConfig, StreamExecutor)>>,
}

impl Entry {
  pub fn new() {}
}

// Utility class to allow Platform objects to manage cached StreamExecutors.
pub struct ExecutorCache {
  cache: Mutex<HashMap<i64, Entry>>,
}

impl ExecutorCache {
  pub fn new() -> Self {
    ExecutorCache { cache: Mutex::new(HashMap::new()) }
  }

  pub fn get_or_create() {}

  // Returns a pointer to he described executor.
  pub fn get(&self, _config: &StreamExecutorConfig) -> Option<StreamExecutor> {
    None
  }

  // Destroys all Executors and clears the cache.
  pub fn destroy_all_executors(&mut self) {
    let mut inner = self.cache.lock().unwrap();
    inner.clear();
  }
}