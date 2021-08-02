// Mask constant for checking if a name has a computed hash code
// and if it is a string that is an integer index.
// The least significant bit indicates whether a hash code has been computed.
// If the hash code has been computed the 2nd bit tells whether the string can
// be used as an integer index (up to MAX_SAFE_INTEGER).
pub const HASH_NOT_COMPUTED_MASK: i32 = 1;
pub const IS_NOT_INTEGER_INDEX_MASK: i32 = 1 << 1;
pub const NOF_HASH_BIT_FIELDS: i32 = 2;

// Shift constant retrieving hash code from hash foeld.
pub const HASH_SHIFT: i32 = NOF_HASH_BIT_FIELDS;

// The Name abstract class captures anything that can be used
// as a property name, i.e., strings and symbols.
// All names store a hash value.
struct Name {}
