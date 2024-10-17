#![allow(dead_code)]

// Serialization of HloModule.
#[derive(Debug, Clone)]
pub struct HloModuleProto {
  name: String,
  // The id of this module.
  id: i64
}

impl HloModuleProto {
  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn id(&self) -> i64 {
    self.id
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