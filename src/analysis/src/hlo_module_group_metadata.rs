#![allow(dead_code)]

use std::collections::HashMap;

use hlo::{
  hlo_computation::HloComputation,
  hlo_instruction::HloInstruction,
  hlo_module::HloModule,
  hlo_opcode::HloOpcode
};

use crate::hlo_alias_analysis::HloAliasAnalysis;

#[derive(Debug, Clone, PartialEq)]
pub enum ComputationKind {
  Invalid,
  WhileCondition,
  WhileBody,
  ConditionalBranch,
  CallFunction,
}

#[derive(PartialEq)]
pub struct TrackedInstruction {
  instruction: Option<HloInstruction>,
  kind: ComputationKind,
  index: i64,
}

impl TrackedInstruction {
  pub fn new() {}

  pub fn instruction(&self) -> &Option<HloInstruction> {
    &self.instruction
  }

  pub fn to_string(&self) -> String {
    unimplemented!()
  }
}

// Represents a channel and the instructions that form the channel.
pub struct Channel {
  send: Option<HloInstruction>,
  recv: Option<HloInstruction>,
  send_done: Option<HloInstruction>,
  recv_done: Option<HloInstruction>,
}

// Class for bookkeeping the information on the given modules, in particular
// on the interaction between computations.
pub struct HloModuleGroupMetadata<'module> {
  companion_sets: Vec<Vec<HloInstruction>>,
  companion_set_index: HashMap<HloInstruction, i64>,
  tracked_instructions: HashMap<HloComputation, TrackedInstruction>,
  tracked_instructions_comms: HashMap<HloInstruction, Vec<HloInstruction>>,
  channels: Vec<Channel>,
  channel_id_map: HashMap<i64, i64>,
  all_reduce_map: HashMap<i64, Vec<HloInstruction>>,
  max_channel_id: i64,
  modules: Vec<HloModule>,
  alias_analysis: HashMap<HloModule, HloAliasAnalysis<'module>>
}

impl<'module> HloModuleGroupMetadata<'module> {
  // Build and return the metadata for the given modules.
  pub fn build(_modules: Vec<HloModule>) {}

  // Returns true if the instruction is one of the 4 channel instructions.
  pub fn is_channel_instruction(&self, instruction: &HloInstruction) -> bool {
    match instruction.opcode() {
      // TODO
      HloOpcode::Send => return true,
      _ => return false
    }
  }

  // Returns true if the instruction is a companion instruction.
  pub fn is_companion_instruction(&self, instruction: &HloInstruction) -> bool {
    self.companion_set_index.contains_key(instruction)
  }

  // Returns true if the instruction is either a cross-module all-reduce
  // instruction in a non-spmd module.
  pub fn is_non_spmd_cross_module_all_reduce(&self, instruction: &HloInstruction) -> bool {
    instruction.is_cross_module_all_reduce() &&
    !instruction.get_module().as_ref().unwrap().config().use_spmd_partitioning()
  }

  // Returns true if the instruction is either a channel instruction, a
  // cross-module non-spmd all-reduce instruction, or a companion instruction.
  pub fn instruction_communicates(&self, instruction: &HloInstruction) -> bool {
    self.is_channel_instruction(instruction) ||
    self.is_companion_instruction(instruction) ||
    self.is_non_spmd_cross_module_all_reduce(instruction)
  }

  // Returns the Channel instance for the given channel id.
  pub fn get_channel(&self, channel_id: i64) -> &Channel {
    debug_assert!(self.channel_id_map.contains_key(&channel_id));
    self.channels.get(*self.channel_id_map.get(&channel_id)
      .unwrap() as usize).unwrap()
  }

  // Returns if the given channel id exists in metadata.
  pub fn has_channel(&self, channel_id: i64) -> bool {
    self.channel_id_map.contains_key(&channel_id)
  }

  // Returns the all-reduce instructions with the same channel_id.
  pub fn get_all_reduce_group(&self, channel_id: i64) -> &Vec<HloInstruction> {
    self.all_reduce_map.get(&channel_id).unwrap()
  }

  // Returns the computation that contains the peer channel intructions for
  // the given instruction.
  pub fn peer_computation(&self, instruction: &HloInstruction) -> &HloComputation {
    debug_assert!(self.is_channel_instruction(instruction));
    let channel =
      self.get_channel(instruction.channel_id().unwrap());
    match instruction.opcode() {
      HloOpcode::Send => channel.recv.as_ref().unwrap().parent(),
      HloOpcode::SendDone => channel.recv.as_ref().unwrap().parent(),
      HloOpcode::Recv => channel.send.as_ref().unwrap().parent(),
      HloOpcode::RecvDone => channel.send.as_ref().unwrap().parent(),
      _ => panic!("Opcode not supported.")
    };
    unreachable!();
  }

