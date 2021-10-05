// Mask constant for checking if a name has a computed hash code
// and if it is a string that is an integer index.
// The least significant bit indicates whether a hash code has been computed.
// If the hash code has been computed the 2nd bit tells whether the string can
// be used as an integer index (up to MAX_SAFE_INTEGER).
pub const HASH_NOT_COMPUTED_MASK: u64 = 1;
pub const IS_NOT_INTEGER_INDEX_MASK: u64 = 1 << 1;
pub const NOF_HASH_BIT_FIELDS: u64 = 2;

// Shift constant retrieving hash code from hash field.
pub const HASH_SHIFT: u64 = NOF_HASH_BIT_FIELDS;

// Only these bits are relevant in the hash, since the top two are shifted out.
pub const HASH_BIT_MASK: u64 = 0xffffffff >> HASH_SHIFT;

// Array index strings this short can keep their index in the hash field.
pub const MAX_CACHED_ARRAY_INDEX_LENGTH: u64 = 7;

// Maximum number of characters to consider when trying to convert a string value
// into an array index.
pub const MAX_ARRAY_INDEX_SIZE: u64 = 10;

pub const ARRAY_INDEX_VALUE_BITS: u64 = 24;

// The Name abstract class captures anything that can be used
// as a property name, i.e., strings and symbols.
// All names store a hash value.
struct Name {}

impl Name {
  // Tells whether the hash code has been computed.
  pub fn has_hash_code() -> bool {
    return Name::is_hash_field_computed(Name::raw_hash_field());
  }

  // Returns a hash value used for the property table.
  // Ensures that the hash value is computed.
  pub fn ensure_hash() {}

  // Returns a hash value used for the property table, assumes the
  // hash is already computed.
  pub fn hash() {}

  // Equality operations.
  pub fn equals() {}

  pub fn as_array_index() {}
  pub fn as_integer_index() {}

  pub fn contains_cached_array_index() {}

  pub fn to_function_name() {}

  pub fn name_short_print() {}

  pub fn is_hash_field_computed(raw_hash_field: u64) -> bool {
    return (raw_hash_field & HASH_NOT_COMPUTED_MASK) == 0;
  }

  fn raw_hash_field() -> u64 {
    return 0; // TODO this method is not defined in target class.
  }
}

struct Symbol {}

impl Symbol {
  // Whether this is a private symbol.
  pub fn is_private() {}

  // Whether this is a spec-defined well-known symbol or not.
  pub fn is_well_known_symbol() {}

  pub fn is_interesting_symbol() {}

  // Whether this is a symbol created by Symbol.for.
  pub fn is_in_public_symbol_table() {}

  pub fn is_private_name() {}
  pub fn set_is_private_name() {}

  pub fn is_private_brand() {}
  pub fn set_is_private_brand() {}
}
