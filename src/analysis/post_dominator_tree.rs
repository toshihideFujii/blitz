#![allow(dead_code)]

// This file exposes interfaces to post dominate information.

struct PostDominatorTree {}
impl PostDominatorTree {
  pub fn new() {}
  pub fn invalidate() {}
  pub fn dominates() {}
}

struct PostDominatorTreeAnalysis {}
impl PostDominatorTreeAnalysis {
  pub fn new() {}
  pub fn run() {}
}

struct PostDominatorTreePrinterPass {}
impl PostDominatorTreePrinterPass {
  pub fn new() {}
  pub fn run() {}
}

struct PostDominatorTreeWrapperPass {}
impl PostDominatorTreeWrapperPass {
  pub fn new() {}
  pub fn get_post_dom_tree() {}
  pub fn run_on_function() {}
  pub fn verify_analysis() {}
  pub fn get_analysis_usage() {}
  pub fn release_memory() {}
  pub fn print() {}
}

pub fn create_post_dom_tree() {}