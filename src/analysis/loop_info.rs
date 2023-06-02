#![allow(dead_code)]

struct Loop {}
impl Loop {
  pub fn new() {}
  pub fn get_loop_depth() {}
  pub fn get_header() {}
  pub fn get_parent_loop() {}
  pub fn get_outermost_loop() {}
  pub fn set_parent_loop() {}
  pub fn contains() {}
  pub fn get_sub_loops() {}
  pub fn get_sub_loops_vector() {}
  pub fn is_innermost() {}
  pub fn is_outermost() {}
  pub fn get_blocks() {}
  pub fn get_num_blocks() {}
  pub fn get_blocks_vector() {}
  pub fn get_blocks_set() {}
  pub fn is_invalid() {}
  pub fn is_loop_exiting() {}
  pub fn is_loop_latch() {}
  pub fn get_num_back_edges() {}
  pub fn get_exiting_blocks() {}
  pub fn get_exiting_block() {}
  pub fn get_exit_blocks() {}
  pub fn get_exit_block() {}
  pub fn has_dedicated_exits() {}
  pub fn get_unique_exit_blocks() {}
  pub fn get_unique_non_latch_exit_blocks() {}
  pub fn get_unique_exit_block() {}
  pub fn has_no_exit_blocks() {}
  pub fn get_exit_edges() {}
  pub fn get_loop_preheader() {}
  pub fn get_loop_predecessor() {}
  pub fn get_loop_latch() {}
  pub fn get_loop_latches() {}
  pub fn get_inner_loops_in_preorder() {}
  pub fn get_loops_in_preorder() {}
  pub fn add_basic_block_to_loop() {}
  pub fn replace_child_loop_with() {}
  pub fn add_child_loop() {}
  pub fn remove_child_loop() {}
  pub fn add_block_entry() {}
  pub fn reverse_block() {}
  pub fn reserve_blocks() {}
  pub fn move_to_header() {}
  pub fn remove_block_from_loop() {}
  pub fn verify_loop() {}
  pub fn verify_loop_nest() {}
  pub fn is_annotated_parallel() {}
  pub fn print() {}

  pub fn is_loop_invariant() {}
  pub fn has_loop_invariant_operands() {}
  pub fn make_loop_invariant() {}
  pub fn get_canonical_induction_variable() {}
  pub fn get_latch_cmp_inst() {}
  pub fn get_incoming_and_back_edge() {}
  pub fn get_bounds() {}
  pub fn get_induction_variable() {}
  pub fn get_induction_descriptor() {}
  pub fn is_auxiliary_induction_variable() {}
  pub fn get_loop_guard_branch() {}
  pub fn is_guard() {}
  pub fn is_rotated_form() {}
  pub fn is_canonical() {}
  pub fn is_lcssa_form() {}
  pub fn is_recursively_lcssa_form() {}
  pub fn is_loop_simplify_form() {}
  pub fn is_safe_to_close() {}
  pub fn get_loop_id() {}
  pub fn set_loop_id() {}
  pub fn set_loop_already_unrolled() {}
  pub fn set_loop_must_progress() {}
  pub fn dump() {}
  pub fn dump_verbose() {}
  pub fn get_start_loc() {}
  pub fn get_loc_range() {}
  pub fn get_name() {}
}

struct LoopInfo {}
impl LoopInfo {
  pub fn new() {}
  pub fn release_memory() {}
  pub fn allocate_loop() {}
  pub fn empty() {}
  pub fn get_loops_in_preorder() {}
  pub fn get_loops_in_reverse_sibling_preorder() {}
  pub fn get_loop_for() {}
  pub fn get_loop_depth() {}
  pub fn is_loop_header() {}
  pub fn get_top_level_loops() {}
  pub fn get_top_level_loop_vector() {}
  pub fn remove_loop() {}
  pub fn change_loop_for() {}
  pub fn change_top_level_loop() {}
  pub fn add_top_level_loop() {}
  pub fn remove_block() {}
  pub fn is_not_already_contained_in() {}
  pub fn analyze() {}
  pub fn print() {}
  pub fn verify() {}
  pub fn destroy() {}

  pub fn invalidate() {}
  pub fn erase() {}
  pub fn replacement_preserves_lcssa_form() {}
  pub fn movement_preserves_lcssa_form() {}
  pub fn would_be_out_loop_use_requiring_lcssa() {}
}

struct LoopInfoWrapperPass {}
impl LoopInfoWrapperPass {
  pub fn new() {}
  pub fn get_loop_info() {}
  pub fn run_on_function() {}
  pub fn verify_analysis() {}
  pub fn release_memory() {}
  pub fn print() {}
  pub fn get_analysis_usage() {}
}

pub fn print_loop() {}
pub fn find_option_md_for_loop_id() {}
pub fn find_option_md_for_loop() {}
pub fn get_optional_bool_loop_attribute() {}
pub fn get_boolean_loop_attribute() {}
pub fn get_optional_int_loop_attribute() {}
pub fn get_int_loop_attribute() {}
pub fn find_string_metadata_for_loop() {}
pub fn has_must_progress() {}
pub fn is_must_progress() {}
pub fn is_finite() {}
pub fn is_valid_as_access_group() {}
pub fn make_post_transformation_metadata() {}