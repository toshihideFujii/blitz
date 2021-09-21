pub enum ExecutionMode {
  BreakPoints,
  SideEffects,
}

pub enum SideEffectState {
  NotComputed,
  HasSideEffects,
  RequiresRuntimeChecks,
  HasNoSideEffect,
}

// The DebugInfo class holds additional information for a function
// being debugged.
struct DebugInfo {}

impl DebugInfo {
  // DebugInfo can be detached from the SharedFunctionImfo if it is empty.
  pub fn is_empty() {}

  pub fn debug_execution_mode() {}

  pub fn set_debug_execution_mode() {}
  pub fn has_instrumented_bytecode_array() {}

  pub fn original_bytecode_array() {}
  pub fn debug_bytecode_array() {}

  pub fn has_break_info() {}
  pub fn clear_break_info() {}

  // Accessors to flag whether to break before entering the function.
  pub fn set_break_at_entry() {}
  pub fn clear_break_at_entry() {}
  pub fn break_at_entry() {}

  // Check if there is a break point at a source position.
  pub fn has_break_point() {}
  pub fn clear_break_at_point() {}
  pub fn set_break_point() {}
  pub fn get_break_points() {}
  pub fn find_break_point_info() {}

  // Get the number of break points for this function.
  pub fn get_break_point_count() {}

  // Returns whether we should be able to break before entering the function.
  pub fn can_break_at_entry() {}

  // Indicates that the funcion should be skipped during stepping.
  pub fn debug_is_blackboxed() {}

  // Indicates that |debug_is_blackboxed| has been computed and set.
  pub fn computed_debug_is_blackboxed() {}

  // Indicates the side effect state.
  pub fn side_effect_state() {}

  pub fn get_side_effec_state() {}

  // Id assigned to the function for debugging.
  pub fn debugging_id() {}

  pub fn has_coverage_info() {}

  // Clears all fields related to block
  pub fn clear_coverage_info() {}

  // Get the break point info object for a source position.
  fn get_break_point_info() {}
}

// The BreakPointInfo class holds information for break points set
// in a function.
// The DebugInfo object holds a BreakPointInfo object for each code
// pointion with one or more break points.
struct BreakPointInfo {}

impl BreakPointInfo {
  // Removes a break point.
  pub fn clear_break_point() {}

  // Set a break point.
  pub fn set_break_point() {}

  // Check if break point info has this break point
  pub fn has_break_point() {}

  // Check if break point info has break point with this id.
  pub fn get_break_point_by_id() {}

  // Get the number of break points for this code offset.
  pub fn get_break_point_count() {}

  pub fn get_statement_position() {}
}

// Holds information related to block code coverage.
struct CoverageInfo {}

impl CoverageInfo {
  pub fn initialize_slot() {}

  pub fn resize_block_count() {}

  // Computes the size for a CoverageInfo instance of a given length.
  pub fn size_for() {}

  // Print debug info.
  pub fn coverage_info_print() {}
}

// Holds breakpoint related information.
// This object is used by inspector.
struct BreakPoint {}
