#![allow(dead_code)]

use std::{collections::{HashMap, HashSet, VecDeque}, hash::Hash};

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
    // ControlFlow
    HloOpcode::Call => return CallContext::ControlFlow,
    HloOpcode::Conditional => return CallContext::ControlFlow,
    HloOpcode::While => return CallContext::ControlFlow,
    HloOpcode::AsyncStart => return CallContext::ControlFlow,
    HloOpcode::AsyncUpdate => return CallContext::ControlFlow,
    HloOpcode::AsyncDone => return CallContext::ControlFlow,
    // Embedded
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
    // None
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

  pub fn to_string(&self) -> String {
    let mut result = self.instruction().name();
    result.push_str(" calls in context ");
    result.push_str(&call_context_to_string(&self.context()));
    result.push_str(": ");
    for comp in self.called_computations() {
      result.push_str(&comp.name());
    }
    result
  }
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

// The call graph for an HLO module. The graph includes a node for each
// computation in the module.
pub struct CallGraph<'module> {
  module: &'module HloModule,
  nodes: Vec<CallGraphNode>,
  node_indices: HashMap<HloComputation, i64>,
  execution_threads: HashSet<String>,
}

impl<'module> CallGraph<'module> {
  pub fn new(module: &'module HloModule, execution_threads: HashSet<String>) -> Self {
    CallGraph {
      module: module,
      nodes: Vec::new(),
      node_indices: HashMap::new(),
      execution_threads: execution_threads
    }
  }

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
  pub fn get_node(&self, computation: &HloComputation) -> &CallGraphNode {
    debug_assert!(self.node_indices.contains_key(computation));
    &self.nodes[*self.node_indices.get(computation).unwrap() as usize]
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

  // Recursive helper for computing whether 'a' dominates 'b' in the call
  // graph. 'b_ancestor' is the currently visited node (which starts at 'b'),
  // and 'visited' is the set of computations which have been visited.
  fn dominates_helper(
    &self,
    a: &HloComputation,
    b: &HloComputation,
    visited: &mut HashSet<HloComputation>) -> bool
  {
    if a == b || visited.contains(b) {
      // The call graph is guaranteed to be acyclic so any previously visited node
      // we encounter was already determined to be dominated.
      return true;
    }
    let b_node = self.get_node(b);
    if b_node.callers().is_empty() {
      // We reached a root node without hitting 'a'. 'a' does not dominate 'b'.
      return false;
    }
    // Walk up the callers of 'b' until we hit 'a' or a root node (no callers).
    visited.insert(b.clone());
    for b_caller in b_node.callers() {
      if !self.dominates_helper(a, b_caller, visited) { return false; }
    }
    true
  }

  // Returns true if 'a' dominates 'b' in the call graph. Computation 'a'
  // dominates computation 'b' iff all callgraph paths in the caller-to-callee
  // direction from a root computation to 'b' pass through computation
  // 'a'. Trivially, a computation dominates itself.
  pub fn dominates(&self, a: &HloComputation, b: &HloComputation) -> bool {
    let mut visited = HashSet::new();
    self.dominates_helper(a, b, &mut visited)
  }

  // Returns true if 'a' can reach 'b' in the call graph. 'a' can reach 'b' if
  // 'a' is 'b' or 'a' can reach one of the callers of 'b'.
  pub fn can_reach(&self, a: &HloComputation, b: &HloComputation) -> bool {
    if a == b {
      return true;
    }
    let b_node = self.get_node(b);
    for b_caller in b_node.callers() {
      if self.can_reach(a, b_caller) { return true; }
    }
    false
  }

  // Returns whether 'instruction' is contained in 'computation' either directly
  // ('instruction->parent' is 'computation') or indirectly ('computation'
  // dominates 'instruction->parent' in the call graph).
  pub fn instruction_is_nested_in(
    &self, instruction: &HloInstruction, computation: &HloComputation) -> bool
  {
    self.dominates(computation, instruction.parent())
  }

  // Returns the nearest call graph ancestors of instructions 'a' and 'b' for
  // which the ancestors are in the same computation. An instruction is an call
  // graph ancestor of 'a' if the instruction calls the computation containing
  // 'a' either directly or transitively. Degeneratively an instruction is an
  // ancestor of itself. nullptr is returned if there is no common ancestor or
  // if the caller chain of 'a' or 'b' diverges (has multiple callers) before
  // the nearest common ancestor.
  //
  // Example:
  //
  // Entry computation:
  //   %x = Call(A, {Constant(42.0)})
  //   %y = Call(B, {%x})
  //
  // Computation A:
  //   %a = Negate(Param())
  //
  // Computation B:
  //   %b = Exp(Param());
  //
  // If called with %a and %b, this function would return (%x, %y). %x is an
  // ancestor of %a, and %y is an ancestor of %b, and %x and %y are in the same
  // computation.
  pub fn nearest_ancestors_in_same_computation(
    &self,
    a: &HloInstruction,
    b: &HloInstruction) -> (Option<&HloInstruction>, Option<&HloInstruction>)
  {
    // Lambda which returns the next instruction in the callee->caller chain in
    // the call graph. This is the unique instruction which calls the computation
    // containing 'instruction'. If more than one instruction calls the
    // computation containing 'instruction' or no instructions call the
    // computation then nullptr is returned.
    let next_caller =
      |instruction: Option<&HloInstruction>| -> Option<&HloInstruction>
    {
      let node =
        self.get_node(instruction.unwrap().parent());
      if node.caller_callsites().len() != 1 {
        if instruction.unwrap().parent().is_async_computation() {
          return Some(node.caller_callsites()[0].instruction());
        }
        return None;
      }
      Some(node.caller_callsites()[0].instruction())
    };

    // Iterate through the callee->caller chains and find the earliest common
    // element.
    let mut a_ancestor  = Some(a);
    let mut b_ancestor = Some(b);
    let a_depth = self.get_node(a.parent()).depth();
    let b_depth = self.get_node(b.parent()).depth();

    // Advance a_ancestor (b_ancestor) up the call chain until the call depth of
    // a_ancestor or b_ancestor are the same. Necessarily each call to next_caller
    // reduces the depth by exactly one.
    if a_depth > b_depth {
      for _i in 0..a_depth-b_depth {
        a_ancestor = next_caller(a_ancestor);
        if a_ancestor.is_none() {
          return (None, None);
        }
      }
    } else if b_depth > a_depth {
      for _i in 0..b_depth-a_depth {
        b_ancestor = next_caller(b_ancestor);
        if b_ancestor.is_none() {
          return (None, None);
        }
      }
    }

    while a_ancestor.is_some() && b_ancestor.is_some() {
      if a_ancestor.unwrap().parent() == b_ancestor.unwrap().parent() {
        //return (a_ancestor, b_ancestor);
      }
      a_ancestor = next_caller(a_ancestor);
      b_ancestor = next_caller(b_ancestor);
    }

    (None, None)
  }

  // Given a set of instructions within a computation, returns nearest common
  // ancestors as Hlo instructions (There could be multiple nearest common
  // ancestors in a DAG). If the given instructions are not in the same
  // computation, this function would report FAILURE.
  //
  // Unlike the `NearestAncestorsInSameComputation` defined above, it:
  //
  // (1) Only compute the nearest common ancestors within a computation, instead
  // of across computations (that's the function
  // `ComputationsNearestCommonAncestors` that defined below).
  //
  // (2) Takes in **a set of** Hlo instructions, instead of two Hlo
  // instructions, and find their nearest common ancestors.
  //
  // Example:
  //
  // Computation A:
  //   %p0   = Param(0)
  //   %p1   = Param(1)
  //   %p2   = Param(2)
  //   %add0 = Add(%p0, %p1)
  //   %mul0 = Mul(%p1, %p2)
  //   %sub0 = Sub(%add0, %mul0)
  //
  // If called with {%p0, %p1}, this function would return {%add0}.
  //
  // Please check the detailed example in
  // `CallGraphTest.NearestCommonAncestorInstructions`.
  pub fn nearest_common_ancestor_instructions(
    &self, instructions: &Vec<HloInstruction>) -> HashSet<HloInstruction>
  {
    if instructions.is_empty() {
      return HashSet::new();
    }

    // Check if all the instructions belong to the same computation.
    let computation = instructions[0].parent();
    for instr in instructions {
      assert_eq!(instr.parent(), computation);
    }

    self.nearest_common_ancestors_helper(instructions)
  }

  // Given a set of computations within a module, returns nearest common
  // ancestors as Hlo computations (There could be multiple nearest common
  // ancestors in a DAG).
  //
  // Entry_computation:
  //   %x = Call(A, {Constant(42.0)})
  //   %y = Call(B, {%x})
  //
  // Computation_A:
  //   %a = Negate(Param())
  //
  // Computation_B:
  //   %b = Exp(Param());
  //
  // If called with {Computation_A, Computation_B}, this function would return
  // {Entry_computation}.
  //
  // Please check the detailed example in
  // `CallGraphTest.NearestCommonAncestorComputations`.
  pub fn nearest_common_ancestor_computations(
    &self, computations: &Vec<HloComputation>) -> HashSet<HloComputation>
  {
    self.nearest_common_ancestors_helper(computations)
  }

  // A template helper function that computes the nearest common ancestors among
  // instructions/computations. `T` can be either `HloInstruction` or
  // `HloComputation`. Computing nearest common ancestors are basically the same
  // for HloInstruction and HloComputation. The only difference is that they
  // require different ways to access the ancestors of one node. Specifically,
  // the ancestors are users_instruction for instructions, and are
  // caller_computations for computations.
  //
  // The overall idea is to conduct BFS from the `starting_nodes`, and keep
  // track of the visited ancestors of each node. For each BFS step, we check if
  // there is a common node in all the visited ancestors, and if yes, that
  // common node is the nearest ancestor we are looking for. Note that, since we
  // are traversing DAG, there could be multiple nearest common ancestors. And
  // there must be at least one common ancestor (i.e., entry computations among
  // computations or root instruction among instructions).
  pub fn nearest_common_ancestors_helper<T>(
    &self, starting_nodes: &Vec<T>) -> HashSet<T> where T: Clone + Eq + Hash
  {
    if starting_nodes.is_empty() {
      return HashSet::new();
    }
    if starting_nodes.len() == 1 {
      let mut set = HashSet::new();
      set.insert(starting_nodes[0].clone());
      return set;
    }

    // There could be multiple nearest common ancestors in a DAG.
    let mut nearest_common_ancestors = HashSet::new();

    // Initialize `visited_ancestors` for each provided nodes.
    let mut visited_ancestors = Vec::new();
    for i in 0..starting_nodes.len() {
      let mut set = HashSet::new();
      set.insert(starting_nodes[i].clone());
      visited_ancestors.push(set);
    }

    // Initialize BFS queue for each provided nodes.
    let mut bfs_queues = Vec::new();
    for i in 0..starting_nodes.len() {
      let mut deque: VecDeque<T> = VecDeque::new();
      deque.push_back(starting_nodes[i].clone());
      bfs_queues.push(deque);
    }

    // Lambda to check if the BFS has finished (i.e., all queues in `bfs_queues`
    // are empty).
    let is_bfs_finished = || -> bool {
      for q in &bfs_queues {
        if !q.is_empty() { return false; }
      }
      true
    };

    // Lambda to check if there are common nodes in all the
    // `visited_ancestors`. Save results in `nearest_common_ancestors`. Return
    // true if they are found, otherwise return false.
    let mut find_common_nodes = || -> bool {
      let common_nodes = &visited_ancestors[0];
      for i in 1..visited_ancestors.len() {
        for k in common_nodes {
          if visited_ancestors[i].contains(k) {
            //common_nodes.remove(k);
          }
        }
      }
      nearest_common_ancestors = common_nodes.clone();
      !nearest_common_ancestors.is_empty()
    };

    // BFS body.
    // For each BFS step, we check if there is a common node in all the visited
    // ancestors (`find_common_nodes()`), and if yes, that common node is the
    // nearest ancestor we are looking for. Otherwise, we conduct BFS from each
    // bfs_queue, and update `bfs_queues` and `visited_ancestors` accordingly.
    while !is_bfs_finished() && !find_common_nodes() {
      /*
      for i in 0..bfs_queues.len() {
        let cur_queue = &mut bfs_queues[i];
        let mut next_queue: VecDeque<T> = VecDeque::new();
        let visited_ancestor = &mut visited_ancestors[i];

        while !cur_queue.is_empty() {
          let node = cur_queue.pop_back();
          // Identify ancestor of node.
          let ancestors_to_visit: Vec<T> = Vec::new();

          for ancestor in &ancestors_to_visit {
            if !visited_ancestor.contains(ancestor) {
              next_queue.push_back(ancestor.clone());
              visited_ancestor.insert(ancestor.clone());
            }
          }
        }
        //bfs_queues[i] = next_queue;
      }
      */
    }

    assert!(!nearest_common_ancestors.is_empty());

    // If one of the computed nearest common ancestors is inside
    // `starting_nodes`, we would only return the ones that are inside
    // `starting_nodes`.
    for nca in starting_nodes {
      if nearest_common_ancestors.contains(nca) {
        for nca in &nearest_common_ancestors {
          if !starting_nodes.contains(nca) {
            //nearest_common_ancestors.remove(nca);
          }
        }
      }
    }
    nearest_common_ancestors
  }

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

  pub fn to_string(&self) -> String {
    let mut out = "Call graph for module".to_string();
    out.push_str(&self.module.name());
    out.push_str(":\n");

    for node in self.nodes() {
      out.push_str("Computation ");
      out.push_str(&node.computation().name());
      out.push_str(":\n");
      out.push_str("  calls:\n");
      for callee in node.callees() {
        out.push_str("    ");
        out.push_str(&callee.name());
        out.push_str("\n");
      }
      out.push_str("  called by:\n");
      for caller in node.callers() {
        out.push_str("    ");
        out.push_str(&caller.name());
        out.push_str("\n");
      }
      out.push_str("  callsites:\n");
      for callsite in node.callsites() {
        out.push_str("    ");
        out.push_str(&callsite.to_string());
        out.push_str("\n");
      }
    }
    out
  }

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