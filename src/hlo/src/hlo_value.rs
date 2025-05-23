#![allow(dead_code)]

use std::cmp::Ordering;

use common::{shape::Shape, shape_tree::ShapeTree, shape_util::ShapeUtil};

use crate::{
  buffer_value::BufferValue, hlo_computation::HloComputation, hlo_instruction::HloInstruction,
};

// abstraction which identifies a specific point in the Blitz graph.
// an HloPosition specifies a shapeIndex within the output of a specific
// instruction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HloPosition {
  pub instruction: HloInstruction,
  pub index: Vec<i64>
}

impl HloPosition {
  pub fn new() {}

  // Returns the shape at this position.
  pub fn shape(&self) -> &Shape {
    unimplemented!()
  }

  pub fn to_string() {}
}

impl PartialOrd for HloPosition {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.instruction.unique_id() < other.instruction.unique_id() {
      Some(Ordering::Less)
    } else if self.instruction.unique_id() == other.instruction.unique_id() {
      Some(Ordering::Equal)
    } else {
      Some(Ordering::Greater)
    }
  }
}

impl Ord for HloPosition {
  fn cmp(&self, other: &Self) -> Ordering {
    if self.instruction.unique_id() < other.instruction.unique_id() {
      Ordering::Less
    } else if self.instruction.unique_id() == other.instruction.unique_id() {
      Ordering::Equal
    } else {
      Ordering::Greater
    }
  }
}

// Defines a single use of an HLO value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HloUse {
  pub instruction: HloInstruction,
  pub operand_number: i64,
  pub operand_index_vec: Vec<i64>
}

impl HloUse {
  pub fn new(
    instruction: HloInstruction,
    operand_number: i64,
    operand_index: Vec<i64>) -> Self
  {
    HloUse {
      instruction: instruction,
      operand_number: operand_number,
      operand_index_vec: operand_index
    }
  }

