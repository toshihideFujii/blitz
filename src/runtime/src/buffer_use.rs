#![allow(dead_code)]

// BufferUse tracks memory access type for a buffer slice, so that Blitz can
// correctly insert synchronization primitives at run time to avoid read/write
// conflicts. Synchronization primitives are specific to the target backend.
pub struct BufferUse {
  access: MemoryAccess
}

pub enum MemoryAccess {
  Read,
  Write,
}

pub struct ReadWriteSet {}

impl ReadWriteSet {
  pub fn default() -> Self {
    ReadWriteSet {  }
  }

  pub fn add(&self, _use_: BufferUse) {
    unimplemented!()
  }

  pub fn add_read(&self) {
      
  }
}