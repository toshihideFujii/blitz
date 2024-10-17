#![allow(dead_code)]

// MaybeOwningDeviceMemory represents either an owned or unowned device memory.
// Like std::variant<se::OwningDeviceMemory, DeviceMemory>. When the object goes
// output of scope, it will free the underlying memory if it owns it.
#[derive(Debug, Clone, Default)]
pub struct MaybeOwningDeviceMemory {}

impl MaybeOwningDeviceMemory {
  pub fn new() {}
  pub fn as_device_memory_base() {}
  pub fn release() {}
  pub fn as_owning_device_memory() {}
  pub fn has_ownership() {}
}