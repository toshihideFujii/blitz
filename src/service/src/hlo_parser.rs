#![allow(dead_code)]

use std::collections::HashMap;

use common::{
  blitz_data::{Algorithm, ConvolutionDimensionNumbers, FftType, FrontendAttributes, OpMetadata, OpShardingType, PaddingConfig, ParameterReplication, Precision, PrimitiveType, RandomAlgorithm, RandomDistribution, ReplicaGroup, StatisticsViz},
    comparison_util::{string_to_comparison_direction, string_to_comparison_type, ComparisonDirection, ComparisonType}, layout::Layout, layout_util::LayoutUtil, literal::Literal, literal_util::LiteralUtil, shape::Shape, shape_util::ShapeUtil};
use hlo::{hlo_computation::HloComputation, hlo_domain_metadata::DomainMetadata, hlo_instruction::{self, FusionKind, HloInstruction}, hlo_module::HloModule, hlo_module_config::HloModuleConfig, hlo_opcode::HloOpcode, hlo_sharding::HloSharding};
use num::complex::Complex64;

use crate::hlo_lexer::{tok_kind_to_string, HloLexer, TokKind};

// Given a string in the HloModule::to_string() format, parses the string and
// creates a HloModule with the given config.
pub fn parse_and_return_unverified_module(
  _str: String, _config: &HloModuleConfig) -> Result<HloModule, String>
{
  unimplemented!()
}

// Parses sharding from str.
pub fn parse_sharding(_str: String) -> Result<HloSharding, String> {
  unimplemented!()
}

// Parses frontend attributes from str.
pub fn parse_frontend_attributes(_str: String) -> Result<FrontendAttributes, String> {
  unimplemented!()
}

// Parses statistics viz from str.
pub fn parse_statistics_viz(_str: String) -> Result<StatisticsViz, String> {
  unimplemented!()
}

// Parses parameter replication from str.
pub fn parse_parameter_replication(_str: String) -> Result<Vec<bool>, String> {
  unimplemented!()
}

// Parses the result of window_util::to_string().
pub fn parse_window(_str: String) {}

// Parses the result of comvolution_dimension_numbers_to_string().
pub fn parse_convolution_demension_numbers(_str: String) {}

// Parses the result of padding_config_to_string().
pub fn parse_padding_config(_str: String) {}

// Parses and returns a Shape::to_string-format string.
pub fn parse_shape(_str: String) -> Result<Shape, String> {
  unimplemented!()
}

// Parses and returns a Layout::to_string-format string.
pub fn parse_layout(_str: String) -> Result<Layout, String> {
  unimplemented!()
}

pub fn parse_replica_groups_only() {}

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
  scoped_name_tables: Vec<HashMap<String, (HloInstruction, usize)>>,
  computation_pool: HashMap<String, (HloComputation, usize)>,
  computations: Vec<HloComputation>,
  error: Vec<String>
}

impl HloParser {
  pub fn new() {}

  // Runs the parser and constructs the resulting HLO in the given (empty)
  // HloModule. Returns the error status in case an error occurred.
  pub fn run(&mut self, module: &HloModule) -> Result<(), String> {
    self.lexer.lex();
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
    self.lexer.lex();
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
    self.lexer.lex();
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
    //self.lexer.lex();
    unimplemented!()
  }

