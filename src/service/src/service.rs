#![allow(dead_code)]

use std::collections::HashSet;

use common::{blitz_data::{
  ChannelHandle, ChannelType,
  ComputationGraphStatsRequest, ComputationStatsResponse, ComputeConstantGraphRequest,
  ComputeConstantResponse, DeviceHandle, ExecuteGtaphParallelRequest,
  ExecuteParallelResponse, ExecutionHandle, ExecutionOptions,
  ExecutionProfile, GlobalDataHandle
}, literal::Literal, shape::Shape};

use stream_executor::platform::Platform;

use crate::{
  allocation_tracker::AllocationTracker, backend::Backend, channel_tracker::ChannelTracker, compilation_cache::CompilationCache, execution_tracker::ExecutionTracker
};

// Options to configure the service when it is created.
pub struct ServiceOptions {
  platform: Option<Box<dyn Platform>>,
  number_of_replicas: i64,
  intra_op_parallelism_threads: i64,
  allowed_devices: Option<HashSet<i64>>
}

impl ServiceOptions {
  pub fn new() -> Self {
    ServiceOptions {
      platform: None,
      number_of_replicas: 1,
      intra_op_parallelism_threads: -1,
      allowed_devices: None
    }
  }

  // Set the platform backing the service, or nullptr for the default platform.
  pub fn set_platform(&mut self, platform: Box<dyn Platform>) -> &mut Self {
    self.platform = Some(platform);
    self
  }

  pub fn platform(&self) -> &Option<Box<dyn Platform>> {
    &self.platform
  }

  // Set the default number of replicas to use when compiling replicated
  // programs.
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
  pub fn set_allowed_devices(&mut self, allowed_devices: HashSet<i64>) -> &mut Self {
    self.allowed_devices = Some(allowed_devices);
    self
  }

  pub fn allowed_devices(&self) -> &Option<HashSet<i64>> {
    &self.allowed_devices
  }
}

// The Blitz service object, which is the same across all platforms. It maintains
// the service state of computations and allocations, and delegates
// target-specific requests to the target-specific infrastructure
// (target-specific compiler, StreamExecutor).
pub struct Service {
  options: ServiceOptions,
  compilation_cache: CompilationCache,
  channel_tracker: ChannelTracker,
  allocation_tracker: AllocationTracker,
  execution_tracker: ExecutionTracker,
  // Backend to compile and execute computations on.
  execute_backend: Backend,
}

impl Service {
  pub fn new() {}

  // Unregisters a previously-allocated global handle.
  //
  // If the handle given is not currently allocated, a NOT_FOUND status is
  // returned.
  pub fn unregister(&self, _data: &GlobalDataHandle) -> Result<(), String> {
    unimplemented!()
  }

  // Deconstructs a tuple. Returns a newly created GlobalDataHandle for each
  // element in the tuple.
  pub fn deconstruct_tuple(&self, _data: &GlobalData) -> Result<Vec<GlobalData>, String> {
    unimplemented!()
  }

  // Compiles a computation into an executable. The request contains the whole
  // computation graph. Returns the handle to the executable.
  pub fn compile(
    &self,
    //computation: &BlitzComputation,s
    _argument_shapes: &Vec<Shape>,
    _execution_options: ExecutionOptions) -> Result<ExecutionHandle, String>
  {
    unimplemented!()
  }

  // Executes an executable with the provided global data passes as immutable
  // arguments. The request contains the handle to the executable. Returns
  // global data output and execution timing.
  pub fn execute(
    &self,
    _handle: &ExecutionHandle,
    _arguments: &Vec<GlobalData>,
    _execution_profile: Option<ExecutionProfile>) -> Result<GlobalData, String>
  {
    unimplemented!()
  }

  // Executes one or more computations in parallel with the provided global data
  // passed as immutable arguments. Returns global data output for each
  // computation.
  pub fn execute_graph_parallel(
    &self,
    _arg: &ExecuteGtaphParallelRequest,
    _result: &ExecuteParallelResponse) -> Result<(), String>
  {
    unimplemented!()
  }

