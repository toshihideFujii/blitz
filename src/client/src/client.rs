#![allow(dead_code)]

use common::{
  blitz_data::{
    ChannelHandle, ChannelType, DeviceHandle, ExecutionHandle, ExecutionOptions, ExecutionProfile
  }, execution_options_util::create_default_execution_options,
  layout::Layout, literal::Literal, shape::{ProgramShape, Shape}
};
use service::{blitz_computation::BlitzComputation, hlo_proto::HloSnapshot, service::{GlobalData, Service}};

//use crate::blitz_computation::BlitzComputation;

// Blitz service's client object -- wraps the service with convenience and
// lifetime-oriented methods.
pub struct Client {
  stub: Service
}

impl Client {
  pub fn new(stub: Service) -> Self {
    Client { stub: stub }
  }

  // Compile the computation with the given argument shapes and returns the
  // handle to the compiled executable. The compiled executable is cached on the
  // service, and the returned handle can be used for execution without
  // re-compile.
  // * The shape and layout of the arguments being executed with will affect how
  //   the computation is compiled. If argument_shapes is empty, the parameters'
  //   shape and layout will be used in the compilation.
  // * If execution_options is not nullptr, these options are passed to the
  //   service to affect how it compiles our computation.  (The pointer does not
  //   need to live beyond this call.)
  // * If execution_options.device_handles should be empty. If you need
  //   non-empty device handles, call 'Execute' instead.
  pub fn compile(
    &self,
    _computation: &BlitzComputation,
    argument_shapes: &Vec<Shape>,
    execution_options: Option<ExecutionOptions>) -> Result<ExecutionHandle, String>
  {
    if execution_options.is_none() {
      self.stub.compile(argument_shapes,
        create_default_execution_options())
    } else {
      self.stub.compile(argument_shapes,
        execution_options.unwrap())
    }
  }

  // Executes the compiled executable for the given handle with the given
  // arguments and returns the global data that was produced from the execution.
  // * If execution_profile is not nullptr then the pointed-to ExecutionProfile
  //   will be filled with profile data from the execution.
  pub fn execute(
    &self,
    handle: &ExecutionHandle,
    arguments: &Vec<GlobalData>,
    execution_profile: Option<ExecutionProfile>) -> Result<GlobalData, String>
  {
    self.stub.execute(handle, arguments, execution_profile)
  }

  // Executes the computation with the given arguments and returns the global
  // data that was produced from the execution.
  // * If execution_options is not nullptr, these options are passed to the
  //   service to affect how it compiles our computation.  (The pointer does not
  //   need to live beyond this call.)
  // * If execution_options.device_handles is not empty, the computation is
  //   executed on the devices associated with the handles by partitioning the
  //   computation based on the attached sharding attributes. Otherwise, a
  //   device is chosen by the service.
  // * If execution_profile is not nullptr then the pointed-to ExecutionProfile
  //   will be filled with profile data from the execution.
  pub fn execute_computation(
    &self,
    _computation: &BlitzComputation,
    _arguments: &Vec<GlobalData>,
    _execution_options: Option<ExecutionOptions>,
    _execution_profile: Option<ExecutionProfile>) -> Result<GlobalData, String>
  {
    unimplemented!()
  }

  // Executes a list XlaComputationInstances and returns global data produced
  // from each computation.
  pub fn execute_parallel(&self) {}

  // Requests device_count device handles available on the target. The returned
  // device handles are used to specify the devices to execute the computations
  // (see ExecuteParallel) or to transfer data (see TransferToServer or
  // TransferToInfeed).
  pub fn get_device_handles(
    &self, device_count: i64) -> Result<Vec<DeviceHandle>, String>
  {
    if device_count < 1 {
      return Err("device_count must be greater than 0".to_string());
    }
    self.stub.get_device_handles(device_count)
  }

  // Transfer the global data provided to this client process, which is
  // returned in the provided literal. Use sparingly to avoid transfer
  // overheads.
  //
  // If shape_with_layout is not nullptr, it points to a shape whose layout will
  // be the layout of the returned literal.
  pub fn transfer(
    &self,
    data: &GlobalData,
    shape_with_layout: Option<Shape>) -> Result<Literal, String>
  {
    self.stub.transfer_to_client(data, shape_with_layout)
  }

  // Transfer the given literal to the server. This allocates memory on the
  // device and copies the literal's contents over. Returns a global data handle
  // that can be used to refer to this value from the client.
  //
  // If device_handle is not nullptr, data is transferred to the associated
  // device (and its replicas if replication is enabled). Otherwise, data is
  // transferred to the default device (and its replicas).
  pub fn transfer_to_server(
    &self,
    literal: &Literal,
    device_handle: Option<DeviceHandle>) -> Result<GlobalData, String>
  {
    self.stub.transfer_to_server(literal, device_handle)
  }

