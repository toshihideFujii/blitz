#![allow(dead_code)]

use std::collections::HashMap;

use common::{blitz_data::{DebugOptions, Precision}, computation_placer::DeviceAssignment, shape::Shape};
use hlo::{hlo_instruction::HloInstruction, hlo_module::HloModule, hlo_module_group::HloModuleGroup};
use stream_executor::{device_memory_allocator::DeviceMemoryAllocator, platform::Platform, stream_executor::StreamExecutor};

use crate::{executable::Executable, metrics_hook_interface::MetricsHookInterface};

// Abstract superclass describing the result of an ahead-of-time compilation.
pub struct AotCompilationResult {}

impl AotCompilationResult {
  pub fn serialize_as_string(&self) -> String {
    unimplemented!()
  }

  pub fn load_executable(
    &self,
    _compiler: &Compiler,
    _executor: &StreamExecutor) -> Result<Executable, String>
  {
    unimplemented!()
  }

  // Returns the optimized HLO module if one was computed and the implementation
  // supports it.
  pub fn optimized_module(&self) -> &HloModule {
    unimplemented!()
  }

  pub fn custom_optimized_module(&self) -> &HloModule {
    unimplemented!()
  }
}

// Abstract superclass describing metadata produced during ahead-of-time
// compilation.
pub struct AotCompilationMetadata {}

impl AotCompilationMetadata {
  pub fn new() {}
  pub fn to_string() {}
}

pub struct TargetConfig {}

pub struct CompileOptions {
  pub device_allocator: Option<DeviceMemoryAllocator>,
  is_autotuning_compilation: bool,
}

impl CompileOptions {
  pub fn default() -> Self {
    CompileOptions {
      device_allocator: None,
      is_autotuning_compilation: false
    }
  }
}

// Abstract compiler interface that is subclassed for compilation on a
// particular platform.
//
// The compiler ties together high level optimization (HLO) and low level
// optimization (LLO) / codegen (CG) to generate efficient executables for the
// target platform.
//
// The platform-based compiler singletons are registered via module initializers
// in their corresponding Blitz compiler libraries, and are registered via the
// RegisterCompilerFactory API below.
//
// Thread-safety: subclasses of Compiler must be thread-safe, as multiple
// Blitz clients may be requesting compilation concurrently for a given
// platform.
#[derive(Debug, Clone)]
pub struct Compiler {}

impl Compiler {
  // Returns the ID of the platform that this compiler targets.
  pub fn platform_id(&self) -> i64 {
    unimplemented!()
  }

  // Runs Hlo passes to optimize the given Hlo module, returns the optimized
  // module.
  pub fn run_hlo_passes(
    &self,
    _module: &HloModule,
    _executor: &StreamExecutor) -> Result<HloModule, String>
  {
    unimplemented!()
  }

  pub fn assign_buffers() {}

  // Compiles the HLO module for execution on a device given by the executor,
  // and returns an executable object or an error status. No HLO passes are
  // applied to module. Generally a module should be passed through RunHloPasses
  // prior to calling this method because some HLO passes are required for
  // correctness. Takes ownership of the HLO module.
  //
  // The compiler may optionally specialize to the individual device
  // (not just type of device) indicated by the executor.
  pub fn run_backend(
    &self,
    _module: &HloModule,
    _executor: &StreamExecutor,
    _options: &CompileOptions) -> Result<Executable, String>
  {
    unimplemented!()
  }

  pub fn run_backend_with_buffer_assignment() {}

  // Returns a (deserialized) AotCompilationResult from a serialized
  // AotCompilationResult.
  pub fn load_aot_compilation_result(
    &self,
    _serialized_aot_result: String) -> Result<AotCompilationResult, String>
  {
    unimplemented!()
  }

  // Compiles a set of HLO modules that can run in parallel, potentially
  // communicating data between the modules, and returns a corresponding
  // sequence of executable objects.
  pub fn compile(
    &self,
    _module_group: &HloModuleGroup,
    _stream_exec: &Vec<Vec<StreamExecutor>>,
    _options: &CompileOptions) -> Result<Vec<Executable>, String>
  {
    unimplemented!()
  }