  pub fn parse_frontend_attributes_only(&mut self) -> Result<FrontendAttributes, String> {
    self.lexer.lex();
    let attributes = FrontendAttributes::new();
    if !self.parse_frontend_attributes(&attributes) {
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
    self.lexer.lex();
    let statistics_viz = StatisticsViz::new();
    if !self.parse_statistics_viz(&statistics_viz) {
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
    self.lexer.lex();
    let mut parameter_replication = ParameterReplication::new();
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
    self.lexer.lex();
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

  pub fn parse_window_only(&self) {}

  pub fn parse_convolution_demension_numbers_only(
    &mut self) -> Result<ConvolutionDimensionNumbers, String>
  {
    self.lexer.lex();
    let dnums = ConvolutionDimensionNumbers::new();
    if !self.parse_convolution_demension_numbers(&dnums) {
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
    self.lexer.lex();
    let padding_config = PaddingConfig::new();
    if !self.parse_padding_config(&padding_config) {
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

  pub fn parse_replica_groups_only(&self, _replica_groups: Vec<ReplicaGroup>) -> bool {
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

  fn parse_window() {}

  fn parse_convolution_demension_numbers(&mut self, _dnums: &ConvolutionDimensionNumbers) -> bool {
    unimplemented!()
  }

  fn parse_padding_config(&mut self, _padding_config: &PaddingConfig) -> bool {
    unimplemented!()
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
  fn parse_single_or_list_metadata(&mut self, _metadata: Vec<OpMetadata>) -> bool {
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
        self.lexer.lex();
        return true;
      },
      TokKind::Replicated => {
        *t = OpShardingType::Replicated;
        self.lexer.lex();
        return true;
      },
      TokKind::Manual => {
        *t = OpShardingType::Manual;
        self.lexer.lex();
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

  fn parse_sharding() {}

  // frontend_attributes ::= '{' attributes '}'
  // attributes
  //   ::= /*empty*/
  //   ::= attribute '=' value (',' attribute '=' value)*
  fn parse_frontend_attributes(
    &mut self, _frontend_attributes: &FrontendAttributes) -> bool
  {
    if self.parse_token(&TokKind::Lbrace,
      "expected '{' to start frontend attributes".to_string()) {
      return false;
    }
    if self.lexer.get_kind() == TokKind::Rbrace {
      // empty
    } else {
      loop {
        let mut attribute = "".to_string();
        if !self.parse_attribute_name(&mut attribute) { return false; }
        if self.lexer.get_kind() != TokKind::String { return false; }
        // TODO
        // frontend_attributes.mutable_map
        self.lexer.lex();
        if !self.eat_if_present(&TokKind::Comma) { break; }
      }
    }
    self.parse_token(&TokKind::Rbrace,
      "expects '}' at the end of frontend attributes".to_string())
  }

  fn parse_statistics_viz(&mut self, _statistics_viz: &StatisticsViz) -> bool {
    unimplemented!()
  }

  fn parse_single_sharding() {}

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
        self.lexer.lex();
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
      self.lexer.lex();
      return true;
    } else if self.lexer.get_kind() == TokKind::False {
      boolean_list.push(false);
      self.lexer.lex();
      return true;
    }
    false
  }

  //fn parse_replica_groups_only() {}

  fn parse_domain() {}
  fn parse_dxd() {}
  fn parse_window_pad() {}
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
    self.lexer.lex();
    true
  }

  fn parse_attribute_name(&mut self, result: &mut String) -> bool {
    if self.lexer.get_kind() != TokKind::AttributeName {
      return self.token_error("expects attribute name".to_string());
    }
    *result = self.lexer.get_str_val();
    self.lexer.lex();
    true
  }

  fn parse_string(&mut self, result: &mut String) -> bool {
    println!("parse_string");
    if self.lexer.get_kind() != TokKind::String {
      return self.token_error("expects string".to_string());
    }
    *result = self.lexer.get_str_val();
    self.lexer.lex();
    true
  }

  fn parse_json_dict(&mut self, result: &mut String) -> bool {
    println!("parse_json_dict");
    if self.lexer.lex_json_dict() != TokKind::String {
      return self.token_error("expects JSON dict".to_string());
    }
    *result = self.lexer.get_str_val();
    self.lexer.lex();
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
    self.lexer.lex();
    true
  }

  fn parse_padding_type() {}

  fn parse_primitive_type(&mut self, result: &mut PrimitiveType) -> bool {
    if self.lexer.get_kind() != TokKind::PrimitiveType {
      return self.token_error("expected primitive type".to_string());
    }
    *result = self.lexer.get_primitive_type_val();
    self.lexer.lex();
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
    self.lexer.lex();
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
    self.lexer.lex();
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
    self.lexer.lex();
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
    self.lexer.lex();
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
    self.lexer.lex();
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
    self.lexer.lex();
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
    self.lexer.lex();
    true
  }

  fn parse_i64(&mut self, result: &mut i64) -> bool {
    println!("parse_int64");
    if self.lexer.get_kind() != TokKind::Int {
      return self.token_error("expects integer".to_string());
    }
    *result = self.lexer.get_i64_val();
    self.lexer.lex();
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
    self.lexer.lex();
    true
  }

  fn parse_complex(&mut self, result: &mut Complex64) -> bool {
    if self.lexer.get_kind() != TokKind::Lparen {
      return self.token_error("expects '(' before complex number".to_string());
    }
    self.lexer.lex();

    let mut real = 0.0;
    let loc = self.lexer.get_loc();
    if !self.parse_double(&mut real) {
      return self.error(loc,
        "expect floating-point value for real part of complex number".to_string());
    }

    if self.lexer.get_kind() != TokKind::Comma {
      return self.token_error("expect comma after real part of complex literal".to_string());
    }
    self.lexer.lex();

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
    self.lexer.lex();
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
    self.lexer.lex();
    true
  }

  fn parse_token(&mut self, kind: &TokKind, msg: String) -> bool {
    println!("parse_token {:?} {:?}", kind, msg);
    if self.lexer.get_kind() != *kind {
      return self.token_error(msg);
    }
    self.lexer.lex();
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

  fn error(&self, _loc: usize, _msg: String) -> bool {
    unimplemented!()
  }

  fn eat_if_present(&mut self, kind: &TokKind) -> bool {
    if self.lexer.get_kind() != *kind {
      return false;
    }
    self.lexer.lex();
    true
  }

  fn add_instruction() {}
  fn add_computation() {}
}