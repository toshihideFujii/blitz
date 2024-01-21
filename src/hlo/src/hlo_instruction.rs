#![allow(dead_code)]

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

pub enum FusionKind {
  Loop,
  Input,
  Output,
  Custom,
}

pub const MAIN_EXECUTION_THREAD: &'static str = "main";

pub struct HloInstruction {

}

impl HloInstruction {
  pub fn new_from_proto() {}
  pub fn new_parameter() {}
  pub fn new_constant() {}
  pub fn new_iota() {}
  pub fn new_top_k() {}
  pub fn new_get_tuple_element() {}
  pub fn new_rng() {}
  pub fn new_rng_bit_generator() {}
  pub fn new_rng_get_and_update_state() {}
  pub fn new_unary() {}
  pub fn new_binary() {}
  pub fn new_ternary() {}
  pub fn new_variadic() {}
  pub fn new_map() {}
  pub fn new_convolve() {}
  pub fn new_fft() {}
  pub fn new_async_start() {}
  pub fn new_async_update() {}
  pub fn new_async_done() {}
  pub fn new_copy_start() {}
  pub fn new_compare() {} 
  pub fn new_triangular_solve() {}
  pub fn new_cholesky() {}
  pub fn new_dot() {}
  pub fn new_reduce_precision() {}
  pub fn new_all_gather() {}
  pub fn new_all_gather_start() {}
  pub fn new_all_reduce() {}
  pub fn new_reduce_scatter() {}
  pub fn new_all_reduce_start() {}
  pub fn new_all_to_all() {}
  pub fn new_collective_permute() {}
  pub fn new_collective_permute_start() {}
  pub fn new_replica_id() {}
  pub fn new_partition_id() {}
  pub fn new_convert() {}
  pub fn new_bitcast() {}
  pub fn new_bitcast_convert() {}
  pub fn new_stochastic_convert() {}
  pub fn new_infeed() {}
  pub fn new_outfeed() {}
  pub fn new_send() {}
  pub fn new_send_done() {}
  pub fn new_recv() {}
  pub fn new_recv_done() {}
  pub fn new_slice() {}
  pub fn new_dynamic_slice() {}
  pub fn new_dynamic_update_slice() {}
  pub fn new_concatenate() {}
  pub fn new_reduce() {}
  pub fn new_reduce_window() {}
  pub fn new_batch_norm_training() {}
  pub fn new_batch_norm_inference() {}
  pub fn new_batch_norm_grad() {}
  pub fn new_select_and_scatter() {}
  pub fn new_broadcast() {}
  pub fn new_broadcast_sequence() {}
  pub fn new_pad() {}
  pub fn new_reshape() {}
  pub fn new_dynamic_reshape() {}
  pub fn new_transpose() {}
  pub fn new_sort() {}
  pub fn new_while() {}
  pub fn new_conditional() {}
  pub fn new_gather() {}
  pub fn new_scatter() {}
  pub fn new_domain() {}
  pub fn new_fusion() {}
  pub fn new_call() {}
  pub fn new_custom_call() {}
  pub fn new_tuple() {}
  pub fn new_reverse() {}
  pub fn new_after_all() {}
  pub fn new_token() {}
  pub fn new_get_dimension_size() {}
  pub fn new_set_dimension_size() {}
  pub fn new_add_dependency() {}

  pub fn is_root() {}
  pub fn is_dead() {}
  pub fn has_side_effect_no_recurse() {}
  pub fn has_side_effect() {}
  pub fn shape() {}
  pub fn operand() {}
  pub fn operand_count() {}
  pub fn operands() {}
  pub fn unique_operands() {}
  pub fn operand_index() {}
  pub fn user_count() {}
  pub fn users() {}
  pub fn user_id() {}
  pub fn is_user_of() {}

  pub fn add_control_dependency_to() {}
  pub fn remove_control_dependency_to() {}
  pub fn drop_all_control_deps() {}
  pub fn safely_drop_all_control_dependencies() {}
  pub fn has_control_dependencies() {}
  pub fn copy_all_control_deps_from() {}
  pub fn control_predecessors() {}
  pub fn control_successors() {}
}