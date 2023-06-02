#![allow(dead_code)]

struct LazyCallGraph {}
impl LazyCallGraph {
  pub fn new() {}
}

struct LCGEdge {}
impl LCGEdge {
  pub fn new() {}
  pub fn get_kind() {}
  pub fn is_call() {}
  pub fn get_node() {}
  pub fn get_function() {}
  fn set_kind() {}
}

struct LCGNode {}
impl LCGNode {
  pub fn new() {}
  pub fn get_graph() {}
  pub fn get_function() {}
  pub fn get_name() {}
  pub fn is_populated() {}
  pub fn is_dead() {}
  pub fn populate() {}
  fn populate_slow() {}
  fn replace_function() {}
  fn clear() {}
  fn dump() {}
}

struct SCC {}
impl SCC {
  pub fn new() {}
  pub fn clear() {}
  pub fn dump() {}
  pub fn size() {}
  pub fn get_outer_ref_scc() {}
  pub fn is_parent_of() {}
  pub fn is_ancestor_of() {}
  pub fn is_child_of() {}
  pub fn is_descendant_of() {}
  pub fn get_name() {}
}

struct LazyCallGraphAnalysis {}
impl LazyCallGraphAnalysis {
  pub fn new() {}
  pub fn run() {}
}

struct LazyCallGraphPrinterPass {}
impl LazyCallGraphPrinterPass {
  pub fn new() {}
  pub fn run() {}
}

struct LazyCallGraphDotPrinterPass {}
impl LazyCallGraphDotPrinterPass {
  pub fn new() {}
  pub fn run() {}
}