  // Returns the backend configurations that the backend will consider for the
  // given HLO. Returns no configurations if the backend does not support
  // configurations for the given HLO.
  //
  // The stream executor is passed in to provide information about the hardware
  // that the backend configurations would be targeting.
  pub fn compute_backend_configs(
    &self,
    _hlo: &HloInstruction,
    _executor: &StreamExecutor)
  {
    unimplemented!()
  }

  // Returns the backend configuration that the backend chooses by default for
  // the given HLO. Returns no configuration if the backend does not support
  // configurations for the given HLO.
  //
  // The stream executor is passed in to provide information about the hardware
  // that the backend configurations would be targeting.
  pub fn compute_default_backend_config(
    &self,
    _hlo: &HloInstruction,
    executor: Option<&StreamExecutor>) -> Option<String>
  {
    debug_assert!(executor.is_some());
    None
  }

  // Compiles the HLO module group for ahead-of-time execution.  This is
  // intended for use in static compilation.
  pub fn compile_ahead_of_time(
    &self,
    _module_group: &HloModuleGroup,
    _options: &AotCompilationOptions) -> Result<Vec<AotCompilationResult>, String>
  {
    unimplemented!()
  }

  // Similar to CompileAheadOfTime above but AotCompilationMetadata
  // has an argument that can be populated during compilation.
  pub fn compile_ahead_of_time_by_metadata(
    &self,
    _module_group: &HloModuleGroup,
    _options: &AotCompilationOptions,
    _metadata: &AotCompilationMetadata) -> Result<Vec<AotCompilationResult>, String>
  {
    unimplemented!()
  }

  // Registers the compiler singleton for the platform. This is assumed to
  // be a singleton, so no ownership is transferred.
  //
  // Precondition: a platform kind must not be registered more than once.
  pub fn register_compiler_factory<F>(
    _platform_id: i64,
    _compiler_factory: F) where F: Fn()->Result<Compiler, String>
  {
    unimplemented!()
  }

  // Returns the compiler singleton pointer if it is available for the given
  // platform, or an error status if it is not.
  pub fn get_for_platform(_platform: &Platform) -> Result<&Self, String>
  {
    unimplemented!()
  }

  // Returns a function that computes the size in bytes of the logical
  // buffer that contains a shape.
  pub fn shape_size_bytes_function(&self) {
    unimplemented!()
  }

  // Returns a function that computes the size in bytes of a given
  // logical buffer.
  pub fn buffer_size_bytes_function(&self) {
    unimplemented!()
  }

  pub fn default_device_shape_representation(
    &self,
    _shape: &Shape) -> Shape
  {
    unimplemented!()
  }

  // Returns an AotCompilationResult of the executable for serialization.
  pub fn export(
    &self,
    _executable: &Executable) -> Result<AotCompilationResult, String>
  {
    unimplemented!()
  }

  // Returns a MetricsHookInterface object used to instrument Compiler's
  // compilation stages.
  pub fn create_metrics_hook(
    &self,
    _filename_prefix: String,
    _hlo_module_name: String) -> Option<MetricsHookInterface>
  {
    None
  }

  // Map from platform kind to compiler factory.
  fn get_platform_compiler_factories() -> HashMap<i64, Self> {
    unimplemented!()
  }

  // Map from platform kind to compiler instance, if we made one already (based
  // on the factories above).
  fn get_platform_compilers() -> HashMap<i64, Self> {
    unimplemented!()
  }
}

// Abstract superclass describing options to an ahead_of-time compilation.
pub struct AotCompilationOptions {
  plaatform_id: i64,
  device_allocator: Option<DeviceMemoryAllocator>,
  debug_options: DebugOptions,
  static_device_assignment: Option<DeviceAssignment>,
  fusion_config: Vec<Vec<bool>>,
  executor: Option<StreamExecutor>,
  profile_version: i64,
  cache_key: String,
  run_backend_only: bool,
  sanitize_dataflow: bool,
  sanitize_abilists_dataflow: Vec<String>,
  target_config: Option<TargetConfig>
}

impl AotCompilationOptions {
  pub fn new(platform_id: i64) -> Self {
    AotCompilationOptions {
      plaatform_id: platform_id,
      device_allocator: None,
      debug_options: DebugOptions::default(),
      static_device_assignment: None,
      fusion_config: Vec::new(),
      executor: None,
      profile_version: 0,
      cache_key: "".to_string(),
      run_backend_only: false,
      sanitize_dataflow: false,
      sanitize_abilists_dataflow: Vec::new(),
      target_config: None
    }
  }

