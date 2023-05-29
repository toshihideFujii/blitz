#![allow(dead_code)]

// This file provides interface used to build and manipulate a
// call graph, which is a very useful tool for interprocedural
// optimization.
//
// Every function in a module is represented as a node in the call
// graph. The callgraph node keeps track of which functions are 
// called by the function corresponding to the node.


// The basic data container for the call graph of a Module of IR.
struct CallGraph {}
impl CallGraph {
  pub fn new() {}
  pub fn print() {}
  pub fn dump() {}
  pub fn get_module() {}
  pub fn invalidate() {}
  pub fn get_external_calling_node() {}
  pub fn get_calls_external_node() {}
  pub fn replace_external_call_edge() {}
  pub fn remove_function_from_module() {}
  pub fn get_or_insert_function() {}
  pub fn populate_call_graph_node() {}
  pub fn add_to_call_graph() {}
}

// A node in the call graph for a module.
// Typically represents a function in the call graph.
// There are also special 'null' nodes used to represent theoretical entries
// in the call graph.
struct CallGraphNode {}
impl CallGraphNode {
  pub fn new() {}
  pub fn get_function() {}
  pub fn empty() {}
  pub fn size() {}
  pub fn get_num_references() {}
  pub fn dump() {}
  pub fn print() {}
  pub fn remove_all_called_functions() {}
  pub fn steal_called_functions_from() {}
  pub fn add_called_function() {}
  pub fn remove_call_edge() {}
  pub fn remove_call_edge_for() {}
  pub fn remove_any_call_edge_to() {}
  pub fn remove_one_abstract_edge_to() {}
  pub fn replace_call_edge_to() {}
}

// An analysis pass to compute the CallGraph for a Module.
// This class implements the concept of an analysis pass used by the
// ModuleAnalysisManager to run an analysis over a module and cache
// the resulting data.
struct CallGraphAnalysis {}
impl CallGraphAnalysis {
  pub fn new() {}
  pub fn run() {}
}

// Printer pass for the CallGraphAnalysis results.
struct CallGraphPrinterPass {}
impl CallGraphPrinterPass {
  pub fn new() {}
  pub fn run() {}
}

// Printer pass for the summarized CallGraphAnalysis results.
struct CallGraphSCCsPrinterPass {}
impl CallGraphSCCsPrinterPass {
  pub fn new() {}
  pub fn run() {}
}

// The ModulePass which wraps up a CallGraph and the logic to build it.
struct CallGraphWrapperPass {}
impl CallGraphWrapperPass {
  pub fn new() {}
  pub fn get_call_graph() {}
  pub fn get_module() {}
  pub fn get_external_calling_node() {}
  pub fn get_calls_external_node() {}
  pub fn remove_function_from_module() {}
  pub fn get_or_insert_function() {}
  pub fn get_analysis_usage() {}
  pub fn run_on_module() {}
  pub fn release_memojry() {}
  pub fn print() {}
  pub fn dump() {}
}