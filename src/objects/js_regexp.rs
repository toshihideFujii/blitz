pub enum Type {
  NotCompiled, // Initial value. No data has been stored in the JSRegExp yet.
  Atom,        // A simple string to match against using an indexOf operation.
  IrregExp,    // Compiled with IrregExp.
  Experimental, // Compiled to use the new linear time engine.
}

// Maximum number of captures allowed.
const MAX_CAPTURES: u64 = 1 << 16;

// This is already an in-object field.
const LAST_INDEX_OFFSET: u64 = 0; // TODO: HEADER_SIZE

// The initial value of the last index field on a new JSRegExp instance.
const INITIAL_LAST_INDEX_VALUE: u64 = 0;

// Indices in the data array.
const TAG_INDEX: u64 = 0;
const SOURCE_INDEX: u64 = TAG_INDEX + 1;
const FLAGS_INDEX: u64 = SOURCE_INDEX + 1;
const DATA_INDEX: u64 = FLAGS_INDEX + 1;

const MIN_DATA_ARRAY_LENGTH: u64 = DATA_INDEX;

// The data fields are used in different ways depending on the value of the tag.
const ATOM_PATTERN_INDEX: u64 = DATA_INDEX;

const ATOM_DATA_SIZE: u64 = ATOM_PATTERN_INDEX + 1;

const IRREGEXP_LATIN1_CODE_INDEX: u64 = DATA_INDEX;
const IRREGEXP_UC16_CODE_INDEX: u64 = DATA_INDEX + 1;
const IRREGEXP_LATIN1_BYTECODE_INDEX: u64 = DATA_INDEX + 2;
const IRREGEXP_UC16_BYTECODE_INDEX: u64 = DATA_INDEX + 3;
const IRREGEXP_MAX_REGISTER_COUNT_INDEX: u64 = DATA_INDEX + 4;
const IRREGEXP_CAPTURE_COUNT_INDEX: u64 = DATA_INDEX + 5;
const IRREGEXP_CAPTURE_NAME_MAP_INDEX: u64 = DATA_INDEX + 6;
const IRREGEXP_TICKS_UNTIL_TIER_UP_INDEX: u64 = DATA_INDEX + 7;
const IRREGEXP_BACKTRACK_LIMIT: u64 = DATA_INDEX + 8;
const IRREGEXP_DATA_SIZE: u64 = DATA_INDEX + 9;

const EXPERIMENTAL_DATA_SIZE: u64 = IRREGEXP_DATA_SIZE;

// In-object fields.
const LAST_INDEX_FIELD_INDEX: u64 = 0;
const IN_OBJECT_FIELD_COUNT: u64 = 1;

// Descriptor array index to important methods in the prototype.
const EXEC_FUNCTION_DESCRIPTOR_INDEX: u64 = 1;
const SYMBOL_MATCH_FUNCTION_DESCRIPTOR_INDEX: u64 = 13;
const SYMBOL_MATCH_ALL_FUNCTION_DESCRIPTOR_INDEX: u64 = 14;
const SYMBOL_REPLACE_FUNCTION_DESCRIPTOR_INDEX: u64 = 15;
const SYMBOL_SEARCH_FUNCTION_DESCRIPTOR_INDEX: u64 = 16;
const SYMBOL_SPLIT_FUNCTION_DESCRIPTOR_INDEX: u64 = 17;

// The uninitialized value for a regexp code object.
const UNINITIALIZED_VALUE: i64 = -1;

const TIER_UP_FOR_SUBJECT_LENGTH_VALUE: u64 = 1000;

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

  // The actual object size including in-object fields.
  pub fn size() {}
}

struct JSRegExpResult {}

struct JSRegExpResultWithIndices {}

struct JSRegExpResultIndices {}
