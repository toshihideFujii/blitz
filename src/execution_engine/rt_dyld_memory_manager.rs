#![allow(dead_code)]

// Interface of the runtime dynamic memory manager base class.

struct McJitMemoryManager {}
impl McJitMemoryManager {
  pub fn new() {}
  pub fn notify_object_loaded() {}
  fn anchor() {}
}

struct  RtDyldMemoryManager {}
impl RtDyldMemoryManager {
  pub fn new() {}
  pub fn register_eh_frames_in_process() {}
  pub fn deregister_eh_frames_in_process() {}
  pub fn register_eh_frames() {}
  pub fn deregister_eh_frames() {}
  pub fn get_symbol_address_in_process() {}
  pub fn get_synbol_address() {}
  pub fn find_symbol() {}
  pub fn get_symbol_address_in_logical_dylib() {}
  pub fn find_symbol_in_logical_dylib() {}
  pub fn get_pointer_to_named_function() {}
}