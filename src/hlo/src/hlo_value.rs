#![allow(dead_code)]

use common::shape::Shape;

use crate::{
  hlo_computation::HloComputation,
  hlo_instruction::HloInstruction,
  buffer_value::BufferValue
};

// abstraction which identifies a specific point in the Blitz graph.
// an HloPosition specifies a shapeIndex within the output of a specific
// instruction.
#[derive(Debug, PartialEq)]
pub struct HloPosition {
  instruction: HloInstruction,
  index: usize
}

impl HloPosition {
  pub fn new() {}

  // Returns the shape at this position.
  pub fn shape(&self) -> &Shape {
    unimplemented!()
  }

  pub fn to_string() {}
}

// Defines a single use of an HLO value.
pub struct HloUse {
  instruction: HloInstruction,
  operand_number: i64,
  operand_index: usize
}

impl HloUse {
  pub fn new() {}
  pub fn to_string() {}
}

pub struct HloValue {
  buffer_value: BufferValue,
  positions: Vec<HloPosition>,
  uses: Vec<HloUse>,
  is_phi: bool,
  live_out_of_module: bool
}

impl HloValue {
  pub fn new() -> Self {
    HloValue {
      buffer_value: BufferValue::new(),
      positions: Vec::new(),
      uses: Vec::new(),
      is_phi: false,
      live_out_of_module: false
    }
  }

  // Predicate comparing HloValues by increasing id, for std::sort.
  pub fn id_less_than(a: &HloValue, b: HloValue) -> bool {
    a.id() < b.id()
  }

  // Sets the positions in the module at which the HloValue appears.
  pub fn set_positions(&mut self, mut positions: Vec<HloPosition>) {
    for pos_a in &positions {
      assert_ne!(pos_a, self.defining_position());
      for pos_b in &positions {
        assert_ne!(pos_a, pos_b)
      }
    }
    self.positions.append(&mut positions);
    self.live_out_of_module |= self.is_root_of(
      self
      .defining_instruction()
      .get_module().as_ref().unwrap()
      .entry_computation().as_ref().unwrap());
  }

  // Returns whether this value is a phi value.
  pub fn is_phi(&self) -> bool {
    self.is_phi
  }

  // Return the position where this value is defined.
  pub fn defining_position(&self) -> &HloPosition {
    self.positions.get(0).unwrap()
  }

  // Return the instruction which defines this HloValue.
  pub fn defining_instruction(&self) -> &HloInstruction {
    &self.defining_position().instruction
  }

  pub fn instruction(&self) -> &HloInstruction {
    self.defining_instruction()
  }

  // Return the shape index at which this HloValue is defined in the output of
  // its defining instruction.
  pub fn defining_index(&self) -> usize {
    self.defining_position().index
  }

  pub fn index(&self) -> usize {
    self.defining_index()
  }

  // Return the shape of this HloValue.
  pub fn shape(&self) -> &Shape {
    self.defining_position().shape()
  }

  // Return all positions of the HloValue in the module.
  pub fn positions(&self) -> &Vec<HloPosition> {
    &self.positions
  }

  // Return all uses of the HloValue.
  pub fn get_uses(&self) -> &Vec<HloUse> {
    &self.uses
  }

  // Returns true if this has a position that is the root of the given
  // computation.
  pub fn is_root_of(&self, computation: &HloComputation) -> bool {
    for pos in &self.positions {
      if pos.instruction.is_root() && pos.instruction.parent() == computation {
        return true;
      }
    }
    false
  }

  // Get whether this HloValue is live out of the module.
  pub fn live_out_of_module(&self) -> bool {
    self.live_out_of_module
  }

  pub fn to_short_string() {}
  pub fn to_string() {}

  // ----- BufferValue methods -----
  pub fn id(&self) -> i64 {
    self.buffer_value.id()
  }
}

// A class representing the possible set of HloValues at a particular point
// (shape index in the output of an instruction) in the Blitz graph.
pub struct HloValueSet {
  values: Vec<HloValue>
}

impl HloValueSet {
  pub fn new() {}
  pub fn assign_union_of() {}

  // Return the vector of HloValues in the set.
  // Values in the vector are unique and stably sorted by value id.
  pub fn values(&self) -> &Vec<HloValue> {
    &self.values
  }

  pub fn add_value() {}

  // Clear all values from the set.
  pub fn clear(&mut self) {
    self.values.clear();
  }

  //pub fn take_values(&self) -> Vec<HloValue> {
    //self.values
  //}

  // Return the unique HLO value in the set.
  pub fn get_unique_value(&self) -> &HloValue {
    assert!(!self.values.is_empty());
    self.values.get(0).unwrap()
  }

  pub fn to_string() {}
}

pub struct InstructionValueSet {}

impl InstructionValueSet {
  pub fn new() {}
  pub fn assign_union_of() {}
  pub fn is_ambiguous() {}
  pub fn to_string() {}
}