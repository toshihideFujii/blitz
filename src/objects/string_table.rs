use super::name;

// A generic key for lookups into the string table, which allows heteromorphic
// lookup and on-demand creation of new strings.
struct StringTableKey {
  raw_hash_field_: u32,
  length_: i32,
}

impl StringTableKey {
  pub fn new(raw_hash_field: u32, length: i32) -> Self {
    StringTableKey {
      raw_hash_field_: raw_hash_field,
      length_: length,
    }
  }

  pub fn raw_hash_field(&self) -> u32 {
    self.raw_hash_field_
  }

  pub fn hash(&self) -> u32 {
    self.raw_hash_field_ >> name::HASH_SHIFT
  }

  pub fn length(&self) -> i32 {
    self.length_
  }

  fn set_raw_hash_field(&mut self, raw_hash_field: u32) {
    self.raw_hash_field_ = raw_hash_field
  }
}

// StringTable, for internalizing strings.
// The lookup methods are designed to be thread-safe,
// in combination with GC safepoints.
struct StringTable {}

impl StringTable {
  pub fn empty_element() {}
  pub fn deleted_element() {}

  pub fn capacity() {}
  pub fn number_of_elements() {}

  // Find string in the string table, using the given key.
  // If the string is not there yet, it is created (by the key) and added.
  // The return value is the string found.
  pub fn lookup_string() {}
  pub fn lookup_key() {}

  pub fn try_string_to_index_or_lookup_existing() {}

  pub fn print() {}
  pub fn get_current_memory_usage() {}

  pub fn iterate_elements() {}
  pub fn drop_old_data() {}
  pub fn notify_elements_removed() {}
}
