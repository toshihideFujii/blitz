#![allow(dead_code)]

// This file contains the declarations of entities that describe
// floating point environment and related functions.

use crate::adt::{floating_point_mode::RoundingMode, string_ref::StringRef, string_switch::StringSwitch};
use super::{fmf::FastMathFlags, instruction::InstructionBase};

// Exception behavior used for floating point operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ExceptionBehavior {
  Ignore,
  MayTrap,
  Strict
}

// Returns a valid RoundingMode enumerator when given a string that is
// valid as input in constrained intrinsic rounding mode metadata.
pub fn convert_str_to_rounding_mode(s: StringRef) -> Option<RoundingMode> {
  StringSwitch::new(s)
    .case(StringRef::new_from_string("round.dynamic"),
    Some(RoundingMode::Dynamic))
    .case(StringRef::new_from_string("round.tonearest"),
    Some(RoundingMode::NearestTiesToEven))
    .case(StringRef::new_from_string("round.tonearestaway"),
    Some(RoundingMode::NearestTiesToAway))
    .case(StringRef::new_from_string("round.downward"),
    Some(RoundingMode::TowardNegative))
    .case(StringRef::new_from_string("round.upward"),
    Some(RoundingMode::TowardPositive))
    .case(StringRef::new_from_string("round.towardzero"),
    Some(RoundingMode::TowardZero))
    .default(None)
}

// For any RoundingMode enumerator, returns a string valid as input in
// constrained intrinsic rounding mode metadata.
pub fn convert_rounding_mode_to_str(rm: RoundingMode) -> StringRef {
  match rm {
    RoundingMode::Dynamic =>
      return StringRef::new_from_string("round.dynamic"),
    RoundingMode::NearestTiesToEven =>
      return StringRef::new_from_string("round.tonearest"),
    RoundingMode::NearestTiesToAway =>
      return StringRef::new_from_string("round.tonearestaway"),
    RoundingMode::TowardNegative =>
      return StringRef::new_from_string("round.downward"),
    RoundingMode::TowardPositive =>
      return StringRef::new_from_string("round.upward"),
    RoundingMode::TowardZero =>
      return StringRef::new_from_string("round.towardzero"),
    _ => panic!("Invalid RoundingMode.")
  };
}

// Returns a valid ExceptionBehavior enumerator when given a string
// valid as input in consrained intrinsic exception behavior metadata.
pub fn convert_str_to_exception_behavior(s: StringRef) -> Option<ExceptionBehavior> {
  StringSwitch::new(s)
    .case(StringRef::new_from_string("fpexcept.ignore"),
    Some(ExceptionBehavior::Ignore))
    .case(StringRef::new_from_string("fpexcept.maytrap"),
    Some(ExceptionBehavior::MayTrap))
    .case(StringRef::new_from_string("fpexcept.strict"),
    Some(ExceptionBehavior::Strict))
    .default(None)
}

// For any ExceptionBehavior enumerator, return a string valid as
// input in constrained intrinsic exception behavior metadata.
pub fn convert_exception_behavior_to_str(eb: ExceptionBehavior) -> StringRef {
  match eb {
    ExceptionBehavior::Strict =>
      return StringRef::new_from_string("fpexcept.strict"),
    ExceptionBehavior::Ignore =>
      return StringRef::new_from_string("fxexcept.ignore"),
    ExceptionBehavior::MayTrap =>
      return StringRef::new_from_string("fpexcept.maytrap")
  };
}

// Returns true if the exception handling behavior and rounding mode
// match what is used in the default floaitng point environment.
pub fn is_default_fp_environment(eb: ExceptionBehavior, rm: RoundingMode) -> bool {
  eb == ExceptionBehavior::Ignore && rm == RoundingMode::NearestTiesToEven
}

// Returns constrained intrinsic id to represent the given instruction
// in strictfp function.
pub fn get_constrained_intrinsic_id(_instr: InstructionBase) {}

// Returns true if the rounding mode RM may be QRM at compile time or
// at run time.
pub fn can_rounding_mode_be(rm: RoundingMode, qrm: RoundingMode) -> bool {
  rm == qrm || rm == RoundingMode::Dynamic
}

// Returns true if the possibility of a signaling NaN can be safely
// ignored.
pub fn can_ignore_s_nan(eb: ExceptionBehavior, fmf: FastMathFlags) -> bool {
  eb == ExceptionBehavior::Ignore || fmf.no_nans()
}