#![allow(dead_code)]

use std::{any::Any, collections::{BTreeMap, HashMap, HashSet}};
use common::{
  array::Array, blitz_data::{
    Algorithm, ConvolutionDimensionNumbers, CustomCallApiVersion, CustomCallSchedule, DimLevelType, FftType, FileLocation, FrontendAttributes, OpMetadata, OpShardingType, PaddingConfig, PaddingType, ParameterReplication, Precision, PrimitiveType, RandomAlgorithm, RandomDistribution, ReplicaGroup, ResultAccuracy, ResultAccuracyMode, ResultAccuracyTolerance, StackFrame, StackFrameIndex, Statisitic, StatisticsViz, Window
  }, comparison_util::{
    ComparisonDirection, ComparisonType, string_to_comparison_direction,
    string_to_comparison_type
  }, layout::{
    Layout, SplitConfig, Tile}, layout_util::LayoutUtil, literal::Literal, literal_util::LiteralUtil, primitive_util::{
    is_complex_type, is_floating_point_type, is_integral_type,
    primitive_type_name, primitive_type_switch
  }, shape::Shape, shape_layout::ShapeLayout, shape_util::ShapeUtil
};
use hlo::{
  collective_ops_utils::{CollectiveOpGroupMode, string_to_collective_op_group_mode}, computation_layout::ComputationLayout, hlo_computation::{HloComputation, HloComputationBuilder}, hlo_domain_metadata::DomainMetadata, hlo_input_output_alias_config::{Alias, AliasKind}, hlo_instruction::{self, FusionKind, HloInstruction, string_to_custom_call_api_version, string_to_custom_call_schedule, string_to_result_accuracy}, hlo_module::{HloModule, OriginalValueRecoveryTable}, hlo_module_config::HloModuleConfig, hlo_opcode::{HloOpcode, hlo_opcode_string, string_to_hlo_opcode}, hlo_original_value::OriginalValue, hlo_schdule::HloSchedule, hlo_sharding::HloSharding, name_uniquer::NameUniquer, tile_assignment::TileAssignment
};
use num::complex::Complex64;
use crate::{
  hlo_lexer::{DIM_LABELS_DXD_PAD_DECIMAL_MASK, HloLexer, TokKind,
  tok_kind_to_string}, shape_inference::ShapeInference
};

pub struct HloParserOptions {
  fill_missing_layouts: bool,
  fill_shortform_constants_with_random_values: bool,
  keep_module_auto_layouts: bool,
  max_recursion_depth: usize,
}

impl HloParserOptions {
  pub fn default() -> Self {
    HloParserOptions {
      fill_missing_layouts: false,
      fill_shortform_constants_with_random_values: false,
      keep_module_auto_layouts: false,
      max_recursion_depth: 8192,
    }
  }

  // When a shape layout is not set (e.g. in the entry computation layout or
  // instruction layout), set the layout to be the default (e.g. {3,2,1,0}).
  pub fn set_fill_missing_layouts(&mut self, value: bool) {
    self.fill_missing_layouts = value;
  }

  pub fn fill_missing_layouts(&self) -> bool {
    self.fill_missing_layouts
  }

  // Fill short form constants (dots) with deterministic random values.
  pub fn set_fill_shortform_constant_with_random_values(&mut self, value: bool) {
    self.fill_shortform_constants_with_random_values = value;
  }

  pub fn fill_shortform_constants_with_random_values(&self) -> bool {
    self.fill_shortform_constants_with_random_values
  }

  // Keep module auto layouts, i.e. do not reset unset entry computation layouts
  // to the default layout.  This is a subset of what fill_missing_layouts=false
  // does.
  pub fn set_keep_module_auto_layouts(&mut self, value: bool) {
    self.keep_module_auto_layouts = value;
  }

  pub fn keep_module_auto_layouts(&self) -> bool {
    self.keep_module_auto_layouts
  }

  pub fn set_max_recursion_depth(&mut self, value: usize) {
    self.max_recursion_depth = value;
  }

  pub fn max_recursion_depth(&self) -> usize {
    self.max_recursion_depth
  }
}

// Given a string in the HloModule::ToString() format, parses the string and
// creates a HloModule with the given config.
// Note: Tests derived from HloHardwareIndependentTestBase should use
// ParseAndReturnVerifiedModule() instead!
pub fn parse_and_return_unverified_module(
  str: String,
  config: HloModuleConfig,
  options: HloParserOptions) -> Result<HloModule, String>
{
  let mut module = HloModule::new("_".to_string(), config);
  let mut parser = HloParser::new(str, options);
  let result = parser.run(&mut module);
  if result.is_err() {
    return Err(result.err().unwrap());
  }
  Ok(module)
}

// Parses sharding from str. str is supposed to contain the body of the
// sharding, i.e. just the rhs of the "sharding={...}" attribute string, e.g.,
// "{replicated}".
pub fn parse_sharding(str: String) -> Result<HloSharding, String> {
  let mut parser =
    HloParser::new(str, HloParserOptions::default());
  parser.parse_sharding_only()
}

// Parses original value from str.
pub fn parse_original_value(_str: String) {
  unimplemented!()
}

// Parses frontend attributes from str. str is supposed to contain the body of
// the frontend attributes , i.e. just the rhs of the
// "frontend_attributes={...}" attribute string, e.g.,
// "{attr_a=a,attr_b=b}".
pub fn parse_frontend_attributes(str: String) -> Result<FrontendAttributes, String> {
  let mut parser = HloParser::new(str, HloParserOptions::default());
  parser.parse_frontend_attributes_only()
}

// Parses statistics viz from str. str is supposed to contain the body of the
// statistics visualization, i.e. just the rhs of the "statistics={...}"
// attribute string, e.g., "{visualizing_index=1,nan_percent=50}".
pub fn parse_statistics_viz(str: String) -> Result<StatisticsViz, String> {
  let mut parser = HloParser::new(str, HloParserOptions::default());
  parser.parse_statistics_viz_only()
}

// Parses parameter replication from str. str is supposed to contain the body of
// the parameter replication, i.e. just the rhs of the
// "parameter_replication={...}" attribute string, e.g., "{true, false}".
pub fn parse_parameter_replication(str: String) -> Result<Vec<bool>, String> {
  let mut parser = HloParser::new(str, HloParserOptions::default());
  parser.parse_parameter_replication_only()
}

// Parses the result of window_util::ToString(const Window&).
pub fn parse_window(str: String) -> Result<Window, String> {
  let mut parsesr = HloParser::new(str, HloParserOptions::default());
  parsesr.parse_window_only()
}

// Parses the result of ConvolutionDimensionNumbersToString(), e.g.
// "b0f_0io->b0f".
pub fn parse_convolution_demension_numbers(
  str: String) -> Result<ConvolutionDimensionNumbers, String>
{
  let mut parser = HloParser::new(str, HloParserOptions::default());
  parser.parse_convolution_demension_numbers_only()
}

// Parses the result of PaddingConfigToString(), e.g. "0_0x1_1".
pub fn parse_padding_config(str: String) -> Result<PaddingConfig, String> {
  let mut parser = HloParser::new(str, HloParserOptions::default());
  parser.parse_padding_config_only()
}

// Parses and returns a Shape::to_string-format string.
pub fn parse_shape(_str: String) -> Result<Shape, String> {
  unimplemented!()
}

// Parses and returns a Layout::to_string-format string.
pub fn parse_layout_by_string(_str: String) -> Result<Layout, String> {
  unimplemented!()
}

// Parses and returns a std::vector<ReplicaGroup> from str. str is supposed to
// contain a list of the replica groups, i.e. just the rhs of the
// "replica_groups={...}" attribute string, e.g., "{{0,1}, {2,3}}".
pub fn parse_replica_groups_only(
  str: String) -> Result<Vec<ReplicaGroup>, String>
{
  let parser = HloParser::new(str, HloParserOptions::default());
  parser.parse_replica_groups_only()
}

fn can_infer_shape(code: &HloOpcode) -> bool {
  match code {
    HloOpcode::Abs => return true,
    HloOpcode::Add => return true,
    HloOpcode::AddDependency => return true,
    HloOpcode::AfterAll => return true,
    HloOpcode::Atan2 => return true,
    HloOpcode::BatchNormGrad => return true,
    HloOpcode::BatchNormInference => return true,
    HloOpcode::BatchNormTraining => return true,
    HloOpcode::Broadcast => return true,
    HloOpcode::Call => return true,
    HloOpcode::Ceil => return true,
    HloOpcode::Cholsky => return true,
    HloOpcode::Clamp => return true,
    HloOpcode::Clz => return true,
    HloOpcode::Compare => return true,
    HloOpcode::Complex => return true,
    HloOpcode::Concatenate => return true,
    HloOpcode::Conditional => return true,
    HloOpcode::Convolution => return true,
    HloOpcode::Copy => return true,
    HloOpcode::Cos => return true,
    HloOpcode::OptimizationBarrier => return true,
    HloOpcode::Divide => return true,
    HloOpcode::Domain => return true,
    HloOpcode::Dot => return true,
    HloOpcode::Erf => return true,
    HloOpcode::Exp => return true,
    HloOpcode::Expm1 => return true,
    HloOpcode::Fft => return true,
    HloOpcode::Floor => return true,
    HloOpcode::Gather => return true,
    HloOpcode::GetDimensionSize => return true,
    HloOpcode::SetDimensionSize => return true,
    HloOpcode::GetTupleElement => return true,
    HloOpcode::Imag => return true,
    HloOpcode::IsFinite => return true,
    HloOpcode::Log => return true,
    HloOpcode::Log1p => return true,
    HloOpcode::Logistic => return true,
    HloOpcode::And => return true,
    HloOpcode::Not => return true,
    HloOpcode::Or => return true,
    HloOpcode::Xor => return true,
    HloOpcode::Map => return true,
    HloOpcode::Maximum => return true,
    HloOpcode::Minimum => return true,
    HloOpcode::Multiply => return true,
    HloOpcode::Negate => return true,
    HloOpcode::Pad => return true,
    HloOpcode::PartitionId => return true,
    HloOpcode::PopulationCount => return true,
    HloOpcode::Power => return true,
    HloOpcode::Real => return true,
    HloOpcode::Reduce => return true,
    HloOpcode::Remainder => return true,
    HloOpcode::ReplicaId => return true,
    HloOpcode::Reverse => return true,
    HloOpcode::RoundNearestAfz => return true,
    HloOpcode::RoundNearestEven => return true,
    HloOpcode::Rsqrt => return true,
    HloOpcode::Scatter => return true,
    HloOpcode::Select => return true,
    HloOpcode::ShiftLeft => return true,
    HloOpcode::ShiftRightArithmetic => return true,
    HloOpcode::ShiftRightLogical => return true,
    HloOpcode::Sign => return true,
    HloOpcode::Sin => return true,
    HloOpcode::Sqrt => return true,
    HloOpcode::Cbrt => return true,
    HloOpcode::ReduceWindow => return true,
    HloOpcode::SelectAndScatter => return true,
    HloOpcode::Sort => return true,
    HloOpcode::Subtract => return true,
    HloOpcode::Tan => return true,
    HloOpcode::Tanh => return true,
    HloOpcode::Transpose => return true,
    HloOpcode::TriangularSolve => return true,
    HloOpcode::Tuple => return true,
    HloOpcode::While => return true,
    HloOpcode::TopK => return true,
    _ => return false
  }
}

enum AttrType {
  Bool,
  Int64,
  Int32,
  Float,
  String,
  Literal,
  BracedInt64List,
  BracedInt64ListList,
  HloComputation,
  BracedHloComputationList,
  FftType,
  PaddingType,
  ComparisonDirection,
  ComparisonType,
  Window,
  ConvolutionDimensionNumbers,
  Sharding,
  FrontendAttributes,
  StatisticsViz,
  BracedBoolListOrBool,
  ParameterReplication,
  InstructionList,
  SliceRanges,
  PaddingConfig,
  Metadata,
  FusionKind,
  Distribution,
  Domain,
  PrecisionList,
  Shape,
  ShapeList,
  Enum,
  RandomAlgorithm,
  PrecisionAlgorithm,
  Aliasing,
  BufferDonor,
  ComputationLayout,
  InstructionAliasing,
  CustomCallSchedule,
  CustomCallApiVersion,
  SparsityDescriptor,
  // A double-quoted string, or a string that looks like a JSON dictionary
  // enclosed in matching curly braces (returned value includes the curlies).
  StringOrJsonDict,
  CollectiveDeviceList,
  ResultAccuracy,
  ResultAccuracyType,
  OriginalValue,
  OriginalValueRecoveryTable,
  Mode
}

struct AttrConfig {
  required: bool, // whether it's required or optional
  attr_type: AttrType, // what type it is
  result: Box<dyn Any> // where to store the parsed result.
}

impl AttrConfig {
  pub fn new(required: bool, attr_type: AttrType, result: Box<dyn Any>) -> Self {
    AttrConfig {
      required: required,
      attr_type: attr_type,
      result: result
    }
  }

  pub fn result(&self) -> &Box<dyn Any> {
    &self.result
  }
}

struct SliceRange {
  starts: Vec<i64>,
  limits: Vec<i64>,
  strides: Vec<i64>,
}

struct DomainData {
  entry_metadata: DomainMetadata,
  exit_metadata: DomainMetadata,
}

impl DomainData {
  pub fn default() -> Self {
    DomainData {
      entry_metadata: DomainMetadata::default(),
      exit_metadata: DomainMetadata::default()
    }
  }
}

struct Scope {}

pub struct HloParser {
  lexer: HloLexer,
  options: HloParserOptions,
  // Used to generate names for anonymous instructions.
  name_uniquer: NameUniquer,

  // A stack for the instruction names. The top of the stack stores the
  // instruction name table for the current scope.
  //
  // A instruction's name is unique among its scope (i.e. its parent
  // computation), but it's not necessarily unique among all computations in the
  // module. When there are multiple levels of nested computations, the same
  // name could appear in both an outer computation and an inner computation. So
  // we need a stack to make sure a name is only visible within its scope,
  scoped_name_tables: Vec<HashMap<String, (HloInstruction, usize)>>,
  computation_pool: HashMap<String, (HloComputation, usize)>,
  computations: Vec<HloComputation>,
  error: Vec<String>,
  // Tracks recursion depth to prevent stack overflow from deeply nested input.
  current_recursion_depth: usize,
}

impl HloParser {
  pub fn new(str: String, options: HloParserOptions) -> Self {
    HloParser {
      lexer: HloLexer::new(str),
      options: options,
      name_uniquer: NameUniquer::new(".".to_string()),
      scoped_name_tables: Vec::new(),
      computation_pool: HashMap::new(),
      computations: Vec::new(),
      error: Vec::new(),
      current_recursion_depth: 0
    }
  }

  pub fn new_for_tests(str: String, options: HloParserOptions) -> Self {
    HloParser {
      lexer: HloLexer::new(str),
      options: options,
      name_uniquer: NameUniquer::new(".".to_string()),
      scoped_name_tables: Vec::new(),
      computation_pool: HashMap::new(),
      computations: Vec::new(),
      error: Vec::new(),
      current_recursion_depth: 0
    }
  }

  // Runs the parser and constructs the resulting HLO in the given (empty)
  // HloModule. Returns the error status in case an error occurred.
  pub fn run(&mut self, module: &mut HloModule) -> Result<(), String> {
    self.lexer.lex(0);
    if self.lexer.get_kind() == TokKind::HloModule ||
       self.lexer.get_kind() == TokKind::Entry ||
       self.lexer.look_ahead() == TokKind::Lbrace
    {
      // This means that the text contains a full HLLO module.
      let mut parse_module_without_header = true;
      if self.lexer.get_kind() == TokKind::HloModule {
        parse_module_without_header = false;
      }
      if !self.parse_hlo_module(module, parse_module_without_header) {
        let mut err_msg =
          "Syntax error when trying to parse the text as a HloModule.".to_string();
        err_msg.push_str(&self.get_error());
        return Err(err_msg);
      }
      return Ok(());
    }
    if !self.parse_single_instruction(module) {
      let mut err_msg = "Syntax error when trying to parse the text as a 
        single HloInstruction:\n".to_string();
      err_msg.push_str(&self.get_error());
      return Err(err_msg);
    }
    Ok(())
  }

  // Returns the error information.
  pub fn get_error(&self) -> String {
    let mut result = "".to_string();
    for err_msg in &self.error {
      result.push_str(&err_msg);
    }
    result
  }

  pub fn parse_shape_only(&mut self) -> Result<Shape, String> {
    self.lexer.lex(0);
    let mut shape = Shape::default();
    if !self.parse_shape(&mut shape, true) {
      let mut error_msg = "Syntax error:\n".to_string();
      error_msg.push_str(&self.get_error());
      return Err(error_msg);
    }
    if self.lexer.get_kind() != TokKind::Eof {
      let error_msg = "Syntax error:\nExtra content after shape".to_string();
      return Err(error_msg);
    }
    Ok(shape)
  }

  pub fn parse_layout_only(&mut self) -> Result<Layout, String> {
    self.lexer.lex(0);
    let mut layout = Layout::default();
    if !self.parse_layout(&mut layout) {
      let mut error_msg = "Syntax error:\n".to_string();
      error_msg.push_str(&self.get_error());
      return Err(error_msg);
    }
    if self.lexer.get_kind() != TokKind::Eof {
      let error_msg = "Syntax error:\nExtra content after layout".to_string();
      return Err(error_msg);
    }
    Ok(layout)
  }

  pub fn parse_sharding_only(&mut self) -> Result<HloSharding, String> {
    self.lexer.lex(0);
    let mut sharding = HloSharding::default();
    if !self.parse_sharding(&mut sharding) {
      let mut err_msg = "Syntax error:\n".to_string();
      err_msg.push_str(&self.get_error());
      return Err(err_msg);
    }
    if self.lexer.get_kind() != TokKind::Eof {
      let err_msg = "Syntax error:\nExtra content after sharding".to_string();
      return Err(err_msg);
    }
    Ok(sharding)
  }

  pub fn parse_frontend_attributes_only(&mut self) -> Result<FrontendAttributes, String> {
    self.lexer.lex(0);
    let mut attributes = FrontendAttributes::default();
    if !self.parse_frontend_attributes(&mut attributes) {
      let mut error_msg = "Syntax error:\n".to_string();
      error_msg.push_str(&self.get_error());
      return Err(error_msg);
    }
    if self.lexer.get_kind() != TokKind::Eof {
      let error_msg =
        "Syntax error:\nExtra content after frontend attributes".to_string();
      return Err(error_msg);
    }
    Ok(attributes)
  }

  pub fn parse_statistics_viz_only(&mut self) -> Result<StatisticsViz, String> {
    self.lexer.lex(0);
    let mut statistics_viz = StatisticsViz::default();
    if !self.parse_statistics_viz(&mut statistics_viz) {
      let mut error_msg = "Syntax error:\n".to_string();
      error_msg.push_str(&self.get_error());
      return Err(error_msg);
    }
    if self.lexer.get_kind() != TokKind::Eof {
      let error_msg =
        "Syntax error:\nExtra content after statistics".to_string();
      return Err(error_msg);
    }
    Ok(statistics_viz)
  }

  pub fn parse_parameter_replication_only(&mut self) -> Result<Vec<bool>, String> {
    self.lexer.lex(0);
    let mut parameter_replication = ParameterReplication::default();
    if !self.parse_parameter_replication(&mut parameter_replication) {
      let mut error_msg = "Syntax error:\n".to_string();
      error_msg.push_str(&self.get_error());
      return Err(error_msg);
    }
    if self.lexer.get_kind() != TokKind::Eof {
      let error_msg =
        "Syntax error:\nExtra content after parameter replication".to_string();
      return Err(error_msg);
    }
    let mut result = vec![];
    for param_replicated in parameter_replication.replicated_at_leaf_buffers() {
      result.push(*param_replicated)
    }
    Ok(result)
  }

  pub fn parse_boolean_list_or_single_boolean_only(&mut self) -> Result<Vec<bool>, String> {
    self.lexer.lex(0);
    let mut booleans = vec![];
    if !self.parse_boolean_list_or_single_boolean(&mut booleans) {
      let mut error_msg = "Syntax error:\n".to_string();
      error_msg.push_str(&self.get_error());
      return Err(error_msg);
    }
    if self.lexer.get_kind() != TokKind::Eof {
      let error_msg =
        "Syntax error:\nExtra content after boolean list".to_string();
      return Err(error_msg);
    }
    Ok(booleans)
  }

  pub fn parse_window_only(&mut self) -> Result<Window, String> {
    self.lexer.lex(0);
    let mut window = Window::default();
    if !self.parse_window(&mut window, false) {
      let mut error_msg = "Syntax error:\n".to_string();
      error_msg.push_str(&self.get_error());
      return Err(error_msg);
    }
    if self.lexer.get_kind() != TokKind::Eof {
      let error_msg =
        "Syntax error:\nExtra content after window".to_string();
      return Err(error_msg);
    }
    Ok(window)
  }

  pub fn parse_convolution_demension_numbers_only(
    &mut self) -> Result<ConvolutionDimensionNumbers, String>
  {
    self.lexer.lex(0);
    let mut dnums = ConvolutionDimensionNumbers::default();
    if !self.parse_convolution_demension_numbers(&mut dnums) {
      let mut error_msg = "Syntax error:\n".to_string();
      error_msg.push_str(&self.get_error());
      return Err(error_msg);
    }
    if self.lexer.get_kind() != TokKind::Eof {
      let error_msg =
        "Syntax error:\nExtra content after convolution dnums".to_string();
      return Err(error_msg);
    }
    Ok(dnums)
  }

  pub fn parse_padding_config_only(&mut self) -> Result<PaddingConfig, String> {
    self.lexer.lex(0);
    let mut padding_config = PaddingConfig::default();
    if !self.parse_padding_config(&mut padding_config) {
      let mut error_msg = "Syntax error:\n".to_string();
      error_msg.push_str(&self.get_error());
      return Err(error_msg);
    }
    if self.lexer.get_kind() != TokKind::Eof {
      let error_msg =
        "Syntax error:\nExtra content after padding_config".to_string();
      return Err(error_msg);
    }
    Ok(padding_config)
  }

