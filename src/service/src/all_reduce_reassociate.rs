#![allow(dead_code)]

use std::collections::HashSet;
use common::{literal::Literal, primitive_util::cast_preserves_values, shape_util::ShapeUtil};
use hlo::{hlo_instruction::HloInstruction, hlo_module::HloModule, hlo_opcode::HloOpcode,
  utils::hlo_query::{contains_layout_constrained_all_reduce, next_channel_id}};

use crate::{all_reduce_key::{get_all_reduce_key, AllReduceKey},
collective_ops_utils::{get_reduction_identity, match_reduction_computation,
  match_reduction_instruction, ReductionKind}};

// A pass that reassociates all-reduce feeding into compatible elementwise
// operations. As an example: add(all-reduce(x), all-reduce(y)) will be replaced
// with all-reduce(add(x,y)). Mathematically, this is replacing
//   add(x0, x1, ... xk) + add(y0, y1, ... yk) with
//   add((x0+y0), (x1+y), ... (xk+yk)
//
//  i.e., reassociating the reduction operation.
pub struct AllReduceReassociate {
  reassociate_converted_ar: bool
}

impl AllReduceReassociate {
  pub fn new(reassociate_converted_ar: bool) -> Self {
    AllReduceReassociate {
      reassociate_converted_ar: reassociate_converted_ar
    }
  }

  pub fn name(&self) -> String {
    "all-reduce-reassociate".to_string()
  }

