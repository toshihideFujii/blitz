#![allow(dead_code)]

use common::shape::ProgramShape;
use hlo::hlo_instruction::HloInstruction;
use crate::hlo_proto::{HloModuleProto, HloSnapshot};

// The computation graph that the user builds up with the BlitzBuilder.
pub struct BlitzComputation {
  unique_id: i64,
  proto: HloModuleProto,
}

impl BlitzComputation {
  pub fn new_from_id(unique_id: i64) -> Self {
    BlitzComputation {
      unique_id: unique_id,
      proto: HloModuleProto::default()
    }
  }

  pub fn new_from_proto(proto: HloModuleProto) -> Self {
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

  pub fn set_name(&mut self, name: String) {
    self.proto.set_name(name);
  }

  pub fn proto(&self) -> &HloModuleProto {
    &self.proto
  }

  pub fn mutable_proto(&mut self) -> &mut HloModuleProto {
    &mut self.proto
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

  pub fn set_program_shape(&mut self, _program_shape: ProgramShape) {
    unimplemented!()
  }

  pub fn set_root_id(&mut self, _root_id: i64) {
    unimplemented!()
  }

  pub fn add_instructions(&mut self, _instruction: HloInstruction) {
    unimplemented!()
  }
}