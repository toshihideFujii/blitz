struct Predicate {}

impl Predicate {
  pub fn predicate() {}
  pub fn get() {}
  fn calculate_value() {}
}

struct CacheEntry {}

impl CacheEntry {
  pub fn code_point() {}
  pub fn value() {}
}

struct Utf16 {}

impl Utf16 {
  pub fn is_surrogate_pair() {}
  pub fn is_lead_surrogate() {}
  pub fn is_trail_surrogate() {}
  pub fn combine_surrogate_pair() {}
  pub fn lead_surrogate() {}
  pub fn trail_surrogate() {}
  pub fn has_unpaired_surrogate() {}
}

struct Latin1 {}

impl Latin1 {
  // Convert the charcter to Latin-1 case equivalent if possioble.
  pub fn try_convert_to_latin1() {}
}

pub fn is_line_terminator(c: u32) -> bool {
  c == 0x000A || c == 0x000D || c == 0x2028 || c == 0x2029
}