  pub fn to_string(&self) -> String {
    unimplemented!()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HloValue {
  buffer_value: BufferValue,
  positions: Vec<HloPosition>,
  uses: Vec<HloUse>,
  is_phi: bool,
  live_out_of_module: bool
}

impl HloValue {
  pub fn default() -> Self {
    HloValue {
      buffer_value: BufferValue::new(),
      positions: Vec::new(),
      uses: Vec::new(),
      is_phi: false,
      live_out_of_module: false
    }
  }

  // Construct an HloValue defined by 'instruction' at shape index 'index'. If
  // is_phi is true, then this value is a phi value, for example, at the
  // parameter of a while body computation. Phi values are only used in the SSA
  // dataflow analysis (HloDataflowAnalysis::ssa_form_ is true).
  pub fn new(
    _id: i64,
    _instruction: &HloInstruction,
    _index: &Vec<i64>,
    _is_phi: bool) -> Self
  {
    /*
    let instance =HloValue {
      buffer_value: BufferValue::new(),
      positions: Vec::new(),
      uses: Vec::new(),
      is_phi: is_phi,
      live_out_of_module: false
    };
    instance
    */
    unimplemented!()
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
  pub fn defining_index(&self) -> &Vec<i64> {
    &self.defining_position().index
  }

  pub fn index(&self) -> &Vec<i64> {
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

  pub fn to_short_string(&self) -> String {
    unimplemented!()
  }

  pub fn to_string(&self) -> String {
    unimplemented!()
  }

  // ----- BufferValue methods -----
  pub fn id(&self) -> i64 {
    self.buffer_value.id()
  }

  pub fn color(&self) -> i64 {
    self.buffer_value.color()
  }
}

impl PartialOrd for HloValue {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.id() < other.id() {
      return Some(Ordering::Less);
    } else if self.id() == other.id() {
      return Some(Ordering::Equal);
    } else {
      return Some(Ordering::Greater);
    }
  }
}

impl Ord for HloValue {
  fn cmp(&self, other: &Self) -> Ordering {
    if self.id() < other.id() {
      return Ordering::Less;
    } else if self.id() == other.id() {
      return Ordering::Equal;
    } else {
      return Ordering::Greater;
    }
  }
}

// A class representing the possible set of HloValues at a particular point
// (shape index in the output of an instruction) in the Blitz graph.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct HloValueSet {
  values: Vec<HloValue>
}

impl HloValueSet {
  pub fn default() -> Self {
    HloValueSet { values: Vec::new() }
  }

  pub fn new(values: Vec<HloValue>) -> Self {
    let mut instance = HloValueSet { values: values };
    instance.sort_and_uniquify_values();
    instance
  }

  // Sets this value set to the union of the given value sets.
  // Returns whether this value set changed.
  pub fn assign_union_of(&mut self, inputs: Vec<HloValueSet>) -> bool {
    let mut original: Vec<HloValue> = Vec::new();
    original.clone_from_slice(&self.values);

    for input in inputs {
      for value in input.values {
        self.values.push(value);
      }
    }
    self.sort_and_uniquify_values();
    if self.values != original { true } else { false }
  }

  // Return the vector of HloValues in the set.
  // Values in the vector are unique and stably sorted by value id.
  pub fn values(&self) -> &Vec<HloValue> {
    &self.values
  }

  // Adds the value to the set.
  // Returns true if the value was added and didn't already exist in the set.
  pub fn add_value(&mut self, value: HloValue) -> bool {
    if self.values.contains(&value) { return false; }
    self.values.push(value);
    self.sort_and_uniquify_values();
    true
  }

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

  pub fn to_string(&self) -> String {
    let mut result = "HloValueSet: ".to_string();
    for value in &self.values {
      result.push_str(&value.to_short_string());
      if Some(value) != self.values.last() {
        result.push_str(", ");
      }
    }
    result
  }

  // Sorts value and removes duplicates.
  fn sort_and_uniquify_values(&mut self) {
    self.values.sort();
    self.values.dedup();
  }
}

// A class collecting the HloValues which might be contained in the output of
// an HLO instruction.
#[derive(Clone, PartialEq)]
pub struct InstructionValueSet {
  pub shape_tree: ShapeTree<HloValueSet>
}

impl InstructionValueSet {
  pub fn new(mut shape: Shape) -> Self {
    InstructionValueSet { shape_tree: ShapeTree::new(&mut shape) }
  }

  // Sets this value set to the union of the given value sets.
  // Returns whether this value set changed.
  pub fn assign_union_of(&mut self, inputs: Vec<InstructionValueSet>) -> bool {
    debug_assert!(inputs.len() > 0);
    for _i in 1..inputs.len() {
      debug_assert!(ShapeUtil::compatible(
        inputs[0].shape(), inputs[1].shape()));
    }
    let mut changed = false;
    for pair in self.mutable_nodes() {
      let index = vec![pair.0 as i64];
      let value_set = &mut pair.1;
      let mut input_value_sets = Vec::new();
      for input in &inputs {
        input_value_sets.push(input.element(&index).clone());
      }
      changed |= value_set.assign_union_of(input_value_sets);
    }
    changed
  }

  // Return true if any value sets for any subshape element is not a singleton.
  pub fn is_ambiguous(&self) -> bool {
    let mut ambiguous = false;
    for node in self.nodes() {
      ambiguous |= node.1.values().len() > 1;
    }
    ambiguous
  }

  pub fn shape(&self) -> &Shape {
    self.shape_tree.shape()
  }

  pub fn element(&self, _index: &Vec<i64>) -> &HloValueSet {
    //self.shape_tree.element(index)
    unimplemented!()
  }

  pub fn mutable_element(&mut self, _index: &Vec<i64>) -> &mut HloValueSet {
    //self.shape_tree.mutable_element(index)
    unimplemented!()
  }

  pub fn find(&self, _index: usize) -> &(usize, HloValueSet) {
    //self.shape_tree.find(index)
    unimplemented!()
  }

  pub fn nodes(&self) -> &Vec<(usize, HloValueSet)> {
    //self.shape_tree.nodes()
    unimplemented!()
  }

  pub fn mutable_nodes(&mut self) -> &mut Vec<(usize, HloValueSet)> {
    //self.shape_tree.mutable_nodes()
    unimplemented!()
  }

  pub fn to_string(&mut self) -> String {
    let mut out = "InstructionValueSet(".to_string();
    out.push_str(ShapeUtil::human_string(self.shape()).as_str());
    out.push_str(")\n");

    let _func = |index: usize, value_set: &mut HloValueSet| {
      out.push_str("  ");
      out.push_str(&index.to_string());
      out.push_str(" : ");
      out.push_str(&value_set.to_string());
      out.push('\n');
    };
    //self.shape_tree.for_each_mutable_element(&mut func);
    out
  }
}