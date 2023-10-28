#![allow(dead_code)]

use std::any::Any;
use std::fmt::Debug;
use crate::ir::{global_value::IntrinsicID, type_::Type, user::User};
use super::{instruction::Instruction, value::{Value, ValueType}, instructions::CallInst, instr_types::CallBase, type_::UnknownType};

// A wrapper class for inspecting calls to intrinsic functions.
// This allows the standard isa/dyncast/cast functionality to work with
// calls to intrinsic functions.
pub trait IntrinsicInst: CallInst {
  // Return the intrinsic ID of this intrinsic.
  //fn get_intrinsic_id(&self) -> IntrinsicID {
    //IntrinsicID::NotIntrinsic
  //}

  // Return true if swapping the first two arguments to the intrinsic
  // produces the same result.
  fn is_commutative(&self) -> bool {
    match self.get_intrinsic_id() {
      IntrinsicID::MaxNum => return true,
      IntrinsicID::MinNum => return true,
      IntrinsicID::Maximum => return true,
      IntrinsicID::Minimum => return true,
      IntrinsicID::SMax => return true,
      IntrinsicID::SMin => return true,
      IntrinsicID::UMax => return true,
      IntrinsicID::UMin => return true,
      IntrinsicID::SAddSat => return true,
      IntrinsicID::UAddSat => return true,
      IntrinsicID::SAddWithOverflow => return true,
      IntrinsicID::UAddWithOverflow => return true,
      IntrinsicID::SMulWithOverflow => return true,
      IntrinsicID::UMulWithOverflow => return true,
      IntrinsicID::SMulFix => return true,
      IntrinsicID::UMulFix => return true,
      IntrinsicID::SMulFixSat => return true,
      IntrinsicID::UMulFixSat => return true,
      IntrinsicID::Fma => return true,
      IntrinsicID::FMulAdd => return true,
      _ => return false
    }
  }

  // Checks if the intrinsic is an annotation.
  fn is_assume_like_intrinsic(&self) -> bool {
    match self.get_intrinsic_id() {
      IntrinsicID::Assume => return true,
      IntrinsicID::SideEffect => return true,
      IntrinsicID::PseudoProbe => return true,
      IntrinsicID::DbgAssign => return true,
      IntrinsicID::DbgDeclare => return true,
      IntrinsicID::DbgValue => return true,
      IntrinsicID::DbgLabel => return true,
      IntrinsicID::InvariantStart => return true,
      IntrinsicID::InvariantEnd => return true,
      IntrinsicID::LifetimeStart => return true,
      IntrinsicID::LifetimeEnd => return true,
      IntrinsicID::ExperimentalNoAliasScopeDecl => return true,
      IntrinsicID::ObjectSize => return true,
      IntrinsicID::PtrAnnotation => return true,
      IntrinsicID::VarAnnotation => return true,
      _ => return false
    }
  }

  fn may_lower_to_function_call(&self) {}
}

pub struct LifetimeIntrinsic {}

// This is the common base class for debug info intrinsics.
pub trait DbgInfoIntrinsic: IntrinsicInst {}

struct DbgVariableIntrinsic {}

struct DbgDeclareInst {}

struct DbgAddrIntrinsic {}

struct DbgValueInst {}

struct DbgAssignIntrinsic {}

// This represents the blitz.dbg.label instruction.
#[derive(Debug)]
pub struct DbgLabelInst {
  t: Box<dyn Type>
}

impl DbgLabelInst {
  pub fn new() -> Self {
    DbgLabelInst { t: Box::new(UnknownType::new()) }
  }

  pub fn get_label(&self) {}
  pub fn get_raw_label(&self) {}

  pub fn class_of(i: Box<dyn IntrinsicInst>) -> bool {
    i.get_intrinsic_id() == IntrinsicID::DbgLabel
  }
}

impl Value for DbgLabelInst {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
      ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}
impl User for DbgLabelInst {}
impl Instruction for DbgLabelInst {
  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}
impl CallBase for DbgLabelInst {}
impl CallInst for DbgLabelInst {}
impl IntrinsicInst for DbgLabelInst {}
impl DbgInfoIntrinsic for DbgLabelInst {}

struct VPIntrinsic {}

struct VPCastIntrinsic {}

struct VPCmpIntrinsic {}

struct ConstrainedFPIntrinsic {}

struct ConstrainedFPCmpIntrinsic {}

struct MinMaxIntrinsic {}

struct BinaryOpIntrinsic {}

struct WithOverflowInst {}

struct SaturatingInst {}

struct AtomicMemIntrinsic {}

struct AtomicMemSetInst {}

struct AtomicMemTransferInst {}

struct AtomicMemCpyInst {}

struct AtomicMemMoveInst {}

struct MemIntrinsic {}

struct MemSetInst {}

struct MemSetInlineInst {}

struct MemTransferInst {}

struct MemCpyInst {}

struct MemMoveInst {}

struct MemCpyInlineInst {}

struct AnyMemIntrinsic {}

struct AnyMemSetInst {}

struct AnyMemTransferInst {}

struct AnyMemCpyInst {}

struct AnyMemMoveInst {}

struct VAStartInst {}

struct VAEndInst {}

struct VACopyInst {}

struct InstrProvCoverInst {}

struct InstrProfIncrementInst {}

struct InstrProfIncrementInstStep {}

struct InstrProfTimestampInst {}

struct InstrProfValueProfileInst {}

#[derive(Debug)]
pub struct PseudoProbeInst {
  t: Box<dyn Type>
}

impl Value for PseudoProbeInst {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }
  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }
  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl User for PseudoProbeInst {
}

impl Instruction for PseudoProbeInst {
  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}