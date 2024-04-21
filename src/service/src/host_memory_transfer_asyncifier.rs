#![allow(dead_code)]

// This pass finds copies between the host memory and device memory and
// converts them into the async ops.
pub struct HostMemoryTransferAsyncifier {}

impl HostMemoryTransferAsyncifier {
  pub fn new() {}
  pub fn name() -> String { "host-memory-transfer-asyncifier".to_string() }
  pub fn run() {}
}