#![allow(dead_code)]

use common::shape::ProgramShape;

// Serialization of HloModule.
#[derive(Debug, Clone)]
pub struct HloModuleProto {
  name: String,
  // The id of this module.
  id: i64
}

impl HloModuleProto {
  pub fn default() -> Self {
    HloModuleProto { name: "".to_string(), id: 0 }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = id;
  }

  pub fn set_entry_computation_name(&mut self, _name: String) {
    unimplemented!()
  }

  pub fn set_entry_computation_id(&mut self, _id: i64) {
    unimplemented!()
  }

  pub fn set_host_program_shape(&mut self, _program_shape: ProgramShape) {
    unimplemented!()
  }
}

// Serialization of BufferAssignment.
pub struct BufferAssignmentProto {}

// Grouping message that contains all of the information above.
pub struct HloProto {
  hlo_module: HloModuleProto,
  buffer_assignment: BufferAssignmentProto
}

// Encapsulates HloProto together with the arguments, result, and
// execution_platform. This message is used for purposes such as
// analysis/replay/file-storage.
pub struct HloSnapshot {}

impl HloSnapshot {
  pub fn new() -> Self {
    HloSnapshot {  }
  }

  pub fn hlo(&self) {
    unimplemented!()
  }

  pub fn has_hlo(&self) -> bool {
    unimplemented!()
  }
}

pub struct HloProfilePrinterData {}