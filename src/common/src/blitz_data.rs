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

// Debugging options for Blitz.
pub struct DebugOptions {

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