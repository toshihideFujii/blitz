#![allow(dead_code)]

use std::mem::size_of;

// void*-analogous device memory allocation. For the typed variation, see
// DeviceMemory<T>.
//
// This is effectively a two-tuple of a pointer and size; however, note that the
// pointer may not be to the virtual address itself -- in OpenCL the pointer is
// to a cl_mem handle that describes the device allocation. Therefore,
// DeviceMemoryBase::opaque does not necessarily produce a pointer that can be
// referenced directly, so use it with caution.
//
// Thread-compatible.
#[derive(Debug, Clone, Default)]
pub struct DeviceMemoryBase {
  size: usize,
  payload: u64
}

impl DeviceMemoryBase {
  pub fn new(size: usize) -> Self {
    DeviceMemoryBase { size: size, payload: 0 }
  }

  // Returns whether the backing memory is the null pointer.
  pub fn is_null(&self) -> bool {
    unimplemented!()
  }

  // Returns the size, in bytes, for the backing memory.
  pub fn size(&self) -> usize {
    self.size
  }

  // Warning: note that the pointer returned is not necessarily directly to
  // device virtual address space, but is platform-dependent.
  pub fn opaque() {}

  // Returns the payload of this memory region.
  pub fn payload(&self) -> u64 {
    self.payload
  }

  // Sets payload to given value.
  pub fn set_payload(&mut self, payload :u64) {
    self.payload = payload;
  }

  // Returns whether the two DeviceMemoryBase segments are identical.
  pub fn is_same_as(&self, _other: &DeviceMemoryBase) -> bool {
    unimplemented!()
  }

  pub fn get_byte_slice(&self, _offset_bytes: u64, _size_bytes: u64) -> Self {
    unimplemented!()
  }

  pub fn reset() {}
}

// Typed wrapper around "void *"-like DeviceMemoryBase.
// For example, DeviceMemory<int> is a simple wrapper around DeviceMemoryBase
// that represents one or more integers in Device memory.
// Thread-compatible.
pub struct DeviceMemory<T> {
  type_: T,
  base: DeviceMemoryBase
}

impl<T> DeviceMemory<T> {
  // Default constructor instantiates a null-pointed, zero-sized memory region.
  pub fn default(t: T) -> Self {
    DeviceMemory { type_: t, base: DeviceMemoryBase::new(0) }
  }

  // This is made protected because it accepts a byte-size instead of an element
  // count, which could potentially be misused given the ElementCount() nature
  // of this interface.
  //
  // In order to specify the desire to use byte size instead of element count
  // explicitly, use MakeFromByteSize.
  pub fn new(opaque: T, size: usize) -> Self {
    DeviceMemory { type_: opaque, base: DeviceMemoryBase::new(size) }
  }

  // Returns the number of elements of type T that constitute this allocation.
  pub fn element_count(&self) -> usize {
    self.base.size() / size_of::<T>()
  }

  // Returns whether this is a single-element allocation.
  pub fn is_scalar(&self) -> bool {
    self.element_count() == 1
  }

  // Returns pointer to the allocated data
  pub fn base() {}

  // Creates a typed area of DeviceMemory with a given opaque pointer and the
  // quantity of bytes in the allocation. This function is broken out to
  // distinguish bytes from an element count.
  pub fn make_form_byte_size(opaque: T, bytes: usize) -> Self {
    DeviceMemory::new(opaque, bytes)
  }

  // Creates a memory region (slice) inside another allocated memory region.
  // Offset and size are specified in terms of ElemT elements.
  pub fn get_slice(&self, _element_offset: u64, _element_count: u64) -> Self {
    unimplemented!()
  }
}