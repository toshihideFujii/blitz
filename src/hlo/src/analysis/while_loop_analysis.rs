#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use common::{literal::Literal, shape::ShapeEqual,
  shape_util::ShapeUtil, blitz_data::PrimitiveType};

use crate::{collective_ops_utils::is_collective,
  evaluator::hlo_evaluator::{HloEvaluator, PrecomputedAnalysis},
  hlo_extractor::{extract_module, ReplaceType}, hlo_instruction::HloInstruction,
  hlo_module::HloModule, hlo_module_config::HloModuleConfig,
  hlo_opcode::HloOpcode, //utils::hlo_query::get_unique_gte_instruction
};

// Returns the precise trip count of the loop if it's statically known,
// nullopt otherwise.
//
// max_brute_force_iters limits the number of steps that are evaluated while
// trying to brute force a loop trip count. trip counts larger than
// max_brute_force_iters may be returned if we can pattern-match the loop
// condition.
pub fn compute_while_loop_trip_count(
  _while_op: &HloInstruction, _max_brute_force_iters: usize) -> Option<i64>
{
  /*
  println!("Getting trip count for loop {:?}", while_op.to_string_default());

  // The loop's induction variable is found at
  //   get-tuple-elem(comp->parameter_instruction(0), *indvar_tuple_idx),
  // where comp is while_op->while_body() or while_op->while_condition().
  let indvar_tuple_idx = get_loop_induction_var_tuple_idx(while_op);
  if indvar_tuple_idx.is_none() { return None; }

  // Now that we know the index of the induction variable, we can we can try to
  // compute how many times the loop executes.  Start by computing the induction
  // variable's initial value.
  let indvar_init_result = evaluate_indvar_init(
    while_op, indvar_tuple_idx.unwrap());
  if indvar_init_result.is_err() { return None; }
  let mut indvar_iter_val = indvar_init_result.unwrap();

  // First, try to pattern-match.
  let trip_count = match_trivial_loop_trip_count(
    while_op, indvar_tuple_idx.unwrap(), &indvar_iter_val);
  if trip_count.is_some() { return trip_count; }

  // If our pattern-match failed, try brute-forcing the loop trip count.
  let while_body = while_op.while_body();
  let while_body_indvar_update =
    while_body.root_instruction().operand(indvar_tuple_idx.unwrap() as usize);
  let while_body_indvar =
    non_constant_operand(while_body_indvar_update);
  if while_body_indvar.is_none() ||
    while_body_indvar.as_ref().unwrap() != &get_unique_gte_instruction(
      while_body.parameter_instruction(0).unwrap(),
      indvar_tuple_idx.unwrap())
  {
    return None;
  }

  let while_cond = while_op.while_condition();
  let while_cond_root = while_cond.root_instruction();
  let while_cond_indvar =
    non_constant_operand(while_cond_root);
  if while_cond_indvar.is_none() ||
    while_cond_indvar.as_ref().unwrap() != &get_unique_gte_instruction(
      while_cond.parameter_instruction(0).unwrap(),
      indvar_tuple_idx.unwrap())
  {
    return None;
  }

  let evaluator = HloEvaluator::new(0);
  for _trip_count in 0..max_brute_force_iters+1 {
    let mut while_cond_indvar_map = HashMap::new();
    while_cond_indvar_map.insert(
      while_cond_indvar.as_ref().unwrap().clone(), indvar_iter_val.clone());
    let result = evaluator.evaluate(
      while_cond_root,
      &PrecomputedAnalysis::default(),
      false,
      &while_cond_indvar_map);
    if result.is_err() {
      return None;
    }
    // Calculate the value of the induction variable after one iteration of the
    // loop, and check whether the while condition is true with this new value.
    let mut while_body_indvar_map = HashMap::new();
    while_body_indvar_map.insert(
      while_body_indvar.as_ref().unwrap().clone(), indvar_iter_val.clone());
    let indvar_next_result = evaluator.evaluate(
      while_body_indvar_update,
      &PrecomputedAnalysis::default(),
      false,
      &while_body_indvar_map);
    if indvar_next_result.is_err() {
      return None;
    }
    indvar_iter_val = indvar_next_result.unwrap();
  }

  println!("Loop has unknown trip count.");
  */
  None
}