  // Returns the path of nested companion instructions, interms of HLO instructions.
  pub fn get_companions_path(
    &self, instruction: &HloInstruction) -> Vec<&TrackedInstruction>
  {
    let mut path = vec![];
    let mut parent = instruction.parent();
    let mut companion =
      self.get_tracked_instruction(parent);
    while companion.is_some() {
      parent = companion.as_ref().unwrap().instruction().as_ref().unwrap().parent();
      path.push(companion.unwrap());
      companion = self.get_tracked_instruction(parent);
    }
    path
  }

  // Checks whether two companion paths (as returned by the get_companions_path()
  // api) are compatible.
  pub fn check_companion_paths_compatibility(
    &self, path0: &Vec<TrackedInstruction>, path1: &Vec<TrackedInstruction>) -> bool
  {
    if path0.len() != path1.len() {
      println!("Companion path size do not match: {:?} != {:?}.",
        path0.len(), path1.len());
      return false;
    }
    for i in 0..path0.len() {
      if &path0[i] != &path1[i] {
        println!("Compnion instructions at path index {:?} do not have the same
          opcopde: {:?} vs {:?}.", i, path0[i].to_string(), path1[i].to_string());
        return false;
      }
    }
    true
  }

  // Returns the unique integer for each module.
  pub fn get_module_id(&self, module: &HloModule) -> i64 {
    for i in 0..self.modules.len() {
      if &self.modules[i] == module {
        return i as i64;
      }
    }
    panic!("Unknown module.");
  }

  // Retrieves the device an instruction is assigned to.
  pub fn get_instruction_device(&self, instruction: &HloInstruction) -> Option<i64> {
    let mut device = instruction.sharding_unique_device();
    if device.is_none() {
      device = Some(self.get_module_id(instruction.get_module().as_ref().unwrap()));
    }
    device
  }

  // Returns the number of modules for devides (excluding the host module).
  pub fn get_device_modules_count(&self) -> usize {
    self.modules.len()
  }

  // Returns the companion set for the given instruction, including the
  // instruction itself.
  pub fn companions(&self, instruction: &HloInstruction) -> &Vec<HloInstruction> {
    debug_assert!(self.companion_set_index.contains_key(instruction));
    self.companion_set(*self.companion_set_index.get(instruction).unwrap() as usize)
  }

  // Returns the companion set at the given index.
  pub fn companion_set(&self, index: usize) -> &Vec<HloInstruction> {
    debug_assert!(index < self.companion_sets.len());
    self.companion_sets.get(index).unwrap()
  }

  // Returns the companion set index of the given instruction.
  pub fn companion_set_index(&self, instruction: &HloInstruction) -> i64 {
    *self.companion_set_index.get(instruction).unwrap()
  }

  // Returns the list of all companion sets in the HLO module group.
  pub fn companion_sets(&self) -> &Vec<Vec<HloInstruction>> {
    &self.companion_sets
  }

  // Returns all channels in the module group.
  pub fn channels(&self) -> &Vec<Channel> {
    &self.channels
  }

  // Returns the maximum channel id used in the module group.
  pub fn max_channel_id(&self) -> i64 {
    self.max_channel_id
  }

  pub fn alias_analysis(&self, _module: &HloModule) -> &HloAliasAnalysis {
    unimplemented!()
  }

  fn record_instructions() {}
  fn verify_channel_instrucitons() {}
  fn add_companion() {}

  // Checks whether a communicating instruction is placed in a valid position
  // within the graph.
  fn check_communicating_instruction(
    &self, instruction: &HloInstruction) -> Result<(), String>
  {
    let computation = instruction.parent();
    let module = computation.parent();
    if module.is_some() &&
       module.as_ref().unwrap().entry_computation() == Some(computation) ||
       self.tracked_instructions.contains_key(computation)
    {
      return Ok(());
    }
    Err("Channel is used in diallowed computation.".to_string())
  }

  fn verify_companion_sets() {}

  // Retrieves a pointer to the stored TrackedInstruction associated with a
  // tracked computation.
  fn get_tracked_instruction(&self, computation: &HloComputation) -> Option<&TrackedInstruction> {
    self.tracked_instructions.get(computation)
  }

  fn dump_collected_stats() {}
}