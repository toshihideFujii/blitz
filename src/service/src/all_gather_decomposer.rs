#![allow(dead_code)]

use std::collections::HashSet;

use common::{
  blitz_data::PrimitiveType,
  //literal_util::LiteralUtil,
  shape::Shape
};
use hlo::{hlo_computation::HloComputation, hlo_instruction::HloInstruction,
  hlo_module::HloModule, hlo_opcode::HloOpcode};

use crate::{
  //collective_decomposer_utils::{create_start_indices_for_collective_decomposition},
  collective_ops_utils::{get_collective_op_group_mode,
  CollectiveOpGroupMode
}};

// AllGatherDecomposer is a pass which converts unsupported all-gathers into
// dynamic-update-slices and all-reduces.
pub struct AllGatherDecomposer {}

impl AllGatherDecomposer {
  pub fn new() -> Self {
    AllGatherDecomposer {  }
  }

  pub fn name() -> String {
    "all_gather_decomposesr".to_string()
  }

  // Run AllGatherDecomposer pass on computations in 'module'.
  // Returns whether the 'module' was changed.
  pub fn run(
    &self,
    module: &mut HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    let mut changed = false;
    for comp in
      module.mutable_make_nonfusion_computations(execution_threads)
    {
      for hlo in comp.mutable_make_instruction_post_order() {
        if hlo.opcode() != HloOpcode::AllGather {
          continue;
        }
        if self.should_decompose(hlo) {
          //let result = self.decompose_all_gather(hlo, comp);
          //check_error(&result);
          changed = true;
        }
      }    
    }
    Ok(changed)
  }

  fn translate_all_gather_to_all_reduce_per_operand(
    &self,
    _group_mode: &CollectiveOpGroupMode,
    _ag: &HloInstruction,
    _output_shape: &Shape,
    _operand: &HloInstruction,
    _comp: &mut HloComputation,
    _ag_dim: i64) -> HloInstruction
  {
    /*
    let start_indices =
      create_start_indices_for_collective_decomposition(
        group_mode, ag.replica_groups(), operand.shape(),
        ag_dim, comp, None);
    check_error(&start_indices);

    let mut zero = comp.add_instruction(
      HloInstruction::create_constant::<i64>(
        LiteralUtil::zero(output_shape.element_type())),
        "".to_string());
    
    zero = comp.add_instruction(HloInstruction::create_broadcast(
      output_shape, zero.clone(), vec![]),
      "".to_string());

    let dus = comp.add_instruction(
      HloInstruction::create_dynamic_update_slice(
        zero.shape(), zero, operand, start_indices.unwrap()),
        "".to_string());

    let ar = comp.add_instruction(
      HloInstruction::create_all_reduce(
        dus.shape(),
        &vec![dus.clone()],
        &make_binary_add(
          &dus.shape().element_type(), comp.parent().as_ref().unwrap()),
          ag.device_list(),
        ag.constrain_layout(),
        ag.channel_id(),
        ag.use_global_device_ids()),
        "".to_string());
    ar.clone()
    */
    unimplemented!()
  }

  fn should_decompose(&self, ag: &HloInstruction) -> bool {
    should_decompose_func(ag)
  }

  fn decompose_all_gather(
    &self, ag: &mut HloInstruction, comp: &mut HloComputation) -> Result<(), String>
  {
    let group_mode =
      get_collective_op_group_mode(
        ag.channel_id().is_some(),
        Some(ag.use_global_device_ids()));
    check_error(&group_mode);

    if ag.operand_count() > 1 {
      let mut tuple_inputs = vec![];
      for i in 0..ag.operand_count() {
        let input_operand = ag.operand(i);
        let output_shape = ag.shape().tuple_shapes(i);
        let ar = self.translate_all_gather_to_all_reduce_per_operand(
          group_mode.as_ref().unwrap(), ag, output_shape, input_operand,
          comp, ag.all_gather_dimension());
        tuple_inputs.push(ar);
      }
    } else {
        let mut ar = self.translate_all_gather_to_all_reduce_per_operand(
          group_mode.as_ref().unwrap(), ag, ag.shape(), ag.operand(0),
          comp, ag.all_gather_dimension());
        let result =
          ag.replace_all_uses_with(&mut ar, "".to_string());
        check_error(&result);
    }
    let result =
      comp.remove_instruction_and_unused_operands(ag);
    check_error(&result);
    Ok(())
  }
}

fn should_decompose_func(_ag: &HloInstruction) -> bool {
  false
}

// Creates a computation of x + y.
fn make_binary_add(_t: &PrimitiveType, _module: &HloModule) -> HloComputation {
  unimplemented!()
}

fn check_error<T>(value: &Result<T, String>) {
  if value.is_err() {
    let err_msg = value.as_ref().err().unwrap();
    assert!(false, "{:?}", err_msg);
  }
}