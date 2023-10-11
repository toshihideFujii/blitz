#![allow(dead_code)]

use std::any::Any;
use std::fmt::Debug;
use crate::ir::global_value::IntrinsicID;
use super::instruction::Instruction;

// A wrapper class for inspecting calls to intrinsic functions.
// This allows the standard isa/dyncast/cast functionality to work with
// calls to intrinsic functions.
pub trait IntrinsicInst {
  // Return the intrinsic ID of this intrinsic.
  fn get_intrinsic_id(&self) -> IntrinsicID {
    IntrinsicID::NotIntrinsic
  }
}

struct LifetimeIntrinsic {}

// This is the common base class for debug info intrinsics.
pub trait DbgInfoIntrinsic: Debug + Instruction {}

struct DbgVariableIntrinsic {}

struct DbgDeclareInst {}

struct DbgAddrIntrinsic {}

struct DbgValueInst {}

struct DbgAssignIntrinsic {}

struct DbgLabelInst {}

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

#[derive(Debug)]
pub struct PseudoProbeInst {}
impl Instruction for PseudoProbeInst {
  fn as_any(&self) -> &dyn Any {
    self
  }
}