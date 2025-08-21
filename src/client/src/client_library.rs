#![allow(dead_code)]

// The "client library" instantiates a local (in-process) Blitz service for
// use by this process, and connects to it with a singleton Blitz local
// client. ClientLibrary::GetOrCreateLocalClient will spawn a local service,
// and return a client that's connected to it and ready to run Blitz
// computations.

use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use service::compile_only_service::CompileOnlyService;
use service::local_service::LocalService;
use stream_executor::platform::Platform;

use crate::compile_only_client::CompileOnlyClient;
use crate::local_client::LocalClient;

// Options to configure the local client when it is created.
pub struct LocalClientOptions {
  platform: Platform,
  number_of_replicas: i64,
  intra_op_parallelism_threads: i64,
  allowed_devices: Option<HashSet<i64>>,
}

impl LocalClientOptions {
  pub fn new(
    platform: Platform,
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
  pub fn set_platform(&mut self, platform: Platform) -> &mut Self {
    self.platform = platform;
    self
  }

  pub fn platform(&self) -> &Platform {
    &self.platform
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

static CLIENT_LIBRARY:
  OnceLock<ClientLibrary> = OnceLock::new();

pub struct ClientLibrary<'backend> {
  local_instances: HashMap<i64, LocalInstance<'backend>>,
  compile_only_instances: HashMap<i64, CompileOnlyInstance<'backend>>
}

impl<'backend> ClientLibrary<'backend> {
  pub fn new() -> Self {
    ClientLibrary {
      local_instances: HashMap::new(),
      compile_only_instances: HashMap::new(),
    }
  }

  // Singleton constructor-or-accessor -- returns a client for the application
  // to issue Blitz commands on. Arguments:
  //
  //   platform : The platform the underlying XLA service should target. If
  //     null then default platform is used.
  //   device_set: Set of device IDs for which the stream executor will be
  //   created, for the given platform.
  pub fn get_or_create_local_client(
    platform: Option<Platform>,
    device_set: Option<HashSet<i64>>) -> Result<&'backend LocalClient<'backend>, String>
  {
    let default_options = LocalClientOptions::new(
      platform.unwrap(), 0, 0, device_set);
    
    ClientLibrary::get_or_create_local_client_by_options(default_options)
  }


  pub fn get_or_create_local_client_by_options(
    options: LocalClientOptions) -> Result<&'backend LocalClient<'backend>, String>
  {
    let platform = options.platform();
    let target =
      CLIENT_LIBRARY.get().unwrap().local_instances.get(&platform.id());
    if target.is_some() {
      return Ok(&target.unwrap().client);
    }

    //let replica_count = options.number_of_replicas();
    unimplemented!()
  }

  // Convenience "or-die" wrapper around the above which returns the existing
  // client library or creates one with default platform and allocator.
  pub fn local_client_or_die() -> &'backend LocalClient<'backend> {
    let client_status =
      ClientLibrary::get_or_create_local_client(None, None);
    check_error(&client_status);
    client_status.unwrap()
  }

  // Returns the service from the service thread. Only used in unit tests to
  // access user computations from client.
  pub fn get_blitz_service(platform: &Platform) -> &LocalService<'backend> {
    let target: Option<&LocalInstance> =
      CLIENT_LIBRARY.get().unwrap().local_instances.get(&platform.id());
    debug_assert!(target.is_some());
    &target.unwrap().service
  }

  // Singleton constructor-or-accessor for compile-only clients. Arguments:
  //
  //   platform : The platform the underlying XLA service should target. If
  //     null then default platform is used.
  pub fn get_or_create_compile_only_client(
    platform: &Platform) -> Result<&CompileOnlyClient, String>
  {
    let target =
      CLIENT_LIBRARY.get().unwrap().compile_only_instances.get(&platform.id());
    if target.is_some() {
      return Ok(&target.unwrap().client);
    }

    unimplemented!()
  }

  // Clears the local instance and compile only instance caches. The client
  // pointers returned by the previous GetOrCreateLocalClient() or
  // GetOrCreateCompileOnlyClient() invocations are not valid anymore.
  pub fn destroy_local_instances() {
    let mut client_library = ClientLibrary::new();
    client_library.local_instances.clear();
    client_library.compile_only_instances.clear();
  }
}

struct LocalInstance<'backend> {
  service: LocalService<'backend>,
  client: LocalClient<'backend>,
}

struct CompileOnlyInstance<'backend> {
  service: CompileOnlyService<'backend>,
  client: CompileOnlyClient<'backend>,
}

fn check_error<T>(value: &Result<T, String>) {
  if value.is_err() {
    let err_msg = value.as_ref().err().unwrap();
    assert!(false, "{:?}", err_msg);
  }
}