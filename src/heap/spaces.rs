// Space is the abstract superclass for all allocation spaces that are
// not sealed after startup (i.e. not ReadOnlySpace)
struct Space {}

impl Space {
  pub fn move_external_backing_store_bytes() {}
  pub fn add_allocation_observer() {}
  pub fn remove_allocation_observer() {}
  pub fn pause_allocation_observers() {}
  pub fn resume_allocation_observers() {}
  pub fn start_next_inline_allocation_step() {}
  pub fn size_of_objects() {}
  pub fn available() {}
  pub fn round_size_down_to_object_alignment() {}
}