  pub fn parse_replica_groups_only(&self) -> Result<Vec<ReplicaGroup>, String> {
    unimplemented!()
  }

  // Returns the map from the instruction name to the instruction itself and its
  // location in the current scope.
  fn current_name_table(&self)
    -> Option<&HashMap<String, (HloInstruction, usize)>> {
    self.scoped_name_tables.last()
  }

  fn mutable_current_name_table(&mut self)
    -> Option<&mut HashMap<String, (HloInstruction, usize)>> {
    self.scoped_name_tables.last_mut()
  }

  fn find_instruction(&self, _name: &String) -> Option<&(HloInstruction, usize)> {
    unimplemented!()
  }

  // Parse a single instruction worth of text.
  fn parse_single_instruction(&mut self, module: &mut HloModule) -> bool {
    if !self.scoped_name_tables.is_empty() { // TODO
      assert!(false, "Parser state is not clean. Please do not call any other
        methods before calling parse_single_instruction.");
      return false;
    }
    let mut builder =
      HloComputationBuilder::new(module.name());

    // The missing instruction hook we register creates the shaped instruction on
    // the fly as a parameter and returns it. 
    let _parameter_count = 0;
    let _create_missing_instruction = || {};

    // Parse the instruction with the registered hook.
    let _scope = HloParserScope::new(&self.scoped_name_tables);
    if self.can_be_shape() {
      // This means that the instruction's left-hand side is probably omitted,
      // e.g. f32[10] fusion(...), calls={...}
      if !self.parse_instruction_rhs(&mut builder, module.mutable_name(),
      self.lexer.get_loc(), true)
      {
        return false;
      }
    } else {
      // This means that the instruction's left-hand side might exist, e.g.
      //  foo = f32[10] fusion(...), calls={...}
      let mut root_name = String::new();
      if !self.parse_instruction(&mut builder, &mut root_name) {
        return false;
      }
    }

    if self.lexer.get_kind() != TokKind::Eof {
      assert!(false, "Syntax error:\nExpected eof after parsing single instruction.
        Did you mean to wtite an HLO module and forget the HloModule header?");
      return false;
    }

    module.add_entry_computation(builder.build(None));
    for comp in &self.computations {
      module.add_embedded_computation(comp.clone());
    }
    //let schedule = schedule_from_instruction_order(module);
    //module.set_schedule(schedule);

    true
  }

  // Parses a module, returning false if an error occurred.
  // if `parse_module_without_header` is true, the parsed text is sequence of
  // computations, and assume computation with `ENTRY` annotation or the last
  // computation as module's entry computation, also using the entry
  // computation's parameter and `ROOT` instruction's layout as module's layout.
  fn parse_hlo_module(
    &mut self, module: &mut HloModule, parse_module_without_header: bool) -> bool
  {
    let mut name = String::new();
    let mut attrs: HashMap<String, AttrConfig> = HashMap::new();

    let is_scheduled: Option<bool> = None;
    let is_scheduled_conf = AttrConfig::new(
      false, AttrType::Bool, Box::new(is_scheduled));
    attrs.insert("is_scheduled".to_string(), is_scheduled_conf);

    let alias_passthrough_params: Option<bool> = None;
    let alias_passthrough_params_conf = AttrConfig::new(
      false, AttrType::Bool, Box::new(alias_passthrough_params));
    attrs.insert("alias_passthrough_params".to_string(),
    alias_passthrough_params_conf);

    let num_partitions: Option<i64> = None;
    let num_partitions_conf = AttrConfig::new(
      false, AttrType::Int64, Box::new(num_partitions));
    attrs.insert("num_partitions".to_string(), num_partitions_conf);

    let replica_count: Option<i64> = None;
    let replica_count_conf = AttrConfig::new(
      false, AttrType::Int64, Box::new(replica_count));
    attrs.insert("replica_count".to_string(), replica_count_conf);

    let entry_computation_layout: Option<ComputationLayout> = None;
    let entry_comp_layout_conf = AttrConfig::new(
      false, AttrType::ComputationLayout,
      Box::new(entry_computation_layout));
    attrs.insert("entry_computation_layout".to_string(), entry_comp_layout_conf);

    let frontend_attrs: Option<FrontendAttributes> = None;
    let frontend_attrs_conf = AttrConfig::new(
      false, AttrType::FrontendAttributes, Box::new(frontend_attrs));
    attrs.insert("frontend_attributes".to_string(), frontend_attrs_conf);

    if !parse_module_without_header {
      if self.lexer.get_kind() != TokKind::HloModule {
        return self.token_error("expects HloModule".to_string());
      }
      // Eat 'HloModule'
      self.lexer.lex(0);
      if !self.parse_name(&mut name) {
        return false;
      }
      if !self.parse_attributes(&mut attrs, true, &None) {
        return false;
      }
      module.set_name(name.clone());
    }

    if !self.parse_stack_frame_index(module) || !self.parse_computations(module)
    {
      return false;
    }

    if parse_module_without_header {
      let mut new_name = "module_".to_string();
      new_name.push_str(&module.entry_computation().unwrap().name());
      name = new_name;
    }
    module.set_name(name);
    
    if is_scheduled.is_some() && is_scheduled.unwrap() == false {
      module.set_schedule(schedule_from_instruction_order(&module.clone()));
    }
    let config = module.mutable_config();
    if alias_passthrough_params.is_some() && alias_passthrough_params.unwrap() == false {
      config.set_alias_passthrough_params(true);
    }
    if num_partitions.is_some() && num_partitions.unwrap() != 1 {
      config.set_num_partitions(num_partitions.unwrap());
      config.set_use_spmd_partitioning(true);
    }
    if replica_count.is_some() && replica_count.unwrap() != 1 {
      config.set_replica_count(replica_count.unwrap());
    }
    let entry_comp_layout_conf: Option<&AttrConfig> =
      attrs.get(&"entry_computation_layout".to_string());
    if entry_comp_layout_conf.is_some() {
      let entry_comp_layout = entry_comp_layout_conf
        .unwrap().result().downcast_ref::<ComputationLayout>().unwrap().clone();
      *config.mutable_entry_computation_layout() =entry_comp_layout;
    } else {
      // If entry_computation_layout is not specified explicitly, we infer the
      // layout from parameter and root instructions.
    }
    let frontend_attrs_conf: Option<&AttrConfig> =
      attrs.get(&"frontend_attributes".to_string());
    if frontend_attrs_conf.is_some() {
      let frontend_attrs = frontend_attrs_conf
        .unwrap().result().downcast_ref::<FrontendAttributes>().unwrap().clone();
      module.set_frontend_attributes(frontend_attrs);
    }

    true
  }

  // Parses a list of `int {file_name_id=int function_name_id=int line=int
  // end_line=int column=int end_column=int}` into
  // StackFrameIndexProto::file_locations.
  pub fn parse_file_location_list(
    &mut self, stack_frame_index: &mut StackFrameIndex) -> bool
  {
    let file_name_id: Option<i64> = None;
    let file_name_conf = AttrConfig::new(
      true, AttrType::Int64, Box::new(file_name_id));

    let func_name_id: Option<i64> = None;
    let func_name_conf = AttrConfig::new(
      true, AttrType::Int64, Box::new(func_name_id));

    let line: Option<i64> = None;
    let line_conf = AttrConfig::new(
      true, AttrType::Int64, Box::new(line));

    let end_line: Option<i64> = None;
    let end_line_conf = AttrConfig::new(
      true, AttrType::Int64, Box::new(end_line));

    let column: Option<i64> = None;
    let column_conf = AttrConfig::new(
      true, AttrType::Int64, Box::new(column));

    let end_column: Option<i64> = None;
    let end_column_conf = AttrConfig::new(
      true, AttrType::Int64, Box::new(end_column));

    let mut attrs = HashMap::new();
    attrs.insert("file_name_id".to_string(), file_name_conf);
    attrs.insert("function_name_id".to_string(), func_name_conf);
    attrs.insert("line".to_string(), line_conf);
    attrs.insert("end_line".to_string(), end_line_conf);
    attrs.insert("column".to_string(), column_conf);
    attrs.insert("end_column".to_string(), end_column_conf);

    while self.eat_if_present(&TokKind::Int) {
      if !self.parse_sub_attributes(&attrs) {
        return false;
      }
      let mut file_location = FileLocation::default();
      
      let file_name_conf: &AttrConfig = attrs.get("file_name_id").unwrap();
      let file_name_id = file_name_conf.result.downcast_ref::<i64>().unwrap();
      file_location.set_file_name_id(*file_name_id);

      let function_name_conf: &AttrConfig = attrs.get("function_name_id").unwrap();
      let function_name_id = function_name_conf.result.downcast_ref::<i64>().unwrap();
      file_location.set_function_name_id(*function_name_id);

      let line_conf: &AttrConfig = attrs.get("line").unwrap();
      let line = line_conf.result.downcast_ref::<i64>().unwrap();
      file_location.set_line(*line);

      let end_line_conf: &AttrConfig = attrs.get("end_line").unwrap();
      let end_line = end_line_conf.result.downcast_ref::<i64>().unwrap();
      file_location.set_end_line(*end_line);

      let column_conf: &AttrConfig = attrs.get("column").unwrap();
      let column = column_conf.result.downcast_ref::<i64>().unwrap();
      file_location.set_column(*column);

      let end_column_conf: &AttrConfig = attrs.get("end_column").unwrap();
      let end_column = end_column_conf.result.downcast_ref::<i64>().unwrap();
      file_location.set_end_column(*end_column);

      stack_frame_index.add_file_location(file_location);
    }
    true  
  }

  // Parses a list of `int {function_location_id=int parent_frame_id=int}` into
  // StackFrameIndexProto::stack_frames.
  fn parse_stack_frames_list(
    &mut self, stack_frame_index: &mut StackFrameIndex) -> bool
  {
    let file_location_id: Option<i64> = None;
    let file_loc_conf = AttrConfig::new(
      true, AttrType::Int64, Box::new(file_location_id));
    
    let parent_frame_id: Option<i64> = None;
    let parent_frame_conf = AttrConfig::new(
      true, AttrType::Int64, Box::new(parent_frame_id));

    let mut attrs = HashMap::new();
    attrs.insert("file_location_id".to_string(), file_loc_conf);
    attrs.insert("parent_frame_id".to_string(), parent_frame_conf);

    while self.lexer.get_kind() == TokKind::Int {
      self.lexer.lex(0);
      if !self.parse_sub_attributes(&attrs) {
        return false;
      }
      let file_loc_conf: Option<&AttrConfig> = attrs.get("file_location_id");
      let file_loc_id = file_loc_conf.unwrap()
        .result().downcast_ref::<i64>().unwrap();

      let parent_frame_conf: Option<&AttrConfig> = attrs.get("parent_frame_id");
      let parent_frame_id = parent_frame_conf.unwrap()
        .result().downcast_ref::<i64>().unwrap();
      let stack_frame = StackFrame::new(
        *file_loc_id, *parent_frame_id);
      
      stack_frame_index.add_stack_frame(stack_frame);
    }
    true    
  }

  fn parse_stack_frame_index(&mut self, module: &mut HloModule) -> bool {
    if !self.eat_if_present(&TokKind::FileNames) {
      return true;
    }
    let mut stack_frame_index = StackFrameIndex::default();

    // Parse file names.
    while self.eat_if_present(&TokKind::Int) {
      let mut file_name = String::new();
      self.parse_string(&mut file_name);
      stack_frame_index.add_file_name(file_name);
    }

    // Parse function names.
    if !self.parse_token(&TokKind::FunctionNames,
      "expects FunctionNames".to_string())
    {
      return false;
    }

    while self.eat_if_present(&TokKind::Int) {
      let mut function_name = String::new();
      self.parse_string(&mut function_name);
      stack_frame_index.add_function_name(function_name);
    }

    // Parse file locations and stack frames.
    if !self.parse_token(
      &TokKind::FileLocations, "expects 'FileLocations'".to_string()) ||
      !self.parse_file_location_list(&mut stack_frame_index) ||
      !self.parse_token(
        &TokKind::StackFrames, "expects 'StackFrames'".to_string()) ||
      !self.parse_stack_frames_list(&mut stack_frame_index)
    {
      return false;
    }

    module.set_stack_frame_index(stack_frame_index);
    true
  }

  // computations ::= (computation)+
  fn parse_computations(&mut self, module: &mut HloModule) -> bool {
    let mut entry_computation = HloComputation::default();
    loop {
      if !self.parse_computation(&mut entry_computation) {
        return false;
      }
      if self.lexer.get_kind() == TokKind::Eof {
        break ;
      }
    }
    for i in 0..self.computations.len() {
      // If entry_computation is not nullptr, it means the computation it pointed
      // to is marked with "ENTRY"; otherwise, no computation is marked with
      // "ENTRY", and we use the last computation as the entry computation. We
      // add the non-entry computations as embedded computations to the module.
      if self.computations[i] != entry_computation {
        module.add_embedded_computation(self.computations[i].clone());
        continue;
      }
      module.add_entry_computation(self.computations[i].clone());
    }
    true
  }

  // computation ::= ('ENTRY')? name (param_list_to_shape)? instruction_list(,
  // 'execution_thread='execution_thread)?
  fn parse_computation(&mut self, entry_computation: &mut HloComputation) -> bool {
    //let maybe_entry_loc = self.lexer.get_loc();
    let is_entry_computation = self.eat_if_present(&TokKind::Entry);

    let mut name = String::new();
    let name_loc = self.lexer.get_loc();
    if !self.parse_name(&mut name) {
      return false;
    }
    let mut shape_loc: usize = 0;
    let mut shape = Shape::default();
    if self.can_be_param_list_to_shape() &&
      !self.parse_param_list_to_shape(&mut shape, &mut shape_loc)
    {
      return false;
    }
    let mut computation = HloComputation::default();
    if !self.parse_instruction_list(&mut computation, &mut name) {
      return false;
    }
    // If param_list_to_shape was present, check compatibility.
    if !ShapeUtil::compatible(
      computation.root_instruction().shape(), &shape)
    {
      assert!(false, "Shape of computation {:?} , {:?} is not compatible 
        with that of root instruction", name, ShapeUtil::human_string(&shape));
      return false;
    }

    let mut attrs: HashMap<String, AttrConfig> = HashMap::new();
    let execution_thread = String::new(); // TODO
    let exec_thread_conf = AttrConfig::new(
      false, AttrType::String, Box::new(Some(execution_thread.clone())));
    attrs.insert("execution_thread".to_string(), exec_thread_conf);

    if !self.parse_attributes(&mut attrs, true, &None) {
      return false;
    }

    computation.set_execution_thread(execution_thread);
    if is_entry_computation {
      *entry_computation = computation.clone();
    }

    self.add_computation(name, computation, name_loc)
  }

  // instruction_list ::= '{' instruction_list1 '}'
  // instruction_list1 ::= (instruction)+
  fn parse_instruction_list(
    &mut self,
    computation: &mut HloComputation,
    computation_name: &mut String) -> bool
  {
    self.scoped_name_tables.push(HashMap::new());

    let mut builder =
      HloComputationBuilder::new(computation_name.clone());
    if !self.parse_token(&TokKind::Lbrace,
      "expects '{' at the beginning of instruction list".to_string())
    {
      self.scoped_name_tables.pop();
      return false;
    }
    let mut root_name = String::new();
    loop {
      if !self.parse_instruction(&mut builder, &mut root_name) {
        self.scoped_name_tables.pop();
        return false;
      }
      if self.lexer.get_kind() == TokKind::Rbrace {
        break;
      }
    }
    if !self.parse_token(&TokKind::Rbrace,
      "expects '}' at the end of instruction list".to_string())
    {
      self.scoped_name_tables.pop();
      return false;
    }
    let mut root = HloInstruction::default();
    if !root_name.is_empty() {
      let root_node: Option<&(HloInstruction, usize)> =
        self.current_name_table().as_ref().unwrap().get(&root_name);
      // This means some instruction was marked as ROOT but we didn't find it in
      // the pool, which should not happen.
      if root_node.is_none() {
        assert!(false, "instruction {:?} was marked as ROOT but the parser has
          not seen it before", root_name);
        self.scoped_name_tables.pop();
        return false;
      }
      root = root_node.unwrap().0.clone();
    }
    // Now root can be either an existing instruction or a nullptr. If it's a
    // nullptr, the implementation of Builder will set the last instruction as
    // the root instruction.
    self.computations.push(builder.build(Some(&root)));
    *computation = self.computations.last().unwrap().clone();

    self.scoped_name_tables.pop();
    true
  }

  // instruction ::= ('ROOT')? name '=' shape opcode operands (attribute)*
  fn parse_instruction(
    &mut self, builder: &mut HloComputationBuilder, root_name: &mut String) -> bool
  {
    let mut name = String::new();
    //let maybe_rootloc = self.lexer.get_loc();
    let is_root = self.eat_if_present(&TokKind::Root);

    let name_loc = self.lexer.get_loc();
    if !self.parse_name(&mut name) ||
      !self.parse_token(&TokKind::Equal, "expects '=' in instruction".to_string())
    {
      return false;
    }
    if is_root {
      if !root_name.is_empty() {
        assert!(false, "one computation should have only one ROOT");
        return false;
      }
      *root_name = name.clone();
    }
    self.parse_instruction_rhs(builder, &mut name, name_loc, true)
  }

  fn parse_instruction_rhs(
    &mut self,
    builder: &mut HloComputationBuilder,
    name: &mut String,
    name_loc: usize,
    allow_attributes: bool) -> bool
  {
    self.current_recursion_depth += 1;
    if self.current_recursion_depth > self.options.max_recursion_depth() {
      assert!(false, "maximum recursion depth exceeded");
    }

    let mut shape = Shape::default();
    let mut opcode = HloOpcode::Abs;
    let mut async_wrapped_opcode = HloOpcode::Abs;
    //let mut operands = vec![];

    let parse_shape = self.can_be_shape();
    if parse_shape &&
      !self.parse_shape(&mut shape,
        true) ||
      !self.parse_opcode(&mut opcode,
        Some(&mut async_wrapped_opcode))
    {
      return false;
    }
    if !parse_shape && !can_infer_shape(&opcode) {
      assert!(false, "cannot infer shape for opcode: {:?}", opcode);
    }

    // Add optional attributes.
    // These are added to any HloInstruction type if present.
    let mut attrs: HashMap<String, AttrConfig> = HashMap::new();

    let sharding = HloSharding::default();
    let config_sharding = AttrConfig::new(
      false, AttrType::Sharding, Box::new(sharding.clone()));
    attrs.insert("sharding".to_string(), config_sharding);
    
    let frontend_attrs = FrontendAttributes::default();
    let config_front_attrs = AttrConfig::new(
      false, AttrType::FrontendAttributes, Box::new(frontend_attrs.clone()));
    attrs.insert("frontend_attributes".to_string(), config_front_attrs);

    let statistics_vis = StatisticsViz::default();
    let config_stats = AttrConfig::new(
      false, AttrType::StatisticsViz, Box::new(statistics_vis.clone()));
    attrs.insert("statistics".to_string(), config_stats);

    let param_replication = ParameterReplication::default();
    let config_param = AttrConfig::new(
      false, AttrType::ParameterReplication, Box::new(param_replication.clone()));
    attrs.insert("parameter_replication".to_string(), config_param);

    let mut predecessors: Vec<HloInstruction> = vec![];
    let config_pred = AttrConfig::new(
      false, AttrType::InstructionList, Box::new(predecessors.clone()));
    attrs.insert("control-predecessors".to_string(), config_pred);

    let original = OriginalValue::default();
    let config_orig = AttrConfig::new(
      false, AttrType::OriginalValue, Box::new(original.clone()));
    attrs.insert("origin".to_string(), config_orig);

    let metadata = OpMetadata::default();
    let config_metadata = AttrConfig::new(
      false, AttrType::Metadata, Box::new(metadata.clone()));
    attrs.insert("metadata".to_string(), config_metadata);

    let backend_config = String::new();
    let config_backend = AttrConfig::new(
      false, AttrType::StringOrJsonDict, Box::new(backend_config.clone()));
    attrs.insert("backend_config".to_string(), config_backend);

    let mut maybe_shape = None;
    if parse_shape {
      maybe_shape = Some(shape);
    }

    let instr_wrapper = self.create_instruction(
      builder, name, &mut maybe_shape, &opcode,
      Some(&async_wrapped_opcode), &mut attrs,
      allow_attributes, &vec![]);
    if instr_wrapper.is_none() {
      return false;
    }
    let mut instruction = instr_wrapper.unwrap();

    // Generate a unique name if the name is empty.  This is used for nested
    // instructions (e.g. the `max` in add(max(x, y), z)).
    //
    // Otherwise, register the given name with the name uniquer.
    if name.is_empty() {
      let mut uniquable_name = hlo_opcode_string(&instruction.opcode());
      uniquable_name.push_str(".anon");
      *name = self.name_uniquer.get_unique_name(&uniquable_name);
    } else {
      self.name_uniquer.get_unique_name(name);
    }

    instruction.set_and_sanitize_name(name);
    if instruction.name() != *name {
      assert!(false, "illegal instruction name: {:?}; suggest renaming to: {:?}",
        name, instruction.name());
      return false;
    }

    // Add shared attributes like metadata to the instruction, if they were seen.
    // sharding
    instruction.set_sharding(
      sharding.normalize_tuple_sharding(instruction.shape()));

    // parameter_replication
    let leaf_count = ShapeUtil::get_leaf_count(instruction.shape());
    let replicated = param_replication.replicated_at_leaf_buffers();
    if leaf_count != replicated.len() {
      assert!(false, "parameter has {:?} leaf buffers, but parameter_replication has
        {:?} elements", leaf_count, replicated.len());
      return false;
    }
    instruction.set_parameter_replicated_at_leaf_buffers(replicated.clone());

    // predecessors
    for pre in &mut predecessors {
      let status = pre.add_control_dependency_to(&instruction);
      if status.is_err() {
        assert!(false, "error adding control dependency for: {:?} status: {:?}",
          name, status.err().unwrap());
        return false;
      }
    }

    // metadata
    instruction.set_metadata(metadata.clone());
    if instruction.is_asynchronous() {
      instruction.async_wrapped_mutable_instruction()
        .set_metadata(metadata);
    }

    // original_value
    instruction.set_original_value(original.clone());
    if instruction.is_asynchronous() {
      instruction.async_wrapped_mutable_instruction()
        .set_original_value(original);
    }

    // backend_config
    instruction.set_raw_backend_config_string(backend_config.clone());
    if instruction.is_asynchronous() {
      instruction.async_wrapped_mutable_instruction()
        .set_raw_backend_config_string(backend_config);
    }

    // frontend_attributes // TODO
    //instruction.set_frontend_attributes(frontend_attrs.clone());
    //if instruction.is_asynchronous() {
      //instruction.async_wrapped_mutable_instruction()
        //.set_frontend_attributes(frontend_attrs);
    //}

    // statictics_vis
    instruction.set_statistics_vis(statistics_vis.clone());
    if instruction.is_asynchronous() {
      instruction.async_wrapped_mutable_instruction()
        .set_statistics_vis(statistics_vis);
    }

    self.add_instruction(name.clone(), instruction, name_loc)
  }

  fn parse_control_predecessors() {}

  // Similar to ParseLiteral(Literal* literal, const Shape& shape), but parse the
// shape instead of accepting one as argument.
  fn parse_literal(&mut self, _literal: &mut Literal) -> bool {
    false
  }

  // literal
  //  ::= tuple
  //  ::= non_tuple
  fn parse_literal_with_shape(&mut self, literal: &mut Literal, shape: &Shape) -> bool
  {
    if shape.is_tuple() {
      self.parse_tuple_literal(literal, shape)
    } else {
      self.parse_non_tuple_literal(literal, shape)
    }
  }

  // tuple
  //  ::= shape '(' literal_list ')'
  // literal_list
  //  ::= /*empty*/
  //  ::= literal (',' literal)*
  fn parse_tuple_literal(
    &mut self, literal: &mut Literal, shape: &Shape) -> bool
  {
    if self.parse_token(&TokKind::Lparen,
        "expects '(' in front of tuple elements".to_string())
    {
      return false;
    }

    let element_count = ShapeUtil::tuple_element_count(shape);
    let mut elements: Vec<Literal> = Vec::new();
    elements.reserve(element_count);
    if self.lexer.get_kind() == TokKind::Rparen {
      // empty
    } else {
      for i in 0..element_count {
        if i > 0 {
          self.parse_token(&TokKind::Comma,
            "expects ',' to separate tuple elements".to_string());
        }
        if !self.parse_literal_with_shape(&mut elements[i],
            ShapeUtil::get_tuple_element_shape(shape, i))
        {
          let mut err_msg = "expects the ".to_string();
          err_msg.push_str(&i.to_string());
          err_msg.push_str("th element");
          return self.token_error(err_msg);
        }
      }
    }
    *literal = LiteralUtil::make_tuple_owned(elements);

    self.parse_token(&TokKind::Rparen,
      "expects ')' at the end of the tuple with elements".to_string())
  }

  // non_tuple
  //   ::= rank01
  //   ::= rank2345
  // rank2345 ::= shape nested_array
  fn parse_non_tuple_literal(
    &mut self, literal: &mut Literal, shape: &Shape) -> bool
  {
    debug_assert!(LayoutUtil::is_dense_array(shape));
    self.parse_dense_literal(literal, shape)
  }

  fn parse_dense_literal(
    &mut self,
    literal: &mut Literal,
    shape: &Shape) -> bool
  {
    // Cast `rank` to int because we call shape.dimensions(int rank) below, and if
    // `rank` is an int64_t, that's an implicit narrowing conversion, which is
    // implementation-defined behavior.
    let rank = shape.dimensions_size();

    // Create a literal with the given shape in default layout.
    *literal = LiteralUtil::create_from_dimensions(
      &shape.element_type(),
      shape.dimensions_vec().clone());

    let mut nest_level = 0;
    let mut linear_index = 0;

    // elems_seen_per_dim[i] is how many elements or sub-arrays we have seen for
    // the dimension i. For example, to parse f32[2,3] {{1, 2, 3}, {4, 5, 6}},
    // when we are parsing the 2nd '{' (right before '1'), we are seeing a
    // sub-array of the dimension 0, so elems_seen_per_dim[0]++. When we are at
    // the first '}' (right after '3'), it means the sub-array ends, and the
    // sub-array is supposed to contain exactly 3 elements, so check if
    // elems_seen_per_dim[1] is 3.
    let mut elems_seen_per_dim = vec![0; rank];
    let get_index_str =
      |dim: usize, elems_seen_per_dim: &Vec<i64>| -> String
    {
      let mut elems_seen_untile_dim = vec![];
      elems_seen_untile_dim.clone_from_slice(&elems_seen_per_dim[0..dim]);
      let mut result = "[".to_string();
      let mut count = 0;
      for elem in &elems_seen_untile_dim {
        result.push_str(&elem.to_string());
        if count != elems_seen_untile_dim.len() {
          result.push_str(",");
        }
        count += 1;
      }
      result.push_str("]");
      result
    };

    let add_one_elem_seen =
      |parser: &mut HloParser,
       elems_seen_per_dim: &mut Vec<i64>,
       nest_level: usize| -> bool
    {
      if rank > 0 {
        if nest_level != rank {
          let mut err_msg = "expects nested array in rank ".to_string();
          err_msg.push_str(&rank.to_string());
          err_msg.push_str(", but sees ");
          err_msg.push_str(&nest_level.to_string());
          return parser.token_error(err_msg);
        }
        elems_seen_per_dim[rank-1] += 1;
        if elems_seen_per_dim[rank-1] > shape.dimensions_vec()[rank-1] {
          let mut err_msg = "expects ".to_string();
          err_msg.push_str(&shape.dimensions(rank-1).to_string());
          err_msg.push_str(" elements on the minor-most dimension, but sees more");
          return parser.token_error(err_msg);
        }
      }
      true
    };
    
    loop {
        match self.lexer.get_kind() {
          TokKind::Lbrace => {
            nest_level += 1;
            if nest_level > rank {
              let mut err_msg = "expects nested array in rank ".to_string();
              err_msg.push_str(&rank.to_string());
              err_msg.push_str(", but sees larger");
              return self.token_error(err_msg);
            }
            if nest_level > 1 {
              elems_seen_per_dim[nest_level-2] += 1;
              if elems_seen_per_dim[nest_level-2] > shape.dimensions(nest_level-2) {
                let mut err_msg = "expects ".to_string();
                err_msg.push_str(&shape.dimensions(nest_level-2).to_string());
                err_msg.push_str(" elements in the ");
                err_msg.push_str(&get_index_str(nest_level-2, &elems_seen_per_dim));
                err_msg.push_str("th element, but sees more");
                return self.token_error(err_msg);
              }
            }
            self.lexer.lex(0);
          }
          TokKind::Rbrace => {
            if nest_level == 0 {
              return self.token_error("unexpected '}' token".to_string());
            }
            nest_level -= 1;
            if elems_seen_per_dim[nest_level] != shape.dimensions(nest_level) {
              let mut err_msg = "expects ".to_string();
              err_msg.push_str(&shape.dimensions(nest_level).to_string());
              err_msg.push_str(" elements in the ");
              err_msg.push_str(&get_index_str(nest_level, &elems_seen_per_dim));
              err_msg.push_str("th element, but sees ");
              err_msg.push_str(&elems_seen_per_dim[nest_level].to_string());
              return self.token_error(err_msg);
            }
            elems_seen_per_dim[nest_level] = 0;
            self.lexer.lex(0);
          }
          TokKind::Lparen => {
            if !is_complex_type(&shape.element_type()) {
              let err_msg = "unexpected '(' in literal. Parens are only valid
                for completx literals".to_string();
              return self.token_error(err_msg);
            }
            linear_index += 1;
            let mut value: Complex64 = Complex64::new(0.0, 0.0);
            let loc = self.lexer.get_loc();
            if !add_one_elem_seen(self, &mut elems_seen_per_dim, nest_level) ||
              !self.parse_complex(&mut value) ||
              !self.set_value_in_literal_c64(loc, value, linear_index, literal)
            {
              return false;
            }
          }
          TokKind::Dots => {
            unimplemented!()
          }
          TokKind::Comma => {
            // Skip.
            self.lexer.lex(0);
          }
          TokKind::True => {
            add_one_elem_seen(self, &mut elems_seen_per_dim, nest_level);
            if self.lexer.get_kind() == TokKind::True ||
              self.lexer.get_kind() == TokKind::False
            {
              if !self.set_value_in_literal_bool(
                self.lexer.get_loc(), self.lexer.get_kind() == TokKind::True,
                linear_index, literal)
              {
                return false;
              }
              linear_index += 1;
              self.lexer.lex(0);
            } else if is_integral_type(&shape.element_type()) ||
                shape.element_type() == PrimitiveType::Pred
            {
              let loc = self.lexer.get_loc();
              let mut value = 0;
              if !self.parse_i64(&mut value) {
                assert!(false, "expects integer for primitive type: {:?}",
                  primitive_type_name(&shape.element_type()));
                return false;
              }
              if !self.set_value_in_literal_i64(loc, value, linear_index, literal) {
                return false;
              }
              linear_index += 1;
            } else if is_floating_point_type(&shape.element_type()) {
              let loc = self.lexer.get_loc();
              let mut value = 0.0;
              if !self.parse_double(&mut value) {
                assert!(false, "expects floating point value for primitive type: {:?}",
                  primitive_type_name(&shape.element_type()));
                return false;
              }
              if !self.set_value_in_literal_f64(loc, value, linear_index, literal) {
                return false;
              }
              linear_index += 1;
            } else {
              let mut err_msg = "unsupported primitive type ".to_string();
              err_msg.push_str(&primitive_type_name(&shape.element_type()));
              return self.token_error(err_msg);
            }
          }
          TokKind::False => {
            add_one_elem_seen(self, &mut elems_seen_per_dim, nest_level);
            if self.lexer.get_kind() == TokKind::True ||
              self.lexer.get_kind() == TokKind::False
            {
              if !self.set_value_in_literal_bool(
                self.lexer.get_loc(), self.lexer.get_kind() == TokKind::True,
                linear_index, literal)
              {
                return false;
              }
              linear_index += 1;
              self.lexer.lex(0);
            } else if is_integral_type(&shape.element_type()) ||
                shape.element_type() == PrimitiveType::Pred
            {
              let loc = self.lexer.get_loc();
              let mut value = 0;
              if !self.parse_i64(&mut value) {
                assert!(false, "expects integer for primitive type: {:?}",
                  primitive_type_name(&shape.element_type()));
                return false;
              }
              if !self.set_value_in_literal_i64(loc, value, linear_index, literal) {
                return false;
              }
              linear_index += 1;
            } else if is_floating_point_type(&shape.element_type()) {
              let loc = self.lexer.get_loc();
              let mut value = 0.0;
              if !self.parse_double(&mut value) {
                assert!(false, "expects floating point value for primitive type: {:?}",
                  primitive_type_name(&shape.element_type()));
                return false;
              }
              if !self.set_value_in_literal_f64(loc, value, linear_index, literal) {
                return false;
              }
              linear_index += 1;
            } else {
              let mut err_msg = "unsupported primitive type ".to_string();
              err_msg.push_str(&primitive_type_name(&shape.element_type()));
              return self.token_error(err_msg);
            }
          }
          TokKind::Int => {
            add_one_elem_seen(self, &mut elems_seen_per_dim, nest_level);
            if self.lexer.get_kind() == TokKind::True ||
              self.lexer.get_kind() == TokKind::False
            {
              if !self.set_value_in_literal_bool(
                self.lexer.get_loc(), self.lexer.get_kind() == TokKind::True,
                linear_index, literal)
              {
                return false;
              }
              linear_index += 1;
              self.lexer.lex(0);
            } else if is_integral_type(&shape.element_type()) ||
                shape.element_type() == PrimitiveType::Pred
            {
              let loc = self.lexer.get_loc();
              let mut value = 0;
              if !self.parse_i64(&mut value) {
                assert!(false, "expects integer for primitive type: {:?}",
                  primitive_type_name(&shape.element_type()));
                return false;
              }
              if !self.set_value_in_literal_i64(loc, value, linear_index, literal) {
                return false;
              }
              linear_index += 1;
            } else if is_floating_point_type(&shape.element_type()) {
              let loc = self.lexer.get_loc();
              let mut value = 0.0;
              if !self.parse_double(&mut value) {
                assert!(false, "expects floating point value for primitive type: {:?}",
                  primitive_type_name(&shape.element_type()));
                return false;
              }
              if !self.set_value_in_literal_f64(loc, value, linear_index, literal) {
                return false;
              }
              linear_index += 1;
            } else {
              let mut err_msg = "unsupported primitive type ".to_string();
              err_msg.push_str(&primitive_type_name(&shape.element_type()));
              return self.token_error(err_msg);
            }
          }
          TokKind::Decimal => {
            add_one_elem_seen(self, &mut elems_seen_per_dim, nest_level);
            if self.lexer.get_kind() == TokKind::True ||
              self.lexer.get_kind() == TokKind::False
            {
              if !self.set_value_in_literal_bool(
                self.lexer.get_loc(), self.lexer.get_kind() == TokKind::True,
                linear_index, literal)
              {
                return false;
              }
              linear_index += 1;
              self.lexer.lex(0);
            } else if is_integral_type(&shape.element_type()) ||
                shape.element_type() == PrimitiveType::Pred
            {
              let loc = self.lexer.get_loc();
              let mut value = 0;
              if !self.parse_i64(&mut value) {
                assert!(false, "expects integer for primitive type: {:?}",
                  primitive_type_name(&shape.element_type()));
                return false;
              }
              if !self.set_value_in_literal_i64(loc, value, linear_index, literal) {
                return false;
              }
              linear_index += 1;
            } else if is_floating_point_type(&shape.element_type()) {
              let loc = self.lexer.get_loc();
              let mut value = 0.0;
              if !self.parse_double(&mut value) {
                assert!(false, "expects floating point value for primitive type: {:?}",
                  primitive_type_name(&shape.element_type()));
                return false;
              }
              if !self.set_value_in_literal_f64(loc, value, linear_index, literal) {
                return false;
              }
              linear_index += 1;
            } else {
              let mut err_msg = "unsupported primitive type ".to_string();
              err_msg.push_str(&primitive_type_name(&shape.element_type()));
              return self.token_error(err_msg);
            }
          }
          TokKind::Inf => {
            add_one_elem_seen(self, &mut elems_seen_per_dim, nest_level);
            if self.lexer.get_kind() == TokKind::True ||
              self.lexer.get_kind() == TokKind::False
            {
              if !self.set_value_in_literal_bool(
                self.lexer.get_loc(), self.lexer.get_kind() == TokKind::True,
                linear_index, literal)
              {
                return false;
              }
              linear_index += 1;
              self.lexer.lex(0);
            } else if is_integral_type(&shape.element_type()) ||
                shape.element_type() == PrimitiveType::Pred
            {
              let loc = self.lexer.get_loc();
              let mut value = 0;
              if !self.parse_i64(&mut value) {
                assert!(false, "expects integer for primitive type: {:?}",
                  primitive_type_name(&shape.element_type()));
                return false;
              }
              if !self.set_value_in_literal_i64(loc, value, linear_index, literal) {
                return false;
              }
              linear_index += 1;
            } else if is_floating_point_type(&shape.element_type()) {
              let loc = self.lexer.get_loc();
              let mut value = 0.0;
              if !self.parse_double(&mut value) {
                assert!(false, "expects floating point value for primitive type: {:?}",
                  primitive_type_name(&shape.element_type()));
                return false;
              }
              if !self.set_value_in_literal_f64(loc, value, linear_index, literal) {
                return false;
              }
              linear_index += 1;
            } else {
              let mut err_msg = "unsupported primitive type ".to_string();
              err_msg.push_str(&primitive_type_name(&shape.element_type()));
              return self.token_error(err_msg);
            }
          }
          TokKind::NegInf => {
            add_one_elem_seen(self, &mut elems_seen_per_dim, nest_level);
            if self.lexer.get_kind() == TokKind::True ||
              self.lexer.get_kind() == TokKind::False
            {
              if !self.set_value_in_literal_bool(
                self.lexer.get_loc(), self.lexer.get_kind() == TokKind::True,
                linear_index, literal)
              {
                return false;
              }
              linear_index += 1;
              self.lexer.lex(0);
            } else if is_integral_type(&shape.element_type()) ||
                shape.element_type() == PrimitiveType::Pred
            {
              let loc = self.lexer.get_loc();
              let mut value = 0;
              if !self.parse_i64(&mut value) {
                assert!(false, "expects integer for primitive type: {:?}",
                  primitive_type_name(&shape.element_type()));
                return false;
              }
              if !self.set_value_in_literal_i64(loc, value, linear_index, literal) {
                return false;
              }
              linear_index += 1;
            } else if is_floating_point_type(&shape.element_type()) {
              let loc = self.lexer.get_loc();
              let mut value = 0.0;
              if !self.parse_double(&mut value) {
                assert!(false, "expects floating point value for primitive type: {:?}",
                  primitive_type_name(&shape.element_type()));
                return false;
              }
              if !self.set_value_in_literal_f64(loc, value, linear_index, literal) {
                return false;
              }
              linear_index += 1;
            } else {
              let mut err_msg = "unsupported primitive type ".to_string();
              err_msg.push_str(&primitive_type_name(&shape.element_type()));
              return self.token_error(err_msg);
            }
          }
          _ => return self.token_error("unexpected token type in a literal".to_string())
        }
        if nest_level<= 0 {
          break;
        }
    }

    *literal = literal.relayout(
      shape.layout().as_ref().unwrap(), &vec![]);
    true
  }

  fn create_instruction(
    &mut self,
    builder: &mut HloComputationBuilder,
    name: &String,
    shape: &mut Option<Shape>,
    opcode: &HloOpcode,
    _async_wrapped_opcode: Option<&HloOpcode>,
    attrs: &mut HashMap<String, AttrConfig>,
    allow_attributes: bool,
    preset_operands: &Vec<HloInstruction>) -> Option<HloInstruction>
  {
    let mut operands = vec![];
    if !preset_operands.is_empty() {
      operands.clone_from(preset_operands);
    }

    let maybe_infer_shape =
      |parser: &mut HloParser,
       shape: &mut Option<Shape>,
       infer: &dyn Fn()->Result<Shape, String>| -> bool
    {
      if shape.is_some() {
        return true;
      }
      let inferred = infer();
      if inferred.is_err() {
        let mut err_msg = "failed to infer shape for opcode:".to_string();
        err_msg.push_str(&hlo_opcode_string(opcode));
        err_msg.push_str(&inferred.err().unwrap());
        return parser.token_error(err_msg);
      }
      *shape = Some(inferred.unwrap());
      true
    };

    let mut create_unary_instruction =
      |parser: &mut HloParser,
       shape: &mut Option<Shape>,
       builder: &mut HloComputationBuilder| -> Option<HloInstruction>
    {
      if preset_operands.is_empty() &&
        parser.parse_operands(&operands, builder) ||
        parser.parse_attributes(attrs, allow_attributes, shape)
      {
        return None;
      }
      let infer = || -> Result<Shape, String> {
        ShapeInference::infer_unary_op_shape(opcode, &operands[0])
      };
      if !maybe_infer_shape(parser, shape, &infer) {
        return None;
      }
      Some(builder.add_instruction(HloInstruction::create_unary(
        &shape.as_ref().unwrap(), opcode,
        &operands[0], None)).clone())
    };

    let create_unary_instruction_with_result_accuracy =
     |parser: &mut HloParser,
      shape: &mut Option<Shape>,
      builder: &mut HloComputationBuilder,
      attrs: &mut HashMap<String, AttrConfig>| -> Option<HloInstruction>
    {
      let result_accuracy: Option<ResultAccuracy> = None;
      let result_acc_config = AttrConfig::new(
        false, AttrType::ResultAccuracy, Box::new(result_accuracy));
      attrs.insert("result_accuracy".to_string(), result_acc_config);
      if preset_operands.is_empty() &&
        !parser.parse_operands(&operands, builder) ||
        !parser.parse_attributes(attrs, allow_attributes, shape)
      {
        return None;
      }
      let infer = || -> Result<Shape, String> {
        ShapeInference::infer_unary_op_shape(opcode, &operands[0])
      };
      if !maybe_infer_shape(parser, shape, &infer) {
        return None;
      }
      let mut accuracy = ResultAccuracy::default();
      let result_acc_config: &AttrConfig =
        attrs.get(&"result_accuracy".to_string()).unwrap();
      let result_acc =
        result_acc_config.result.as_ref().downcast_ref::<ResultAccuracy>();
      if result_acc.is_some() {
        accuracy = result_acc.unwrap().clone();
      } else {
        accuracy.set_mode(ResultAccuracyMode::Default);
      }
      Some(builder.add_instruction(HloInstruction::create_unary(
        &shape.as_ref().unwrap(), opcode,
        &operands[0], Some(accuracy))).clone())
    };

    let  create_binary_instruction =
      |parser: &mut HloParser,
       shape: &mut Option<Shape>,
       builder: &mut HloComputationBuilder,
       attrs: &mut HashMap<String, AttrConfig>| -> Option<HloInstruction>
    {
      if preset_operands.is_empty() &&
        !parser.parse_operands(&operands, builder) ||
        !parser.parse_attributes(attrs, allow_attributes, shape)
      {
        return None;
      }
      let infer = || -> Result<Shape, String> {
        ShapeInference::infer_binary_op_shape(
          opcode, &operands[0], &operands[1])
      };
      if !maybe_infer_shape(parser, shape, &infer) {
        return None;
      }
      Some(builder.add_instruction(HloInstruction::create_binary(
        &shape.as_ref().unwrap(), opcode,
        &operands[0], &operands[1])).clone())
    };

    match opcode {
      HloOpcode::Parameter => {
        let mut parameter_number = 0;
        if !self.parse_token(&TokKind::Lparen,
          "expects '(' before parameter number".to_string()) ||
          !self.parse_i64(&mut parameter_number)
        {
          return None;
        }
        //let loc = self.lexer.get_loc();
        if parameter_number < 0 {
          assert!(false, "parameter number must be >= 0");
          return None;
        }
        if !self.parse_token(&TokKind::Rparen,
          "expects ')' after parameter number".to_string()) ||
          !self.parse_attributes(attrs, allow_attributes, shape)
        {
          return None;
        }
        let param_name = name.clone();
        let result = builder.add_parameter(
          HloInstruction::create_parameter(
            parameter_number, shape.as_ref().unwrap(), param_name));
        if result.is_err() {
          return None;
        }
        Some(result.ok().unwrap().clone())
      }
      HloOpcode::Constant => {
        let mut literal: Literal = Literal::default();
        if !self.parse_token(&TokKind::Lparen,
          "expects '(' before constant literal".to_string()) ||
          !self.parse_literal_with_shape(&mut literal, shape.as_ref().unwrap()) ||
          !self.parse_token(&TokKind::Rparen,
            "expects ')' after constant literal".to_string()) ||
          !self.parse_attributes(attrs, allow_attributes, shape)
        {
          return None;
        }
        Some(builder.add_instruction(HloInstruction::create_constant(literal)).clone())
      }
      HloOpcode::Iota => {
        let iota_dimension: Option<i64> = None;
        let iota_dim_config = AttrConfig::new(
          true, AttrType::Int64, Box::new(iota_dimension));
        attrs.insert("iota_dimension".to_string(), iota_dim_config);
        if preset_operands.is_empty() &&
          !self.parse_operands(&operands, builder) ||
          !self.parse_attributes(attrs, allow_attributes, shape)
        {
          return None;
        }
        Some(builder.add_instruction(HloInstruction::create_iota(
          shape.as_ref().unwrap(), iota_dimension.unwrap())).clone())
      }
      HloOpcode::TopK => {
        unimplemented!()
      }
      // Unary ops with result accuracy.
      HloOpcode::Acos =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Acosh =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Asin =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Asinh =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Atanh =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Expm1 =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Log =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Log1p =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Logistic =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Sqrt =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Cbrt =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Rsqrt =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Tanh =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Erf =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Sin =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Cos =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Cosh =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Tan =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),
      HloOpcode::Exp =>
        return create_unary_instruction_with_result_accuracy(
          self, shape, builder, attrs),

      // Unary ops.
      HloOpcode::Abs =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::AllGatherDone =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::AllReduceDone =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::RoundNearestAfz =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::RoundNearestEven =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::Bitcast =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::Ceil =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::Clz =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::CollectivePermuteDone =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::Copy =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::CopyDone =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::OptimizationBarrier =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::Imag =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::IsFinite =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::Floor =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::Not =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::Negate =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::PopulationCount =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::Real =>
        return create_unary_instruction(self, shape, builder),
      HloOpcode::Sign =>
        return create_unary_instruction(self, shape, builder),

      // Binary ops.
      HloOpcode::Add =>
        return create_binary_instruction(self, shape, builder, attrs),
      HloOpcode::Divide =>
        return create_binary_instruction(self, shape, builder, attrs),
      HloOpcode::Multiply =>
        return create_binary_instruction(self, shape, builder, attrs),
      HloOpcode::Subtract =>
        return create_binary_instruction(self, shape, builder, attrs),
      HloOpcode::Atan2 =>
        return create_binary_instruction(self, shape, builder, attrs),
      HloOpcode::Complex =>
        return create_binary_instruction(self, shape, builder, attrs),
      HloOpcode::Maximum =>
        return create_binary_instruction(self, shape, builder, attrs),
      HloOpcode::Minimum =>
        return create_binary_instruction(self, shape, builder, attrs),
      HloOpcode::Power =>
        return create_binary_instruction(self, shape, builder, attrs),
      HloOpcode::Remainder =>
        return create_binary_instruction(self, shape, builder, attrs),
      HloOpcode::And =>
        return create_binary_instruction(self, shape, builder, attrs),
      HloOpcode::Or =>
        return create_binary_instruction(self, shape, builder, attrs),
      HloOpcode::Xor =>
        return create_binary_instruction(self, shape, builder, attrs),
      HloOpcode::ShiftLeft =>
        return create_binary_instruction(self, shape, builder, attrs),
      HloOpcode::ShiftRightArithmetic =>
        return create_binary_instruction(self, shape, builder, attrs),
      HloOpcode::ShiftRightLogical =>
        return create_binary_instruction(self, shape, builder, attrs),
      HloOpcode::StochasticConvert =>
        return create_binary_instruction(self, shape, builder, attrs),

      // Ternary ops.
      HloOpcode::Clamp => {
        if preset_operands.is_empty() &&
          !self.parse_operands(&operands, builder) ||
          !self.parse_attributes(attrs, allow_attributes, shape)
        {
          return None;
        }
        let infer = || -> Result<Shape, String> {
          ShapeInference::infer_ternary_op_shape(
            opcode, &operands[0], &operands[1], &operands[2])
        };
        if !maybe_infer_shape(self, shape, &infer) {
          return None;
        }
        Some(builder.add_instruction(HloInstruction::create_ternary(
          shape.as_ref().unwrap(), opcode, &operands[0],
          &operands[1], &operands[2])).clone())
      }
      HloOpcode::Select => {
        if preset_operands.is_empty() &&
          !self.parse_operands(&operands, builder) ||
          !self.parse_attributes(attrs, allow_attributes, shape)
        {
          return None;
        }
        let infer = || -> Result<Shape, String> {
          ShapeInference::infer_ternary_op_shape(
            opcode, &operands[0], &operands[1], &operands[2])
        };
        if !maybe_infer_shape(self, shape, &infer) {
          return None;
        }
        Some(builder.add_instruction(HloInstruction::create_ternary(
          shape.as_ref().unwrap(), opcode, &operands[0],
          &operands[1], &operands[2])).clone())
      }

      // Other supported ops.
      HloOpcode::Convert => {
        if preset_operands.is_empty() &&
          !self.parse_operands(&operands, builder) ||
          !self.parse_attributes(attrs, allow_attributes, shape)
        {
          return None;
        }
        Some(builder.add_instruction(HloInstruction::create_convert(
          shape.as_ref().unwrap(), &operands[0])).clone())
      }
      HloOpcode::BitcastConvert => {
        if preset_operands.is_empty() &&
          !self.parse_operands(&operands, builder) ||
          !self.parse_attributes(attrs, allow_attributes, shape)
        {
          return None;
        }
        Some(builder.add_instruction(HloInstruction::create_bitcast_convert(
          shape.as_ref().unwrap(), &operands[0])).clone())
      }
      _ => return None
    }
  }

  fn get_real<T>(value: T) -> T {
    value
  }

  fn is_finite(_value: i64) -> bool {
    unimplemented!()
  }

  fn check_parsed_value_is_in_range(_loc: usize, _value: i64) -> bool {
    unimplemented!()
  }

  fn set_value_in_literal_helper<ParsedElemT>(
    &mut self,
    loc: usize,
    value: ParsedElemT,
    index: i64,
    literal: &mut Literal) -> bool
      where ParsedElemT: 'static
  {
    if index >= ShapeUtil::elements_in(literal.shape()) {
      let mut err_msg =
        "tries to set value to a literal in shape at linear index ".to_string();
      err_msg.push_str(&index.to_string());
      err_msg.push_str("but the index is out of range");
      return self.error(loc, err_msg);
    }
    let _handle_nan = || {};

    literal.mutable_data(&vec![])[index as usize] = Box::new(value);
    true  
  }

  fn set_value_in_literal_i64(
    &mut self,
    loc: usize,
    value: i64,
    index: i64,
    literal: &mut Literal) -> bool
  {
    let elt_t = literal.shape().element_type().clone();
    let mut f = |t: PrimitiveType| -> bool {
      if t == PrimitiveType::Pred {
        let mut bool_v = true;
        if value == 0 { bool_v = false; }
         return self.set_value_in_literal_helper(loc, bool_v, index, literal);
      }
      if is_integral_type(&t) {
        return self.set_value_in_literal_helper(loc, value, index, literal);
      }
      assert!(false, "unknown integral primitive type {:?}",
        primitive_type_name(&literal.shape().element_type()));
      false
    };
    primitive_type_switch(&mut f, elt_t)
  }

  fn set_value_in_literal_f64(
    &self,
    _loc: usize,
    _value: f64,
    _index: i64,
    _literal: &mut Literal) -> bool
  {
    unimplemented!()
  }

  fn set_value_in_literal_bool(
    &self,
    _loc: usize,
    _value: bool,
    _index: i64,
    _literal: &mut Literal) -> bool
  {
    unimplemented!()
  }

  fn set_value_in_literal_c64(
    &self,
    _loc: usize,
    _value: Complex64,
    _index: i64,
    _literal: &mut Literal) -> bool
  {
    unimplemented!()
  }

  // operands ::= '(' operands1 ')'
  // operands1
  //   ::= /*empty*/
  //   ::= operand (, operand)*
  // operand ::= (shape)? name
  //         ::= (shape)? opcode operands
  fn parse_operands(
    &mut self,
    _operands: &Vec<HloInstruction>,
    _builder: &HloComputationBuilder) -> bool
  {
    unimplemented!()
  }

  // attributes ::= (',' attribute)*
  //
  // Parses attributes given names and configs of the attributes. Each parsed
  // result is passed back through the result pointer in corresponding
  // AttrConfig. Note that the result pointer must point to a optional<T> typed
  // variable which outlives this function. Returns false on error. You should
  // not use the any of the results if this function failed.
  //
  // If allow_attributes is false, returns an error if any attributes are
  // present.  This is used for contexts in which attributes are not allowed but
  // e.g. we *also* want to raise an error if any required attributes are
  // missing.
  //
  // Example usage:
  //
  //  absl::flat_hash_map<std::string, AttrConfig> attrs;
  //  optional<int64_t> foo;
  //  attrs["foo"] = {/*required=*/false, AttrTy::kInt64, &foo};
  //  optional<Window> bar;
  //  attrs["bar"] = {/*required=*/true, AttrTy::kWindow, &bar};
  //  if (!ParseAttributes(attrs)) {
  //    return false; // Do not use 'foo' 'bar' if failed.
  //  }
  //  // Do something with 'bar'.
  //  if (foo) { // If attr foo is seen, do something with 'foo'. }
  //
  fn parse_attributes(
    &mut self,
    attrs: &mut HashMap<String, AttrConfig>,
    allow_attributes: bool,
    shape: &Option<Shape>) -> bool
  {
    //let loc = self.lexer.get_loc();
    let mut seen_attrs = HashSet::new();
    if allow_attributes {
      while self.eat_if_present(&TokKind::Comma) {
        if !self.parse_attribute_helper(attrs, &mut seen_attrs, shape) {
          return false;
        }
      }
    }
    // Check that all required attrs were seen.
    for attr in attrs {
      if attr.1.required && seen_attrs.get(attr.0) == None {
        assert!(false, "attribute {:?} is expected but not seen", attr.0);
        return false;
      }
    }
    true
  }

  fn parse_attribute_helper(
    &mut self,
    attrs: &mut HashMap<String, AttrConfig>,
    seen_attrs: &mut HashSet<String>,
    _shape: &Option<Shape>) -> bool
  {
    let loc = self.lexer.get_loc();
    let mut name = String::new();
    if !self.parse_attribute_name(&mut name) {
      assert!(false, "error parsing attributes [loc: {:?}]", loc);
    }
    println!("Parsing attribute {:?}", name);
    if !seen_attrs.insert(name.clone()) {
      assert!(false, "attribute {:?} already exists [loc: {:?}]",name, loc);
    }
    let attrs_is_empty = attrs.is_empty();
    let value = attrs.get_mut(&name);
    if value.is_none() {
      let mut allowed_attrs = String::new();
      if attrs_is_empty {
        allowed_attrs = "No attributes are allowed here.".to_string();
      } else {
        // TODO
      }
      assert!(false, "unexpected attribute {:?}. {:?}", name, allowed_attrs);
    }
    let mut success = |attr_config: &mut AttrConfig| -> bool {
      match attr_config.attr_type {
        AttrType::Bool => {
          let mut result = false;
          if !self.parse_bool(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::Int64 => {
          let mut result = 0;
          if !self.parse_i64(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::Int32 => {
          let result: i32 = 0;
          if !self.parse_i64(&mut (result as i64)) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::Float => {
          let mut result = 0.0;
          if !self.parse_double(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::HloComputation => {
          let mut result = HloComputation::default();
          if !self.parse_hlo_computation(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::BracedHloComputationList => {
          let mut result = vec![];
          if !self.parse_hlo_computation_list(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::FftType => {
          let mut result = FftType::FFT;
          if !self.parse_fft_type(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::PaddingType => {
          let mut result = PaddingType::Invalid;
          if !self.parse_padding_type(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::ComparisonDirection => {
          let mut result = ComparisonDirection::Eq;
          if !self.parse_comparison_direction(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::ComparisonType => {
          let mut result = ComparisonType::Float;
          if !self.parse_comparison_type(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::Enum => {
          if self.lexer.get_kind() != TokKind::Ident {
            assert!(false, "expects an enumeration value");
            return false;
          }
          let result = self.lexer.get_str_val();
          self.lexer.lex(0);
          attr_config.result = Box::new(result);
          true
        }
        AttrType::Window => {
          let mut result = Window::default();
          if !self.parse_window(&mut result, true) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::ConvolutionDimensionNumbers => {
          let mut result =
            ConvolutionDimensionNumbers::default();
          if !self.parse_convolution_demension_numbers(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::Sharding => {
          let mut result = HloSharding::default();
          if !self.parse_sharding(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::CollectiveDeviceList =>{ // TODO
          true
        }
        AttrType::FrontendAttributes => {
          let mut result = FrontendAttributes::default();
          if !self.parse_frontend_attributes(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::StatisticsViz => {
          let mut result = StatisticsViz::default();
          if !self.parse_statistics_viz(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::ParameterReplication => {
          let mut result = ParameterReplication::default();
          if !self.parse_parameter_replication(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::InstructionList => {
          let mut result = vec![];
          if !self.parse_instruction_names(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::FusionKind => {
          let mut result = FusionKind::Custom;
          if !self.parse_fusion_kind(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        // TODO: ConvKind
        AttrType::BracedInt64List => {
          let mut result = vec![];
          if !self.parse_i64_list(&TokKind::Lbrace, &TokKind::Rbrace,
            &TokKind::Comma, &mut result)
          {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::BracedInt64ListList => {
          let mut result = vec![];
          if !self.parse_i64_list_list(&TokKind::Lbrace, &TokKind::Rbrace,
            &TokKind::Comma, &mut result)
          {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        // TODO: SliceRanges
        AttrType::PaddingConfig => {
          let mut result = PaddingConfig::default();
          if !self.parse_padding_config(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::String => {
          let mut result = String::new();
          if !self.parse_string(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::StringOrJsonDict => {
          let mut result = String::new();
          if self.lexer.get_kind() == TokKind::String {
            if !self.parse_string(&mut result) {
              return false;
            }
          } else if self.lexer.get_kind() == TokKind::Lbrace {
            if !self.parse_json_dict(&mut result) {
              return false;
            }
          } else {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::OriginalValue => {
          true
        }
        AttrType::OriginalValueRecoveryTable => {
          let mut result =
            OriginalValueRecoveryTable::default();
          if !self.parse_original_value_recovery_table(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::Metadata => {
          let mut result = OpMetadata::default();
          if !self.parse_metadata(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::Distribution => {
          let mut result = RandomDistribution::Invalid;
          if !self.parse_random_distribution(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::Domain => {
          true
        }
        AttrType::PrecisionList => {
          let mut result = vec![];
          if !self.parse_precision_list(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::Shape => {
          let mut result = Shape::default();
          if !self.parse_shape(&mut result, true) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::ShapeList => {
          let mut result = vec![];
          if !self.parse_shape_list(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::RandomAlgorithm => {
          let mut result = RandomAlgorithm::Default;
          if !self.parse_random_algorithm(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::PrecisionAlgorithm => {
          let mut result = Algorithm::Unset;
          if !self.parse_algorithm(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::ResultAccuracyType => {
          let mut result = ResultAccuracyMode::Default;
          if !self.parse_result_accuracy_type(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::Aliasing => {
          let mut aliasing_data = BTreeMap::new();
          if !self.parse_aliasing(&mut aliasing_data) {
            return false;
          }
          attr_config.result = Box::new(aliasing_data);
          true
        }
        AttrType::BufferDonor => {
          true
        }
        AttrType::ComputationLayout => {
          let mut computation_layout =
            ComputationLayout::new(ShapeLayout::new(Shape::default()));
          if !self.parse_computation_layout(&mut computation_layout) {
            return false;
          }
          attr_config.result = Box::new(computation_layout);
          true
        }
        AttrType::InstructionAliasing => {
          let mut aliasing_output_operand_pairs = vec![];
          if !self.parse_instruction_output_operand_aliasing(
            &mut aliasing_output_operand_pairs)
          {
            return false;
          }
          attr_config.result = Box::new(aliasing_output_operand_pairs);
          true
        }
        AttrType::Literal => {
          let mut result = Literal::default();
          if !self.parse_literal(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::CustomCallSchedule => {
          let mut result = CustomCallSchedule::None;
          if !self.parse_parse_custom_call_schedule(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::CustomCallApiVersion => {
          let mut result = CustomCallApiVersion::Unspecified;
          if !self.parse_custom_call_api_version(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::ResultAccuracy => {
          let mut result = ResultAccuracy::default();
          if !self.parse_result_accuracy(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        AttrType::Mode => {
          let mut result = CollectiveOpGroupMode::CrossPartition;
          if !self.parse_collective_op_group_mode(&mut result) {
            return false;
          }
          attr_config.result = Box::new(result);
          true
        }
        // TODO: SparsityConfig
        _ => return false
      }
    };
    if !success(value.unwrap()) {
      assert!(false, "error parsing attribute {:?}", name);
    }
    true
  }

  // sub_attributes ::= '{' (','? attribute)* '}'
  //
  // Usage is the same as ParseAttributes. See immediately above.
  fn parse_sub_attributes(
    &mut self, _attrs: &HashMap<String, AttrConfig>) -> bool
  {
    unimplemented!()
  }

  fn copy_attribute_to_proto_message() {}

  fn parse_attributes_as_proto_message() {}

  fn parse_computation_name(&mut self, value: &mut HloComputation) -> bool {
    let mut name = "".to_string();
    let loc = self.lexer.get_loc();
    if !self.parse_name(&mut name) {
      return self.error(loc, "expects computation name".to_string());
    }
    let computation = self.computation_pool.get(&name);
    if computation.is_none() {
      let mut err_msg = "computation does not exist: ".to_string();
      err_msg.push_str(&name);
      return self.error(loc, err_msg);
    }
    *value = computation.unwrap().0.clone();
    true
  }

  // '{' name+ '}'
  fn parse_instruction_names(&mut self, instructions: &mut Vec<HloInstruction>) -> bool {
    if self.parse_token(&TokKind::Lbrace,
      "expects '{' at the beginning of instruction name list".to_string())
    {
      return false;
    }
    let loc = self.lexer.get_loc();
    loop {
      let mut name = "".to_string();
      if !self.parse_name(&mut name) {
        return self.error(loc, "expects a instruction name".to_string());
      }
      let instr = self.find_instruction(&name);
      if instr.is_none() {
        let mut err_msg = "instruction ".to_string();
        err_msg.push_str(&name);
        err_msg.push_str(" is not defined");
        return self.token_error(err_msg);
      }
      instructions.push(instr.unwrap().0.clone());
      if !self.eat_if_present(&TokKind::Comma) { break; }
    }

    self.parse_token(&TokKind::Rbrace,
      "expects '}' at the end of instruction name list".to_string())
  }

  // ::= '{' size stride? pad? lhs_dilate? rhs_dilate? '}'
  // The subattributes can appear in any order. 'size=' is required, others are
  // optional.
  fn parse_window(&mut self, window: &mut Window, expect_outer_curlies: bool) -> bool {
    //let loc = self.lexer.get_loc();
    if expect_outer_curlies && !self.parse_token(&TokKind::Lbrace,
      "expected '{' to start window attribute".to_string())
    {
      return false;
    }
    let size = vec![];
    let stride = vec![];
    let pad: Vec<Vec<i64>> = vec![];
    let lhs_dilate = vec![];
    let rhs_dilate = vec![];
    let rhs_reversal = vec![];

    let mut end_token = TokKind::Eof;
    if expect_outer_curlies {
      end_token = TokKind::Rbrace;
    }

    while self.lexer.get_kind() != end_token {
      //let attr_loc = self.lexer.get_loc();
      let mut field_name = String::new();
      if !self.parse_attribute_name(&mut field_name) {
        assert!(false, "expects aub-attributes in window");
        return false;
      }
      if field_name == "size".to_string() {
        if !self.parse_dxd(&"size".to_string(), &size) {
          return false;
        }
      }
      if field_name == "stride".to_string() {
        if !self.parse_dxd(&"stride".to_string(), &stride) {
          return false;
        }
      }
      if field_name == "lhs_dilate".to_string() {
        if !self.parse_dxd(&"lhs_dilate".to_string(), &lhs_dilate) {
          return false;
        }
      }
      if field_name == "rhs_dilate".to_string() {
        if !self.parse_dxd(&"rhs_dilate".to_string(), &rhs_dilate) {
          return false;
        }
      }
      if field_name == "pad".to_string() {
        if !self.parse_window_pad(&pad) {
          return false;
        }
      }
      if field_name == "rhs_reversal".to_string() {
        if !self.parse_dxd(&"rhs_reversal".to_string(), &rhs_reversal) {
          return false;
        }
      }
      assert!(false, "unexpected attribute name: {:?}", field_name);
      return false;
    }

    if !stride.is_empty() && stride.len() != size.len() {
      assert!(false, "expects 'stride=' has the same size as 'size='");
      return false;
    }
    if !lhs_dilate.is_empty() && lhs_dilate.len() != size.len() {
      assert!(false, "expects 'lhs_dilate=' has the same size as 'size='");
      return false;
    }
    if !rhs_dilate.is_empty() && rhs_dilate.len() != size.len() {
      assert!(false, "expects 'rhs_dilate=' has the same size as 'size='");
      return false;
    }
    if !pad.is_empty() && pad.len() != size.len() {
      assert!(false, "expects 'pad=' has the same size as 'size='");
      return false;
    }

    for i in 0..size.len() {
      window.add_dimensions().set_size(size[i]);
      if !pad.is_empty() {
        window.mutable_dimensions(i).set_padding_low(pad[i][0]);
        window.mutable_dimensions(i).set_padding_high(pad[i][i]);
      }
      // If some field is not present, it has the default value.
      let mut stride_value = 1;
      if !stride.is_empty() {
        stride_value = stride[i];
      }
      window.mutable_dimensions(i).set_stride(stride_value);
      let mut base_dilation = 1;
      if !lhs_dilate.is_empty() {
        base_dilation = lhs_dilate[i];
      }
      window.mutable_dimensions(i).set_base_dilation(base_dilation);
      let mut window_dilation = 1;
      if !rhs_dilate.is_empty() {
        window_dilation = rhs_dilate[i];
      }
      window.mutable_dimensions(i).set_window_dilation(window_dilation);
      let mut window_reversal = false;
      if !rhs_reversal.is_empty() {
        window_reversal = rhs_reversal[i] == 1;
      }
      window.mutable_dimensions(i).set_window_reversal(window_reversal);
    }

    !expect_outer_curlies ||
    self.parse_token(&TokKind::Rbrace,
      "expected '}' to end window attribute".to_string())
  }

  // This is the inverse of HloInstruction::ConvolutionDimensionNumbersToString.
  // The string looks like "dim_labels=0bf_0io->0bf".
  //
  // '?' dims don't appear in ConvolutionDimensionNumbers.  There can be more than
  // one '?' dim.
  fn parse_convolution_demension_numbers(
    &mut self, dnums: &mut ConvolutionDimensionNumbers) -> bool
  {
    if self.lexer.get_kind() != TokKind::DimLabels {
      assert!(false, "expects dim labels pattern, e.g., 'bf0_0io->0bf'");
      return false;
    }
    let str = self.lexer.get_str_val();

    // The str is expected to have 3 items, lhs, rhs, out, and it must look like
    // lhs_rhs->out, that is, the first separator is "_" and the second is "->".
    let split1: Vec<&str> = str.split('_').collect();
    if split1.len() != 2 {
      assert!(false, "expects 3 items: lhs, rhs, and output dims, but sees {:?}", str);
      return false;
    }
    let split11_val = split1[1].to_string();
    let split2: Vec<&str> = split11_val.split("->").collect();
    if split2.len() != 2 {
      assert!(false, "expexts 3 items: lhs, rhs, and output dims, but sees {:?}", str);
      return false;
    }

    let lhs = split1[0].to_string();
    let rhs = split2[0].to_string();
    let out = split2[1].to_string();

    let mut chars = HashSet::new();
    let mut is_unique = |str: &String| -> bool {
      for c in str.chars() {
        // '?' dims are skipped.
        if c == '?' { continue; }
        if !chars.insert(c) { return false; }
      }
      true
    };

    // **** lhs
    if !is_unique(&lhs) {
      assert!(false, "expects unique lhs dimension numbers, but sees {:?}", lhs);
      return false;
    }
    // Count number of spatial dimensions.
    for c in lhs.chars() {
      if c != 'b' && c != 'f' && c != '?' {
        dnums.add_input_spatial_dimensions(-1);
      }
    }
    for i in 0..lhs.len() {
      let c = lhs.chars().nth(i).unwrap();
      if c == '?' {
        continue;
      } else if c == 'b' {
        dnums.set_input_batch_dimension(i as i64);
      } else if c == 'f' {
        dnums.set_input_feature_dimension(i as i64);
      } else if c >= '0' {
        let c_val = c.to_digit(10).unwrap() as usize;
        let zero_val = '0'.to_digit(10).unwrap() as usize;
        if c_val < zero_val + lhs.len() {
          // TODO
          //dnums.set_input_spatial_dimensions(c - '0', i);
        }
      } else {
        assert!(false, "expects [0-{:?}bf?] in lhs dimension numbers", lhs.len()-1);
        return false;
      }
    }

    // ***** rhs
    if !is_unique(&rhs) {
      assert!(false, "expects unique rhs dimension numbers, but sees {:?}", rhs);
      return false;
    }
    // Count number of spatial dimensions.
    for c in rhs.chars() {
      if c != 'i' && c != 'o' && c != '?' {
        dnums.add_kernel_spatial_dimensions(-1);
      }
    }
    for i in 0..rhs.len() {
      let c = rhs.chars().nth(i).unwrap();
      if c == '?' {
        continue;
      } else if c == 'i' {
        dnums.set_kernel_input_feature_dimension(i as i64);
      } else if c == 'o' {
        dnums.set_kernel_output_feature_dimension(i as i64);
      } else if c >= '0' {
        let c_val = c.to_digit(10).unwrap() as usize;
        let zero_val = '0'.to_digit(10).unwrap() as usize;
        if c_val < zero_val + rhs.len() {
          // TODO
        }
      } else {
        assert!(false, "expects [0-{:?}io?] in rhs dimension numbers", rhs.len()-1);
        return false;
      }
    }

    // ***** output
    if !is_unique(&out) {
      assert!(false, "expects unique output dimension numbers, but sees {:?}", out);
      return false;
    }
    // Count number of spatial dimensions.
    for c in out.chars() {
      if c != 'b' && c != 'f' && c != '?' {
        dnums.add_output_spatial_dimensions(-1);
      }
    }
    for i in 0..out.len() {
      let c = out.chars().nth(i).unwrap();
      if c == '?' {
        continue;
      } else if c == 'b' {
        dnums.set_output_batch_dimension(i as i64);
      } else if c == 'f' {
        dnums.set_output_feature_dimension(i as i64);
      } else if c >= '0' {
        let c_val = c.to_digit(10).unwrap() as usize;
        let zero_val = '0'.to_digit(10).unwrap() as usize;
        if c_val < zero_val + out.len() {
          // TODO
        }
      } else {
        assert!(false, "expects [0-{:?}bf?] in output dimension numbers", out.len()-1);
        return false;
      }
    }

    // lhs, rhs, and output should have the same number of spatial dimensions.
    if dnums.input_spatial_dimensions_size() != dnums.output_spatial_dimensions_size() ||
      dnums.input_spatial_dimensions_size() != dnums.kernel_spatial_dimensions_size()
    {
      assert!(false, "input, kernel, and output must have same number of
        spatial dimensions, but got {:?}, {:?}, {:?} respectively.",
        dnums.input_spatial_dimensions_size(),
        dnums.kernel_spatial_dimensions_size(),
        dnums.output_spatial_dimensions_size());
    }

    self.lexer.lex(0);
    true
  }

  // This is the inverse xla::ToString(PaddingConfig). The padding config string
  // looks like "0_0_0x3_3_1". The string is first separated by 'x', each
  // substring represents one PaddingConfigDimension. The substring is 3 (or 2)
  // numbers joined by '_'.
  fn parse_padding_config(&mut self, padding: &mut PaddingConfig) -> bool {
    if self.lexer.get_kind() != TokKind::Pad {
      assert!(false, "expects padding config, e.g., '0_0_0x3_3_1'");
      return false;
    }
    //let loc = self.lexer.get_loc();
    let str = self.lexer.get_str_val();
    let splitted_x: Vec<&str> = str.split('x').collect();
    for padding_dim_str in splitted_x {
      let splitted_i64_str: Vec<&str> = padding_dim_str.split('_').collect();
      let mut padding_dim = vec![];
      for i64_str in splitted_i64_str {
        let value = i64_str.chars().nth(0).unwrap().to_digit(10).unwrap() as i64;
        padding_dim.push(value);
      }
      let dim = padding.add_dimensions();
      dim.set_edge_padding_low(padding_dim[0]);
      dim.set_edge_padding_high(padding_dim[1]);
      let mut interior_value = 0;
      if padding_dim.len() == 3 {
        interior_value = padding_dim[2];
      }
      dim.set_interior_padding(interior_value);
    }
    self.lexer.lex(0);
    true
  }

  // '{' metadata_string '}'
  fn parse_metadata(&mut self, _metadata: &mut OpMetadata) -> bool {
    let _attrs: HashMap<String, AttrConfig> = HashMap::new();

    let _op_type = String::new();
    let _op_name = String::new();
    let _source_file = String::new();
    let _source_line = 0;
    //let profile_type = vec![];
    let _deduplicated_name = String::new();
    let _preserve_layout = false;

    /*
    attrs.insert("op_type".to_string(), 
      AttrConfig::new(false, AttrType::String, op_type.clone()));
    attrs.insert("op_name".to_string(), 
      AttrConfig::new(false, AttrType::String, op_name.clone()));
    attrs.insert("source_file".to_string(), 
      AttrConfig::new(false, AttrType::String, source_file.clone()));
    attrs.insert("source_line".to_string(), 
      AttrConfig::new(false, AttrType::Int32, source_line.to_string()));
    */
    //attrs.insert("profile_type".to_string(), 
      //AttrConfig::new(false, AttrType::BracedInt64List, profile_type));

    // TODO 

    true
  }

  // ::= single_metadata | ('{' [single_metadata (',' single_metadata)*] '}')
  fn parse_single_or_list_metadata(&mut self, _metadata: &Vec<OpMetadata>) -> bool {
    if self.lexer.get_kind() == TokKind::Lbrace && self.lexer.look_ahead() == TokKind::Lbrace {
      if !self.parse_token(&TokKind::Lbrace,
        "expected '{' to start metadata lsit".to_string())
      {
        return false;
      }

      if self.lexer.get_kind() != TokKind::Rbrace {
        // TODO
      }

      return self.parse_token(&TokKind::Rbrace,
        "expected '}' to end metadata list".to_string());
    }

    true // TODO
  }

  fn parse_op_sharding_type(&mut self, t: &mut OpShardingType) -> bool {
    match self.lexer.get_kind() {
      TokKind::Maximal => {
        *t = OpShardingType::Maximal;
        self.lexer.lex(0);
        return true;
      },
      TokKind::Replicated => {
        *t = OpShardingType::Replicated;
        self.lexer.lex(0);
        return true;
      },
      TokKind::Manual => {
        *t = OpShardingType::Manual;
        self.lexer.lex(0);
        return true;
      },
      _ => return false
    }
  }

  fn parse_list_sharding_type(&mut self, types: &mut Vec<OpShardingType>) -> bool {
    if !self.parse_token(&TokKind::Lbrace,
      "expected '{' to start sharding type list".to_string())
    {
      return false;
    }
    if self.lexer.get_kind() != TokKind::Rbrace {
      loop {
        let mut t = OpShardingType::Unknown;
        if !self.parse_op_sharding_type(&mut t) { return false; }
        types.push(t); // check
        if !self.eat_if_present(&TokKind::Comma) { break; }
      }
    }
    self.parse_token(&TokKind::Rbrace,
      "expected '}' to end sharding type list".to_string())
  }

  // ::= '{' (single_sharding | tuple_sharding) '}'
  // tuple_sharding ::= single_sharding* (',' single_sharding)*
  fn parse_sharding(&mut self, sharding: &mut HloSharding) -> bool {
    // A single sharding starts with '{' and is not followed by '{'.
    // A tuple sharding starts with '{' and is followed by '{', or is '{''}' for
    // an empty tuple.
    if !self.parse_token(&TokKind::Lbrace,
      "expected '{' to start sharding attribute".to_string())
    {
      return false;
    }
    if self.lexer.get_kind() != TokKind::Lbrace &&
      self.lexer.get_kind() != TokKind::Rbrace
    {
      return self.parse_single_sharding(sharding, true);
    }
    // Tuple sharding.
    // Allow empty tuple shardings.
    let mut tuple_shardings = vec![];
    if self.lexer.get_kind() != TokKind::Rbrace {
      loop {
        let mut tuple_sharding = HloSharding::default();
        if !self.parse_single_sharding(&mut tuple_sharding, false) {
          return false;
        }
        tuple_shardings.push(tuple_sharding);
        if !self.eat_if_present(&TokKind::Comma) { break; }
      }
    }
    *sharding = HloSharding::flat_tuple(tuple_shardings);
    self.parse_token(&TokKind::Rbrace,
      "expected '}' to end sharding attribute".to_string())
  }

  // frontend_attributes ::= '{' attributes '}'
  // attributes
  //   ::= /*empty*/
  //   ::= attribute '=' value (',' attribute '=' value)*
  fn parse_frontend_attributes(
    &mut self, frontend_attributes: &mut FrontendAttributes) -> bool
  {
    if !self.parse_token(&TokKind::Lbrace,
      "expected '{' to start frontend attributes".to_string()) {
      return false;
    }
    if self.lexer.get_kind() == TokKind::Rbrace {
      // empty
    } else {
      loop {
        let mut attribute = "".to_string();
        if !self.parse_attribute_name(&mut attribute) {
          return false;
        }
        let mut result = String::new();
        if self.lexer.get_kind() == TokKind::String {
          if !self.parse_string(&mut result) {
            return false;
          }
        } else if self.lexer.get_kind() == TokKind::Lbrace {
          if !self.parse_json_dict(&mut result) {
            return false;
          }
        } else {
          return false;
        }
        frontend_attributes.mutable_map().insert(attribute, result);
        if !self.eat_if_present(&TokKind::Comma) { break; }
      }
    }
    self.parse_token(&TokKind::Rbrace,
      "expects '}' at the end of frontend attributes".to_string())
  }

  // statistics
  //    ::= '{' /*empty*/ '}'
  //    ::= '{' index, single_statistic '}'
  // index ::= 'visualizing_index=' value
  // single_statistic ::= statistic '=' value (',' statistic '=' value)*
  fn parse_statistics_viz(&mut self, statistics_viz: &mut StatisticsViz) -> bool {
    if !self.parse_token(&TokKind::Lbrace,
      "expected '{' to start statistics".to_string())
    {
      return false;
    }
    if self.lexer.get_kind() == TokKind::Rbrace {
      // empty
    } else {
      // index must exist
      let mut visualizing_index_attr_name = String::new();
      if !self.parse_attribute_name(&mut visualizing_index_attr_name) {
        return false;
      }
      if self.lexer.get_kind() != TokKind::Int {
        return false;
      }
      statistics_viz.set_stat_index_to_visualize(self.lexer.get_i64_val());
      self.lexer.lex(0);

      // then process statistics
      while self.eat_if_present(&TokKind::Comma) {
        let mut stat_name = String::new();
        if !self.parse_attribute_name(&mut stat_name) {
          return false;
        }
        if self.lexer.get_kind() != TokKind::Decimal &&
          self.lexer.get_kind() != TokKind::Int
        {
          return false;
        }
        let mut statistic = Statisitic::default();
        statistic.set_stat_name(stat_name);
        if self.lexer.get_kind() == TokKind::Decimal {
          statistic.set_stat_val(self.lexer.get_decimal_val() as i64);
        } else {
          statistic.set_stat_val(self.lexer.get_i64_val());
        }
        self.lexer.lex(0);
        statistics_viz.add_statistics(statistic);
      }
    }
    self.parse_token(&TokKind::Rbrace,
      "expexts '}' at the end of statistics".to_string())
  }

  // devices argument is optional: if not present, the tile assignment is assumed
  // to be an iota tile assignment.
  fn parse_tile_assignment(
    &mut self,
    tile_assignment_dimensions: &mut Vec<i64>,
    iota_reshape_dims: &mut Vec<i64>,
    iota_transpose_perm: &mut Vec<i64>,
    devices: &mut Vec<i64>) -> bool
  {
    if !self.parse_token(&TokKind::Lsquare,
      "expected '[' to shtart sharding devices shape".to_string())
    {
      return false;
    }
    loop {
      let mut dim = -1;
      if !self.parse_i64(&mut dim) {
        return false;
      }
      tile_assignment_dimensions.push(dim);
      if !self.eat_if_present(&TokKind::Comma) {
        break;
      }
    }
    if !self.parse_token(&TokKind::Rsquare,
      "expected ']' to end sharding devices shape".to_string())
    {
      return false;
    }
    if self.lexer.get_kind() == TokKind::Leq {
      self.lexer.lex(0);
      if !self.parse_token(&TokKind::Lsquare,
        "expected '[' to start sharding iota_reshape_dims".to_string())
      {
        return false;
      }
      loop {
        let mut dim = 0;
        if !self.parse_i64(&mut dim) {
          return false;
        }
        iota_reshape_dims.push(dim);
        if !self.eat_if_present(&TokKind::Comma) {
          break;
        }
      }
      if iota_reshape_dims.is_empty() {
        assert!(false, "expected non-empty iota_reshape_dims");
        return false;
      }
      if !self.parse_token(&TokKind::Rsquare,
        "expected ']' to end sharding iota_reshape_dims".to_string())
      {
        return false;
      }
      if iota_reshape_dims.len() == 1 {
        iota_transpose_perm.push(0);
      } else {
        if self.lexer.get_kind() != TokKind::Ident ||
          self.lexer.get_str_val() != "T".to_string()
        {
          assert!(false, "expected 'T(' to start sharding devices iota_transpose_perm");
          return false;
        }
        self.lexer.lex(0);
        if !self.parse_token(&TokKind::Lparen,
          "expected 'T(' to start sharding devices iota_transpose_perm".to_string())
        {
          return false;
        }
        loop {
          let mut dim = -1;
          if !self.parse_i64(&mut dim) {
            return false;
          }
          if dim >= iota_reshape_dims.len() as i64 {
            assert!(false, "out of range iota minor_to_major value {:?}", dim);
            return false;
          }
          iota_transpose_perm.push(dim);
          if !self.eat_if_present(&TokKind::Comma) {
            break;
          }
        }
        if !self.parse_token(&TokKind::Rparen,
          "expected ')' to end sharding devices iota_transpose_perm".to_string())
        {
          return false;
        }
      }
    } else {
      loop {
        let mut device = 0;
        if !self.parse_i64(&mut device) {
          return false;
        }
        devices.push(device);
        if !self.eat_if_present(&TokKind::Comma) {
          break;
        }
      }
    }
    true
  }

  // ::= '{' 'replicated'? 'manual'? 'maximal'? 'unknown'? ('device=' int)? shape?
  //         ('devices=' ('[' dims ']')* device_list)?
  //         (('shard_like' | 'shard_as') int)* '}'
  //         ('metadata=' metadata)*
  //
  // dims ::= int_list
  // device_list ::= int_list? ('<=[' int_list ']{' int_list '}')?
  // metadata ::= single_metadata |
  //              ('{' [single_metadata (',' single_metadata)*] '}')
  // last_tile_dims ::= sharding_type_list
  fn parse_single_sharding(
    &mut self, sharding: &mut HloSharding, lbrace_pre_lexed: bool) -> bool
  {
    if !lbrace_pre_lexed && !self.parse_token(&TokKind::Lbrace,
      "expected '{' to start sharding attribute".to_string())
    {
      return false;
    }

    let mut maximal = false;
    let mut replicated = false;
    let mut manual = false;
    let mut unknown = false;
    let mut unreduced = false;
    let mut last_tile_dim_replicate = false;
    let mut _last_tile_dims = false;
    let mut shard_like = false;
    let mut shard_as = false;
    let mut shard_group_id = -1;
    let mut devices = vec![];
    let mut tile_assignment_dimensions = vec![];
    let mut iota_reshape_dims = vec![];
    let mut iota_transpose_perm = vec![];
    let mut subgroup_types = vec![];
    let metadata = vec![];

    while self.lexer.get_kind() != TokKind::Rbrace {
      match self.lexer.get_kind() {
        TokKind::Maximal => {
          maximal = true;
          self.lexer.lex(0);
        }
        TokKind::Replicated => {
          replicated = true;
          self.lexer.lex(0);
        }
        TokKind::Manual => {
          manual = true;
          self.lexer.lex(0);
        }
        TokKind::Unknown => {
          unknown = true;
          self.lexer.lex(0);
        }
        TokKind::Unreduced => {
          unreduced = true;
          self.lexer.lex(0);
        }
        TokKind::AttributeName => {
          if self.lexer.get_str_val() == "device".to_string() {
            if self.lexer.lex(0) != TokKind::Int {
              return self.token_error("device= attribute must be an integer".to_string());
            }
            devices.push(self.lexer.get_i64_val());
            self.lexer.lex(0);
          } else if self.lexer.get_str_val() == "devices".to_string() {
            self.lexer.lex(0);
            if !self.parse_tile_assignment(
              &mut tile_assignment_dimensions,
              &mut iota_reshape_dims,
              &mut iota_transpose_perm,
              &mut devices)
            {
              return false;
            }
          } else if self.lexer.get_str_val() == "metadata".to_string() {
            self.lexer.lex(0);
            if !self.parse_single_or_list_metadata(&metadata) {
              return false;
            }
          } else if self.lexer.get_str_val() == "last_tile_dims".to_string() {
            _last_tile_dims = true;
            self.lexer.lex(0);
            if !self.parse_list_sharding_type(&mut subgroup_types) {
              return false;
            }
          } else {
            return self.token_error("unknown attribute in sharding:
              expected device=, devices=, metadata= or last_tile_dims=".to_string());
          }
        }
        TokKind::LastTileDimReplicate => {
          last_tile_dim_replicate = true;
          self.lexer.lex(0);
        }
        TokKind::ShardAs => {
          shard_as = true;
          self.lexer.lex(0);
          if !self.parse_i64(&mut shard_group_id) {
            return false;
          }
        }
        TokKind::ShardLike => {
          shard_like = true;
          self.lexer.lex(0);
          if !self.parse_i64(&mut shard_group_id) {
            return false;
          }
        }
        TokKind::Rbrace => break,
        _ => {
          let mut err_msg = "unexpected token: ".to_string();
          err_msg.push_str(&self.lexer.get_kind().to_string());
          return self.token_error(err_msg)
        }
      }
    }

    if replicated {
      if !devices.is_empty() {
        assert!(false, "replicated shardings should not have any devices assigned");
        return false;
      }
      *sharding = HloSharding::replicate(metadata);
    } else if maximal {
      if devices.len() != 1 {
        assert!(false, "maximal shardings should have exactly one device assigned");
        return false;
      }
      *sharding = HloSharding::assign_device(
        devices[0], metadata, false);
    } else if manual {
      if !devices.is_empty() {
        assert!(false, "manual shardings should not have any devices assigned");
        return false;
      }
      *sharding = HloSharding::manual(metadata);
    } else if unknown {
      if !devices.is_empty() {
        assert!(false, "unknown shardings should not have any devices assigned");
        return false;
      }
      *sharding = HloSharding::unknown(metadata);
    } else if unreduced {
      if !devices.is_empty() {
        assert!(false, "unreduced shardings should not have any devices assigned");
        return false;
      }
      *sharding = HloSharding::unreduced(metadata);
    } else {
      if tile_assignment_dimensions.is_empty() {
        assert!(false, "non-maximal sharding must have a tile assignment list
          including dimensions");
        return false;
      }
      if iota_transpose_perm.len() != iota_reshape_dims.len() {
        assert!(false, "iota_transpose_perm should have the same rank as
          iota_reshape_dims");
        return false;
      }
      if last_tile_dim_replicate {
        debug_assert!(subgroup_types.is_empty());
        subgroup_types.push(OpShardingType::Replicated);
      }
      if !iota_reshape_dims.is_empty() {
        debug_assert!(devices.is_empty());
        if subgroup_types.is_empty() {
          *sharding = HloSharding::iota_tile(
            &tile_assignment_dimensions, &iota_reshape_dims,
            &iota_transpose_perm, &metadata);
        } else {
          *sharding = HloSharding::subgroup(
            TileAssignment::new_from_vecs(&tile_assignment_dimensions,
              &iota_reshape_dims, &iota_transpose_perm),
            subgroup_types, metadata);
        }
      } else {
        if devices.len() <= 1 {
          assert!(false, "non-maximal shardings must have more than one desice assigned");
          return false;
        }
        let mut tiles: Vec<i64> = vec![];
        tiles.clone_from(&tile_assignment_dimensions);
        let mut array = Array::new(&tiles);
        array.set_values(&devices);
        if subgroup_types.is_empty() {
          *sharding = HloSharding::tile(
            TileAssignment::new_from_array(array), &metadata);
        } else {
          *sharding = HloSharding::subgroup(
            TileAssignment::new_from_array(array), subgroup_types, metadata);
        }
      }
    }

    if shard_as || shard_like {
      let mut shard_g = HloSharding::shard_like(shard_group_id);
      if shard_as {
        shard_g = HloSharding::shard_as(shard_group_id);
      }
      sharding.set_shard_group(shard_g);
    }

    self.lexer.lex(0);
    true
  }

  // parameter_replication ::=
  //   '{' ('true' | 'false')* (',' ('true' | 'false'))*  '}'
  fn parse_parameter_replication(
    &mut self, parameter_replication: &mut ParameterReplication) -> bool
  {
    if !self.parse_token(&TokKind::Lbrace,
      "expected '{' to start parameter_replication attribute".to_string()) {
      return false;
    }
    if self.lexer.get_kind() != TokKind::Rbrace {
      loop {
        if self.lexer.get_kind() == TokKind::True {
          parameter_replication.add_replicated_at_leaf_buffers(true);
        } else if self.lexer.get_kind() == TokKind::False {
          parameter_replication.add_replicated_at_leaf_buffers(false);
        } else {
          return false;
        }
        self.lexer.lex(0);
        if !self.eat_if_present(&TokKind::Comma) { break; }
      }
    }
    self.parse_token(&TokKind::Rbrace,
      "expects '}' at the end parameter_replication attribute".to_string())
  }

  // boolean_list ::=
  //   ('true' | 'false') | ('{' ('true' | 'false')* (',' ('true' | 'false'))*
  //   '}')
  fn parse_boolean_list_or_single_boolean(
    &mut self, booleans: &mut Vec<bool>) -> bool
  {
    if self.lexer.get_kind() != TokKind::Lbrace &&
       self.lexer.get_kind() != TokKind::True &&
       self.lexer.get_kind() != TokKind::False
    {
      return self.token_error(
        "Expected list of booleans or true/false value".to_string());
    }
    if self.parse_boolean(booleans) {
      return true;
    }
    if !self.parse_token(&TokKind::Lbrace,
      "expected '{' to start boolean list attribute".to_string())
    {
      return  false;
    }
    if self.lexer.get_kind() != TokKind::Rbrace {
      loop {
        if !self.parse_boolean(booleans) { return false; }
        if !self.eat_if_present(&TokKind::Comma) { break; }
      }
    }

    self.parse_token(&TokKind::Rbrace,
      "expected '}' to end boolean list attribute".to_string())
  }

  fn parse_boolean(&mut self, boolean_list: &mut Vec<bool>) -> bool {
    if self.lexer.get_kind() == TokKind::True {
      boolean_list.push(true);
      self.lexer.lex(0);
      return true;
    } else if self.lexer.get_kind() == TokKind::False {
      boolean_list.push(false);
      self.lexer.lex(0);
      return true;
    }
    false
  }

  //fn parse_replica_groups_only() {}

  // domain ::= '{' 'kind=' domain_kind ',' 'entry=' entry_sharding ','
  //            'exit=' exit_sharding '}'
  fn parse_domain(&mut self, _domain: &DomainData) -> bool {
    unimplemented!()
  }

  fn parse_dxd(&mut self, _name: &String, _result: &Vec<i64>) -> bool {
    unimplemented!()
  }

  fn parse_window_pad(&mut self, _pad: &Vec<Vec<i64>>) -> bool {
    unimplemented!()
  }

  // ::= '{' ranges '}'
  //   ::= /*empty*/
  //   ::= range (',' range)*
  // range ::= '[' start ':' limit (':' stride)? ']'
  //
  // The slice ranges are printed as:
  //
  //  {[dim0_start:dim0_limit:dim0stride], [dim1_start:dim1_limit], ...}
  //
  // This function extracts the starts, limits, and strides as 3 vectors to the
  // result. If stride is not present, stride is 1. For example, if the slice
  // ranges is printed as:
  //
  //  {[2:3:4], [5:6:7], [8:9]}
  //
  // The parsed result will be:
  //
  //  {/*starts=*/{2, 5, 8}, /*limits=*/{3, 6, 9}, /*strides=*/{4, 7, 1}}
  fn parse_slice_ranges(&mut self, result: &mut SliceRange) -> bool {
    if !self.parse_token(
      &TokKind::Lbrace, "expects '{' to start ranges".to_string())
    {
      return false;
    }
    let mut ranges: Vec<Vec<i64>> = vec![];
    if self.lexer.get_kind() == TokKind::Rbrace {
      // empty
      return self.parse_token(
      &TokKind::Rbrace, "expects '}' to end ranges".to_string());
    }
    loop {
      ranges.push(vec![]);
      let _loc = self.lexer.get_loc();
      if !self.parse_i64_list(&TokKind::Lsquare, &TokKind::Rsquare,
        &TokKind::Colon, ranges.last_mut().unwrap())
      {
        return false;
      }
      let range = ranges.last().unwrap();
      if range.len() != 2 && range.len() != 3 {
        let mut err_msg =
          "expects [start:limit:step] or [start:limit] but sees ".to_string();
        err_msg.push_str(&range.len().to_string());
        err_msg.push_str("elements");
        assert!(false, "{:?}", err_msg);
      }
      if !self.eat_if_present(&TokKind::Comma) {
        break;
      }
    }
    for range in &ranges {
      result.starts.push(range[0]);
      result.limits.push(range[1]);
      let mut value = 1;
      if range.len() == 3 { value = range[2]; }
      result.strides.push(value);
    }
    self.parse_token(
      &TokKind::Rbrace, "expects '}' to end ranges".to_string())
  }

  // precisionlist ::= start precision_elements end
  // precision_elements
  //   ::= /*empty*/
  //   ::= precision_val (delim precision_val)*
  fn parse_precision_list(&mut self, result: &mut Vec<Precision>) -> bool {
    let mut parse_and_add_item
      = |parser: &mut HloParser| -> bool
    {
      let mut item = Precision::Default;
      if !parser.parse_precision(&mut item) {
        return false;
      }
      result.push(item);
      true
    };
    self.parse_list(
      &TokKind::Lbrace,
      &TokKind::Rbrace,
      &TokKind::Comma,
      Box::new(&mut parse_and_add_item))
  }

  fn parse_hlo_computation(&mut self, result: &mut HloComputation) -> bool {
    self.current_recursion_depth += 1; // TODO
    if self.current_recursion_depth >= self.options.max_recursion_depth() {
      assert!(false, "maximum recursion depth exceeded");
    }
    if self.lexer.get_kind() == TokKind::Lbrace {
      // This means it is a nested computation.
      return self.parse_instruction_list(
        result, &mut "_".to_string());
    }
    // This means it is a computation name.
    self.parse_computation_name(result)
  }

  fn parse_hlo_computation_list(&mut self, result: &mut Vec<HloComputation>) -> bool {
    let mut parse_and_add_item
      = |parser: &mut HloParser| -> bool
    {
      let mut computation = HloComputation::default();
      if !parser.parse_hlo_computation(&mut computation) {
        return false;
      }
      println!("parsed computation {:?}", computation.name());
      result.push(computation);
      true
    };
    self.parse_list(
      &TokKind::Lbrace,
      &TokKind::Rbrace,
      &TokKind::Comma,
      Box::new(&mut parse_and_add_item))
  }

  // shapelist ::= '{' shapes '}'
  // precision_elements
  //   ::= /*empty*/
  //   ::= shape (',' shape)*
  fn parse_shape_list(&mut self, result: &mut Vec<Shape>) -> bool {
    let mut parse_and_add_item
      = |parser: &mut HloParser| -> bool
    {
      let mut shape = Shape::default();
      if !parser.parse_shape(&mut shape, true) {
        return false;
      }
      result.push(shape);
      true
    };
    self.parse_list(
      &TokKind::Lbrace,
      &TokKind::Rbrace,
      &TokKind::Comma,
      Box::new(&mut parse_and_add_item))
  }

  // int64_tlist ::= start int64_elements end
  // int64_elements
  //   ::= /*empty*/
  //   ::= int64_val (delim int64_val)*
  fn parse_i64_list(
    &mut self,
    start: &TokKind,
    end: &TokKind,
    delim: &TokKind,
    result: &mut Vec<i64>) -> bool
  {
    // Next token can be `end` or an int64. So, we pass skip_mask hint to the
    // lexer to avoid parsing unnecessary expensive patterns.
    let mut msg = "expects a list to end with".to_string();
    msg.push_str(&tok_kind_to_string(end));
    if self.parse_token(start, msg.clone()) {
      return false;
    }
    if self.lexer.get_kind() == *end {
      // empty
    } else {
      loop {
        let mut i = 0;
        // The next token must be a `delim` or the `end` token, both of which are
        // small and lexing it fast. So, we don't pass any skip hint to the lexer.
        if !self.parse_i64(&mut i) {
          return false;
        }
        result.push(i);
        if self.lexer.get_kind() != *delim {
          break;
        }
        // The next token must be an int64. Otherwise, it would be a parsing
        // error. So, we pass skip_mask hint to the lexer to avoid parsing
        // unnecessary expensive patterns.
        self.lexer.lex(DIM_LABELS_DXD_PAD_DECIMAL_MASK);
      }
    }
    // After the `end` token, we don't have any idea on the next token. So, we
    // don't pass any skip_mask hint to the lexer.
    self.parse_token(end, msg)
  }

  // int64_tlistlist ::= start int64_tlist_elements end
  // int64_tlist_elements
  //   ::= /*empty*/
  //   ::= int64_tlist (delim int64_tlist)*
  // int64_tlist ::= start int64_elements end
  // int64_elements
  //   ::= /*empty*/
  //   ::= int64_val (delim int64_val)*
  fn parse_i64_list_list(
    &mut self,
    start: &TokKind,
    end: &TokKind,
    delim: &TokKind,
    result: &mut Vec<Vec<i64>>) -> bool
  {
    let mut parse_and_add_item
      = |parser: &mut HloParser| -> bool
    {
      let mut item = vec![];
      if !parser.parse_i64_list(start, end, delim, &mut item) {
        return false;
      }
      result.push(item);
      true
    };
    self.parse_list(
      &TokKind::Lbrace,
      &TokKind::Rbrace,
      &TokKind::Comma,
      Box::new(&mut parse_and_add_item))
  }

  // 'parse_and_add_item' is an lambda to parse an element in the list and add
  // the parsed element to the result. It's supposed to capture the result.
  fn parse_list(
    &mut self,
    start: &TokKind,
    end: &TokKind,
    delim: &TokKind,
    mut parse_and_add_item: Box<&mut dyn FnMut(&mut HloParser)->bool>) -> bool
  {
    let mut err_msg = "expects a list starting with ".to_string();
    err_msg.push_str(&tok_kind_to_string(start));
    if !self.parse_token(start, err_msg) {
      return false;
    }

    if self.lexer.get_kind() == *end {
      // empty
    } else {
      loop {
        if !parse_and_add_item(self) { return false; }
        if !self.eat_if_present(delim) { break; }
      }
    }

    let mut err_msg = "expects a list to end with ".to_string();
    err_msg.push_str(&tok_kind_to_string(end));
    self.parse_token(end, err_msg)
  }

  // param_list_to_shape ::= param_list '->' shape
  fn parse_param_list_to_shape(
    &mut self, shape: &mut Shape, shape_loc: &mut usize) -> bool
  {
    if !self.parse_param_list() ||
       ! self.parse_token(&TokKind::Arrow, "expects '->'".to_string())
    {
      return false;
    }
    *shape_loc = self.lexer.get_loc();
    self.parse_shape(shape, true)
  }

  // param_list ::= '(' param_list1 ')'
  // param_list1
  //   ::= /*empty*/
  //   ::= param (',' param)*
  // param ::= name shape
  fn parse_param_list(&mut self) -> bool {
    if !self.parse_token(&TokKind::Lparen,
      "expects '(' at the beginning of param list".to_string())
    {
      return false;
    }
    if self.lexer.get_kind() == TokKind::Rparen {
      // empty
    } else {
      loop {
        let mut shape = Shape::default();
        let mut name = String::new();
        if !self.parse_name(&mut name) ||
          !self.parse_shape(&mut shape, true)
        {
          return false;
        }
        if !self.eat_if_present(&TokKind::Comma) { break; }
      }
    }
    self.parse_token(&TokKind::Rparen,
      "expects ')' at the end of param list".to_string())
  }

  fn parse_name(&mut self, result: &mut String) -> bool {
    println!("parse_name");
    if self.lexer.get_kind() != TokKind::Ident &&
      self.lexer.get_kind() != TokKind::Name
    {
      return self.token_error("expects name".to_string());
    }
    *result = self.lexer.get_str_val();
    self.lexer.lex(0);
    true
  }

  fn parse_attribute_name(&mut self, result: &mut String) -> bool {
    if self.lexer.get_kind() != TokKind::AttributeName {
      return self.token_error("expects attribute name".to_string());
    }
    *result = self.lexer.get_str_val();
    self.lexer.lex(0);
    true
  }

  fn parse_string(&mut self, result: &mut String) -> bool {
    println!("parse_string");
    if self.lexer.get_kind() != TokKind::String {
      return self.token_error("expects string".to_string());
    }
    *result = self.lexer.get_str_val();
    self.lexer.lex(0);
    true
  }

  fn parse_json_dict(&mut self, result: &mut String) -> bool {
    println!("parse_json_dict");
    if self.lexer.lex_json_dict() != TokKind::String {
      return self.token_error("expects JSON dict".to_string());
    }
    *result = self.lexer.get_str_val();
    self.lexer.lex(0);
    true
  }

  // dimension_sizes ::= '[' dimension_list ']'
  // dimension_list
  //   ::= /*empty*/
  //   ::= '?'
  //   ::= <=? int64_t (',' param)*
  // param ::= name shape
  fn parse_dimension_sizes(
    &mut self,
    dimension_sizes: &mut Vec<i64>,
    dynamic_dimensions: &mut Vec<bool>) -> bool
  {
    let mut parse_and_add_item =
      |parser: &mut HloParser| -> bool
    {
      let mut i = 0;
      let mut is_dynamic = false;
      if parser.lexer.get_kind() == TokKind::QuestionMark {
        i = Shape::UNBOUNDED_SIZE;
        is_dynamic = true;
        parser.lexer.lex(0);
      } else {
        if parser.lexer.get_kind() == TokKind::Leq {
          is_dynamic = true;
          parser.lexer.lex(0);
        }
        if !parser.parse_i64(&mut i) {
          return false;
        }
      }
      dimension_sizes.push(i);
      dynamic_dimensions.push(is_dynamic);
      true
    };
    self.parse_list(
      &TokKind::Lsquare, &TokKind::Rsquare,
      &TokKind::Comma,
      Box::new(&mut parse_and_add_item))
  }

  // shape ::= shape_val_
  // shape ::= '(' tuple_elements ')'
  // shape ::= 'b(' shape ')'
  // tuple_elements
  //   ::= /*empty*/
  //   ::= shape (',' shape)*
  fn parse_shape(
    &mut self,
    result: &mut Shape,
    allow_fallback_to_default_layout: bool) -> bool
  {
    if self.lexer.get_kind() == TokKind::Ident &&
      self.lexer.get_str_val() == "b".to_string() &&
      self.lexer.look_ahead() == TokKind::Lparen // Buffer shape
    {

    }
    if self.eat_if_present(&TokKind::Lparen) { // Tuple

    }
    let mut primitive_t = PrimitiveType::S64;
    if !self.parse_primitive_type(&mut primitive_t) {
      return false;
    }
    // Each element contains a dimension size and a bool indicating whether this
    // is a dynamic dimension.
    let mut dimension_sizes = vec![];
    let mut dynamic_dimensions = vec![];
    if !self.parse_dimension_sizes(&mut dimension_sizes, &mut dynamic_dimensions) {
      return false;
    }
    result.set_element_type(primitive_t);
    for i in 0..dimension_sizes.len() {
      if !Shape::is_valid_dimension_size(
        dimension_sizes[i], dynamic_dimensions[i]) {
        return false;
      }
      result.add_dimensions(
        dimension_sizes[i], dynamic_dimensions[i]);
    }
    if (allow_fallback_to_default_layout && self.options.fill_missing_layouts()) ||
      ShapeUtil::is_scalar(result)
    {
      LayoutUtil::set_to_default_layout(result);
    }
    // We need to lookahead to see if a following open brace is the start of a
    // layout. The specific problematic case is:
    //
    // ENTRY %foo (x: f32[42]) -> f32[123] {
    //  ...
    // }
    //
    // The open brace could either be the start of a computation or the start of a
    // layout for the f32[123] shape. We consider it the start of a layout if the
    // next token after the open brace is an integer or a colon.
    if self.lexer.get_kind() == TokKind::Lbrace &&
      (self.lexer.look_ahead() == TokKind::Int ||
      self.lexer.look_ahead() == TokKind::Colon)
    {
      let mut layout = Layout::default();
      if !self.parse_layout(&mut layout) {
        return false;
      }
      if layout.minor_to_major_size() != result.dimensions_size() {
        assert!(false, "dimensions size is {:?}, but minor to major size is {:?}",
          result.dimensions_size(), layout.minor_to_major_size());
        return false;
      }
      if layout.has_physical_shape() {
        assert!(false, "layout has physical shape, but is not for a sparse array: {:?}",
          layout.to_string());
        return false;
      }
      *result.mutable_layout() = Some(layout);
    }
    true
  }

// layout
//   ::= '{' int64_list
//       (':' dim_level_types
//            tiles
//            tail_padding_alignment_in_elements
//            element_size_in_bits
//            memory_space
//            split_configs
//            physical_shape
//            dynamic_shape_metadata_prefix_bytes)?
//       '}'
// element_size_in_bits
//   ::= /*empty*/
//   ::= 'E' '(' int64_t ')'
// memory_space
//   ::= /*empty*/
//   ::= 'S' '(' int64_t ')'
  fn parse_layout(&mut self, layout: &mut Layout) -> bool {
    let mut minor_to_major = vec![];
    let dim_level_types = vec![];
    let tiles = vec![];
    let index_primitive_t = PrimitiveType::Invalid;
    let pointer_primitive_t = PrimitiveType::Invalid;
    let mut element_size_in_bits = 0;
    let mut memory_space = 0;
    let split_configs = vec![];
    let mut physical_shape = Some(Shape::default());
    let mut dynamic_shape_metadata_prefix_bytes = 0;
    let mut tail_padding_alignment_in_elements = 1;

    let mut parse_and_add_item =
      |parser: &mut HloParser| -> bool
    {
      let mut i = 0;
      if !parser.parse_i64(&mut i) {
        return false;
      }
      minor_to_major.push(i);
      true
    };

    let mut msg = "expects layout to start with ".to_string();
    msg.push_str(&tok_kind_to_string(&TokKind::Lbrace));
    if !self.parse_token(&TokKind::Lbrace, msg) {
      return false; 
    }
    if self.lexer.get_kind() != TokKind::Rbrace {
      if self.lexer.get_kind() == TokKind::Int {
        // Parse minor to major.
        loop {
          if !parse_and_add_item(self) {
            return false;
          }
          if self.eat_if_present(&TokKind::Comma) {
            break;
          }
        }
      }
      if self.lexer.get_kind() == TokKind::Colon {
        self.lexer.lex(0);
        if self.lexer.get_kind() == TokKind::Ident &&
          self.lexer.get_str_val() == "D".to_string()
        {
          self.lexer.lex(0);
          self.parse_dim_level_types(&dim_level_types);
        }
        if self.lexer.get_kind() == TokKind::Ident &&
          self.lexer.get_str_val() == "T".to_string()
        {
          self.lexer.lex(0);
          self.parse_tiles(&tiles);
        }
        if self.lexer.get_kind() == TokKind::Ident &&
          self.lexer.get_str_val() == "L".to_string()
        {
          self.lexer.lex(0);
          self.parse_layout_int_attribute(
            &mut tail_padding_alignment_in_elements,
            "multiple padded to in elements".to_string());
        }
        if self.lexer.get_kind() == TokKind::Octothorp {
          self.lexer.lex(0);

        }
        if self.lexer.get_kind() == TokKind::Asterisk {

        }
        if self.lexer.get_kind() == TokKind::Ident &&
          self.lexer.get_str_val() == "E".to_string()
        {
          self.lexer.lex(0);
          self.parse_layout_int_attribute(&mut element_size_in_bits,
            "element size in bitz".to_string());
        }
        if self.lexer.get_kind() == TokKind::Ident &&
          self.lexer.get_str_val() == "S".to_string()
        {
          self.lexer.lex(0);
          self.parse_layout_int_attribute(&mut memory_space,
            "memory space".to_string());
        }
        if self.lexer.get_kind() == TokKind::Ident &&
          self.lexer.get_str_val() == "SC".to_string()
        {
          self.lexer.lex(0);
          self.parse_split_configs(&split_configs);
          
        }
        if self.lexer.get_kind() == TokKind::Ident &&
          self.lexer.get_str_val() == "P".to_string()
        {
          self.lexer.lex(0);
          self.parse_physical_shape(physical_shape.as_mut().unwrap());
        }
        if self.lexer.get_kind() == TokKind::Ident &&
          self.lexer.get_str_val() == "M".to_string()
        {
          self.lexer.lex(0);
          self.parse_layout_int_attribute(
            &mut dynamic_shape_metadata_prefix_bytes,
            "dynamic shape ,etadata prefix bytes".to_string());
        }
      }
    }
    let mut msg = "expect layout to end with ".to_string();
    msg.push_str(&tok_kind_to_string(&TokKind::Rbrace));
    if !self.parse_token(&TokKind::Rbrace, msg) {
      return false;
    }
    let mut vec_tiles = vec![];
    for i in 0..tiles.len() {
      vec_tiles.push(tiles[i].clone());
    }
    *layout = LayoutUtil::make_layout(
      &minor_to_major,
      vec_tiles,
      tail_padding_alignment_in_elements,
      index_primitive_t,
      pointer_primitive_t,
      element_size_in_bits,
      memory_space,
      split_configs,
      physical_shape,
      dynamic_shape_metadata_prefix_bytes);

    true
  }

  // int_attribute
  //   ::= /*empty*/
  //   ::= attr_token '(' attr_value ')'
  // attr_token
  //   ::= 'E' | 'S'
  // attr_value
  //   ::= int64_t
  fn parse_layout_int_attribute(
    &mut self, attr_value: &mut i64, attr_desc: String) -> bool
  {
    let mut err_msg = "expects ".to_string();
    err_msg.push_str(&attr_desc);
    err_msg.push_str(" to start with ");
    err_msg.push_str(&tok_kind_to_string(&TokKind::Lparen));

    if !self.parse_token(&TokKind::Lparen, err_msg) {
      return false;
    }
    if !self.parse_i64(attr_value) {
      return false;
    }

    let mut err_msg = "expects ".to_string();
    err_msg.push_str(&attr_desc);
    err_msg.push_str(" to end with ");
    err_msg.push_str(&tok_kind_to_string(&TokKind::Rparen));

    if !self.parse_token(&TokKind::Rparen, err_msg) {
      return false;
    }
    true
  }

  // dim_level_types
  //   ::=  /* empty */
  //   ::= 'D' '(' dim_level_type_list ')'
  // dim_level_type_list
  //   ::= /* empty */
  //   ..= dim_level_type (',' dim_level_type)*
  // dim_level_type
  //   ::= 'D'
  //   ::= 'C'
  //   ::= 'S'
  fn parse_dim_level_types(
    &mut self, _dim_level_types: &Vec<DimLevelType>) -> bool 
  {
    unimplemented!()
  }

  // tiles
  //   ::= /*empty*/
  //   ::= 'T' ('(' dim_list ')')+
  // dim_list
  //   ::= /*empty*/
  //   ::= (int64_t | '*') (',' (int64_t | '*'))*
  fn parse_tiles(&mut self, _tiles: &Vec<Tile>) -> bool {
    unimplemented!()
  }

  // split_configs
  //   ::= /*empty*/
  //   ::= 'SC' ('(' int64_t ':' int64_list ')')+
  fn parse_split_configs(&mut self, _split_configs: &Vec<SplitConfig>) -> bool {
    unimplemented!()
  }

  // physical_shape
  //   ::= /*empty*/
  //   ::= 'P' '(' shape ')'
  fn parse_physical_shape(&mut self, physical_shape: &mut Shape) -> bool {
    let mut err_msg = "expects physical shape to start with ".to_string();
    err_msg.push_str(&tok_kind_to_string(&TokKind::Lparen));
    if !self.parse_token(&TokKind::Lparen, err_msg) {
      return false;
    }
    self.parse_shape(physical_shape, true);
    let mut err_msg = "expects physical shape to end with ".to_string();
    err_msg.push_str(&tok_kind_to_string(&TokKind::Rparen));
    if !self.parse_token(&TokKind::Rparen, err_msg) {
      return false;
    }
    true
  }

  fn parse_opcode(
    &mut self,
    opcode: &mut HloOpcode,
    async_wrapped_opcode: Option<&mut HloOpcode>) -> bool
  {
    if self.lexer.get_kind() != TokKind::Ident {
      return self.token_error("expects opcode".to_string());
    }
    let val = self.lexer.get_str_val();
    let status_or_result =
      string_to_hlo_opcode(&val);
    if status_or_result.is_err() {
      let mut try_parsing_async_op =
        |suffix: &str, async_opcode: HloOpcode| -> bool
      {
        if val.ends_with(suffix) {
          *opcode = async_opcode.clone();
          // TODO
          //status_or_result = string_to_hlo_opcode(name)
          return true;
        }
        false
      };
      if try_parsing_async_op("-start", HloOpcode::AsyncStart) ||
        try_parsing_async_op("-updaate", HloOpcode::AsyncUpdate) ||
        try_parsing_async_op("-done", HloOpcode::AsyncDone)
      {
        if status_or_result.is_err() {
          assert!(false, "expects async wrapped opcode but sees: {:?}", val);
          return false;
        }
        *async_wrapped_opcode.unwrap() = status_or_result.unwrap();
      }
    } else {
      *opcode = status_or_result.unwrap();
    }
    self.lexer.lex(0);
    true
  }

  fn parse_fft_type(&mut self, _result: &mut FftType) -> bool {
    println!("parse_fft_type");
    if self.lexer.get_kind() != TokKind::Ident {
      return self.token_error("expects fft type".to_string());
    }
    let _val = self.lexer.get_str_val();
    // TODO: fft_type_parse
    self.lexer.lex(0);
    true
  }

  fn parse_padding_type(&mut self, _result: &mut PaddingType) -> bool {
    println!("parse_padding_type");
    if self.lexer.get_kind() != TokKind::Ident {
       return self.token_error("expects padding type".to_string());
    }
    let _val = self.lexer.get_str_val();
    // TODO
    self.lexer.lex(0);
    true
  }

  fn parse_primitive_type(&mut self, result: &mut PrimitiveType) -> bool {
    if self.lexer.get_kind() != TokKind::PrimitiveType {
      return self.token_error("expected primitive type".to_string());
    }
    *result = self.lexer.get_primitive_type_val();
    self.lexer.lex(0);
    true
  }

  fn parse_comparison_direction(&mut self, result: &mut ComparisonDirection) -> bool {
    println!("parse_comparison_direction");
    if self.lexer.get_kind() != TokKind::Ident {
      return self.token_error("expects comparison direction".to_string());
    }
    let val = self.lexer.get_str_val();
    let comparison_direction =
      string_to_comparison_direction(&val);
    if comparison_direction.is_err() {
      let mut err_msg = "expects comparison direction but sees: ".to_string();
      err_msg.push_str(&val);
      return self.token_error(err_msg);
    }
    *result = comparison_direction.unwrap();
    self.lexer.lex(0);
    true
  }

  fn parse_comparison_type(&mut self, result: &mut ComparisonType) -> bool {
    println!("parse_comparison_type");
    if self.lexer.get_kind() != TokKind::Ident {
      return self.token_error("expects comparison type".to_string());
    }
    let val = self.lexer.get_str_val();
    let comparison_type =
      string_to_comparison_type(&val);
    if comparison_type.is_err() {
      let mut err_msg = "expects comparison type but sees: ".to_string();
      err_msg.push_str(&val);
      return self.token_error(err_msg);
    }
    *result = comparison_type.unwrap();
    self.lexer.lex(0);
    true
  }

  fn parse_fusion_kind(&mut self, result: &mut FusionKind) -> bool {
    println!("parse_fusion_kind");
    if self.lexer.get_kind() != TokKind::Ident {
      return self.token_error("expexts fusion kind".to_string());
    }
    let val = self.lexer.get_str_val();
    let fusion_kind =
      hlo_instruction::string_to_fusion_kind(&val);
    if fusion_kind.is_err() {
      let mut err_msg = "expects fusion kind but sees: ".to_string();
      err_msg.push_str(&val);
      return self.token_error(err_msg);
    }
    *result = fusion_kind.unwrap();
    self.lexer.lex(0);
    true
  }

  fn parse_random_distribution(&mut self, result: &mut RandomDistribution) -> bool {
    println!("parse_random_distribution");
    if self.lexer.get_kind() != TokKind::Ident {
      return self.token_error("expects random distribution".to_string());
    }
    let val = self.lexer.get_str_val();
    let random_distribution =
      hlo_instruction::string_to_random_distribution(&val);
    if random_distribution.is_err() {
      let mut err_msg = "expects random distribution but sees: ".to_string();
      err_msg.push_str(&val);
      return self.token_error(err_msg);
    }
    *result = random_distribution.unwrap();
    self.lexer.lex(0);
    true
  }

  fn parse_collective_op_group_mode(
    &mut self, result: &mut CollectiveOpGroupMode) -> bool
  {
    println!("parse_collective_op_group_mode");
    if self.lexer.get_kind() != TokKind::Ident {
      assert!(false, "expects collective op group mode");
      return false;
    }
    let val = self.lexer.get_str_val();
    let status_or =
      string_to_collective_op_group_mode(val.clone());
    if status_or.is_err() {
      assert!(false, "expects collective op group mode but sees {:?}, error: {:?}",
        val, status_or.err().unwrap());
      return false;
    }
    *result = status_or.unwrap();
    self.lexer.lex(0);
    true
  }

  fn parse_random_algorithm(&mut self, result: &mut RandomAlgorithm) -> bool {
    println!("parse_random_algorithm");
    if self.lexer.get_kind() != TokKind::Ident {
      return self.token_error("expects random algorithm".to_string());
    }
    let val = self.lexer.get_str_val();
    let random_algorithm =
      hlo_instruction::string_to_random_algorithm(&val);
    if random_algorithm.is_err() {
      let mut err_msg = "expects random algorithm but sees: ".to_string();
      err_msg.push_str(&val);
      return self.token_error(err_msg);
    }
    *result = random_algorithm.unwrap();
    self.lexer.lex(0);
    true
  }

  fn parse_precision(&mut self, result: &mut Precision) -> bool {
    println!("parse_precision");
    if self.lexer.get_kind() != TokKind::Ident {
      return self.token_error("expects precision".to_string());
    }
    let val = self.lexer.get_str_val();
    let precision = hlo_instruction::string_to_precision(&val);
    if precision.is_err() {
      let mut err_msg = "expects precision but sees: ".to_string();
      err_msg.push_str(&val);
      return self.token_error(err_msg);
    }
    *result = precision.unwrap();
    self.lexer.lex(0);
    true
  }

  fn parse_algorithm(&mut self, result: &mut Algorithm) -> bool {
    println!("parse_algorithm");
    if self.lexer.get_kind() != TokKind::Ident {
      return self.token_error("expects algorithm".to_string());
    }
    let val = self.lexer.get_str_val();
    let algorithm = hlo_instruction::string_to_algorithm(&val);
    if algorithm.is_err() {
      let mut err_msg = "expects algorithm but sees: ".to_string();
      err_msg.push_str(&val);
      return self.token_error(err_msg);
    }
    *result = algorithm.unwrap();
    self.lexer.lex(0);
    true
  }

  fn parse_result_accuracy_type(&mut self, result: &mut ResultAccuracyMode) -> bool {
    println!("parse_result_accuracy_type");
    if self.lexer.get_kind() != TokKind::Ident {
      return self.token_error("expects ResultAccuracy type".to_string());
    }
    let val = self.lexer.get_str_val();
    let mode = string_to_result_accuracy(&val);
    if mode.is_err() {
      let mut err_msg = "expects ResultAccuracy type but sees: ".to_string();
      err_msg.push_str(&val);
      err_msg.push_str(", error: ");
      err_msg.push_str(&mode.err().unwrap());
      return self.token_error(err_msg);
    }
    *result = mode.ok().unwrap();
    self.lexer.lex(0);
    true
  }

  fn parse_result_accuracy_tolerance(
    &mut self, result_tolerance: &mut ResultAccuracyTolerance) -> bool
  {
    println!("parse_result_accuracy_tolerance");
    if !self.parse_token(&TokKind::Lbrace,
        "expected '{' to start result accuracy list".to_string())
    {
      return false;
    }
    let mut ulps = 0.0;
    let mut rtol = 0.0;
    let mut atol = 0.0;
    if self.lexer.get_kind() != TokKind::Rbrace {
      loop {
        let mut name = String::new();
        if !self.parse_attribute_name(&mut name) {
          return self.error(self.lexer.get_loc(),
            "expects string for result_accuracy tolerance type".to_string());
        }
        if name == "ulps" {
          if self.parse_double(&mut ulps) {
            result_tolerance.set_ulps(ulps);
          }
        } else if name == "rtol" {
          if self.parse_double(&mut rtol) {
            result_tolerance.set_rtol(rtol);
          }
        } else if name == "atol" {
          if self.parse_double(&mut atol) {
            result_tolerance.set_atol(atol);
          }
        } else {
          let mut err_msg = "invalid attribute name: ".to_string();
          err_msg.push_str(&name);
          return self.error(self.lexer.get_loc(), err_msg);
        }
        if !self.eat_if_present(&TokKind::Comma) {
          break;
        }
      }
    }
    self.parse_token(&TokKind::Rbrace,
      "expects '}' at the end of result precision".to_string())
  }

  fn parse_result_accuracy(&mut self, result: &mut ResultAccuracy) -> bool {
    println!("parse_result_accuracy");
    if !self.parse_token(&TokKind::Lbrace,
      "expected '{' to start result precision list".to_string())
    {
      return false;
    }
    let mut mode = ResultAccuracyMode::Default;
    let mut result_tolerance = ResultAccuracyTolerance::default();
    let mut name = String::new();
    if !self.parse_attribute_name(&mut name) {
      return self.error(self.lexer.get_loc(),
      "expects string for result_accuracy spec".to_string());
    }
    if name == "mode" {
      if self.parse_result_accuracy_type(&mut mode) {
        result.set_mode(mode);
      }
    } else if name == "tolerance" {
      if self.parse_result_accuracy_tolerance(&mut result_tolerance) {
        *result.mutable_tolerance() = result_tolerance;
      }
    } else {
      let mut err_msg = "invalid attribute name: ".to_string();
      err_msg.push_str(&name);
      return self.error(self.lexer.get_loc(), err_msg);
    }
    self.parse_token(&TokKind::Rbrace,
      "expected '}' to end result_accuracy".to_string())
  }

  fn parse_i64(&mut self, result: &mut i64) -> bool {
    println!("parse_int64");
    if self.lexer.get_kind() != TokKind::Int {
      return self.token_error("expects integer".to_string());
    }
    *result = self.lexer.get_i64_val();
    self.lexer.lex(0);
    true
  }

  fn parse_double(&mut self, result: &mut f64) -> bool {
    match self.lexer.get_kind() {
      TokKind::Decimal => {
        let val = self.lexer.get_decimal_val();
        if val.is_infinite() {
          let mut err_msg = "Constant is out of range for double (+/-".to_string();
          err_msg.push_str(&f64::MAX.to_string());
          err_msg.push_str(") and so is unparsable.");
          return self.token_error(err_msg);
        }
      },
      TokKind::Int => *result = self.lexer.get_i64_val() as f64,
      TokKind::Inf => *result = f64::INFINITY,
      TokKind::NegInf => *result = f64::NEG_INFINITY,
      _ => return self.token_error("expects decimal or integer".to_string())
    };
    self.lexer.lex(0);
    true
  }

  fn parse_complex(&mut self, result: &mut Complex64) -> bool {
    if self.lexer.get_kind() != TokKind::Lparen {
      return self.token_error("expects '(' before complex number".to_string());
    }
    self.lexer.lex(0);

    let mut real = 0.0;
    let loc = self.lexer.get_loc();
    if !self.parse_double(&mut real) {
      return self.error(loc,
        "expect floating-point value for real part of complex number".to_string());
    }

    if self.lexer.get_kind() != TokKind::Comma {
      return self.token_error("expect comma after real part of complex literal".to_string());
    }
    self.lexer.lex(0);

    let mut imag = 0.0;
    let loc = self.lexer.get_loc();
    if !self.parse_double(&mut imag) {
      return self.error(loc,
        "expect floating-point value for imaginary part of complex number".to_string());
    }

    if self.lexer.get_kind() != TokKind::Rparen {
      return self.token_error("expect ')' after complex number".to_string());
    }

    *result = Complex64::new(real, imag);
    self.lexer.lex(0);
    true
  }

  fn parse_bool(&mut self, result: &mut bool) -> bool {
    if self.lexer.get_kind() != TokKind::True &&
       self.lexer.get_kind() != TokKind::False
    {
      return self.token_error("expexts true or false".to_string());
    }
    if self.lexer.get_kind() == TokKind::True {
      *result = true;
    } else {
      *result = false;
    }
    self.lexer.lex(0);
    true
  }

  fn parse_token(&mut self, kind: &TokKind, msg: String) -> bool {
    println!("parse_token {:?} {:?}", kind, msg);
    if self.lexer.get_kind() != *kind {
      return self.token_error(msg);
    }
    self.lexer.lex(0);
    true
  }

  // OriginalValueRecoveryTable ::= '{' OriginalArray ':' OriginalArray ','
  //   HloModule | OriginalValueRecoveryTable '}'
  pub fn parse_original_value_recovery_table(
    &mut self, _table: &mut OriginalValueRecoveryTable) -> bool
  {
    unimplemented!()    
  }

  fn parse_unsigned_integer_type() {}

  fn parse_shape_index(&mut self, out: &mut Vec<i64>) -> bool {
    if self.parse_token(&TokKind::Lbrace,
      "expects '{' at the start of shape index".to_string())
    {
      return false;
    }
    let mut idxs = vec![];
    while self.lexer.get_kind() != TokKind::Rbrace {
      let mut idx = 0;
      if !self.parse_i64(&mut idx) {
        return false;
      }
      idxs.push(idx);
      if !self.eat_if_present(&TokKind::Comma) {
        break;
      }
    }
    if self.parse_token(&TokKind::Rbrace,
      "expects '}' at the end of shape index".to_string())
    {
      return false;
    }
    *out = idxs;
    true
  }

  fn parse_aliasing(
    &mut self, aliasing_data: &mut BTreeMap<Vec<i64>, Alias>) -> bool
  {
    if self.parse_token(&TokKind::Lbrace,
      "expects '{' at the start of aliasing description".to_string())
    {
      return false;
    }
    while self.lexer.get_kind() != TokKind::Rbrace {
      let mut out = vec![];
      if !self.parse_shape_index(&mut out) {
        return false;
      }
      let err_msg = "expected format: (<output_shape_index>,
        <input_param_shape_index>) or <output_shape_index>: <input_param>".to_string();
      if self.parse_token(&TokKind::Colon, err_msg.clone()) {
        return false;
      }
      if self.parse_token(&TokKind::Lparen, err_msg.clone()) {
        return false;
      }
      let mut param_num = 0;
      self.parse_i64(&mut param_num);
      if self.parse_token(&TokKind::Comma, err_msg.clone()) {
        return false;
      }
      let mut param_idx = vec![];
      if self.parse_shape_index(&mut param_idx) {
        return false;
      }
      let mut alias_kind = AliasKind::May;
      if self.eat_if_present(&TokKind::Comma) {
        let mut t = String::new();
        self.parse_name(&mut t);
        if &t == "must-alias" {
          alias_kind = AliasKind::Must;
        } else if &t == "may-alias" {
          alias_kind = AliasKind::May;
        } else {
          return self.token_error(
            "unexpected aliasing kind; expected SYSTEM or USER".to_string());
        }
      }
      aliasing_data.insert(out, Alias::new(
        param_num, param_idx, alias_kind));
      if self.parse_token(&TokKind::Rparen, err_msg.clone()) {
        return false;
      }
      if !self.eat_if_present(&TokKind::Comma) {
        break;
      }
    }
    if self.parse_token(&TokKind::Rbrace,
      "expects '}' at the end of aliasing description".to_string())
    {
      return false;
    }
    true
  }

  fn parse_buffer_donor() {}

  fn parse_computation_layout(
    &mut self, computation_layout: &mut ComputationLayout) -> bool
  {
    if !self.parse_token(&TokKind::Lbrace,
      "expects '{' at the start of aliasing description".to_string())
    {
      return false;
    }
    if !self.parse_token(&TokKind::Lparen,
      "expects ( before parameter shape list".to_string())
    {
      return false;
    }
    while self.lexer.get_kind() != TokKind::Rparen {
      let mut param = Shape::default();
      if !self.parse_shape(&mut param,
        !self.options.keep_module_auto_layouts())
      {
        return false;
      }
      computation_layout.add_parameter_layout(ShapeLayout::new(param));
      if self.lexer.get_kind() == TokKind::Rparen {
        break;
      }
      if !self.parse_token(&TokKind::Comma,
        "expects , between parameter shapes".to_string())
      {
        return false
      }
    }
    if !self.parse_token(&TokKind::Rparen,
      "expects ) at end of parameter shape list".to_string())
    {
      return false;
    }
    if !self.parse_token(&TokKind::Arrow,
      "expects -> before result shape".to_string())
    {
      return false;
    }
    let mut result = Shape::default();
    if !self.parse_shape(&mut result,
      !self.options.keep_module_auto_layouts())
    {
      return false;
    }
    *computation_layout.mutable_result_layout() = ShapeLayout::new(result);
    if !self.parse_token(&TokKind::Rbrace,
      "expects '}' at the end of computation layouts".to_string())
    {
      return false;
    }
    true
  }

  fn parse_instruction_output_operand_aliasing(
    &mut self,
    aliasing_output_operand_pairs: &mut Vec<(Vec<i64>, (i64, Vec<i64>))>) -> bool
  {
    if !self.parse_token(&TokKind::Lbrace,
      "expects '{' at the start of instruction aliasing description".to_string())
    {
      return false;
    }
    while self.lexer.get_kind() != TokKind::Rbrace {
      let mut out = vec![];
      if !self.parse_shape_index(&mut out) {
        return false;
      }
      let err_msg = "expected format: <output_shape_index>: (<operand_index>, 
        <operand_shape_index>)".to_string();
      if !self.parse_token(&TokKind::Colon, err_msg.clone()) {
        return false;
      }
      if !self.parse_token(&TokKind::Lparen, err_msg.clone()) {
        return false;
      }
      let mut operand_index = 0;
      self.parse_i64(&mut operand_index);
      if !self.parse_token(&TokKind::Comma, err_msg.clone()) {
        return false;
      }
      let mut operand_shape_index = vec![];
      if !self.parse_shape_index(&mut operand_shape_index) {
        return false;
      }
      aliasing_output_operand_pairs.push((out, (operand_index, operand_shape_index)));
      if !self.parse_token(&TokKind::Rparen, err_msg.clone()) {
        return false;
      }
      if !self.eat_if_present(&TokKind::Comma) {
        break;
      }
    }
    if !self.parse_token(&TokKind::Rbrace,
      "expects '}' at the end of instruction aliasing description".to_string())
    {
      return false;
    }
    true
  }

  fn parse_parse_custom_call_schedule(
    &mut self, result: &mut CustomCallSchedule) -> bool
  {
    println!("parse_custom_call_schedule");
    if self.lexer.get_kind() != TokKind::Ident {
      return self.token_error("expects custom-call schedule".to_string());
    }
    let val = self.lexer.get_str_val();
    let status_or_result =
      string_to_custom_call_schedule(&val);
    if status_or_result.is_err() {
      let mut err_msg = "expects custom-call schedule but sees: ".to_string();
      err_msg.push_str(&val);
      err_msg.push_str(", error: ");
      err_msg.push_str(&status_or_result.err().unwrap());
      return self.token_error(err_msg);
    }
    *result = status_or_result.ok().unwrap();
    self.lexer.lex(0);
    true
  }

  fn parse_custom_call_api_version(
    &mut self, result: &mut CustomCallApiVersion) -> bool
  {
    println!("parse_custom_call_api_version");
    if self.lexer.get_kind() != TokKind::Ident {
      return self.token_error("expects custom call API version".to_string());
    }
    let val = self.lexer.get_str_val();
    let status_or_result =
      string_to_custom_call_api_version(&val);
    if status_or_result.is_err() {
      let mut err_msg = "expects custom-call API version but sees: ".to_string();
      err_msg.push_str(&val);
      err_msg.push_str(", error: ");
      err_msg.push_str(&status_or_result.err().unwrap());
      return self.token_error(err_msg);
    }
    *result = status_or_result.ok().unwrap();
    self.lexer.lex(0);
    true
  }

  fn parse_sparsity_descriptor() {}

  fn can_be_shape(&mut self) -> bool {
    // A non-tuple shape starts with a PrimitiveType token; a tuple shape starts
    // with '('.
    self.lexer.get_kind() == TokKind::PrimitiveType ||
    self.lexer.get_kind() == TokKind::Lparen ||
    (self.lexer.get_kind() == TokKind::Ident &&
    self.lexer.get_str_val() == "b".to_string() &&
    self.lexer.look_ahead() == TokKind::Lparen)
  }

  fn can_be_param_list_to_shape(&self) -> bool {
    self.lexer.get_kind()== TokKind::Lparen
  }

  // Logs the currentparsing line and the given message. Always return false.
  fn token_error(&mut self, msg: String) -> bool {
    self.error(self.lexer.get_loc(), msg)
  }

  fn error(&mut self, loc: usize, msg: String) -> bool {
    let line_col = self.lexer.get_line_and_column(loc);
    let line = line_col.0;
    let col = line_col.1;

    let mut error_lines = vec![];
    let mut err_msg = "was parsing ".to_string();
    err_msg.push_str(&line.to_string());
    err_msg.push_str(":");
    err_msg.push_str(&col.to_string());
    err_msg.push_str(&msg);
    error_lines.push(err_msg);

    println!("Error: {:?}", error_lines);
    false
  }

  fn eat_if_present(&mut self, kind: &TokKind) -> bool {
    if self.lexer.get_kind() != *kind {
      return false;
    }
    self.lexer.lex(0);
    true
  }

  // Adds the instruction to the pool. Returns false and emits an error if the
  // instruction already exists.
  fn add_instruction(
    &mut self,
    name: String,
    instruction: HloInstruction,
    name_loc: usize) -> bool
  {
    let result =
      self.mutable_current_name_table()
        .unwrap().insert(name.clone(), (instruction, name_loc));
    if result.is_none() {
      assert!(false, "instruction already exists: {:?}", name);
      return false;
    }
    true
  }

  // Adds the computation to the pool. Returns false and emits an error if the
  // computation already exists.
  fn add_computation(
    &mut self,
    name: String,
    computation: HloComputation, 
    name_loc: usize) -> bool
  {
    let result =
      self.computation_pool.insert(name.clone(), (computation, name_loc));
    if result.is_none() {
      assert!(false, "computation already exists: {:?}", name);
      return false;
    }
    true
  }
}

// A helper class which pushes and pops to an InstrNameTable stack via RAII.
pub struct HloParserScope {
  scoped_name_tables: Vec<HashMap<String, (HloInstruction, usize)>>
}

impl HloParserScope {
  pub fn new(
    scoped_name_tables: &Vec<HashMap<String, (HloInstruction, usize)>>) -> Self
  {
    let mut cloned_tables = vec![];
    cloned_tables.clone_from(scoped_name_tables);
    HloParserScope {
      scoped_name_tables: cloned_tables
    }
  }
}

// Creates and returns a schedule created using the order of the instructions in
// the HloComputation::instructions() vectors in the module.
fn schedule_from_instruction_order(module: &HloModule) -> HloSchedule<'_> {
  let mut schedule = HloSchedule::new(module);
  for comp in module.computations() {
    if !comp.is_fusion_computation() {
      for instr in comp.instructions() {
        schedule.get_or_create_mutable_sequence(
          module, comp).push_back(instr.clone());
      }
    }  
  }
  schedule
}

#[cfg(test)]
mod tests {
  use common::array2d::Array2D;
use hlo::hlo_instruction::HloPrintOptions;

use crate::hlo_verifier::HloVerifier;

  use super::*;

  // An HLO module derived class which verifies itself on destruction. This class
  // is intended to be used in unit tests. Any verification errors are raised via
  // ADD_FAILURE.
  pub struct VerifiedHloModule<'func> {
    module: HloModule,
    verifier: HloVerifier<'func>
  }

  impl<'func> VerifiedHloModule<'func> {
    pub fn new(
      name: String,
      config: HloModuleConfig,
      verifier_layout_sensitive: bool,
      allow_mixed_precision_in_hlo_verifier: bool,
      shape_size_func: &'func dyn Fn(&Shape)->i64,
      instruction_can_change_layout_func: &'func dyn Fn(&HloInstruction)->bool) -> Self
    {
      VerifiedHloModule {
        module: HloModule::new(name, config),
        verifier: HloVerifier::new(verifier_layout_sensitive,
          allow_mixed_precision_in_hlo_verifier,
          instruction_can_change_layout_func, shape_size_func,
          false)
      }
    }

    // Given a string in the HloModule::ToString() format, parses the string and
    // builds the VerifiedHloModule in place. Before calling this method, the
    // module must be empty (no computations). Finally verifies the module using
    // HloVerifier and returns the status.
    pub fn parse_hlo_string_and_verify_module(
      &mut self,
      str: String,
      options: HloParserOptions) -> Result<bool, String>
    {
      debug_assert!(self.module.computation_count() == 0);
      let mut parser = HloParser::new_for_tests(str, options);
      if parser.run(&mut self.module).is_err() {
        return Err("parse error".to_string());
      }
      self.verify()
    }

    // Verifies the module and flags any error with ADD_FAILURE. 'message' is
    // included in the failure message.
    pub fn verify_or_add_failure(&self, message: String) {
      let status = self.verify();
      if status.is_err() {
        let mut err_msg = "HloVerifier failed on module ".to_string();
        err_msg.push_str(&self.module.name());
        if !message.is_empty() {
          err_msg.push_str("(");
          err_msg.push_str(&message);
          err_msg.push_str(")");
          assert!(false, "{:?}", err_msg);
        }
      }
    }

    // Verifies the module using HloVerifier and returns the status.
    pub fn verify(&self) -> Result<bool, String> {
      if self.module.computation_count() == 0 {
        // The computation was never built. Nothing to verify.
        return Ok(true);
      }
      self.verifier.run(&self.module, HashSet::new())
    }
  }

  fn parse_and_return_verified_module(hlo_text: String) -> Result<HloModule, String> {
    parse_and_return_verified_module_inner(
      "test_hlo_parser".to_string(), hlo_text, HloModuleConfig::default())
  }

  fn parse_and_return_verified_module_inner(
    _name: String,
    _hlo_text: String,
    _config: HloModuleConfig) -> Result<HloModule, String>
  {
    unimplemented!()  
  }

  #[test]
  fn test_empty() {
    let original = "".to_string();
    let result = parse_and_return_unverified_module(
      original, HloModuleConfig::default(),
      HloParserOptions::default());
    assert!(result.is_err());
  }

  #[test]
  fn test_garbage() {
    let original =
      "HloModule thi$ str1ng makes# N0 sen$e @all!*&^%$".to_string();
    let result = parse_and_return_unverified_module(
      original, HloModuleConfig::default(),
      HloParserOptions::default());
    assert!(result.is_err());
  }

  #[test]
  fn test_wrong_opcode() {
    let original = "HloModule wrong_opcode:
ENTRY %blabla (x: f32[], y: f32[]) -> f32[] {
  %x = f32[]{} parameter(0)
  %y = f32[]{} parameter(1)
  %le = pred[]{} le(f32[]{} %x, f32[]{} %y)
}
".to_string();
    let result = parse_and_return_unverified_module(
      original, HloModuleConfig::default(),
      HloParserOptions::default());
    assert!(result.is_err());
  }

  #[test]
  fn test_metadata_with_cholesky() {
    unimplemented!()
  }

  #[test]
  fn test_wrong_shape() {
    let original = "HloModule wrong_opcode:
ENTRY %blabla (x: g32[]) -> g32[] {
  %x = g32[]{} parameter(0)
}
".to_string();
    let result = parse_and_return_unverified_module(
      original, HloModuleConfig::default(),
      HloParserOptions::default());
    assert!(result.is_err());
  }

  #[test]
  fn test_wrong_operand_size() {
    let original = "HloModule wrong_opcode:
ENTRY %blabla (x: f32[]) -> pred[] {
  %x = f32[]{} parameter(0)
  %eq = pred[]{} compare(f32[]{} %x), direction=EQ
}
".to_string();
    let result = parse_and_return_unverified_module(
      original, HloModuleConfig::default(),
      HloParserOptions::default());
    assert!(result.is_err());
  }

  #[test]
  fn test_operand_not_found() {
    let original = "HloModule operand_not_found:
ENTRY %blabla (x: f32[]) -> pred[] {
  %x = f32[]{} parameter(0)
  %eq = pred[]{} compare(f32[]{} %x, f32[]{} %y), direction=EQ
}
".to_string();
    let result = parse_and_return_unverified_module(
      original, HloModuleConfig::default(),
      HloParserOptions::default());
    assert!(result.is_err());
  }

  #[test] // FAIL
  fn test_compact_gte_non_tuple() {
    let original = "HloModule test
ENTRY test {
  p0 = f32[10] parameter(0)
  ROOT root = f32[10] add(f32[10] %p0#0, f32[10] %p0#0)
}".to_string();
    let result = parse_and_return_unverified_module(
      original, HloModuleConfig::default(),
      HloParserOptions::default());
    assert!(result.is_err()); // TODO: err msg
  }

  #[test]
  fn test_compact_gte_index_out_of_bounds() {
    let original = "HloModule test
ENTRY test {
  p0 = (f32[10], f16[10]) parameter(0)
  ROOT root = f32[10] add(f32[10] %p0#2, f32[10] %p0#2)
}".to_string();
    let result = parse_and_return_unverified_module(
      original, HloModuleConfig::default(),
      HloParserOptions::default());
    assert!(result.is_err()); // TODO: err msg
  }

  #[test]
  fn test_compact_gte_negative_index() {
    let original = "HloModule test
ENTRY test {
  p0 = (f32[10], f16[10]) parameter(0)
  ROOT root = f32[10] add(f32[10] %p0#-1, f32[10] %p0#-1)
}".to_string();
    let result = parse_and_return_unverified_module(
      original, HloModuleConfig::default(),
      HloParserOptions::default());
    assert!(result.is_err()); // TODO: err msg
  }

  #[test] // FAIL
  fn test_compact_gte_round_trip() {
    let original = "
HloModule test, entry_computation_layout={((f32[10]{0}, f16[10]{0}))->f32[10]{0}}

ENTRY %test {
  %p0 = (f32[10]{0}, f16[10]{0}) parameter(0)
  ROOT %root = f32[10]{0} add(f32[10]{0} %p0#0, f32[10]{0} %p0#0)
}".to_string();
    let module_wrapper = parse_and_return_unverified_module(
      original.clone(), HloModuleConfig::default(),
      HloParserOptions::default());
    assert!(module_wrapper.is_ok());
    let module = module_wrapper.unwrap();

    let mut options = HloPrintOptions::short_parsable();
    options.set_compact_gte(true);
    options.set_print_operand_shape(true);
    options.set_print_percent(true);

    let printed = module.to_string();
    let original_splitted: String = original.split_ascii_whitespace().collect();
    let printed_splitted: String = printed.split_ascii_whitespace().collect();
    assert_eq!(original_splitted, printed_splitted);
  }

  #[test] // FAIL
  fn test_more_constants() {
    let original = "HloModule SelectScalarS32True_module
ENTRY %SelectScalarS32True.v4 () -> s32[] {
  %constant.2 = pred[] constant(true)
  %constant.1 = s32[] constant(-42), sharding={replicated}
  %constant = s32[] constant(42)
  %select = s32[] select(pred[] %constant.2, s32[] %constant.1, s32[] %constant)
}
".to_string();

    let result = parse_and_return_unverified_module(
      original, HloModuleConfig::default(),
      HloParserOptions::default());
    assert!(result.is_ok());
    // Constant instructions have no name. The string will be parsed successfully
    // but the constant names will not be exactly the same.
  }

  #[test] // FAIL
  fn test_configuration_field() {
    let original = "HloModule AModule
ENTRY %configuration_test() -> s32[] {
  %constant = s32[] constant(42), backend_config=\"foo bar\"
}".to_string();
    let result = parse_and_return_unverified_module(
      original, HloModuleConfig::default(),
      HloParserOptions::default());
    assert!(result.is_ok());
    //assert_eq!(result.ok().as_ref().unwrap().entry_computation().as_ref()
      //.unwrap().root_instruction().raw_backend_config_string(),
      //"foo bar".to_string());
  }

  #[test] // FAIL
  fn test_literal_dimensions_error() {
    let original = "HloModule some_2x3_module

ENTRY %some_2x3 () -> f32[2,3] {
  ROOT %constant = f32[2,3]{1,0} constant(}{1, 2, 3}, {4, 5, 6}})
}

".to_string();
    let result = parse_and_return_unverified_module(
      original, HloModuleConfig::default(),
      HloParserOptions::default());
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "unexpected '}' token".to_string());
  }

  #[test]
  fn test_parse_sharding() {
    let original = "{maximal device=42}".to_string();
    let sharding = parse_sharding(original.clone());
    assert!(sharding.is_ok());
    assert_eq!(sharding.unwrap().to_string(false), original);
  }

  #[test]
  fn test_parse_sharding_partial_replication() {
    let original =
      "{devices=[2,2]0,1,2,3 last_tile_dim_replicate}".to_string();
    let sharding = parse_sharding(original.clone());
    assert!(sharding.is_ok());
    assert_eq!(sharding.unwrap().to_string(false), original);
    let array_2d: Array2D<i64> =
      Array2D::new_from(vec![vec![0, 1], vec![2, 3]]);
    let tiling_last_dim_replicated =
      TileAssignment::new_from_array_2d(array_2d);
    assert_eq!(HloSharding::partial_tile(tiling_last_dim_replicated,
      vec![]).to_string(false), original);
  }

  #[test]
  fn test_parse_sharding_subgroup() {
    let mut original = String::new();
    original.push_str("{devices=[2,2,2,2]0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15 ");
    original.push_str("last_tile_dims={manual, replicated}}");
    let sharding = parse_sharding(original.clone());
    assert!(sharding.is_ok());
    assert_eq!(sharding.unwrap().to_string(false), original);

    let mut tile_assignment = Array::new(&vec![2, 2, 2, 2]);
    tile_assignment.fill_iota(0);
    let subgroup_types =
      vec![OpShardingType::Manual, OpShardingType::Replicated];
    assert_eq!(HloSharding::subgroup_from_array(
      tile_assignment, subgroup_types, vec![]).to_string(false), original);
  }

  #[test]
  fn test_parse_trivial_sharding_partial_replication() {
    let original = "{devices=[2,2]<=[4] last_tile_dim_replicate}".to_string();
    let sharding = parse_sharding(original.clone());
    assert!(sharding.is_ok());
    assert_eq!(sharding.unwrap().to_string(false), original);
    let tiling_last_dim_replicated =
      TileAssignment::new_from_vec(&vec![2, 2]);
    assert_eq!(HloSharding::partial_tile(
      tiling_last_dim_replicated, vec![]).to_string(false), original);
  }

  #[test]
  fn test_parse_trivial_sharding_subgroup() {
    let original =
      "{devices=[2,2,2,2]<=[16] last_tile_dims={manual, replicated}}".to_string();
    let sharding = parse_sharding(original.clone());
    assert!(sharding.is_ok());
    assert_eq!(sharding.unwrap().to_string(false), original);
    let tile_assignment =
      TileAssignment::new_from_vec(&vec![2, 2, 2, 2]);
    let subgroup_types =
      vec![OpShardingType::Manual, OpShardingType::Replicated];
    assert_eq!(HloSharding::subgroup(
      tile_assignment, subgroup_types, vec![]).to_string(false), original);
  }

  #[test]
  fn test_parse_transposed_iota_sharding_partial_replication() {
    let original =
      "{devices=[2,2]<=[2,2]T(1,0) last_tile_dim_replicate}".to_string();
    let sharding = parse_sharding(original.clone());
    assert!(sharding.is_ok());
    assert_eq!(sharding.unwrap().to_string(false), original);

    let tiling_last_dim_replicated = TileAssignment::new_from_vecs(
      &vec![2, 2], &vec![2, 2], &vec![1, 0]);
    assert_eq!(HloSharding::partial_tile(
      tiling_last_dim_replicated, vec![]).to_string(false), original);
  }

  #[test]
  fn test_parse_transposed_iota_sharding_subgroup() {
    let original =
      "{devices=[2,2,2,2]<=[2,2,4]T(2,1,0) last_tile_dims={manual, replicated}}".to_string();
    let sharding = parse_sharding(original.clone());
    assert!(sharding.is_ok());
    assert_eq!(sharding.unwrap().to_string(false), original);

    let tile_assignment = TileAssignment::new_from_vecs(
      &vec![2, 2, 2, 2], &vec![2, 2, 4], &vec![2, 1, 0]);
    let subgroup_types =
      vec![OpShardingType::Manual, OpShardingType::Replicated];
    assert_eq!(HloSharding::subgroup(
      tile_assignment, subgroup_types, vec![]).to_string(false), original);
  }

  #[test]
  fn test_parse_shard_as() {
    let original = "{manual shard_as 1}".to_string();
    let sharding = parse_sharding(original.clone());
    assert!(sharding.is_ok());
    assert_eq!(sharding.unwrap().to_string(false), original);
    assert_eq!(HloSharding::manual(vec![])
      .set_shard_group(HloSharding::shard_as(1)).to_string(false), original);
  }

  #[test]
  fn test_parse_shard_like() {
    let original =
      "{devices=[2,2,2,2]<=[16] last_tile_dims={manual, replicated} shard_like 1}".to_string();
    let sharding = parse_sharding(original.clone());
    assert!(sharding.is_ok());
    assert_eq!(sharding.unwrap().to_string(false), original);
  }

  #[test]
  fn test_parse_unknown_sharding() {
    let original = "{unknown}".to_string();
    let sharding = parse_sharding(original.clone());
    assert!(sharding.is_ok());
    assert_eq!(sharding.unwrap().to_string(false), original);
    assert_eq!(HloSharding::unknown(vec![]).to_string(false), original);
  }

  #[test]
  fn test_parse_frontend_attributes() {
      
  }
}