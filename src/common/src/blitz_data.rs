#![allow(dead_code)]

use std::{/*collections::HashMap,*/ hash::Hash};

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

pub struct PrecisionConfig {}

impl PrecisionConfig {
  pub fn operand_precision(&self) -> &Vec<Precision> {
    unimplemented!()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OpMetadata {}

impl OpMetadata {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrontendAttributes {
  //map: HashMap<String, String>
}

impl FrontendAttributes {
  pub fn new() -> Self {
    FrontendAttributes {
      //map: HashMap::new()
    }
  }

  //pub fn map(&self) -> &HashMap<String, String> {
    //&self.map
  //}

  //pub fn mutable_map(&mut self) -> &mut HashMap<String, String> {
    //&mut self.map
  //}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Statisitic {
  stat_name: String,
  stat_val: i64, // TODO: f64
}

impl Statisitic {
  pub fn new() -> Self {
    Statisitic { stat_name: "".to_string(), stat_val: 0 }
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
  pub fn new() -> Self {
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
  Unknown,
}

pub struct OpSharding {}

pub struct DotDimensionNumbers {
  lhs_contracting_dimensions: i64,
  rhs_contracting_dimensions: i64,
  lhs_batch_dimensions: i64,
  rhs_batch_dimensions: i64,
}

impl DotDimensionNumbers {
  pub fn lhs_contracting_dimensions(&self) -> i64 {
    self.lhs_contracting_dimensions
  }
}

// Describes whether all data-parallelism replicas will receive the same
// parameter data at each buffer.
pub struct ParameterReplication {
  replicated_at_leaf_buffers: Vec<bool>
}

impl ParameterReplication {
  pub fn new() -> Self {
    ParameterReplication {
      replicated_at_leaf_buffers: Vec::new()
    }
  }

  pub fn replicated_at_leaf_buffers(&self) -> &Vec<bool> {
    &self.replicated_at_leaf_buffers
  }

  pub fn add_replicated_at_leaf_buffers(&mut self, _value: bool) {
    unimplemented!()
  }
}

pub struct ConvolutionDimensionNumbers {

}

impl ConvolutionDimensionNumbers {
  pub fn new() -> Self {
    ConvolutionDimensionNumbers {  }
  }
}

pub struct PaddingConfig {}

impl PaddingConfig {
  pub fn new() -> Self {
    PaddingConfig {  }
  }
}

pub struct ReplicaGroup {}

impl ReplicaGroup {
  pub fn new() {}
}

pub enum RandomDistribution {
  Invalid,
  Uniform,
  Normal,
}

pub enum RandomAlgorithm {
  Default,
  ThreeFry,
  Philox,
}

pub enum ShapeChecks {
  Ignore,
  Runtime,
  CompileTime,
}

pub enum StepMarkerLocation {
  AtEntry,
  AtTopLevelWhileLoop,
  AtSecondLevelWhileLoop,
  None,
}

pub enum CollectiveOpType {
  NoOp,
  AllReduce,
  AllGather,
  ReduceScatter,
  CollectiveBroadcast,
  AllTotal,
  CollectivePermute,
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
pub struct DebugOptions {
  //--------------------------------------------------------------------------//
  // Blitz:CPU options.
  //--------------------------------------------------------------------------//

  // When true, Blitz:CPU uses HLO module scheduler that is optimized for
  // extracting concurrency at the cost of extra memory: we extend the live
  // ranges of temporaries to allow Blitz runtime to schedule independent
  // operations in parallel on separate threads.
  blitz_cpu_enable_concurrency_optimized_scheduler: bool,

  // When true, "unsafe" mathematical optimizations are enabled. These
  // transformations include but are not limited to:
  //
  //  - Reducing the precision of operations (e.g. using an approximate sin
  //    function, or transforming x/y into x * (1/y)).
  //  - Assuming that operations never produce or consume NaN or +/- Inf (this
  //    behavior can be adjusted using blitz_cpu_fast_math_allow_{nans|infs}).
  //  - Assuming that +0 and -0 are indistinguishable.
  blitz_cpu_enable_fast_math: bool,

  // When false we lower the Minimum and Maximum hlos in the CPU backend such
  // that Min(NotNaN, NaN) = Min(NaN, NotNaN) = NaN.  In other words, if flag
  // this is false we always propagate NaNs through Min and Max.
  //
  // Note, this does not correspond to the exact same behavior as the gpu flag
  // below!
  blitz_cpu_enable_fast_min_max: bool,

  // When blitz_cpu_enable_fast_math is true then this controls whether we forbid
  // to use the reciprocal of an argument instead of division. Ignored when
  // blitz_cpu_enable_fast_math is false.
  blitz_cpu_fast_math_honor_division: bool,

  // When blitz_cpu_enable_fast_math is true then this controls whether we forbid
  // to approximate calculations for functions. Ignored when
  // blitz_cpu_enable_fast_math is false.
  blitz_cpu_fast_math_honor_functions: bool,

  // When blitz_cpu_enable_fast_math is true then this controls whether we allow
  // operations to produce infinites. Ignored when blitz_cpu_enable_fast_math is
  // false.
  blitz_cpu_fast_maath_honor_infs: bool,

  // When blitz_cpu_enable_fast_math is true then this controls whether we allow
  // operations to produce NaNs.  Ignored when blitz_cpu_enable_fast_math is
  // false.
  blitz_cpu_fast_math_honor_nans: bool,

  // When true, Blitz:CPU uses the thunk runtime to execute compiled program.
  blitz_cpu_use_thunk_runtime: bool,

  // The number of parts to split the LLVM module into before codegen. This
  // allows Blitz to compile all parts in parallel, and resolve kernel symbols
  // from different dynamic libraries.
  blitz_cpu_parallel_codegen_split_count: i64,

  // A `prefer-vector-width` value that is passed to the LLVM backend. Default
  // value is `256` (AVX2 on x86 platforms).
  blitz_cpu_prefer_vector_width: i64,


  //--------------------------------------------------------------------------//
  // A bag of Blitz options that have to be categorized.
  //--------------------------------------------------------------------------//

  // Show addresses of HLO ops in graph dump.
  blitz_hlo_graph_addresses: bool,

  // Instrument the computation to collect per-HLO cycle counts.
  blitz_hlo_profile: bool,

  // List of HLO passes to disable/enable. These names must exactly match the
  // pass names as specified by the HloPassInterface::name() method.
  //
  // At least one of blitz_disable_hlo_passes and blitz_enable_hlo_passes_only must
  // be empty.
  blitz_disable_hlo_passes: Vec<String>,
  blitz_enable_hlo_passes_only: Vec<String>,

  // Disables all HLO passes.  Notes that some passes are necessary for
  // correctness and the invariants that must be satisfied by "fully optimized"
  // HLO are different for different devices and may change over time.  The only
  // "guarantee", such as it is, is that if you compile Blitz and dump the
  // optimized HLO for some graph, you should be able to run it again on the
  // same device with the same build of Blitz.
  blitz_disable_all_hlo_passes: bool,

  // Numerical optimization level for the Blitz compiler backend; the specific
  // interpretation of this value is left to the backends.
  blitz_backend_optimization_level: i64,

  // Embed the compiler IR as a string in the executable.
  blitz_embed_ir_in_executable: bool,

  // Eliminate implicit broadcasts when lowering user computations to HLO
  // instructions; use explicit broadcast instead.
  blitz_eliminate_hlo_implicit_broadcast: bool,

  // When generating calls to Eigen in the CPU backend, use multi-threaded Eigen
  // mode.
  blitz_cpu_multi_thread_eigen: bool,

  // Path to directory with cuda/ptx tools and libraries.
  blitz_gpu_cuda_data_dir: String,

  // Enable flush-to-zero semantics in the GPU backend.
  blitz_gpu_ftz: bool,

  // If true, in LLVM-based backends, emit !alias.scope metadata in
  // generated IR.
  blitz_llvm_enable_alias_scope_metadata: bool,

  // If true, in LLVM-based backends, emit !noalias metadata in the
  // generated IR.
  blitz_llvm_enable_noalias_metadata: bool,

  // If true, in LLVM-based backends, emit !invariant.load metadata in
  // the generated IR.
  blitz_llvm_enable_invariant_load_metadata: bool,

  // If true, a set of expensive LLVM optimization passes will not be run.
  blitz_llvm_disable_expensive_passes: bool,

  // This is used by ClientLibraryTestBase::ComputeAndCompare*. If true, the
  // computation will run n! times with all permunations of layouts for the
  // output shape in rank n. For example, with a 3D shape, all permutations of
  // the set {0, 1, 2} are tried.
  blitz_test_all_output_layouts: bool,

  // This is used by ClientLibraryTestBase::ComputeAndCompare*. If true, the
  // computation will run for all permunations of layouts of all input
  // arguments. For example, with 2 input arguments in 2D and 4D shapes, the
  // computation will run 2! * 4! times.
  blitz_test_all_input_layouts: bool,

  // Assign colors based on sharding information when generating the Graphviz
  // HLO graph.
  blitz_hlo_graph_sharding_color: bool,

  // Generate calls to MKL-DNN in the CPU backend.
  blitz_cpu_use_mkl_dnn: bool,

  // When true we lower the Minimum and Maximum hlos in the GPU backend such
  // that Min(NotNaN, NaN) = Min(NaN, NotNaN) = NotNaN.  In other words, if flag
  // this is true we don't propagate NaNs through Min and Max.
  //
  // Note, this does not correspond to the exact same behavior as the cpu flag
  // above!
  blitz_gpu_enable_fast_min_max: bool,

  // Allows xla to increase the output precision of floating point operations
  // and all floating-point conversions to be simplified, including those
  // that affect the numerics. The `FloatNormalization` pass inserts many
  // `f32 -> bf16 -> f32` conversion pairs. These are not removed by the
  // `AlgebraicSimplifier`, as that will only simplify conversions that are
  // no-ops, e.g. `bf16 -> f32 -> bf16`. Removing these improves accuracy.
  blitz_allow_excess_precision: bool,

  // Crashes the program when any kind of verification fails, instead of just
  // logging the failures. One example is cross checking of convolution results
  // among different algorithms.
  blitz_gpu_crash_on_verification_failures: bool,

  // 0:   Disable gemm and convolution autotuning.
  // 1:   Enable autotuning, but disable correctness checking.
  // 2:   Also set output buffers to random numbers during autotuning.
  // 3:   Also reset output buffers to random numbers after autotuning each
  //      algorithm.
  // 4+:  Also check for correct outputs and for out-of-bounds reads/writes.
  //
  // Default: 4.
  blitz_gpu_autotune_level: i64,

  // Force the host platform to pretend that there are these many host
  // "devices".  All these devices are backed by the same threadpool.  Defaults
  // to 1.
  //
  // Setting this to anything other than 1 can increase overhead from context
  // switching but we let the user override this behavior to help run tests on
  // the host that run models in parallel across multiple devices.
  blitz_force_host_platform_device_count: i64,

  // If set to true Blitz:GPU invokes `ptxas` with -O0 (default is -O3).
  blitz_gpu_disable_gpuasmm_optimizations: i64,

  blitz_gpu_shape_checks: ShapeChecks,

  // Enable fast math with eigen in the HLO evaluator.
  blitz_hlo_evaluator_use_fast_path: bool,

  // Temporary option to allow support for both the R1 and the scalar index
  // versions of DynamicSlice and DynamicUpdateSlice. Only used for testing.
  blitz_allow_scalar_index_dynamic_ops: bool,

  // Option to emit a target-specific marker to indicate the start of a training
  // step. The location of the marker (if any) is determined by the option
  // value.
  blitz_step_marker_location: StepMarkerLocation,

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
  blitz_dump_to: String,

  // If specified, will only dump modules which match this regexp.
  blitz_dump_hlo_module_re: String,

  // If this flag is specified, will also dump HLO before and after passes that
  // match this regular expression.  Set to .* to dump before/after all passes.
  blitz_dump_hlo_pass_re: String,

  // Specifies the format that HLO is dumped in.  Multiple of these may be
  // specified.
  blitz_dump_hlo_as_text: bool,
  blitz_dump_hlo_as_proto: bool,
  blitz_dump_hlo_as_dot: bool,
  blitz_dump_hlo_as_url: bool,

  // Dump HLO graphs as an HTML (DOT -> SVG inlined in HTML)
  blitz_dump_hlo_as_html: bool,

  // Dump the visualization of the fusion progress.
  blitz_dump_fusion_visualization: bool,

  // If true, every time an HLO module is run, we will dump an HloSnapshot
  // (essentially, a serialized module plus its inputs) to the --blitz_dump_to
  // directory.
  blitz_dump_hlo_snapshots: bool,

  // Include a timestamp in the dumped filenames.
  blitz_dump_include_timestamp: bool,

  // Max number of hlo module dumps in a directory. Set to < 0 for unbounded.
  blitz_dump_max_hlo_modules: i64,

  // Dump HloModuleMetadata as a text proto for each HLO module.
  blitz_dump_module_metadata: bool,

  // GZip-compress protos dumped via --blitz_dump_hlo_as_proto.
  blitz_dump_compress_proto: bool,

  // Dump HLO in long text format. Ignored unless blitz_dump_hlo_as_text is true.
  blitz_dump_hlo_as_long_text: bool,

  //
  // END flags controlling dumping HLO modules.
  //

  // Overrides for Blitz GPU's convolution layout heuristic.
  blitz_gpu_force_conv_nchw: bool,
  blitz_gpu_force_conv_nhwc: bool,

  // Paths to files with ptx code.
  blitz_gpu_ptx_file: Vec<String>,

  // Whether to dump llvm ir when compiling to ptx.
  blitz_gpu_dump_llvmir: bool,

  // Whether to dump mlir using pretty print form.
  blitz_dump_enable_mlir_pretty_form: bool,

  // Denylist for cuDNN convolutions.
  blitz_gpu_algorithm_denylist_path: String,

  // True if TraceMe annotations are enabled for Blitz:CPU.
  blitz_cpu_enable_xprof_traceme: bool,

  // It is usually preferable to not fallback to the driver; it can consume more
  // memory, or have bugs.
  blitz_gpu_unsafe_fallback_to_driver_on_ptxas_not_found: bool,

  // Extra parameters to pass the GPU assembler.
  blitz_gpu_asm_extra_flags: String,

  // Per-heap size constraint. New heaps will be created if per-heap max size is
  // reached.
  blitz_multiheap_size_constraint_per_heap: i64,

  // Enable detailed logging into vlog. If this is disabled, no
  // compilation summary will be printed in the end of computation.
  blitz_detailed_logging: bool,

  // Enable HLO dumping. If this is disabled, no HLO modules will be dumped.
  blitz_enable_dumping: bool,

  // Overrides normal multi-threaded compilation setting to use this many
  // threads. Setting to 0 (the default value) means no enforcement.
  blitz_gpu_force_compilation_parallelism: i64,
  blitz_gpu_enable_llvm_module_compilation_parallelism: bool,

  // Guarantees run-to-run determinism.
  // This flag implies --xla_gpu_exclude_nondeterministic_ops and in addition
  // disables autotuning.
  blitz_gpu_deterministic_ops: bool,

  // Paths to files with LLVM code.
  blitz_gpu_llvm_ir_file: Vec<String>,

  blitz_gpu_disable_async_collectives: Vec<CollectiveOpType>,

  // Size threshold (in bytes) for the GPU collective combiners.
  blitz_gpu_all_reduce_combine_threshold_bytes: i64,
  blitz_gpu_all_gather_combine_threshold_bytes: i64,
  blitz_gpu_reduce_scatter_combine_threshold_bytes: i64,

  // Combine all-gather/scatter-reduce ops with the same dimension or
  // irrespective of their dimension.
  blitz_gpu_enable_all_gather_combine_by_dim: bool,
  blitz_gpu_enable_reduce_scatter_combine_by_dim: bool,
}

impl DebugOptions {
  pub fn new() -> Self {
    DebugOptions {
      blitz_cpu_enable_concurrency_optimized_scheduler: false,
      blitz_cpu_enable_fast_math: false,
      blitz_cpu_enable_fast_min_max: false,
      blitz_cpu_fast_math_honor_division: false,
      blitz_cpu_fast_math_honor_functions: false,
      blitz_cpu_fast_maath_honor_infs: false,
      blitz_cpu_fast_math_honor_nans: false,
      blitz_cpu_use_thunk_runtime: false,
      blitz_cpu_parallel_codegen_split_count: 0,
      blitz_cpu_prefer_vector_width: 0,
      blitz_hlo_graph_addresses: false,
      blitz_hlo_profile: false,
      blitz_disable_hlo_passes: Vec::new(),
      blitz_enable_hlo_passes_only: Vec::new(),
      blitz_disable_all_hlo_passes: false,
      blitz_backend_optimization_level: 0,
      blitz_embed_ir_in_executable: false,
      blitz_eliminate_hlo_implicit_broadcast: false,
      blitz_cpu_multi_thread_eigen: false,
      blitz_gpu_cuda_data_dir: "".to_string(),
      blitz_gpu_ftz: false,
      blitz_llvm_enable_alias_scope_metadata: false,
      blitz_llvm_enable_noalias_metadata: false,
      blitz_llvm_enable_invariant_load_metadata: false,
      blitz_llvm_disable_expensive_passes: false,
      blitz_test_all_output_layouts: false,
      blitz_test_all_input_layouts: false,
      blitz_hlo_graph_sharding_color: false,
      blitz_cpu_use_mkl_dnn: false,
      blitz_gpu_enable_fast_min_max: false,
      blitz_allow_excess_precision: false,
      blitz_gpu_crash_on_verification_failures:false,
      blitz_gpu_autotune_level: 0,
      blitz_force_host_platform_device_count: 0,
      blitz_gpu_disable_gpuasmm_optimizations: 0,
      blitz_gpu_shape_checks: ShapeChecks::Ignore,
      blitz_hlo_evaluator_use_fast_path: false,
      blitz_allow_scalar_index_dynamic_ops: false,
      blitz_step_marker_location: StepMarkerLocation::None,
      blitz_dump_to: "".to_string(),
      blitz_dump_hlo_module_re: "".to_string(),
      blitz_dump_hlo_pass_re: "".to_string(),
      blitz_dump_hlo_as_text: false,
      blitz_dump_hlo_as_proto: false,
      blitz_dump_hlo_as_dot: false,
      blitz_dump_hlo_as_url: false,
      blitz_dump_hlo_as_html: false,
      blitz_dump_fusion_visualization: false,
      blitz_dump_hlo_snapshots: false,
      blitz_dump_include_timestamp: false,
      blitz_dump_max_hlo_modules: 0,
      blitz_dump_module_metadata: false,
      blitz_dump_compress_proto: false,
      blitz_dump_hlo_as_long_text: false,
      blitz_gpu_force_conv_nchw: false,
      blitz_gpu_force_conv_nhwc: false,
      blitz_gpu_ptx_file: Vec::new(),
      blitz_gpu_dump_llvmir: false,
      blitz_dump_enable_mlir_pretty_form: false,
      blitz_gpu_algorithm_denylist_path: "".to_string(),
      blitz_cpu_enable_xprof_traceme: false,
      blitz_gpu_unsafe_fallback_to_driver_on_ptxas_not_found: false,
      blitz_gpu_asm_extra_flags: "".to_string(),
      blitz_multiheap_size_constraint_per_heap: 0,
      blitz_detailed_logging: false,
      blitz_enable_dumping: false,
      blitz_gpu_force_compilation_parallelism: 0,
      blitz_gpu_enable_llvm_module_compilation_parallelism: false,
      blitz_gpu_deterministic_ops: false,
      blitz_gpu_llvm_ir_file: Vec::new(),
      blitz_gpu_disable_async_collectives: Vec::new(),
      blitz_gpu_all_reduce_combine_threshold_bytes: 0,
      blitz_gpu_all_gather_combine_threshold_bytes: 0,
      blitz_gpu_reduce_scatter_combine_threshold_bytes: 0,
      blitz_gpu_enable_all_gather_combine_by_dim: false,
      blitz_gpu_enable_reduce_scatter_combine_by_dim: false,
    }
  }

  pub fn set_blitz_cpu_enable_concurrency_optimized_scheduler(&mut self, value: bool) {
    self.blitz_cpu_enable_concurrency_optimized_scheduler = value;
  }

  pub fn set_blitz_cpu_enable_fast_math(&mut self, value: bool) {
    self.blitz_cpu_enable_fast_math = value;
  }

  pub fn set_blitz_cpu_enable_fast_min_max(&mut self, value: bool) {
    self.blitz_cpu_enable_fast_min_max = value;
  }

  pub fn set_blitz_cpu_fast_math_honor_division(&mut self, value: bool) {
    self.blitz_cpu_fast_math_honor_division = value;
  }

  pub fn set_blitz_cpu_fast_math_honor_functions(&mut self, value: bool) {
    self.blitz_cpu_fast_math_honor_functions = value;
  }

  pub fn set_blitz_cpu_fast_maath_honor_infs(&mut self, value: bool) {
    self.blitz_cpu_fast_maath_honor_infs = value;
  }

  pub fn set_blitz_cpu_fast_math_honor_nans(&mut self, value: bool) {
    self.blitz_cpu_fast_math_honor_nans = value;
  }

  pub fn set_blitz_cpu_use_thunk_runtime(&mut self, value: bool) {
    self.blitz_cpu_use_thunk_runtime = value;
  }

  pub fn set_blitz_cpu_parallel_codegen_split_count(&mut self, value: i64) {
    self.blitz_cpu_parallel_codegen_split_count = value;
  }

  pub fn set_blitz_cpu_prefer_vector_width(&mut self, value: i64) {
    self.blitz_cpu_prefer_vector_width = value;
  }

  pub fn set_blitz_hlo_graph_addresses(&mut self, value: bool) {
    self.blitz_hlo_graph_addresses = value;
  }

  pub fn set_blitz_hlo_profile(&mut self, value: bool) {
    self.blitz_hlo_profile = value;
  }
}

enum Transpose {
  Invalid,
  NoTranspose,
  Transpose,
  Adjoint,
}

pub struct TriangularSolveOptions {
  left_side: bool,
  lower: bool,
  unit_diagonal: bool,
  transpose_a: Transpose,
}

pub struct GatherDimensionNumbers {}

pub enum SparsityType {
  Invalid,
  StructuredNM,  
}

pub struct SparsityDescriptor {
  t: SparsityType,
  index: i64,
  dimension: i64,
  n: i64,
  m: i64,
}

pub struct Window {}

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
pub struct GlobalDataHandle {
  handle: i64
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
  t: ChannelType
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
pub struct ExecutionProfile {}

impl ExecutionProfile {
  pub fn new() -> Self {
    ExecutionProfile {  }
  }
}

// These settings control how Blitz compiles and/or runs code.  Not all settings
// will have an effect on every platform.
pub struct ExecutionOptions {
  debug_options: Option<DebugOptions>
}

impl ExecutionOptions {
  pub fn new() -> Self {
    ExecutionOptions { debug_options: None }
  }

  pub fn set_debug_options(&mut self, debug_options: DebugOptions) {
    self.debug_options = Some(debug_options);
  }
}