  // Requests one or more device handles from the target.
  //
  // When N device handles are requested and the number of replicas is R, at
  // least N * R devices must be available. The devices are assigned based on
  // the device ordinals such that the first R available devices are assigned to
  // the first set of replicas, and the next R devices to the second set of
  // replicas, etc. Each returned device handle represents the device with the
  // replica id 0.
  pub fn get_device_handles(&self, _device_count: i64) -> Result<Vec<DeviceHandle>, String> {
    unimplemented!()
  }

  // Requests that global data be transferred to the client in literal form.
  pub fn transfer_to_client<T>(
    &self,
    _data: &GlobalData,
    _shape_with_layout: Option<Shape>) -> Result<Literal<T>, String>
    where T: Clone + Default + PartialEq
  {
    unimplemented!()
  }

  // Transfers data from a literal provided by the client, into device memory.
  pub fn transfer_to_server<T>(
    &self,
    _literal: &Literal<T>,
    _device_handle: Option<DeviceHandle>) -> Result<GlobalData, String>
    where T: Clone + Default + PartialEq
  {
    unimplemented!()
  }

  // Transfers data from a literal provided by the client, into the Infeed
  // buffer of the device.
  pub fn transfer_to_infeed<T>(
    &self,
    _literal: &Literal<T>,
    _replica_id: i64,
    _device_handle: Option<DeviceHandle>) -> Result<(), String>
    where T: Clone + Default + PartialEq
  {
    unimplemented!()
  }

  // Transfers data from the Outfeed othe device to the literal provided by the
  // client.
  pub fn transfer_from_outfeed<T>(
    &self,
    _shape_with_layout: &Shape,
    _replica_id: i64,
    _device_handle: Option<DeviceHandle>) -> Result<Literal<T>, String>
    where T: Clone + Default + PartialEq
  {
    unimplemented!()
  }

  // Resets devices, clearing all existing state on all the devices associated
  // with this service (including memory allocated on the devices).
  //
  // ResetDevice may only be called where no previous Execution state on the
  // device is used by the next Execution.
  //
  // ResetDevice should be called before an Execution that expect the device to
  // be in the reset state. For example, if the prior Execution modifies device
  // state (e.g., architectural state) that the next Execution depends on.
  pub fn reset_device(&self) -> Result<(), String> {
    unimplemented!()
  }

  pub fn compute_constant_graph(
    &self,
    _arg: &ComputeConstantGraphRequest,
    _result: &ComputeConstantResponse) -> Result<(), String>
  {
    unimplemented!()
  }

  // Returns the shape (with layout) of an array associated with a given data
  // handle.
  pub fn get_shape(&self, _data: &GlobalData) -> Result<Shape, String> {
    unimplemented!()
  }

  // Retrieves the statistics of a computation.
  pub fn get_computation_graph_stats(
    &self,
    _arg: &ComputationGraphStatsRequest,
    _result: &ComputationStatsResponse) -> Result<(), String>
  {
    unimplemented!()
  }

  // Creates a unique channel handle that can be used for Send/Recv
  // instructions.
  pub fn create_channel_handle(&self, _t: ChannelType) -> Result<ChannelHandle, String> {
    unimplemented!()
  }

  pub fn backend(&self) -> &Backend {
    &self.execute_backend
  }

  pub fn mutable_backend() {}
  pub fn create_module_config() {}
  pub fn validate_result_shape() {}
}

// A GlobalData object represents a globally-accessible allocation of
// data in the associated Blitz service.
pub struct GlobalData {
  handle: GlobalDataHandle,
  parent: Service
}

impl GlobalData {
  pub fn new() {}

  pub fn handle(&self) -> &GlobalDataHandle {
    &self.handle
  }
}

// A struct to represent a computation instance to be executed.
// * If execution_options.device_handles is not empty, the computation is
//   executed on the devices associated with the handles by partitioning the
//   computation based on the attached sharding attributes. Otherwise, a
//   device is chosen by the service.
pub struct BlitzComputationInstance {
  
}