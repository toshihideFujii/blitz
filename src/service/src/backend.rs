#![allow(dead_code)]

use std::collections::HashSet;

use stream_executor::{device_memory_allocator::StreamExecutorMemoryAllocator, platform::Platform, stream_executor::StreamExecutor};

use crate::{compiler::Compiler, computation_placer::ComputationPlacer, transfer_manager::TransferManager};

// Options to configure the backend when it is created.
pub struct BackendOptions {
  platform: Option<Platform>,
  intra_op_parallelism_threads: i64,
  allowed_devices: Option<HashSet<i64>>
}

impl BackendOptions {
  pub fn new() -> Self {
    BackendOptions {
      platform: None,
      intra_op_parallelism_threads: 0,
      allowed_devices: None
    }
  }

  // Set the platform backing the backend, or nullptr for the default platform.
  pub fn set_platform(&mut self, platform: Platform) -> &mut Self {
    self.platform = Some(platform);
    self
  }

  pub fn platform(&self) -> &Option<Platform> {
    &self.platform
  }

  // Sets the thread pool size for parallel execution of an individual operator.
  // The default value of -1 will result in initializing the thread pool with
  // the number of threads equal to the number of cores in the system.
  pub fn set_intra_op_parallelism_threads(&mut self, num_threads: i64) -> &mut Self {
    self.intra_op_parallelism_threads = num_threads;
    self
  }

  pub fn intra_op_parallelism_threads(&self) -> i64 {
    self.intra_op_parallelism_threads
  }

  // Sets the allowed_devices for selectively constructing stream executors
  // on the platform.
  pub fn set_allowed_devices(&mut self, allowed_devices: HashSet<i64>) -> &mut Self {
    self.allowed_devices = Some(allowed_devices);
    self
  }

  pub fn allowed_devices(&self) -> &Option<HashSet<i64>> {
    &self.allowed_devices
  }
}

// Class which encapsulates an Blitz backend. It includes everything necessary
// to compile and execute computations on a particular platform.
//
// It also offers a pooling API for creation/use of initialized streams:
//
//    StreamPool::Ptr stream = backend->BorrowStream().value();
pub struct Backend {
  platform: Platform,
  compiler: Compiler,
  transfer_manager: TransferManager,
  computation_placer: ComputationPlacer,
  stream_executors: Vec<StreamExecutor>,
  memory_allocator: StreamExecutorMemoryAllocator,
}

impl Backend {
  pub fn new() {}

  // Accessors for the various objects.
  pub fn platform(&self) -> &Platform {
    &self.platform
  }

  pub fn compiler(&self) -> &Compiler {
    &self.compiler
  }

  pub fn memory_allocator() {}
  pub fn shared_memory_allocator() {}

  pub fn transfer_manager(&self) -> &TransferManager {
    &self.transfer_manager
  }

  pub fn computation_placer(&self) -> &ComputationPlacer {
    &self.computation_placer
  }

  // Returns the number of devices of the platform type which are visible. Not
  // all of these devices may be usable by Blitz.
  pub fn device_count(&self) -> usize {
    self.stream_executors.len()
  }

  pub fn default_device_ordinal() {}

  // Returns stream executors of all supported devices for this backend. The
  // executors are ordered by the device ordinal.
  pub fn stream_executors(&self) -> &Vec<StreamExecutor> {
    &self.stream_executors
  }

  pub fn stream_executor() {}

  // Returns the stream executor for the default device ordinal. This stream
  // executor can only be used when the number of computations is 1 (replication
  // can be > 1).
  pub fn default_stream_executor(&self) -> &StreamExecutor{
    assert!(!self.stream_executors.is_empty());
    &self.stream_executors[0]
  }

  pub fn borrow_stream() {}
  pub fn stream_borrower_with_priority() {}
  pub fn device_ordinal_supported() {}
  pub fn device_name() {}
  pub fn devices_equivalent() {}
  pub fn eigen_intra_op_thread_pool_device() {}
  pub fn eigen_intra_op_thread_pool() {}
  pub fn reset_devices() {}
}