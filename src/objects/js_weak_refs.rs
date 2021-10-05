struct JSFinalizationRegistry {}

impl JSFinalizationRegistry {
  pub fn register_weak_cell_with_unregister_token() {}

  pub fn unregister() {}

  pub fn remove_unregister_token() {}

  // Returns true if the cleared_cells list is non-empty.
  pub fn needs_cleanup() {}

  pub fn remove_cell_from_unregister_token_map() {}
}

// Internal object for storing weak references in JSFinalizationRegistry.
struct WeakCell {}

impl WeakCell {
  // Provide relaxed load access to target field.
  pub fn relaxed_target() {}

  // Provide relaxed load access to the unregister token field.
  pub fn relaxed_unregister_token() {}

  // Nullify is called during GC and it modifies the pointers in WeakCell
  // and JSFinalizationRegistry.
  pub fn nullify() {}

  pub fn remove_from_finalization_registry_cells() {}
}

struct JSWeakRef {}
