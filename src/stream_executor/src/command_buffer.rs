#![allow(dead_code)]

use std::u64;

use crate::{device_memory::DeviceMemoryBase, kernel::{Kernel, KernelArgs}, launch_dim::{BlockDim, ThreadDim}, stream::Stream, stream_executor::StreamExecutor};

pub enum CommandBufferState {
  Create,
  Update,
  Finalized,
}

pub enum CommandBufferMode {
  Primary,
  Nested,
}

// Command buffer represent a "bundle of work items" for StreamExecutor device
// that can be submitted with one API call, e.g. command buffer might have
// multiple device kernels and synchronization barriers between them. Command
// buffers allow to amortize the cost of launching "work" on device by building
// it on the host ahead of time without expensive interaction with underlying
// device.
pub struct CommandBuffer {}

impl CommandBuffer {
  // Creates a new empty command buffer on the given executor.
  pub fn new(_executor: &dyn StreamExecutor, _mode: CommandBufferMode) -> Self {
    unimplemented!()
  }

  // Creates a new command buffer on the given executor by tracing `function`
  // invocation. All StreamExecutor operations on a Stream argument will be
  // recorded into the command buffer. Returned command buffer is finalized, and
  // can't be updated.
  //
  // Command buffer tracing should be used only when it is impossible to use
  // explicit construction APIs, e.g. when calling external libraries. By
  // default we construct traced command buffers in nested mode because the
  // primary use case for traced command buffers is to be inserted into primary
  // command buffers constructed with explicit APIs.
  pub fn trace(
    _executor: &dyn StreamExecutor,
    _func: Box<dyn Fn()>,
    _mode: CommandBufferMode) -> Self
  {
    unimplemented!()
  }

  // Creates a new command buffer on the given executor by tracing `function`
  // invocation using a user provided stream that will be passed to `function`.
  pub fn trace_by_stream(
    _executor: &dyn StreamExecutor,
    _stream: &dyn Stream,
    _func: Box<dyn Fn()>,
    _mode: CommandBufferMode) -> Self
  {
    unimplemented!()  
  }

  pub fn supports_conditional_commands() {}

  // Adds an execution barrier to a given execution scope: all commands added
  // before a barrier in a the execution scope will complete before any of the
  // commands added after a barrier in the same execution scope.
  pub fn barrier(
    &self,
    _executor: &dyn StreamExecutor,
    _execution_scope_id: u64) -> Result<(), String>
  {
    unimplemented!()
  }

  // Adds a kernel launch command.
  pub fn launch(
    &self,
    _execution_scope_id: u64,
    _threads: &ThreadDim,
    _blocks: &BlockDim,
    _kernel: &dyn Kernel,
    _args: &KernelArgs) -> Result<(), String>
  {
    unimplemented!()
  }

  // Adds a nested command buffer.
  pub fn add_nested_command_buffer(
    &self,
    _execution_scope_id: u64,
    _nested: &CommandBuffer) -> Result<(), String>
  {
    unimplemented!()
  }

  // Adds a device-to-device memory copy to the default execution scope.
  pub fn memcpy_device_to_device(
    &self,
    _dst: &DeviceMemoryBase,
    _src: &DeviceMemoryBase, _size: u64) -> Result<(), String>
  {
    unimplemented!()
  }

  // Adds a memset command.
  pub fn memset(
    &self,
    _execution_scope_id: u64,
    _dst: &DeviceMemoryBase,
    //_bit_pattern:,
    _num_elements: usize) -> Result<(), String>
  {
    unimplemented!()
  }

  pub fn allocate() {}
  pub fn free() {}

  // Adds a conditional operation that will execute a command buffer constructed
  // by `then_builder` if `pred` value is `true`.
  pub fn if_(
    &self,
    _execution_scope_id: u64,
    _executor: &dyn StreamExecutor,
    //_pred: ,
    //_then_builder: 
    ) -> Result<(), String>
  {
    unimplemented!()
  }

  // Adds a conditional operation that will execute a command buffer constructed
  // by `then_builder` if `pred` value is `true`, or a command buffer
  // constructed by `else_builder` if `pred` is `false`.
  pub fn if_else(
    &self,
    _execution_scope_id: u64,
    _executor: &dyn StreamExecutor
    // TODO
    ) -> Result<(), String>
  {
    unimplemented!()
  }

  // Adds a conditional operation that will execute a command buffer constructed
  // by the `branches` builder at `index`. If `index` is out of range, then it
  // will run a conditional command buffer constructed by the last builder.
  pub fn case(
    &self,
    _execution_scope_id: u64,
    _executor: &dyn StreamExecutor) -> Result<(), String>
  {
    unimplemented!()
  }

  // Adds a conditional operation that will execute a command buffer constructed
  // by the `body_builder` exactly `num_iteration` times. This means the
  // condition is known at compile time (`num_iteration` < `loop_counter`), and
  // does not require a `cond_builder`.
  pub fn for_(
    &self,
    _execution_scope_id: u64,
    _executor: &dyn StreamExecutor,
    _num_iteration: i64) -> Result<(), String>
  {
    unimplemented!()
  }

  // Adds a conditional operation that will execute a command buffer constructed
  // by the `cond_builder` that must update `pred` value, and then depending on
  // the value might execute command buffer constructed by `body_builder` and
  // `cond_builder`. Will continue while `pred` value (which is continuously
  // updated by `cond_builder`) is `true`.
  //
  // In pseudocode:
  //
  //   cond_builder()
  //   while(pred):
  //     body_builder()
  //     cond_builder()
  //
  // We use execution scope builder for the condition because we have to build
  // condition twice: (1) before the conditional node in the scope defined by
  // `execution_scope_id` (2) inside the loop body with default execution scope.
  pub fn while_(
    &self,
    _execution_scope_id: u64,
    _executor: &dyn StreamExecutor) -> Result<(), String>
  {
    unimplemented!()
  }

  // Finalizes command buffer and makes it executable. Once command buffer is
  // finalized no commands can be added to it.
  pub fn finalize(&self) -> Result<(), String> {
    unimplemented!()
  }

  // Begins command buffer update. Command buffer update should be finalized
  // before it can be executed.
  pub fn update(&self) -> Result<(), String> {
    unimplemented!()
  }

  // Returns command buffer execution mode.
  pub fn mode(&self) -> CommandBufferMode {
    unimplemented!()
  }

  // Returns command buffer state.
  pub fn state(&self) -> CommandBufferState {
    unimplemented!()
  }
}