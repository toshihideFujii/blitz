#![allow(dead_code)]

// Blitz IR instance of the generic uniformity analysis.

struct UniformityInfoAnalysis {}
impl UniformityInfoAnalysis {
  pub fn new() {}
  pub fn run() {}
}

struct UniformityInfoPrinterPass {}
impl UniformityInfoPrinterPass {
  pub fn new() {}
  pub fn run() {}
}

struct UniformityInfoWrapperPass {}
impl UniformityInfoWrapperPass {
  pub fn new() {}
  pub fn get_uniformity_info() {}
  pub fn run_on_function() {}
  pub fn get_analysis_usage() {}
  pub fn release_memory() {}
  pub fn print() {}
}