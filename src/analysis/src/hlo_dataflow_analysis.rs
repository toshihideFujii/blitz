#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use common::{shape::ShapeEqual, shape_util::ShapeUtil};
use hlo::{
  hlo_instruction::HloInstruction,
  hlo_module::HloModule,
  hlo_opcode::HloOpcode,
  hlo_value::{HloPosition, HloUse, HloValue, HloValueSet, InstructionValueSet}
};

use service::{
  call_graph::{CallContext, CallGraph},
  hlo_phi_graph::PhiGraph
};

// Identifies one array input of an HloInstruction.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct HloOperandIndex {
  pub operand_number: i64,
  pub operand_index: Vec<i64>,
}

impl HloOperandIndex {
  pub fn default() -> Self {
    HloOperandIndex { operand_number: 0, operand_index: Vec::new() }
  }

  pub fn new(operand_number: i64, operand_index: Vec<i64>) -> Self {
    HloOperandIndex {
      operand_number: operand_number,
      operand_index: operand_index
    }
  }
}

pub struct ForwardedOperand {
  operand_number: i64,
  operand_index: usize,
}

fn is_1d_slice_without_strides(instr: &HloInstruction) -> bool {
  instr.opcode() == HloOpcode::Slice &&
  instr.slice_starts().len() == 1 &&
  instr.slice_limits().len() == 1 &&
  instr.slice_strides().len() == 1 &&
  instr.slice_strides()[0] == 1
}

pub fn is_slice_input_fusion(unnested_hlo: &HloInstruction) -> bool {
  if !unnested_hlo.is_input_fusion() {
    return false;
  }
  let root = unnested_hlo.fused_expression_root();
  if root.opcode() != HloOpcode::Tuple {
    return false;
  }
  for instr in root.operands() {
    if !is_1d_slice_without_strides(instr) { return false; }
  }
  true
}

// Returns whether we can prove the transitive uses of `param` are in effect
// elementwise. In other words, we prove that the "transitive use closure" will
// all be computed in the same iteration space without any reorder of elements.
// In addition, we check that the "transitive use closure" includes the output
// in the `root_tuple`.
// Theoretically, We can prove more patterns but our primary use case is
// SliceInputFusion.
pub fn are_transitive_uses_effectively_elementwise(
  _param: &HloInstruction,
  _root_tuple: &HloInstruction,
  _out_shape_idx: &Vec<i64>) -> bool
{
  false    
}

// analysis which identifies all HLO values and their uses in an HLO module.
pub struct HloDataflowAnalysis<'module> {
  call_graph: CallGraph<'module>,
  module: &'module HloModule,
  execution_threads: HashSet<String>,
  ssa_form: bool,
  bitcast_defines_value: bool,
  values: HashMap<i64, HloValue>,
  value_sets: HashMap<HloInstruction, InstructionValueSet>,
  value_ids_to_delete: Vec<i64>,
  values_vector: Vec<HloValue>,
  next_value_id: i64,
  phi_graph: PhiGraph
}

impl<'module> HloDataflowAnalysis<'module> {
  pub fn new(
    module: &'module HloModule,
    ssa_form: bool,
    bitcast_defines_value: bool,
    execution_threads: HashSet<String>) -> Self
  {
    HloDataflowAnalysis {
      call_graph: CallGraph::build(&module, &execution_threads.clone()),
      module: module,
      execution_threads: execution_threads,
      ssa_form: ssa_form,
      bitcast_defines_value: bitcast_defines_value,
      values: HashMap::new(),
      value_sets: HashMap::new(),
      value_ids_to_delete: Vec::new(),
      values_vector: Vec::new(),
      next_value_id: 0,
      phi_graph: PhiGraph::new()
    }
  }

  pub fn run(
    _module: &HloModule,
    _ssa_form: bool,
    _bitcast_defines_value: bool,
    _can_share_buffer: Option<&dyn Fn(&HloInstruction, &HloInstruction, usize) -> bool>,
    _forwards_operand: Option<&dyn Fn(&HloInstruction, usize) -> ForwardedOperand>,
    _execution_threads: &HashSet<String>) -> Result<HloDataflowAnalysis<'module>, String>
  {
    unimplemented!()
  }

  // Returns true if 'instruction' defines an HLO value at the given shape index
  // of its output.
  pub fn value_is_defined_at(
    &self,
    instruction: &HloInstruction,
    index: &Vec<i64>) -> bool
  {
    let value_set = self.get_value_set(instruction, index);
    if value_set.values().len() != 1 { return false; }
    value_set.get_unique_value().defining_instruction() == instruction
  }

  // Returns the HloValue defined by 'instruction' at the given shape index of
  // its output.
  pub fn get_value_defined_at(
    &self,
    instruction: &HloInstruction,
    index: &Vec<i64>) -> &HloValue
  {
    debug_assert!(self.value_is_defined_at(instruction, index));
    self.get_unique_value_at(instruction, index)
  }

  // Returns the InstructionValueSet for the given instruction.
  pub fn get_instruction_value_set(
    &self,
    instruction: &HloInstruction) -> Option<&InstructionValueSet>
  {
    debug_assert!(self.value_sets.contains_key(instruction));
    self.value_sets.get(instruction)
  }

  pub fn get_mutable_instruction_value_set(
    &mut self,
    instruction: &HloInstruction) -> Option<&mut InstructionValueSet>
  {
    debug_assert!(self.value_sets.contains_key(instruction));
    self.value_sets.get_mut(instruction)
  }

  pub fn get_instruction_value_set_pairs(
    &self, _instruction: &HloInstruction) -> &Vec<(&Vec<i64>, &HloValueSet)>
  {
    unimplemented!()    
  }

  pub fn get_mutable_instruction_value_set_pairs(
    &mut self, _instruction: &HloInstruction) -> &mut Vec<(&Vec<i64>, &HloValueSet)>
  {
    unimplemented!()    
  }

  pub fn set_instruction_value_set(
    &mut self,
    instruction: HloInstruction,
    value_set: InstructionValueSet)
  {
    self.value_sets.insert(instruction, value_set);
  }