  pub fn run(
    &self,
    module: &mut HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  { 
    if contains_layout_constrained_all_reduce(module) {
      let err_msg = "Akip all_reduce_reassociate because the
        module contains all-reduce with constrained layouts".to_string();
      return Err(err_msg);
    }

    let _next_channel_id = next_channel_id(module);
    let mut changed = false;
    for comp in
      module.mutable_computations_by_exec_threads(execution_threads)
    {
      for inst in comp.mutable_make_instruction_post_order() {
        // Check if the instruction we want to reassociate with will match any
        // valid all-reduce reduction function. Save the ReductionKind object for
        // later.
        let kind = match_reduction_instruction(inst);
        if kind.is_none() { continue; }

        let reduction_identity = // TODO
          get_reduction_identity::<i64>(&kind.as_ref().unwrap(), &inst.shape().element_type());
        // Unsupported reduction type.
        if reduction_identity.is_none() { continue; }

        // Find LHS all-reduce.
        let mut lhs = look_through_for_all_reduce(
          inst.mutable_operand(0).unwrap(),
          reduction_identity.as_ref().unwrap());
        if lhs.is_none() { continue; }

        // Find RHS all-reduce.
        let mut rhs = look_through_for_all_reduce(
          inst.mutable_operand(1).unwrap(),
          reduction_identity.as_ref().unwrap());
        if rhs.is_none() { continue; }

        if !inst.shape().is_array() { continue; }

        if lhs.as_ref().unwrap().opcode() != rhs.as_ref().unwrap().opcode() ||
          lhs.as_ref().unwrap().opcode() == HloOpcode::DynamicSlice &&
          !ShapeUtil::compatible(lhs.as_ref().unwrap().operand(0).shape(),
          rhs.as_ref().unwrap().operand(0).shape())
        {
          continue;
        }
        #[allow(unused_assignments)]
        let mut ar0: Option<&mut HloInstruction> = None;
        #[allow(unused_assignments)]
        let mut ar1: Option<&mut HloInstruction> = None;
        let mut reduce_scatter_pattern_match = false;
        if lhs.as_ref().unwrap().opcode() == HloOpcode::DynamicSlice {
          //let original_rhs_operand =
            //rhs.as_mut().unwrap().mutable_operand(0);
          //let result = rhs.as_ref().unwrap().replace_operand_with(
            //0,
            //lhs.as_ref().unwrap().mutable_operand(0).unwrap().clone());
          //check_error(&result);
          ar0 = lhs.as_mut().unwrap().mutable_operand(0);
          ar1 = rhs.as_mut().unwrap().mutable_operand(1);
          reduce_scatter_pattern_match = true;
        } else {
          ar0 = Some(lhs.as_mut().unwrap());
          ar1 = Some(rhs.as_mut().unwrap());
        }
        // Because we look through pads it might not be profitable to actually
        // reassociate if reassociating makes us all-reduce more values.
        //if !reassociate_all_reduce_is_profitable(
          //&lhs.as_ref().unwrap(), &rhs.as_ref().unwrap(), &inst)
        //{
          //continue;
        //}
        let mut convert0: Option<&mut HloInstruction> = None;
        let mut convert1: Option<&mut HloInstruction> = None;
        if !match_operands_to_all_reduce_with_optional_convert(
          inst, convert0.as_ref().unwrap(), convert1.as_ref().unwrap())
        {
          println!("One or both inputs are type-converted.");
        }
        // Check to see if input converts are present and preserving values and
        // precision.
        let should_promote_ar = convert0.is_some() || convert1.is_some();
        if should_promote_ar {
          if !self.reassociate_converted_ar {
            println!("Pro,otion of all_reduces for reassociation will be disabled.");
            continue;
          }
          if !are_compatible_converts(&convert0, &convert1) {
            println!("Inputs Converts are not preserving value, skipping");
            continue;
          }
        }

        let mut _op_operand0 = inst.mutable_operand(0);
        let mut _op_operand1 = inst.mutable_operand(1);
        if convert0.is_some() {
          _op_operand0 = convert0.as_mut().unwrap().mutable_operand(0);
        }
        if convert1.is_some() {
          _op_operand1 = convert1.as_mut().unwrap().mutable_operand(0);
        }
        if !are_compatible(ar0.as_ref().unwrap(), ar1.as_ref().unwrap(),
          kind.as_ref().unwrap().clone(),
          should_promote_ar)
        {
          println!("all-reduce operations are not compatible, skipping");
          continue;
        }
        println!("reassociated:");
        println!("  AR0:{:?}", ar0.as_ref().unwrap().to_string_default());
        println!("  AR1:{:?}", ar1.as_ref().unwrap().to_short_string());

        let _op_users = inst.users();
        let mut _new_op_operand0 =
          ar0.as_mut().unwrap().mutable_operand(0);
        let mut _new_op_operand1 =
          ar1.as_mut().unwrap().mutable_operand(1);
        if convert0.is_some() {
          let ar0_operand =
            ar0.as_mut().unwrap().mutable_operand(0);
          let result = convert0.as_mut().unwrap().replace_operand_with(
            0,
            ar0_operand.unwrap().clone());
          check_error(&result);
          _new_op_operand0 = convert0;
        }
        if convert1.is_some() {
          let ar1_operand =
            ar1.as_mut().unwrap().mutable_operand(0);
          let result = convert1.as_mut().unwrap().replace_operand_with(
            0,
            ar1_operand.unwrap().clone());
          check_error(&result);
          _new_op_operand1 = convert1;
        }

        let _new_op = inst.clone();
        if should_promote_ar {
          //new_op = comp.add_instruction(
            //inst.clone_with_new_opereands(
              //inst.shape(),&vec![
                //new_op_operand0.unwrap().clone(),
                //new_op_operand1.unwrap().clone()]),
            //"".to_string()).clone();
        } else if reduce_scatter_pattern_match {
          //new_op = comp.add_instruction(
            //inst.clone_with_new_opereands(
              //ar0.as_ref().unwrap().shape(),&vec![
                //new_op_operand0.unwrap().clone(),
                //new_op_operand1.unwrap().clone()]),
            //"".to_string()).clone();
        }

        let mut _new_ar_out_shape = inst.shape();
        debug_assert!(!should_promote_ar || !reduce_scatter_pattern_match);
        if should_promote_ar {

        } else if reduce_scatter_pattern_match {
          _new_ar_out_shape = ar0.as_ref().unwrap().shape();
        } else {
          //let result = ar0.as_mut().unwrap().replace_all_uses_with(
            //ar0.as_mut().unwrap().mutable_operand(0).unwrap(),
            //"".to_string());
          //check_error(&result);
          //let result = ar1.as_mut().unwrap().replace_all_uses_with(
            //ar1.as_mut().unwrap().mutable_operand(0).unwrap(),
            //"".to_string());
          //check_error(&result);
        }

        if should_promote_ar {

        } else if reduce_scatter_pattern_match {
            
        } else {
            
        }

        // Note that RemoveInstructionAndUnusedOperands may not remove the 2
        // all-reduce operands of `inst` if they are not safe to remove otherwise,
        // so manually these instructions.
        if should_promote_ar || reduce_scatter_pattern_match {

        }
        if reduce_scatter_pattern_match {

        }
        //let result = comp.remove_instruction(ar0.as_ref().unwrap());
        //check_error(&result);
        if ar0.as_ref().unwrap() != ar1.as_ref().unwrap() {
          //let result = comp.remove_instruction(ar1.as_ref().unwrap());
        //check_error(&result);
        }
        changed = true;
      }
    }
    Ok(changed)
  }
}

fn are_all_reduce_keys_equal(
  key0: &AllReduceKey,
  key1: &AllReduceKey,
  ignore_element_type: bool) -> bool
{
  if ignore_element_type {
    key0.0 == key1.0 && key0.1 == key1.1 && key0.2 == key1.2 &&
    key0.3 == key1.3 && key0.4 == key1.4 && key0.5 == key1.5
  } else {
    key0 == key1
  }
}

// Returns if the given all reduce instructions are compatible with each other.
// Note that since the given all-reduce instructions are connected to another
// instruction by a direct data flow edge, they must belong to the same domain.
// As a result, we don't need to include any domain information in the
// AllReduceKey to check compatibility.
fn are_compatible(
  ar0: &HloInstruction,
  ar1: &HloInstruction,
  op_kind: ReductionKind,
  ignore_element_type: bool) -> bool
{
  let key0 = get_all_reduce_key(
      ar0, None, false);
  let key1 = get_all_reduce_key(
    ar1, None, false);
  
  let kind0 = match_reduction_computation(ar0.to_apply());
  key0.is_some() && key1.is_some() &&
  are_all_reduce_keys_equal(&key0.unwrap(), &key1.unwrap(), ignore_element_type) &&
  kind0.unwrap() == op_kind
}

// Look-through some formatting operations that might be in front of the
// all-reduces we want to reassociate. Making sure the chain only has 1 user
// throughout. Also check for possible reduce-scatter patterns (all-reduce +
// dynamic-slice).
fn look_through_for_all_reduce<T>(
  mut instr: &mut HloInstruction,
  reduction_identuty: &Literal<T>) -> Option<HloInstruction>
  where T: Default + Clone + PartialEq
{
  if instr.opcode() == HloOpcode::DynamicSlice {
    if instr.operand(0).opcode() != HloOpcode::AllReduce ||
      instr.operand(0).user_count() != 1 || instr.user_count() != 1
    {
      return None;
    }
    return Some(instr.clone());
  }
  while instr.opcode() != HloOpcode::AllReduce {
    if instr.user_count() != 1 {
      return None;
    }
    if instr.opcode() != HloOpcode::Reshape && instr.opcode() != HloOpcode::Pad &&
      instr.opcode() != HloOpcode::Slice && instr.opcode() != HloOpcode::Convert
    {
      return None;
    }
    if instr.opcode() == HloOpcode::Pad {
      if !instr.operand(1).is_constant() { return None; }
      if instr.operand(1).literal() != reduction_identuty { return None; }
    }
    instr = instr.mutable_operand(0).unwrap();
  }
  if instr.user_count() != 1 {
    return None;
  }
  Some(instr.clone())
}

// Because we can look through pads its possible that reassociating the
// all-reduce makes us reduce over more than the sum of the two unpadded
// individual all-reduces. Check that's not the case.
fn reassociate_all_reduce_is_profitable(
  ar0: &HloInstruction,
  ar1: &HloInstruction,
  reassociated_inst: &HloInstruction) -> bool
{
  let mut pre_reassociated_size = ShapeUtil::elements_in(ar0.shape());
  if ar0 != ar1 {
    pre_reassociated_size += ShapeUtil::elements_in(ar1.shape());
  }
  pre_reassociated_size >= ShapeUtil::elements_in(reassociated_inst.shape())
}

fn are_compatible_converts(
  convert0: &Option<&mut HloInstruction>,
  convert1: &Option<&mut HloInstruction>) -> bool
{
  let mut is_compatible = false;
  // For numerical stability, we only re-order ops with casts from a narrow type
  // to a wider type.
  if convert0.is_some() {
    is_compatible &= cast_preserves_values(
      &convert0.as_ref().unwrap().operand(0).shape().element_type(),
        &convert1.as_ref().unwrap().shape().element_type());
  }
  if convert1.is_some() {
    is_compatible &= cast_preserves_values(
      &convert1.as_ref().unwrap().operand(0).shape().element_type(),
      &convert1.as_ref().unwrap().shape().element_type());
  }
  if convert0.is_some() && convert1.is_some() {
    debug_assert!(convert0.as_ref().unwrap().shape().element_type() ==
      convert1.as_ref().unwrap().operand(0).shape().element_type());
    is_compatible &=
      convert0.as_ref().unwrap().shape().element_type() ==
      convert1.as_ref().unwrap().operand(0).shape().element_type()
  }
  is_compatible
}

fn optional_convert_with_one_user() {}

fn match_operands_to_all_reduce_with_optional_convert(
  _inst: &HloInstruction,
  _convert0: &HloInstruction,
  _convert1: &HloInstruction) -> bool
{
  unimplemented!()
}

fn check_error<T>(value: &Result<T, String>) {
  if value.is_err() {
    let err_msg = value.as_ref().err().unwrap();
    assert!(false, "{:?}", err_msg);
  }
}