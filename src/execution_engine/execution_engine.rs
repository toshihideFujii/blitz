#![allow(dead_code)]

// Helper class for helping synchronize access to the global address
// map table. Access to this class should be serialized under a mutex.
struct ExecutionEngineState {}

// Abstract interface for implementation execution of blitz modules,
// designed to support both interpreter and just-in-time (JIT) compiler
// implementations.
struct ExecutionEngine {
  // Whether lazy JIT compilation is enabled.
  compiling_lazily_: bool,
  // Whether JIT compilation of external global variables is allowed.
  gv_compilation_disabled_: bool,
  // Whether the JIT should perform lookups of external symbols.
  symbol_searching_disabled_: bool,
  // Whether the JIT should verify IR modules during compilation.
  verify_modules_: bool
}

impl ExecutionEngine {
  // Add a Module to the list of modules that we can JIT from.
  pub fn add_module() {}

  // Add an ObjectFile to the execution engine.
  pub fn add_object_file() {}

  // Add an Archive to the execution engine.
  pub fn add_archive() {}

  pub fn get_data_layout() {}

  pub fn remove_module() {}

  // Search all of the active modules to find the function that
  // defines fn_mame.
  pub fn find_function_named() {}

  pub fn find_global_variable_named() {}

  // Execute the specified function with the specified arguments,
  // and return the result.
  pub fn run_function() {}

  // Returns the address of the specified function by using dlsym 
  // function call.
  pub fn get_pointer_to_named_function() {}

  pub fn map_section_address() {}

  // Run code generation for the specified module and load it into memory.
  pub fn generate_code_for_module() {}

  pub fn finalize_object() {}

  // Returns true if an error has been recorded.
  pub fn has_error() {}

  // Clear the error message.
  pub fn clear_error_message() {}

  // Returns the most recent error message.
  pub fn get_error_message() {}

  pub fn run_static_constructors_destructors() {}

  pub fn run_function_as_main() {}

  // Tell the execution engine that the specified global is
  // at the specified location.
  pub fn add_global_mapping() {}

  pub fn clear_all_global_mappings() {}

  pub fn clear_global_mappings_from_module() {}

  // Replace an existing mapping for GV with a new address.
  pub fn update_global_mapping() {}

  // This returns the address of the specified global symbol.
  pub fn get_address_to_global_if_available() {}

  pub fn get_pointer_to_global_if_available() {}

  pub fn get_pointer_to_global() {}

  pub fn get_pointer_to_function() {}

  pub fn get_pointer_to_function_or_stub() {}

  // Return the address of the specified global value.
  pub fn get_global_value_address() {}

  // Return the address of the specified function.
  pub fn get_function_address() {}

  // Return the blitz global value object that starts at
  // the specified address.
  pub fn get_global_value_at_address() {}

  pub fn store_value_to_memory() {}

  pub fn initialize_memory() {}

  pub fn get_or_emit_global_variable() {}

  // Registers a listener to be called back on various events within
  // the JIT.
  pub fn register_jit_event_listener() {}
  pub fn unregister_jit_event_listener() {}

  // Sets the pre-compiled object cache.
  pub fn set_object_cache() {}

  // Return the target machine.
  pub fn get_target_machine() {}

  pub fn disable_lazy_compilation() {}

  pub fn is_compiling_lazily() {}

  pub fn disable_gv_compilation() {}

  pub fn is_gv_compilation_disabled() {}

  pub fn disable_symbol_searching() {}

  pub fn is_symbol_searching_disabled() {}

  // Enable/Disable IR module verification.
  pub fn set_verify_modules() {}
  pub fn get_verify_modules() {}

  pub fn install_lazy_function_creator() {}
}