#![allow(dead_code)]

use std::{collections::HashMap, ops::BitOrAssign};

use crate::{hlo_computation::HloComputation, hlo_instruction::HloInstruction};

#[derive(Debug, Clone, PartialEq)]
struct BitSet {
  size: usize,
  vector: Vec<u64>
}

impl BitSet {
  const BITS: usize = 64;

  pub fn new(size: usize) -> Self {
    BitSet {
      size: size,
      vector: vec![0; (size + BitSet::BITS - 1) / BitSet::BITS]
    }
  }

  // Returns the bit at the given index.
  pub fn get(&self, index: usize) -> bool {
    assert!(index < self.size);
    self.vector[index / BitSet::BITS] & (1 << (index % BitSet::BITS)) != 0
  }

  // Sets the bit at the given index.
  pub fn set(&mut self, index: usize) {
    assert!(index < self.size);
    self.vector[index / BitSet::BITS] |= 1 << (index %BitSet::BITS);
  }

  // Sets the bitvector to all zeros.
  pub fn set_to_zero(&mut self) {
    self.vector.fill(0);
  }
}

// Sets this bit-set to union of this bit-set and 'rhs'.
impl BitOrAssign for BitSet {
  fn bitor_assign(&mut self, rhs: Self) {
    for i in 0..self.vector.len() {
      self.vector[i] |= rhs.vector[i];
    }
  }
}

// A class for representing reachability between HloInstructions.
pub struct HloReachabilityMap {
  indices: HashMap<(i64, i64), usize>,
  bit_sets: Vec<BitSet>,
  tmp_bit_set: BitSet
}

impl HloReachabilityMap {
  pub fn new(instructions: Vec<HloInstruction>) -> Self {
    let mut instance = HloReachabilityMap {
      indices: HashMap::new(),
      bit_sets: vec![BitSet::new(instructions.len()); instructions.len()],
      tmp_bit_set: BitSet::new(0)
    };
    for i in 0..instructions.len() {
      instance.bit_sets[i].set(i);
      instance.indices.insert(
        HloReachabilityMap::get_key(instructions.get(i).as_ref().unwrap()),
        i);
    }
    instance
  }

  pub fn build(_computation: &HloComputation) -> Self {
    unimplemented!()
  }

  pub fn build_with_restrictions() {}

  // Set the reachability set of 'instruction' to the union of the reachability
  // sets of 'inputs'.
  pub fn set_reachability_to_union(
    &mut self,
    inputs: Vec<HloInstruction>,
    instruction: &HloInstruction) -> bool
  {
    let index = self.get_index(instruction);
    self.tmp_bit_set = self.bit_sets[index].clone();
    self.set_reachability_to_union_helper(inputs, index);
    self.bit_sets[index] != self.tmp_bit_set
  }

  // As above, but faster because it does not check if the reachability changed.
  pub fn fast_set_rachability_to_union(
    &mut self,
    inputs: Vec<HloInstruction>,
    instruction: &HloInstruction)
  {
    self.set_reachability_to_union_helper(inputs, self.get_index(instruction));
  }

  // As above, but use Index instead if it's already looked up which is even
  // faster since no hash map lookup will occur.
  pub fn fast_set_rachability_to_union_by_index(
    &mut self,
    _input_indices: &Vec<usize>,
    _index: usize)
  {
    unimplemented!()    
  }

  pub fn get_index(&self, instruction: &HloInstruction) -> usize {
    let keys = self.indices.keys();
    let mut index = 0;
    for i in keys {
      if *i == HloReachabilityMap::get_key(instruction) { return index; }
      index += 1;
    }
    unreachable!("Key is not exist.");
  }

  // Sets entry so that is_reachable(a, b) will reeturn true.
  pub fn set_reachable(&mut self, a: &HloInstruction, b: &HloInstruction) {
    self.set_reachable_by_index(self.get_index(a), self.get_index(b))
  }

  pub fn set_reachable_by_index(&mut self, a: usize, b: usize) {
    self.bit_sets[b].set(a);
  }

  pub fn update_reachability_through_instruction() {}

  // Returns true if 'b' is reachable from 'a'.
  pub fn is_reachable(&self, a: &HloInstruction, b: &HloInstruction) -> bool {
    self.is_reachable_by_index(self.get_index(a), self.get_index(b))
  }

  pub fn is_reachable_by_index(&self, a: usize, b: usize) -> bool {
    self.bit_sets[b].get(a)
  }

  // Returns true if 'b' is reachable from 'a' or 'a' is reachable from 'b'.
  pub fn is_connected(&self, a: &HloInstruction, b: &HloInstruction) -> bool {
    self.is_connected_by_index(self.get_index(a), self.get_index(b))
  }

  pub fn is_connected_by_index(&self, a: usize, b: usize) -> bool {
    self.is_reachable_by_index(a, b) || self.is_reachable_by_index(b, a)
  }

  // Checks if an instruction is in the Reachability map.
  pub fn is_present(&self, instruction: &HloInstruction) -> bool {
    self.indices.contains_key(&HloReachabilityMap::get_key(instruction))
  }

  // Replace the instruction 'original' with 'replacement' in the reachability map.
  pub fn replace(&mut self, original: &HloInstruction, replacement: &HloInstruction) {
    if HloReachabilityMap::get_key(original) !=
       HloReachabilityMap::get_key(replacement)
    {
      self.indices.insert(HloReachabilityMap::get_key(replacement),
        self.get_index(original));
    }
  }

  fn get_key(instruction: &HloInstruction) -> (i64, i64) {
    (instruction.get_module().as_ref().unwrap().unique_id(), instruction.unique_id())
  }

  fn set_reachability_to_union_helper(
    &mut self,
    inputs: Vec<HloInstruction>,
    index: usize)
  {
    let mut input_indices = Vec::new();
    input_indices.reserve(inputs.len());
    for input in &inputs {
      input_indices.push(self.get_index(input));
    }
    self.set_reachability_to_union_helper_by_indices(input_indices, index)
  }

  fn set_reachability_to_union_helper_by_indices(
    &mut self,
    input_indices: Vec<usize>,
    index: usize)
  {
    let mut found = false;
    for i in &input_indices {
      if *i == index {
        found = true;
      }    
    }
    if !found { self.bit_sets[index].set_to_zero(); }
    self.bit_sets[index].set(index);

    for i in &input_indices {
      if *i != index {
        //self.bit_sets[index] |= self.bit_sets[*i];
      }
    }
  }

  pub fn reset(&self) {
    unimplemented!()
  }
}