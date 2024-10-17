#![allow(dead_code)]

use crate::{
  kernel::{Kernel, KernelMetadata},
  launch_dim::ThreadDim,
};

pub struct HostKernel {
  name: String,
  metadata: KernelMetadata,
  arity: usize,
}

impl HostKernel {
  pub fn new() {}

  pub fn set_arity(&mut self, arity: usize) {
    self.arity = arity;
  }
}

impl Kernel for HostKernel {
  fn args_packing(&self) {
    unimplemented!()
  }

  fn arity(&self) -> usize {
    self.arity
  }

  fn get_max_occupied_blocks_per_core(
    &self,
    _threads: &ThreadDim,
    _dynamic_shared_memory_bytes: usize) -> Result<i64, String>
  {
    unimplemented!()
  }

  fn metadata(&self) -> &KernelMetadata {
    &self.metadata
  }

  fn name(&self) -> &String {
    &self.name
  }

  fn set_args_packing(&mut self) {
    unimplemented!()
  }

  fn set_metadata(&mut self, metadata: KernelMetadata) {
    self.metadata = metadata;
  }

  fn set_name(&mut self, name: String) {
    self.name = name;
  }
}