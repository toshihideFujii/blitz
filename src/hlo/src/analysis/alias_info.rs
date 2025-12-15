#![allow(dead_code)]

//use common::shape::Shape;

use crate::{analysis::hlo_dataflow_analysis::HloOperandIndex, hlo_instruction::HloInstruction,
  hlo_opcode::HloOpcode};

pub struct AliasInfo {}

impl AliasInfo {
  pub fn default() -> Self {
    AliasInfo {  }
  }

  // Returns whether the buffer assigned to `operand` at ShapeIndex
  // `operand_index` needs to share the buffer assigned to `user` at ShapeIndex
  // `user_index`. This is a wrapper around GetInPlaceInputOutputPairs() that
  // checks whether the given operand and user appear as part of what
  // GetInPlaceInputOutputPairs() returns.
  pub fn must_alias(
    &self,
    operand: &HloInstruction,
    operand_index: &Vec<i64>,
    user: &HloInstruction,
    user_index: &Vec<i64>)-> bool
  {
    for pair in self.get_in_place_input_output_pairs(user) {
      if user.operand(pair.0.operand_number as usize) == operand &&
        &pair.0.operand_index == operand_index &&
        &pair.1 == user_index
      {
        return true;
      }
    }
    false
  }

  // Returns the pairs of inputs and outputs that must share the same buffer,
  // according to the aliasing rules for the `user` instruction.
  //
  // This function only considers array values as inputs and outputs, so
  // when tuples are present it "sees through" to the array values inside. The
  // HloUse describing the input parameter contains not only the operand number
  // but also a shape index describing its position inside a nested tuple shape
  // (if any). Similarly, the output parameter is described by a shape index
  // into the nested tuple shape (if any) of the output value.
  //
  // For example, for this hypothetical op:
  //   %foo = (f32[1], (f32[2], f32[3]))
  //              op((f32[4], f32[5]) %arg0, f32[6] %arg1)
  //
  // ... the results can include any of the 3 * 3 = 9 possible pairs of
  // input and output arrays.
  pub fn get_in_place_input_output_pairs(
    &self, user: &HloInstruction) -> Vec<(HloOperandIndex, Vec<i64>)>
  {
    let hint =
      self.get_non_default_in_place_input_output_pairs(user);
    if hint.is_some() {
      return hint.unwrap();
    }
    if user.opcode() == HloOpcode::AllReduceStart {
      if user.raw_backend_config_string().contains("nvshmem") {
        return vec![];
      }
    }
    if is_default_in_place_operation(user) {
      //let num_in_place_operands = user.operand_count();
      // TODO
    }
    if user.opcode() == HloOpcode::CollectivePermute &&
      user.operands().len() == 4
    {
      if user.operand(1).shape().is_tuple() {
        let mut in_place_pairs = vec![];
        let op_index =
          HloOperandIndex::new(1, vec![]);
        in_place_pairs.push((op_index, vec![]));
        for i in 0..user.operand(1).shape().tuple_shapes_vec().len() {
          let op_index =
            HloOperandIndex::new(1, vec![i as i64]);
          in_place_pairs.push((op_index, vec![i as i64]));
        }
        return in_place_pairs;
      }
      let op_index =
        HloOperandIndex::new(1, vec![]);
      return vec![(op_index, vec![])];
    }
    if user.opcode() == HloOpcode::CollectivePermuteStart &&
      user.operands().len() == 4
    {
      if user.operand(1).shape().is_tuple() {
        let mut in_place_pairs = vec![];
        let op_index =
          HloOperandIndex::new(1, vec![]);
        in_place_pairs.push((op_index, vec![1]));
        for i in 0..user.operand(1).shape().tuple_shapes_vec().len() {
          let op_index =
            HloOperandIndex::new(1, vec![i as i64]);
          in_place_pairs.push((op_index, vec![1, i as i64]));
        }
        return in_place_pairs;
      }
      let op_index =
        HloOperandIndex::new(1, vec![]);
      return vec![(op_index, vec![])];
    }
    if user.opcode() == HloOpcode::CustomCall {
      // TODO
    }
    if user.opcode() == HloOpcode::Fusion {
      let aliasing_pairs =
        user.output_to_operand_aliasing();
      let mut in_place_pairs =
        self.get_fusion_instruction_in_place_input_output_pairs(user);
      if !aliasing_pairs.is_empty() {
        for pair in &aliasing_pairs {
          let mut output_shape_index = vec![];
          output_shape_index.clone_from(&pair.0);
          let operand_index = pair.1.0;
          let mut operand_shape_index = vec![];
          operand_shape_index.clone_from(&pair.1.1);
          let op_index = HloOperandIndex::new(
            operand_index, operand_shape_index);
          in_place_pairs.push((op_index, output_shape_index));
        }
      }
      return in_place_pairs;
    }
    if user.opcode() == HloOpcode::SetDimensionSize {
      let dimension = user.dimension();
      let mut in_place_pairs = vec![];
      if user.shape().is_dynamic_dimension(dimension) ==
        user.shape().is_dynamic_dimension(dimension)
      {
        let op_index =
          HloOperandIndex::new(1, vec![]);
        in_place_pairs.push((op_index, vec![]));
      }
      return in_place_pairs;
    }
    if user.opcode() == HloOpcode::RaggedAllToAll {
      let op_index =
        HloOperandIndex::new(1, vec![]);
      return vec![(op_index, vec![])];
    }

    vec![]
  }

  // Backend-specific may-alias hint. If an empty optional is returned, the
  // default rules in HloDataflowAnalysis are used. `operand` should be an
  // operand of `user`. `operand_index` should be the output index of `operand`,
  // `user_index` should be the output index of `user`.
  pub fn may_alias(&self,
    _operand: &HloInstruction,
    _operand_index: &Vec<i64>,
    _user: &HloInstruction,
    _user_index: &Vec<i64>) -> Option<bool>
  {
    None
  }

