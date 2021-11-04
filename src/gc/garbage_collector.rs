struct Config {}
impl Config {}

// GC interface that allows abstraction over the actual GC invocation.
// This is needed to mock/fake GC for testing.
struct GarbageCollector {}

impl GarbageCollector {
  // Executes a garbage collection specified in config.
  pub fn collect_gargage() {}
  pub fn start_incremental_garbage_collection() {}

  // The current epoch that the GC maintains.
  // The epoch is increased on every GC invocation.
  pub fn epoch() {}
}
