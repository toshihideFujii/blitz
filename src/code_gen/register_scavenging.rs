#![allow(dead_code)]

struct ScavengeInfo {}

struct RegScavenger {}
impl RegScavenger {
  pub fn new() {}
  pub fn assign_reg_to_scavenging_index() {}
  pub fn enter_basic_block() {}
  pub fn enter_basic_block_end() {}
  pub fn forward() {}
  pub fn backward() {}
  pub fn skip_to() {}
  pub fn get_current_position() {}
  pub fn is_reg_used() {}
  pub fn get_regs_available() {}
  pub fn find_unused_reg() {}
  pub fn add_scavenging_frame_index() {}
  pub fn is_scavenging_frame_index() {}
  pub fn get_scavenging_frame_indices() {}
  pub fn scavenge_register() {}
  pub fn scavenge_register_backwards() {}
  pub fn set_reg_used() {}
}

pub fn scavenge_frame_virtual_regs() {}