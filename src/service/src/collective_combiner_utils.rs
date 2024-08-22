#![allow(dead_code)]

use hlo::{hlo_computation::HloComputation, hlo_instruction::HloInstruction};

// Combines instructions with matching keys together.
//
// Instructions are combined in topological post-order.
//
// `key_fn` should return equal keys for two instructions that might be combined
// together. Instructions will be combined until the threshold for output byte
// size or instruction count is reached.
pub fn combine_instructions_by_key<K>(
  _computation: &HloComputation,
  _key_fn: Box<dyn Fn(&HloInstruction) -> Option<K>>,
  _combine_fn: Box<dyn Fn(&Vec<HloInstruction>) -> Result<(), String>>,
  _combine_threshold_bytes: i64,
  _combine_threshold_count: i64) -> Result<bool, String>
{
  unimplemented!()
}