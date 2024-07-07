#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use hlo::{
  hlo_instruction::HloInstruction,
  hlo_module::HloModule,
  hlo_opcode::HloOpcode,
  hlo_value::{HloValue, HloValueSet, InstructionValueSet}
};

use crate::{call_graph::CallGraph, hlo_phi_graph::PhiGraph};

// Identifies one array input of an HloInstruction.
struct HloOperandIndex {
  operand_number: i64,
  operand_index: usize,
}

pub struct ForwardedOperand {
  operand_number: i64,
  operand_index: usize,
}

// analysis which identifies all HLO values and their uses in an HLO module.
pub struct HloDataflowAnalysis {
  call_graph: CallGraph,
  module: HloModule,
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

impl HloDataflowAnalysis {
  pub fn new(
    module: HloModule,
    ssa_form: bool,
    bitcast_defines_value: bool,
    execution_threads: HashSet<String>) -> Self
  {
    HloDataflowAnalysis {
      call_graph: CallGraph::build(&module, execution_threads.clone()),
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
    _execution_threads: &HashSet<String>) -> Result<HloDataflowAnalysis, String>
  {
    unimplemented!()
  }

  // Returns true if 'instruction' defines an HLO value at the given shape index
  // of its output.
  pub fn value_is_defined_at(
    &self, instruction: &HloInstruction, index_vec: &Vec<i64>) -> bool
  {
    let value_set = self.get_value_set(instruction, index_vec);
    if value_set.values().len() != 1 { return false; }
    value_set.get_unique_value().defining_instruction() == instruction
  }

  // Returns the HloValue defined by 'instruction' at the given shape index of
  // its output.
  pub fn get_value_defined_at(
    &self, instruction: &HloInstruction, index_vec: &Vec<i64>) -> &HloValue
  {
    debug_assert!(self.value_is_defined_at(instruction, index_vec));
    self.get_unique_value_at(instruction, index_vec)
  }

  // Returns the InstructionValueSet for the given instruction.
  pub fn get_instruction_value_set(
    &self, instruction: &HloInstruction) -> Option<&InstructionValueSet>
  {
    debug_assert!(self.value_sets.contains_key(instruction));
    self.value_sets.get(instruction)
  }

  pub fn get_mutable_instruction_value_set(
    &mut self, instruction: &HloInstruction) -> Option<&mut InstructionValueSet>
  {
    debug_assert!(self.value_sets.contains_key(instruction));
    self.value_sets.get_mut(instruction)
  }

  pub fn set_instruction_value_set(
    &mut self, instruction: HloInstruction, value_set: InstructionValueSet)
  {
    self.value_sets.insert(instruction, value_set);
  }

  // Returns all values that are contained in the output of this instruction in
  // a flattened set.
  pub fn get_flattend_value_set(&self, instruction: &HloInstruction) -> HloValueSet {
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
    _instruction: &HloInstruction,
    _index_vec: &Vec<i64>) -> &HloValueSet
  {
    //self.get_instruction_value_set(instruction).unwrap().element(index)
    unimplemented!()
  }

  pub fn get_mutable_value_set(
    &mut self, instruction: &HloInstruction, index: usize) -> &mut HloValueSet
  {
    self.get_mutable_instruction_value_set(instruction).unwrap().mutable_element(index)
  }

  // Returns the unique value in the HloValueSet at the given instruction and
  // shape index.
  pub fn get_unique_value_at(&self, instruction: &HloInstruction, index_vec: &Vec<i64>) -> &HloValue {
    self.get_value(self.get_value_set(instruction, index_vec).get_unique_value().id())
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
    unimplemented!()
  }

  // Returns true if 'user' cannot possibly use the buffer at 'index' in 'operand'.
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
                fusion_param, &vec![use_.operand_index as i64]);
            return value.get_uses().is_empty();
          }
          return false;
        }
      }
    }
    true
  }

  pub fn can_share_operand_buffer_with_user(
    &self,
    _operand: &HloInstruction,
    _operand_index: usize,
    _user: &HloInstruction,
    _user_index: usize) -> bool
  {
    unimplemented!()
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

  pub fn get_in_place_input_output_pairs() {}
  pub fn verify() {}

  fn are_transitive_uses_elementwise_or_tuple() {}
  fn optimize_phi_values() {}
  fn new_hlo_value() {}

  // Marks the HloValue with the given ID for the deletion.
  fn mark_value_for_deletion(&mut self, value_id: i64) {
    let value = self.get_value(value_id);
    print!("mark_value_for_deletion( {:?} )", value.to_short_string());
    self.value_ids_to_delete.push(value_id);
  }

  fn delete_marked_values() {}
  fn initialize_instruction_value_sets() {}
  fn update_instructionn_value_set() {}

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
      self.phi(conditional, inputs)
    } else {
      self.get_mutable_instruction_value_set(conditional)
        .as_mut().unwrap().assign_union_of(inputs)
    }
  }

  fn update_copy_value_set() {}
  fn update_custom_call_value_set() {}

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

  fn update_get_tuple_element_value_set() {}
  fn update_parameter_value_set() {}

  fn update_async_start_value_set() {}
  fn update_async_update_value_set() {}
  fn update_async_done_value_set() {}

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

  fn update_copy_done_value_set() {}
  fn update_optimization_barrier_value_set() {}
  fn update_recv_done_value_set() {}
  fn update_send_value_set() {}
  fn update_tuple_value_set() {}

  fn update_while_value_set(&mut self, while_value: &HloInstruction) -> bool {
    debug_assert!(while_value.opcode() == HloOpcode::While);
    let mut inputs = vec![];
    inputs.push(self.get_instruction_value_set(
      while_value.while_body().root_instruction()).unwrap().clone());
    inputs.push(self.get_instruction_value_set(
      while_value.operand(0)).unwrap().clone());
    
    if self.ssa_form {
      self.phi(while_value, inputs)
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

  fn update_all_gather_start_value_set() {}
  fn update_all_gather_done_value_set() {}
  fn update_all_reduce_done_value_set() {}
  fn update_collective_permute_start_value_set() {}
  fn update_collective_permute_done_value_set() {}

  fn propagate() {}

  // Returns the result of the SSA Phi function applied to the given inputs at
  // the given instruction.
  fn phi(&self, _instruction: &HloInstruction, _inputs: Vec<InstructionValueSet>) -> bool {
    unimplemented!()
  }

  fn update_positions_of_values_at() {}
}