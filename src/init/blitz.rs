struct Blitz {}

impl Blitz {
  // Global actions.
  pub fn initialize() {}
  pub fn teardown() {}

  // Report process out of memory.
  // This function will not return, but will terminate this execution.
  pub fn fatal_process_out_of_memory() {}

  pub fn initialize_platform() {}
  pub fn shutdown_platform() {}
  pub fn get_current_platform() {}
  pub fn set_platform_for_testing() {}
  pub fn set_snapshot_blob() {}
}