  // Returns all values that are contained in the output of this instruction in
  // a flattened set.
  pub fn get_flattend_value_set(
    &self,
    instruction: &HloInstruction) -> HloValueSet
  {
    let mut value_set = HloValueSet::default();
    let value_set_tree =
      self.get_instruction_value_set(instruction).unwrap();
    
    let mut all_sets = vec![];
    for pair in value_set_tree.nodes() {
      let value_set = &pair.1;
      all_sets.push(value_set.clone());
    }

    value_set.assign_union_of(all_sets);
    value_set
  }

  // Returns the HloValueSet for the given instruction at the given index or the
  // given position.
  pub fn get_value_set(
    &self,
    instruction: &HloInstruction,
    index: &Vec<i64>) -> &HloValueSet
  {
    self.get_instruction_value_set(instruction).unwrap().element(index)
  }

  pub fn get_mutable_value_set(
    &mut self,
    instruction: &HloInstruction,
    index: &Vec<i64>) -> &mut HloValueSet
  {
    self.get_mutable_instruction_value_set(instruction).unwrap().mutable_element(index)
  }

  pub fn get_value_set_by_pos(&self, pos: &HloPosition) -> &HloValueSet {
    self.get_value_set(&pos.instruction, &pos.index)
  }

  // Returns the unique value in the HloValueSet at the given instruction and
  // shape index.
  pub fn get_unique_value_at(
    &self,
    instruction: &HloInstruction,
    index_vec: &Vec<i64>) -> &HloValue
  {
    let value_set = self.get_value_set(instruction, index_vec);
    self.get_value(value_set.get_unique_value().id())
  }

  // Returns the HloValue with the given id.
  pub fn get_value(&self, value_id: i64) -> &HloValue {
    debug_assert!(self.values.contains_key(&value_id), "Value not found.");
    self.values.get(&value_id).unwrap()
  }

  // Returns the total number of HloValues.
  pub fn value_count(&self) -> usize {
    self.values.len()
  }

  // Returns a vector of all HloValues stabily sorted by HloValue::id.
  pub fn values(&self) -> &Vec<HloValue> {
    &self.values_vector
  }

  // Returns the call graph used for computing the dataflow.
  pub fn call_graph(&self) -> &CallGraph {
    &self.call_graph
  }

  pub fn to_string(&self) -> String {
    let mut out = "HloDataflowAnalysis, module ".to_string();
    out.push_str(&self.module.name());
    out.push_str("\n");

    out.push_str("  Instruction value sets:\n");
    for comp in self.module.computations() {
      if HloInstruction::is_thread_included(
        comp.execution_thread(), &self.execution_threads)
      {
        continue;
      }
      for inst in comp.instructions() {
        out.push_str("Instruction: \n  ");
        out.push_str(&inst.name());
        out.push_str(":\n");
        if inst.shape().is_tuple() {
          /*
          let mut instr_v_set =
            self.get_instruction_value_set(inst);
          assert!(instr_v_set.is_some());
          let mut func =
            |index: &Vec<i64>, value_set: &HloValueSet|
          {
            out.push_str("      tuple index ");
            for i in index {
              out.push_str(&i.to_string());
            }
            out.push_str(":\n");
            for value in value_set.values() {
              out.push_str("        ");
              out.push_str(&value.to_short_string());
              if self.value_is_defined_at(inst, &vec![]) {
                out.push_str(" (def)");
              }
              out.push_str("n");
            }
          };
          instr_v_set.as_ref().unwrap().shape_tree.for_each_element(&func);
          */
        } else {
          let top_level_value_set =
            self.get_value_set(inst, &vec![]);
          for value in top_level_value_set.values() {
            out.push_str("      ");
            out.push_str(&value.to_short_string());
            if self.value_is_defined_at(inst, &vec![]) {
              out.push_str(" (def)");
            }
            out.push_str("n");
          }
        }
      }
    }

    out.push_str("  HloValues:\n");
    for value in self.values() {
      out.push_str(&value.to_string());
    }
    out
  }

  // Returns true if 'user' cannot possibly use the buffer at 'index' in
  // 'operand'. Returns false otherwise.
  // 'operand' does not have to be an operand of 'user'. This can be the
  // case with indirect uses.
  pub fn does_not_use_operand_buffer(
    &self, operand: &HloInstruction, index_vec: &Vec<i64>, user: &HloInstruction) -> bool
  {
    for value in self.get_value_set(operand, index_vec).values() {
      for use_ in value.get_uses() {
        if &use_.instruction == user {
          if user.is_loop_fusion() {
            let fusion_param =
              user.fused_parameter(use_.operand_number);
            let value =
              self.get_value_defined_at(
                fusion_param, &use_.operand_index_vec);
            return value.get_uses().is_empty();
          }
          return false;
        }
      }
    }
    true
  }

