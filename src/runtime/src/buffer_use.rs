#![allow(dead_code)]

use common::shape::Shape;
use service::buffer_assignment::BufferAllocationSlice;

// BufferUse tracks memory access type for a buffer slice. This is used to
// let BLITZ:
// - Correctly insert synchronization primitives at run time to avoid read/write
//   conflicts. Synchronization primitives are specific to the target backend.
// - Determine whether a buffer has defined contents before/after we execute a
//   thunk. This is used to detect non-deterministic behavior via checksumming.
// - We also use shape to know how the bytes in the slice are reinterpreted by
//   thunks. Shape can be used by rewriters in ThunkPassPipeline.
#[derive(Debug, Clone, PartialEq)]
pub struct BufferUse {
  slice: BufferAllocationSlice,
  shape: Shape,
  access: MemoryAccess,
  content_validity: ContentValidity
}

impl BufferUse {
  pub fn new(
    slice: BufferAllocationSlice,
    access: MemoryAccess,
    content_validity: ContentValidity,
    shape: Shape) -> Self
  {
    BufferUse { slice, shape, access, content_validity }
  }

  pub fn new_without_validity(
    slice: BufferAllocationSlice,
    access: MemoryAccess,
    shape: Shape) -> Self
  {
    let mut content_validity = ContentValidity::DefinedOnOutput;
    if access == MemoryAccess::Read {
      content_validity = ContentValidity::DefinedOnInputAndOutput;
    }
    BufferUse { slice, shape, access, content_validity }
  }

  pub fn read(slice: BufferAllocationSlice, shape: Shape) -> Self {
    BufferUse::new(slice, MemoryAccess::Read,
      ContentValidity::DefinedOnInputAndOutput, shape)
  }

  pub fn write(slice: BufferAllocationSlice, shape: Shape) -> Self {
    BufferUse::new(slice, MemoryAccess::Write,
      ContentValidity::DefinedOnOutput, shape)
  }

  pub fn scratch(slice: BufferAllocationSlice, shape: Shape) -> Self {
    BufferUse::new(slice, MemoryAccess::Write,
      ContentValidity::Undefined, shape)
  }

  pub fn consume(slice: BufferAllocationSlice, shape: Shape) -> Self {
    BufferUse::new(slice, MemoryAccess::Write,
      ContentValidity::DefinedOnInput, shape)
  }

  // Returns true if the buffer contains initialized data when thunk starts
  // execution.
  pub fn has_defined_contents_on_input(&self) -> bool {
    self.content_validity == ContentValidity::DefinedOnInput ||
    self.content_validity == ContentValidity::DefinedOnInputAndOutput
  }

  // Returns true if the buffer contains initialized data when thunk finishes
  // execution.
  pub fn has_defined_contents_on_output(&self) -> bool {
    self.content_validity == ContentValidity::DefinedOnOutput ||
    self.content_validity == ContentValidity::DefinedOnInputAndOutput
  }