  // Backend-specific hook that allows to deviate from the default must-alias
  // rules in GetInPlaceInputOutputPairs(). If an empty optional is returned,
  // the default rules are used. Otherwise, the return value of this function is
  // used.
  pub fn get_non_default_in_place_input_output_pairs(
    &self, _user: &HloInstruction) -> Option<Vec<(HloOperandIndex, Vec<i64>)>>
  {
    None
  }

  // Returns in-place input/output pairs for the given fusion instruction,
  // according to the aliasing rules for the corresponding fusion computation.
  fn get_fusion_instruction_in_place_input_output_pairs(
    &self, _fusion: &HloInstruction) -> Vec<(HloOperandIndex, Vec<i64>)>
  {
    /*
    let in_place_input_output_pairs = vec![];
    let func = |sub_shape: &Shape, index: &mut Vec<i64>| {
      // Start from the root instruction of the fusion computation and follow
      // tuple indirection backwards to find the "output source", i.e. the
      // instruction that is the original source of the array output in
      // question. If there is no such indirection the "output source" will
      // just be the fusion root instruction itself.
      let output_source_instruction = fusion.fused_expression_root();
      let output_source_index = index;
      let pair = follow_tuple_indirection(
        output_source_instruction, output_source_index);

      // The aliasing rules of the "output source" instruction determine the
      // aliasing rules for the entire fusion. If we can connect (following
      // tuple indirection) the input of an "in-place" pair to one of the
      // fusion's inputs, and the output of this "in-place" pair to the fusion
      // output in question, then this fusion input and output must alias.
      let in_place_pairs =
        self.get_in_place_input_output_pairs(output_source_instruction);

      let mut in_place_input_source = None;
      let mut in_place_input_index = None;
      for output_source_in_place_pair in &in_place_pairs {
        let input = &output_source_in_place_pair.0;
        let output_index = &output_source_in_place_pair.1;
        if output_index == output_source_index {
          in_place_input_source =
            Some(output_source_instruction.operand(input.operand_number as usize));
          in_place_input_index = Some(&input.operand_index);
          // Follow tuple indirection backwards from the instruction input to
          // try to find a fusion parameter. If found, that parameter aliases
          // the current output. If not, the current output aliases no input.
          let result = follow_tuple_indirection(
            in_place_input_source.as_ref().unwrap(),
            in_place_input_index.as_mut().unwrap());
          in_place_input_source = Some(&result.0);
          in_place_input_index = Some(&result.1);
          if in_place_input_source.as_ref().unwrap().opcode() == HloOpcode::Fusion {
            // Nested fusions can have aliasing that allows us to peephole
            // through to their producer.
            let nested_in_place_input_output_pairs =
              self.get_in_place_input_output_pairs(in_place_input_source.as_ref().unwrap());
            for pair in &nested_in_place_input_output_pairs {
              // If the nested fusion has aliasing that matches the index of
              // this input for its output, then peephole to its input.
              in_place_input_source = Some(in_place_input_source.as_ref().unwrap()
                .operand(pair.0.operand_number as usize));
              in_place_input_index = Some(&pair.0.operand_index);
              let result = follow_tuple_indirection(
                in_place_input_source.as_ref().unwrap(),
                in_place_input_index.as_mut().unwrap());
              in_place_input_source = Some(&result.0);
              in_place_input_index = Some(&result.1);
            }
          }
        }
      }
      // Skip bitcast
      if in_place_input_source.is_some() &&
        in_place_input_source.as_ref().unwrap().opcode() == HloOpcode::Bitcast
      {

      }
      if in_place_input_source.is_some() &&
        in_place_input_source.as_ref().unwrap().opcode() == HloOpcode::Parameter
      {

      }
    };

    in_place_input_output_pairs
    */
    unimplemented!()
  }
}

// Removes layers of tuple indirection introduced via 'tuple' and
// 'get-tuple-element' instructions to more directly identify the source of the
// given HLO value (identified by the given `ShapeIndex` into the output of the
// given `HloInstruction`).
//
// e.g. for the following:
//    %x = some-op(...)
//    %foo = get-tuple-element(%x), index=0
//    %bar = tuple(%y, %foo)
//
// ... FollowTupleIndirection(%bar, {1}) == {%x, {0}} (output 1 of 'bar' comes
// from output 0 of %x).
//
// Note that all 'tuple' instructions are followed before all
// 'get-tuple-element' instructions are followed. This is because it is assumed
// that tupling a value and then extracting it from the tuple again will not
// occur in properly-optimized IR.
pub fn follow_tuple_indirection(
  mut instruction: &HloInstruction,
  operand_index: &mut Vec<i64>) -> (HloInstruction, Vec<i64>)
{
  while instruction.opcode() == HloOpcode::Tuple && !operand_index.is_empty(){
    instruction = instruction.operand(operand_index[0] as usize);
    operand_index.pop();
  }
  while instruction.opcode() == HloOpcode::GetTupleElement {
    operand_index[0] = instruction.tuple_index();
    instruction = instruction.operand(0);
  }
  let mut cloned_index = vec![];
  cloned_index.clone_from(operand_index);
  (instruction.clone(), cloned_index)
}

fn is_default_in_place_operation(hlo: &HloInstruction) -> bool {
  let opcode = hlo.opcode();
  opcode == HloOpcode::DynamicUpdateSlice ||
  opcode == HloOpcode::Scatter ||
  opcode == HloOpcode::AllReduceStart
}