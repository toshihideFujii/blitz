#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use hlo::{
  hlo_computation::HloComputation,
  hlo_instruction::HloInstruction,
  hlo_module::HloModule, hlo_opcode::HloOpcode
};

#[derive(Debug, Clone, PartialEq)]
pub enum CallContext {
  Embedded,
  ControlFlow,
  Both,
  None,
}

pub fn call_context_to_string(context: &CallContext) -> String {
  match *context {
    CallContext::None => return "None".to_string(),
    CallContext::ControlFlow => return "ControlFlow".to_string(),
    CallContext::Embedded => return "Embedded".to_string(),
    CallContext::Both => return "Both".to_string(),
  }
}

pub fn get_instruction_call_context(opcode: &HloOpcode) -> CallContext {
  match *opcode {
    HloOpcode::Call => return CallContext::ControlFlow,
    HloOpcode::Conditional => return CallContext::ControlFlow,
    HloOpcode::While => return CallContext::ControlFlow,
    HloOpcode::AsyncStart => return CallContext::ControlFlow,
    HloOpcode::AsyncUpdate => return CallContext::ControlFlow,
    HloOpcode::AsyncDone => return CallContext::ControlFlow,

    HloOpcode::AllReduce => return CallContext::Embedded,
    HloOpcode::ReduceScatter => return CallContext::Embedded,
    HloOpcode::AllReduceStart => return CallContext::Embedded,
    HloOpcode::Map => return CallContext::Embedded,
    HloOpcode::Reduce => return CallContext::Embedded,
    HloOpcode::ReduceWindow => return CallContext::Embedded,
    HloOpcode::Scatter => return CallContext::Embedded,
    HloOpcode::SelectAndScatter => return CallContext::Embedded,
    HloOpcode::Sort => return CallContext::Embedded,
    HloOpcode::TopK => return CallContext::Embedded,
    HloOpcode::Fusion => return CallContext::Embedded,
    HloOpcode::CustomCall => return CallContext::Embedded,

    _ => return CallContext::None
  }
}

// Represents an HLO instruction which calls one or more computations.
pub struct CallSite {
  instruction: HloInstruction,
  called_computations: Vec<HloComputation>,
  context: CallContext
}

impl CallSite {
  pub fn new(
    instruction: HloInstruction,
    called_computations: Vec<HloComputation>,
    context: CallContext) -> Self
  {
    CallSite {
      instruction: instruction,
      called_computations: called_computations,
      context: context
    }
  }

  // Returns the instruction associated with this call site.
  pub fn instruction(&self) -> &HloInstruction {
    &self.instruction
  }

  pub fn mutable_instruction(&mut self) -> &mut HloInstruction {
    &mut self.instruction
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
pub struct CallGraphNode {
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

impl CallGraphNode {
  pub fn new(computation: HloComputation) -> Self {
    CallGraphNode {
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

  pub fn mutable_computation(&mut self) -> &mut HloComputation {
    &mut self.computation
  }

  // Returns the call sites in this computation. These are the instructions in
  // this computation which call other computations.
  pub fn callsites(&self) -> &Vec<CallSite> {
    &self.callsites
  }

  pub fn mutable_callsites(&mut self) -> &mut Vec<CallSite> {
    &mut self.callsites
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
      self.callers.push(caller.clone());
      self.caller_set.insert(caller.clone());
    }
    self.caller_callsites.push(caller_callsite);
  }

  // If instruction calls any computations adds a call site for this instruction
  // to the call graph node.
  fn add_call_site_for_instruction(
    &mut self,
    instruction: &HloInstruction,
    execution_threads: &HashSet<String>)
  {
    debug_assert_eq!(instruction.parent(), self.computation());
    let context = get_instruction_call_context(&instruction.opcode());
    if !instruction.called_computations().is_empty() {
      debug_assert!(context == CallContext::ControlFlow ||
        context == CallContext::Embedded);
      let callsite = CallSite::new(
        instruction.clone(),
        instruction.called_computations().clone(),
        context);
      self.callsites.push(callsite);
      self.callsite_instructions.insert(instruction.clone(), self.callsites.len());

      for callee in self.callsites.last().unwrap().called_computations() {
        if HloInstruction::is_thread_included(
          callee.execution_thread(),
          execution_threads) &&
          !self.callee_set.contains(callee)
        {
          self.callees.push(callee.clone());
          self.callee_set.insert(callee.clone());
        }
      }
    }
  }
}

pub struct CallGraph {
  module: HloModule,
  nodes: Vec<CallGraphNode>
}

impl CallGraph {
  pub fn new() {}

  // Builds and returns a call graph for the given HLO module. If a non-empty
  // execution_threads is provided, only computations that are in
  // execution_threads will be part of the returned call graph.
  pub fn build(
    _module: &HloModule,
    _execution_threads: &HashSet<String>) -> Self
  {
    unimplemented!()
  }

  // Returns the node associated with the given computation.
  pub fn get_node(&self, _computation: &HloComputation) -> &CallGraphNode {
    unimplemented!()
  }

  // Returns the vector of all nodes in the call graph.
  pub fn nodes(&self) -> &Vec<CallGraphNode> {
    &self.nodes
  }

  // Calls the given function on each node in the call graph.
  pub fn visit_nodes<F>(
    &self,
    visitor_func: F,
    visit_unreachable_nodes: bool
  ) -> Result<(), String>
    where F: Fn(&CallGraphNode) -> Result<(), String>
  {
    let visited: HashSet<CallGraphNode> = HashSet::new();
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

  // Returns true if 'a' dominates 'b' in the call graph.
  pub fn dominates(&self, _a: &HloComputation, _b: &HloComputation) -> bool {
    false
  }

  // Returns true if 'a' can reach 'b' in the call graph.
  pub fn can_reach(&self, a: &HloComputation, b: &HloComputation) -> bool {
    if a == b { return true; }
    let b_node = self.get_node(b);
    for b_caller in b_node.callers() {
      if self.can_reach(a, b_caller) { return true; }
    }
    false
  }

  // Returns whether 'instruction' is contained in 'computation' either directly
  // or indirectly.
  pub fn instruction_is_nested_in(
    &self, instruction: &HloInstruction, computation: &HloComputation) -> bool
  {
    self.dominates(computation, instruction.parent())
  }

  pub fn nearest_ancestors_in_same_computation() {}
  pub fn nearest_common_ancestor_instructions() {}
  pub fn nearest_common_ancestor_computations() {}
  pub fn nearest_common_ancestors_helper() {}

  // Returns whether the call graph is flattened.
  pub fn is_flattened(&self) -> bool {
    for node in self.nodes() {
      if node.context() == CallContext::Both {
        return false;
      }
      if node.context() == CallContext::ControlFlow &&
        !node.computation().is_async_computation() &&
         node.caller_callsites().len() > 1
      {
        return false;
      }
    }
    true
  }

  // Returns a vector of instructions calling the passed computation.
  pub fn get_computation_callers(&self, c: &HloComputation) -> Vec<HloInstruction> {
    let mut callers = Vec::new();
    for callsite in self.get_node(c).caller_callsites() {
      callers.push(callsite.instruction().clone())
    }
    callers
  }

  pub fn to_string() {}

  // Helper method for visit_nodes().
  // Traverses the call graph from 'node' in DFS post order (callee before
  // caller) calling visitor_func on each node.
  fn visit_nodes_internal<F>(
    &self,
    _visitor_func: &F,
    _node: &CallGraphNode,
    _visited: &HashSet<CallGraphNode>
  ) -> Result<(), String>
    where F: Fn(&CallGraphNode) -> Result<(), String>
  {
    
    Ok(())
  }
}