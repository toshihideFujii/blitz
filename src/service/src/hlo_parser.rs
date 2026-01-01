#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use common::{
  blitz_data::{
    Algorithm, ConvolutionDimensionNumbers, FftType, FrontendAttributes, OpMetadata,
    OpShardingType, PaddingConfig, ParameterReplication, Precision, PrimitiveType,
    RandomAlgorithm, RandomDistribution, ReplicaGroup, Statisitic, StatisticsViz, Window
  },
  comparison_util::{
    ComparisonDirection, ComparisonType, string_to_comparison_direction, string_to_comparison_type
  },
  layout::Layout, layout_util::LayoutUtil, literal::Literal, literal_util::LiteralUtil,
  shape::Shape, shape_util::ShapeUtil
};
use hlo::{
  hlo_computation::HloComputation, hlo_domain_metadata::DomainMetadata,
  hlo_instruction::{self, FusionKind, HloInstruction}, hlo_module::HloModule,
  hlo_module_config::HloModuleConfig, hlo_opcode::HloOpcode, hlo_sharding::HloSharding, tile_assignment::TileAssignment
};
use num::complex::Complex64;
use crate::hlo_lexer::{tok_kind_to_string, HloLexer, TokKind};

pub struct HloParserOptions {
  fill_missing_layouts: bool,
  fill_shortform_constants_with_random_values: bool,
  keep_module_auto_layouts: bool,
}

