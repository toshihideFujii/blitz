#![allow(dead_code)]

pub struct HloPosition {}

impl HloPosition {
  pub fn new() {}
  pub fn shape() {}
  pub fn to_string() {}
}

pub struct HloUse {}

impl HloUse {
  pub fn new() {}
  pub fn to_string() {}
}

pub struct HloValue {}

impl HloValue {
  pub fn new() {}
  pub fn id_less_than() {}
  pub fn set_positions() {}
  pub fn is_phi() {}
  pub fn defining_position() {}
  pub fn defining_instruction() {}
  pub fn instruction() {}
  pub fn defining_index() {}
  pub fn index() {}
  pub fn shape() {}
  pub fn positions() {}
  pub fn get_uses() {}
  pub fn is_root_of() {}
  pub fn live_out_of_module() {}
  pub fn to_short_string() {}
  pub fn to_string() {}
}

pub struct HloValueSet {}

impl HloValueSet {
  pub fn new() {}
  pub fn assign_union_of() {}
  pub fn values() {}
  pub fn add_value() {}
  pub fn clear() {}
  pub fn take_values() {}
  pub fn get_unique_value() {}
  pub fn to_string() {}
}

pub struct InstructionValueSet {}

impl InstructionValueSet {
  pub fn new() {}
  pub fn assign_union_of() {}
  pub fn is_ambiguous() {}
  pub fn to_string() {}
}