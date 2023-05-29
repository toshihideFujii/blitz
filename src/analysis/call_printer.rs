#![allow(dead_code)]

// This file defines external functions that can be called to
// explicitly instantiate the call graph printer.

// Pass for printing the call grah to a dot file.
struct CallGraphDotPrinterPass {}
impl CallGraphDotPrinterPass {
  pub fn new() {}
  pub fn run() {}
}

// Pass for viewing the call graph.
struct CallGraphViewerPass {}
impl CallGraphViewerPass {
  pub fn new() {}
  pub fn run() {}
}

pub fn create_call_graph_viewer_pass() {}
pub fn create_call_graph_dot_printer_pass() {}