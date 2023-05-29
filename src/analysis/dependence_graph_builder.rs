#![allow(dead_code)]

// This file defines a builder interface that can be used to
// populate dependence graphs such as DDG and PDG.

struct AbstractDependenceGraphBuilder {}
impl AbstractDependenceGraphBuilder {
  pub fn new() {}
  pub fn populate() {}
  pub fn compute_instruction_ordinals() {}
  pub fn create_fine_grained_nodes() {}
  pub fn create_def_use_edges() {}
  pub fn create_memory_dependencey_edges() {}
  pub fn create_and_connect_root_node() {}
  pub fn create_pi_blocks() {}
  pub fn simplify() {}
  pub fn sort_nodes_topologically() {}

  pub fn create_root_node() {}
  pub fn create_fine_grained_node() {}
  pub fn create_pi_block() {}
  pub fn create_def_use_edge() {}
  pub fn create_memory_edge() {}
  pub fn create_rooted_edge() {}
  pub fn get_nodes_in_pi_block() {}
  pub fn destroy_edge() {}
  pub fn destroy_node() {}
  pub fn should_create_pi_blocks() {}
  pub fn should_simplify() {}
  pub fn are_nodes_mergeable() {}
  pub fn merge_nodes() {}
  pub fn get_ordinal() {}
}