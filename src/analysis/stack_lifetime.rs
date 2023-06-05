#![allow(dead_code)]


struct StackLifetime {}
impl StackLifetime {
  pub fn new() {}
  pub fn run() {}
  pub fn get_markers() {}
  pub fn get_live_range() {}
  pub fn is_reachable() {}
  pub fn is_alive_after() {}
  pub fn get_full_live_range() {}
  pub fn print() {}
}

struct StackLifetimePrinterPass {}
impl StackLifetimePrinterPass {
  pub fn new() {}
  pub fn run() {}
  pub fn print_pipeline() {}
}