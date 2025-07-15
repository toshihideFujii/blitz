#![allow(dead_code)]

use std::collections::HashMap;
use common::{blitz_data::GlobalDataHandle, shape::Shape, shape_util::ShapeUtil};
use stream_executor::{device_memory::DeviceMemoryBase, device_memory_allocator::OwningDeviceMemory};
use crate::{backend::Backend, shaped_buffer::{ScopedShapedBuffer, ShapedBuffer}};

// Data structure encapsulating single memory allocation on the device.
pub struct Allocation {
  device_memory: OwningDeviceMemory,
  ref_count: i64
}

impl Allocation {
  pub fn new(device_mmeory: OwningDeviceMemory, ref_count: i64) -> Self {
    Allocation {
      device_memory: device_mmeory,
      ref_count: ref_count
    }
  }
}

// A map from device memory opaque value to allocation. One such map is
// maintained per device ordinal.
pub type AllocationMap = HashMap<i64, Allocation>;

// Tracks allocations for the Blitz service; allocations can be registered
// with shape/device/tag and resolved from a handle for later use.
pub struct AllocationTracker {
  // Backend to use with this tracker. The backend supplies the memory allocator
  // to use when deallocating memory.
  backend: Backend,
  next_handle: i64,
  opaque_to_allocation_map: HashMap<i64, AllocationMap>,
  handle_to_shaped_buffers: HashMap<i64, Vec<ShapedBuffer>> // TODO: lock 
}

impl AllocationTracker {
  // The allocator is used for deallocating memory when allocations are
  // deregistered. All registered allocations must have the same platform as the
  // allocator.
  pub fn new(backend: Backend) -> Self {
    AllocationTracker {
      backend: backend,
      next_handle: 1,
      opaque_to_allocation_map: HashMap::new(),
      handle_to_shaped_buffers: HashMap::new()
    }
  }

  fn release_if_scoped_shaped_buffer(b: &ShapedBuffer) -> &ShapedBuffer { b }

  // Registers a shaped buffer of device memory, and returns a corresponding
  // handle that can be used for talking to XLA clients. The given shaped buffer
  // will be treated as the buffer corresponding to the only replica.
  pub fn register(
    &mut self,
    shaped_buffer: ScopedShapedBuffer,
    tag: String) -> Result<GlobalDataHandle, String>
  {
    // TODO: lock
    println!("register");
    let mut replicated_buffers = vec![];
    replicated_buffers.push(shaped_buffer);
    self.register_internal(replicated_buffers, tag)
  }

  fn register_internal(
    &mut self,
    replicated_buffers: Vec<ScopedShapedBuffer>,
    tag: String) -> Result<GlobalDataHandle, String>
  {
    println!("register_internal");
    println!("tag: {:?} with {:?} shaped_buffers.", tag, replicated_buffers.len());

    let handle = self.next_handle;
    self.next_handle += 1;
    for shaped_buffer in &replicated_buffers {
      let mut shape_indices: Vec<Vec<i64>> = vec![];
      let mut func = |_shape: &Shape, index: &Vec<i64>| {
        let mut value = vec![];
        value.clone_from(index);
        shape_indices.push(value);
      };
      ShapeUtil::for_each_subshape(shaped_buffer.on_device_shape(), &mut func);
      // Add shaped_buffer's buffers to opaque_to_allocation_map_, which owns them.
      for index in &shape_indices {
        self.add_allocation_or_increment_ref_count(
          shaped_buffer.buffer(index).clone(),
          shaped_buffer.device_ordinal());
      }
      // If ShapedBufferTy is ScopedShapedBuffer, release the ScopedShapedBuffer
      // into a regular ShapedBuffer, which is stored in
      // handle_to_shaped_buffers_.
      let buffer =
        self.handle_to_shaped_buffers.get_mut(&handle);
      let val = AllocationTracker::release_if_scoped_shaped_buffer(
        shaped_buffer.base()).clone();
      buffer.unwrap().push(val);
    }

    let mut result = GlobalDataHandle::default();
    result.set_handle(handle);
    println!("handle: {:?}", handle);
    Ok(result)
  }

  // Adds the given device address to the allocation tracker, or if it already
  // exists, then increment its reference count.
  pub fn add_allocation_or_increment_ref_count(
    &mut self,
    device_memory: DeviceMemoryBase,
    device_ordinal: i64)
  {
    let allocation_map_wrapper =
      self.opaque_to_allocation_map.get_mut(&device_ordinal);
    assert!(allocation_map_wrapper.is_some());

    let allocation_map = allocation_map_wrapper.unwrap();
    let target = allocation_map.get_mut(&device_memory.opaque());
    if target.is_none() {
      let memory = OwningDeviceMemory::new(
        device_memory.clone(), device_ordinal, self.backend.memory_allocator());
      let allocation = Allocation::new(memory, 1);
      allocation_map.insert(device_memory.opaque(), allocation);
    } else {
      target.unwrap().ref_count += 1;   
    }
  }