  // Returns the ID of the platform to which these options apply.
  pub fn platform_id(&self) -> i64 {
    self.plaatform_id
  }

  pub fn replica_count(&self) -> i64 { 0 }

  pub fn num_cores(&self) -> i64 { 0 }

  pub fn use_spmd_partitioning(&self) -> bool { false }

  pub fn use_auto_spmd_partitioning(&self) -> bool { false }

  pub fn auto_spmd_partitioning_mesh_shape(&self) -> Vec<i64> { vec![] } 

  pub fn auto_spmd_partitioning_mesh_ids(&self) -> Vec<i64> { vec![] }

  pub fn duplicaate_hlo(&self) -> bool { false }

  pub fn matrix_unit_operand_precision(&self) -> Precision {
    Precision::Default
  }

  // Optional allocator that may be used for allocating temp space on the device
  // during compilation.
  pub fn device_allocator(&self) -> &Option<DeviceMemoryAllocator> {
    &self.device_allocator
  }

  pub fn set_device_allocator(&mut self, device_allocator: DeviceMemoryAllocator) {
    self.device_allocator = Some(device_allocator);
  }

  pub fn debug_options(&self) -> &DebugOptions {
    &self.debug_options
  }

  pub fn mutable_debug_options(&mut self) -> &mut DebugOptions {
    &mut self.debug_options
  }

  pub fn has_static_device_assignment(&self) -> bool {
    self.static_device_assignment.is_some()
  }

  pub fn static_device_assignment(&self) -> &DeviceAssignment {
    assert!(self.static_device_assignment.is_some());
    self.static_device_assignment.as_ref().unwrap()
  }

  pub fn set_static_device_assignment(&mut self, device_assignment: DeviceAssignment) {
    self.static_device_assignment = Some(device_assignment);
  }

  pub fn fusion_config_collection(&self) {}
  pub fn set_fusion_config_collection(&mut self) {}

  pub fn fusion_config(&self) -> &Vec<Vec<bool>> {
    &self.fusion_config
  }

  pub fn set_fusion_config(&mut self, fusion_config: Vec<Vec<bool>>) {
    self.fusion_config = fusion_config;
  }

  pub fn executor(&self) -> &Option<StreamExecutor> {
    &self.executor
  }

  pub fn set_executor(&mut self, executor: StreamExecutor) {
    self.executor = Some(executor);
  }

  // Optional profile_version and cache key may be used to trigger recompilation
  // when a compilation cache is used.
  pub fn profile_version(&self) -> i64 {
    self.profile_version
  }

  pub fn set_profile_version(&mut self, profile_version: i64) {
    self.profile_version = profile_version;
  }

  pub fn cache_key(&self) -> String {
    self.cache_key.clone()
  }

  pub fn set_cache_key(&mut self, cache_key: String) {
    self.cache_key = cache_key;
  }

  pub fn run_backend_only(&self) -> bool {
    self.run_backend_only
  }

  pub fn set_run_backend_only(&mut self, run_backend_only: bool) {
    self.run_backend_only = run_backend_only;
  }

  pub fn sanitize_dataflow(&self) -> bool {
    self.sanitize_dataflow
  }

  pub fn set_sanitize_dataflow(&mut self, sanitize_dataflow: bool) {
    self.sanitize_dataflow = sanitize_dataflow;
  }

  pub fn sanitize_abilists_dataflow(&self) -> &Vec<String> {
    &self.sanitize_abilists_dataflow
  }

  pub fn set_sanitize_abilists_dataflow(&mut self, abilists: Vec<String>) {
    self.sanitize_abilists_dataflow = abilists;
  }

  pub fn target_config(&self) -> &Option<TargetConfig> {
    &self.target_config
  }

  pub fn set_target_config(&mut self, target_config: TargetConfig) {
    self.target_config = Some(target_config);
  }

  pub fn use_shardy_partitioner(&self) -> bool {
    unimplemented!()
  }

  pub fn deduplicate_hlo(&self) -> bool {
    unimplemented!()
  }
}