// Returns an upper bound on the trip count of the loop if it's statically
// known, nullopt otherwise.
pub fn compute_while_loop_trip_count_upper_bound(
  while_op: &HloInstruction) -> Option<i64>
{
  let exact_trip_count = compute_while_loop_trip_count(
    while_op, 128);
  if exact_trip_count.is_some() {
    println!("Loop has exact trip count.");
    return exact_trip_count;
  }
  // There is one more case we know how to handle. If the loop condition only
  // looks at one element of the tuple, and the loop body sets this element to a
  // constant, there are two options:
  // 1) Evaluating the condition on this constant returns true. In this case,
  // the loop either executes 0 times, or is an infinite loop, depending on the
  // init value.
  // 2) Evaluating the condition on this constant returns false. In this case,
  // the loop executes 0 or 1 times, depending on the init value. This means
  // that, regardless of the init value, the upper bound on the trip count is 1.

  // Check whether the condition depends on a single parameter, and find out
  // which.
  let while_cond = while_op.while_condition();
  let while_cond_param =
    while_cond.parameter_instruction(0).unwrap();
  let cond_gte = get_only_gte(while_cond_param);
  if cond_gte.is_none() {
    println!("Induction variable not found in loop condition: {:?}",
      while_cond.root_instruction().to_string_default());
    return None;
  }
  // Now check whether this gets set to a constant by the while body.
  let while_body = while_op.while_body();
  let while_body_root = while_body.root_instruction();
  if while_body_root.opcode() != HloOpcode::Tuple {
    println!("While body's root is not a tuple instruction: {:?}",
      while_body_root.to_string_default());
    return None;
  }
  let indvar_index = cond_gte.as_ref().unwrap().tuple_index();
  let while_body_indvar =
    while_body_root.operand(indvar_index as usize);
  if while_body_indvar.opcode() != HloOpcode::Constant {
    println!("While body does not set the IV to a constant: {:?}",
      while_body_indvar.to_string_default());
    return None;
  }
  // Create a new while cond computation accessing only the single parameter
  // extracted by the GTE above to avoid excessive memory allocation for the
  // evaluator.
  let mut replacements = HashMap::new();
  let new_param = HloInstruction::create_parameter(
    0,
    &ShapeUtil::make_tuple_shape(
      vec![cond_gte.as_ref().unwrap().shape().clone()]),
    "temp".to_string());
  replacements.insert(cond_gte.unwrap().clone(),
    HloInstruction::create_get_tuple_element(&new_param, 0));
  replacements.insert(while_cond_param.clone(), new_param);
  let mut new_module = HloModule::new("temp_mod".to_string(),
  HloModuleConfig::default());
  let new_computation = new_module.add_embedded_computation(
    while_cond.clone_with_replacements(
      &replacements, &vec![], None,
      "clone".to_string(), None));

  // We have a constant. Evaluate the condition on this constant.
  let evaluator: HloEvaluator = HloEvaluator::new(0);
  let mut fake_input = Literal::create_from_shape(
    new_computation.parameter_instruction(0).unwrap().shape());
  let result = fake_input.copy_from(
    while_body_indvar.mutable_literal(),
    &vec![0],
    &vec![],
    false);
  debug_assert!(result.is_ok());

  let eval_result = evaluator.evaluate_computation(
    new_computation, &vec![fake_input]);
  if eval_result.is_err() {
    println!("Couldn't evaluate while loop condition.");
    return None;
  }

  let cond_result_pred = eval_result.unwrap();
  let result = ShapeEqual::new().ignore_layout().equal(
    cond_result_pred.shape(), &ShapeUtil::make_shape(
      &PrimitiveType::Pred, vec![]));
  debug_assert!(result);

  // Per the explanation above, if the evaluated condition returns false, the
  // loop executes at most once.
  //let cond_returns_true = cond_result_pred.get_first_element();
  //if !*cond_returns_true {
    //println!("Upper bound on the trip count is 1");
    //return Some(1);
  //}

  println!("Loop has no known upper bound on the trip count.");
  None
}

