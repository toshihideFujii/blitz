#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use hlo::{
  hlo_computation::HloComputation,
  hlo_instruction::HloInstruction,
  hlo_module::HloModule
};

#[derive(Clone, PartialEq)]
pub enum CallContext {
  Embedded,
  ControlFlow,
  Both,
  None,
}

pub fn call_context_to_string() {}

pub fn get_instruction_call_context() {}

// Represents an HLO instruction which calls one or more computations.
pub struct CallSite {
  instruction: HloInstruction,
  called_computations: Vec<HloComputation>,
  context: CallContext
}

impl CallSite {
  pub fn new() {}

  // Returns the instruction associated with this call site.
  pub fn instruction(&self) -> &HloInstruction {
    &self.instruction
  }

  // Returns the computations called at this call site.
  pub fn called_computations(&self) -> &Vec<HloComputation> {
    &self.called_computations
  }

  // Returns the context in which computations are called at this call site.
  pub fn context(&self) -> CallContext {
    self.context.clone()
  }

  pub fn to_string() {}
}

// A node in the call graph representing an HLO computation.
pub struct CallGrapgNode {
  computation: HloComputation,
  callees: Vec<HloComputation>,
  callee_set: HashSet<HloComputation>,
  callers: Vec<HloComputation>,
  caller_set: HashSet<HloComputation>,
  callsites: Vec<CallSite>,
  callsite_instructions: HashMap<HloInstruction, usize>,
  caller_callsites: Vec<CallSite>,
  context: CallContext,
  depth: i64,
}

impl CallGrapgNode {
  pub fn new(computation: HloComputation) -> Self {
    CallGrapgNode {
      computation: computation,
      callees: Vec::new(),
      callee_set: HashSet::new(),
      callers: Vec::new(),
      caller_set: HashSet::new(),
      callsites: Vec::new(),
      callsite_instructions: HashMap::new(),
      caller_callsites: Vec::new(),
      context: CallContext::None,
      depth: 0
    }
  }

  // Returns the computation represented by his call graph node.
  pub fn computation(&self) -> &HloComputation {
    &self.computation
  }

  // Returns the call sites in this computation.
  pub fn callsites(&self) -> &Vec<CallSite> {
    &self.callsites
  }

  // Returns the callsite associated with the given instruction.
  pub fn get_call_site(&self, instruction: &HloInstruction) -> Option<&CallSite> {
    let index = self.callsite_instructions.get(instruction);
    if index.is_none() {
      return None;
    }
    self.callsites.get(*index.unwrap())
  }

  // Returns the computations called by this computation.
  pub fn callees(&self) -> &Vec<HloComputation> {
    &self.callees
  }

  // Returns the call sites in other computations which call this computation.
  pub fn caller_callsites(&self) -> &Vec<CallSite> {
    &self.caller_callsites
  }

  // Returns the computations which call this computation.
  pub fn callers(&self) -> &Vec<HloComputation> {
    &self.callers
  }

  // Returns the context in which ehis computation is called.
  pub fn context(&self) -> CallContext {
    self.context.clone()
  }

  // Returns the depth of this node in the call graph.
  pub fn depth(&self) -> i64 {
    self.depth
  }

  pub fn to_string(&self) -> String {
    self.computation.name()
  }

  // Sets the context in which this computation is called.
  fn set_context(&mut self, context: CallContext) {
    self.context = context;
  }

  // Sets the depth of this node in the graph.
  fn set_depth(&mut self, depth: i64) {
    self.depth = depth;
  }

  // Adds a callsite which calls this computation.
  fn add_caller_call_site(&mut self, caller_callsite: CallSite) {
    let caller = caller_callsite.instruction().parent();
    if !self.callee_set.contains(caller) {
      // TODO
    }
    self.caller_callsites.push(caller_callsite);
  }

  // If instruction calls any computations adds a call site for this instruction
  // to the call graph node.
  fn add_call_site_for_instruction(&mut self, _instruction: &HloInstruction) {}
}

pub struct CallGraph {
  module: HloModule,
  nodes: Vec<CallGrapgNode>
}

impl CallGraph {
  pub fn new() {}

  // Builds and return a call graph for the given HLO module.
  pub fn build(
    _module: HloModule,
    _execution_threads: Option<HashSet<String>>) -> Option<Self>
  {
    None
  }

  // Returns the node associated with the given computation.
  pub fn get_node(&self, _computation: &HloComputation) -> &CallGrapgNode {
    unimplemented!()
  }

  pub fn nodes() {}

  // Calls the given function on each node in the call graph.
  pub fn visit_nodes<F>(
    &self,
    visitor_func: F,
    visit_unreachable_nodes: bool
  ) -> Result<(), String>
    where F: Fn(&CallGrapgNode) -> Result<(), String>
  {
    let visited: HashSet<CallGrapgNode> = HashSet::new();
    if visit_unreachable_nodes {
      // Traverse from all roots in the call graph.
      for node in &self.nodes {
        if node.callers.is_empty() {
          return self.visit_nodes_internal(&visitor_func, node, &visited);
        }
      }
    } else {
      // Traverse only from the entry computation.
      return self.visit_nodes_internal(
        &visitor_func,
        self.get_node(self.module.entry_computation().unwrap()),
        &visited);
    }
    Ok(())
  }

  pub fn dominates() {}
  pub fn can_reach() {}
  pub fn instruction_is_nested_in() {}
  pub fn nearest_ancestors_in_same_computation() {}
  pub fn nearest_common_ancestor_instructions() {}
  pub fn nearest_common_ancestor_computations() {}
  pub fn nearest_common_ancestors_helper() {}
  pub fn is_flattened() {}
  pub fn get_computation_callers() {}
  pub fn to_string() {}

  // Helper method for visit_nodes().
  // Traverses the call graph from 'node' in DFS post order (callee before
  // caller) calling visitor_func on each node.
  fn visit_nodes_internal<F>(
    &self,
    _visitor_func: &F,
    _node: &CallGrapgNode,
    _visited: &HashSet<CallGrapgNode>
  ) -> Result<(), String>
    where F: Fn(&CallGrapgNode) -> Result<(), String>
  {
    Ok(())
  }
}