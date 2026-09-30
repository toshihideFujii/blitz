#![allow(dead_code)]

use std::{collections::HashMap, hash::Hash, sync::OnceLock};
use crate::{debug_options_flags::get_debug_options_from_flags, shape::Shape};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
  Invalid,
  Pred,
  S4,
  S8,
  S16,
  S32,
  S64,
  U4,
  U8,
  U16,
  U32,
  U64,
  F16,
  F32,
  BF16,
  F64,
  F8E5M2,
  F8E4M3FN,
  F8E4M3B11FNUZ,
  F8E5M2FNUZ,
  F8E4M3FNUZ,
  F8E8M0FNU,
  C64,
  C128,
  Tuple,
  Token,
  OpaqueType,
}

pub const PRIMITIVE_TYPE_ARRAYSIZE: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DimLevelType {
  Dense,
  Compressed,
  Singleton,
  LooseCompressed,
}

#[derive(Clone, PartialEq)]
pub enum Precision {
  Default,
  High,
  Highest,
  PackedNibble,
}

pub enum Algorithm {
  Unset,
  DotAnyF8anyF8F32,
}

#[derive(Debug, Clone)]
pub struct PrecisionConfig {}

impl PrecisionConfig {
  pub fn operand_precision(&self) -> &Vec<Precision> {
    unimplemented!()
  }
}

// The type optimization profiles in use for Op-level optimizations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProfileType {
  Invalid,
  Window,
  Flag,
  Integer,
}

// The source of the optimization profile.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProfileSource {
  Unknown,
  Embedded,
  Remote,
}

// The compilation event that triggered the use of the profile.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompilationEvent {
  Unknown,
  ForstCompilation,
  Recompilation,
}

// Information about the optimization profile that this operation contains.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ProfileInfo {
  profile_type: Vec<ProfileType>,
  //relative_speedup: f64,
  profile_source: ProfileSource,
  compilation_event: CompilationEvent,
}

impl ProfileInfo {
  pub fn new() -> Self {
    ProfileInfo {
      profile_type: Vec::new(),
      profile_source: ProfileSource::Unknown,
      compilation_event: CompilationEvent::Unknown,
    }
  }
}

// Symbolization metadata for HLO Instructions.
//
// This metadata is used for debugging Blitz code generation, as well as
// performance profiling of Blitz-generated executables.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OpMetadata {
  op_type: String,
  op_name: String,
  source_file: String,
  source_line: i64,
  profile_type: ProfileType,
  size_of_generated_code_in_bytes: i64,
  size_of_memory_working_set_in_bytes: i64,
  profile_info: ProfileInfo,
  deduplicated_name: String,
  preserve_layout: bool,
  stack_frame_id: i64,
  scheduling_name: String
}

impl OpMetadata {
  pub fn default() -> Self {
    OpMetadata {
      op_type: "".to_string(),
      op_name: "".to_string(),
      source_file: "".to_string(),
      source_line: 0,
      profile_type: ProfileType::Invalid,
      size_of_generated_code_in_bytes: 0,
      size_of_memory_working_set_in_bytes: 0,
      profile_info: ProfileInfo::new(),
      deduplicated_name: "".to_string(),
      preserve_layout: false,
      stack_frame_id: 0,
      scheduling_name: "".to_string()
    }
  }

  pub fn creation_pass_id(&self) -> i64 { 0 }
  pub fn set_creation_pass_id(&mut self, _id: i64) {}
  pub fn set_size_of_generated_code_in_bytes(&mut self, _code_size_in_bytes: i64) {}
  pub fn set_size_of_memory_working_set_in_bytes(&mut self, _working_set_size_in_bytes: i64) {}
  pub fn op_name(&self) -> String { "".to_string() }
  pub fn set_op_name(&mut self, _name: String) {}
  pub fn op_type(&self) -> String { "".to_string() }
  pub fn set_logical_creation_pass_id(&mut self, _pass_id: i64) {}
  pub fn set_deduplicated_name(&mut self, _deduplicated_name: String) {}
  pub fn set_preserve_layout(&mut self, _preserve_layout: bool) {}
  pub fn source_file(&self) -> String { "".to_string() }
  pub fn source_line(&self) -> usize { 0 }
  pub fn profile_type(&self) -> String { "".to_string() }
  pub fn deduplicated_name(&self) -> String { "".to_string() }
  pub fn preserve_layout(&self) -> bool { false }

  pub fn clear(&mut self) {
    unimplemented!()
  }
}

// Generic map of attributes used to pass hints / configuration options from
// the Python frontend to the Blitz backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontendAttributes {
  map: HashMap<String, String>
}

impl FrontendAttributes {
  pub fn default() -> Self {
    FrontendAttributes {
      map: HashMap::new()
    }
  }

  pub fn map(&self) -> &HashMap<String, String> {
    //&self.map
    unimplemented!()
  }

  pub fn mutable_map(&mut self) -> &mut HashMap<String, String> {
    //&mut self.map
    unimplemented!()
  }

  pub fn set_attribute(&mut self, _key: String, _value: String) {
    unimplemented!()
  }

  pub fn has_attribute(&self, _key: String) -> bool {
    unimplemented!()
  }

