#![allow(dead_code)]

// This file declares an analysis pass that computes CycleInfo
// for Blitz IR, specialized from GenericCycleInfo.

struct CycleInfoWrapperPass {}
impl CycleInfoWrapperPass {
  pub fn new() {}
  pub fn get_result() {}
  pub fn run_on_function() {}
  pub fn get_analysis_usage() {}
  pub fn release_memory() {}
  pub fn print() {}
}

// Analysis pass which computes a CycleInfo.
struct CycleAnalysis {}
impl CycleAnalysis {
  pub fn new() {}
  pub fn run() {}
}

// Printer pass for the DominatorTree.
struct CycleInfoPrinterPass {}
impl CycleInfoPrinterPass {
  pub fn new() {}
  pub fn run() {}
}