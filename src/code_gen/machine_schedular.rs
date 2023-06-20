#![allow(dead_code)]

// This file provides an interface for customizing the standard
// MachineSchedular pass.

struct MachineSchedContext {}

struct MachineSchedRegistry {}
impl MachineSchedRegistry {
  pub fn new() {}
  pub fn get_next() {}
  pub fn get_list() {}
  pub fn set_listener() {}
}

struct MachineSchedPolisy {}

struct MachineSchedStrategy {}
impl MachineSchedStrategy {
  pub fn new() {}
  pub fn anchor() {}
  pub fn init_policy() {}
  pub fn dump_policy() {}
  pub fn should_track_pressure() {}
  pub fn should_track_lane_masks() {}
  pub fn do_mbb_sched_regions_top_down() {}
  pub fn initialize() {}
  pub fn enter_mbb() {}
  pub fn leave_mbb() {}
  pub fn register_roots() {}
  pub fn pick_node() {}
  pub fn schedule_tree() {}
  pub fn sched_node() {}
  pub fn release_top_node() {}
  pub fn release_bottom_node() {}
}

struct ScheduleDAGMI {}
impl ScheduleDAGMI {
  pub fn new() {}
  pub fn do_mbb_sched_regions_top_down() {}
  pub fn get_lis() {}
  pub fn have_reg_liveness() {}
  pub fn add_mutation() {}
  pub fn top() {}
  pub fn bottom() {}
  pub fn enter_region() {}
  pub fn schedule() {}
  pub fn start_block() {}
  pub fn finish_block() {}
  pub fn move_instruction() {}
  pub fn get_next_cluster_pred() {}
  pub fn get_next_cluster_succ() {}
  pub fn view_graph() {}

  pub fn post_process_dag() {}
  pub fn init_queues() {}
  pub fn update_queues() {}
  pub fn place_debug_values() {}
  pub fn dump_schedule() {}
  pub fn dump_schedule_trace_top_down() {}
  pub fn dump_schedule_trace_bottom_up() {}
  pub fn check_sched_limit() {}
  pub fn find_roots_and_bias_edges() {}
  pub fn release_succ() {}
  pub fn release_successors() {}
  pub fn release_pred() {}
  pub fn release_predecessors() {}
}