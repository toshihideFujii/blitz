#![allow(dead_code)]

// Interface for the runtime dynamic linker facilities of the MC-JIT.

struct LoadedObjectInfo {}
impl LoadedObjectInfo {
  pub fn new() {}
  pub fn reassign_section_address() {}
  pub fn loaded_object_info() {}
  pub fn get_object_for_debug() {}
  pub fn get_section_load_address() {}
}

struct MemoryManager {}
impl MemoryManager {
  pub fn new() {}
  pub fn allocate_code_section() {}
  pub fn allocate_data_section() {}
  pub fn allocate_tls_section() {}
  pub fn reserve_allocation_space() {}
  pub fn needs_to_reserve_allocation_space() {}
  pub fn allow_stub_allocation() {}
  pub fn register_eh_frames() {}
  pub fn deregister_eh_frames() {}
  pub fn finalize_memory() {}
  pub fn notify_object_loaded() {}
  fn anchor() {}
}

struct  RuntimeDyld {}
impl RuntimeDyld {
  pub fn new() {}
  pub fn load_object() {}
  pub fn get_symbol_local_address() {}
  pub fn get_symbol_section_id() {}
  pub fn get_symbol() {}
  pub fn get_symbol_table() {}
  pub fn resolve_relocations() {}
  pub fn map_section_address() {}
  pub fn get_section_content() {}
  pub fn get_section_load_address() {}
  pub fn set_notify_stub_emitted() {}
  pub fn register_eh_frames() {}
  pub fn deregister_eh_frames() {}
  pub fn has_error() {}
  pub fn get_error_string() {}
  pub fn set_process_all_sections() {}
  pub fn finalize_with_memory_manager_locking() {}
}