  // Returns true if 'user' (at 'user_index') can share a buffer with its
  // operand 'operand' (at 'operand_index'). Returns false otherwise.
  // REQUIRES: 'operand' is an operand of 'user'.
  pub fn can_share_operand_buffer_with_user(
    &self,
    operand: &HloInstruction,
    operand_index: &Vec<i64>,
    user: &HloInstruction,
    user_index: &Vec<i64>) -> bool
  {
    if operand.opcode() == HloOpcode::Constant { return false; }
    let operand_subshape =
      ShapeUtil::get_subshape(operand.shape(), operand_index);
    let user_subshape =
      ShapeUtil::get_subshape(user.shape(), user_index);
    
    if is_slice_input_fusion(user) {
      let fusion_param =
        user.fused_parameter(user.operand_index(operand) as i64);
      // We don't require the same dimensions but only the same number of elements
      // and type (to make sure the same buffer size).
      return operand_subshape.is_array() && user_subshape.is_array() &&
        ShapeUtil::elements_in(&operand_subshape) ==
        ShapeUtil::elements_in(&user_subshape) &&
        ShapeUtil::same_element_type(&operand_subshape, &user_subshape) &&
        are_transitive_uses_effectively_elementwise(
          fusion_param, user.fused_expression_root(), user_index);
    }

    let shape_equal = ShapeEqual::new().equal(
      &operand_subshape, &user_subshape);
    // Check that operand and user emit the same shape and layout.
    if shape_equal {
      // Must-alias relationship returns true for in-place operations (DUS and DUS
      // fusions), regardless of the backend.
      let pairs =
        HloDataflowAnalysis::get_in_place_input_output_pairs(user);
      for operand_and_output_index in pairs {
        if &operand_and_output_index.1 != user_index {
          continue;
        }
        for hlo_use in self.get_unique_value_at(
          operand, operand_index).get_uses() {
          let mut operand_index = vec![];
          operand_index.clone_from(&operand_and_output_index.0.operand_index);
          if hlo_use == &HloUse::new(
            user.clone(),
            operand_and_output_index.0.operand_number,
            operand_index)
          {
            return true;
          }
        }
      }
    }

    // can_share_buffer : TODO

    if !shape_equal {
      return false;
    }

    if user.opcode() == HloOpcode::Fusion {

    }

    // There is nothing inherently wrong with while and conditional ops to have
    // input/output buffers to alias with each other, even when the indices are
    // different in the while case. It is a problem when this aliasing causes HLO
    // ops inside these while or conditional to have input/output buffer aliasing
    // that isn't allowed. So allow while and conditional to share buffers with
    // operands and we will discover any problematic sharing when we explore the
    // ops inside these computations.
    if user.opcode() == HloOpcode::While || user.opcode() == HloOpcode::Conditional {
      return true;
    }

    if user.opcode() == HloOpcode::DynamicUpdateSlice ||
      user.opcode() == HloOpcode::Scatter ||
      user.opcode() == HloOpcode::TriangularSolve ||
      user.opcode() == HloOpcode::SetDimensionSize
    {
      // We eliminated other users in HloOrdering::LiveRangeStrictlyBefore
      // so here we just need to check that the use is at the right operand index.
      let operand_indices = user.operand_indices(operand);
      let mut operand_no = 0;
      if user.opcode() == HloOpcode::TriangularSolve {
        operand_no = 1;
      }
      return operand_indices.len() == 1 && operand_indices[0] == operand_no;
    }

    if user.opcode() == HloOpcode::Sort {
      // Only valid if there are no other users.
      if user.users().len() != 1 {
        return false;
      }
      // If we only sort keys, the output of sort is not a tuple, so we can always
      // share the buffer.
      if user.operand_count() == 1 {
        return true;
      }
      debug_assert!(!user_index.is_empty());
      // Only share with the right tuple element buffer.
      let operand_indices = user.operand_indices(operand);
      return operand_indices.len() == 1 && user_index[0] == operand_indices[0];
    }

    if user.opcode() == HloOpcode::Call {
      // Get all uses of value defined by 'operand' at 'operand_index'.
      let uses = self.get_value_defined_at(
        operand, operand_index).get_uses();
      // Return true iff:
      // *) There exists two uses of 'operand'.
      // *) One use is by 'user' (caller).
      // *) One use is by root instruction of called computation (callee root).
      //    (Note: we check the root of the called computation, because the
      //     root result buffer is required to alias with the Call result buffer).
      // *) The root instruction of the called computation is element-wise on
      //    'operand'.
      let mut found_caller_use = false;
      for use_ in uses {
        if use_.instruction == *user { found_caller_use = true; }
      }
      let callee_root = user.to_apply().root_instruction();
      let mut found_elementwise_callee_use = false;
      for use_ in uses {
        if use_.instruction == *callee_root &&
          callee_root.is_elementwise_on_operand(use_.operand_number)
        {
          found_elementwise_callee_use = true;
        }
      }
      return uses.len() == 2 && found_caller_use && found_elementwise_callee_use;
    }
    
    // Loop fusions that contain transposing copies won't reach here as they have
    // different layouts, which fails the check in the beginning of this function.
    user.is_elementwise_on_operand(user.operand_index(operand) as i64)
  }

  pub fn module(&self) -> &HloModule {
    &self.module
  }

  // Returns true if the operation is an in-place operation and its operand 0
  // must alias with the output.
  pub fn is_in_place_operation(opcode: &HloOpcode) -> bool {
    *opcode == HloOpcode::DynamicUpdateSlice || *opcode == HloOpcode::Scatter
  }

  // Returns true if the operation is the start/done of an asynchronous operation,
  // where the buffer used/produced by the op needs to stay alive until the
  // asynchronous operation completes.
  pub fn is_asynchronous_operation_start(opcode: &HloOpcode) -> bool {
    *opcode == HloOpcode::Send ||
    *opcode == HloOpcode::Recv ||
    *opcode == HloOpcode::CopyStart ||
    *opcode == HloOpcode::AllReduceStart ||
    *opcode == HloOpcode::AllGatherStart ||
    *opcode == HloOpcode::CollectivePermuteStart ||
    *opcode == HloOpcode::AsyncStart
  }

  pub fn is_asynchronous_operation_done(opcode: &HloOpcode) -> bool {
    *opcode == HloOpcode::SendDone ||
    *opcode == HloOpcode::RecvDone ||
    *opcode == HloOpcode::CopyDone ||
    *opcode == HloOpcode::AllReduceDone ||
    *opcode == HloOpcode::AllGatherDone ||
    *opcode == HloOpcode::CollectivePermuteDone ||
    *opcode == HloOpcode::AsyncDone
  }

