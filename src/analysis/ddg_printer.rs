#![allow(dead_code)]

// This file defines the dot printer for the Data-Dependence Graph (DDG).

struct DDGDotPrinterPass {}
impl DDGDotPrinterPass {
  pub fn new() {}
  pub fn run() {}
}

struct DotGraphTraits {}
impl DotGraphTraits {
  pub fn new() {}
  pub fn get_graph_name() {}
  pub fn get_node_label() {}
  pub fn get_edge_attributes() {}
  pub fn is_node_hidden() {}

  fn get_simple_node_label() {}
  fn get_verbose_node_label() {}
  fn get_simple_edge_attributes() {}
  fn get_verbose_edge_attributes() {}
}