
#![allow(dead_code)]

#[derive(PartialEq)]
pub struct HloModuleMetadata {
  next_pass_id: i64
}

impl HloModuleMetadata {
  pub fn new() {}
  pub fn proto() {}
  pub fn record_pass_start() {}
  pub fn record_pass_end() {}
  pub fn prepartitioning_metadata() {}
  pub fn set_prepartitioning_metadata() {}
  pub fn set_module_group_name() {}
  pub fn set_canonical_module_id() {}
  pub fn add_partitioning_module_id() {}
  pub fn current_pass_id() {}
  pub fn set_current_pass_name() {}
  pub fn set_current_pass_pipeline_name() {}
  pub fn add_current_pass_dump_filename() {}
  pub fn set_current_pass_module_changed() {}
  pub fn set_current_pass_module_id() {}
  pub fn add_current_pass_module_group_module_id() {}
}