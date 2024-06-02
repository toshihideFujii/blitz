#![allow(dead_code)]

use std::collections::HashMap;

use crate::{hlo_computation::HloComputation, hlo_instruction::HloInstruction, hlo_module::HloModule};

pub struct HloInstructionSequence {
  instruction_sequence: Vec<HloInstruction>,
  id_sequence: Vec<i64>
}

impl HloInstructionSequence {
  pub fn new() -> Self {
    HloInstructionSequence {
      instruction_sequence: Vec::new(),
      id_sequence: Vec::new() 
    }
  }

  // Adds the instruction to the end of the sequence.
  pub fn push_pack(&mut self, instruction: HloInstruction) {
    let id = instruction.unique_id();
    self.instruction_sequence.push(instruction);
    self.id_sequence.push(id);
  }

  // Remove the instruction from the sequence.
  pub fn remove_instruction(&mut self, instruction: &HloInstruction) {
    let mut inst_found = false;
    let mut id_found = false;
    let mut inst_index = 0;
    let mut id_index = 0;
    for i in 0..self.instruction_sequence.len() {
      if self.instruction_sequence.get(i).unwrap().unique_id() ==
        instruction.unique_id() { inst_found = true; inst_index = i; break; }
    }
    if inst_found {
      for i in 0..self.id_sequence.len() {
        if *self.id_sequence.get(i).unwrap() == instruction.unique_id() {
          id_found = true; id_index = i; break;
        }
      }
      if inst_found && id_found {
        self.instruction_sequence.remove(inst_index);
        self.id_sequence.remove(id_index);
      }
    }
  }

  // Replaces the old instruction with the new instruction in the sequence.
  pub fn replace_instruction(&mut self, old: &HloInstruction, new: HloInstruction) {
    let mut inst_found = false;
    let mut id_found = false;
    let mut inst_index = 0;
    let mut id_index = 0;
    for i in 0..self.instruction_sequence.len() {
      if self.instruction_sequence.get(i).unwrap().unique_id() ==
        old.unique_id() { inst_found = true; inst_index = i; break; }
    }
    if inst_found {
      for i in 0..self.id_sequence.len() {
        if *self.id_sequence.get(i).unwrap() == old.unique_id() {
          id_found = true; id_index = i; break;
        }
      }
      if inst_found && id_found {
        self.instruction_sequence.remove(inst_index);
        self.id_sequence.remove(id_index);

        let new_id = new.unique_id();
        self.instruction_sequence.insert(inst_index, new);
        self.id_sequence.insert(id_index, new_id);
      }
    }
  }

  // Clears the sequence of all instructions.
  pub fn clear(&mut self) {
    self.instruction_sequence.clear();
    self.id_sequence.clear();
  }

  pub fn size(&self) -> usize {
    self.instruction_sequence.len()
  }

  // Returns the sequence of HLO instructions.
  pub fn instructions(&self) -> &Vec<HloInstruction> {
    &self.instruction_sequence
  }

  // Returns the unique IDs of the instructions in the sequence (in order).
  pub fn ids(&self) -> &Vec<i64> {
    &self.id_sequence
  }
}

// A class representing a sequential schedule of instructionns for an HLO module.
pub struct HloSchedule {
  //module: Box<HloModule>,
  sequences: HashMap<i64, HloInstructionSequence>,
  execution_threads: HashMap<i64, String>,
}

impl HloSchedule {
  pub fn new(/*module: HloModule*/) -> Self {
    HloSchedule {
      //module: Box::new(module),
      sequences: HashMap::new(),
      execution_threads: HashMap::new()
    }
  }

  // Returns a reference to the sequence for the given computation.
  pub fn sequence(&self, computation: &HloComputation)
    -> Option<&HloInstructionSequence>
  {
    self.sequences.get(&computation.unique_id())
  }

  // Returns the sequence for the given computation.
  pub fn get_or_create_sequence(
    &mut self,
    _module: &HloModule,
    computation: &HloComputation) -> &HloInstructionSequence
  {
    if !self.sequences.contains_key(&computation.unique_id()) {
      // No sequence found for computation. Create and return an empty one.
      //debug_assert!(computation.parent().unwrap() == *module);
      self.execution_threads.insert(computation.unique_id(), computation.execution_thread());
      let sequence = HloInstructionSequence::new();
      self.sequences.insert(computation.unique_id(), sequence);
    }

    self.sequences.get(&computation.unique_id()).unwrap()
  }

  // Sets the sequence for the given computation to the given sequence.
  pub fn set_sequence(
    &mut self,
    computation: &HloComputation,
    sequence: HloInstructionSequence)
  {
    //assert!(*computation.parent().as_ref().unwrap() == *self.module.as_ref()); 
    self.sequences.insert(
      computation.unique_id(), sequence);
    self.execution_threads.insert(
      computation.unique_id(), computation.execution_thread());
  }

  // Returns a map from HloComputation unique id to instruction sequence.
  pub fn sequences(&self) -> &HashMap<i64, HloInstructionSequence> {
    &self.sequences
  }

  pub fn num_sequences_by_execution_thread() {}

  // Returns true if the schedule has a sequence for the given computation.
  pub fn is_computation_scheduled(&self, computation: &HloComputation) -> bool {
    self.sequences.contains_key(&computation.unique_id())
  }

  // Removes the computation from the sequences.
  pub fn remove_computation(&mut self, computation: &HloComputation) {
    self.sequences.remove(&computation.unique_id());
    self.execution_threads.remove(&computation.unique_id());
  }

  // Removes the instruction from the computation's sequence.
  pub fn remove_instruction(
    &mut self,
    computation: &HloComputation,
    instruction: &HloInstruction)
  {
    let mut sequence =
      self.sequences.get_mut(&computation.unique_id());
    assert!(sequence.is_some());
    sequence.as_mut().unwrap().remove_instruction(&instruction);
  }

  // Replaces the old instruction with the new instruction in the computation's
  // sequence.
  pub fn replace_instruction(
    &mut self,
    computation: &HloComputation,
    old: &HloInstruction,
    new: HloInstruction)
  {
    let mut sequence =
      self.sequences.get_mut(&computation.unique_id());
    assert!(sequence.is_some());
    sequence.as_mut().unwrap().replace_instruction(old, new);
  }

  pub fn update() {}
  pub fn verify() {}
  pub fn to_string() {}

  pub fn empty(&self) -> bool {
    self.sequences.is_empty()
  }

  pub fn module(&self) -> &HloModule {
    //&self.module.as_ref()
    unimplemented!()
  }
}