  // Decrements the reference count of the given device memory. Then, if it is
  // zero, deallocate the memory.
  pub fn decrement_ref_count(
    &mut self,
    device_memory: &DeviceMemoryBase,
    device_ordinal: i64) -> Result<(), String>
  {
    let allocation_map =
      self.opaque_to_allocation_map.get_mut(&device_ordinal).unwrap();
    
    let allocation =
      allocation_map.get_mut(&device_memory.opaque()).unwrap();
    assert!(allocation.ref_count >= 1);

    if allocation.ref_count == 1 {
      let result = allocation.device_memory.free();
      check_error(&result);
      let removed =
        allocation_map.remove(&device_memory.opaque());
      assert!(removed.is_some());
    } else {
      allocation.ref_count -= 1;
    }
    Ok(())
  }

  // Registers a vector of shaped buffers of device memory, one per replica, and
  // returns a corresponding handle that can be used for talking to Blitz clients.
  pub fn register_replicated_buffers(
    &mut self,
    replicated_buffers: Vec<ScopedShapedBuffer>,
    tag: String) -> Result<GlobalDataHandle, String>
  {
    println!("register_replicated_buffers");
    self.register_internal(replicated_buffers, tag)
  }

  // Unregister the allocation for the given data handle.
  pub fn unregister(&mut self, data: &GlobalDataHandle) -> Result<(), String> {
    // TODO: lock
    println!("unregister");
    println!("handle: {:?}", data.handle());

    let replicated_buffers_wrapper =
      self.resolve_internal(data.clone());
    check_error(&replicated_buffers_wrapper);

    let replicated_buffers = replicated_buffers_wrapper.unwrap();
    for shaped_buffer in replicated_buffers {
      let mut shape_indices: Vec<Vec<i64>> = vec![];
      let mut func = |_shape: &Shape, index: &Vec<i64>| {
        let mut val = vec![];
        val.clone_from(index);
        shape_indices.push(val);
      };
      ShapeUtil::for_each_subshape(shaped_buffer.on_device_shape(), &mut func);
      for index in &shape_indices {
        let result = self.decrement_ref_count(
          shaped_buffer.buffer(index), shaped_buffer.device_ordinal());
        check_error(&result);
      }
    }
    // Keep a nullptr as a tombstone for unregistered handles. This enables
    // better error messages. That is, "handle has been deallocated" versus
    // "handle does not exist".
    let target =
      self.handle_to_shaped_buffers.get_mut(&data.handle());
    if target.is_none() {
      let mut err_msg =
        "no allocation record for global data handle: ".to_string();
      err_msg.push_str(&data.handle().to_string());
      return Err(err_msg);
    }
    for shaped_buffer in target.unwrap() {
      shaped_buffer.clear();
    }
    Ok(())
  }

  pub fn deconstruct_tuple(
    &self,
    _data: GlobalDataHandle) -> Result<Vec<GlobalDataHandle>, String>
  {
    unimplemented!()
  }

  // Resolve a handle from an XLA client to a vector of shaped buffers, one per
  // replica, or provide an error status to say whether any of those buffers
  // were not found (or found, but found deallocated).
  pub fn resolve(
    &self,
    data: GlobalDataHandle) -> Result<Vec<ShapedBuffer>, String>
  {
    // TODO: lock
    self.resolve_internal(data)
  }

  // Resolves a handle from an Blitz client and replica id to a shaped buffer, or
  // provide an error status to say whether it was not found (or found, but
  // found deallocated).
  pub fn resolve_for_replica(
    &self,
    data: GlobalDataHandle,
    replica_id: i64) -> Result<ShapedBuffer, String>
  {
    // TODO: lock
    let replicated_buffers_wrapper =
      self.resolve_internal(data);
    check_error(&replicated_buffers_wrapper);

    let replicated_buffers = replicated_buffers_wrapper.unwrap();
    if replica_id as usize >= replicated_buffers.len() {
      let mut err_msg = "Requesting buffer for replica ".to_string();
      err_msg.push_str(&replica_id.to_string());
      err_msg.push_str(", but found buffers only for ");
      err_msg.push_str(&replicated_buffers.len().to_string());
      err_msg.push_str(" replica.");
      return Err(err_msg);
    }
    Ok(replicated_buffers[replica_id as usize].clone())
  }

  fn resolve_internal(
    &self,
    data: GlobalDataHandle) -> Result<Vec<ShapedBuffer>, String>
  {
    println!("resolve: {:?}", data.handle());
    let target =
      self.handle_to_shaped_buffers.get(&data.handle());
    if target.is_none() {
      let mut err_msg =
        "No allocation record for global data handle: ".to_string();
      err_msg.push_str(&data.handle().to_string());
      return Err(err_msg);
    }
    let mut replicated_buffers = vec![];
    for shaped_buffer in target.unwrap() {
      replicated_buffers.push(shaped_buffer.clone());
    }
    Ok(replicated_buffers)
  }
}

fn check_error<T>(value: &Result<T, String>) {
  if value.is_err() {
    let err_msg = value.as_ref().err().unwrap();
    assert!(false, "{:?}", err_msg);
  }
}