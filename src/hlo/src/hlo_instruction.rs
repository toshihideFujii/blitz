#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use common::{
  blitz_data::{
    Algorithm, ConvolutionDimensionNumbers, DotDimensionNumbers, FftType, FrontendAttributes, GatherDimensionNumbers, OpMetadata, PaddingConfig, ParameterReplication, Precision, PrecisionConfig, PrimitiveType, RandomAlgorithm, RandomDistribution, ScatterDimensionNummbers, SliceDimensions, SparsityDescriptor, Statisitic, StatisticsViz, TriangularSolveOptions, WhileLoopBackendConfig, Window
  },
  comparison_util::{ComparisonDirection, ComparisonType},
  literal::Literal,
  printer::{Printer, StringPrinter},
  shape::Shape, shape_util::ShapeUtil
};

use crate::{
  dfs_hlo_visitor_with_default::DfsHloVisitor,
  hlo_computation::HloComputation,
  hlo_domain_metadata::DomainMetadata,
  hlo_instructions::{
    HloAsyncInstruction,
    HloAsyncStartInstruction,
    HloBatchNormGradInstruction,
    HloBatchNormInferenceInstruction,
    HloBatchNormTrainingInstruction,
    //HloBroadcastInstruction,
    HloCallInstruction,
    HloCollectiveInstruction,
    HloCompareInstruction,
    //HloConcatenateInstruction,
    HloConstantInstruction,
    HloCopyStartInstruction,
    HloDynamicReshapeInstruction,
    HloDynamicSliceInstruction,
    HloDynamicUpdateSliceInstruction,
    //HloGetTupleElementInstruction,
    HloInfeedInstruction,
    HloIotaInstruction,
    HloMapInstruction,
    HloOutfeedInstruction,
    //HloParameterInstruction,
    HloRecvDoneInstruction,
    HloRecvInstruction,
    HloReduceInstruction,
    HloReducePrecisionInstruction,
    HloReshapeInstruction,
    HloSendDoneInstruction,
    HloSendInstruction,
    HloSliceInstruction,
    HloSortInstruction,
    HloTopKInstruction,
    HloTransposeInstruction
  }, hlo_module::HloModule, hlo_opcode::HloOpcode, hlo_sharding::HloSharding
};

#[derive(Clone, PartialEq)]
pub enum PrintSubcomputationMode {
  Off,
  NameOnly,
  FullBodies,
  NonSequentialBodies,
}

pub struct HloPrintOptions {
  print_operand_index_annotation_interval: i64,
  print_subcomputation_mode: PrintSubcomputationMode,
  indent_amount: i64,
  print_large_constants: bool,
  print_only_essential_constants: bool,
  print_metadata: bool,
  print_backend_config: bool,
  print_infeed_outfeed_config: bool,
  compact_operands: bool,
  include_layout_in_shapes: bool,
  print_result_shape: bool,
  print_operand_shape: bool,
  print_operand_names: bool,
  print_program_shape: bool,
  print_percent: bool,
  print_control_dependencies: bool,
  canonicalize_instruction_names: bool,
  is_in_nested_computation: bool,
  print_ids: bool,
  canonicalize_computations: bool,
  print_extra_attributes: bool,
  syntax_sugar_async_ops: bool,
  print_name_after_closing_brace: bool,
}

impl HloPrintOptions {
  pub fn new() -> Self {
    HloPrintOptions {
      print_operand_index_annotation_interval: 5,
      print_subcomputation_mode: PrintSubcomputationMode::NameOnly,
      indent_amount: 0,
      print_large_constants: false,
      print_only_essential_constants: false,
      print_metadata: true,
      print_backend_config: true,
      print_infeed_outfeed_config: true,
      compact_operands: false,
      include_layout_in_shapes: true,
      print_result_shape: true,
      print_operand_shape: true,
      print_operand_names: true,
      print_program_shape: true,
      print_percent: true,
      print_control_dependencies: true,
      canonicalize_instruction_names: false,
      is_in_nested_computation: false,
      print_ids: true,
      canonicalize_computations: false,
      print_extra_attributes: true,
      syntax_sugar_async_ops: true,
      print_name_after_closing_brace: false,
    }
  }

  pub fn default() -> Self {
    HloPrintOptions::new()
  }

  pub fn short_parsable() -> Self {
    let mut hlo_print_options = HloPrintOptions::new();
    hlo_print_options
      .set_print_large_constants(true)
      .set_print_subcomputation_mode(PrintSubcomputationMode::NameOnly)
      .set_print_metadata(false)
      .set_print_backend_config(false)
      .set_print_operand_shape(false)
      .set_print_operand_index_annotation_interval(0)
      .set_print_program_shape(false)
      .set_print_percent(false)
      .set_print_control_dependencies(false);
    hlo_print_options
  }

  pub fn canonical() -> Self {
    let mut hlo_pkrint_options = HloPrintOptions::new();
    hlo_pkrint_options
      .set_print_subcomputation_mode(PrintSubcomputationMode::FullBodies)
      .set_print_metadata(false)
      .set_print_backend_config(false)
      .set_compact_operands(false)
      .set_print_operand_names(false)
      .set_print_operand_shape(true)
      .set_print_operand_index_annotation_interval(0)
      .set_print_program_shape(false)
      .set_print_percent(false)
      .set_print_control_dependencies(false)
      .set_canonicalize_instruction_names(true);
    hlo_pkrint_options
  }

  pub fn fingerprint() -> Self {
    let mut hlo_print_options = HloPrintOptions::canonical();
    hlo_print_options
      .set_print_infeed_outfeed_config(false)
      .set_print_only_essential_constants(true)
      .set_print_ids(false)
      .set_canonicalize_computations(true);
    hlo_print_options
  }

  pub fn module_fingerprint() -> Self {
    let mut hlo_print_options = HloPrintOptions::fingerprint();
    hlo_print_options.set_print_operand_shape(false);
    hlo_print_options
  }

  pub fn set_print_large_constants(&mut self, value: bool) -> &mut HloPrintOptions {
    self.print_large_constants = value;
    self
  }

  pub fn set_print_only_essential_constants(&mut self, value: bool) -> &mut HloPrintOptions {
    self.print_only_essential_constants = value;
    self
  }

  pub fn set_print_subcomputation_mode(&mut self, value: PrintSubcomputationMode) -> &mut HloPrintOptions {
    self.print_subcomputation_mode = value;
    self
  }

  pub fn set_print_metadata(&mut self, value: bool) -> &mut HloPrintOptions {
    self.print_metadata = value;
    self
  }

  pub fn set_print_backend_config(&mut self, value: bool) -> &mut HloPrintOptions {
    self.print_backend_config = value;
    self
  }

  pub fn set_print_infeed_outfeed_config(&mut self, value: bool) -> &mut HloPrintOptions {
    self.print_infeed_outfeed_config = value;
    self
  }

  pub fn set_print_result_shape(&mut self, value: bool) -> &mut HloPrintOptions {
    self.print_result_shape = value;
    self
  }

  pub fn set_print_operand_shape(&mut self, value: bool) -> &mut HloPrintOptions {
    self.print_operand_shape = value;
    self
  }

  pub fn set_print_operand_index_annotation_interval(&mut self, value: i64) -> &mut HloPrintOptions {
    self.print_operand_index_annotation_interval = value;
    self
  }

  pub fn set_print_operand_names(&mut self, value: bool) -> &mut HloPrintOptions {
    self.print_operand_names = value;
    self
  }

  pub fn set_print_ids(&mut self, value: bool) -> &mut HloPrintOptions {
    self.print_ids = value;
    self
  }

  pub fn set_print_extra_attributes(&mut self, value: bool) -> &mut HloPrintOptions {
    self.print_extra_attributes = value;
    self
  }

  pub fn set_print_program_shape(&mut self, value: bool) -> &mut HloPrintOptions {
    self.print_program_shape = value;
    self
  }

  pub fn set_print_percent(&mut self, value: bool) -> &mut HloPrintOptions {
    self.print_percent = value;
    self
  }

  pub fn set_print_control_dependencies(&mut self, value: bool) -> &mut HloPrintOptions {
    self.print_control_dependencies = value;
    self
  }

  pub fn set_syntax_sugar_async_ops(&mut self, value: bool) -> &mut HloPrintOptions {
    self.syntax_sugar_async_ops = value;
    self
  }

  pub fn set_compact_operands(&mut self, value: bool) -> &mut HloPrintOptions {
    self.compact_operands = value;
    self
  }

  pub fn set_include_layout_in_shapes(&mut self, value: bool) -> &mut HloPrintOptions {
    self.include_layout_in_shapes = value;
    self
  }

  pub fn set_canonicalize_instruction_names(&mut self, value: bool) -> &mut HloPrintOptions {
    self.canonicalize_instruction_names = value;
    self
  }

