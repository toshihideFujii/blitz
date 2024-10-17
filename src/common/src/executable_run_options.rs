#![allow(dead_code)]

use std::collections::HashMap;

use stream_executor::{device_memory::DeviceMemoryBase, device_memory_allocator::DeviceMemoryAllocator, stream::Stream};

use crate::{blitz_data::ExecutionProfile, shape::Shape};

// A unique identifier for a particular "logical execution" of an Blitz model.
//
// A logical execution might encompass multiple executions of one or more
// HloModules.  Runs that are part of the same logical execution can
// communicate via collective ops (e.g. kAllToAll), whereas runs that are part
// of different logical executions are isolated.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RunId {
  data: i64,
}

impl RunId {
  pub fn new(value: i64) -> Self {
    RunId {
      data: value
    }
  }

  pub fn to_string(&self) -> String {
    let mut result = String::from("RunId: ");
    result.push_str(self.data.to_string().as_str());
    result
  }

  pub fn to_int(&self) -> i64 {
    self.data
  }
}

// Callback used by the GPU backend only. This is an "one-sided" version of
// ThenDoHostCallback that enqueues a callback onto a stream. The difference
// with ThenDoHostCallback is that the device does not block waiting for the
// callback to complete; instead the callback is scheduled by the runtime.
// This functionality must be provided by the caller, and hence is provided in
// callback form.
type ThenExecutionDunction = dyn Fn(&dyn Stream, &dyn Fn());

// Callback for sending device buffer to a channel. Returned event will be
// recorded on a `stream` once the send operation is completed and data was
// copied from the `src` memory. `frontend_attrs` contains frontend specific
// attributes for the send.
type SendDeviceMemoryFunction = dyn Fn(i64, &dyn Stream, &Shape);

// Callback for receiving device buffer from a channel. Returned event will be
// recorded on a `stream` once the recv operation is completed and data was
// copied into the `dst` memory. `frontend_attrs` contains frontend specific
// attributes for the receive.
type RecvDeviceMemoryFunction =
  dyn Fn(i64, &dyn Stream, &Shape, &DeviceMemoryBase, &HashMap<String, String>);

// Class containing options for running a LocalExecutable.
pub struct ExecutableRunOptions {
  allocator: Option<DeviceMemoryAllocator>,
  device_ordinal: i64,
  stream: Option<Box<dyn Stream>>,
  rng_seed: i64,
  launch_id: i64,
  device_to_host_stream: Option<Box<dyn Stream>>,
  host_to_device_stream: Option<Box<dyn Stream>>,
  run_id: RunId,
  execution_profile: ExecutionProfile,
  then_execute_function: Option<Box<ThenExecutionDunction>>,
  send_device_memory_function: Option<Box<SendDeviceMemoryFunction>>,
  recv_device_memory_function: Option<Box<RecvDeviceMemoryFunction>>,
  local_device_count: i64,
}

impl ExecutableRunOptions {
  pub fn new() -> Self {
    ExecutableRunOptions {
      allocator: None,
      device_ordinal: 0,
      stream: None,
      rng_seed: 0,
      launch_id: 0,
      device_to_host_stream: None,
      host_to_device_stream: None,
      run_id: RunId::new(0),
      execution_profile: ExecutionProfile::new(),
      then_execute_function: None,
      send_device_memory_function: None,
      recv_device_memory_function: None,
      local_device_count: 0,
    }
  }

  // Specifies the allocator to use during execution.
  pub fn set_allocator(&mut self, allocator: DeviceMemoryAllocator) -> &mut Self {
    self.allocator = Some(allocator);
    self
  }

  pub fn allocator(&self) -> &DeviceMemoryAllocator {
    self.allocator.as_ref().unwrap()
  }

  // If set, this is the device to run the computation on. Valid device_ordinal
  // values are: 0 to # of devices - 1. These values are identical to the device
  // ordinal values used by StreamExecutor. The device must be of the same type
  // as the executable was compiled for. A value of -1 indicates this option has
  // not been set.
  pub fn set_device_ordinal(&mut self, device_ordinal: i64) -> &mut Self {
    self.device_ordinal = device_ordinal;
    self
  }

  pub fn device_ordinal(&self) -> i64 {
    self.device_ordinal
  }

  // If set, this is the stream to run the computation on. The platform of the
  // stream must match the platform the executable was built for.  A value of
  // nullptr indicates the option has not been set.
  pub fn set_stream(&mut self, stream: Box<dyn Stream>) -> &mut Self {
    self.stream = Some(stream);
    self
  }

