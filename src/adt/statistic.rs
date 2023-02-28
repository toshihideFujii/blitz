#![allow(dead_code)]

// This file defines the Statistic class, which is designed to
// be an easy way to expose verious metrics from passes.

struct TrackingStatistic {}
impl TrackingStatistic {
  pub fn new() {}

  pub fn get_debug_type() {}
  pub fn get_name() {}
  pub fn get_desc() {}
  pub fn get_value() {}
  pub fn update_max() {}
}

pub fn enable_statistics() {}
pub fn are_statistics_enabled() {}
pub fn create_info_output_file() {}
pub fn print_statistics() {}
pub fn print_statistics_json() {}
pub fn get_statistics() {}
pub fn reset_statistics() {}