  // Returns the pairs of inputs and outputs that must share the same buffer,
  // according to the aliasing rules for that instruction.
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
    instruction: &HloInstruction) -> Vec<(HloOperandIndex, Vec<i64>)>
  {
    if HloDataflowAnalysis::is_in_place_operation(&instruction.opcode()) {
      // TODO
      return vec![(HloOperandIndex::default(), vec![])];
    } else if instruction.opcode() == HloOpcode::CollectivePermute &&
      instruction.operands().len() == 4
    {
      if instruction.operand(1).shape().is_tuple() {
        let mut in_place_pairs =
          vec![(HloOperandIndex::new(1, vec![]), vec![])];
        let tuple_shapes_len =
          instruction.operand(1).shape().tuple_shapes_vec().len();
        for i in 0..tuple_shapes_len {
          in_place_pairs.push((
            HloOperandIndex::new(1, vec![i as i64]),
            vec![i as i64]));
        }
        return in_place_pairs;
      } else {
        return vec![(HloOperandIndex::new(1, vec![]), vec![])];
      }
    } else if instruction.opcode() == HloOpcode::CollectivePermuteStart &&
      instruction.operands().len() == 4
    {
      if instruction.operand(1).shape().is_tuple() {
        let mut in_place_pairs =
          vec![(HloOperandIndex::new(1, vec![]), vec![1])];
        let tuple_shapes_len =
          instruction.operand(1).shape().tuple_shapes_vec().len();
        for i in 0..tuple_shapes_len {
          in_place_pairs.push((
            HloOperandIndex::new(1, vec![i as i64]),
            vec![1, i as i64]));
        }
        return in_place_pairs;
      } else {
        return vec![(HloOperandIndex::new(1, vec![]), vec![1])];
      }
    } else if instruction.opcode() == HloOpcode::CustomCall {
      // Custom Calls previously assumed that aliased operands were
      // forwarded, but now supports modifiction semantics.
      let aliasing_pairs =
        instruction.output_to_operand_aliasing();
      let mut in_place_pairs = vec![];
      for pair in aliasing_pairs {
        let output_shape_index = pair.0;
        let operand_index = pair.1.0;
        let operand_shape_index = pair.1.1;
        in_place_pairs.push((HloOperandIndex::new(
          operand_index, 
          operand_shape_index),
          output_shape_index));
      }
      return in_place_pairs;
    } else if instruction.opcode() == HloOpcode::AllReduceStart {
      if instruction.operands().len() == 1 {
        return vec![(HloOperandIndex::default(), vec![])];
      }
      let mut in_place_pairs = vec![];
      for i in 0..instruction.operands().len() {
        in_place_pairs.push((
          HloOperandIndex::new(i as i64, vec![]),
          vec![i as i64]));
      }
      return in_place_pairs;
    } else if instruction.opcode() == HloOpcode::Fusion {
        
    } else if instruction.opcode() == HloOpcode::SetDimensionSize {
      let mut in_place_pairs = vec![];
      let dimension = instruction.dimension();
      if instruction.shape().is_dynamic_dimension(dimension) ==
        instruction.shape().is_dynamic_dimension(dimension)
      {
        in_place_pairs.push((HloOperandIndex::default(), vec![]));
      }
      return in_place_pairs;
    } else if instruction.opcode() == HloOpcode::RaggedAllToAll {
      return vec![(HloOperandIndex::new(1, vec![]), vec![])];
    }

    vec![(HloOperandIndex::default(), vec![])]
  }

  // Verifies various invariants of the dataflow analysis.
  pub fn verify(&self) -> Result<(), String> {
    // Verify each HloValue appears in the value sets that the value's positions()
    // indicate.
    for value in self.values() {
      for pos in value.positions() {
        let value_set = self.get_value_set_by_pos(pos);
        for _value in value_set.values() {
          // TODO
        }
      }
    }
    // For each value in each value set, verify that the value set's position
    // appears in the value's positions().
    for comp in self.module.computations() {
      if HloInstruction::is_thread_included(
        comp.execution_thread(), &self.execution_threads)
      {
        continue;
      }
      for inst in comp.instructions() {
        if inst.opcode() == HloOpcode::AsyncStart &&
          inst.async_wrapped_opcode() == HloOpcode::Call ||
          inst.async_wrapped_opcode() == HloOpcode::CustomCall
        {
          continue;
        }
        //for pair in self.get_instruction_value_set(inst) {
          // TODO
        //}
      }
    }
    Ok(())
  }

  fn are_transitive_uses_elementwise_or_tuple(&self, inst: &HloInstruction) -> bool {
    let mut visited = HashSet::new();
    let mut stack = vec![];
    stack.push(inst);

    while !stack.is_empty() {
      let current = stack.pop();
      visited.insert(current.unwrap());
      for user in current.unwrap().users() {
        // Found a user that is non-elementwise on current instruction.
        for use_index in user.operand_indices(current.unwrap()) {
          if !user.is_elementwise_on_operand(use_index) &&
            user.opcode() != HloOpcode::Tuple
          {
            return false;
          }
        }
        if !visited.contains(user) {
          stack.push(user);
        }
      }
    }
    true
  }

  // 1. During value propagation (Propagate function), always create phi
  // values once it see multiple inputs merging at the same point. It then
  // records those phi values as well as their inputs in a phi graph.
  //
  // 2. Post value propagation, Dataflow analysis can then do certain
  // optimization(OptimizePhiValues) on the phi graph to prune uncessary phi
  // nodes.
  //
  // Note that this applies in SSA form, and Both of the functions are
  // guaranteed to exit.
  //
  fn optimize_phi_values(&mut self) {
    // Only applicable to SSA form where phis are defined.
    if !self.ssa_form { return; }

    println!("Before phi graph optimization");
    println!("{:?}", self.phi_graph.to_string());
    self.phi_graph.optimize();
    println!("After phi graph optimization");
    println!("{:?}", self.phi_graph.to_string());

    for comp in self.module.computations() {
      if !HloInstruction::is_thread_included(
        comp.execution_thread(), &self.execution_threads)
      {
        continue;
      }
      /*
      for inst in comp.instructions() {
        let inst_value_set =
          self.get_mutable_instruction_value_set(inst); 
        println!("inst: {:?}", inst.name());
        //println!("{:?}", inst_value_set.unwrap().to_string());
        let mut func =
          |_index: &Vec<i64>, value_set: &mut HloValueSet|
        {
          let values = value_set.values();
          if !(values.len() == 1 && values[0].is_phi()) {
            return;
          }
          let phi_id = values[0].id();
          let new_id = self.phi_graph.find_optimized_value(phi_id);
          if new_id != phi_id {
            value_set.clear();
            let new_value = self.get_value(new_id);
            value_set.add_value(new_value.clone());
            self.mark_value_for_deletion(phi_id);
          }
        };
        inst_value_set.unwrap().shape_tree.for_each_mutable_element(&mut func);
      }
      */
    }
  }

  // Returns a new HloValue defined at the given instruction and shape index.
  fn new_hlo_value(
    &mut self,
    instruction: &HloInstruction,
    index: &Vec<i64>,
    is_phi: bool) -> HloValue
  {
    let value_id = self.next_value_id;
    self.next_value_id += 1;
    let hlo_value = HloValue::new(value_id, instruction, index, is_phi); 
    let result = self.values.insert(value_id, hlo_value);
    debug_assert!(result.is_some());
    result.unwrap()
  }

  // Marks the HloValue with the given ID for the deletion.
  fn mark_value_for_deletion(&mut self, value_id: i64) {
    let value = self.get_value(value_id);
    print!("mark_value_for_deletion( {:?} )", value.to_short_string());
    self.value_ids_to_delete.push(value_id);
  }

  // Deletes all HloValues marked for deletion. Should be called after
  // propagation is complete.
  fn delete_marked_values(&mut self) {
    // Use a set to prevent deleting an id twice.
    let mut id_set = HashSet::new();
    for value in &self.value_ids_to_delete {
      let _ = id_set.insert(*value);
    }

    for value_id in id_set {
      self.values.remove(&value_id);
    }
    self.value_ids_to_delete.clear();
  }

  // Constructs and initializes the InstructionValueSets of all instructions to
  // contain exactly the HloValues defined by each instruction. These values can
  // then propagated throughout the HLO graph by calling Propagate.
  fn initialize_instruction_value_sets(&mut self) -> Result<(), String> {
    let hash_set = HashSet::new();
    for comp in self.module.make_computation_post_order(
      &hash_set, false)
    {
      if !HloInstruction::is_thread_included(
        comp.execution_thread(), &self.execution_threads)
      {
        continue;
      }
      let call_graph_node = self.call_graph.get_node(&comp);
      for inst in comp.make_instruction_post_order() {
        // Create an empty shape tree.
        self.value_sets.insert(inst.clone(),
        InstructionValueSet::new(inst.shape().clone()));

        // For each sub-shape of the instruction shape, add a new HloValue to its
        // HloValueSet. should_define may be provided to define a subset of
        // values.
        let define_all_values =
          |dfs: &mut HloDataflowAnalysis,
           instruction: &HloInstruction,
           _func: &dyn Fn(&Vec<i64>)-> bool|
        {
          for pair in
            dfs.get_instruction_value_set_pairs(instruction)
          {
            let _index = pair.0;
            let defines_value = false;
            // TODO: forwards_value

            if defines_value {
              //let value = dfs.new_hlo_value(instruction, index, false);
              //dfs.get_value_set(instruction, index).add_value(value);
            }
          }
        };

        // Add a new HloValue to the HloValueSet corresponding to the given index
        // of the instruction shape.
        let define_value_at =
          |dfs: &mut HloDataflowAnalysis, instruction: &HloInstruction, index: &Vec<i64>|
        {
          let value = dfs.new_hlo_value(instruction, index, false);
          let v_set = dfs.get_mutable_value_set(instruction, index);
          v_set.add_value(value);
        };

        match inst.opcode() {
          HloOpcode::Bitcast => {
            if self.bitcast_defines_value {
              let func = |_index: &Vec<i64>| -> bool { return true; };
              define_all_values(self, inst, &func);
            }
            break
          },
          // These instructions define no values. The values in their output
          // flow from their operands or from cross computation dataflow.
          HloOpcode::AddDependency => break,
          HloOpcode::While => break,
          HloOpcode::Call => break,
          HloOpcode::Conditional => break,
          HloOpcode::GetTupleElement => break,
          HloOpcode::Domain => break,
          HloOpcode::OptimizationBarrier => break,
          HloOpcode::Parameter => {
            if call_graph_node.context() == CallContext::Both {
              // We do not support a subcomputation that is called from both a
              // parallel and sequential context. In this case, the parameter
              // would both define a value and propagate a value from its
              // caller. This limitation is not really a problem because the call
              // graph is typically flattened.
              unimplemented!();
            }
            if call_graph_node.caller_callsites().is_empty() ||
              call_graph_node.context() == CallContext::Embedded
            {
              // Parameters of computations called in a parallel context (eg, map
              // and reduce) as well as parameters of dead computations define all
              // values in their output. Otherwise the values of the parameter
              // come from the caller (eg, operands to the kCall instruction).
              let func = |_index: &Vec<i64>| -> bool { return true; };
              define_all_values(self, inst, &func);
            }
            break;
          },
          // These instructions only define their top-level values. Any other
          // values flow from their operands.
          HloOpcode::Copy => {
            define_value_at(self, inst, &vec![]);
            break;
          }
          HloOpcode::Tuple => {
            define_value_at(self, inst, &vec![]);
            break;
          }
          HloOpcode::AsyncStart => {
            // AsyncStart produces a tuple of {{aliased operands}, {destination},
            // contexts}. It defines all of the tuple-shaped values and the
            // contexts.
            // If the thread is excluded, then we don't track the contained
            // dataflow, and define the destination values too.
            let thread_included = HloInstruction::is_thread_included(
              inst.async_execution_thread(), &self.execution_threads);
            let func = |index: &Vec<i64>| -> bool {
              ShapeUtil::get_subshape(inst.shape(), index).is_tuple() ||
              (!thread_included && index.first() == Some(&1)) ||
              index.first() > Some(&1)
            };
            define_all_values(self, inst, &func);
            break;
          }
          HloOpcode::AsyncUpdate => {
            // AsyncUpdate produces a tuple of {{aliased operands}, {destination},
            // contexts} where all of the array-typed values alias with the
            // operand. So, only tuple-shaped values are defined by AsyncUpdate.
            let func = |index: &Vec<i64>| -> bool {
              ShapeUtil::get_subshape(inst.shape(), index).is_tuple()
            };
            define_all_values(self, inst, &func);
            break;
          }
          HloOpcode::AsyncDone => {
            // AsyncDone's output aliases its output. It defines all remaining
            // tuple-shaped values.
            let func = |index: &Vec<i64>| -> bool {
              ShapeUtil::get_subshape(inst.shape(), index).is_tuple()
            };
            define_all_values(self, inst, &func);
            break;
          }
          HloOpcode::CopyStart => {
            // CopyStart produces a tuple of {destination buffer, aliased operand,
            // U32 context}.
            define_value_at(self, inst, &vec![]);
            define_value_at(self, inst, &vec![0]);
            define_value_at(self, inst, &vec![2]);
            break;
          }
          HloOpcode::CopyDone => {
            // CopyDone consumes a tuple produced by CopyStart and produces an
            // element. Its output aliases its input tuple element {0}.
            break;
          }
          HloOpcode::AllGatherStart => {
            // AllGatherStart produces a tuple of
            // {aliased operands, destination buffers}. If there is more than
            // one operand, then both aliased operands and destination buffers
            // will be tuples themselves. all-gather-start will define all tuples
            // and all tuple leaves (arrays) in tuple sub-index 1 (destination
            // buffers).
            let func = |index: &Vec<i64>| -> bool {
              ShapeUtil::get_subshape(inst.shape(), index).is_tuple() || index.first() == Some(&1)
            };
            define_all_values(self, inst, &func);
            break;
          }
          HloOpcode::AllGatherDone => {
            // AllGatherDone's output aliases its input tuple element {1}.
            if inst.shape().is_tuple() {
              define_value_at(self, inst, &vec![]);
            }
            break;
          }
          HloOpcode::AllReduceDone => {
            // AllReduceDone's output aliases its input.
            break;
          }
          HloOpcode::CollectivePermuteStart => {
            // TODO
            break;
          }
          HloOpcode::CollectivePermuteDone => {
            // CollectivePermuteDone's output aliases its input tuple element {1}.
            if inst.shape().is_tuple() {
              define_value_at(self, inst, &vec![]);
            }
            break;
          }
          HloOpcode::RecvDone => {
            // RecvDone produces a two-element tuple. Element zero aliases its
            // input tuple element {0}; element one is a token.
            define_value_at(self, inst, &vec![]);
            define_value_at(self, inst, &vec![1]);
            break;
          }
          HloOpcode::Send => {
            // Send produces a tuple of {aliased operand, U32 context, token},
            // therefore only defines the top-level tuple and the tuple elements
            // at {1} and {2}.
            define_value_at(self, inst, &vec![]);
            define_value_at(self, inst, &vec![1]);
            define_value_at(self, inst, &vec![2]);
            break;
          }
          _ => {
            let func = |_index: &Vec<i64>| -> bool { return true; };
            define_all_values(self, inst, &func);
            break;
          }
        }
      }
    }
    Ok(())
  }

  // Updates the value set of the given instruction based on the values flowing
  // into the instruction (operands and cross-computation dataflow).
  fn update_instructionn_value_set(&mut self, instruction: &HloInstruction) -> bool {
    #[allow(unused_assignments)]
    let mut changed = false;
    match instruction.opcode() {
      HloOpcode::AddDependency =>
        changed = self.update_add_dependency_value_set(instruction),
      HloOpcode::AllGatherStart =>
        changed = self.update_all_gather_start_value_set(instruction),
      HloOpcode::AllGatherDone =>
        changed= self.update_all_gather_done_value_set(instruction),
      HloOpcode::AsyncStart =>
        changed = self.update_async_start_value_set(instruction),
      HloOpcode::AsyncUpdate =>
        changed = self.update_async_update_value_set(instruction),
      HloOpcode::AsyncDone =>
        changed = self.update_async_done_value_set(instruction),
      HloOpcode::Bitcast =>
        changed = self.update_bitcast_value_set(instruction),
      HloOpcode::Domain =>
        changed = self.update_domain_value_set(instruction),
      HloOpcode::Copy =>
        changed = self.update_copy_value_set(instruction),
      HloOpcode::GetTupleElement =>
        changed = self.update_get_tuple_element_value_set(instruction),
      HloOpcode::Tuple =>
        changed = self.update_tuple_value_set(instruction),
      HloOpcode::Parameter =>
        changed = self.update_parameter_value_set(instruction),
      HloOpcode::Call =>
        changed = self.update_call_value_set(instruction),
      HloOpcode::While =>
        changed = self.update_while_value_set(instruction),
      HloOpcode::Send =>
        changed = self.update_send_value_set(instruction),
      HloOpcode::RecvDone =>
        changed = self.update_recv_done_value_set(instruction),
      HloOpcode::CopyStart =>
        changed = self.update_copy_start_value_set(instruction),
      HloOpcode::CopyDone =>
        changed = self.update_copy_done_value_set(instruction),
      HloOpcode::Conditional =>
        changed = self.update_conditional_value_set(instruction),
      HloOpcode::AllReduceDone =>
        changed = self.update_all_reduce_done_value_set(instruction),
      HloOpcode::CollectivePermuteStart =>
        changed = self.update_collective_permute_start_value_set(instruction),
      HloOpcode::CollectivePermuteDone =>
        changed = self.update_collective_permute_done_value_set(instruction),
      HloOpcode::OptimizationBarrier =>
        changed = self.update_optimization_barrier_value_set(instruction),
      _ => unimplemented!(),
    }

    // TODO: forwards_value
    changed
  }

  // Updates the value set of the given instruction based on the values flowing
  // into the instruction value set changed.
  fn update_bitcast_value_set(&mut self, bitcast: &HloInstruction) -> bool {
    debug_assert!(bitcast.opcode() == HloOpcode::Bitcast);
    let operand_set =
      self.get_instruction_value_set(bitcast.operand(0)).unwrap();
    let bitcast_set =
      self.get_instruction_value_set(bitcast).unwrap();

    if self.bitcast_defines_value && operand_set != bitcast_set {
      self.set_instruction_value_set(bitcast.clone(), operand_set.clone());
      return true;
    }
    false
  }

  fn update_call_value_set(&mut self, call: &HloInstruction) -> bool {
    debug_assert!(call.opcode() == HloOpcode::Call);
    let value_set =
      self.get_instruction_value_set(call).unwrap();
    let root_value_set =
      self.get_instruction_value_set(call.to_apply().root_instruction()).unwrap();

    if value_set != root_value_set {
      self.set_instruction_value_set(call.clone(), root_value_set.clone());
      return true;
    }
    false
  }

  fn update_conditional_value_set(&mut self, conditional: &HloInstruction) -> bool {
    debug_assert!(conditional.opcode() == HloOpcode::Conditional);
    let mut inputs = vec![];
    for j in 0..conditional.branch_count() {
      let inst_value_set =
        self.get_instruction_value_set(conditional.branch_computation(j).root_instruction());
      inputs.push(inst_value_set.unwrap().clone());
    }
    if self.ssa_form {
      self.phi(conditional, &inputs)
    } else {
      self.get_mutable_instruction_value_set(conditional)
        .as_mut().unwrap().assign_union_of(inputs)
    }
  }

  fn update_copy_value_set(&mut self, copy: &HloInstruction) -> bool {
    assert_eq!(copy.opcode(), HloOpcode::Copy);
    let mut changed = false;
    for pair in
      self.get_mutable_instruction_value_set_pairs(copy)
    {
      let index = pair.0;
      if index.is_empty() {
        // Copy shallow copies and thus defines the top-level value so nothing to update.
        continue;
      }
      let _value_set = pair.1;
      //let opreand_value_set =
        //self.get_value_set(copy.operand(0), index);
      //if value_set != opreand_value_set {
        //pair.1 = opreand_value_set;
        changed = true;
      //}
    }
    changed
  }

  fn update_domain_value_set(&mut self, domain: &HloInstruction) -> bool {
    debug_assert!(domain.opcode() == HloOpcode::Domain);
    let mut changed = false;
    let nodes =
      self.get_instruction_value_set(domain).unwrap().nodes();
    for pair in nodes {
      let index = pair.0 as i64;
      let value_set = &pair.1;
      let operand_value_set =
        self.get_value_set(domain.operand(0), &vec![index]);
      if value_set != operand_value_set {
        // TODO
        //pair.1 = operand_value_set.clone();
        changed = true;
      }
    }
    changed
  }

  fn update_get_tuple_element_value_set(&mut self, gte: &HloInstruction) -> bool {
    assert_eq!(gte.opcode(), HloOpcode::GetTupleElement);
    let mut changed = false;

    // The GetTupleElement instruction forwards the values from the specified
    // tuple element.
    for pair in
      self.get_mutable_instruction_value_set_pairs(gte)
    {
      let index = pair.0;
      let _value_set = pair.1;
      // The corresponding ShapeIndex of the operand is simply the GTE ShapeIndex
      // with the tuple element number prefixed.
      let mut operand_index = vec![gte.tuple_index()];
      for i in index {
        operand_index.push(*i);
      }
      //let operand_value_set =
        //self.get_value_set(gte.operand(0), &operand_index);
      //if value_set != operand_value_set {
        //pair.1 = operand_value_set;
        changed = true;
      //}
    }
    changed
  }

  fn update_parameter_value_set(&self, _instruction: &HloInstruction) -> bool {
    unimplemented!()
  }

  fn update_async_start_value_set(&self, _instruction: &HloInstruction) -> bool {
    unimplemented!()
  }

  fn update_async_update_value_set(&self, _instruction: &HloInstruction) -> bool {
    unimplemented!()
  }

  fn update_async_done_value_set(&self, _instruction: &HloInstruction) -> bool {
    unimplemented!()
  }

  fn update_copy_start_value_set(&mut self, copy_start: &HloInstruction) -> bool {
    debug_assert!(copy_start.opcode() == HloOpcode::CopyStart);
    let mut changed = false;
    let operand_value_set =
      self.get_value_set(copy_start.operand(0), &vec![0]);
    let value_set =
      self.get_value_set(copy_start, &vec![1]);
    if value_set != operand_value_set {
      // TODO
      //self.set_instruction_value_set(copy_start.clone(), operand_value_set.clone());
      changed = true;
    }
    changed
  }

  fn update_copy_done_value_set(&self, copy_done: &HloInstruction) -> bool {
    assert_eq!(copy_done.opcode(), HloOpcode::CopyDone);
    let mut changed = false;
    // CopyDone forwards the operand value at {0} to element {} of its output.
    let operand_value_set = 
      self.get_value_set(copy_done.operand(0), &vec![0]);
    let value_set = self.get_value_set(copy_done, &vec![]);
    if value_set != operand_value_set {
      //value_set = operand_value_set; // TODO
      changed = true;
    }
    changed
  }

  fn update_optimization_barrier_value_set(&mut self, barrier: &HloInstruction) -> bool {
    // Optimization Barriers just forward their operand. Given that barriers can
    // have a tuple operand, we iterate through its indexes, like for copies.
    // Unlike copies though we also propagate the top-level value.
    assert_eq!(barrier.opcode(), HloOpcode::OptimizationBarrier);
    let mut changed = false;
    for pair in
      self.get_mutable_instruction_value_set_pairs(barrier)
    {
      let _index = pair.0;
      let _value_set = pair.1;
      //let operand_value_set =
        //self.get_value_set(barrier.operand(0), index);
      //if value_set != operand_value_set {
        //value_set = operand_value_set; // TODO
        changed = true;
      //}
    }
    changed
  }

  fn update_recv_done_value_set(&mut self, recv_done: &HloInstruction) -> bool {
    assert_eq!(recv_done.opcode(), HloOpcode::RecvDone);
    let mut changed = false;
    // RecvDone forwards the operand value at {0} to element {0} of its output.
    for pair in
      self.get_mutable_instruction_value_set_pairs(recv_done)
    {
      let index = pair.0;
      let _value_set = pair.1;
      if index.is_empty() || index[0] != 0 {
        continue;
      }
      //let operand_value_set =
        //self.get_value_set(recv_done.operand(0), index);
      //if value_set != operand_value_set {
        //value_set = operand_value_set // TODO
        changed = true;
      //}
    }
    changed
  }

  fn update_send_value_set(&mut self, send: &HloInstruction) -> bool {
    assert_eq!(send.opcode(), HloOpcode::Send);
    let mut changed = false;
    // Send forwards the operand value to the output tuple at {0}.
    for pair in
      self.get_mutable_instruction_value_set_pairs(send)
    {
      let operand_index = pair.0;
      let _operand_value_set = pair.1;

      let mut index = vec![0];
      for i in operand_index {
        index.push(*i);
      }

      //let value_set = self.get_value_set(send, &index);
      //if value_set != operand_value_set {
        //value_set = operand_value_set; // TODO
        changed = true;
      //}
    }
    changed
  }

  fn update_tuple_value_set(&mut self, tuple: &HloInstruction) -> bool {
    assert_eq!(tuple.opcode(), HloOpcode::Tuple);
    let mut changed = false;
    for i in 0..tuple.operands().len() {
      // Copy the value set(s) of each operand into the respective position in the
      // Tuple instruction's value sets.
      for pair in
        self.get_mutable_instruction_value_set_pairs(tuple)
      {
        let operand_index = pair.0;
        let _operand_value_set = pair.1;

        let mut index = vec![i as i64];
        for op_index in operand_index {
          index.push(*op_index);
        }

        //let value_set = self.get_value_set(tuple, &index);
        //if value_set != operand_value_set {
          //value_set = operand_value_set; // TODO
          changed = true;
        //}
      }
    }
    changed
  }

  fn update_while_value_set(&mut self, while_value: &HloInstruction) -> bool {
    debug_assert!(while_value.opcode() == HloOpcode::While);
    let mut inputs = vec![];
    inputs.push(self.get_instruction_value_set(
      while_value.while_body().root_instruction()).unwrap().clone());
    inputs.push(self.get_instruction_value_set(
      while_value.operand(0)).unwrap().clone());
    
    if self.ssa_form {
      self.phi(while_value, &inputs)
    } else {
      self.get_mutable_instruction_value_set(
        while_value).unwrap().assign_union_of(inputs)
    }
  }

  fn update_add_dependency_value_set(&mut self, add_dependency: &HloInstruction) -> bool {
    debug_assert!(add_dependency.opcode() == HloOpcode::AddDependency);
    let operand_set =
      self.get_instruction_value_set(add_dependency.operand(0)).unwrap();
    let add_dependency_set =
      self.get_instruction_value_set(add_dependency).unwrap();
    
    if operand_set != add_dependency_set {
      self.set_instruction_value_set(
        add_dependency.clone(), operand_set.clone());
      return true;
    }
    false
  }

  fn update_all_gather_start_value_set(&self, all_gather_start: &HloInstruction) -> bool {
    assert_eq!(all_gather_start.opcode(), HloOpcode::AllGatherStart);
    let mut changed = false;
    // AllGatherStart forwards the operand values to element {0} of its output.
    for i in 0..all_gather_start.operand_count() {
      let operand_value_set =
        self.get_value_set(all_gather_start.operand(i), &vec![]);
      
      let mut output_index = vec![0];
      if all_gather_start.operand_count() > 1 {
        output_index.push(i as i64);
      }

      let value_set = self.get_value_set(all_gather_start, &output_index);
      if value_set != operand_value_set {
        //value_set = operand_value_set; // TODO
        changed = true;
      }
    }
    changed
  }

  fn update_all_gather_done_value_set(&mut self, all_gather_done: &HloInstruction) -> bool {
    assert_eq!(all_gather_done.opcode(), HloOpcode::AllGatherDone);
    let mut changed = false;

    // AllGatherDone forwards the operand value at {1} to its output. If the
    // output is a tuple, then that tuple is defined by all-gather-done, so
    // only update the value set for tuple leaf elements (arrays).
    for pair in
      self.get_mutable_instruction_value_set_pairs(all_gather_done)
    {
      let output_index = pair.0;
      let _value_set = pair.1;
      if !ShapeUtil::get_subshape(all_gather_done.shape(), output_index).is_array() {
        continue;
      }
      let mut operand_index = vec![1];
      for i in output_index {
        operand_index.push(*i);
      }
      //let operand_value_set =
        //self.get_value_set(all_gather_done.operand(0), &operand_index);
      //if value_set != operand_value_set {
        //value_set = operand_value_set; // TODO
        changed = true;
      //}
    }
    changed
  }

  fn update_all_reduce_done_value_set(&mut self, all_reduce_done: &HloInstruction) -> bool {
    assert_eq!(all_reduce_done.opcode(), HloOpcode::AllReduceDone);
    let mut changed = false;

    // AllReduceDone forwards its only operand.
    for pair in
      self.get_mutable_instruction_value_set_pairs(all_reduce_done)
    {
      let output_index = pair.0;
      let _value_set = pair.1;

      let mut operand_index = vec![];
      for i in output_index {
        operand_index.push(*i);
      }

      //let operand_value_set =
        //self.get_value_set(all_reduce_done.operand(0), &operand_index);
      //if value_set != operand_value_set {
        //value_set = operand_value_set; // TODO
        changed = true;
      //}
    }
    changed
  }

  fn update_collective_permute_start_value_set(&self, c_perm_start: &HloInstruction) -> bool {
    assert_eq!(c_perm_start.opcode(), HloOpcode::CollectivePermuteStart);
    let changed = false;

    // CollectivePermuteStart forwards the operand value to element {0} of its
    // output.
    
    // TODO
    changed
  }

  fn update_collective_permute_done_value_set(&self, _instruction: &HloInstruction) -> bool {
    unimplemented!()
  }

  fn propagate() {}

  // Returns the result of the SSA Phi function applied to the given inputs at
  // the given instruction.
  fn phi(&mut self, instruction: &HloInstruction, inputs: &Vec<InstructionValueSet>) -> bool {
    assert!(self.ssa_form);
    println!("phi({:?})", instruction.name());

    if self.bitcast_defines_value {

    } else {
        
    }

    let mut changed = false;
    for pair in
      self.get_mutable_instruction_value_set_pairs(instruction)
    {
      let index = pair.0;
      let value_set = pair.1;

      // Positions with phi values should never have more than one value in the
      // value set.
      assert!(value_set.values().len() > 1);
      let mut current_value: Option<&HloValue> = None;
      if value_set.values().len() == 1 {
        current_value = Some(&value_set.values()[0]);
      }

      // Construct a vector of value IDs of the inputs.
      let mut input_value_ids = vec![];
      for input in inputs {
        for value in input.element(index).values() {
          input_value_ids.push(value.id());
        }
      }

      // Remove the existing phi value (if it exists). The phi can be its own
      // input, for example, in while body parameters where the body passes
      // through the parameter value.
      let current_value_defined_here = current_value.is_some() &&
        current_value.unwrap().defining_instruction() == instruction &&
        current_value.unwrap().defining_index() == index;

      println!("after input_value_ids.size = {:?}", input_value_ids.len());
      if input_value_ids.is_empty() {
        // A value set which has at least one element should never have its value
        // set reduced to zero elements. During dataflow value sets only can go
        // from empty to non-empty, not the reverse.
        assert_eq!(value_set.values().len(), 0);
      } else if input_value_ids.len() == 1 {
        // Only a single value reaches this point. There should be no phi, and
        // this value set should contain this single value.
        //let new_value = self.get_value(input_value_ids[0]);
        if current_value.is_none() {
          //value_set.clear();
          //value_set.add_value(new_value);
          //changed = true;
        //} else if current_value != Some(new_value) {
          if current_value_defined_here {
            //self.mark_value_for_deletion(current_value.unwrap().id());
          }
          //value_set.clear();
          //value_set.add_value(new_value);
          changed = true;
        }
      } else {
        // Multiple distinct values reach this point. A phi value is
        // necessary.
        assert!(input_value_ids.len() > 1);
        let phi_defined_here = current_value_defined_here &&
          current_value.unwrap().is_phi();
        if current_value.is_none() || !phi_defined_here {

        } else if phi_defined_here {
            
        }
      }
    }
    changed
  }

  fn update_positions_of_values_at() {}
}