  pub fn stream(&self) -> &Option<Box<dyn Stream>> {
    &self.stream
  }

  // If set, this is the stream to perform host to device transfers on (e.g. any
  // pre-computation transfers). The platform of the stream must match the
  // platform the executable was built for. A value of nullptr indicates the
  // option has not been set.
  pub fn set_host_to_device_stream(&mut self, stream: Box<dyn Stream>) -> &mut Self {
    self.host_to_device_stream = Some(stream);
    self
  }

  pub fn host_to_device_stream(&self) -> &Option<Box<dyn Stream>> {
    &self.host_to_device_stream
  }

  // If set, this is the stream to perform device to host transfers on.
  // The platform of the stream must match the platform the executable was
  // built for. A value of nullptr indicates the option has not been set.
  pub fn set_device_to_host_stream(&mut self, stream: Box<dyn Stream>) -> &mut Self {
    self.device_to_host_stream = Some(stream);
    self
  }

  pub fn device_to_host_stream(&self) -> &Option<Box<dyn Stream>> {
    &self.device_to_host_stream
  }

  // Sets the thread pool device on which to run Eigen subcomputations.
  //
  // This field must be set for XLA:CPU models that call Eigen routines, but may
  // be null otherwise.  Routines that use this field should always CHECK (or
  // TF_RET_CHECK) that it's not null before dereferencing it, so that users get
  // a clean crash rather than a segfault.
  //
  // Does not take ownership.
  pub fn set_intra_op_thread_pool() {}
  pub fn intra_op_thread_pool() {}

  // If set, profiling information is written to 'profile'.
  pub fn set_execution_profile(&mut self, profile: ExecutionProfile) -> &mut Self {
    self.execution_profile = profile;
    self
  }

  pub fn execution_profile(&self) -> &ExecutionProfile {
    &self.execution_profile
  }

  pub fn set_device_assignment() {}
  pub fn device_assignment() {}

  pub fn set_rng_seed(&mut self, rng_seed: i64) -> &mut ExecutableRunOptions {
    self.rng_seed = rng_seed;
    self
  }

  pub fn rng_seed(&self) -> i64 {
    self.rng_seed
  }

  pub fn set_launch_id(&mut self, launch_id: i64) -> &mut ExecutableRunOptions {
    self.launch_id = launch_id;
    self
  }

  pub fn launch_id(&self) -> i64 {
    self.launch_id
  }

  pub fn set_run_id(&mut self, run_id: RunId) -> &mut ExecutableRunOptions {
    self.run_id = run_id;
    self
  }

  pub fn run_id(&self) -> RunId {
    self.run_id.clone()
  }

  // See documentation on ThenExecuteFunction.
  pub fn set_then_executable_function(
    &mut self, f: Box<ThenExecutionDunction>) -> &mut Self
  {
    self.then_execute_function = Some(f);
    self
  }

  pub fn then_executable_function(&self) -> &Option<Box<ThenExecutionDunction>> {
    &self.then_execute_function
  }

  // See documentation on SendDeviceMemoryFunction.
  pub fn set_send_device_memory_function(
    &mut self, f: Box<SendDeviceMemoryFunction>) -> &mut Self
  {
    self.send_device_memory_function = Some(f);
    self
  }

  pub fn send_device_memory_function(&self) -> &Option<Box<SendDeviceMemoryFunction>> {
    &self.send_device_memory_function
  }

  // See documentation on RecvDeviceMemoryFunction.
  pub fn set_recv_device_memory_function(
    &mut self, f: Box<RecvDeviceMemoryFunction>) -> &mut Self
  {
    self.recv_device_memory_function = Some(f);
    self
  }

  pub fn recv_device_memory_function(&self) -> &Option<Box<RecvDeviceMemoryFunction>> {
    &self.recv_device_memory_function
  }

  pub fn set_cpu_executable_run_options() {}
  pub fn cpu_executable_run_options() {}

  pub fn set_gpu_executable_run_options() {}
  pub fn gpu_executable_run_options() {}

  pub fn set_local_device_count(&mut self, local_device_counnt: i64) -> &mut Self {
    self.local_device_count = local_device_counnt;
    self
  }

  pub fn local_device_counnt(&self) -> i64 {
    self.local_device_count
  }
}