  pub fn set_canonicalize_computations(&mut self, value: bool) -> &mut HloPrintOptions {
    self.canonicalize_computations = value;
    self
  }

  pub fn set_indent_amount(&mut self, value: i64) -> &mut HloPrintOptions {
    self.indent_amount = value;
    self
  }

  pub fn set_is_in_nested_computation(&mut self, value: bool) -> &mut HloPrintOptions {
    self.is_in_nested_computation = value;
    self
  }

  pub fn set_print_name_after_closing_brace(&mut self, value: bool) -> &mut HloPrintOptions {
    self.print_name_after_closing_brace = value;
    self
  }

  pub fn print_large_constants(&self) -> bool {
    self.print_large_constants
  }

  pub fn print_only_essential_constants(&self) -> bool {
    self.print_only_essential_constants
  }

  pub fn print_subcomputation_mode(&self) -> PrintSubcomputationMode {
    self.print_subcomputation_mode.clone()
  }

  pub fn print_metadata(&self) -> bool{
    self.print_metadata
  }

  pub fn print_backend_config(&self) -> bool {
    self.print_backend_config
  }

  pub fn print_infeed_outfeed_config(&self) -> bool {
    self.print_infeed_outfeed_config
  }

  pub fn compact_operands(&self) -> bool {
    self.compact_operands
  }

  pub fn include_layout_in_shapes(&self) -> bool {
    self.include_layout_in_shapes
  }

  pub fn print_result_shape(&self) -> bool {
    self.print_result_shape
  }

  pub fn print_operand_shape(&self) -> bool {
    self.print_operand_shape
  }

  pub fn print_operand_names(&self) -> bool {
    self.print_operand_names
  }

  pub fn print_operand_index_annotation_interval(&self) -> i64 {
    self.print_operand_index_annotation_interval
  }

  pub fn print_ids(&self) -> bool {
    self.print_ids
  }

  pub fn print_program_shape(&self) -> bool {
    self.print_program_shape
  }

  pub fn print_percent(&self) -> bool {
    self.print_percent
  }

  pub fn print_control_dependencies(&self) -> bool {
    self.print_control_dependencies
  }

  pub fn print_extra_attributes(&self) -> bool {
    self.print_extra_attributes
  }

  pub fn syntax_sugar_async_ops(&self) -> bool {
    self.syntax_sugar_async_ops
  }

  pub fn canonicalize_instruction_names(&self) -> bool {
    self.canonicalize_instruction_names
  }

  pub fn indent_amount(&self) -> i64 {
    self.indent_amount
  }

  pub fn is_in_nested_computation(&self) -> bool {
    self.is_in_nested_computation
  }

