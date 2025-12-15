#![allow(dead_code)]

use crate::{hlo_instruction::HloInstruction, hlo_module::HloModule};

// Define ReplaceTypeSelector, which is a lambda that, given an HLO
// instruction, returns ReplaceType, which indicated which type of op should be
// used to replace.
//
// kReplaceParam: hlo instruction will be replaced with parameter. Note that it
// can only replace the instructions at the entry computation with parameters.
// If `cross_computation` is enabled and users attempt to replace an instruction
// in non-entry computation with a parameter, this library would report FATAL.
//
// kReplaceConst: hlo instruction will be replaced with randomly-generated
// constant of the same shape. Note that it could be very slow if hlo
// instruction has a large shape. It can be used in both entry and non-entry
// computation.
//
// kReplaceZeroBroadcast: hlo instruction will be replaced with a broadcasted
// zero constant of the same shape. It can be used in both entry and non-entry
// computation.
//
// kReplaceRandomBroadcast: hlo instruction will be replaced with a broadcasted
// random constant of the same shape. It can be used in both entry and non-entry
// computation.
pub enum ReplaceType {
  ReplaceParam,
  ReplaceConst,
  ReplaceZeroBroadcast,
  ReplaceRandomBroadcast,
}

pub fn extract_module(
  _instruction: &HloInstruction,
  _height: i64,
  _extract_selector: &dyn Fn(&HloInstruction)->bool,
  _replace_type_selector: &dyn Fn(&HloInstruction)->ReplaceType,
  _cross_computation: bool,
  _inline_calls_and_fusions: bool,
  _run_verifier: bool) -> HloModule
{
  unimplemented!()    
}