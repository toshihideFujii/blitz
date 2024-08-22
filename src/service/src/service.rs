#![allow(dead_code)]

use std::collections::HashSet;

use common::blitz_data::{
  CompileRequest, CompileResponse, ComputationGraphStatsRequest,
  ComputationStatsResponse, ComputeConstantGraphRequest, ComputeConstantResponse,
  CreateChannelHandleRequest, CreateChannelHandleResponse, DeconstructTupleRequest,
  DeconstructTupleResponse, ExecuteGtaphParallelRequest, ExecuteParallelResponse,
  ExecuteRequest, ExecuteResponse, GetDeviceHandlesRequest, GetDeviceHandlesResponse,
  GetShapeRequest, GetShapeResponse, ResetDeviceRequest, ResetDeviceResponse,
  TransferFromOutfeedRequest, TransferFromOutfeedResponse, TransferToClientRequest,
  TransferToClientResponse, TransferToInfeedRequest, TransferToInfeedResponse,
  TransferToServerRequest, TransferToServerResponse, UnregisterRequest, UnregisterResponse
};
use stream_executor::platform::Platform;

use crate::{allocation_tracker::AllocationTracker, channel_tracker::ChannelTracker, compilation_cache::CompilationCache, execution_tracker::ExecutionTracker};

// Options to configure the service when it is created.
pub struct ServiceOptions {
  platform: Option<Platform>,
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
  pub fn set_platform(&mut self, platform: Platform) -> &mut Self {
    self.platform = Some(platform);
    self
  }

  pub fn platform(&self) -> &Option<Platform> {
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
  execution_tracker: ExecutionTracker
}

impl Service {
  pub fn new() {}

  // Unregisters a previously-allocated global handle.
  //
  // If the handle given is not currently allocated, a NOT_FOUND status is
  // returned.
  pub fn unregister(
    &self,
    _arg: &UnregisterRequest,
    _result: &UnregisterResponse) -> Result<(), String>
  {
    unimplemented!()
  }

  // Deconstructs a tuple. Returns a newly created GlobalDataHandle for each
  // element in the tuple.
  pub fn deconstruct_tuple(
    &self,
    _arg: &DeconstructTupleRequest,
    _result: &DeconstructTupleResponse) -> Result<(), String>
  {
    unimplemented!()
  }

  // Compiles a computation into an executable. The request contains the whole
  // computation graph. Returns the handle to the executable.
  pub fn compile(
    &self,
    _arg: &CompileRequest,
    _result: &CompileResponse) -> Result<(), String>
  {
    unimplemented!()
  }

  // Executes an executable with the provided global data passes as immutable
  // arguments. The request contains the handle to the executable. Returns
  // global data output and execution timing.
  pub fn execute(
    &self,
    _arg: &ExecuteRequest,
    _result: &ExecuteResponse) -> Result<(), String>
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
  pub fn get_device_handles(
    &self,
    _arg: &GetDeviceHandlesRequest,
    _result: &GetDeviceHandlesResponse) -> Result<(), String>
  {
    unimplemented!()
  }

  // Requests that global data be transferred to the client in literal form.
  pub fn transfer_to_client(
    &self,
    _arg: &TransferToClientRequest,
    _result: &TransferToClientResponse)
  {
    unimplemented!()
  }

  // Transfers data from a literal provided by the client, into device memory.
  pub fn transfer_to_server(
    &self,
    _arg: &TransferToServerRequest,
    _result: &TransferToServerResponse) -> Result<(), String>
  {
    unimplemented!()
  }

  // Transfers data from a literal provided by the client, into the Infeed
  // buffer of the device.
  pub fn transfer_to_infeed(
    &self,
    _arg: &TransferToInfeedRequest,
    _result: &TransferToInfeedResponse) -> Result<(), String>
  {
    unimplemented!()
  }

  // Transfers data from the Outfeed othe device to the literal provided by the
  // client.
  pub fn transfer_from_outfeed(
    &self,
    _arg: &TransferFromOutfeedRequest,
    _result: &TransferFromOutfeedResponse) -> Result<(), String>
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
  pub fn reset_device(
    &self,
    _arg: &ResetDeviceRequest,
    _result: &ResetDeviceResponse) -> Result<(), String>
  {
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
  pub fn get_shape(
    &self, _arg: &GetShapeRequest, _result: &GetShapeResponse) -> Result<(), String>
  {
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
  pub fn create_channel_handle(
    &self,
    _arg: &CreateChannelHandleRequest,
    _result: &CreateChannelHandleResponse) -> Result<(), String>
  {
    unimplemented!()
  }

  pub fn backend() {}
  pub fn mutable_backend() {}
  pub fn create_module_config() {}
  pub fn validate_result_shape() {}
}