// The below function identifies a subset of all possible auxiliary
// induction variables (AIV). Specifically, candidates are gtes, e.g.,
// gte(param0, N)
// The function checks if the loop body plumbs the AIV
// through the same tuple index at root, and that ops involving AIV
// involve constants.
//   op2 = op(constants, gte(param0, N), constants)
//   op3 = op(constants, f(op2, gte(param0, N), constants)
//   op4 = op(constants, f(op3, constants)
//   root = tuple(..., op4, ...)
// Further, the ops are restricted to basic math ops (+,-,*,/).
// Finally, loop invariant GTEs are excluded from AIVs.
// We can expand the ops category/nature of AIVs as needed.
pub fn get_auziliary_loop_induction_vars(
  _while_op: &HloInstruction) -> &Vec<HloInstruction>
{
  unimplemented!()
}

// Tries to get the tuple index of the induction variable of a while loop.
//
// Checks that the loop condition and body both plumb the induction variable
// through the same tuple index, and that they both apply exactly one op to the
// induction variable before  deciding whether to do another loop iteration (in
// the loop condition's case) or packing the induction variable into the result
// tuple (in the loop body's case).
//
// Specifically, checks that the loop condition has structure
//
//   root = op(constants, get-tuple-elem(param0, N), constants)
//
// and the loop body has the structure
//
//   inc = op(constants, get-tuple-elem(param0, N), constants)
//   root = tuple(..., inc, ...)  // inc is N'th operand of tuple().
//
// If so, returns N.  Otherwise, returns nullopt.
pub fn get_loop_induction_var_tuple_idx(
  while_op: &HloInstruction) -> Option<i64>
{
  get_loop_induction_var_tuple_idx_with_known_values(
    while_op, HashMap::new())
}

// Same as above, but also handles cases where the range of the induction
// variable depends on previously known values rather than constants.
fn get_loop_induction_var_tuple_idx_with_known_values(
  _while_op: &HloInstruction,
  _known_values: HashMap<HloInstruction, i64>) -> Option<i64>
{
  unimplemented!()    
}

// Checks the following conditions:
//  - `i`, the induction variable, is initialized to a scalar constant K
//    (namely, `indvar_init`),
//  - the while condition does `i < N` or `i <= N` (where N is a known constant)
//  - the while body does `i += C` (where C is a positive constant)
// If so, it's trivial to compute the loop bound as `(N - K) div C` or
// `(N - K + 1) div C`, respectively.
pub fn match_trivial_loop_trip_count(
  _while_op: &HloInstruction,
  _indvar_tuple_idx: i64,
  _indvar_init: &Literal) -> Option<i64>
{
  //let indvar_init_val = LiteralUtil::lite
  unimplemented!()    
}

// Same as above, but returns the loop range, i.e., start (inclusive), end
// (inclusive) and step instead of the trip count.
pub fn match_trivial_loop_range(_while_op: &HloInstruction) {
  unimplemented!()
}

// Traces through a chain of copy instructions and a GTE-tuple pair until a
// non-copy or a non-GTE-tuple pair is found.
fn trace_through_copy_and_tuple_chain(instr: &HloInstruction) -> HloInstruction {
  if instr.opcode() == HloOpcode::GetTupleElement &&
    instr.operand(0).opcode() == HloOpcode::Tuple
  {
    return trace_through_copy_and_tuple_chain(
      instr.operand(0).operand(instr.tuple_index() as usize));
  }
  if instr.opcode() == HloOpcode::Copy ||
    instr.opcode() == HloOpcode::CopyStart ||
    instr.opcode() == HloOpcode::CopyDone
  {
    return trace_through_copy_and_tuple_chain(instr.operand(0));
  }
  instr.clone()
}

