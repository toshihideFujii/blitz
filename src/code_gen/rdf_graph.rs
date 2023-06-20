#![allow(dead_code)]

struct NodeAttrs {}
impl NodeAttrs {
  pub fn new() {}
  pub fn type_() {}
  pub fn kind() {}
  pub fn flags() {}
  pub fn set_type() {}
  pub fn set_kind() {}
  pub fn set_flags() {}
  pub fn contains() {}
}

struct NodeAllocator {}
impl NodeAllocator {
  pub fn new() {}
  pub fn ptr() {}
  pub fn id() {}
  pub fn clear() {}
  fn start_new_block() {}
  fn need_new_block() {}
  fn make_id() {}
}

struct TargetOperandInfo {}
impl TargetOperandInfo {
  pub fn new() {}
  pub fn is_preserving() {}
  pub fn is_clobbering() {}
  pub fn is_fixed_reg() {}
}

struct PackedRegisterRef {}

struct LaneMaskIndex {}
impl LaneMaskIndex {
  pub fn new() {}
  pub fn get_lane_mask_for_index() {}
  pub fn get_index_for_lane_mask() {}
}

struct NodeBase {}
impl NodeBase {
  pub fn new() {}
  pub fn get_type() {}
  pub fn get_kind() {}
  pub fn get_flags() {}
  pub fn get_next() {}
  pub fn get_attrs() {}
  pub fn set_attrs() {}
  pub fn set_flags() {}
  pub fn append() {}
  pub fn init() {}
  pub fn set_next() {}
}

struct RefNode {}
impl RefNode {
  pub fn new() {}
  pub fn get_reg_ref() {}
  pub fn get_op() {}
  pub fn set_reg_ref() {}
  pub fn get_reaching_def() {}
  pub fn set_reaching_def() {}
  pub fn get_sibling() {}
  pub fn set_sibling() {}
  pub fn is_use() {}
  pub fn is_def() {}
  pub fn get_next_ref() {}
  pub fn get_reached_def() {}
  pub fn set_reached_def() {}
  pub fn get_reached_use() {}
  pub fn set_reached_use() {}
  pub fn link_to_def() {}
}

struct PhiUseNode {}
impl PhiUseNode {
  pub fn new() {}
  pub fn get_predecessor() {}
  pub fn set_predecessor() {}
}

struct CodeNode {}
impl CodeNode {
  pub fn new() {}
  pub fn get_code() {}
  pub fn set_code() {}
  pub fn get_first_member() {}
  pub fn get_last_member() {}
  pub fn add_member() {}
  pub fn add_member_after() {}
  pub fn remove_member() {}
  pub fn members() {}
  pub fn members_if() {}
}