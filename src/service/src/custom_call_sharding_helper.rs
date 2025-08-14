#![allow(dead_code)]

use hlo::{hlo_instruction::HloInstruction, hlo_sharding::HloSharding};

// Helper class that helps implement sharding propagation policies for
// CustomCalls. It is called and used by the ShardingPropagation pass. Meant to
// be overridden by targets.
pub struct CustomCallShardingHelper {}

impl CustomCallShardingHelper {
  pub fn new() {}

  // Function that manipulates an instruction sharding based on a user wanting
  // to update the sharding of an instruction.
  pub fn propagate_user_sharding(
    &self,
    _instruction: &HloInstruction,
    _user: &HloInstruction,
    sharding: &HloSharding) -> HloSharding
  {
    sharding.clone()
  }

  // Infer sharding from the operands of an instruction.
  pub fn infer_sharding_from_operands(
    &self, _instrction: &HloInstruction) -> Option<HloSharding>
  {
    None
  }

  // Returns if the instruction passed as parameter is a supported custom-call
  // for which the functions of this class are implemented.
  pub fn is_custom_call_shardable(
    &self, _instruction: &HloInstruction) -> bool
  {
    false
  }

  // Returns the list of instructions in sub-computations that must be sharded
  // in the same way as `instruction`.
  pub fn get_related_instructions(
    &self, _instruction: &HloInstruction) -> Vec<HloInstruction>
  {
    unimplemented!()
  }

  // Returns if the given custom-call instruction can propagate sharding to its
  // operands.
  pub fn can_propagate_sharding_to_operands(
    &self, _instruction: &HloInstruction) -> bool
  {
    true
  }
}