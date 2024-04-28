#![allow(dead_code)]

pub struct ModuleHandle {}

// Describes how to load a module on a target platform.
pub struct MultiModuleLoaderSpec {
  cuda_cubin_in_memory: Vec<u8>,
  cuda_ptx_in_memory: Vec<u8>,
}

impl MultiModuleLoaderSpec {
  pub fn new() {}

  pub fn has_cuda_cubin_in_memory(&self) -> bool {
    !self.cuda_cubin_in_memory.is_empty()
  }

  pub fn cuda_cubin_in_memory(&self) -> &Vec<u8> {
    &self.cuda_cubin_in_memory
  }

  pub fn has_cuda_ptx_in_memory() {}
  pub fn cuda_ptx_in_memory() {}

  pub fn add_cuda_cubin_in_memory(&mut self, cubin_bytes: Vec<u8>) {
    assert!(!cubin_bytes.is_empty());
    self.cuda_cubin_in_memory = cubin_bytes;
  }

  pub fn add_cuda_ptx_in_memory() {}
}