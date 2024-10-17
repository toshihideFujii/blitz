#![allow(dead_code)]

// The "client library" instantiates a local (in-process) Blitz service for
// use by this process, and connects to it with a singleton Blitz local
// client. ClientLibrary::GetOrCreateLocalClient will spawn a local service,
// and return a client that's connected to it and ready to run Blitz
// computations.

use std::collections::HashSet;

use stream_executor::platform::Platform;

// Options to configure the local client when it is created.
pub struct LocalClientOptions {
  platform: Box<dyn Platform>,
  number_of_replicas: i64,
  intra_op_parallelism_threads: i64,
  allowed_devices: Option<HashSet<i64>>,
}

impl LocalClientOptions {
  pub fn new(
    platform: Box<dyn Platform>,
    number_of_replicas: i64,
    intra_op_parallelism_threads: i64,
    allowed_devices: Option<HashSet<i64>>) -> Self
  {
    LocalClientOptions {
      platform: platform,
      number_of_replicas: number_of_replicas,
      intra_op_parallelism_threads: intra_op_parallelism_threads,
      allowed_devices: allowed_devices
    }
  }

  // Set the platform backing the service, or nullptr for the default platform.
  pub fn set_platform(&mut self, platform: Box<dyn Platform>) -> &mut Self {
    self.platform = platform;
    self
  }

  pub fn platform(&self) -> &dyn Platform {
    self.platform.as_ref()
  }

  // Set the number of replicas to use when compiling replicated programs.
  pub fn set_number_of_replicas(&mut self, number_of_replicas: i64) -> &mut Self {
    self.number_of_replicas = number_of_replicas;
    self
  }

  pub fn number_of_replicas(&self) -> i64 {
    self.number_of_replicas
  }

  // Sets the thread pool size for parallel execution of an individual operator.
  pub fn set_intra_op_parallelism_threads(&mut self, num_threads: i64) -> &mut Self {
    self.intra_op_parallelism_threads = num_threads;
    self
  }

  pub fn intra_op_parallelism_threads(&self) -> i64 {
    self.intra_op_parallelism_threads
  }

  // Sets the allowed_devices set for selectively constructing stream executors
  // on the platform.
  pub fn set_allowed_devices(&mut self, allowed_devices: Option<HashSet<i64>>) -> &mut Self {
    self.allowed_devices = allowed_devices;
    self
  }

  pub fn allowed_devices(&self) -> &Option<HashSet<i64>> {
    &self.allowed_devices
  }
}

pub struct ClientLibrary {}

impl ClientLibrary {
  pub fn new() {}
  pub fn get_or_create_local_client() {}
  pub fn local_client_or_die() {}
  pub fn get_blitz_service() {}
  pub fn get_or_create_compile_only_client() {}
  pub fn destroy_local_instances() {}
}