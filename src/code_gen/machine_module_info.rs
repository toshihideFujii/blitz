#![allow(dead_code)]

// Collect meta information for a module.

struct MachineModuleInfo {}
impl MachineModuleInfo {
  pub fn new() {}
  pub fn initialize() {}
  pub fn finalize() {}
  pub fn get_target() {}
  pub fn get_context() {}
  pub fn get_module() {}
  pub fn get_or_create_machine_function() {}
  pub fn get_machine_function() {}
  pub fn delete_machine_function_for() {}
  pub fn insert_function() {}
  pub fn get_obj_file_info() {}
  pub fn has_debug_info() {}
  pub fn uses_msvc_floating_point() {}
  pub fn set_uses_msvc_floating_point() {}
  pub fn set_current_call_site() {}
  pub fn get_current_call_site() {}
  pub fn invalidate() {}
}

struct MachineModuleInfoWrapperPass {}
impl MachineModuleInfoWrapperPass {
  pub fn new() {}
  pub fn do_initialization() {}
  pub fn fo_finalization() {}
  pub fn get_mmi() {}
}

struct MachineModuleAnalysis {}
impl MachineModuleAnalysis {
  pub fn new() {}
  pub fn run() {}
}