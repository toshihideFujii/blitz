#![allow(dead_code)]

// This file defines the interface for the loop nest analysis.

struct LoopNest {}
impl LoopNest {
  pub fn new() {}
  pub fn get_loop_nest() {}
  pub fn are_perfectly_nested() {}
  pub fn get_intervening_instructions() {}
  pub fn get_max_perfect_depth() {}
  pub fn skip_empty_block_until() {}
  pub fn get_outermost_loop() {}
  pub fn get_innermost_loop() {}
  pub fn get_loop() {}
  pub fn get_loop_index() {}
  pub fn get_num_loops() {}
  pub fn get_loops() {}
  pub fn get_loops_at_depth() {}
  pub fn get_perfect_loops() {}
  pub fn get_nest_depth() {}
  pub fn are_all_loops_rotated_form() {}
  pub fn get_parent() {}
  pub fn get_name() {}
  fn analyze_loop_nest_for_perfect_nest() {}
}

struct LoopNestAnalysis {}
impl LoopNestAnalysis {
  pub fn new() {}
  pub fn run() {}
}

struct LoopNestPrinterPass {}
impl LoopNestPrinterPass {
  pub fn new() {}
  pub fn run() {}
}