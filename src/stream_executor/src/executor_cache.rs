#![allow(dead_code)]

use std::{
  collections::HashMap, sync::Mutex,
};

use crate::{
  host::host_executor::HostExecutor,
  //platform::StreamExecutorConfig,
  //stream_executor::StreamExecutor,
};

// Utility class to allow Platform objects to manage cached StreamExecutors.
//#[derive(Debug, Clone)]
pub struct ExecutorCache {
  // Maps ordinal number to a cached executor for that ordinal.
  cache: Mutex<HashMap<i64, HostExecutor>>,
}

impl ExecutorCache {
  pub fn new() -> Self {
    ExecutorCache { cache: Mutex::new(HashMap::new()) }
  }

  // Looks up 'ordinal' in the cache. Returns a pointer to the existing
  // executor, if already present, or creates it using 'factory', if it does
  // not. Factories may be executed concurrently for different device ordinals.
  pub fn get_or_create(
    &self,
    ordinal: i64,
    factory: &mut dyn FnMut()->Result<HostExecutor, String>) -> Result<HostExecutor, String>
  {
    // In the fast path case, the cache already has an entry and we can just
    // return after Get() which only takes a shared lock and not a unique lock.
    // If we need to create, we take a unique lock on cache_.
    let fast_result = self.get(ordinal);
    if fast_result.is_ok() {
      return fast_result;
    }
    println!("building executor");
    let result = factory();
    self.cache.lock().unwrap().insert(ordinal, result.unwrap());
    self.get(ordinal)
  }

  // Returns a pointer to the described executor (if one with a matching ordinal
  // has been created), or a NOT_FOUND status.
  pub fn get(&self, ordinal: i64) -> Result<HostExecutor, String> {
    let binding = self.cache.lock().unwrap();
    let cache = binding.get(&ordinal);
    if cache.is_some() {
      return Ok(cache.unwrap().clone());
    }
    let mut err_msg = "No executors registered for ordinal ".to_string();
    err_msg.push_str(&ordinal.to_string());
    Err(err_msg)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_on_empty_cache_fails() {
    let cache = ExecutorCache::new();
    assert_eq!(cache.get(0).is_ok(), false);
  }

  #[test]
  fn test_get_returns_expected_executor() {
    let cache = ExecutorCache::new();

    let mut called = 0;
    let mut factory = || -> Result<HostExecutor, String> {
      if called == 0 {
        called += 1;
        return Ok(HostExecutor::default(0));
      } else if called == 1 {
        called += 1;
        return Ok(HostExecutor::default(1));
      } else {
        return Err("Bad call to factory.".to_string());
      }
    };

    let executor_0 = cache.get_or_create(0, &mut factory);
    assert_eq!(cache.get(0).unwrap(), executor_0.unwrap());
    let executor_1 = cache.get_or_create(1, &mut factory);
    assert_eq!(cache.get(1).unwrap(), executor_1.unwrap());
  }
}