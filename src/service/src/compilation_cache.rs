#![allow(dead_code)]

use std::collections::HashMap;
use common::blitz_data::ExecutionHandle;
use crate::executable::Executable;

// A cache which stores Executables indexed by computation handle and version.
pub struct CompilationCache {
  cache: HashMap<i64, Executable>
}

impl CompilationCache {
  pub fn default() -> Self {
    CompilationCache {
      cache: HashMap::new()
    }
  }

  pub fn insert(&mut self, executable: Executable) -> ExecutionHandle {
    let key = get_unique_id();
    println!("inserting cache key: {:?}", key);

    assert!(!self.cache.contains_key(&key));
    self.cache.insert(key, executable);

    let mut handle = ExecutionHandle::default();
    handle.set_handle(key);
    handle
  }

  // Lookup the Executable for the specified handle in the cache. Return a
  // shared_ptr to the Executable if it exists in the cache.
  pub fn lookup(&self, handle: &ExecutionHandle) -> Result<&Executable, String> {
    let key = handle.handle();
    println!("looking up cache key: {:?}", key);

    if self.cache.contains_key(&key) {
      let result = self.cache.get(&key);
      println!("hit executable: {:?}", result.as_ref().unwrap().module().name());
      Ok(result.as_ref().unwrap())
    } else {
      println!("cache key not found: {:?}", key);
      let mut err_msg = "can not find executable with handle ".to_string();
      err_msg.push_str(&key.to_string());
      Err(err_msg)
    }
  }
}

static mut COUNTER: i64 = 0;

fn get_unique_id() -> i64 {
  #[allow(unused_assignments)]
  let mut id = 0;
  unsafe {
    id = COUNTER;
    COUNTER += 1;
  }
  id
}