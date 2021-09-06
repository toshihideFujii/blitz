struct View {}

impl View {
  // Pushes an entry onto the worklist.
  pub fn push() {}

  // Pops an entry from the worklist.
  pub fn pop() {}

  // Returns true if the local portion of the worklist is empty.
  pub fn is_local_empty() {}

  // Returns true if the worklist is empty.
  // Can only be used from the main thread without concurrent access.
  pub fn is_empty() {}

  pub fn is_global_pool_empty() {}

  pub fn local_push_segment_size() {}

  pub fn flush_to_global() {}
}

const MAX_NUM_TASKS: i32 = 8;

// A concurrent worklist based on segments.
// Each tasks gets private push and pop segments.
// Empty pop segments are swapped with their corresponding push segments.
// Full push segments are published to a global pool of segments and
// replaced with empty segments.
struct Worklist {}

impl Worklist {
  pub fn new() {}

  // Swaps content with the given worklist.
  // Local buffers need to be empty, not thread safe.
  pub fn swap() {}

  pub fn push() {}

  pub fn pop() {}

  pub fn local_push_segment_size() {}

  pub fn is_local_empty() {}

  pub fn is_global_pool_empty() {}

  pub fn is_empty() {}

  pub fn are_locals_empty() {}

  pub fn local_size() {}

  // Thread-safe but may return an outdated result.
  pub fn global_pool_size() {}

  // Clears all segments.
  // Frees the global segment pool.
  // Assume that no other tasks are running.
  pub fn clear() {}

  pub fn update() {}
  pub fn iterate() {}
  pub fn iterate_global_pool() {}
  pub fn flush_to_global() {}
  pub fn merge_global_pool() {}
}
