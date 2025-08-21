#![allow(dead_code)]

use hlo::hlo_instruction::HloInstruction;

// AllToAllDecomposer is a pass which converts unsupported array all_to_all
// into tuple all_to_all or array all_to_all with a minimum rank where the split
// dimension is the size of the replica_groups.
pub struct AllToAllDecomposer {
  decompose_to_tuple: bool,
  min_array_rank: i64
}

impl AllToAllDecomposer {
  pub fn new(decompose_to_tuple: bool, min_array_rank: i64) -> Self {
    AllToAllDecomposer {
      decompose_to_tuple: decompose_to_tuple,
      min_array_rank: min_array_rank
    }
  }

  pub fn name() -> String {
    "all-to-all-decomposer".to_string()
  }

  fn instruction_matches_pattern(&self, instruction: &HloInstruction) -> bool {
    // Do not attempt to change layout constrained collectives.
    if instruction.constrain_layout() {
      return false;
    }
    if instruction.shape().is_tuple() {
      return false;
    }
    if self.decompose_to_tuple {
      return true;
    }
    instruction.shape().dimensions_vec().len() < self.min_array_rank as usize
  }

  fn expand_instruction(
    _instruction: &HloInstruction) -> Result<HloInstruction, String>
  {
    unimplemented!()
  }
}