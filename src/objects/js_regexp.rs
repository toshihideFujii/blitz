pub enum Type {
  NotCompiled, // Initial value. No data has been stored in the JSRegExp yet.
  Atom,        // A simple string to match against using an indexOf operation.
  IrregExp,    // Compiled with IrregExp.
  Experimental, // Compiled to use the new linear time engine.
}

// Maximum number of captures allowed.
const MAX_CAPTURES: u64 = 1 << 16;

// Regular expressions
// The regular expression holds a single reference to a FixedArray
// in the DataOffset field.
// The FixedArray contains the following data:
// - tag: type of regexp implementation (not compiled yet, atom or irregexp)
// - reference to the original source string
// - reference to the original flag string
struct JSRegExp {}

impl JSRegExp {
  pub fn new() {}

  pub fn initialize() {}

  pub fn flag_from_char() {}

  pub fn flags_from_string() {}

  pub fn string_from_flags() {}

  pub fn can_tier_up() {}

  pub fn marked_for_tier_up() {}

  pub fn reset_last_tier_up_tick() {}

  pub fn tier_up_tick() {}

  pub fn mark_tier_up_for_next_exec() {}

  pub fn type_tag() {}

  pub fn type_supports_captures() {}

  // Number of captures (without the match itself).
  pub fn capture_count() {}

  // Each capture (including the match itself) needs two registers.
  pub fn registers_for_capture_count() {}

  pub fn max_register_count() {}

  pub fn get_flags() {}

  pub fn pattern() {}

  pub fn escaped_pattern() {}

  pub fn capture_name_map() {}

  pub fn data_at() {}

  pub fn set_data_at() {}

  pub fn set_capture_name_map() {}

  pub fn code_index() {}

  pub fn bytecode_index() {}

  pub fn code() {}

  pub fn bytecode() {}

  pub fn should_produce_bytecode() {}

  pub fn has_compiled_code() {}

  pub fn discard_compiled_code_for_serialization() {}

  pub fn backtrack_limit() {}
}
