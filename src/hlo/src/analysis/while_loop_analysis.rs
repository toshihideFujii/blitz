use common::literal::Literal;

use crate::hlo_instruction::HloInstruction;

// Returns the precise trip count of the loop if it's statically known,
// nullopt otherwise.
//
// max_brute_force_iters limits the number of steps that are evaluated while
// trying to brute force a loop trip count. trip counts larger than
// max_brute_force_iters may be returned if we can pattern-match the loop
// condition.
pub fn compute_while_loop_trip_count(
  _while_op: &HloInstruction, _max_brute_force_iters: usize) -> usize
{
  unimplemented!()
}

// Returns an upper bound on the trip count of the loop if it's statically
// known, nullopt otherwise.
pub fn compute_while_loop_trip_count_upper_bound(_while_op: &HloInstruction) -> usize {
  unimplemented!()
}

// The below function identifies a subset of all possible auxiliary
// induction variables (AIV). Specifically, candidates are gtes, e.g.,
// gte(param0, N)
pub fn get_auziliary_loop_induction_vars(_while_op: &HloInstruction) -> &Vec<HloInstruction> {
  unimplemented!()
}

// Returns the tuple index of the loop induction variable if there is such an
// induction variable detected. It is also checked that all ops that depend on
// the induction variable have scalar shape. Otherwise returns nullopt.
pub fn get_loop_induction_var_tuple_idx(_while_op: &HloInstruction) -> i64 {
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
  _while_op: &HloInstruction, _indvar_tuple_idx: i64, _indvar_init: &Literal<i64>) -> usize
{
  unimplemented!()    
}

// Same as above, but returns the loop range, i.e., start (inclusive), end
// (inclusive) and step instead of the trip count.
pub fn match_trivial_loop_range(_while_op: &HloInstruction) {
  unimplemented!()
}