const ABSENT: i64 = -2;
pub const LENGTH: usize = 64;

struct Key {}

// Cache for mapping (map, property name) into descriptor index.
struct DescriptorLookupCache {
  keys_: [Key; LENGTH],
  results_: [u64; LENGTH],
}

impl DescriptorLookupCache {
  pub fn new() {}

  // Lookup descriptor index for (map, name).
  pub fn lookup() {}

  // Update an element in the cache.
  pub fn update() {}

  // Clear the cache.
  pub fn clear() {}

  fn hash() {}
}
