#![allow(dead_code)]

// Stack safety analysis detects allocas and arguments with safe access.

struct StackSafetyInfo {}
impl StackSafetyInfo {
  pub fn new() {}
  pub fn get_info() {}
  pub fn print() {}
  pub fn get_param_accesses() {}
}

struct StackSafetyGlobalInfo {}
impl StackSafetyGlobalInfo {
  pub fn new() {}
  pub fn is_safe() {}
  pub fn stack_access_is_safe() {}
  pub fn print() {}
  pub fn dump() {}
}

struct StackSafetyAnalysis {}
impl StackSafetyAnalysis {
  pub fn new() {}
  pub fn run() {}
}

struct StackSafetyPrinterPass {}
impl StackSafetyPrinterPass {
  pub fn new() {}
  pub fn run() {}
}