  pub fn access(&self) -> MemoryAccess {
    self.access.clone()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemoryAccess {
  Read, // The buffer is only read.
  Write, // The buffer is read and written to.
}

// Flags that indicate whether the contents of a buffer are defined before and
// after execution of a thunk.
#[derive(Debug, Clone, PartialEq)]
pub enum ContentValidity {
  Undefined,
  DefinedOnInput,
  DefinedOnOutput,
  DefinedOnInputAndOutput,
}

// BufferReadWriteSet tracks a set of read and write buffer slices.
#[derive(Debug, Clone)]
pub struct BufferReadWriteSet {
  read: Vec<BufferUse>,
  write: Vec<BufferUse>
}

impl BufferReadWriteSet {
  pub fn default() -> Self {
    BufferReadWriteSet { read: Vec::new(), write: Vec::new() }
  }

  pub fn add(&mut self, buffer_use: BufferUse) {
    match buffer_use.access() {
      MemoryAccess::Read => self.add_read(buffer_use),
      MemoryAccess::Write => self.add_write(buffer_use),
    }
  }

  pub fn add_read(&mut self, buffer_use: BufferUse) {
    self.read.push(buffer_use);
  }

  pub fn add_write(&mut self, buffer_use: BufferUse) {
    self.write.push(buffer_use);
  }

  pub fn add_all(&mut self, uses: Vec<BufferUse>) {
    for b_use in uses {
      self.add(b_use);
    }
  }

  pub fn has_conflicts(&self, buffer_use: &BufferUse) -> bool {
    let overlaps =
      |set: &Vec<BufferUse>, b_use: &BufferUse| -> bool
    {
      for other in set {
        if other.slice.overlaps_with(&b_use.slice) || other.slice == b_use.slice {
          return true;
        }
      }
      false
    };
    if buffer_use.access() == MemoryAccess::Write {
      return overlaps(&self.write, buffer_use) ||
        overlaps(&self.read, buffer_use);
    } else {
      return overlaps(&self.write, buffer_use);
    }
  }

  pub fn has_conflicts_by_other(&self, other: &BufferReadWriteSet) -> bool {
    for read in &other.read {
      if self.has_conflicts(read) { return true; }
    }
    for write in &other.write {
      if self.has_conflicts(write) { return true; }
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use common::shape_util::ShapeUtil;
  use service::buffer_assignment::BufferAllocation;
  use common::blitz_data::PrimitiveType;
  use super::*;

  #[test]
  fn test_equality() {
    let alloc =
      BufferAllocation::new(0, 1024, 0);
    let slice0_shape = ShapeUtil::make_shape(
      &PrimitiveType::F32, vec![2]);
    let slice_0 =
      BufferAllocationSlice::new(alloc, 0, 8);

    let use_read_0 = BufferUse::read(slice_0.clone(), slice0_shape.clone());
    let use_read_1 = BufferUse::read(slice_0.clone(), slice0_shape.clone());
    let use_write = BufferUse::write(slice_0.clone(), slice0_shape.clone());
    let use_scratch = BufferUse::scratch(slice_0.clone(), slice0_shape.clone());
    let use_consume = BufferUse::consume(slice_0.clone(), slice0_shape.clone());

    assert_eq!(use_read_0, use_read_1);
    assert_ne!(use_read_0, use_write);
    assert_ne!(use_read_0, use_scratch);
    assert_ne!(use_read_0, use_consume);

    assert_ne!(use_write, use_scratch);
    assert_ne!(use_write, use_consume);

    assert_ne!(use_scratch, use_consume);
  }

  #[test]
  fn test_has_defined_contents() {
    let alloc =
      BufferAllocation::new(0, 1024, 0);
    let slice_shape = ShapeUtil::make_shape(
      &PrimitiveType::F32, vec![2]);
    let slice =
      BufferAllocationSlice::new(alloc, 0, 8);

    let read = BufferUse::read(slice.clone(), slice_shape.clone());
    assert!(read.has_defined_contents_on_input());
    assert!(read.has_defined_contents_on_output());

    let write = BufferUse::write(slice.clone(), slice_shape.clone());
    assert!(!write.has_defined_contents_on_input());
    assert!(write.has_defined_contents_on_output());

    let scratch = BufferUse::scratch(slice.clone(), slice_shape.clone());
    assert!(!scratch.has_defined_contents_on_input());
    assert!(!scratch.has_defined_contents_on_output());

    let consume = BufferUse::consume(slice.clone(), slice_shape.clone());
    assert!(consume.has_defined_contents_on_input());
    assert!(!consume.has_defined_contents_on_output());
  }

  #[test]
  fn test_absl_stringify() {
    unimplemented!()
  }

  #[test]
  fn test_read_write_set() {
    let mut rwsest = BufferReadWriteSet::default();
    let alloc =
      BufferAllocation::new(0, 1024, 0);
    let slice_shape = ShapeUtil::make_shape(
      &PrimitiveType::F32, vec![2]);

    let slice_0 =
      BufferAllocationSlice::new(alloc.clone(), 0, 8);
    let slice_1 =
      BufferAllocationSlice::new(alloc.clone(), 4, 8);
    let slice_2 =
      BufferAllocationSlice::new(alloc.clone(), 8, 8);

    rwsest.add(BufferUse::read(slice_0, slice_shape.clone()));
    assert_eq!(rwsest.has_conflicts(&BufferUse::read(slice_1.clone(),
      slice_shape.clone())), false);
    assert_eq!(rwsest.has_conflicts(&BufferUse::write(slice_1.clone(),
      slice_shape.clone())), true);
    assert_eq!(rwsest.has_conflicts(&BufferUse::write(slice_2.clone(),
      slice_shape.clone())), false);

    rwsest.add(BufferUse::read(slice_1.clone(), slice_shape.clone()));
    assert_eq!(rwsest.has_conflicts(&BufferUse::write(slice_2.clone(),
      slice_shape.clone())), true);
  }
}