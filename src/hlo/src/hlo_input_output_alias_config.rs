#![allow(dead_code)]

use common::{shape::Shape, shape_tree::ShapeTree};

use crate::hlo_module::HloModule;

// The kind of aliases which can be set. A kMayAlias is one setup at
// compilation time by the user, and has to be respected. A kMustAlias one
// might be setup by the compiler, if it decides it is convenient to do so.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum AliasKind {
  May,
  #[default]
  Must,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Alias {
  parameter_number: i64,
  parameter_index: Vec<i64>,
  kind: AliasKind
}

impl Alias {
  pub fn new(
    parameter_number: i64,
    parameter_index: Vec<i64>,
    kind: AliasKind) -> Self
  {
    Alias {
      parameter_number: parameter_number,
      parameter_index: parameter_index,
      kind: kind
    }
  }

  pub fn must_alias(&self) -> bool {
    self.kind == AliasKind::Must
  }

  pub fn to_string(&self) -> String {
    let mut result = "(".to_string();
    result.push_str(&self.parameter_number.to_string());
    result.push_str(", ");
    
    result.push_str("[");
    let mut counter = 0;
    for i in &self.parameter_index {
      if counter == self.parameter_index.len() { break; }
      result.push_str(&i.to_string());
      result.push_str(", ");
      counter += 1;
    }
    result.push_str("], ");

    if self.kind == AliasKind::Must {
      result.push_str("must-alias");
    } else {
      result.push_str("may-alias");
    }
    result.push_str(")");
    result
  }
}

// This class specifies the alias map from output index to parameter number and
// parameter index in the entry computation.
#[derive(PartialEq)]
pub struct HloInputOutputAliasConfig {
  alias: ShapeTree<Alias>
}

impl HloInputOutputAliasConfig {
  pub fn new(mut output_shape: Shape) -> Self {
    HloInputOutputAliasConfig {
      alias: ShapeTree::new(&mut output_shape)
    }
  }

  // Sets up alias config from `output_index` to `param_index` at
  // `param_number`.
  pub fn setup_alias(
    &self,
    _output_index: &Vec<i64>,
    _param_number: i64,
    _param_index: &Vec<i64>,
    _must_alias: AliasKind)
  {
    unimplemented!()
  }

  // Returns true if the given parameter is aliased with one of the output
  // buffers.
  pub fn parameter_has_alias(&self, _param_number: i64, _param_index: &Vec<i64>) -> bool {
    unimplemented!()
  }

  // Checks whether the provided output index has already been aliased.
  pub fn output_has_alias(_output_index: &Vec<i64>) -> bool {
    unimplemented!()
  }

  // Returns the number of parameter and index of the parameter buffer that the
  // given output buffer index is aliased with. A nullopt is returned if there
  // is no parameter is aliased with the specific output.
  pub fn get_aliased_parameter(_output_index: &Vec<i64>) -> Option<Alias> {
    unimplemented!()
  }

  // Returns if the parameter at the given parameter number and parameter
  // index must-alias with an output.
  pub fn parameter_must_alias(_param_number: i64, _param_index: &Vec<i64>) -> bool {
    unimplemented!()
  }

  // Iterates through each aliased output and input.  
  pub fn for_each_alias<F>(&self, _func: F)
    where F: Fn(&Vec<i64>, &Alias)
  {
    unimplemented!()   
  }

  // Verifies that the given config is valid for the given module.
  // Specifically, the config's input and output should be in-bound and size ofF
  // the aliased buffers should match.
  pub fn verify<F>(&self, _module: &HloModule, _func: F) -> Result<(), String>
    where F: Fn(&Shape) -> i64
  {
    unimplemented!()
  }

  pub fn for_each_alias_with_status(&self) {
    unimplemented!()
  }

  // Returns the shape of the output of the alias config.
  pub fn shape(&self) -> &Shape {
    unimplemented!()
  }

  pub fn to_string(&self) -> String {
    unimplemented!()
  }

  pub fn to_short_string(&self) -> String {
    unimplemented!()
  }
}

#[derive(PartialEq)]
pub struct HloBufferDonorConfig {}

impl HloBufferDonorConfig {
    
}