impl HloParserOptions {
  pub fn default() -> Self {
    HloParserOptions {
      fill_missing_layouts: false,
      fill_shortform_constants_with_random_values: false,
      keep_module_auto_layouts: false
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
  let module = HloModule::new("_".to_string(), config);
  let mut parser = HloParser::new(str, options);
  let result = parser.run(&module);
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
pub fn parse_layout(_str: String) -> Result<Layout, String> {
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

fn can_infer_shape(code: HloOpcode) -> bool {
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
  StringOrJsonDict,
}

struct AttrConfig {
  required: bool,
  attr_type: AttrType,
  result: String // TODO
}

impl AttrConfig {
  pub fn new(required: bool, attr_type: AttrType, result: String) -> Self {
    AttrConfig {
      required: required,
      attr_type: attr_type,
      result: result
    }
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

struct Scope {}

pub struct HloParser {
  lexer: HloLexer,
  options: HloParserOptions,
  scoped_name_tables: Vec<HashMap<String, (HloInstruction, usize)>>,
  computation_pool: HashMap<String, (HloComputation, usize)>,
  computations: Vec<HloComputation>,
  error: Vec<String>
}

impl HloParser {
  pub fn new(str: String, options: HloParserOptions) -> Self {
    HloParser {
      lexer: HloLexer::new(str),
      options: options,
      scoped_name_tables: Vec::new(),
      computation_pool: HashMap::new(),
      computations: Vec::new(),
      error: Vec::new(),
    }
  }

  // Runs the parser and constructs the resulting HLO in the given (empty)
  // HloModule. Returns the error status in case an error occurred.
  pub fn run(&mut self, module: &HloModule) -> Result<(), String> {
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
          "Syntac error when trying to parse the text as a HloModule.".to_string();
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
    let mut shape = Shape::new();
    if !self.parse_shape(&mut shape) {
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
    let layout = Layout::new();
    if !self.parse_layout(&layout) {
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
  fn current_name_table(&self) -> Option<&HashMap<String, (HloInstruction, usize)>> {
    self.scoped_name_tables.last()
  }

  fn find_instruction(&self, _name: &String) -> Option<&(HloInstruction, usize)> {
    unimplemented!()
  }

  fn parse_single_instruction(&self, _module: &HloModule) -> bool {
    unimplemented!()
  }

  fn parse_hlo_module(&self, _module: &HloModule, _parse_module_without_header: bool) -> bool {
    unimplemented!()
  }

  fn parse_computations() {}
  fn parse_computation() {}
  fn parse_instruction_list() {}
  fn parse_instruction() {}
  fn parse_instruction_rhs() {}
  fn parse_control_predecessors() {}

  // literal
  //  ::= tuple
  //  ::= non_tuple
  fn parse_literal<T>(&mut self, literal: &mut Literal<T>, shape: &Shape) -> bool
    where T: Clone + Default + PartialEq
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
  fn parse_tuple_literal<T>(&mut self, literal: &mut Literal<T>, shape: &Shape) -> bool
    where T: Clone + Default + PartialEq
  {
    if self.parse_token(&TokKind::Lparen,
        "expects '(' in front of tuple elements".to_string())
    {
      return false;
    }

    let element_count = ShapeUtil::tuple_element_count(shape);
    let mut elements: Vec<Literal<T>> = Vec::new();
    elements.reserve(element_count);
    if self.lexer.get_kind() == TokKind::Rparen {
      // empty
    } else {
      for i in 0..element_count {
        if i > 0 {
          self.parse_token(&TokKind::Comma,
            "expects ',' to separate tuple elements".to_string());
        }
        if !self.parse_literal(&mut elements[i],
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
  fn parse_non_tuple_literal<T>(&mut self, literal: &Literal<T>, shape: &Shape) -> bool
    where T: Clone + Default + PartialEq
  {
    debug_assert!(LayoutUtil::is_dense_array(shape));
    self.parse_dense_literal(literal, shape)
  }

  fn parse_dense_literal<T>(&mut self, _literal: &Literal<T>, _shape: &Shape) -> bool
    where T: Clone + Default + PartialEq
  {
    unimplemented!()
  }

  fn create_instruction() {}

  fn set_value_in_literal() {}
  fn set_value_in_literal_helper() {}

  fn check_parsed_value_is_in_range() {}
  fn parse_operands() {}

  fn parse_attributes() {}
  fn parse_sub_attributes() {}
  fn parse_attribute_helper() {}

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
  fn parse_metadata(&mut self, _metadata: &OpMetadata) -> bool {
    let mut attrs: HashMap<String, AttrConfig> = HashMap::new();

    let op_type = String::new();
    let op_name = String::new();
    let source_file = String::new();
    let source_line = 0;
    //let profile_type = vec![];
    let _deduplicated_name = String::new();
    let _preserve_layout = false;

    attrs.insert("op_type".to_string(), 
      AttrConfig::new(false, AttrType::String, op_type.clone()));
    attrs.insert("op_name".to_string(), 
      AttrConfig::new(false, AttrType::String, op_name.clone()));
    attrs.insert("source_file".to_string(), 
      AttrConfig::new(false, AttrType::String, source_file.clone()));
    attrs.insert("source_line".to_string(), 
      AttrConfig::new(false, AttrType::Int32, source_line.to_string()));
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
    _tile_assignment_dimensions: &Vec<i64>,
    _iota_reshape_dims: &Vec<i64>,
    _iota_transpose_perm: &Vec<i64>,
    _devices: &Vec<i64>) -> bool
  {
    unimplemented!()    
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
    let tile_assignment_dimensions = vec![];
    let iota_reshape_dims = vec![];
    let iota_transpose_perm = vec![];
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
              &tile_assignment_dimensions,
              &iota_reshape_dims, &iota_transpose_perm, &devices)
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
      if !iota_transpose_perm.len() != iota_reshape_dims.len() {
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
            &subgroup_types, &metadata);
        }
      } else {
        if devices.len() <= 1 {
          assert!(false, "non-maximal shardings must have more than one desice assigned");
          return false;
        }
        let mut tiles: Vec<i64> = vec![];
        tiles.clone_from(&tile_assignment_dimensions);
        devices.clone_from(&tile_assignment_dimensions);
        if subgroup_types.is_empty() {
          *sharding = HloSharding::tile(TileAssignment::new_from_vec(&tiles), &metadata);
        } else {
          *sharding = HloSharding::subgroup(
            TileAssignment::new_from_vec(&tiles), &subgroup_types, &metadata);
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

  fn parse_domain() {}

  fn parse_dxd(&mut self, _name: &String, _result: &Vec<i64>) -> bool {
    unimplemented!()
  }

  fn parse_window_pad(&mut self, _pad: &Vec<Vec<i64>>) -> bool {
    unimplemented!()
  }

  fn parse_slice_ranges() {}
  fn parse_precision_list() {}
  fn parse_hlo_computation() {}
  fn parse_hlo_computation_list() {}
  fn parse_shape_list() {}
  fn parse_int64_list_list() {}

  fn parse_list(
    &mut self,
    start: &TokKind,
    end: &TokKind,
    delim: &TokKind,
    mut parse_and_add_item: Box<dyn FnMut()->bool>) -> bool
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
        if !parse_and_add_item() { return false; }
        if !self.eat_if_present(delim) { break; }
      }
    }

    let mut err_msg = "expects a list to end with ".to_string();
    err_msg.push_str(&tok_kind_to_string(end));
    self.parse_token(end, err_msg)
  }

  // param_list_to_shape ::= param_list '->' shape
  fn parse_param_list_to_shape(&mut self, shape: &mut Shape, shape_loc: &mut usize) -> bool {
    if !self.parse_param_list() ||
       ! self.parse_token(&TokKind::Arrow, "expects '->'".to_string())
    {
      return false;
    }
    *shape_loc = self.lexer.get_loc();
    self.parse_shape(shape)
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
        let mut shape = Shape::new();
        let mut name = String::new();
        if !self.parse_name(&mut name) || !self.parse_shape(&mut shape) {
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
    if self.lexer.get_kind() != TokKind::Ident && self.lexer.get_kind() != TokKind::Name {
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
    &mut self, _dimension_sizes: &mut Vec<i64>, _dynamic_dimensions: &mut Vec<bool>) -> bool
  {
    /*
    let parse_and_add_item = || -> bool {
      let mut i = 0;
      let mut is_dynamic = false;
      if self.lexer.get_kind() == TokKind::QuestionMark {
        i = Shape::UNBOUNDED_SIZE;
        is_dynamic = true;
        self.lexer.lex();
      } else {
        if self.lexer.get_kind() == TokKind::Leq {
          is_dynamic = true;
          self.lexer.lex();
        }
        if !self.parse_i64(&mut i) {
          return false;
        }
      }
      dimension_sizes.push(i);
      dynamic_dimensions.push(is_dynamic);
      true
    };
    self.parse_list(&TokKind::Lsquare, &TokKind::Rsquare,
      &TokKind::Comma, Box::new(parse_and_add_item))
      */
    false
  }

  fn parse_shape(&mut self, _shape: &mut Shape) -> bool {
    unimplemented!()
  }

  fn parse_layout(&mut self, _layout: &Layout) -> bool {
    unimplemented!()
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

  fn parse_dim_level_types() {}
  fn parse_tiles() {}
  fn parse_split_configs() {}

  // physical_shape
  //   ::= /*empty*/
  //   ::= 'P' '(' shape ')'
  fn parse_physical_shape(&mut self, physical_shape: &mut Shape) -> bool {
    let mut err_msg = "expects physical shape to start with ".to_string();
    err_msg.push_str(&tok_kind_to_string(&TokKind::Lparen));
    if !self.parse_token(&TokKind::Lparen, err_msg) {
      return false;
    }
    self.parse_shape(physical_shape);
    let mut err_msg = "expects physical shape to end with ".to_string();
    err_msg.push_str(&tok_kind_to_string(&TokKind::Rparen));
    if !self.parse_token(&TokKind::Rparen, err_msg) {
      return false;
    }
    true
  }

  fn parse_opcode() {}

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

  fn parse_padding_type() {}

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

  fn parse_unsigned_integer_type() {}

  fn parse_aliasing() {}
  fn parse_buffer_donor() {}
  fn parse_computation_layout() {}
  fn parse_instruction_output_operand_aliasing() {}
  fn parse_parse_custom_call_schedule() {}
  fn parse_custom_call_api_version() {}
  fn parse_sparsity_descriptor() {}
  fn parse_shape_index() {}

  fn can_be_shape() {}
  fn can_be_param_list_to_shape() {}

  // Logs the currentparsing line and the given message. Always return false.
  fn token_error(&self, msg: String) -> bool {
    self.error(self.lexer.get_loc(), msg)
  }

  fn error(&self, _loc: usize, msg: String) -> bool {
    println!("Error: {:?}", msg);
    assert!(false);
    false
  }

  fn eat_if_present(&mut self, kind: &TokKind) -> bool {
    if self.lexer.get_kind() != *kind {
      return false;
    }
    self.lexer.lex(0);
    true
  }

  fn add_instruction() {}
  fn add_computation() {}
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_sharding() {
    let original = "{maximal device=42}".to_string();
    let sharding = parse_sharding(original.clone());
    assert!(sharding.is_ok());
    assert_eq!(sharding.unwrap().to_string(false), original);
  }
}