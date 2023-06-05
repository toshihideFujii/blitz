#![allow(dead_code)]

struct MemInstructionInfo {}
impl MemInstructionInfo {
  pub fn new() {}
  pub fn is_unordered() {}
}

struct HardwareLoopinfo {}
impl HardwareLoopinfo {
  pub fn new() {}
  pub fn is_hardware_loop_candidate() {}
  pub fn can_analyze() {}
}

struct IntrinsicCostAttributes {}
impl IntrinsicCostAttributes {
  pub fn new() {}
  pub fn get_id() {}
  pub fn get_inst() {}
  pub fn get_return_type() {}
  pub fn get_flags() {}
  pub fn get_scalarization_cost() {}
  pub fn get_args() {}
  pub fn get_arg_types() {}
  pub fn is_type_based_only() {}
  pub fn skip_scalarization_cost() {}
}