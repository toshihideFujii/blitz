// An abstract superclass for classes representing JavaScript function values.
// It doesn't carry any functionality but allows function classes to be
// identified in the type system.
struct JSFunctionOrBoundFunction {}

// JSBoundFunction describes a bound function exotic object.
struct JSBoundFunction {}

impl JSBoundFunction {
  pub fn get_name() {}
  pub fn get_length() {}

  pub fn to_string() {}
}

// JSFunction describes JavaScript functions.
struct JSFunction {}

impl JSFunction {
  // The information about the function that can be shared by instances.
  pub fn shared() {}

  // The context for this function.
  pub fn context() {}
  pub fn has_context() {}
  pub fn set_context() {}
  pub fn global_proxy() {}
  pub fn native_context() {}
  pub fn length() {}

  pub fn get_name() {}

  // Returns the address of the function code's instruction start.
  pub fn code_entry_point() {}

  // Get the abstract code associated with the function, which will either be
  // a Code object or a BytecodeArray.
  pub fn abstract_code() {}

  // The predicates for querying code kinds related to this function have
  // specific terminology:
  // - Attached: all code kinds that are directly attached this JSFunction object.
  // - Available: all code kinds that are either attached or available through
  //   indirect means such as the feedback vector's optimized code cache.
  // - Active: the single code kind that would be executed if this function
  //   were called in its current state. Note that there may not be an active code
  //   kind if the function is not compiled.
  pub fn has_attached_optimized_code() {}
  pub fn has_available_optimized_code() {}
  pub fn has_attached_code_kind() {}
  pub fn has_available_code_kind() {}

  pub fn get_active_tier() {}
  pub fn active_tier_is_ignition() {}
  pub fn active_tier_is_turbofan() {}
  pub fn active_tier_is_midtier_turboprop() {}
  pub fn active_tier_is_toptier_turboprop() {}

  pub fn next_tier() {}

  // Similar to SharedFunctionInfo::can_discard_compiled.
  // Returns true, if the attached code can be recreated at a later point by replacing it
  // with compileLazy.
  pub fn can_discard_compiled() {}

  // Tells whether or not this function checks its optimization marker in its
  // feedback vector.
  pub fn checks_optimization_marker() {}

  // Tells whether or not this function has a (non-zero) optimization marker.
  pub fn has_optimization_marker() {}

  // Mark this function for lazy recompilation.
  // The function will be recompiled the next time it is executed.
  pub fn mark_for_optimization() {}

  // Tells whether or not the function is already marked for lazy recompilation.
  pub fn is_marked_for_optimization() {}
  pub fn is_marked_for_concurrent_optimization() {}

  // Tells whether or not the function is on the concurrent recompilation queue.
  pub fn is_in_optimization_queue() {}

  // Sets the optimization marker in the function's feedback vector.
  pub fn set_optimization_marker() {}

  // Clears the optimization marker in the function's feedback vector.
  pub fn clear_optimization_marker() {}

  // Sets the interrupt budget based on wheter the function has a feedback vector
  // and any optimised code.
  pub fn set_interrupt_budget() {}

  pub fn compute_instance_size_with_min_slack() {}

  pub fn complete_in_object_slack_tracking_if_active() {}

  // Calculate the instance size and in-object properties count.
  pub fn calculate_instance_size_helper() {}

  // Functions related to feedback vector.
  // feedback_vector() can be used once the function has feedback vectors allocated.
  pub fn feedback_vector() {}
  pub fn has_feedback_vector() {}
  pub fn ensure_feedback_vector() {}

  // Functions related to closure feedback cell array that holds feedback cells
  // used to create closures from this function.
  // We allocate closure feedback cell arrays afte compile, when we want to allocate
  // feedback vectors lazilly.
  pub fn has_closure_feedback_cell_array() {}

  pub fn closure_feedback_cell_array() {}

  pub fn ensure_closure_feedback_cell_array() {}

  // Initializes the feedback cell of |function|.
  pub fn initialize_feedback_cell() {}

  // Unconditionally clear the type feedback vector.
  pub fn clear_type_feedback_info() {}

  // Resets function to clear compiled data after bytecode has been flushed.
  pub fn needs_reset_due_to_flushed_bytecode() {}

  pub fn reset_if_bytecode_flushed() {}

  pub fn set_initial_map() {}

  pub fn ensure_has_initial_map() {}

  // Creates a map that mathces the constructor's initial map
  pub fn get_derived_map() {}

  pub fn get_derived_rab_gsab_map() {}

  // Returns if this function has been compiled to native code yet.
  pub fn is_compiled() {}

  pub fn get_header_size() {}

  // Prints the name of the function.
  pub fn print_name() {}

  //pub fn get_name() {}

  pub fn set_name() {}

  // The function's name if it is configured, otherwise shared function info
  // debug name.
  pub fn get_debug_name() {}

  // The function's string representation implemented according to ES6.
  pub fn to_string() {}

  fn get_attached_code_kinds() {}

  fn get_available_code_kinds() {}
}
