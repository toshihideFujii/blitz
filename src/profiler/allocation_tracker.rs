struct AllocationTraceNode {}

impl AllocationTraceNode {
  pub fn new() {}

  pub fn find_child() {}

  pub fn find_or_add_child() {}

  pub fn add_allocation() {}

  pub fn function_info_index() {}

  pub fn allocation_size() {}

  pub fn allocation_count() {}

  pub fn id() {}

  pub fn children() {}

  pub fn print() {}
}

struct AllocationTraceTree {}

impl AllocationTraceTree {
  pub fn new() {}

  pub fn add_path_from_end() {}

  pub fn root() {}

  pub fn next_node_id() {}

  pub fn print() {}
}

struct AddressToTraceMap {}

impl AddressToTraceMap {
  pub fn add_range() {}
  pub fn get_trace_node_id() {}
  pub fn move_object() {}
  pub fn clear() {}
  pub fn size() {}
  pub fn print() {}
}

struct UnresolvedLocation {}

impl UnresolvedLocation {
  pub fn resolve() {}
}

struct AllocationTracker {}

impl AllocationTracker {
  pub fn new() {}

  pub fn prepare_for_serialization() {}

  pub fn allocation_event() {}

  pub fn trace_tree() {}

  pub fn function_info_list() {}

  pub fn address_to_trace() {}

  fn add_function_info() {}

  fn function_info_index_for_vmstate() {}
}