// Finds and returns the non-constant operand in instr, if there is only one
// such operand.
//
// Returns nullptr if instr doesn't have exactly one unique non-constant
// operand.
fn non_constant_operand(instr: &HloInstruction) -> Option<HloInstruction> {
  let mut result = None;
  for operand in instr.operands() {
    if result.is_some() && &result.unwrap() != operand {
      return None;
    }
    result = Some(operand.clone());
  }
  result
}

// If all of instr's operands are either constants or have the form
//   get-tuple-element(gte_operand, x),
// then returns a vector of all such x. Otherwise, returns nullopt.
fn get_gte_operand_indices(
  instr: &HloInstruction, gte_operand: &HloInstruction) -> Option<HashSet<i64>>
{
  let mut tuple_indices = HashSet::new();
  for operand in instr.operands() {
    if operand.opcode() == HloOpcode::Constant {
      continue;
    }
    let mut possibly_gte = operand;
    if operand.opcode() == HloOpcode::Copy {
      possibly_gte = operand.operand(0);
    }
    if possibly_gte.opcode() != HloOpcode::GetTupleElement {
      return None;
    }
    if possibly_gte.operand(0) != gte_operand {
      return None;
    }
    let operand_tuple_id = possibly_gte.tuple_index();
    tuple_indices.insert(operand_tuple_id);
  }
  Some(tuple_indices)
}

// This function returns true if the operation is a simple scalar operation.
// While loop analysis can execute such an operation at compile time without
// incurring huge overheads.
fn is_scalar_op(op: &HloInstruction) -> bool {
  if is_collective(op) {
    return false;
  }
  match op.opcode() {
    HloOpcode::Send => return false,
    HloOpcode::SendDone => return false,
    HloOpcode::Recv => return false,
    HloOpcode::RecvDone => return false,
    HloOpcode::CustomCall => return false,
    _ => println!("Do nothing."),
  }
  for comp in op.called_computations() {
    for inst in comp.instructions() {
      if !is_scalar_op(inst) { return false; }
    }
  }
  ShapeUtil::is_scalar(op.shape())
}

// If `out` is a function of a some values in the tuple `in` and has no other
// dependence, i.e. if `out=f(gte1(in), gte2(in),...)`, then this function will
// return the all the get-tuple-element indices for the dependence.
//
// For example, in the following HLO, this function will return `1`:
//   in = (s32[], s32[], s32[]) tuple(a,b,c)
//   gte.1 = get-tuple-element(in), index=1
//   out = fusion(gte.1), ...
// Also checks whether all ops on the path from `in` to `out` are ops with a
// scalar shape.
fn get_gte_dependence_indices(
  out: &HloInstruction, inn: &HloInstruction) -> Option<HashSet<i64>>
{
  let tuple_idxs =
    get_gte_operand_indices(out, inn);
  if tuple_idxs.is_some() {
    return tuple_idxs;
  }
  if out.parent() != inn.parent() || !inn.shape().is_tuple() {
    return None;
  }

  // Extracts the instruction `out` as a function of the instruction `in`.
  // HloModule extracted
  // ENTRY main {
  //   in = parameter(0)
  //   //... some calculations
  //   ROOT out = ...
  // }
  let extract_selector =
    |inst: &HloInstruction| -> bool {
    inst != inn
  };
  let replace_type_selector =
    |_inst: &HloInstruction| -> ReplaceType {
    ReplaceType::ReplaceParam
  };
  let extracted = extract_module(
    out,
    -1,
    &extract_selector,
    &replace_type_selector,
    false,
    true,
    false);

  let entry = extracted.entry_computation().unwrap();
  // Check that the extracted module takes nothing but `in` as input. If `out`
  // does not depend on in, the extracted module will have some other shape for
  // input.
  if entry.num_parameters() != 1 ||
    entry.parameter_instruction(0).unwrap().shape() != inn.shape()
  {
    return None;
  }
  let param = entry.parameter_instruction(0).unwrap();
  // If there are no users for the input `in`, it would mean that `out` does not
  // depend on a get-tuple-element of `in`.
  if param.user_count() == 0 {
    return None;
  }
  // If any of the users of the input `in` is not a get-tuple-element
  // instruction, then that would mean that the output does not depend uniquely
  // on a get-tuple-element of on `in`, instead it depends on some other
  // calculations on `in`.
  for inst in param.users() {
    if inst.opcode() != HloOpcode::GetTupleElement { return None; }
  }
  // At this point we already know that the all the users are get-tuple-elements
  // and that there is at least one user. Now, extract all indices of the users.
  let mut candidate_indices = HashSet::new();
  for user in param.users() {
    candidate_indices.insert(user.tuple_index());
  }

  for inst in entry.instructions() {
    if inst.opcode() != HloOpcode::Parameter && !is_scalar_op(inst) { return None; }   
  }
  
  Some(candidate_indices)
}