  pub fn print_name_after_closing_brace(&self) -> bool {
    self.print_name_after_closing_brace
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FusionKind {
  Loop,
  Input,
  Output,
  Custom,
}

pub const MAIN_EXECUTION_THREAD: &'static str = "main";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rare {
  called_computations: Vec<HloComputation>,
  control_predecessors: Vec<HloInstruction>,
  control_successors: Vec<HloInstruction>,
  frontend_attributes: FrontendAttributes,
  statistics_vis: StatisticsViz,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Users {
  users: Vec<HloInstruction>
}

impl Users {
  pub fn new() -> Self {
    Users { users: Vec::new() }
  }

  pub fn empty(&self) -> bool {
    self.users.is_empty()
  }

  pub fn size(&self) -> usize {
    self.users.len()
  }

  pub fn vec(&self) -> &Vec<HloInstruction> {
    &self.users
  }

  pub fn mutable_vec(&mut self) -> &mut Vec<HloInstruction> {
    &mut self.users
  }

  pub fn clear(&mut self) {
    self.users.clear()
  }

  pub fn contains(&self, _instruction: &HloInstruction) -> bool {
    false //self.users.contains(instruction)
  }

  pub fn add_user(&mut self, user: HloInstruction) {
    self.users.push(user)
  }

  pub fn maybe_remove_user() {}

  pub fn remove_user(&mut self, user: HloInstruction) {
    let mut target_index = 0;
    for i in 0..self.users.len() {
      if self.users.get(i) == Some(&user) {
        target_index = i;
      }
    }
    self.users.remove(target_index);
  }

  pub fn user_id(&self, _user: &HloInstruction) -> i64 { 0 }

  pub fn sort_instruction_users() {}
  pub fn check_invariants() {}
}

struct BackendConfigRep {}

const BODY_COMPUTATION_INDEX: usize = 0;
const CONDITION_COMPUTATION_INDEX: usize = 1;
const SELSECT_COMPUTATION_INDEX: usize = 0;
const SCATTER_COMPUTATION_INDEX: usize = 1;
const TRUE_COMPUTATION_INDEX: usize = 0;
const FALSE_COMPUTATION_INDEX: usize = 1;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HloInstruction {
  unique_id: i64,
  index_in_parent: u32,
  opcode: HloOpcode,
  is_default_config: bool,
  cleaned_up: bool,
  marked_as_dead: bool,
  operands: Vec<HloInstruction>,
  rare: Option<Rare>,
  users: Users,
  parent: Option<Box<HloComputation>>,
  sharding: Option<HloSharding>,
  shape: Shape,
  name: String,
  metadata: Option<OpMetadata>,
  collective_instruction: Option<HloCollectiveInstruction>
}

impl HloInstruction {
  pub fn default() -> Self {
    HloInstruction {
      unique_id: 0,
      index_in_parent: 0,
      opcode: HloOpcode::Abs,
      is_default_config: false,
      cleaned_up: false,
      marked_as_dead: false,
      operands: Vec::new(),
      rare: None,
      users: Users::new(),
      parent: None,
      sharding: None,
      shape: Shape::new(),
      name: "".to_string(),
      metadata: None,
      collective_instruction: None
    }
  }

  pub fn create_from_proto() {}

  // Creates a parameter-retrieving insstruction.
  pub fn create_parameter(
    _parameter_number: i64,
    _shape: &Shape,
    _name: String) -> HloInstruction //HloParameterInstruction
  {
    //HloParameterInstruction::new(parameter_number, shape, name)
    unimplemented!()
  }

  // Creates a literal constant instruction.
  pub fn create_constant<T>(literal: Literal<T>) -> HloConstantInstruction<T>
    where T: Clone + Default + PartialEq
  {
    HloConstantInstruction::new(literal)
  }

  // Creates an iota instruction.
  pub fn create_iota(shape: &Shape, iota_dimension: i64) -> HloIotaInstruction {
    HloIotaInstruction::new(shape, iota_dimension)
  }

  // Creates a top-k instruction returning the top k values along the last
  // dimension of the input operand.
  pub fn create_top_k(
    shape: &Shape,
    input: &HloInstruction,
    k: i64,
    largest: bool) -> HloTopKInstruction
  {
    HloTopKInstruction::new(shape, input, k, largest)
  }

  // Creates a get tuple element instruction.
  pub fn create_get_tuple_element_by_shape(
    _shape: &Shape,
    _operand: &HloInstruction,
    _index: i64) -> HloInstruction
  {
    //HloGetTupleElementInstruction::new(shape, operand, index)
    unimplemented!()
  }

  pub fn create_get_tuple_element(
    _operand: &HloInstruction,
    _index: i64) -> HloInstruction
  {
    //HloGetTupleElementInstruction::new(shape, operand, index)
    unimplemented!()
  }

  // Creates a random number generation instruction that fills a shape with
  // random numbers from a given distribution.
  pub fn create_rng() {}

  pub fn create_rng_bit_generator() {}
  pub fn create_rng_get_and_update_state() {}

  // Creates a unary instruction (one operand).
  pub fn create_unary(
    _shape: &Shape,
    _opcode: HloOpcode,
    _operand: &HloInstruction) {}

  pub fn create_binary() {}
  pub fn create_ternary() {}

  pub fn create_variadic(
    _shape: &Shape,
    _opcode: HloOpcode,
    _operands: Vec<HloInstruction>)
  {

  }

  // Creates a map instruction, where the computation (given by the handle) is
  // applied element-wise to every element in operands (across the operands,
  // at a given index).
  pub fn create_map(
    shape: &Shape,
    operands: Vec<HloInstruction>,
    map_computation: HloComputation) -> HloMapInstruction
  {
    HloMapInstruction::new(shape, operands, map_computation)
  }

  pub fn create_convolve() {}
  pub fn create_fft() {}

  pub fn create_async_start(
    shape: &Shape,
    operands: Vec<HloInstruction>,
    async_computation: HloComputation,
    async_execution_thread: String) -> HloAsyncStartInstruction
  {
    HloAsyncStartInstruction::new(
      shape, operands, async_computation, async_execution_thread)
  }

  pub fn create_async_update(
    shape: &Shape,
    operand: HloInstruction) -> HloAsyncInstruction
  {
    HloAsyncInstruction::new(shape, operand)
  }

  pub fn create_async_done(
    shape: &Shape,
    operand: HloInstruction) -> HloAsyncInstruction
  {
    HloAsyncInstruction::new(shape, operand)
  }

  // Creates a copy-start op, indicating whether this is a cross-program
  // prefetch or not.
  pub fn create_copy_start(
    shape: &Shape,
    operand: HloInstruction,
    cross_program_prefetch_index: Option<i64>) -> HloCopyStartInstruction
  {
    HloCopyStartInstruction::new(shape, operand, cross_program_prefetch_index)
  }

  // Creates a compare op, performing the comparison specified in direction.
  pub fn create_compare(
    shape: &Shape,
    lhs: &HloInstruction,
    rhs: &HloInstruction,
    direction: ComparisonDirection,
    t: ComparisonType) -> HloCompareInstruction
  {
    HloCompareInstruction::new(shape, lhs, rhs, direction, t)
  }

  pub fn create_triangular_solve() {}
  pub fn create_cholesky() {}
  pub fn create_dot() {}

  // Creates a reduce-precision op, where operand is the data to reduce in
  // precision, and exponent_bits and mantissa-bits descirbe the precision
  // to reduce it to.
  pub fn create_reduce_precision(
    shape: &Shape,
    operand: HloInstruction,
    exponent_bits: i32,
    mantissa_bits: i32) -> HloReducePrecisionInstruction
  {
    HloReducePrecisionInstruction::new(shape, operand, exponent_bits, mantissa_bits)
  }

  pub fn create_all_gather() {}
  pub fn create_all_gather_start() {}
  pub fn create_all_reduce() {}
  pub fn create_reduce_scatter() {}
  pub fn create_all_reduce_start() {}
  pub fn create_all_to_all() {}
  pub fn create_collective_permute() {}
  pub fn create_collective_permute_start() {}

  // Creates an instruction that returns a u32 replica ID.
  pub fn create_replica_id(_shape: &Shape) {}

  // Creates an instruction that returns a u32 partition ID.
  pub fn create_partition_id(_shape: &Shape) {}

  pub fn create_convert() {}

  // Creates a bitcast instruction, where operand is the data to convert
  // and shape is the target shape for the conversion.
  pub fn create_bitcast(_shape: &Shape, _operand: &HloInstruction) -> HloInstruction {
    unimplemented!()
  }

  pub fn create_bitcast_convert() {}
  pub fn create_stochastic_convert() {}

  // Creates an infeed instruction, which reads data of the given shape
  // from the Infeed interface of the device.
  pub fn create_infeed(
    infeed_shape: &Shape,
    token_operand: HloInstruction,
    config: String) -> HloInfeedInstruction
  {
    HloInfeedInstruction::new(infeed_shape, token_operand, config)
  }

  // Creates an outfeed instruction, which outputs data. outfeed_shape is the
  // shape of the data being outfed *not* the shape of the outfeed instruction
  // which is a token.
  pub fn create_outfeed(
    outfeed_shape: &Shape,
    operand: HloInstruction,
    token_operand: HloInstruction,
    outfeed_config: String) -> HloOutfeedInstruction
  {
    HloOutfeedInstruction::new(outfeed_shape, operand, token_operand, outfeed_config)
  }

  // Creates an asynchronous send instruction with the given channel id, which
  // initiates sending the operand data to a unique receive instruction in
  // another computation that has the same shannel id.
  pub fn create_send(
    operand: HloInstruction,
    token: HloInstruction,
    channel_id: i64,
    is_host_transfer: bool) -> HloSendInstruction
  {
    HloSendInstruction::new(operand, token, channel_id, is_host_transfer)
  }

  // Blocks until data transfer for the Send instruction (operand) is complete.
  // The operand most be Senf.
  pub fn create_send_done(
    operand: HloInstruction,
    is_host_transfer: bool) -> HloSendDoneInstruction
  {
    HloSendDoneInstruction::new(operand, is_host_transfer)
  }

  // Creates an asynchronous receive instruction with the given channel id.
  // which allocates resources to receive data of the given shape from a unique
  // send instruction in another computation that has the same channel id.
  pub fn create_recv(
    shape: &Shape,
    token: HloInstruction,
    channel_id: i64,
    is_host_transfer: bool) -> HloRecvInstruction
  {
    HloRecvInstruction::new(shape, token, channel_id, is_host_transfer)
  }

  pub fn create_recv_done(
    operand: HloRecvInstruction,
    is_host_transfer: bool) -> HloRecvDoneInstruction
  {
    HloRecvDoneInstruction::new(operand, is_host_transfer)
  }

  pub fn create_slice(
    shape: &Shape,
    operand: HloInstruction,
    start_indices: Vec<i64>,
    limit_indices: Vec<i64>,
    strides: Vec<i64>) -> HloSliceInstruction
  {
    HloSliceInstruction::new(shape, operand, start_indices, limit_indices, strides)
  }

  pub fn create_dynamic_slice(
    shape: &Shape,
    operand: HloInstruction,
    start_indices: Vec<HloInstruction>,
    slice_sizes: Vec<usize>) -> HloDynamicSliceInstruction
  {
    HloDynamicSliceInstruction::new(shape, operand, start_indices, slice_sizes)
  }

  pub fn create_dynamic_update_slice(
    shape: &Shape,
    operand: HloInstruction,
    update: HloInstruction,
    start_indices: Vec<HloInstruction>) -> HloDynamicUpdateSliceInstruction
  {
    HloDynamicUpdateSliceInstruction::new(shape, operand, update, start_indices)
  }

  pub fn create_concatenate(
    _shape: &Shape,
    _operands: Vec<HloInstruction>,
    _dimension: i64) -> HloInstruction
  {
    //HloConcatenateInstruction::new(shape, operands, dimension)
    unimplemented!()
  }

  pub fn create_reduce(
    shape: &Shape,
    _operand: HloInstruction,
    _init_value: HloInstruction,
    dimensions_to_reduce: Vec<i64>,
    reduce_computation: HloComputation) -> HloReduceInstruction
  {
    HloReduceInstruction::new(
      shape, Vec::new(), dimensions_to_reduce, reduce_computation)
  }

  pub fn create_reduce_window(
    _shape: &Shape,
    _operand: HloInstruction,
    _init_value: HloInstruction) {}

  pub fn create_batch_norm_training(
    shape: &Shape,
    operand: HloInstruction,
    scale: HloInstruction,
    offset: HloInstruction,
    epsilon: f64,
    feature_index: i64) -> HloBatchNormTrainingInstruction
  {
    HloBatchNormTrainingInstruction::new(
      shape, operand, scale, offset, epsilon, feature_index)
  }

  pub fn create_batch_norm_inference(
    shape: &Shape,
    operand: HloInstruction,
    scale: HloInstruction,
    offset: HloInstruction,
    mean: HloInstruction,
    variance: HloInstruction,
    epsilon: f64,
    feature_index: i64) -> HloBatchNormInferenceInstruction
  {
    HloBatchNormInferenceInstruction::new(
      shape, operand, scale, offset, mean, variance, epsilon, feature_index)
  }

  pub fn create_batch_norm_grad(
    shape: &Shape,
    operand: HloInstruction,
    scale: HloInstruction,
    mean: HloInstruction,
    variance: HloInstruction,
    grad_output: HloInstruction,
    epsilon: f64,
    feature_index: i64) -> HloBatchNormGradInstruction
  {
    HloBatchNormGradInstruction::new(
      shape, operand, scale, mean, variance, grad_output, epsilon, feature_index)
  }

  pub fn create_select_and_scatter() {}

  // Creates a broadcast instruction.
  pub fn create_broadcast(
    _shape: &Shape,
    _operand: HloInstruction,
    _broadcast_dimensions: Vec<i64>) -> HloInstruction //HloBroadcastInstruction
  {
    //HloBroadcastInstruction::new(shape, operand, broadcast_dimensions)
    unimplemented!()
  }

  pub fn create_broadcast_sequence() {}
  pub fn create_pad() {}

  pub fn create_reshape(
    shape: &Shape,
    operand: HloInstruction,
    inferred_dimension: i64) -> HloReshapeInstruction
  {
    HloReshapeInstruction::new(shape, operand, inferred_dimension)
  }

  pub fn create_dynamic_reshape(
    shape: &Shape,
    data_operand: HloInstruction,
    dim_sizes: Vec<HloInstruction>) -> HloDynamicReshapeInstruction
  {
    HloDynamicReshapeInstruction::new(shape, data_operand, dim_sizes)
  }

  pub fn create_transpose(
    shape: &Shape,
    operand: HloInstruction,
    dimensions: Vec<i64>) -> HloTransposeInstruction
  {
    HloTransposeInstruction::new(shape, operand, dimensions)
  }

  pub fn create_sort(
    shape: &Shape,
    dimension: i64,
    operands: Vec<HloInstruction>,
    compare: HloComputation,
    is_stable: bool) -> HloSortInstruction
  {
    HloSortInstruction::new(shape, dimension, operands, compare, is_stable)
  }

  pub fn create_while(
    _shape: &Shape,
    _condition: HloComputation,
    _body: HloComputation,
    _init: HloInstruction) {}
  
  pub fn create_conditional(
    _shape: &Shape,
    _pred: HloInstruction,
    _true_computation_arg: HloInstruction,
    _true_computation: HloComputation,
    _false_computation_arg: HloInstruction,
    _false_computation: HloComputation) {}
  
  pub fn create_gather() {}
  pub fn create_scatter() {}
  pub fn create_domain() {}
  pub fn create_fusion() {}

  pub fn create_call(
    shape: &Shape,
    called_computation_root: HloInstruction) -> HloCallInstruction
  {
    HloCallInstruction::new(shape, called_computation_root)
  }

  pub fn create_custom_call() {}
  pub fn create_tuple() {}
  pub fn create_reverse() {}
  pub fn create_after_all() {}
  pub fn create_token() {}
  pub fn create_get_dimension_size() {}
  pub fn create_set_dimension_size() {}
  pub fn create_add_dependency() {}

  // Returns true if 'execution_thread' is included in the 'execution_threads_set'.
  pub fn is_thread_included(
    execution_thread: String,
    execution_threads_set: &HashSet<String>) -> bool
  {
    execution_threads_set.is_empty() ||
    execution_threads_set.contains(&execution_thread)
  }

  // Returns the opcode for this instruction.
  pub fn opcode(&self) -> HloOpcode {
    self.opcode.clone()
  }

  // Returns whether this instruciton is the root of its parent computation.
  pub fn is_root(&self) -> bool {
    if self.parent.is_some() {
      let parent = self.parent.as_ref().unwrap().as_ref();
      return parent.root_instruction() == self;
    }
    false
  }

  // Does this instruction have no users.
  pub fn is_dead(&self) -> bool {
    self.users.empty() && !self.is_root()
  }

  // Returns true if this instruction has a side effect, irrespective of whether
  // any called computations may contain an instruction with side effects.
  pub fn has_side_effect_no_recurse(&self) -> bool {
    match self.opcode {
      HloOpcode::Send => return true,
      HloOpcode::SendDone => return true,
      HloOpcode::Recv => return true,
      HloOpcode::RecvDone => return true,
      HloOpcode::Rng => return true,
      HloOpcode::RngGetAndUpdateState => return true,
      HloOpcode::Infeed => return true,
      HloOpcode::Outfeed => return true,
      HloOpcode::AllReduceStart => return true,
      HloOpcode::AllReduceDone => return true,
      HloOpcode::AllGatherStart => return true,
      HloOpcode::AllGatherDone => return true,
      HloOpcode::CollectivePermuteStart => return true,
      HloOpcode::CollectivePermuteDone => return true,
      _ => return false
    }
  }

  // Returns true if this instruction has a side effect.
  // An instruction has a side effect if it uses certain opcodes or calls
  // a computation with a side effect.
  pub fn has_side_effect(&self) -> bool {
    if self.has_side_effect_no_recurse() {
      return true;
    }
    false
  }

  // Returns the result shape of this instruction.
  pub fn shape(&self) -> &Shape {
    &self.shape
  }

  pub fn mutable_shape(&mut self) -> &mut Shape {
    &mut self.shape
  }

  pub fn set_shape(&mut self, shape: Shape) {
    self.shape = shape;
  }

  // Returns the i-th operand to this instruction.
  pub fn operand(&self, i: usize) -> &HloInstruction {
    self.operands.get(i).unwrap()
  }

  pub fn mutable_operand(&mut self, i: usize) -> Option<&mut HloInstruction> {
    self.operands.get_mut(i)
  }

  // Returns the number of operands to this instruction.
  pub fn operand_count(&self) -> usize {
    self.operands.len()
  }

  // Returns the vector of operands of this instruction.
  pub fn operands(&self) -> &Vec<HloInstruction> {
    &self.operands
  }

  pub fn mutable_operands(&mut self) -> &mut Vec<HloInstruction> {
    &mut self.operands
  }

  // Returns the vector of unique oprands, in the same order they are found
  // within the operand vector.
  pub fn unique_operands(&self) -> Vec<HloInstruction> {
    let mut unique = vec![];
    let mut seen = HashSet::new();
    for operand in self.operands() {
      if seen.insert(operand) {
        unique.push(operand.clone());
      }
    }
    unique
  }

  // Returns the index of 'target' in the operands sequence.
  pub fn operand_index(&self, target: &HloInstruction) -> usize {
    for i in 0..self.operand_count() {
      if self.operands.get(i).unwrap() == target {
        return i;
      }
    }
    panic!("target was not an operand.");
  }

  // Returns the number of users of this instruction.
  pub fn user_count(&self) -> usize {
    self.users.size()
  }

  // Returns the users of this instruction.
  pub fn users(&self) -> &Vec<HloInstruction> {
    self.users.vec()
  }

  pub fn mutable_users(&mut self) -> &mut Vec<HloInstruction> {
    self.users.mutable_vec()
  }

  // Returns the index of the user in the users vector.
  pub fn user_id(&self, user: &HloInstruction) -> i64 {
    self.users.user_id(user)
  }

  // Returns true if this instruction is a user of 'instruction'.
  pub fn is_user_of(&self, instruction: &HloInstruction) -> bool {
    instruction.users.contains(self)
  }

  pub fn add_control_dependency_to() {}
  pub fn remove_control_dependency_to() {}

  // Drops all control predecessors and successors from this HLO instruction.
  pub fn drop_all_control_deps(&mut self) -> Result<(), String> {
    if self.has_rare() {
      for _ctrl_succ in &mut self.mutable_rare().control_successors {
        // TODO
        //let mut index = MAX;
        //for i in 0..ctrl_succ.rare().control_predecessors.len() {
          //if ctrl_succ.rare().control_predecessors[i] == *self {
            //index = i;
            //break;
          //}
        //}
        //ctrl_succ.mutable_rare().control_predecessors.remove(index);
      }
      for _ctrl_pred in &self.rare().control_predecessors {
        // TODO
      }
      self.mutable_rare().control_successors.clear();
      self.mutable_rare().control_predecessors.clear();
    }
    Ok(())
  }

  pub fn safely_drop_all_control_dependencies() {}

  // Returns if instruction has any control dependencies.
  pub fn has_control_dependencies(&self) -> bool {
    false
  }

  // Copies the control predecessors and successors on this HLO instruction to
  // 'inst'.
  pub fn copy_all_control_deps_from(&self, _inst: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  // Returns the set of control predecessors / successors of this instruction.
  pub fn control_predecessors(&self) -> &Vec<HloInstruction>{
    assert!(self.rare.is_some());
    &self.rare.as_ref().unwrap().control_predecessors
  }

  pub fn control_successors(&self) -> &Vec<HloInstruction> {
    assert!(self.rare.is_some());
    &self.rare.as_ref().unwrap().control_successors
  }

  pub fn identical() -> bool {
    false
  }

  pub fn same_op() -> bool {
    false
  }

  pub fn identical_ignoring_commutative_operand_order() {}
  pub fn identical_ignoring_channel_id_values() {}

  // Returns whether the instruction has a constant operand.
  pub fn has_constant_operand(&self) -> bool {
    for operand in &self.operands {
      if operand.is_constant() { return true; }
    }
    false
  }

  // Replaces the use of this instruction in 'user' with 'new_producer'.
  pub fn replace_use_with(
    &mut self, user: &HloInstruction, new_producer: &mut HloInstruction) -> Result<(), String>
  {
    debug_assert!(ShapeUtil::compatible_ignoring_fp_precision(self.shape(), new_producer.shape()));
    self.replace_use_with_different_shape(user, new_producer)
  }

  // Same as replace_use_with(), but new_producer can have a different shape.
  pub fn replace_use_with_different_shape(
    &mut self, user: &HloInstruction, new_producer: &mut HloInstruction) -> Result<(), String>
  {
    println!("Replacing uses of {:?} in  {:?} with {:?}.",
      self.name(), user.name(), new_producer.name());
    
    self.remove_user(user.clone()); // check

    let mut count = 0;
    for operand in user.operands() {
      if operand == self { count += 1; }
    }
    debug_assert!(count >= 0);

    // TODO
    new_producer.add_user(user.clone());
    if user.opcode() == HloOpcode::Fusion {
      let result = user.deduplicate_fusion_operands();
      if result.is_err() { return result; }
    }

    Ok(())
  }

  // Replaces the specified operand with new_operand. The old and new operands
  // must have compatible shapes ignoring floating-point precision.
  //
  // This function does NOT remove duplicated operands even if this instruction
  // is a fusion, so that the existing operand numbers do not change.
  pub fn replace_operand_with(
    &mut self, _operand_num: i64, _new_operand: HloInstruction) -> Result<(), String>
  {
    unimplemented!()
  }

  pub fn replace_operand_with_different_shape() {}

  // Decomposes fusion back to individual parts.
  pub fn defuse(&mut self) -> Result<(), String> {
    if self.opcode() != HloOpcode::Fusion {
      return Ok(());
    }
    println!("Defusiing instruction: {:?}", self.to_string_default());

    let _fused_computation = self.fused_instructions_computation();
    let mut _defused_instructions: HashMap<&HloInstruction, &mut HloInstruction> = HashMap::new();
    for _i in 0..self.operand_count() {
      //defused_instructions.insert(
        //fused_computation.parameter_instruction(i).unwrap(),
        //self.mutable_operand(i).unwrap());
    }

    // TODO

    Ok(())
  }

  // Replaces all uses of this instruction with the new producer.
  pub fn replace_all_uses_with(
    &mut self, new_producer: &mut HloInstruction, trigger: String) -> Result<(), String>
  {
    let mut print_options = HloPrintOptions::short_parsable();
    print_options.set_print_operand_shape(true);
    print_options.set_print_extra_attributes(false);

    if !ShapeUtil::compatible_ignoring_fp_precision(self.shape(), new_producer.shape()) {
      let mut result = "The shape doesn't match when replacing".to_string();
      result.push_str(&self.to_string(&print_options));
      result.push_str(" with ");
      result.push_str(&new_producer.to_string(&print_options));
      result.push_str(". ");
      result.push_str(&self.shape().to_string(false));
      result.push_str(" is not compatible with ");
      result.push_str(&new_producer.shape().to_string(false));
      result.push('\n');
      result.push_str(&trigger);
      result.push('\n');
      result.push_str(" triggered this wrong replacement.");
      return Err(result);
    }
    self.replace_all_uses_with_different_shape(new_producer)
  }

  // Same as replace_all_uses_with, but new_producer can have a different shape.
  pub fn replace_all_uses_with_different_shape(
    &mut self, new_producer: &mut HloInstruction) -> Result<(), String>
  {
    let mut new_producer_is_user = false;
    for user in self.users() {
      if user == new_producer {
        new_producer_is_user = true;
      } else {
        // TODO
        new_producer.add_user(user.clone());
        if user.opcode() == HloOpcode::Fusion {
          // TODO
        }
      }
    }
    self.users.clear();
    if new_producer_is_user {
      self.add_user(new_producer.clone());
    }
    if self.parent.is_some() && self.parent().root_instruction() == self {
      self.mutable_parent().set_root_instruction(
        new_producer.clone(), true);
    }

    Ok(())
  }

  pub fn accept(&self,
    _visitor: &dyn DfsHloVisitor,
    _call_finish_visit: bool,
    _ignore_control_predecessors: bool,
    _cross_computation: bool) -> Result<(), String>
  {
    unimplemented!()
  }

  pub fn accept_with_operand_order() {}
  pub fn visit() {}
  pub fn latest_non_gte_ancestor_and_index() {}
  pub fn latest_non_gte_ancesotr() {}
  pub fn is_effective_bitcast() {}

  // Gets/sets the to_apply HloComputation for call, map, reduce, etc.
  pub fn to_apply(&self) -> &HloComputation {
    if self.has_to_apply() {
      assert!(self.called_computations().len() == 1);
      return self.called_computations().get(0).unwrap();
    }
    unreachable!("Invalid opcode for to_apply().");
  }

  pub fn set_to_apply(&mut self, computation: HloComputation) {
    if self.has_to_apply() {
      assert!(self.called_computations().len() == 1);
      self.mutable_called_computations().insert(0, computation);
    }
    unreachable!("Invalid opcode for to_apply().");
  }

  pub fn has_to_apply(&self) -> bool {
    match self.opcode {
      HloOpcode::AllReduce => return true,
      HloOpcode::AllReduceStart => return true,
      HloOpcode::Call => return true,
      HloOpcode::Map => return true,
      HloOpcode::Reduce => return true,
      HloOpcode::ReduceScatter => return true,
      HloOpcode::ReduceWindow => return true,
      HloOpcode::Scatter => return true,
      HloOpcode::Sort => return true,
      HloOpcode::TopK => return true,
      HloOpcode::CustomCall => return self.called_computations().len() == 1,
      _ => return false
    }
  }

  // Get/sets the while_condition of while_body HloComputation for while.
  pub fn while_condition(&self) -> &HloComputation {
    assert!(self.opcode == HloOpcode::While);
    self.called_computations().get(CONDITION_COMPUTATION_INDEX).unwrap()
  }

  pub fn while_body(&self) -> &HloComputation {
    assert!(self.opcode == HloOpcode::While);
    self.called_computations().get(BODY_COMPUTATION_INDEX).unwrap()
  }

  pub fn mutable_while_body(&mut self) -> &mut HloComputation {
    unimplemented!()
  }

  pub fn set_while_condition(&mut self, computation: HloComputation) {
    assert!(self.opcode == HloOpcode::While);
    self.mutable_called_computations()
      .insert(CONDITION_COMPUTATION_INDEX, computation);
  }

  pub fn set_while_body(&mut self, computation: HloComputation) {
    assert!(self.opcode == HloOpcode::While);
    self.mutable_called_computations()
      .insert(BODY_COMPUTATION_INDEX, computation);
  }

  pub fn while_init(&self) -> &HloInstruction {
    assert!(self.opcode == HloOpcode::While);
    self.operands.get(0).unwrap()
  }

  // Get/sets the true and false HloComputation for conditional.
  pub fn true_computation(&self) -> &HloComputation {
    assert!(self.opcode == HloOpcode::Conditional);
    assert!(self.operand(0).shape().element_type() == PrimitiveType::Pred);
    self.called_computations().get(TRUE_COMPUTATION_INDEX).unwrap()
  }

  pub fn false_computation(&self) -> &HloComputation {
    assert!(self.opcode == HloOpcode::Conditional);
    assert!(self.operand(0).shape().element_type() == PrimitiveType::Pred);
    self.called_computations().get(FALSE_COMPUTATION_INDEX).unwrap()
  }

  // Gets the branch HloComputation for conditional.
  pub fn branch_computations(&self) -> &Vec<HloComputation> {
    assert!(self.opcode == HloOpcode::Conditional);
    self.called_computations()
  }

  pub fn branch_count(&self) -> usize {
    assert!(self.opcode == HloOpcode::Conditional);
    self.called_computations().len()
  }

  pub fn branch_computation(&self, b: usize) -> &HloComputation {
    assert!(self.opcode == HloOpcode::Conditional);
    assert!(b < self.called_computations().len());
    self.called_computations().get(b).unwrap()
  }

  // Sets a branch HloComputation for conditional.
  pub fn set_branch_computation(&mut self, b: usize, computation: HloComputation) {
    assert!(self.opcode == HloOpcode::Conditional);
    self.mutable_called_computations().insert(b, computation);
  }

  pub fn signature_string() {}

  // Prints a debugging string that represents this instruction.
  pub fn print(_printer: &dyn Printer, _options: &HloPrintOptions) {}

  pub fn to_string_default(&self) -> String {
    self.to_string(&HloPrintOptions::default())
  }

  // Returns a debugging string that represents this instruction.
  pub fn to_string(&self, options: &HloPrintOptions) -> String {
    let printer = StringPrinter::new();
    HloInstruction::print(&printer, options);
    "".to_string() // TODO
  }

  pub fn print_extra_attributes() {}
  pub fn extra_attributes_to_string() {}

  pub fn to_short_string(&self) -> String {
    unimplemented!()
  }

  pub fn print_with_canonical_name_map() {}
  pub fn to_proto() {}
  pub fn to_category() {}

  // Returns true if this instruction is fused, ie contained within a fusion
  // instruction.
  pub fn is_fused(&self) -> bool {
    self.parent.is_some() &&
    self.parent.as_ref().unwrap().is_fusion_computation()
  }

  pub fn is_loop_fusion(&self) -> bool {
    self.opcode == HloOpcode::Fusion && self.fusion_kind() == FusionKind::Loop
  }

  pub fn is_input_fusion(&self) -> bool {
    self.opcode == HloOpcode::Fusion && self.fusion_kind() == FusionKind::Input
  }

  pub fn is_output_fusion(&self) -> bool {
    self.opcode == HloOpcode::Fusion && self.fusion_kind() == FusionKind::Output
  }

  pub fn is_custom_fusion(&self) -> bool {
    self.opcode == HloOpcode::Fusion && self.fusion_kind() == FusionKind::Custom
  }

  // Returns true if this instruction can be legally fused into a fusion
  // instruction.
  pub fn is_fusible(&self) -> bool {
    match self.opcode {
      HloOpcode::Domain => return false,
      HloOpcode::Parameter => return false,
      HloOpcode::While => return false,
      HloOpcode::Conditional => return false,
      HloOpcode::Call => return false,

      HloOpcode::Fusion => return true,
      HloOpcode::Map => return true,
      HloOpcode::Reduce => return true,
      HloOpcode::ReduceWindow => return true,
      
      HloOpcode::Rng => return self.user_count() <= 1,
      _ => return !self.has_side_effect()
    }
  }

  pub fn is_custom_call(&self, target: String) -> bool {
    self.opcode == HloOpcode::CustomCall && self.custom_call_target() == target
  }

  // Returns the sharding applied to this operator.
  pub fn sharding(&self) -> &HloSharding {
    assert!(self.has_sharding());
    self.sharding.as_ref().unwrap()
  }

  // Returns the sharding applied to this operator, or default if none exists.
  pub fn sharding_or_default(&self, _default: &HloSharding) -> &HloSharding {
    if self.has_sharding() {
      self.sharding.as_ref().unwrap()
    } else {
      //default.clone()
      unimplemented!();
    }
  }

  // Returns the sharding unique device, if any.
  pub fn sharding_unique_device(&self) -> Option<i64> {
    if !self.has_sharding() { return None; }
    self.sharding.as_ref().unwrap().unique_device()
  }

  pub fn set_sharding(&mut self, sharding: HloSharding) {
    self.sharding = Some(sharding);
  }

  // Copies the sharding of another instruction.
  pub fn copy_sharding(&mut self, hlo: &HloInstruction) {
    self.set_sharding(hlo.sharding().clone());
  }

  pub fn set_single_sharding() {}
  pub fn set_device_sharding() {}

  // Remove any sharding from this operator.
  pub fn clear_sharding(&mut self) {
    self.sharding = None;
  }

  // Return true if this operator has a sharding assigned.
  pub fn has_sharding(&self) -> bool {
    self.sharding.is_some()
  }

  // Checks whether the instruction has compatible sharding with the other
  // instruction.
  pub fn has_compatible_sharding(&self, other: &HloInstruction) -> bool {
    if !self.has_sharding() {
      return !other.has_sharding();
    }
    if other.has_sharding() {
      return self.sharding() == other.sharding();
    }
    false
  }

  pub fn setup_derived_instruction() {}
  pub fn clone_with_new_shape() {}

  // Clones the HLO instruction as above but with new shape and operands.
  pub fn clone_with_new_opereands(
    &self, _shape: &Shape, _new_operands: &Vec<HloInstruction>) -> HloInstruction
  {
    unimplemented!()
  }

  // Returns the computations this instruction directly calls (if any).
  pub fn called_computations(&self) -> &Vec<HloComputation> {
    assert!(self.rare.is_some());
    &self.rare.as_ref().unwrap().called_computations
  }

  pub fn mutable_called_computations(&mut self) -> &mut Vec<HloComputation> {
    assert!(self.rare.is_some());
    &mut self.rare.as_mut().unwrap().called_computations
  }

  pub fn has_called_computations(&self) -> bool {
    self.has_rare() && !self.called_computations().is_empty()
  }

  pub fn might_have_called_computations() {}
  pub fn replace_called_computations() {}

  // Clears out the called computations.
  pub fn clear_called_computations(&mut self) {
    if self.has_rare() {
      self.mutable_rare().called_computations.clear();
    }
  }

  // Returns true if this instruction performs an elementwise operation on
  // 'operand_idx' -th operand.
  pub fn is_elementwise_on_operand(&self, operand_idx: i64) -> bool {
    self.is_elementwise_impl(Some(operand_idx))
  }

  // Returns true if this instruction is elementwise on all its operands.
  pub fn is_elementwise(&self) -> bool {
    self.is_elementwise_impl(None)
  }

  pub fn is_op_elementwise(opcode: &HloOpcode) -> bool {
    match opcode {
      // Unary elementwise operations.
      HloOpcode::Abs => return true,
      HloOpcode::RoundNearestAfz => return true,
      HloOpcode::RoundNearestEven => return true,
      HloOpcode::Ceil => return true,
      HloOpcode::Clz => return true,
      HloOpcode::Convert => return true,
      HloOpcode::BitcastConvert => return true,
      HloOpcode::Copy => return true,
      HloOpcode::Erf => return true,
      HloOpcode::Exp => return true,
      HloOpcode::Expm1 => return true,
      HloOpcode::Floor => return true,
      HloOpcode::Imag => return true,
      HloOpcode::IsFinite => return true,
      HloOpcode::Log => return true,
      HloOpcode::Log1p => return true,
      HloOpcode::Not => return true,
      HloOpcode::Negate => return true,
      HloOpcode::PopulationCount => return true,
      HloOpcode::Real => return true,
      HloOpcode::ReducePrecision => return true,
      HloOpcode::Rsqrt => return true,
      HloOpcode::Sign => return true,
      HloOpcode::Sin => return true,
      HloOpcode::Sqrt => return true,
      HloOpcode::Cbrt => return true,
      HloOpcode::Tan => return true,
      HloOpcode::Tanh => return true,

      // Binary elementwise operations.
      HloOpcode::Add => return true,
      HloOpcode::Atan2 => return true,
      HloOpcode::Compare => return true,
      HloOpcode::Complex => return true,
      HloOpcode::Divide => return true,
      HloOpcode::Maximum => return true,
      HloOpcode::Minimum => return true,
      HloOpcode::Multiply => return true,
      HloOpcode::Power => return true,
      HloOpcode::Remainder => return true,
      HloOpcode::Subtract => return true,
      HloOpcode::And => return true,
      HloOpcode::Or => return true,
      HloOpcode::Xor => return true,
      HloOpcode::ShiftLeft => return true,
      HloOpcode::ShiftRightArithmetic => return true,
      HloOpcode::ShiftRightLogical => return true,
      HloOpcode::StochasticConvert => return true,

      // Ternary elementwise operations.
      HloOpcode::Select => return true,
      HloOpcode::Clamp => return true,
      _ => return false
    }
  }

  // Returns true if this instruction is a cross module all-reduce instruction. 
  pub fn is_cross_module_all_reduce(&self) -> bool {
    if self.opcode == HloOpcode::AllReduce || self.opcode == HloOpcode::AllReduceStart {
      return self.channel_id() != None;
    } else if self.opcode == HloOpcode::AllReduceDone {
      assert_eq!(self.operand_count(), 1);
      let operand = self.operand(0);
      assert_eq!(operand.opcode(), HloOpcode::AllReduceStart);
      return operand.channel_id() != None;
    }
    false
  }

  // Returns true if this is a cross-replica all-reduce instruction.
  pub fn is_cross_replica_all_reduce(&self) -> bool {
    if self.opcode == HloOpcode::AllReduce || self.opcode == HloOpcode::AllReduceStart {
      return self.channel_id() == None;
    } else if self.opcode == HloOpcode::AllReduceDone {
      assert_eq!(self.operand_count(), 1);
      let operand = self.operand(0);
      assert_eq!(operand.opcode(), HloOpcode::AllReduceStart);
      return operand.channel_id() == None;
    }
    false
  }

  // Returns true if this instruction is bunary and elementwise.
  pub fn is_elementwise_binary(&self) -> bool {
    self.is_elementwise() && self.operand_count() == 2
  }

  pub fn reuse_operand_elements() {}
  pub fn operand_indices() {}
  pub fn reshape_merely_inserts_or_deletes_1_sized_dimensions() {}

  // Gets the string identifier for this instruction.
  pub fn name(&self) -> String {
    self.name.clone()
  }

  pub fn set_name(&mut self, _name: String) {
    unimplemented!()
  }

  // Sets the string identifier for this instruction. Name will be sanitized to
  // match the regexp "[a-zA-Z_][a-zA-Z0-9.-]*".
  pub fn set_and_sanitize_name(&mut self, _name: String) {
      
  }

  pub fn uniquify_name() {}

  // Clear the unique ID of the instruction so that it can be re-assigned, such
  // as for the purpose of compacting the instruction unique IDs.
  pub fn clear_unique_id_internal(&mut self) {
    self.unique_id = -1;
  }

  // Set the unique id for this instruction to 'id'.
  pub fn set_unique_id(&mut self, id: i64) {
    assert_eq!(self.unique_id, -1);
    assert!(id >= 0);
    self.unique_id = id;
  }

  // Returns the unique ID assigned to this node.
  pub fn unique_id(&self) -> i64 {
    self.unique_id
  }

  pub fn backend_config() {}

  pub fn set_backend_config(&mut self, _config: WhileLoopBackendConfig) {
    unimplemented!()
  }

  pub fn preserve_layout() {}
  pub fn has_backend_config() {}
  pub fn clear_backend_config() {}
  pub fn copy_backend_config_from() {}

    // Adds or overrides a single attribute in the HloInstruction.
  pub fn set_frontend_attribute(&mut self, key: String, value: String) {
    self.mutable_rare().frontend_attributes.set_attribute(key, value);
  }

  pub fn set_frontend_attributes(&mut self, frontend_attributes: FrontendAttributes) {
    //if !self.has_rare() && frontend_attributes.map().is_empty() { return; }
    self.mutable_rare().frontend_attributes = frontend_attributes;
  }

  pub fn add_frontend_attributes(&mut self, _frontend_attributes: FrontendAttributes) {
    //if !frontend_attributes.map().is_empty() {
      //let map =
        //self.mutable_rare().frontend_attributes.mutable_map();
      //for (k, v) in frontend_attributes.map().iter() {
        //map.insert(k.clone(), v.clone());
      //}
    //}
  }

  pub fn frontend_attributes(&self) -> &FrontendAttributes {
    &self.rare().frontend_attributes
  }

  pub fn mutable_frontend_attributes(&mut self) -> &mut FrontendAttributes {
    &mut self.mutable_rare().frontend_attributes
  }

  pub fn add_single_statistic(&mut self, statistic: Statisitic) {
    self.mutable_rare().statistics_vis.add_statistics(statistic);
  }

  pub fn set_stat_index_to_visualize(&mut self, index: i64) {
    self.mutable_rare().statistics_vis.set_stat_index_to_visualize(index);
  }

  // Whether this specific instruction has statistics.
  pub fn has_statistics(&self) -> bool {
    !self.statistics_vis().statiscics().is_empty()
  }

  // Whether any instruction within the same HLO mosule as this has statistics.
  pub fn module_has_statistics(&self) -> bool {
    self.statistics_vis().stat_index_to_viaualize() == -1
  }

  pub fn statistic_to_visualize(&self) -> &Statisitic {
    let index = self.statistics_vis().stat_index_to_viaualize();
    &self.statistics_vis().statiscics()[index as usize]
  }

  pub fn set_statistics_vis(&mut self, statistics_vis: StatisticsViz) {
    self.mutable_rare().statistics_vis = statistics_vis;
  }

  pub fn statistics_vis(&self) -> &StatisticsViz {
    &self.rare().statistics_vis
  }

  pub fn raw_backend_config_string() {}
  pub fn set_raw_backend_config_string() {}

  pub fn is_default_config(&self) -> bool {
    self.is_default_config
  }

  pub fn set_default_config(&mut self) {
    self.is_default_config = true;
  }

  pub fn backend_config_to_raw_string() {}

  pub fn precision_config(&self) -> &PrecisionConfig {
    unimplemented!()
  }

  pub fn mutable_precision_config() {}

  // Sets the debug metadata for this instruction, excluding cration_pass_id,
  // which should never be copied anywhere.
  pub fn set_metadata(&mut self, metadata: &OpMetadata) {
    let creation_pass_id = metadata.creation_pass_id();
    self.metadata = Some(metadata.clone());
    self.metadata.as_mut().unwrap().set_creation_pass_id(creation_pass_id);
  }

  pub fn set_size_of_generated_code_in_bytes(&mut self, code_size_in_bytes: i64) {
    assert!(self.metadata.is_some());
    self.metadata.as_mut().unwrap()
      .set_size_of_generated_code_in_bytes(code_size_in_bytes)
  }

  pub fn set_size_of_memory_working_set_in_bytes(&mut self, working_set_size_in_bytes: i64) {
    assert!(self.metadata.is_some());
    self.metadata.as_mut().unwrap()
      .set_size_of_memory_working_set_in_bytes(working_set_size_in_bytes);
  }

  pub fn set_creation_pass_id(&mut self, pass_id: i64) {
    assert!(self.metadata.is_some());
    self.metadata.as_mut().unwrap().set_creation_pass_id(pass_id);
  }

  pub fn set_metadata_op_name(&mut self, name: String) {
    assert!(self.metadata.is_some());
    self.metadata.as_mut().unwrap().set_op_name(name);
  }

  pub fn set_logical_creation_pass_id(&mut self, pass_id: i64) {
    assert!(self.metadata.is_some());
    self.metadata.as_mut().unwrap().set_logical_creation_pass_id(pass_id);
  }

  pub fn set_metadata_deduplicated_name(&mut self, deduplicated_name: String) {
    assert!(self.metadata.is_some());
    self.metadata.as_mut().unwrap().set_deduplicated_name(deduplicated_name);
  }

  pub fn set_metadata_preserve_layout(&mut self, preserve_layout: bool) {
    assert!(self.metadata.is_some());
    self.metadata.as_mut().unwrap().set_preserve_layout(preserve_layout);
  }

  pub fn metadata(&self) -> &OpMetadata {
    assert!(self.metadata.is_some());
    self.metadata.as_ref().unwrap()
  }

  // Set/get the computation containing this instruction.
  pub fn set_parent(&mut self, parent: HloComputation) {
    self.parent = Some(Box::new(parent));
  }

  pub fn parent(&self) -> &HloComputation {
    self.parent.as_ref().unwrap()
  }

  pub fn mutable_parent(&mut self) -> &mut HloComputation {
    self.parent.as_mut().unwrap()
  }

  // Returns the module for this instruction.
  pub fn get_module(&self) -> &Option<HloModule> {
    if self.parent.is_some() {
      return self.parent.as_ref().unwrap().parent();
    }
    &None
  }

  pub fn sort_instruction_user_and_control_lists() {}

  pub fn feature_index(&self) -> i64 {
    unimplemented!()
  }

  pub fn epsilon() {}

  pub fn fft_type(&self) -> FftType {
    unimplemented!()
  }

  pub fn fft_length(&self) -> &Vec<i64> {
    unimplemented!()
  }

  // Delegates to HloChannelInstruction::channel_id.
  pub fn channel_id(&self) -> Option<i64> {
    None
  }

  pub fn set_channel_id(&mut self, _channel_id: Option<i64>) {}

  pub fn dimensions(&self) -> &Vec<i64> {
    unimplemented!()
  }

  pub fn add_dimensions(&mut self, _dim: i64) {
    unimplemented!()
  }

  pub fn dimensions_number(&self, _index: i64) -> i64 {
    unimplemented!()
  }

  pub fn concatenate_dimension(&self) -> i64 {
    unimplemented!()
  }

  pub fn dimension(&self) -> i64 {
    unimplemented!()
  }

  pub fn inferred_dimension() {}
  pub fn is_rank_2_transpose() {}

  pub fn slice_starts(&self) -> &Vec<i64> {
    unimplemented!()
  }

  pub fn mutable_slice_starts() {}

  pub fn slice_strides(&self) -> &Vec<i64> {
    unimplemented!()
  }

  pub fn mutable_slice_strides() {}

  pub fn slice_limits(&self) -> &Vec<i64> {
    unimplemented!()
  }

  pub fn add_slice_dimensions(&mut self, _slice_dim: SliceDimensions) {
    unimplemented!()
  }

  pub fn literal<T>(&self) -> &Literal<T>
    where T: Clone + Default + PartialEq
  {
    unimplemented!()
  }

  pub fn mutable_literal<T>(&self) -> &mut Literal<T>
    where T: Clone + Default + PartialEq
  {
    unimplemented!()
  }

  pub fn set_literal<T>(&mut self, _literal: Literal<T>)
    where T: Clone + Default + PartialEq
  {
    unimplemented!()
  }

  pub fn is_constant(&self) -> bool { false }

  pub fn relayout_constant() {}
  pub fn append_instruction_into_called_computation() {}

  // ##### HloFusionInstruction : start #####
  pub fn add_fusion_operand() {}
  pub fn merge_fusion_instruction() {}
  pub fn merge_fusion_instruction_into_multi_output() {}

  pub fn fused_instructions_computation(&self) -> &HloComputation {
    unimplemented!()
  }

  pub fn fused_expression_root(&self) -> &HloInstruction {
    unimplemented!()
  }

  pub fn fused_instructions() {}
  pub fn fused_instruction_count() {}

  // Delegates to HloFusionInstruction::fused_parameters.
  pub fn fused_parameter(&self, _parameter_number: i64) -> &HloInstruction {
    unimplemented!()
  }

  pub fn fused_parameters() {}

  // Delegates to HloFusionInstruction::is_multi_output_fusion
  pub fn is_multi_output_fusion(&self) -> bool {
    unimplemented!()
  }

  // Delegates to HloFusionInstruction::fusion_kind
  pub fn fusion_kind(&self) -> FusionKind {
    FusionKind::Custom
  }

  pub fn set_fusion_kind() {}

  // If multiple operands are the same instruction, keeps only one of them.
  pub fn deduplicate_fusion_operands(&self) -> Result<(), String> {
    unimplemented!()
  }

  // ##### HloFusionInstruction : end #####

  pub fn random_distribution() {}

  pub fn set_distribution(&mut self, _distribution: RandomDistribution) {
    unimplemented!()
  }

  pub fn parameter_number(&self) -> i64 {
    unimplemented!()
  }

  pub fn set_parameter_number(&mut self, _parameter_number: i64) {
    unimplemented!()
  }

  pub fn set_parameter_replicated_at_leaf_buffers() {}
  pub fn parameter_replicated_at_leaf_byffers() {}

  pub fn mutable_parameter_replication(&mut self) -> &mut ParameterReplication {
    unimplemented!()
  }

  pub fn tuple_index(&self) -> i64 {
    unimplemented!()
  }

  pub fn set_tuple_index() {}
  pub fn exponent_bits() {}

  pub fn set_exponent_bits(&mut self, _exponent_bits: i64) {
    unimplemented!()    
  }

  pub fn infeed_config() {}
  pub fn set_infeed_config() {}
  pub fn outfeed_config() {}
  pub fn set_outfeed_cofig() {}
  pub fn outfeed_shape() {}
  pub fn mutable_outfeed_shape() {}
  pub fn replica_groups() {}
  pub fn source_target_pairs() {}

  pub fn convolution_dimension_numberes(&self) -> &ConvolutionDimensionNumbers {
    unimplemented!()
  }

  pub fn set_convolution_dimension_numberes() {}

  pub fn feature_group_count(&self) -> i64 {
    unimplemented!()
  }

  pub fn set_feature_group_count() {}

  pub fn batch_group_count(&self) -> i64 {
    unimplemented!()
  }

  pub fn set_batch_group_count() {}
  pub fn select() {}
  pub fn scatter() {}
  pub fn set_select() {}
  pub fn set_scatter() {}

  // Delegates to HloCustomCallInstruction::custom_call_target.
  pub fn custom_call_target(&self)-> String { "".to_string() }

  pub fn set_custom_call_target() {}

  pub fn padding_config(&self) -> &PaddingConfig {
    unimplemented!()
  }

  pub fn mutable_padding_config() {}

  pub fn set_padding_config(&mut self, _padding_config: PaddingConfig) {
    unimplemented!()
  }

  pub fn padding_type() {}
  pub fn slice_sizes() {}

  pub fn dynamic_slice_sizes(&self) -> &Vec<i64> {
    unimplemented!()
  }

  pub fn add_dynamic_slice_sizes(&mut self, _size: i64) {
    unimplemented!()
  }

  pub fn dynamic_slice_sizes_list() {}

  pub fn index_shapes(&self) -> &Vec<Shape> {
    unimplemented!()
  }

  pub fn gather_dimension_numbers(&self) -> &GatherDimensionNumbers {
    unimplemented!()
  }

  pub fn set_gather_dimension_numbers(
    &mut self, _dimension_numbers: GatherDimensionNumbers)
  {
    unimplemented!()    
  }

  pub fn gather_slice_sizes(&self) -> &Vec<i64> {
    unimplemented!()
  }

  pub fn add_gather_slice_sizes(&mut self, _size: i64) {
    unimplemented!()
  }

  pub fn scatter_dimension(&self) -> i64 {
    unimplemented!()
  }

  pub fn dot_dimension_numbers(&self) -> &DotDimensionNumbers {
    unimplemented!()
  }

  pub fn operand_side_metadata(&self) -> &DomainMetadata {
    unimplemented!()
  }

  pub fn user_side_metadata(&self) -> &DomainMetadata {
    unimplemented!()
  }

  pub fn is_asynchronous(&self) -> bool {
    unimplemented!()
  }

  pub fn async_chain_start() {}
  pub fn async_chain_done() {}
  pub fn async_wrapped_computation() {}

  pub fn async_wrapped_instruction(&self) -> &HloInstruction {
    unimplemented!()
  }

  pub fn async_wrapped_opcode(&self) -> HloOpcode {
    unimplemented!()
  }

  pub fn async_execution_thread() {}
  pub fn set_async_execution_thread() {}
  pub fn set_called_computations_execution_thread() {}
  pub fn cross_program_prefetch_index() {}
  pub fn comparison_direction() {}

  pub fn set_comparison_direction(&mut self, _direction: ComparisonDirection) {
    unimplemented!()
  }

  pub fn set_comparison_type(&mut self, _t: ComparisonType) {
    unimplemented!()
  }

  pub fn comparison_order() {}

  pub fn triangular_solve_options(&self) -> &TriangularSolveOptions {
    unimplemented!()
  }

  pub fn cholsky_options() {}
  pub fn output_operand_aliasing() {}
  pub fn append_operand() {}

  // HloReducePrecisionInstruction
  pub fn operand_bits(&self) -> i64 {
    unimplemented!()
  }

  pub fn mantissa_bits(&self) -> i64 {
    unimplemented!()
  }

  pub fn set_mantissa_bits(&mut self, _mantissa_bits: i64) {
    unimplemented!()
  }
  // HloReducePrecisionInstruction

  // HloCollectiveInstruction
  pub fn is_collective_instruction(&self) -> bool {
    self.collective_instruction.is_some()
  }

  pub fn constrain_layout(&self) -> bool {
    self.collective_instruction.as_ref().unwrap().constrain_layout()
  }

  pub fn iota_dimension(&self) -> i64 {
    unimplemented!()
  }

  pub fn sparsity(&self) -> &Vec<SparsityDescriptor> {
    unimplemented!()
  }

  pub fn window(&self) -> &Window {
    unimplemented!()
  }

  pub fn unique_indices(&self) -> bool {
    unimplemented!()
  }

  pub fn scatter_indices(&self) -> &HloInstruction {
    unimplemented!()
  }

  pub fn scatter_dimension_numbers(&self) -> &ScatterDimensionNummbers {
    unimplemented!()
  }

  pub fn scatter_updates(&self) -> &Vec<HloInstruction> {
    unimplemented!()
  }

  pub fn set_indices_are_sorted(&mut self, _indices_are_sorted: bool) {
    unimplemented!()
  }

  fn print_extra_attributes_impl() {}
  // HloCollectiveInstruction

  // HloTopKInstruction
  pub fn k(&self) -> i64{
    unimplemented!()
  }
  // HloTopKInstruction

  fn is_elementwise_impl(&self, _operand_idx: Option<i64>) -> bool {
    false
  }

  fn print_operand_with_canonical_name_map() {}
  fn identical_slow_path() {}
  fn create_nary() {}

  // Adds a user for this instruction.
  fn add_user(&mut self, user: HloInstruction) {
    self.users.add_user(user);
  }

  // Removes a user for this instruction.
  fn remove_user(&mut self, user: HloInstruction) {
    self.users.remove_user(user);
  }

  fn get_backend_config_internal() {}
  fn mark_as_dead() {}

  fn is_marked_as_dead(&self) -> bool {
    false
  }

  fn has_rare(&self) -> bool {
    self.rare.is_some()
  }

  fn rare(&self) -> &Rare {
    assert!(self.has_rare());
    self.rare.as_ref().unwrap()
  }

  fn mutable_rare(&mut self) -> &mut Rare {
    assert!(self.has_rare());
    self.rare.as_mut().unwrap()
  }
}

pub fn string_to_fusion_kind(name: &String) -> Result<FusionKind, String> {
  if name == "Loop" {
    Ok(FusionKind::Loop)
  } else if name == "Input" {
    Ok(FusionKind::Input)
  } else if name == "Output" {
    Ok(FusionKind::Output)
  } else if name == "Custom" {
    Ok(FusionKind::Custom)
  } else {
    Err("Unknown fusion kind".to_string())
  }
}

pub fn string_to_random_distribution(_name: &String) -> Result<RandomDistribution, String> {
  unimplemented!()
}

pub fn string_to_random_algorithm(_name: &String) -> Result<RandomAlgorithm, String> {
  unimplemented!()
}

pub fn string_to_precision(_name: &String) -> Result<Precision, String> {
  unimplemented!()
}

pub fn string_to_algorithm(_name: &String) -> Result<Algorithm, String> {
  unimplemented!()
}