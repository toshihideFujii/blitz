#![allow(dead_code)]

use std::collections::HashMap;

use common::{
  blitz_data::PrimitiveType, literal::Literal, shape::ShapeEqual,
  shape_util::ShapeUtil
};
use hlo::{
  evaluator::hlo_evaluator::HloEvaluator, hlo_instruction::HloInstruction,
  hlo_module::HloModule, hlo_module_config::HloModuleConfig, hlo_opcode::HloOpcode
};

// Finds and returns the non-constant operand in instr.
// CHECK-fails if instr doesn't have exactly one unique non-constant operand.
fn non_constant_operand(instr: &HloInstruction) -> &HloInstruction {
  let mut result: Option<&HloInstruction> = None;
  for operand in instr.operands() {
    if !operand.is_constant() {
      if result.is_some() {
        assert_eq!(result.unwrap(), operand);
      }
      result = Some(operand);
    }
  }
  assert!(result.is_some());
  result.unwrap()
}

// If all of instr's operands are either constants or have the form
//   get-tuple-element(gte_operand, N)
// for the same value N, returns N.  Otherwise, returns nullopt.
fn get_gte_operand_index(
  instr: &HloInstruction, gte_operand: &HloInstruction) -> Option<i64>
{
  println!("get_gte_operand_index({:?}. {:?})",
    instr.to_string_default(), gte_operand.to_string_default());

  // All operands of `instr` must be either constants or of the form
  //   get-tuple-element(gte_operand, tuple_idx)
  // for the same value tuple_idx. We also support the case where GTE feeds a
  // copy that is then used.
  let mut tuple_idx: Option<i64> = None;
  for operand in instr.operands() {
    // TODO

    let mut possibly_gte_operand = operand;
    if operand.opcode() == HloOpcode::Copy {
      possibly_gte_operand = operand.operand(0);
    }

    if possibly_gte_operand.opcode() != HloOpcode::GetTupleElement {
      return None;
    }

    // TODO

    let operand_tuple_idx = possibly_gte_operand.tuple_index();
    // This is the first GTE we are seeing. Set tuple_idx.
    if tuple_idx.is_none() {
      tuple_idx = Some(operand_tuple_idx);
    } else {
      if operand_tuple_idx != tuple_idx.unwrap() {
        return None;
      }
    }
  }

  tuple_idx
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
pub fn get_auxiliary_loop_induction_vars(_while_op: &HloInstruction) -> &Vec<HloInstruction> {
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
pub fn get_loop_induction_var_tuple_idx(while_op: &HloInstruction) -> Option<i64> {
  assert_eq!(while_op.opcode(), HloOpcode::While);
  println!("Finding induction variable for loop {:?}", while_op.to_short_string());

  unimplemented!()
}

// Computes a + b, returning nullopt if it overflows.
fn checked_add(a: i64, b: i64) -> Option<i64> {
  // Overflow occurred iff `a` and `b` have the same sign and `a + b` has a
  // different sign, see Hacker's Delignt 2nd Ed. pp 28.
  let aa = a as u64;
  let bb = b as u64;
  let result = (aa + bb) as i64;
  if (a >= 0) == (b >= 0) && (result >= 0) != (a >= 0) {
    return None;
  }
  Some(result)
}

fn check_subtract(a: i64, b: i64) -> Option<i64> {
  let aa = a as u64;
  let bb = b as u64;
  let result = (aa - bb) as i64;
  // Overflow occurred iff `a` and `b` have different signs and the sign of
  // `a - b` is the same as that of `b`, see Hacker's Delight 2nd Ed. pp 29.
  if (a >= 0) != (b >= 0) && (result >= 0) == (b >= 0) {
    return None;
  }
  Some(result)
}

// Checks the following conditions:
//  - `i`, the induction varaiable, is initialized to a scalar constant K
//    (namely, `indvar_init`),
//  - the while condition does `i < N` or `i <= N` (where N is a know constant)
//  - the while body does `i++`.
// If so, it's trivial to compute the loop bound as `N - k` or `N - k + 1`,
// respectively.
pub fn match_trivial_loop_trip_count<T>(
  _while_op: &HloInstruction,
  _indvar_tuple_idx: i64,
  _indvar_init: Literal<T>) -> Option<i64>
  where T: Clone + Default + PartialEq
{
  unimplemented!()
}

// Returns the precise trip count of the loop if it's statically known,
// nullopt otherwise.
//
// max_brute_force_iters limits the number of steps that are evaluated while
// trying to brute force a loop trip count. trip counts larger than
// max_brute_force_iters may be returned if we can pattern-match the loop
// condition.
pub fn compute_while_loop_trip_count(
  while_op: &HloInstruction, max_brute_force_iters: i64) -> Option<i64>
{
  println!("Getting trip count for loop {:?}", while_op.to_string_default());

  // The loop's induction variable is found at
  //   get-tuple-elem(comp->parameter_instruction(0), *indvar_tuple_idx),
  // where comp is while_op->while_body() or while_op->while_condition().
  let indvar_tuple_idx = get_loop_induction_var_tuple_idx(while_op);
  if indvar_tuple_idx.is_none() {
    return None;
  }

  // Now that we know the index of the induction variable, we can try to
  // compute how many times the loop executes.  Start by computing the induction
  // variable's initial value.
  let evaluator = HloEvaluator::new(0);
  let while_init = while_op.operand(0);
  let indvar_init = while_init.operand(indvar_tuple_idx.unwrap() as usize);
  let indvar_init_result =
    evaluator.evaluate(indvar_init, false);
  if indvar_init_result.is_err() {
    println!("Couldn't evaluate induction variable init, {:?}, {:?}",
      indvar_init_result.err().unwrap(),
      indvar_init.to_string_default());
    return None;
  }
  //let indvar_iter_val = indvar_init_result.

  // First, try to pattern-match.
  let trip_count = match_trivial_loop_trip_count(
    while_op,
    indvar_tuple_idx.unwrap(),
    indvar_init_result.unwrap());
  if trip_count.is_some() {
    return trip_count;
  }

  // If our pattern-match failed, try brute-forcing the loop trip count.
  let while_body = while_op.while_body();
  let while_body_indvar_update =
    while_body.root_instruction().operand(indvar_tuple_idx.unwrap() as usize);
  let _while_body_indvar =
    non_constant_operand(while_body_indvar_update);

  let while_cond = while_op.while_condition();
  let while_cond_root = while_cond.root_instruction();
  let _while_cond_indvar = non_constant_operand(while_cond_root);

  for trip_count in 0..max_brute_force_iters + 1 {
    let map: HashMap<HloInstruction, Literal<bool>> = HashMap::new();
    //map.insert(while_cond_indvar.clone(), indvar_init_result.unwrap());
    let result =
      evaluator.evaluate_with_substitutions(while_cond_root, map); 
    if result.is_err() {
      println!("Couldn't evaluate while cond: {:?}", result.err().unwrap());
      return None;
    }
    for val in result.unwrap().data_default() {
      // TODO
      if *val == true {
        println!("Loop has static trip count of {:?}", trip_count);
        return Some(trip_count);
      }
    }
    // Calculate the value of the induction variable after one iteration of the
    // loop, and check whether the while condition is true with this new value.
    let map2: HashMap<HloInstruction, Literal<bool>> = HashMap::new();
    let indvar_next_result =
      evaluator.evaluate_with_substitutions(
        while_body_indvar_update, map2);
    if indvar_next_result.is_err() {
      println!("Couldn't evaluate induction variable update: {:?}",
        indvar_next_result.err().unwrap());
      return None;
    }
    // indvar_iter_val = indvar_next_result.unwrap();
  }

  println!("Loop has unknown trip count.");
  None
}

// If the only user of this instruction is a get-tuple-element, return that
// get-tuple-element, otherwise return null. If this runs before CSE/DCE, we may
// get a false negative if there are several copies of the same GTE, or there
// are unused GTEs, but we can live with this.
fn get_only_gte(inst: &HloInstruction) -> Option<&HloInstruction> {
  if inst.user_count() != 1 {
    return None;
  }
  let user = inst.users().last();
  if user.is_none() {
    return None;
  }
  if user.unwrap().opcode() != HloOpcode::GetTupleElement {
    return None;
  }
  user
}

pub fn compute_while_loop_trip_count_upper_bound<T>(while_op: &HloInstruction) -> Option<i64>
  where T: Clone + Default
{
  // If we know the exact trip count, it's also the upper bound.
  let exact_trip_count =
    compute_while_loop_trip_count(while_op, 128);
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
  let while_cond_param = while_cond.parameter_instruction(0);
  let cond_gte = get_only_gte(while_cond_param.unwrap());
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

  let indvar_index = cond_gte.unwrap().tuple_index();
  let while_body_indvar = while_body_root.operand(indvar_index as usize);
  if while_body_indvar.opcode() != HloOpcode::Constant {
    println!("While body does not set the IV to a constant: {:?}",
      while_body_indvar.to_string_default());
    return None;
  }

  // Create a new while cond computation accessing only the single parameter
  // extracted by the GTE above to avoid excessive memory allocation for the
  // evaluator.
  let mut replacements: HashMap<HloInstruction, HloInstruction> = HashMap::new();
  let new_param = HloInstruction::create_parameter(
    0,
    &ShapeUtil::make_tuple_shape(vec![cond_gte.unwrap().shape().clone()]),
    "temp".to_string());
  let value =
    HloInstruction::create_get_tuple_element(&new_param, 0);
  replacements.insert(cond_gte.unwrap().clone(), value);
  replacements.insert(while_cond_param.unwrap().clone(), new_param);
  
  // TODO
  let mut new_module = HloModule::new(
    "temp_mod".to_string(), HloModuleConfig::new_default());
  let new_computation = new_module.add_embedded_computation(
    while_cond.clone_with_replacements(
      &replacements, &vec![], None,
      "clone".to_string(), None));

  // We have a constant. Evaluate the condition on this constant.
  let evaluator: HloEvaluator<bool> = HloEvaluator::new(0);
  let mut fake_input = Literal::new_from_shape(
    new_computation.parameter_instruction(0).unwrap().shape());
  let result = fake_input.copy_from(
    while_body_indvar.mutable_literal(),
    &vec![0],
    &vec![],
    false);
  assert!(result.is_ok());
  let eval_result =
    evaluator.evaluate_computation(new_computation, &vec![fake_input]);
  if eval_result.is_err() {
    println!("Couldn't evaluate while loop condition.");
    return None;
  }

  let cond_result_pred = eval_result.unwrap();
  assert!(ShapeEqual::new().ignore_layout().equal(
    cond_result_pred.shape(), 
    &ShapeUtil::make_shape(&PrimitiveType::BF16, vec![])));

  // Per the explanation above, if the evaluated condition returns false, the
  // loop executes at most once.
  let cond_returns_true = cond_result_pred.get_first_element();
  if !(*cond_returns_true) {
    println!("Upper bound on the trip count is 1");
    return Some(1);
  }

  println!("Loop has no known upper bound on the trip count.");
  None
}