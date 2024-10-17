#![allow(dead_code)]

use common::shape::ProgramShape;
use crate::hlo_proto::{HloModuleProto, HloSnapshot};

// The computation graph that the user builds up with the BlitzBuilder.
pub struct BlitzComputation {
  unique_id: i64,
  proto: HloModuleProto,
}

impl BlitzComputation {
  pub fn new(proto: HloModuleProto) -> Self {
    BlitzComputation { unique_id: proto.id(), proto: proto }
  }

  // Returns the "program shape" (parameter and return shapes) for this
  // computation.
  pub fn get_program_shape(&self) -> Result<ProgramShape, String> {
    unimplemented!()
  }

  pub fn name(&self) -> &String {
    self.proto.name()
  }

  pub fn proto(&self) -> &HloModuleProto {
    &self.proto
  }

  // Requests that we snapshot the computation into a serializable protocol
  // buffer form.
  pub fn shapshot(&self) -> HloSnapshot {
    unimplemented!()
  }

  // Returns true if this object is a null Computation.
  pub fn is_null(&self) -> bool {
    self.unique_id == -1
  }
}