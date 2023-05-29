#![allow(dead_code)]

// This file defines the Data-Dependence Graph (DDG).

enum DDGNodeKind {
  Unknown,
  SingleInstruction,
  MultiInstruction,
  PiBlock,
  Root
}

struct DDGNode {}
impl DDGNode {
  pub fn new() {}
  pub fn get_kind() {}
  pub fn collect_instructions() {}
  pub fn set_kind() {}
}

struct RootDDGNode {}
impl RootDDGNode {
  pub fn new() {}
  pub fn class_of() {}
}

struct SimpleDDGNode {}
impl SimpleDDGNode {
  pub fn new() {}
  pub fn get_instructions() {}
  pub fn get_first_instruction() {}
  pub fn get_last_instruction() {}
  pub fn class_of() {}
  fn append_instructions() {}
}

struct PiBlockDDGNode {}
impl PiBlockDDGNode {
  pub fn new() {}
  pub fn get_nodes() {}
  pub fn class_of() {}
}

enum DDGEdgeKind {
  Unknown,
  RegisterDefUse,
  MemoryDependence,
  Rooted
}

struct DDGEdge {}
impl DDGEdge {
  pub fn new() {}
  pub fn get_kind() {}
  pub fn is_def_use() {}
  pub fn is_memory_dependence() {}
  pub fn is_rooted() {}
}

struct DataDependenceGraph {}
impl DataDependenceGraph {
  pub fn new() {}
  pub fn get_pi_block() {}
  pub fn add_node() {}
}

struct DDGBuilder {}
impl DDGBuilder {
  pub fn new() {}
  pub fn create_root_node() {}
  pub fn create_fine_grained_node() {}
  pub fn create_pi_block() {}
  pub fn create_def_use_edge() {}
  pub fn create_memory_edge() {}
  pub fn create_rooted_edge() {}
  pub fn get_nodes_in_pi_block() {}
  pub fn are_nodes_mergeable() {}
  pub fn merge_nodes() {}
  pub fn should_simplify() {}
  pub fn should_create_pi_blocks() {}
}

// Analysis pass that builds the DDG for a loop.
struct DDGAnalysis {}
impl DDGAnalysis {
  pub fn new() {}
  pub fn run() {}
}

// Textual printer pass for the DDG of a loop.
struct DDGAnalysisPrinterPass {}
impl DDGAnalysisPrinterPass {
  pub fn new() {}
  pub fn run() {}
}