// If `out` is a function of a single value in the tuple `in` and has no other
// dependence, i.e. if `out=f(gte(in))`, then this function will return the
// unique get-tuple-element index for the dependence.
//
// For example, in the following HLO, this function will return `1`:
//   in = (s32[], s32[], s32[]) tuple(a,b,c)
//   gte.1 = get-tuple-element(in), index=1
//   out = fusion(gte.1), ...
// Also checks whether all ops on the path from `in` to `out` are ops with a
// scalar shape.
fn get_unique_gte_dependence_index(
  out: &HloInstruction, inn: &HloInstruction) -> Option<i64>
{
  let tuple_idxs = get_gte_dependence_indices(out, inn);
  if tuple_idxs.is_none() {
    return None;
  }
  if tuple_idxs.as_ref().unwrap().len() != 1 {
    return None;
  }
  Some(*tuple_idxs.as_ref().unwrap().iter().next().unwrap())
}

// Computes a + b, returning nullopt if it overflows.
fn checked_add(a: i64, b: i64) -> Option<i64> {
  let aa = a as u64;
  let bb = b as u64;
  let result = (aa + bb) as i64;

  let a_ge_0 = a >= 0;
  let b_ge_0 = b >= 0;
  let result_ge_0 = result >= 0;

  if a_ge_0 == b_ge_0 && result_ge_0 != a_ge_0 {
    return None;
  }
  Some(result)
}

// Computes a - b, returning nullopt if it overflows.
fn checked_subtract(a: i64, b: i64) -> Option<i64> {
  let aa = a as u64;
  let bb = b as u64;
  let result = (aa - bb) as i64;

  let a_ge_0 = a >= 0;
  let b_ge_0 = b >= 0;
  let result_ge_0 = result >= 0;

  if a_ge_0 != b_ge_0 && result_ge_0 == b_ge_0 {
    return None;
  }
  Some(result)
}

fn evaluate_indvar_init(
  while_op: &HloInstruction,
  indvar_tuple_idx: i64) -> Result<Literal, String>
{
  let evaluator = HloEvaluator::new(0);
  let while_init = while_op.operand(0);
  let indvar_init = while_init.operand(indvar_tuple_idx as usize);
  let value = trace_through_copy_and_tuple_chain(indvar_init);
  evaluator.evaluate(&value,
    &PrecomputedAnalysis::default(),
    false,
  &HashMap::new())
}

fn match_loop_range_with_known_values(
  _while_op: &HloInstruction,
  _induction_var_idx: i64,
  _known_values: &HashMap<HloInstruction, i64>) -> Option<i64>
{
  unimplemented!()    
}

// If the only user of this instruction is a get-tuple-element, return that
// get-tuple-element, otherwise return null. If this runs before CSE/DCE, we may
// get a false negative if there are several copies of the same GTE, or there
// are unused GTEs, but we can live with this.
fn get_only_gte(inst: &HloInstruction) -> Option<HloInstruction> {
  if inst.user_count() != 1 {
    return None;
  }
  let user = inst.users().last();
  if user.unwrap().opcode() != HloOpcode::GetTupleElement {
    return None;
  }
  Some(user.unwrap().clone())
}