  // Transfer the given literal to the Infeed interface of the device.
  //
  // device_handle and replica_id together specify a particular device; a device
  // assigned for the given replica_id among the replicas that the given device
  // handle belongs to.
  pub fn transfer_to_infeed(
    &self,
    literal: &Literal,
    replica_id: i64,
    device_handle: Option<DeviceHandle>) -> Result<(), String>
  {
    self.stub.transfer_to_infeed(literal, replica_id, device_handle)
  }

  // Transfers from the Outfeed of the device.
  //
  // device_handle and replica_id together specify a particular device; a device
  // assigned for the given replica_id among the replicas that the given device
  // handle belongs to.
  pub fn transfer_from_outfeed(
    &self,
    shape_with_layout: &Shape,
    replica_id: i64,
    device_handle: Option<DeviceHandle>) -> Result<Literal, String>
  {
    self.stub.transfer_from_outfeed(shape_with_layout, replica_id, device_handle)
  }

  // Resets the device, clearing all existing state on the device.
  pub fn reset_device(&self) -> Result<(), String> {
    self.stub.reset_device()
  }

  // Executes the computation with the given arguments and transfers the result
  // to the client as a literal. Parameters are defined the same as for
  // Execute() and Transfer().
  pub fn execute_and_transfer(
    &self,
    _computation: &BlitzComputation,
    _arguments: &Vec<GlobalData>,
    _execution_options: Option<ExecutionOptions>,
    _execution_profile: Option<ExecutionProfile>) -> Result<Literal, String>
  {
    unimplemented!()
  }

  // Computes the value of the given computation using a non-optimized
  // interpreter on the host.
  //
  // The computation must not depend on any parameters, or on stateful operators
  // such as `RngNormal` or `Infeed`.
  //
  // This functionality can be useful when translating a computation into XLA
  // where something that looked dynamic is required by Blitz to be specified as a
  // constant. E.g. the source computation (outside of Blitz) may include a
  // dynamic computation of the shape of something and ComputeConstant lets you
  // determine what the value of that computation is in the case where the value
  // can be determined at compile time.
  //
  // If output_layout is non-null, then the output of the computation will be
  // stored using that layout.
  pub fn compute_constant(
    &self,
    _computation: &BlitzComputation,
    _output_layout: Option<Layout>) -> Result<Literal, String>
  {
    unimplemented!()
  }

  // Unregister the memory for the given GlobalData on the device.
  pub fn unregister(&self, data: &GlobalData) -> Result<(), String> {
    self.stub.unregister(data.handle())
  }

  // Returns a vector of global data handles that point to the tuple elements.
  pub fn deconstruct_tuple(&self, data: &GlobalData) -> Result<Vec<GlobalData>, String> {
    self.stub.deconstruct_tuple(data)
  }
  
  // Returns the Shape of the given array specified by 'data'. The shape
  // includes the Layout of the array as it is stored on the service.
  pub fn get_shape(&self, data: &GlobalData) -> Result<Shape, String> {
    self.stub.get_shape(data)
  }

  // As above, but returns the shape of the provided computation (parameter
  // types/names and return type).
  pub fn get_computation_shape(
    &self, computation: &BlitzComputation) -> Result<ProgramShape, String>
  {
    computation.get_program_shape()
  }

  // Creates a channel handle that can be used to transfer data between two
  // computations on different devices via a pair of Send and Recv instructions.
  pub fn create_channel_handle(&self) -> Result<ChannelHandle, String> {
    self.create_channel_handle_by_type(ChannelType::DeviceToDevice)
  }

  // Create a channel for communicating with the host via a SendtoHost or
  // RecvFromHost operation.
  pub fn create_host_to_device_channel_handle(&self) -> Result<ChannelHandle, String> {
    self.create_channel_handle_by_type(ChannelType::HostToDevice)
  }

  pub fn create_device_to_host_channel_handle(&self) -> Result<ChannelHandle, String> {
    self.create_channel_handle_by_type(ChannelType::DeviceToHost)
  }

  pub fn load_snapshot(&self, _module: &HloSnapshot) -> Result<BlitzComputation, String> {
    unimplemented!()
  }

  pub fn stub(&self) -> &Service {
    &self.stub
  }

  fn create_channel_handle_by_type(&self, t: ChannelType) -> Result<ChannelHandle, String> {
    self.stub.create_channel_handle(t)
  }
}