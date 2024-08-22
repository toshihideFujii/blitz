#![allow(dead_code)]

use common::blitz_data::ExecutionHandle;

use crate::executable::Executable;

// A cache which stores Executables indexed by computation handle and version.
pub struct CompilationCache {}

impl CompilationCache {
  pub fn new() -> Self {
    CompilationCache {  }
  }

  pub fn insert(&mut self, _executable: &Executable) -> &ExecutionHandle {
    unimplemented!()
  }

  // Lookup the Executable for the specified handle in the cache. Return a
  // shared_ptr to the Executable if it exists in the cache.
  pub fn lookup(&self, _handle: &ExecutionHandle) -> Result<Executable, String> {
    unimplemented!()
  }
}