  pub fn clear(&mut self) {
    unimplemented!()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Statisitic {
  stat_name: String,
  stat_val: i64, // TODO: f64
}

impl Statisitic {
  pub fn default() -> Self {
    Statisitic { stat_name: "".to_string(), stat_val: 0 }
  }

  pub fn set_stat_name(&mut self, stat_name: String) {
    self.stat_name = stat_name;
  }

  pub fn stat_name(&self) -> String {
    self.stat_name.clone()
  }

  pub fn set_stat_val(&mut self, stat_val: i64) {
    self.stat_val = stat_val;
  }

  pub fn stat_val(&self) -> i64 {
    self.stat_val
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StatisticsViz {
  stat_index_to_viaualize: i64,
  statiscics: Vec<Statisitic>
}

impl StatisticsViz {
  pub fn default() -> Self {
    StatisticsViz { stat_index_to_viaualize: 0, statiscics: Vec::new() }    
  }

  pub fn add_statistics(&mut self, statistic: Statisitic) {
    self.statiscics.push(statistic);
  }

  pub fn stat_index_to_viaualize(&self) -> i64 {
    self.stat_index_to_viaualize
  }

  pub fn set_stat_index_to_visualize(&mut self, index: i64) {
    self.stat_index_to_viaualize = index;
  }

  pub fn statiscics(&self) -> &Vec<Statisitic> {
    &self.statiscics
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PaddingType {
  Invalid,
  // Only valid portion of the base are covered.
  Valid,
  // Extra is added to produce same output size as the input.
  Same,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FftType {
  FFT,
  IFFT,
  RFFT,
  IRFFT,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OpShardingType {
  Replicated,
  Maximal,
  Tuple,
  Other,
  Manual,
  Unreduced,
  Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpSharding {
  t: OpShardingType,
  tile_shape: Shape,
  tile_assignment_dimensions: Vec<i64>,
  tile_assignment_devices: Vec<i64>
}

impl OpSharding {
  pub fn new() -> Self {
    OpSharding {
      t: OpShardingType::Unknown,
      tile_shape: Shape::default(),
      tile_assignment_dimensions: Vec::new(),
      tile_assignment_devices: Vec::new()
    }
  }

  pub fn type_(&self) -> OpShardingType {
    self.t.clone()
  }

  pub fn set_type(&mut self, t: OpShardingType) {
    self.t = t;
  }

  pub fn set_tile_shape(&mut self, tile_shape: Shape) {
    self.tile_shape = tile_shape;
  }

  pub fn add_tile_assignment_dimensions(&mut self, dimension: i64) {
    self.tile_assignment_dimensions.push(dimension);
  }

  pub fn add_tile_assignment_devices(&mut self, device: i64) {
    self.tile_assignment_devices.push(device);
  }
}

#[derive(Debug, Clone)]
pub struct DotDimensionNumbers {
  lhs_contracting_dimensions: i64,
  rhs_contracting_dimensions: i64,
  lhs_batch_dimensions: i64,
  rhs_batch_dimensions: i64,
}

impl DotDimensionNumbers {
  pub fn default() -> Self {
    DotDimensionNumbers {
      lhs_contracting_dimensions: 0,
      rhs_contracting_dimensions: 0,
      lhs_batch_dimensions: 0,
      rhs_batch_dimensions: 0
    }
  }

  pub fn lhs_contracting_dimensions(&self) -> i64 {
    self.lhs_contracting_dimensions
  }

  pub fn add_lhs_contracting_dimensions(&mut self, _dims: i64) {
    unimplemented!()
  }

  pub fn add_rhs_contracting_dimensions(&mut self, _dims: i64) {
    unimplemented!()
  }
}

// Describes whether all data-parallelism replicas will receive the same
// parameter data at each buffer.
#[derive(Debug, Clone)]
pub struct ParameterReplication {
  // A list of boolean values for the flattened leaf buffers. Each value
  // indicates whether the corresponding leaf buffer is replicated.
  //
  // If this field is empty, it means no buffer is replicated. Otherwise, the
  // number of elements in this field must match the number of leaf buffers in
  // the HLO instruction's shape.
  replicated_at_leaf_buffers: Vec<bool>
}

impl ParameterReplication {
  pub fn default() -> Self {
    ParameterReplication {
      replicated_at_leaf_buffers: vec![false; 1]
    }
  }

  pub fn replicated_at_leaf_buffers(&self) -> &Vec<bool> {
    &self.replicated_at_leaf_buffers
  }

  pub fn add_replicated_at_leaf_buffers(&mut self, _value: bool) {
    unimplemented!()
  }
}

#[derive(Debug, Clone)]
pub struct ConvolutionDimensionNumbers {
  input_batch_dimension: i64,
  input_feature_dimension: i64,
  input_spatial_dimensions: Vec<i64>,
  kernel_input_feature_dimension: i64,
  kernel_output_feature_dimension: i64,
  kernel_spatial_dimensions: Vec<i64>,
  output_batch_dimension: i64,
  output_feature_dimension: i64,
  output_spatial_dimensions: Vec<i64>
}

impl ConvolutionDimensionNumbers {
  pub fn default() -> Self {
    ConvolutionDimensionNumbers {
      input_batch_dimension: 0,
      input_feature_dimension: 0,
      input_spatial_dimensions: Vec::new(),
      kernel_input_feature_dimension: 0,
      kernel_output_feature_dimension: 0,
      kernel_spatial_dimensions: Vec::new(),
      output_batch_dimension: 0,
      output_feature_dimension: 0,
      output_spatial_dimensions: Vec::new()
    }
  }

  pub fn input_batch_dimension(&self) -> i64 {
    self.input_batch_dimension
  }

  pub fn set_input_batch_dimension(&mut self, dimension: i64) {
    self.input_batch_dimension = dimension;
  }

  pub fn input_feature_dimension(&self) -> i64 {
    self.input_feature_dimension
  }

  pub fn set_input_feature_dimension(&mut self, dimension: i64) {
    self.input_feature_dimension = dimension;
  }

  pub fn output_batch_dimension(&self) -> i64 {
    self.output_batch_dimension
  }

  pub fn set_output_batch_dimension(&mut self, dimension: i64) {
    self.output_batch_dimension = dimension;
  }

  pub fn output_feature_dimension(&self) -> i64 {
    self.output_feature_dimension
  }

  pub fn set_output_feature_dimension(&mut self, dimension: i64) {
    self.output_feature_dimension = dimension;
  }

  pub fn kernel_input_feature_dimension(&self) -> i64 {
    self.kernel_input_feature_dimension
  }

  pub fn set_kernel_input_feature_dimension(&mut self, dimension: i64) {
    self.kernel_input_feature_dimension = dimension;
  }

  pub fn kernel_output_feature_dimension(&self) -> i64 {
    self.kernel_output_feature_dimension
  }

  pub fn set_kernel_output_feature_dimension(&mut self, dimension: i64) {
    self.kernel_output_feature_dimension = dimension;
  }

  pub fn input_spatial_dimensions_size(&self) -> usize {
    self.input_spatial_dimensions.len()
  }

  pub fn set_input_spatial_dimensions(&self, _c: char, _i: usize) {
    unimplemented!()
  }

  pub fn input_spatial_dimensions(&self, index: usize) -> i64 {
    self.input_spatial_dimensions[index]
  }

  pub fn input_spatial_dimensions_vec(&self) -> &Vec<i64> {
    &self.input_spatial_dimensions
  }

  pub fn add_input_spatial_dimensions(&mut self, dimension: i64) {
    self.input_spatial_dimensions.push(dimension);
  }

  pub fn output_spatial_dimensions_size(&self) -> usize {
    self.output_spatial_dimensions.len()
  }

  pub fn output_spatial_dimensions(&self, index: usize) -> i64 {
    self.output_spatial_dimensions[index]
  }

  pub fn output_spatial_dimensions_vec(&self) -> &Vec<i64> {
    &self.output_spatial_dimensions
  }

  pub fn add_output_spatial_dimensions(&mut self, dimension: i64) {
    self.output_spatial_dimensions.push(dimension);
  }

  pub fn kernel_spatial_dimensions_size(&self) -> usize {
    self.kernel_spatial_dimensions.len()
  }

  pub fn kernel_spatial_dimensions(&self, index: usize) -> i64 {
    self.kernel_spatial_dimensions[index]
  }

  pub fn kernel_spatial_dimensions_vec(&self) -> &Vec<i64> {
    &self.kernel_spatial_dimensions
  }

  pub fn add_kernel_spatial_dimensions(&mut self, dimension: i64) {
    self.kernel_spatial_dimensions.push(dimension);
  }
}

#[derive(Debug, Clone)]
pub struct PaddingConfigDimension {
  edge_padding_low: i64,
  edge_padding_high: i64,
  interior_padding: i64,
}

impl PaddingConfigDimension {
  pub fn default() -> Self {
    PaddingConfigDimension {
      edge_padding_low: 0,
      edge_padding_high: 0,
      interior_padding: 0
    }
  }

  pub fn edge_padding_low(&self) -> i64 {
    self.edge_padding_low
  }

  pub fn set_edge_padding_low(&mut self, edge_padding_low: i64) {
    self.edge_padding_low = edge_padding_low;
  }

  pub fn edge_padding_high(&self) -> i64 {
    self.edge_padding_high
  }

  pub fn set_edge_padding_high(&mut self, edge_padding_high: i64) {
    self.edge_padding_high = edge_padding_high;
  }

  pub fn interior_padding(&self) -> i64 {
    self.interior_padding
  }

  pub fn set_interior_padding(&mut self, interior_padding: i64) {
    self.interior_padding = interior_padding;
  }
}

// Describes the padding configuration for Pad operation. The padding amount on
// both edges as well as between the elements are specified for each dimension.
#[derive(Debug, Clone)]
pub struct PaddingConfig {
  dimensions: Vec<PaddingConfigDimension>
}

impl PaddingConfig {
  pub fn default() -> Self {
    PaddingConfig { dimensions: Vec::new() }
  }

  pub fn dimensions_vec(&self) -> &Vec<PaddingConfigDimension> {
    &self.dimensions
  }

  pub fn dimension(&self, _dimno: i64) -> &PaddingConfigDimension {
    unimplemented!()
  }

  pub fn add_dimensions(&mut self) -> &mut PaddingConfigDimension {
    let dimension = PaddingConfigDimension::default();
    self.dimensions.push(dimension);
    self.dimensions.last_mut().unwrap()
  }

  pub fn mutable_dimensions(&mut self, _dimno: i64) -> &mut PaddingConfigDimension {
    unimplemented!()
  }

  pub fn dimensions_size(&self) -> usize {
    unimplemented!()
  }

  pub fn short_debug_string(&self) -> String {
    unimplemented!()
  }
}

// Describes the replica groups in a cross replica op (e.g., all-reduce and
// all-to-all).
#[derive(Debug, Clone, PartialEq)]
pub struct ReplicaGroup {
  // The ids of the replicas that belongs to the same group. The ordering of the
  // ids matters in some ops (e.g., all-to-all).
  replica_ids: Vec<i64>
}

impl ReplicaGroup {
  pub fn new() -> Self {
    ReplicaGroup { replica_ids: Vec::new() }
  }

  pub fn replica_ids(&self) -> &Vec<i64> {
    &self.replica_ids
  }

  pub fn mutable_replica_ids(&mut self) -> &mut Vec<i64>{
    &mut self.replica_ids
  }

  pub fn add_replica_ids(&mut self, _id: i64) {
    unimplemented!()
  }
}

pub enum RandomDistribution {
  Invalid,
  // Creates a uniform-distribution-generated random number on the semi-open
  // interval [parameter[0], parameter[1]).
  Uniform,
  // Creates a normal-distribution-generated random number with mean
  // parameter[0] and standard deviation parameter[1].
  Normal,
}

pub fn random_distribution_name(dist: &RandomDistribution) -> String {
  match dist {
    RandomDistribution::Invalid => return "Invalid".to_string(),
    RandomDistribution::Uniform => return "Unform".to_string(),
    RandomDistribution::Normal => return "Normal".to_string(),
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RandomAlgorithm {
  Default,
  ThreeFry,
  Philox,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StepMarkerLocation {
  AtEntry,
  AtTopLevelWhileLoop,
  AtSecondLevelWhileLoop,
  None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CollectiveOpType {
  NoOp,
  AllReduce,
  AllGather,
  ReduceScatter,
  CollectiveBroadcast,
  AllToAll,
  CollectivePermute,
}

impl CollectiveOpType {
  pub fn to_string(&self) -> String {
    if *self == CollectiveOpType::AllReduce {
      return "AllReduce".to_string();
    } else if *self == CollectiveOpType::AllGather {
      return "AllGather".to_string();
    } else if *self == CollectiveOpType::ReduceScatter {
      return "ReduceScatter".to_string();
    } else if *self == CollectiveOpType::CollectiveBroadcast {
      return "CollectiveBroadcast".to_string();
    } else if *self == CollectiveOpType::AllToAll {
      return "AllToAll".to_string();
    } else if *self == CollectiveOpType::CollectivePermute {
      return "CollectivePermute".to_string();
    }
    "NoOp".to_string()
  }
}

// Enables strict PGLE checking. If an FDO profile is specified and latency
 // hiding scheduler encounters missing instructions in the profile
 // compilation will halt or warn depending on the value of this option.
#[derive(Debug, Clone)]
pub enum PgleStrictnessLevel {
  Off,
  Warn,
  Error,
}

#[derive(Debug, Clone)]
pub enum AutotuneCacheMode {
  Unspecified,
  Update,
  Read,
}

#[derive(Debug, Clone)]
pub enum LibNvJitLinkMode {
  Auto,
  Disabled,
  Enabled,
}

#[derive(Debug, Clone)]
pub enum WhileLoopUnrolling {
  NoUnroll,
  DoubleBuffer,
  FullUnroll,
  AutoUnroll,
}

#[derive(Debug, Clone)]
pub enum PartitioningAlgorithm {
  Noop,
  Exp0,
  Exp1,
  Exp2,
}

// Experimental optimizations for SPMD-based pipeline parallelism on GPU.
#[derive(Debug, Clone)]
pub enum PipelineParallelismOptLevel {
  Disable,
  Enable,
  EnableCycleDecomposer,
}

#[derive(Debug, Clone)]
pub enum ShapeChecks {
  Ignore,
  Runtime,
  CompileTime,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandBufferCmdType {
  Invalid,
  Fusion,
  Cublas,
  Cudnn,
  Collectives,
  Conditional,
  While,
  CustomCall,
  Cublaslt,
  DynamicSliceFusion,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LibraryFusionType {
  Invalid,
  Dot,
  Eltwise,
  Reduce,
  IndividualDot,
  IndividualConvolution
}

#[derive(Debug, Clone, PartialEq)]
pub enum XnnGraphFusionMode {
  Diabled,
  Greedy,
  GreedySlinky,
  BypasscostModel,
}

// Code generation backends implemented for autotuning across XLA GPU and CPU.
// When adding a fission backend for a backend X, it should be named X_FISSION.
#[derive(Debug, Clone, PartialEq)]
pub enum AutotuneBackend {
  Cudnn,
  Triton,
  Cublas,
  Cublaslt,
  Rocblas,
  Hipblaslt,
  Miopen,
  CustomKernel,
  BlockLevelEmitter,
  NativeEmitter,
  LlvmKernelEmitter,
  CublasFission,
  CublasltFission,
  CustomKernelFission,
  RocblasFission,
  HipblasltFission,
}
// Debugging options for Blitz. These options may change at any time - there are
// no guarantees about backward or forward compatibility for these fields.
//
// Debug options naming and organization:
//
// 1. Backend-agnostic options: `blitz_$flag_name` - go first, and sorted
//    alphabetically by the flag name.
//
// 2. Backend-specific options: `blitz_$backend_$flag_name` - must be in the
//    corresponding backend section, and sorted alphabetically by the flag name.
//
#[derive(Debug, Clone)]
pub struct DebugOptions {
  cpu_enable_concurrency_optimized_scheduler: bool,
  cpu_enable_fast_math: bool,
  cpu_enable_fast_min_max: bool,
  cpu_fast_math_honor_division: bool,
  cpu_fast_math_honor_functions: bool,
  cpu_fast_maath_honor_infs: bool,
  cpu_fast_math_honor_nans: bool,
  cpu_use_thunk_runtime: bool,
  cpu_parallel_codegen_split_count: i64,
  cpu_prefer_vector_width: i64,
  backend_optimization_level: usize,
  eliminate_hlo_implicit_broadcast: bool,
  cpu_multi_thread_eigen: bool,
  gpu_cuda_data_dir: String,
  llvm_enable_alias_scope_metadata: bool,
  llvm_enable_noalias_metadata: bool,
  llvm_enable_invariant_load_metadata: bool,
  llvm_disable_expensive_passes: bool,
  gpu_autotune_level: usize,

  // Temporary option to allow support for both the R1 and the scalar index
  // versions of DynamicSlice and DynamicUpdateSlice. Only used for testing.
  allow_scalar_index_dynamic_ops: bool,

  // Option to emit a target-specific marker to indicate the start of a training
  // step. The location of the marker (if any) is determined by the option
  // value.
  step_marker_location: StepMarkerLocation,

  //
  // BEGIN flags controlling dumping HLO modules for debugging.
  //
  // When dumping is enabled, HLO modules dumped at the very beginning and end
  // of compilation, and optionally also during the pass pipeline.
  //
  // In general, if you set one of these flags, we will try to infer reasonable
  // defaults for the others.  For example:
  //
  //  * Setting --blitz_dump_to=/tmp/foo without specifying a format
  //    with --blitz_dump_hlo_as_* will turn on --blitz_dump_hlo_as_text.
  //
  //  * Setting --blitz_dump_hlo_as_text without specifying --blitz_dump_to will
  //    dump to stdout.
  //

  // Directory to dump into.
  dump_to: String,
  flags_reset: bool,

  // If specified, will only dump modules which match this regexp.
  dump_hlo_module_re: String,

  // If this flag is specified, will also dump HLO before and after passes that
  // match this regular expression.  Set to .* to dump before/after all passes.
  dump_hlo_pass_re: String,

  // Specifies the format that HLO is dumped in.  Multiple of these may be
  // specified.
  dump_hlo_as_text: bool,
  dump_hlo_as_proto: bool,
  dump_hlo_as_dot: bool,
  dump_hlo_as_url: bool,
  dump_hlo_as_html: bool,
  dump_fusion_visualization: bool,

  // If true, every time an HLO module is run, we will dump an HloSnapshot
  // (essentially, a serialized module plus its inputs) to the --blitz_dump_to
  // directory.
  dump_hlo_snapshots: bool,
  dump_include_timestamp: bool,
  dump_max_hlo_modules: i64,
  dump_module_metadata: bool,

  // GZip-compress protos dumped via --blitz_dump_hlo_as_proto.
  dump_compress_proto: bool,

  //
  // END flags controlling dumping HLO modules.
  //

  // Overrides for Blitz GPU's convolution layout heuristic.
  gpu_force_conv_nchw: bool,
  gpu_force_conv_nhwc: bool,

  // Paths to files with ptx code.
  gpu_ptx_file: Vec<String>,

  // Whether to dump llvm ir when compiling to ptx.
  gpu_dump_llvmir: bool,

  // Denylist for cuDNN convolutions.
  gpu_algorithm_denylist_path: String,

  // Extra parameters to pass the GPU assembler.
  gpu_asm_extra_flags: String,

  // Overrides normal multi-threaded compilation setting to use this many
  // threads. Setting to 0 (the default value) means no enforcement.
  gpu_force_compilation_parallelism: i64,

  // Guarantees run-to-run determinism.
  // This flag implies --xla_gpu_exclude_nondeterministic_ops and in addition
  // disables autotuning.
  gpu_deterministic_ops: bool,

  // Paths to files with LLVM code.
  gpu_llvm_ir_file: Vec<String>,

  gpu_disable_async_collectives: Vec<CollectiveOpType>,

  gpu_autotune_max_solutions: usize,
  gpu_generate_debug_info: bool,
  gpu_generate_line_info: bool,
  gpu_use_runtime_fusion: bool,
  dump_hlo_as_long_text: bool,
  dump_large_constants: bool,
  dump_enable_mlir_pretty_form: bool,
  dump_full_hlo_config: bool,
  gpu_unsupported_annotate_with_emitter_loc: bool,
  debug_buffer_assignment_show_max: usize,
  cpu_use_fusion_emitters: bool,
  cpu_use_xnnpack: bool,

  // Call oneDNN custom call thunks in the CPU backend
  cpu_experimental_onednn_custom_call: bool,
  cpu_experimental_xnn_graph_fusion_mode: XnnGraphFusionMode,

  // Stores the fusion types enabled for oneDNN in LibraryRewriter pass.
  cpu_experimental_onednn_fusion_type: Vec<LibraryFusionType>,

  // Stores the fusion types enabled for XNNPACK in LibraryRewriter pass.
  cpu_experimental_xnn_fusion_type: Vec<LibraryFusionType>,

  // When set, BLITZ:CPU will only generate code up to the specified ISA.
  // (It will not use newer ISAs.) Using the string format allows us to extend
  // the flag for more flexible control if necessary.
  cpu_max_isa: String,

  cpu_generate_unique_c_style_kernel_entry_points: bool,
  gpu_fused_attension_use_cudnn_rng: bool,

  // Determine the types of commands that are recorded into command buffers.
  gpu_enable_command_buffer: Vec<CommandBufferCmdType>,
  gpu_enable_cublaslt: bool,
  gpu_graph_min_graph_size: usize,
  gpu_graph_enable_concurrent_region: bool,
  cmd_buffer_trace_cache_size: usize,
  gpu_collectives_use_persistent_cliques: bool,
  gpu_enable_fast_min_max: bool,
  gpu_strict_conv_algorithm_picker: bool,
  allow_excess_precision: bool,
  force_host_platform_device_count: usize,
  gpu_all_reduce_combine_threshold_bytes: usize,
  gpu_all_gather_combine_threshold_bytes: usize,
  gpu_reduce_scatter_combine_threshold_bytes: usize,
  gpu_collective_permute_combine_threshold_bytes: usize,
  gpu_enable_all_gather_combine_by_dim: bool,
  gpu_enable_reduce_scatter_combine_by_dim: bool,
  gpu_enable_approx_costly_collectives: bool,
  gpu_enable_reassociation_for_converted_ar: bool,
  cpu_enable_xprof_traceme: bool,
  gpu_unsafe_fallback_to_driver_on_ptxas_not_found: bool,
  multiheap_size_constraint_per_heap: i64,
  detailed_logging: bool,
  enable_dumping: bool,
  gpu_enable_custom_fusions: bool,
  gpu_enable_dynamic_slice_fusion: bool,
  gpu_nccl_termination_timeout_seconds: i64,
  gpu_enable_shared_constants: bool,
  gpu_enable_nccl_user_buffers: bool,
  gpu_enable_nccl_comm_splitting: bool,
  gpu_nccl_init_max_rank_per_root_ratio: usize,
  gpu_temp_buffer_use_separate_color: bool,
  gpu_require_exclusive_lock: bool,
  gpu_redzone_scratch_max_megabytes: usize,
  gpu_redzone_padding_bytes: usize,
  gpu_shape_checks: ShapeChecks,
  dump_latency_hiding_schedule: bool,
  gpu_enable_analytical_latency_estimator: bool,
  gpu_pgle_profile_file_or_directory_path: String,
  gpu_memory_limit_slop_factor: usize,
  enable_highest_priority_async_stream: bool,
  gpu_enable_pipelined_collectives: bool,
  gpu_enable_pipelined_all_reduce: bool,
  gpu_enable_pipelined_all_gather: bool,
  gpu_enable_pipelined_reduce_scatter: bool,
  gpu_enable_pipelined_p2p: bool,
  gpu_collective_permute_decomposer_threshold: usize,
  gpu_experimental_pipeline_parallelism_opt_level: PipelineParallelismOptLevel,
  gpu_experimental_collective_cse_distance_threshold: usize,
  gpu_experimental_enable_subchannel_dequantisaction_fusion: bool,
  partitioning_algorithm: PartitioningAlgorithm,
  gpu_enable_triton_gemm: bool,
  gpu_unsupported_enable_generic_triton_emitter_for_gemms: bool,
  gpu_unsupported_enable_triton_multi_output_fusion: bool,
  gpu_establish_cudnn_int8x32_convolution_reordering: bool,
  gpu_triton_gemm_any: bool,
  gpu_verify_triton_fusion_numerics: bool,
  gpu_enable_while_loop_reduce_scatter_code_motion: bool,
  gpu_collective_inflation_factor: usize,
  llvm_force_inline_before_split: bool,
  gpu_exhaustive_tiling_search: bool,
  gpu_experimental_enable_triton_heroless_priority_fusion: bool,
  gpu_auto_spmd_partitioning_memory_budget_gb: usize,
  gpu_auto_spmd_partitioning_memory_budget_ratio: f64,
  gpu_triton_gemm_disable_reduced_precision_reduction: bool,
  gpu_unsafe_pipelined_loop_annotator: bool,
  gpu_copy_insertion_use_region_analysis: bool,
  gpu_collect_cost_model_stats: bool,
  gpu_enable_split_k_autotuning: bool,
  gpu_enable_reduction_epilogue_fusion: bool,
  gpu_enable_nccl_clique_optimization: bool,
  gpu_cublas_fallback: bool,
  gpu_cudnn_gemm_fusion_level: usize,
  enable_while_loop_double_buffering: bool,
  gpu_enable_while_loop_unrolling: WhileLoopUnrolling,
  gpu_ensure_minor_dot_contraction_dims: bool,
  gpu_filter_kernels_spilling_registers_on_autotuning: bool,
  gpu_fail_ptx_compilation_on_register_spilling: bool,
  gpu_llvm_verification_level: usize,

  // Description of the target platform in GpuTargetConfigProto format; if
  // provided, deviceless compilation is assumed, and the current device is
  // ignored.
  gpu_target_config_filename: String,
  gpu_enable_cub_radix_sort: bool,
  gpu_enable_cudnn_layer_norm: bool,
  gpu_threshold_for_windowed_einsum_mib: usize,
  gpu_operand_bytes_threshold_for_windowed_einsum: i64,
  gpu_enable_triton_hopper: bool,
  gpu_experimental_enable_dynamic_dot_search_space: bool,
  gpu_experimental_enable_fusion_block_level_rewriter: bool,
  gpu_enable_llvm_module_compilation_parallelism: bool,
  gpu_enable_libnvptxcompiler: bool,
  gpu_libnvjitlink_mode: LibNvJitLinkMode,
  gpu_nccl_collective_max_nchannels: usize,
  gpu_nccl_p2p_max_nchannels: usize,
  gpu_multi_streamed_windowed_einsum: bool,
  gpu_experimental_stream_annotation: bool,
  gpu_gemm_rewrite_size_threshold: usize,
  gpu_use_memcpy_local_p2p: bool,
  reduce_window_rewrite_base_length: usize,
  gpu_require_complete_aot_autotune_results: bool,
  gpu_enable_host_memory_offloading: bool,
  gpu_nccl_terminate_on_error: bool,
  gpu_shared_autotuning: bool,
  syntax_sugar_async_ops: bool,
  gpu_per_fusion_autotune_cache_dir: String,
  gpu_experimental_autotune_cache_mode: AutotuneCacheMode,
  gpu_experimental_autotune_backends: Vec<AutotuneBackend>,
  gpu_autotune_gemm_rtol: f64,
  enable_command_buffers_during_profiling: bool,
  gpu_cudnn_gemm_max_plans: usize,
  gpu_pgle_accuracy_checker: PgleStrictnessLevel,
  gpu_executable_warn_stuck_timeout_seconds: usize,
  gpu_executable_terminate_timeout_seconds: usize,
  gpu_experimental_collective_perf_table_path: String,
  gpu_experimental_matmul_perf_table_path: String,
  gpu_experimental_disable_binary_libraries: bool,
  ignore_channel_id: bool,
  gpu_dot_merger_threshold_mb: usize,
  enable_fasst_math: bool,
  gpu_experimental_parallel_collective_overlap_limit: usize,
  pjrt_allow_auto_layout_in_hlo: bool,
  gpu_enable_scatter_determinism_expander: bool,
  gpu_unsupported_enable_ragged_all_to_all_decomposer: bool,
  gpu_unsupported_use_ragged_all_to_all_one_shot_kernel: bool,
  gpu_unsupported_pack_dot_operands_along_k_dimension: bool,
  gpu_unsupported_enable_all_reduce_decomposer: bool,
  unsupported_crash_on_hlo_pass_fix_max_iterations: bool,
  hlo_pass_fix_detect_cycles: bool,
  gpu_experimental_enable_sync_collective_combining: bool,
  unsupported_crash_on_hlo_pass_silent_hlo_change: bool,
  unsupported_crash_on_hlo_pass_noop_change: bool,
}

impl DebugOptions {
  pub fn default() -> Self {
    DebugOptions {
      cpu_enable_concurrency_optimized_scheduler: false,
      cpu_enable_fast_math: false,
      cpu_enable_fast_min_max: false,
      cpu_fast_math_honor_division: false,
      cpu_fast_math_honor_functions: false,
      cpu_fast_maath_honor_infs: false,
      cpu_fast_math_honor_nans: false,
      cpu_use_thunk_runtime: false,
      cpu_parallel_codegen_split_count: 0,
      cpu_prefer_vector_width: 0,
      backend_optimization_level: 0,
      eliminate_hlo_implicit_broadcast: false,
      cpu_multi_thread_eigen: false,
      gpu_cuda_data_dir: "".to_string(),
      llvm_enable_alias_scope_metadata: false,
      llvm_enable_noalias_metadata: false,
      llvm_enable_invariant_load_metadata: false,
      llvm_disable_expensive_passes: false,
      gpu_autotune_level: 0,
      allow_scalar_index_dynamic_ops: false,
      step_marker_location: StepMarkerLocation::None,
      dump_to: String::new(),
      flags_reset: false,
      dump_hlo_module_re: "".to_string(),
      dump_hlo_pass_re: "".to_string(),
      dump_hlo_as_text: false,
      dump_hlo_as_proto: false,
      dump_hlo_as_dot: false,
      dump_hlo_as_url: false,
      dump_hlo_as_html: false,
      dump_fusion_visualization: false,
      dump_hlo_snapshots: false,
      dump_include_timestamp: false,
      dump_max_hlo_modules: 0,
      dump_module_metadata: false,
      dump_compress_proto: false,
      gpu_force_conv_nchw: false,
      gpu_force_conv_nhwc: false,
      gpu_ptx_file: Vec::new(),
      gpu_dump_llvmir: false,
      gpu_algorithm_denylist_path: "".to_string(),
      gpu_asm_extra_flags: "".to_string(),
      gpu_force_compilation_parallelism: 0,
      gpu_deterministic_ops: false,
      gpu_llvm_ir_file: Vec::new(),
      gpu_disable_async_collectives: Vec::new(),
      gpu_autotune_max_solutions: 0,
      gpu_generate_debug_info: false,
      gpu_generate_line_info: false,
      gpu_use_runtime_fusion: false,
      dump_hlo_as_long_text: true,
      dump_large_constants: false,
      dump_enable_mlir_pretty_form: false,
      dump_full_hlo_config: false,
      gpu_unsupported_annotate_with_emitter_loc: false,
      debug_buffer_assignment_show_max: 0,
      cpu_use_fusion_emitters: false,
      cpu_use_xnnpack: false,
      cpu_experimental_onednn_custom_call: false,
      cpu_experimental_xnn_graph_fusion_mode: XnnGraphFusionMode::Diabled,
      cpu_experimental_onednn_fusion_type: Vec::new(),
      cpu_experimental_xnn_fusion_type: Vec::new(),
      cpu_max_isa: String::new(),
      cpu_generate_unique_c_style_kernel_entry_points: false,
      gpu_fused_attension_use_cudnn_rng: false,
      gpu_enable_command_buffer: vec![
        CommandBufferCmdType::Fusion, CommandBufferCmdType::Cublas,
        CommandBufferCmdType::Cublaslt, CommandBufferCmdType::CustomCall,
        CommandBufferCmdType::Cudnn, CommandBufferCmdType::DynamicSliceFusion
      ],
      gpu_enable_cublaslt: false,
      gpu_graph_min_graph_size: 0,
      gpu_graph_enable_concurrent_region: false,
      cmd_buffer_trace_cache_size: 0,
      gpu_collectives_use_persistent_cliques: false,
      gpu_enable_fast_min_max: false,
      gpu_strict_conv_algorithm_picker: false,
      allow_excess_precision: false,
      force_host_platform_device_count: 0,
      gpu_all_reduce_combine_threshold_bytes: 0,
      gpu_all_gather_combine_threshold_bytes: 0,
      gpu_reduce_scatter_combine_threshold_bytes: 0,
      gpu_collective_permute_combine_threshold_bytes: 0,
      gpu_enable_all_gather_combine_by_dim: false,
      gpu_enable_reduce_scatter_combine_by_dim: false,
      gpu_enable_approx_costly_collectives: false,
      gpu_enable_reassociation_for_converted_ar: false,
      cpu_enable_xprof_traceme: false,
      gpu_unsafe_fallback_to_driver_on_ptxas_not_found: false,
      multiheap_size_constraint_per_heap: 0,
      detailed_logging: false,
      enable_dumping: false,
      gpu_enable_custom_fusions: false,
      gpu_enable_dynamic_slice_fusion: false,
      gpu_nccl_termination_timeout_seconds: 0,
      gpu_enable_shared_constants: false,
      gpu_enable_nccl_user_buffers: false,
      gpu_enable_nccl_comm_splitting: false,
      gpu_nccl_init_max_rank_per_root_ratio: 0,
      gpu_temp_buffer_use_separate_color: false,
      gpu_require_exclusive_lock: false,
      gpu_redzone_scratch_max_megabytes: 0,
      gpu_redzone_padding_bytes: 0,
      gpu_shape_checks: ShapeChecks::Ignore,
      dump_latency_hiding_schedule: false,
      gpu_enable_analytical_latency_estimator: false,
      gpu_pgle_profile_file_or_directory_path: "".to_string(),
      gpu_memory_limit_slop_factor: 0,
      enable_highest_priority_async_stream: false,
      gpu_enable_pipelined_collectives: false,
      gpu_enable_pipelined_all_reduce: false,
      gpu_enable_pipelined_all_gather: false,
      gpu_enable_pipelined_reduce_scatter: false,
      gpu_enable_pipelined_p2p: false,
      gpu_collective_permute_decomposer_threshold: 0,
      gpu_experimental_pipeline_parallelism_opt_level: PipelineParallelismOptLevel::Disable,
      gpu_experimental_collective_cse_distance_threshold: 0,
      gpu_experimental_enable_subchannel_dequantisaction_fusion: false,
      partitioning_algorithm: PartitioningAlgorithm::Noop,
      gpu_enable_triton_gemm: false,
      gpu_unsupported_enable_generic_triton_emitter_for_gemms: false,
      gpu_unsupported_enable_triton_multi_output_fusion: false,
      gpu_establish_cudnn_int8x32_convolution_reordering: false,
      gpu_triton_gemm_any: false,
      gpu_verify_triton_fusion_numerics: false,
      gpu_enable_while_loop_reduce_scatter_code_motion: false,
      gpu_collective_inflation_factor: 0,
      llvm_force_inline_before_split: false,
      gpu_exhaustive_tiling_search: false,
      gpu_experimental_enable_triton_heroless_priority_fusion: false,
      gpu_auto_spmd_partitioning_memory_budget_gb: 0,
      gpu_auto_spmd_partitioning_memory_budget_ratio: 0.0,
      gpu_triton_gemm_disable_reduced_precision_reduction: false,
      gpu_unsafe_pipelined_loop_annotator: false,
      gpu_copy_insertion_use_region_analysis: false,
      gpu_collect_cost_model_stats: false,
      gpu_enable_split_k_autotuning: false,
      gpu_enable_reduction_epilogue_fusion: false,
      gpu_enable_nccl_clique_optimization: false,
      gpu_cublas_fallback: false,
      gpu_cudnn_gemm_fusion_level: 0,
      enable_while_loop_double_buffering: false,
      gpu_enable_while_loop_unrolling: WhileLoopUnrolling::NoUnroll,
      gpu_ensure_minor_dot_contraction_dims: false,
      gpu_filter_kernels_spilling_registers_on_autotuning: false,
      gpu_fail_ptx_compilation_on_register_spilling: false,
      gpu_llvm_verification_level: 0,
      gpu_target_config_filename: String::new(),
      gpu_enable_cub_radix_sort: false,
      gpu_enable_cudnn_layer_norm: false,
      gpu_threshold_for_windowed_einsum_mib: 0,
      gpu_operand_bytes_threshold_for_windowed_einsum: 0,
      gpu_enable_triton_hopper: false,
      gpu_experimental_enable_dynamic_dot_search_space: false,
      gpu_experimental_enable_fusion_block_level_rewriter: false,
      gpu_enable_llvm_module_compilation_parallelism: false,
      gpu_enable_libnvptxcompiler: false,
      gpu_libnvjitlink_mode: LibNvJitLinkMode::Auto,
      gpu_nccl_collective_max_nchannels: 0,
      gpu_nccl_p2p_max_nchannels: 0,
      gpu_multi_streamed_windowed_einsum: false,
      gpu_experimental_stream_annotation: false,
      gpu_gemm_rewrite_size_threshold: 0,
      gpu_use_memcpy_local_p2p: false,
      reduce_window_rewrite_base_length: 0,
      gpu_require_complete_aot_autotune_results: false,
      gpu_enable_host_memory_offloading: false,
      gpu_nccl_terminate_on_error: false,
      gpu_shared_autotuning: false,
      syntax_sugar_async_ops: false,
      gpu_per_fusion_autotune_cache_dir: "".to_string(),
      gpu_experimental_autotune_cache_mode: AutotuneCacheMode::Unspecified,
      gpu_experimental_autotune_backends: Vec::new(),
      gpu_autotune_gemm_rtol: 0.0,
      enable_command_buffers_during_profiling: false,
      gpu_cudnn_gemm_max_plans: 0,
      gpu_pgle_accuracy_checker: PgleStrictnessLevel::Off,
      gpu_executable_warn_stuck_timeout_seconds: 0,
      gpu_executable_terminate_timeout_seconds: 0,
      gpu_experimental_collective_perf_table_path: "".to_string(),
      gpu_experimental_matmul_perf_table_path: "".to_string(),
      gpu_experimental_disable_binary_libraries: false,
      ignore_channel_id: false,
      gpu_dot_merger_threshold_mb: 0,
      enable_fasst_math: false,
      gpu_experimental_parallel_collective_overlap_limit: 0,
      pjrt_allow_auto_layout_in_hlo: false,
      gpu_enable_scatter_determinism_expander: false,
      gpu_unsupported_enable_ragged_all_to_all_decomposer: false,
      gpu_unsupported_use_ragged_all_to_all_one_shot_kernel: false,
      gpu_unsupported_pack_dot_operands_along_k_dimension: false,
      gpu_unsupported_enable_all_reduce_decomposer: false,
      unsupported_crash_on_hlo_pass_fix_max_iterations: false,
      hlo_pass_fix_detect_cycles: false,
      gpu_experimental_enable_sync_collective_combining: false,
      unsupported_crash_on_hlo_pass_silent_hlo_change: false,
      unsupported_crash_on_hlo_pass_noop_change: false,
      
    }
  }

  pub fn set_blitz_llvm_enable_alias_scope_metadata(&mut self, value: bool) {
    self.llvm_enable_alias_scope_metadata = value;
  }

  pub fn set_blitz_llvm_enable_noalias_metadata(&mut self, value: bool) {
    self.llvm_enable_noalias_metadata = value;
  }

  pub fn set_blitz_llvm_enable_invariant_load_metadata(&mut self, value: bool) {
    self.llvm_enable_invariant_load_metadata = value;
  }

  pub fn set_blitz_llvm_disable_expensive_passes(&mut self, value: bool) {
    self.llvm_disable_expensive_passes = value;
  }

  pub fn set_blitz_backend_optimization_level(&mut self, value: usize) {
    self.backend_optimization_level = value;
  }

  pub fn set_blitz_gpu_autotune_level(&mut self, value: usize) {
    self.gpu_autotune_level = value;
  }

  pub fn set_blitz_gpu_autotune_max_solutions(&mut self, value: usize) {
    self.gpu_autotune_max_solutions = value;
  }

  pub fn set_blitz_cpu_multi_thread_eigen(&mut self, value: bool) {
    self.cpu_multi_thread_eigen = value;
  }

  pub fn set_blitz_cuda_data_dir(&mut self, value: String) {
    self.gpu_cuda_data_dir = value.clone();
  }

  pub fn set_blitz_gpu_generate_debug_info(&mut self, value: bool) {
    self.gpu_generate_debug_info = value;
  }

  pub fn set_blitz_gpu_generate_line_info(&mut self, value: bool) {
    self.gpu_generate_line_info = value;
  }

  pub fn set_blitz_gpu_use_runtime_fusion(&mut self, value: bool) {
    self.gpu_use_runtime_fusion = value;
  }

  pub fn set_blitz_eliminate_hlo_implicit_broadcast(&mut self, value: bool) {
    self.eliminate_hlo_implicit_broadcast = value;
  }

  pub fn set_blitz_dump_hlo_as_html(&mut self, value: bool) {
    self.dump_hlo_as_html = value;
  }

  pub fn set_blitz_dump_fusion_visualization(&mut self, value: bool) {
    self.dump_fusion_visualization = value;
  }

  pub fn set_blitz_dump_include_timestamp(&mut self, value: bool) {
    self.dump_include_timestamp = value;
  }

  pub fn set_blitz_dump_max_hlo_modules(&mut self, value: i64) {
    self.dump_max_hlo_modules = value;
  }

  pub fn set_blitz_dump_module_metadata(&mut self, value: bool) {
    self.dump_module_metadata = value;
  }

  // dump_hlo_as_long_text
  pub fn blitz_dump_hlo_as_long_text(&self) -> bool {
    self.dump_hlo_as_long_text
  }

  pub fn set_blitz_dump_hlo_as_long_text(&mut self, value: bool) {
    self.dump_hlo_as_long_text = value;
  }

  pub fn set_blitz_dump_large_constants(&mut self, value: bool) {
    self.dump_large_constants = value;
  }

  pub fn set_blitz_dump_enable_mlir_pretty_form(&mut self, value: bool) {
    self.dump_enable_mlir_pretty_form = value;
  }

  pub fn set_blitz_dump_full_hlo_config(&mut self, value: bool) {
    self.dump_full_hlo_config = value;
  }

  pub fn set_blitz_gpu_unsupported_annotate_with_emitter_loc(&mut self, value: bool) {
    self.gpu_unsupported_annotate_with_emitter_loc = value;
  }

  pub fn set_blitz_debug_buffer_assignment_show_max(&mut self, value: usize) {
    self.debug_buffer_assignment_show_max = value;
  }

  pub fn set_blitz_cpu_use_fusion_emitters(&mut self, value: bool) {
    self.cpu_use_fusion_emitters = value;
  }

  pub fn set_blitz_cpu_use_xnnpack(&mut self, value: bool) {
    self.cpu_use_xnnpack = value;
  }

  pub fn blitz_cpu_experimental_onednn_custom_call(&self) -> bool {
    self.cpu_experimental_onednn_custom_call
  }

  pub fn set_blitz_cpu_experimental_onednn_custom_call(&mut self, value: bool) {
    self.cpu_experimental_onednn_custom_call = value;
  }

  pub fn blitz_cpu_experimental_xnn_graph_fusion_mode(&mut self) -> XnnGraphFusionMode {
    self.cpu_experimental_xnn_graph_fusion_mode.clone()
  }

  pub fn set_blitz_cpu_experimental_xnn_graph_fusion_mode(&mut self, value: XnnGraphFusionMode) {
    self.cpu_experimental_xnn_graph_fusion_mode = value.clone();
  }

  pub fn blitz_cpu_experimental_onednn_fusion_type(&self) -> Vec<LibraryFusionType> {
    let mut result = vec![];
    result.clone_from(&self.cpu_experimental_onednn_fusion_type);
    result
  }

  pub fn add_blitz_cpu_experimental_onednn_fusion_type(&mut self, value: LibraryFusionType) {
    if !self.cpu_experimental_onednn_fusion_type.contains(&value) {
      self.cpu_experimental_onednn_fusion_type.push(value);
    }
  }

  pub fn remove_blitz_cpu_experimental_onednn_fusion_type(&mut self, value: LibraryFusionType) {
    if !self.cpu_experimental_onednn_fusion_type.contains(&value) { return; }
    let mut index = 0;
    for t in &self.cpu_experimental_onednn_fusion_type {
      if *t == value { break; }
      index += 1;
    }
    self.cpu_experimental_onednn_fusion_type.remove(index);
  }

  pub fn clear_blitz_cpu_experimental_onednn_fusion_type(&mut self) {
    self.cpu_experimental_onednn_fusion_type.clear();
  }

  pub fn blitz_cpu_experimental_xnn_fusion_type(&self) -> Vec<LibraryFusionType> {
    let mut result = vec![];
    result.clone_from(&self.cpu_experimental_xnn_fusion_type);
    result
  }

  pub fn add_blitz_cpu_experimental_xnn_fusion_type(&mut self, value: LibraryFusionType) {
    if !self.cpu_experimental_xnn_fusion_type.contains(&value) {
      self.cpu_experimental_xnn_fusion_type.push(value);
    }
  }

  pub fn remove_blitz_cpu_experimental_xnn_fusion_type(&mut self, value: LibraryFusionType) {
    if !self.cpu_experimental_xnn_fusion_type.contains(&value) { return; }
    let mut index = 0;
    for t in &self.cpu_experimental_xnn_fusion_type {
      if *t == value { break; }
      index += 1;
    }
    self.cpu_experimental_xnn_fusion_type.remove(index);
  }

  pub fn clear_blitz_cpu_experimental_xnn_fusion_type(&mut self) {
    self.cpu_experimental_xnn_fusion_type.clear();
  }

  pub fn set_blitz_cpu_enable_concurrency_optimized_scheduler(&mut self, value: bool) {
    self.cpu_enable_concurrency_optimized_scheduler = value;
  }

  pub fn blitz_cpu_max_isa(&self) -> &String {
    &self.cpu_max_isa
  }

  pub fn set_blitz_cpu_max_isa(&mut self, value: String) {
    self.cpu_max_isa = value;
  }

  pub fn set_blitz_cpu_generate_unique_c_style_kernel_entry_points(&mut self, value: bool) {
    self.cpu_generate_unique_c_style_kernel_entry_points = value;
  }

  pub fn set_blitz_gpu_fused_attension_use_cudnn_rng(&mut self, value: bool) {
    self.gpu_fused_attension_use_cudnn_rng = value;
  }

  pub fn blitz_cpu_enable_fast_math(&self) -> bool {
    self.cpu_enable_fast_math
  }

  pub fn set_blitz_cpu_enable_fast_math(&mut self, value: bool) {
    self.cpu_enable_fast_math = value;
  }

  pub fn blitz_cpu_enable_platform_dependent_math(&self) -> bool {
    unimplemented!()
  }

  pub fn set_blitz_cpu_enable_platform_dependent_math(&mut self, _value: bool) {
    unimplemented!()
  }

  pub fn set_blitz_cpu_enable_fast_min_max(&mut self, value: bool) {
    self.cpu_enable_fast_min_max = value;
  }

  pub fn set_blitz_cpu_fast_math_honor_division(&mut self, value: bool) {
    self.cpu_fast_math_honor_division = value;
  }

  pub fn set_blitz_cpu_fast_math_honor_functions(&mut self, value: bool) {
    self.cpu_fast_math_honor_functions = value;
  }

  pub fn set_blitz_cpu_fast_maath_honor_infs(&mut self, value: bool) {
    self.cpu_fast_maath_honor_infs = value;
  }

  pub fn set_blitz_cpu_fast_math_honor_nans(&mut self, value: bool) {
    self.cpu_fast_math_honor_nans = value;
  }

  pub fn set_blitz_cpu_use_thunk_runtime(&mut self, value: bool) {
    self.cpu_use_thunk_runtime = value;
  }

  pub fn set_blitz_cpu_parallel_codegen_split_count(&mut self, value: i64) {
    self.cpu_parallel_codegen_split_count = value;
  }

  pub fn set_blitz_cpu_prefer_vector_width(&mut self, value: i64) {
    self.cpu_prefer_vector_width = value;
  }

  pub fn set_blitz_gpu_enable_cublaslt(&mut self, value: bool) {
    self.gpu_enable_cublaslt = value;
  }

  pub fn blitz_gpu_enable_command_buffer(&self) -> Vec<CommandBufferCmdType> {
    let mut result = vec![];
    result.clone_from(&self.gpu_enable_command_buffer);
    result
  }

  pub fn add_blitz_gpu_enable_command_buffer(&mut self, value: CommandBufferCmdType) {
    if !self.gpu_enable_command_buffer.contains(&value) {
      self.gpu_enable_command_buffer.push(value);
    }
  }

  pub fn remove_blitz_gpu_enable_command_buffer(&mut self, value: CommandBufferCmdType) {
    if !self.gpu_enable_command_buffer.contains(&value) { return; }
    let mut index = 0;
    for cmd in &self.gpu_enable_command_buffer {
      if *cmd == value { break; }
      index += 1;
    }
    self.gpu_enable_command_buffer.remove(index);
  }

  pub fn clear_blitz_gpu_enable_command_buffer(&mut self) {
    self.gpu_enable_command_buffer.clear();
  }

  pub fn set_blitz_gpu_graph_min_graph_size(&mut self, value: usize) {
    self.gpu_graph_min_graph_size = value;
  }

  pub fn set_blitz_gpu_graph_enable_concurrent_region(&mut self, value: bool) {
    self.gpu_graph_enable_concurrent_region = value;
  }

  pub fn set_blitz_cmd_buffer_trace_cache_size(&mut self, value: usize) {
    self.cmd_buffer_trace_cache_size = value;
  }

  pub fn set_blitz_gpu_collectives_use_persistent_cliques(&mut self, value: bool) {
    self.gpu_collectives_use_persistent_cliques = value;
  }

  pub fn set_blitz_gpu_enable_fast_min_max(&mut self, value: bool) {
    self.gpu_enable_fast_min_max = value;
  }

  pub fn set_blitz_gpu_strict_conv_algorithm_picker(&mut self, value: bool) {
    self.gpu_strict_conv_algorithm_picker = value;
  }

  pub fn set_blitz_allow_excess_precision(&mut self, value: bool) {
    self.allow_excess_precision = value;
  }

  pub fn set_blitz_force_host_platform_device_count(&mut self, value: usize) {
    self.force_host_platform_device_count = value;
  }

  pub fn set_blitz_gpu_all_reduce_combine_threshold_bytes(&mut self, value: usize) {
    self.gpu_all_reduce_combine_threshold_bytes = value;
  }

  pub fn set_blitz_gpu_all_gather_combine_threshold_bytes(&mut self, value: usize) {
    self.gpu_all_gather_combine_threshold_bytes = value;
  }

  pub fn set_blitz_gpu_reduce_scatter_combine_threshold_bytes(&mut self, value: usize) {
    self.gpu_reduce_scatter_combine_threshold_bytes = value;
  }

  pub fn set_blitz_gpu_collective_permute_combine_threshold_bytes(&mut self, value: usize) {
    self.gpu_collective_permute_combine_threshold_bytes = value;
  }

  // gpu_disable_async_collectives
  pub fn blitz_gpu_disable_async_collectives(&self) -> &Vec<CollectiveOpType> {
    &self.gpu_disable_async_collectives
  }

  pub fn add_blitz_gpu_disable_async_collectives(&mut self, op_type: CollectiveOpType) {
    self.gpu_disable_async_collectives.push(op_type);
  }

  pub fn clear_gpu_disable_async_collectives(&mut self) {
    self.gpu_disable_async_collectives.clear();
  }

  pub fn set_blitz_gpu_enable_all_gather_combine_by_dim(&mut self, value: bool) {
    self.gpu_enable_all_gather_combine_by_dim = value;
  }

  pub fn set_blitz_gpu_enable_reduce_scatter_combine_by_dim(&mut self, value: bool) {
    self.gpu_enable_reduce_scatter_combine_by_dim = value;
  }

  pub fn set_blitz_gpu_enable_approx_costly_collectives(&mut self, value: bool) {
    self.gpu_enable_approx_costly_collectives = value;
  }

  pub fn set_blitz_gpu_enable_reassociation_for_converted_ar(&mut self, value: bool) {
    self.gpu_enable_reassociation_for_converted_ar = value;
  }

  pub fn set_blitz_cpu_enable_xprof_traceme(&mut self, value: bool) {
    self.cpu_enable_xprof_traceme = value;
  }

  pub fn set_blitz_gpu_unsafe_fallback_to_driver_on_ptxas_not_found(&mut self, value: bool) {
    self.gpu_unsafe_fallback_to_driver_on_ptxas_not_found = value;
  }

  pub fn set_blitz_multiheap_size_constraint_per_heap(&mut self, value: i64) {
    self.multiheap_size_constraint_per_heap = value;
  }

  pub fn set_blitz_detailed_logging(&mut self, value: bool) {
    self.detailed_logging = value;
  }

  pub fn set_blitz_enable_dumping(&mut self, value: bool) {
    self.enable_dumping = value;
  }

  pub fn set_blitz_gpu_enable_custom_fusions(&mut self, value: bool) {
    self.gpu_enable_custom_fusions = value;
  }

  pub fn set_blitz_gpu_enable_dynamic_slice_fusion(&mut self, value: bool) {
    self.gpu_enable_dynamic_slice_fusion = value;
  }

  pub fn set_blitz_gpu_nccl_termination_timeout_seconds(&mut self, value: i64) {
    self.gpu_nccl_termination_timeout_seconds = value;
  }

  pub fn set_blitz_gpu_enable_shared_constants(&mut self, value: bool) {
    self.gpu_enable_shared_constants = value;
  }

  pub fn set_blitz_gpu_enable_nccl_user_buffers(&mut self, value: bool) {
    self.gpu_enable_nccl_user_buffers = value;
  }

  pub fn set_blitz_gpu_enable_nccl_comm_splitting(&mut self, value: bool) {
    self.gpu_enable_nccl_comm_splitting = value;
  }

  pub fn set_blitz_gpu_nccl_init_max_rank_per_root_ratio(&mut self, value: usize) {
    self.gpu_nccl_init_max_rank_per_root_ratio = value;
  }

  pub fn set_blitz_gpu_temp_buffer_use_separate_color(&mut self, value: bool) {
    self.gpu_temp_buffer_use_separate_color = value;
  }

  pub fn set_blitz_gpu_require_exclusive_lock(&mut self, value: bool) {
    self.gpu_require_exclusive_lock = value;
  }

  pub fn set_blitz_gpu_redzone_scratch_max_megabytes(&mut self, value: usize) {
    self.gpu_redzone_scratch_max_megabytes = value;
  }

  pub fn set_blitz_gpu_redzone_padding_bytes(&mut self, value: usize) {
    self.gpu_redzone_padding_bytes = value;
  }

  pub fn set_blitz_gpu_shape_checks(&mut self, value: ShapeChecks) {
    self.gpu_shape_checks = value.clone();
  }

  pub fn set_blitz_dump_latency_hiding_schedule(&mut self, value: bool) {
    self.dump_latency_hiding_schedule = value;
  }

  pub fn set_blitz_gpu_enable_analytical_latency_estimator(&mut self, value: bool) {
    self.gpu_enable_analytical_latency_estimator = value;
  }

  pub fn set_blitz_gpu_pgle_profile_file_or_directory_path(&mut self, value: String) {
    self.gpu_pgle_profile_file_or_directory_path = value;
  }

  pub fn set_blitz_gpu_memory_limit_slop_factor(&mut self, value: usize) {
    self.gpu_memory_limit_slop_factor = value;
  }

  pub fn set_blitz_enable_highest_priority_async_stream(&mut self, value: bool) {
    self.enable_highest_priority_async_stream = value;
  }

  pub fn set_blitz_gpu_enable_pipelined_collectives(&mut self, value: bool) {
    self.gpu_enable_pipelined_collectives = value;
  }

  pub fn set_blitz_gpu_enable_pipelined_all_reduce(&mut self, value: bool) {
    self.gpu_enable_pipelined_all_reduce = value;
  }

  pub fn set_blitz_gpu_enable_pipelined_all_gather(&mut self, value: bool) {
    self.gpu_enable_pipelined_all_gather = value;
  }

  pub fn set_blitz_gpu_enable_pipelined_reduce_scatter(&mut self, value: bool) {
    self.gpu_enable_pipelined_reduce_scatter = value;
  }

  pub fn set_blitz_gpu_enable_pipelined_p2p(&mut self, value: bool) {
    self.gpu_enable_pipelined_p2p = value;
  }

  pub fn set_blitz_gpu_collective_permute_decomposer_threshold(&mut self, value: usize) {
    self.gpu_collective_permute_decomposer_threshold = value;
  }

  pub fn set_blitz_gpu_experimental_pipeline_parallelism_opt_level(&mut self, value: PipelineParallelismOptLevel) {
    self.gpu_experimental_pipeline_parallelism_opt_level = value.clone();
  }

  pub fn set_blitz_gpu_experimental_collective_cse_distance_threshold(&mut self, value: usize) {
    self.gpu_experimental_collective_cse_distance_threshold = value;
  }

  pub fn set_blitz_gpu_experimental_enable_subchannel_dequantisaction_fusion(&mut self, value: bool) {
    self.gpu_experimental_enable_subchannel_dequantisaction_fusion = value;
  }

  pub fn set_blitz_partitioning_algorithm(&mut self, value: PartitioningAlgorithm) {
    self.partitioning_algorithm = value.clone();
  }

  pub fn set_blitz_gpu_enable_triton_gemm(&mut self, value: bool) {
    self.gpu_enable_triton_gemm = value;
  }

  pub fn set_blitz_gpu_unsupported_enable_generic_triton_emitter_for_gemms(&mut self, value: bool) {
    self.gpu_unsupported_enable_generic_triton_emitter_for_gemms = value;
  }

  pub fn set_blitz_gpu_unsupported_enable_triton_multi_output_fusion(&mut self, value: bool) {
    self.gpu_unsupported_enable_triton_multi_output_fusion = value;
  }

  pub fn set_blitz_gpu_enable_cudnn_int8x32_convolution_reordering(&mut self, value: bool) {
    self.gpu_establish_cudnn_int8x32_convolution_reordering = value;
  }

  pub fn set_blitz_gpu_triton_gemm_any(&mut self, value: bool) {
    self.gpu_triton_gemm_any = value;
  }

  pub fn set_blitz_gpu_verify_triton_fusion_numerics(&mut self, value: bool) {
    self.gpu_verify_triton_fusion_numerics = value;
  }

  pub fn set_blitz_gpu_enable_while_loop_reduce_scatter_code_motion(&mut self, value: bool) {
    self.gpu_enable_while_loop_reduce_scatter_code_motion = value;
  }

  pub fn set_blitz_gpu_collective_inflation_factor(&mut self, value: usize) {
    self.gpu_collective_inflation_factor = value;
  }

  pub fn set_blitz_llvm_force_inline_before_split(&mut self, value: bool) {
    self.llvm_force_inline_before_split = value;
  }

  pub fn set_blitz_gpu_exhaustive_tiling_search(&mut self, value: bool) {
    self.gpu_exhaustive_tiling_search = value;
  }

  pub fn set_blitz_gpu_experimental_enable_triton_heroless_priority_fusion(&mut self, value: bool) {
    self.gpu_experimental_enable_triton_heroless_priority_fusion = value;
  }

  pub fn set_blitz_gpu_auto_spmd_partitioning_memory_budget_gb(&mut self, value: usize) {
    self.gpu_auto_spmd_partitioning_memory_budget_gb = value;
  }

  pub fn set_blitz_gpu_auto_spmd_partitioning_memory_budget_ratio(&mut self, value: f64) {
    self.gpu_auto_spmd_partitioning_memory_budget_ratio = value;
  }

  pub fn set_blitz_gpu_triton_gemm_disable_reduced_precision_reduction(&mut self, value: bool) {
    self.gpu_triton_gemm_disable_reduced_precision_reduction = value;
  }

  pub fn set_blitz_gpu_unsafe_pipelined_loop_annotator(&mut self, value: bool) {
    self.gpu_unsafe_pipelined_loop_annotator = value;
  }

  pub fn set_blitz_gpu_copy_insertion_use_region_analysis(&mut self, value: bool) {
    self.gpu_copy_insertion_use_region_analysis = value;
  }

  pub fn set_blitz_gpu_collect_cost_model_stats(&mut self, value: bool) {
    self.gpu_collect_cost_model_stats = value;
  }

  pub fn set_blitz_gpu_enable_split_k_autotuning(&mut self, value: bool) {
    self.gpu_enable_split_k_autotuning = value;
  }

  pub fn set_blitz_gpu_enable_reduction_epilogue_fusion(&mut self, value: bool) {
    self.gpu_enable_reduction_epilogue_fusion = value;
  }

  pub fn set_blitz_gpu_enable_nccl_clique_optimization(&mut self, value: bool) {
    self.gpu_enable_nccl_clique_optimization = value;
  }

  pub fn set_blitz_gpu_cublas_fallback(&mut self, value: bool) {
    self.gpu_cublas_fallback = value;
  }

  pub fn set_blitz_gpu_cudnn_gemm_fusion_level(&mut self, value: usize) {
    self.gpu_cudnn_gemm_fusion_level = value;
  }

  pub fn set_blitz_enable_while_loop_double_buffering(&mut self, value: bool) {
    self.enable_while_loop_double_buffering = value;
  }

  pub fn set_blitz_gpu_enable_while_loop_unrolling(&mut self, value: WhileLoopUnrolling) {
    self.gpu_enable_while_loop_unrolling = value.clone();
  }

  pub fn set_blitz_gpu_ensure_minor_dot_contraction_dims(&mut self, value: bool) {
    self.gpu_ensure_minor_dot_contraction_dims = value;
  }

  pub fn set_blitz_gpu_filter_kernels_spilling_registers_on_autotuning(&mut self, value: bool) {
    self.gpu_filter_kernels_spilling_registers_on_autotuning = value;
  }

  pub fn set_blitz_gpu_fail_ptx_compilation_on_register_spilling(&mut self, value: bool) {
    self.gpu_fail_ptx_compilation_on_register_spilling = value;
  }

  pub fn set_blitz_gpu_llvm_verification_level(&mut self, value: usize) {
    self.gpu_llvm_verification_level = value;
  }

  // gpu_target_config_filename
  pub fn blitz_gpu_target_config_filename(&self) -> String {
    self.gpu_target_config_filename.clone()
  }

  pub fn set_blitz_gpu_target_config_filename(&mut self, value: String) {
    self.gpu_target_config_filename = value;
  }

  pub fn set_blitz_gpu_enable_cub_radix_sort(&mut self, value: bool) {
    self.gpu_enable_cub_radix_sort = value;
  }

  pub fn set_blitz_gpu_enable_cudnn_layer_norm(&mut self, value: bool) {
    self.gpu_enable_cudnn_layer_norm = value;
  }

  pub fn set_blitz_gpu_threshold_for_windowed_einsum_mib(&mut self, value: usize) {
    self.gpu_threshold_for_windowed_einsum_mib = value;
  }

  pub fn set_blitz_gpu_operand_bytes_threshold_for_windowed_einsum(&mut self, value:i64) {
    self.gpu_operand_bytes_threshold_for_windowed_einsum = value;
  }

  pub fn set_blitz_gpu_enable_triton_hopper(&mut self, value: bool) {
    self.gpu_enable_triton_hopper = value;
  }

  pub fn set_blitz_gpu_experimental_enable_dynamic_dot_search_space(&mut self, value: bool) {
    self.gpu_experimental_enable_dynamic_dot_search_space = value;
  }

  pub fn set_blitz_gpu_experimental_enable_fusion_block_level_rewriter(&mut self, value: bool) {
    self.gpu_experimental_enable_fusion_block_level_rewriter = value;
  }

  pub fn set_blitz_gpu_enable_llvm_module_compilation_parallelism(&mut self, value: bool) {
    self.gpu_enable_llvm_module_compilation_parallelism = value;
  }

  pub fn set_blitz_gpu_enable_libnvptxcompiler(&mut self, value: bool) {
    self.gpu_enable_libnvptxcompiler = value;
  }

  pub fn set_blitz_gpu_libnvjitlink_mode(&mut self, value: LibNvJitLinkMode) {
    self.gpu_libnvjitlink_mode = value.clone();
  }

  pub fn set_blitz_gpu_nccl_collective_max_channnels(&mut self, value: usize) {
    self.gpu_nccl_collective_max_nchannels = value;
  }

  pub fn set_blitz_gpu_nccl_p2p_max_nchannels(&mut self, value: usize) {
    self.gpu_nccl_p2p_max_nchannels = value;
  }

  pub fn set_blitz_gpu_multi_streamed_windowed_einsum(&mut self, value: bool) {
    self.gpu_multi_streamed_windowed_einsum = value;
  }

  pub fn set_blitz_gpu_experimental_stream_annotation(&mut self, value: bool) {
    self.gpu_experimental_stream_annotation = value;
  }

  pub fn set_blitz_gpu_gemm_rewrite_size_threshold(&mut self, value: usize) {
    self.gpu_gemm_rewrite_size_threshold = value;
  }

  pub fn set_blitz_gpu_use_memcpy_local_p2p(&mut self, value: bool) {
    self.gpu_use_memcpy_local_p2p = value;
  }

  pub fn set_blitz_reduce_window_rewrite_base_length(&mut self, value: usize) {
    self.reduce_window_rewrite_base_length = value;
  }

  pub fn set_blitz_gpu_require_complete_aot_autotune_results(&mut self, value: bool) {
    self.gpu_require_complete_aot_autotune_results = value;
  }

  pub fn set_blitz_gpu_enable_host_memory_offloading(&mut self, value: bool) {
    self.gpu_enable_host_memory_offloading = value;
  }

  pub fn set_blitz_gpu_nccl_terminate_on_error(&mut self, value: bool) {
    self.gpu_nccl_terminate_on_error = value;
  }

  pub fn set_blitz_gpu_shared_autotuning(&mut self, value: bool) {
    self.gpu_shared_autotuning = value;
  }

  pub fn set_blitz_syntax_sugar_async_ops(&mut self, value: bool) {
    self.syntax_sugar_async_ops = value;
  }

  pub fn set_blitz_gpu_per_fusion_autotune_cache_dir(&mut self, value: String) {
    self.gpu_per_fusion_autotune_cache_dir = value;
  }

  pub fn set_blitz_gpu_experimental_autotune_cache_mode(&mut self, value: AutotuneCacheMode) {
    self.gpu_experimental_autotune_cache_mode = value.clone();
  }

  pub fn blitz_gpu_experimental_autotune_backends(&self) -> Vec<AutotuneBackend> {
    let mut result = vec![];
    result.clone_from(&self.gpu_experimental_autotune_backends);
    result
  }

  pub fn add_blitz_gpu_experimental_autotune_backends(&mut self, value: AutotuneBackend) {
    if !self.gpu_experimental_autotune_backends.contains(&value) {
      self.gpu_experimental_autotune_backends.push(value);
    }
  }

  pub fn remove_blitz_gpu_experimental_autotune_backends(&mut self, value: AutotuneBackend) {
    if !self.gpu_experimental_autotune_backends.contains(&value) { return; }
    let mut index = 0;
    for ab in &self.gpu_experimental_autotune_backends {
      if *ab == value { break; }
      index += 1;
    }
    self.gpu_experimental_autotune_backends.remove(index);
  }

  pub fn clear_blitz_gpu_experimental_autotune_backends(&mut self) {
    self.gpu_experimental_autotune_backends.clear();
  }

  pub fn set_blitz_gpu_autotune_gemm_rtol(&mut self, value: f64) {
    self.gpu_autotune_gemm_rtol = value;
  }

  pub fn set_blitz_enable_command_buffers_during_profiling(&mut self, value: bool) {
    self.enable_command_buffers_during_profiling = value;
  }

  pub fn set_blitz_gpu_cudnn_gemm_max_plans(&mut self, value: usize) {
    self.gpu_cudnn_gemm_max_plans = value;
  }

  pub fn set_blitz_gpu_pgle_accuracy_checker(&mut self, value: PgleStrictnessLevel) {
    self.gpu_pgle_accuracy_checker = value.clone();
  }

  pub fn set_blitz_gpu_executable_warn_stuck_timeout_seconds(&mut self, value: usize) {
    self.gpu_executable_warn_stuck_timeout_seconds = value;
  }

  pub fn set_blitz_gpu_executable_terminate_timeout_aeconds(&mut self, value: usize) {
    self.gpu_executable_terminate_timeout_seconds = value;
  }

  pub fn set_blitz_gpu_experimental_collective_perf_table_path(&mut self, value: String) {
    self.gpu_experimental_collective_perf_table_path = value;
  }

  pub fn set_blitz_gpu_experimental_matmul_perf_table_path(&mut self, value: String) {
    self.gpu_experimental_matmul_perf_table_path = value;
  }

  pub fn set_blitz_gpu_experimental_disable_binary_libraries(&mut self, value: bool) {
    self.gpu_experimental_disable_binary_libraries = value;
  }

  pub fn set_blitz_ignore_channel_id(&mut self, value: bool) {
    self.ignore_channel_id = value;
  }

  pub fn set_blitz_gpu_dot_merger_threshold_mb(&mut self, value: usize) {
    self.gpu_dot_merger_threshold_mb = value;
  }

  pub fn set_blitz_enable_fast_math(&mut self, value: bool) {
    self.enable_fasst_math = value;
  }

  pub fn set_blitz_gpu_experimental_parallel_collective_overlap_limit(&mut self, value: usize) {
    self.gpu_experimental_parallel_collective_overlap_limit = value;
  }

  pub fn set_blitz_pjrt_allow_auto_layout_in_hlo(&mut self, value: bool) {
    self.pjrt_allow_auto_layout_in_hlo = value;
  }

  pub fn set_blitz_gpu_enable_scatter_determinism_expander(&mut self, value: bool) {
    self.gpu_enable_scatter_determinism_expander = value;
  }

  pub fn set_blitz_gpu_unsupported_enable_ragged_all_to_all_decomposer(&mut self, value: bool) {
    self.gpu_unsupported_enable_ragged_all_to_all_decomposer = value;
  }

  pub fn set_blitz_gpu_unsupported_use_ragged_all_to_all_one_shot_kernel(&mut self, value: bool) {
    self.gpu_unsupported_use_ragged_all_to_all_one_shot_kernel = value;
  }

  pub fn set_blitz_gpu_unsupported_pack_dot_operands_along_k_dimension(&mut self, value: bool) {
    self.gpu_unsupported_pack_dot_operands_along_k_dimension = value;
  }

  pub fn set_blitz_gpu_unsupported_enable_all_reduce_decomposer(&mut self, value: bool) {
    self.gpu_unsupported_enable_all_reduce_decomposer = value;
  }

  pub fn set_blitz_unsupported_crash_on_hlo_pass_fix_max_iterations(&mut self, value: bool) {
    self.unsupported_crash_on_hlo_pass_fix_max_iterations = value;
  }

  // hlo_pass_fix_detect_cycles
  pub fn blitz_hlo_pass_fix_detect_cycles(&self) -> bool {
    self.hlo_pass_fix_detect_cycles
  }

  pub fn set_blitz_hlo_pass_fix_detect_cycles(&mut self, value: bool) {
    self.hlo_pass_fix_detect_cycles = value;
  }

  pub fn blitz_hlo_evaluator_use_fasst_path(&self) -> bool {
    unimplemented!()
  }

  pub fn set_blitz_hlo_evaluator_use_fast_path(&mut self, _value: bool) {
    unimplemented!()
  }

  pub fn set_blitz_gpu_experimental_enable_sync_collective_combining(&mut self, value: bool) {
    self.gpu_experimental_enable_sync_collective_combining = value;
  }

  pub fn set_blitz_unsupported_crash_on_hlo_pass_silent_hlo_change(&mut self, value: bool) {
    self.unsupported_crash_on_hlo_pass_silent_hlo_change = value;
  }

  pub fn set_blitz_unsupported_crash_on_hlo_pass_noop_change(&mut self, value: bool) {
    self.unsupported_crash_on_hlo_pass_noop_change = value;
  }

  pub fn blitz_flags_reset(&self) -> bool {
    unimplemented!()
  }

  pub fn blitz_dump_hlo_snapshots(&self) -> bool {
    unimplemented!()
  }

  // dump_to
  pub fn blitz_dump_to(&self) -> String {
    self.dump_to.clone()
  }

  pub fn set_blitz_dump_to(&mut self, value: String) {
    self.dump_to = value;
  }
}

#[derive(Debug, Clone, PartialEq)]
enum Transpose {
  Invalid,
  NoTranspose,
  Transpose,
  Adjoint,
}

#[derive(Debug, Clone)]
pub struct TriangularSolveOptions {
  left_side: bool,
  lower: bool,
  unit_diagonal: bool,
  transpose_a: Transpose,
}

#[derive(Debug, Clone)]
pub struct CholeskyOptions {
  lower: bool
}

impl CholeskyOptions {
  pub fn set_lower(&mut self, lower: bool) {
    self.lower = lower;
  }
}

#[derive(Debug, Clone)]
pub struct GatherDimensionNumbers {}

#[derive(Debug, Clone, PartialEq)]
pub enum SparsityType {
  Invalid,
  StructuredNM,  
}

// Contains sparsity metadata for a sparse dot operation.
// The only supported type atm is structured 2:4 sparsity, which is natively
// supported on NVidia GPUs.
// Restrictions:
// - only one operand of the dot operation may be sparse;
// - only the contracting dimension may be sparse.
#[derive(Debug, Clone)]
pub struct SparsityDescriptor {
  t: SparsityType,
  // Sparse operand index (0 or 1).
  index: i64,
  // Sparse dimension number.
  dimension: i64,
  // Structured N:M sparsity (N < M).
  n: i64,
  m: i64,
}

#[derive(Debug, Clone)]
pub struct Window {
  dimensions: Vec<WindowDimension>
}

impl Window {
  pub fn default() -> Self {
    Window { dimensions: Vec::new() }
  }

  pub fn add_dimensions(&mut self) ->  &mut WindowDimension {
    let dimension = WindowDimension::default();
    self.dimensions.push(dimension);
    self.dimensions.last_mut().unwrap()
  }

  pub fn mutable_dimensions(&mut self, i: usize) -> &mut WindowDimension {
    &mut self.dimensions[i]
  }
}

#[derive(Debug, Clone)]
pub struct WindowDimension {
  // The size of the window in this dimension. For a rectangle, this would be
  // the width or height.
  size: i64,
  // The stride at which the window moves across the base area in this
  // dimension. In other words, this is the spacing between different
  // positions of the window in this dimension.
  stride: i64,
  // If positive, means the amount of padding to add to the base area at the low
  // end of this dimension; if negative, its negative means the number of
  // elements removed from the low end of this dimension. For example, in the
  // horizontal dimension of a rectangle, this would be the number of padding
  // values to pad on the left, given that indices increase when going right.
  // The actual padding value depends upon the context. Convolution pads with
  // zeros. ReduceWindow and SelectAndScatter pads with the reduce function's
  // init value.
  padding_row: i64,
  // As padding_low, but on the high end of this dimension. For example, in the
  // horizontal dimension of a rectangle, this would be the number of values to
  // pad on the right, given that indices increase when going right.
  padding_high: i64,
  // Dilation factor of the sliding window in this dimension. A dilation factor
  // of 1 means no dilation. window_dilation - 1 no-op entries ("holes") are
  // implicitly placed between each kernel element. This value may not be less
  // than 1. See documentation for convolution.
  window_dilation: i64,
  // Dilation factor of the base area in this dimension. A dilation factor of 1
  // means no dilation. base_dilation - 1 no-op entries ("holes") are implicitly
  // placed between each base area element. This value may not be less than 1.
  // See documentation for convolution.
  base_dilation: i64,
  // Window reversal means that this dimension was logically reversed before the
  // operation.
  window_reversal: bool,
}

impl WindowDimension {
  pub fn default() -> Self {
    WindowDimension {
      size: 0,
      stride: 0,
      padding_row: 0,
      padding_high: 0,
      window_dilation: 0,
      base_dilation: 0,
      window_reversal: false,
    }
  }

  pub fn set_size(&mut self, size: i64) {
    self.size = size;
  }

  pub fn set_stride(&mut self, _stride: i64) {
    unimplemented!()
  }

  pub fn set_padding_low(&mut self, _padding_row: i64) {
    unimplemented!()
  }

  pub fn set_padding_high(&mut self, _padding_high: i64) {
    unimplemented!()
  }

  pub fn set_window_dilation(&mut self, window_dilation: i64) {
    self.window_dilation = window_dilation;
  }

  pub fn set_base_dilation(&mut self, _base_dilation: i64) {
    unimplemented!()
  }

  pub fn set_window_reversal(&mut self, window_reversal: bool) {
    self.window_reversal = window_reversal;
  }
}

#[derive(Debug, Clone)]
pub struct ScatterDimensionNummbers {
  update_window_dims: i64,
  inserted_window_dims: i64,
  index_vector_dim: i64
}

impl ScatterDimensionNummbers {
  pub fn index_vector_dim(&self) -> i64 {
    self.index_vector_dim
  }

  pub fn update_window_dims(&self) -> &Vec<i64> {
    //self.update_window_dims
    unimplemented!()
  }

  pub fn scatter_dims_to_operand_dims(&self) -> &Vec<i64> {
    unimplemented!()
  }

  pub fn inserted_window_dims(&self) -> &Vec<i64> {
    unimplemented!()
  }
}

// Handle given to a user that represents a globally accessible allocation.
// Contrast this against a ComputationDataHandle, which is not globally
// accessible, since it only exists within a specific computation.
#[derive(Debug, Clone)]
pub struct GlobalDataHandle {
  handle: i64
}

impl GlobalDataHandle {
  pub fn default() -> Self {
    GlobalDataHandle { handle: 0 }    
  }

  pub fn handle(&self) -> i64 {
    self.handle
  }

  pub fn set_handle(&mut self, handle: i64) {
    self.handle = handle;
  }
}

pub struct UnregisterRequest {}

pub struct UnregisterResponse {}

pub struct DeconstructTupleRequest {
  tuple_handle: GlobalDataHandle
}

pub struct DeconstructTupleResponse {
  element_handles: Vec<GlobalDataHandle>
}

pub struct CompileRequest {}

pub struct CompileResponse {}

pub struct ExecuteRequest {}

pub struct ExecuteResponse {}

pub struct ExecuteGtaphParallelRequest {}

pub struct ExecuteParallelResponse {
  responses: Vec<ExecuteResponse>
}

pub struct GetDeviceHandlesRequest {
  device_count: i64
}

pub struct GetDeviceHandlesResponse {

}

pub struct TransferToClientRequest {}

pub struct TransferToClientResponse {}

pub struct TransferToServerRequest {}

pub struct TransferToServerResponse {}

pub struct TransferToInfeedRequest {}

pub struct TransferToInfeedResponse {}

pub struct TransferFromOutfeedRequest {}

pub struct TransferFromOutfeedResponse {}

pub struct ResetDeviceRequest {}

pub struct ResetDeviceResponse {}

pub struct ComputeConstantGraphRequest {}

pub struct ComputeConstantResponse {}

pub struct GetShapeRequest {}
pub struct GetShapeResponse {}

pub struct ComputationGraphStatsRequest {}

pub struct ComputationStatsResponse {}

pub struct CreateChannelHandleRequest {}

pub struct CreateChannelHandleResponse {}

// Handle given to a user that represents an execution that the user launched
// asynchronously on the device.
pub struct ExecutionHandle {
  handle: i64
}

impl ExecutionHandle {
  pub fn default() -> Self {
    ExecutionHandle { handle: 0 }
  }

  pub fn handle(&self) -> i64 {
    self.handle
  }

  pub fn set_handle(&mut self, handle: i64) {
    self.handle = handle;
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChannelType {
  Invalid,
  DeviceToDevice,
  DeviceToHost,
  HostToDevice,
}

// Handle given to a user to represent a channel between two computations
// via a Send and Recv instruction pair. Channels are unbuffered, so Send
// instructions will be blocked until the data is transferred.
pub struct ChannelHandle {
  t: ChannelType,
  handle: i64
}

impl ChannelHandle {
  pub fn new(t: ChannelType, handle: i64) -> Self {
    ChannelHandle { t: t, handle: handle }
  }

  pub fn handle(&self) -> i64 {
    self.handle
  }

  pub fn set_handle(&mut self, handle: i64) {
    self.handle = handle;
  }

  pub fn type_(&self) -> ChannelType {
    self.t.clone()
  }
}

// A backend-config for kWhile loops that stores the loop's trip count, if it is
// known.
//
// This is useful for backends that can implement a `for i in 0..N` loop more
// efficiently than a `while` loop.  For example, on GPUs, we can implement a
// `for i in 0..N` loop by enqueueing the kernels for the loop body N times,
// whereas implementing a `while` loop requires a host-device sync on each
// iteration.
pub struct WhileLoopBackendConfig {
  known_trip_count: usize
}

impl WhileLoopBackendConfig {
  pub fn new() -> Self {
    WhileLoopBackendConfig { known_trip_count: 0 }
  }

  pub fn set_known_trip_count(&mut self, count: usize) {
    self.known_trip_count = count;
  }
}

// Profile data from the execution of a computation.
#[derive(Debug, Clone)]
pub struct ExecutionProfile {}

impl ExecutionProfile {
  pub fn new() -> Self {
    ExecutionProfile {  }
  }

  pub fn compute_time_ns(&self) -> i64 {
    unimplemented!()
  }

  pub fn set_compute_time_ns(&mut self, _time: f64) {
    unimplemented!()
  }

  pub fn compute_and_transfer_time_ns(&self) -> f64 {
    unimplemented!()
  }

  pub fn set_executable_size_in_bytes(&mut self, _bytes: i64) {
    unimplemented!()
  }
}

// These settings control how Blitz compiles and/or runs code.  Not all settings
// will have an effect on every platform.
pub struct ExecutionOptions {
  debug_options: DebugOptions,
  num_replicas: i64,
  num_partitions: i64,
  use_spmd_partitioning: bool,
  use_shardy_partitioner: bool,
  use_auto_spmd_partitioning: bool,
  deduplicate_hlo: bool,
}

impl ExecutionOptions {
  pub fn default() -> Self {
    ExecutionOptions {
      debug_options: DebugOptions::default(),
      num_replicas: 0,
      num_partitions: 0,
      use_spmd_partitioning: false,
      use_shardy_partitioner: false,
      use_auto_spmd_partitioning: false,
      deduplicate_hlo: false,
    }
  }

  pub fn debug_options(&self) -> &'static OnceLock<DebugOptions> {
    get_debug_options_from_flags()
  }

  pub fn set_debug_options(&mut self, debug_options: DebugOptions) {
    self.debug_options = debug_options;
  }

  pub fn device_handles(&self) -> &Vec<DeviceHandle> {
    unimplemented!()
  }

  pub fn device_handles_size(&self) -> usize {
    unimplemented!()
  }

  pub fn set_num_replicas(&mut self, num_replicas: i64) {
    self.num_replicas = num_replicas;
  }

  pub fn set_num_partitions(&mut self, num_partitions: i64) {
    self.num_partitions = num_partitions;
  }

  pub fn set_use_spmd_partitioning(&mut self, use_spmd_partitioning: bool) {
    self.use_spmd_partitioning = use_spmd_partitioning;
  }

  pub fn set_use_shardy_partitioner(&mut self, use_shardy_partitioner: bool) {
    self.use_shardy_partitioner = use_shardy_partitioner;
  }

  pub fn set_use_auto_spmd_partitioning(&mut self, use_auto_spmd_partitioning: bool) {
    self.use_auto_spmd_partitioning = use_auto_spmd_partitioning;
  }

  pub fn mutable_auto_spmd_partitioning_mesh_shape(&mut self) {
    unimplemented!()
  }

  pub fn set_deduplicate_hlo(&mut self, deduplicate_hlo: bool) {
    self.deduplicate_hlo = deduplicate_hlo;
  }

  pub fn has_shape_with_output_layout(&self) -> bool {
    unimplemented!()
  }
}

// Specifies the data type used by an operation.
pub enum DataType {
  Float,
  Double,
  Half,
  Int8,
  Int32,
  ComplexFloat,
  ComplexDouble,
  BF16,
  F8E5M2,
  F8E4M3FN,
  F8E5M2FNUZ,
  F8E4M3FNUZ,
  Int64,
}

// Handle given to a user that represents a replicated virtual device. Each
// replicated device represents N physical devices for execution where N is the
// number of replicas.
pub struct DeviceHandle {
  handle: i64,
  device_count: i64
}

impl DeviceHandle {
  pub fn default() -> Self {
    DeviceHandle { handle: 0, device_count: 0 }
  }

  pub fn set_handle(&mut self, handle: i64) {
    self.handle = handle;
  }

  pub fn set_device_count(&mut self, device_count: i64) {
    self.device_count = device_count;
  }
}

pub enum CustomCallSchedule {
  None,
  Latest,
  Earliest,
}

// The version of the API used by the custom call function. The signatures for
// each version are given below.
pub enum Ersion {
  Unspecified,
  Original,
  StatusReturning,
  StatusReturningUnified,
  TypedFfi
}

// Describes the [begin, end) index range and stride for slices.
pub struct SliceDimensions {
  start: i64,
  limit: i64,
  stride: i64,
}

impl SliceDimensions {
  pub fn default() -> Self {
    SliceDimensions { start: 0, limit: 0, stride: 0 }
  }
  
  pub fn set_start(&mut self, start: i64) {
    self.start = start;
  }

  pub fn set_limit(&mut self, limit: i64) {
    self.limit = limit;
  }

  pub fn set_stride(&mut self, stride: i64) {
    self.stride = stride;
  }
}

// Describes the types of accuracy the user can request for unary ops with
// multiple implementations.
#[derive(Debug, Clone)]
pub struct ResultAccuracy {
  mode: ResultAccuracyMode,
  tolerance: ResultAccuracyTolerance,
}

impl ResultAccuracy {
  pub fn default() -> Self {
    ResultAccuracy {
      mode: ResultAccuracyMode::Default,
      tolerance: ResultAccuracyTolerance::default()
    }
  }

  pub fn set_mode(&mut self, mode: ResultAccuracyMode) {
    self.mode = mode;
  }

  pub fn mutable_tolerance(&mut self) -> &mut ResultAccuracyTolerance {
    &mut self.tolerance
  }
}

#[derive(Debug, Clone)]
pub enum ResultAccuracyMode {
  Default,
  Highest,
}

#[derive(Debug, Clone)]
pub struct ResultAccuracyTolerance {
  // Absolute error tolerance for unary instructions.
  atol: f64,
  // Relative error tolerance for unary instructions.
  rtol: f64,
  // The error in ulps (units in the last place) is relative to machine
  // precision.
  ulps: f64,
}

impl ResultAccuracyTolerance {
  pub fn default() -> Self {
    ResultAccuracyTolerance { atol: 0.0, rtol: 0.0, ulps: 0.0 }
  }

  pub fn set_atol(&mut self, atol: f64) {
    self.atol = atol;
  }

  pub fn set_rtol(&mut self, rtol: f64) {
    self.rtol = rtol;
  }

  pub fn set_ulps(&mut self, ulps: f64) {
    self.ulps = ulps;
  }
}

// A trace of a HeapSimulator run.
#[derive(Debug, Clone, PartialEq)]
pub struct HeapSimulatorTrace {

}

pub struct LogicalBuffer {
  color: i64,
}

impl LogicalBuffer {
  pub fn color(&self) -> i64 {
    self.color
  }
}

// Serialization of frame.
#[derive(Clone, PartialEq)]
pub struct StackFrame {
  file_location_id: i64,
  parent_frame_id: i64,
}

impl StackFrame {
  pub fn new(file_location_id: i64, parent_frame_id: i64) -> Self {
    StackFrame {
      file_location_id: file_location_id,
      parent_frame_id: parent_frame_id
    }
  }
}

// Serialization of file position.
#[derive(Clone, PartialEq)]
pub struct FileLocation {
  file_name_id: i64,
  function_name_id: i64,
  line: i64,
  end_line: i64,
  column: i64,
  end_column: i64,
}

impl FileLocation {
  pub fn default() -> Self {
    FileLocation {
      file_name_id: 0, function_name_id: 0, line: 0,
      end_line: 0, column: 0, end_column: 0
    }
  }

  pub fn set_file_name_id(&mut self, file_name_id: i64) {
    self.file_name_id = file_name_id;
  }

  pub fn set_function_name_id(&mut self, function_name_id: i64) {
    self.function_name_id = function_name_id;
  }

  pub fn set_line(&mut self, line: i64) {
    self.line = line;
  }

  pub fn set_end_line(&mut self, end_line: i64) {
    self.end_line = end_line;
  }

  pub fn set_column(&mut self, column: i64) {
    self.column = column;
  }

  pub fn set_end_column(&mut self, end_column: i64) {
    self.end_column = end_column;
  }
}

// Serialization of stack frames index representations.
// Stack frames index presented in four flat arrays:
// 1. File names array.
// 2. Function names array.
// 3. File location array.
// 4. Frame array.
// All reference ids in sub-protos are 1-based positions of the
// entity in the flat array.
// Ids are 1-based to keep 0 value as representation of non-set property.
#[derive(Clone, PartialEq)]
pub struct StackFrameIndex {
  file_names: Vec<String>,
  function_names: Vec<String>,
  file_locations: Vec<FileLocation>,
  stack_frams: Vec<StackFrame>
}

impl StackFrameIndex {
  pub fn default() -> Self {
    StackFrameIndex {
      file_names: Vec::new(),
      function_names: Vec::new(),
      file_locations: Vec::new(),
      stack_frams: Vec::new()
    }
  }

  pub fn add_file_name(&mut self, file_name: String) {
    self.file_names.push(file_name);
  }

  pub fn add_function_name(&mut self, function_name: String) {
    self.function_names.push(function_name);
  }

  pub fn add_file_location(&mut self, file_location: FileLocation) {
    self.file_locations.push(file_location);
  }

  pub fn add_stack_frame(&mut self, stack_frame: StackFrame) {
    self.stack_frams.push(stack_frame);
  }
}

// A [first; last] range of integers, inclusive on both ends.
pub struct IntRangeInclusive {
  first: i64,
  last: i64
}

impl IntRangeInclusive {
  pub fn new() -> Self {
    IntRangeInclusive { first: 0, last: 0 }
  }

  pub fn first(&self) -> i64 {
    self.first
  }

  pub fn last(&self) -> i64 {
    self.last
  }

  pub fn set_first(&mut self, first: i64) {
    self.first = first;
  }

  pub fn set_last(&mut self, last: i64) {
    self.last = last;
  }
}

// The version of the API used by the custom call function. The signatures for
// each version are given below.
// TODO(b/189822916): Remove this enum when all clients are migrated to the
// status-returning API.
pub enum CustomCallApiVersion {
  Unspecified,
  Original,
  StatusReturning,
  StatusReturningUnified,
  TypedFfi,
}