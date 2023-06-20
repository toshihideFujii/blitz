#![allow(dead_code)]

struct MemoryRegionInfo {}
impl MemoryRegionInfo {
  pub fn new() {}
  pub fn is_zero_fill() {}
  pub fn set_content() {}
  pub fn set_zero_fill() {}
  pub fn get_content() {}
  pub fn get_zero_fill_length() {}
  pub fn set_target_address() {}
  pub fn get_target_address() {}
}

struct RuntimeDyldChecker {}
impl RuntimeDyldChecker {
  pub fn new() {}
  pub fn check() {}
  pub fn check_all_rules_in_buffer() {}
  pub fn get_section_addr() {}
  pub fn get_section_load_address() {}
}