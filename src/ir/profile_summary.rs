#![allow(dead_code)]

// This file defines the profile summary data structure.

struct ProfileSummaryEntry {}

enum Kind {
  Instr,
  CSInstr,
  Sample
}

struct ProfileSummary {}

impl ProfileSummary {
  pub fn new() {}
  pub fn get_kind() {}
  pub fn get_md() {}
  pub fn get_from_md() {}
  pub fn get_detailed_summary() {}
  pub fn get_num_functions() {}
  pub fn get_max_function_count() {}
  pub fn get_num_counts() {}
  pub fn get_total_count() {}
  pub fn get_max_count() {}
}