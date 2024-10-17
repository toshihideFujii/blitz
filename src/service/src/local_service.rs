#![allow(dead_code)]

use common::{blitz_data::GlobalDataHandle, shape::Shape};

use crate::{backend::Backend, blitz_computation::BlitzComputation, compiler::AotCompilationResult, executable::Executable, executable_build_options::ExecutableBuildOptions, service::{Service, ServiceOptions}, shaped_buffer::ShapedBuffer};

// Service implementation that extends the Blitz service to leverage running
// in the same process as the client.
pub struct LocalService {
  service: Service,
}

impl LocalService {
  pub fn new(_options: ServiceOptions) -> Self {
    unimplemented!()
  }

  pub fn backend(&self) -> &Backend {
    self.service.backend()
  }

  // Builds Executables with the given BlitzComputation, argument layouts and
  // options. If result_layout is non-null, then the executable is compiled to
  // produce a result of the given layout.  If device_allocator is non-null,
  // then the compiler may use it to allocate temp space on the device.  The
  // compiler is responsible for freeing any memory it allocates this way.
  pub fn compile_executables(
    &self,
    _computation: &BlitzComputation,
    _argument_layouts: &Vec<Shape>,
    _build_options: &ExecutableBuildOptions) -> Result<Vec<Executable>, String>
  {
    unimplemented!()
  }

  // Same as CompileExecutables() above, but return AotCompilationResult objects
  // (instead of Executable objects), which can be persisted to later load
  // Executable objects.
  pub fn compile_aot_results(
    &self,
    _computation: &BlitzComputation,
    _argument_layouts: &Vec<Shape>,
    _build_options: &ExecutableBuildOptions) -> Result<Vec<AotCompilationResult>, String>
  {
    unimplemented!()
  }

  // Returns the device ordinal that corresponds to the given replica number.
  //
  // This returns an error if there is not a one-to-one correspondence of
  // replicas to device ordinals, but is useful as a short term mechanism for
  // the "easy" case where a single replica is a single device.
  pub fn replica_number_to_device_ordinal(
    &self, _replica_number: i64) -> Result<i64, String>
  {
    unimplemented!()
  }

  // Converts a GlobalDataHandle into a pointer to a ShapedBuffer that's valid
  // as long as the handle is valid.
  pub fn global_data_to_shaped_buffer(
    &self, _data: &GlobalDataHandle, _replica_number: i64) -> Result<ShapedBuffer, String>
  {
    unimplemented!()
  }

  pub fn register_replicated_buffers() {}
}