#![allow(dead_code)]

use std::collections::HashSet;

use hlo::hlo_module::HloModule;

// This pass finds copies between the host memory and device memory and
// converts them into the async ops.
pub struct HostMemoryTransferAsyncifier {
  host_memory_space_color: i64
}

impl HostMemoryTransferAsyncifier {
  pub fn new(host_memory_space_color: i64) -> Self {
    HostMemoryTransferAsyncifier {
      host_memory_space_color: host_memory_space_color
    }
  }

  pub fn name() -> String {
    "host-memory-transfer-asyncifier".to_string()
  }

  pub fn run(
    &self,
    _module: &HloModule,
    _execution_threads: HashSet<String>) -> Result<bool, String>
  {
    unimplemented!()
  }
}