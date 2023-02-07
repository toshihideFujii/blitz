#![allow(dead_code)]

// An instance of this class exists for every pass known by
// the system, and can be obtained from a live Pass by calling
// its get_pass_info() method.
struct PassInfo {}

impl PassInfo {
  pub fn get_pass_name() {}

  pub fn get_pass_argument() {}

  pub fn get_type_info() {}

  pub fn is_pass_id() {}

  pub fn is_analysis_group() {}
  pub fn is_analysis() {}

  pub fn is_cfg_only_pass() {}

  pub fn get_normal_ctor() {}
  pub fn set_normal_ctor() {}

  pub fn create_pass() {}

  pub fn add_interface_implemented() {}
  pub fn get_interface_implemented() {}
}