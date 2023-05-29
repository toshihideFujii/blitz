#![allow(dead_code)]

struct CFGViewerPass {}
impl CFGViewerPass {
  pub fn new() {}
  pub fn run() {}
}

struct CFGOnlyViewerPass {}
impl CFGOnlyViewerPass {
  pub fn new() {}
  pub fn run() {}
}

struct CFGPrinterPass {}
impl CFGPrinterPass {
  pub fn new() {}
  pub fn run() {}
}

struct CFGOnlyPrinterPass {}
impl CFGOnlyPrinterPass {
  pub fn new() {}
  pub fn run() {}
}

struct DotFuncInfo {}
impl DotFuncInfo {
  pub fn new() {}
  pub fn get_bfi() {}
  pub fn get_bpi() {}
  pub fn get_function() {}
  pub fn get_max_freq() {}
  pub fn get_freq() {}
  pub fn set_heat_colors() {}
  pub fn show_heat_colors() {}
  pub fn set_raw_edge_weights() {}
  pub fn use_raw_edge_weights() {}
  pub fn set_edge_weights() {}
  pub fn show_edge_weights() {}
}