#![allow(dead_code)]

use std::collections::HashMap;

use hlo::{
  hlo_computation::HloComputation, hlo_instruction::HloInstruction, hlo_module::HloModule,
  hlo_opcode::HloOpcode, hlo_reachability::HloReachabilityMap,
  hlo_schdule::{HloInstructionSequence, HloSchedule}, hlo_value::{HloUse, HloValue}
};

use service::call_graph::CallGraph;
use crate::hlo_dataflow_analysis::HloDataflowAnalysis;

#[derive(Clone, PartialEq)]
pub enum ExecutionConstraint {
  IsSame,
  RunBeforeStart,
  RunBeforeEnd,
  RunExclusiveBefore,
  RunExclusiveAfter,
  RunAfter,
  Unordered,
}

// Base class for describing a partial ordering of HLO instructions.
pub struct HloOrdering<'module> {
  module: HloModule,
  call_graph: CallGraph<'module>,
}

impl<'module> HloOrdering<'module> {
  pub fn new() {}

  // Return the execution constraint between a and b.
  pub fn get_execution_constraint(
    &self, _a: &HloInstruction, _b: &HloInstruction) -> ExecutionConstraint
  {
    unimplemented!()
  }

  // Returns true if instruction 'a' executes befoere instruction 'b'.
  pub fn executes_before(&self, a: &HloInstruction, b: &HloInstruction) -> bool {
    match self.get_execution_constraint(a, b) {
      ExecutionConstraint::IsSame => return false,
      ExecutionConstraint::RunBeforeStart => return true,
      ExecutionConstraint::RunBeforeEnd => return true,
      ExecutionConstraint::RunExclusiveBefore => return true,
      ExecutionConstraint::RunExclusiveAfter => return false,
      ExecutionConstraint::RunAfter => return false,
      ExecutionConstraint::Unordered => return false,
    }
  }

  // Returns whether the value 'a' is defined befoer the value 'b' under the
  // given ordering.
  pub fn is_defined_before(&self, a: &HloValue, b: &HloValue) -> bool {
    // Entry parameter shoould always be defined before other instructions.
    let module = b.defining_instruction().get_module().as_ref().unwrap();
    if b.defining_instruction().parent() == module.entry_computation().unwrap() &&
       b.defining_instruction().opcode() == HloOpcode::Parameter
    {
      return false;
    }

    if a.defining_instruction().parent() == module.entry_computation().unwrap() &&
       a.defining_instruction().opcode() == HloOpcode::Parameter
    {
      return true;
    }

    // Phi values require special handling.
    let is_body_or_condition_phi = |v: &HloValue| -> bool {
      v.is_phi() && v.defining_instruction().opcode() == HloOpcode::Parameter
    };
    if is_body_or_condition_phi(a) &&
      !is_body_or_condition_phi(b) &&
       self.call_graph.instruction_is_nested_in(
        b.defining_instruction(), a.defining_instruction().parent())
    {
      return true;
    }
    if is_body_or_condition_phi(b) &&
       self.call_graph.instruction_is_nested_in(
        a.defining_instruction(), b.defining_instruction().parent())
    {
      return false;
    }

    // If 'b' is a while phi and 'a' is in the body or condition, then 'a'
    // executes before 'b'.
    if b.is_phi() &&
       b.defining_instruction().opcode() == HloOpcode::While &&
      (self.call_graph.instruction_is_nested_in(
        a.defining_instruction(), b.defining_instruction().while_body()) ||
       self.call_graph.instruction_is_nested_in(
        a.defining_instruction(), b.defining_instruction().while_condition())
      )
    {
      return true;
    }

    // If 'b' is a conditional phi and 'a' is in some branch computation, then 'a'
    // executes before 'b'.
    if b.is_phi() &&
       b.defining_instruction().opcode() == HloOpcode::Conditional
    {
      for j in 0..b.defining_instruction().branch_count() {
        if self.call_graph.instruction_is_nested_in(
          a.defining_instruction(), b.defining_instruction().branch_computation(j))
        {
          return true;
        }
      }
    }
      
    self.executes_before(a.defining_instruction(), b.defining_instruction())
  }

  // Returns whether the given use is before the given value definition under
  // the given ordering.
  pub fn uses_before_value_definition(
    &self,
    mut uses: Vec<HloUse>,
    value: &HloValue,
    dataflow: &HloDataflowAnalysis,
    use_is_always_before_def_in_same_instr: bool) -> bool
  {
    let mut has_use_in_exclusive_banches = false;
    let mut has_escaped_use_in_conditional = false;

    let mut use_is_before_value_definition = |use_: &mut HloUse| -> bool {
      println!("use_is_before_value_definition(use={:?}, value={:?})",
        use_.to_string(), value.to_short_string());

      match self.get_execution_constraint(&use_.instruction, value.defining_instruction()) {
        ExecutionConstraint::IsSame => {
          // If the use is at the instruction where the value is defined, then the
          // use is before the def if the instruction allows buffer sharing (in
          // place computation).
          if use_is_always_before_def_in_same_instr ||
             dataflow.can_share_operand_buffer_with_user(
              use_.instruction.mutable_operand(use_.operand_number as usize).unwrap(),
              &use_.operand_index_vec,
              value.defining_instruction(),
              value.defining_index())
          {
            println!(" use is  value def, and instruction can share use buffer.");
            return true;
          }
        },
        ExecutionConstraint::RunExclusiveAfter => {
          // If the use is located in a branch that is exclusive to the branch
          // where value is located, in order for them to interfere, there must be
          // an execution path where the value's definition can reach the use, so
          // that the wrong value would reach use if their live ranges are merged.
          println!(" use and value def are in exclusive branches.");
          if !has_escaped_use_in_conditional {
            has_use_in_exclusive_banches = true;
            println!("Allowing them to share buffer.\n");
            return true;
          }
          println!("value def has escaped use in conditional. \n");
        },
        ExecutionConstraint::RunExclusiveBefore => {
          println!(" use instruction executes before value-defining instruction.");
          return true;
        },
        ExecutionConstraint::RunBeforeStart => {
          println!(" use instruction executes before value-defining instruction.");
          return true;
        },
        ExecutionConstraint::RunBeforeEnd => {
          println!(" use instruction executes before value-defining instruction.");
          return true;
        },
        ExecutionConstraint::RunAfter => {
          // Treat CollectivePermuteDone as a special case as it shares the buffer
          // from its operand (CollectivePermuteStart).
          if use_is_always_before_def_in_same_instr &&
            use_.instruction.opcode() == HloOpcode::CollectivePermuteDone &&
            use_.instruction.operand(0) == value.instruction()
          {
            return true;
          }
        },
        ExecutionConstraint::Unordered => {},
      }

      // The use at a while is an input to a phi, and logically occurs before
      // values are defined in the body.
      if use_.instruction.opcode() == HloOpcode::While {
        let blitz_while = &use_.instruction;
        if self.call_graph.instruction_is_nested_in(
          value.defining_instruction(), blitz_while.while_body())
        {
          println!("  use is while {:?} and def is in body.", blitz_while.name());
          return true;
        }
        if self.call_graph.instruction_is_nested_in(
          value.defining_instruction(), blitz_while.while_condition())
        {
          if value.defining_instruction() !=
            blitz_while.while_condition().parameter_instruction(0).unwrap()
          {
            println!("  use is while {:?} and def is in condition and is not the parameter.",
              blitz_while.name());
            return false;
          } else {
            println!("  use is while {:?} and def is in condition and is the parameter.",
              blitz_while.name());
            return true;
          }
        }
      }
      // Similary if the value is defined at a while, it logically occurs after
      // any uses in the body or condition computations.
      if value.defining_instruction().opcode() == HloOpcode::While {
        debug_assert!(value.is_phi());
        let blitz_while = value.defining_instruction();
        if self.call_graph.instruction_is_nested_in(
            &use_.instruction, blitz_while.while_body()) ||
           self.call_graph.instruction_is_nested_in(
            &use_.instruction, blitz_while.while_condition())
        {
          println!("  value is while {:?} and use is in condition or body.",
            value.defining_instruction().name());
          return true;
        }
      }
      // The use at a call occurs before values that are defined in the called
      // computation.
      if use_.instruction.opcode() == HloOpcode::Call {
        let call = &use_.instruction;
        if self.call_graph.instruction_is_nested_in(
          value.defining_instruction(), call.to_apply())
        {
          println!("  use is call {:?} and def is in called computation.", call.name());
          return true;
        }
      }
      // The use at an async call occurs before values that are defined in the
      // called computation of the async wrapped instruction.
      if use_.instruction.is_asynchronous() &&
         use_.instruction.async_wrapped_opcode() == HloOpcode::Call
      {
        let async_ = &use_.instruction;
        if self.call_graph.instruction_is_nested_in(
          value.defining_instruction(), async_.async_wrapped_instruction().to_apply())
        {
          println!("  use is async {:?} and def is in called computation", async_.name());
          return true;
        }
      }
      // In general the use of a value in the conditional parameter should be
      // considered to be before a definition in one of its branches, and
      // therefore allowed in live range merging, if there is no
      // surrounding loop that creates a backward control flow path that
      // allows the definition in the branch to have its value flow backward
      // into the conditional and then flow into another branch in the
      // conditional that uses the value.
      if use_.instruction.opcode() == HloOpcode::Conditional {
        let conditional = &use_.instruction;
        for j in 0..conditional.branch_count() {
          if self.call_graph.instruction_is_nested_in(
            value.defining_instruction(), conditional.branch_computation(j))
          {
            if dataflow.value_is_defined_at(
              conditional.operand(use_.operand_number as usize), &vec![0])
            {
              for value_use in value.get_uses() {
                println!("def have use: {:?}.", value_use.to_string());
                if &value_use.instruction == value_use.instruction.parent().root_instruction() {
                  println!("def use is conditional root.");
                  has_escaped_use_in_conditional = true;
                  break;
                }
              }
            }
          }
          if !has_use_in_exclusive_banches {
            println!("  use is conditional {:?} and def is in {:?} th branch computation.",
              conditional.name(), j);
            return true;
          }
        }
        if value.defining_instruction() == conditional {
          println!("  use is conditional {:?} and def is {:?}.",
            use_.to_string(), value.to_short_string());
          return true;
        }
      }

      println!("  use is not before value definition.");
      false
    };

    for use_ in &mut uses {
      if !use_is_before_value_definition(use_) { return false; }
    }

    true
  }

  // Returns whether the given values interfere.
  pub fn may_interfere(
    &self, a: &HloValue, b: &HloValue, dataflow: &HloDataflowAnalysis) -> bool
  {
    // Buffers without disjoint liveness may interfere.
    !self.live_range_strictly_before(
      a, b, dataflow, false)
    &&
    !self.live_range_strictly_before(
      b, a, dataflow, false)
  }

  // Returns true if the live range of the given value 'a' is strictly before
  // the live range of value 'b' using the given HLO ordering.
  pub fn live_range_strictly_before(
    &self,
    a: &HloValue,
    b: &HloValue,
    dataflow: &HloDataflowAnalysis,
    use_is_always_before_def_in_same_instr: bool) -> bool
  {
    println!("live_range_strictly_before(a={:?}. b={:?})",
      a.to_short_string(), b.to_short_string());
    println!("Parent: {:?}", a.instruction().parent().to_string());

    if !self.is_defined_before(a, b) {
      println!("{:?} not defined before {:?}.", a.to_short_string(), b.to_short_string());
      return false;
    }

    if a.live_out_of_module() {
      println!("{:?} is live out of module and not defined before {:?}.",
        a.to_short_string(), b.to_short_string());
      return false;
    }

    // If the root instruction aliases the buffer 'a', the live range of 'a' is
    // until the end of the computation and can never be strictly before another
    // buffer nested in the same computation.
    for pos in a.positions() {
      if pos.instruction.parent().root_instruction() == &pos.instruction &&
        self.call_graph().instruction_is_nested_in(
          b.instruction(), pos.instruction.parent())
      {
        return false;
      }
    }

    // All uses of 'a' must be before 'b' is defined.
    let mut uses = vec![];
    for use_ in a.get_uses() {
      if dataflow.does_not_use_operand_buffer(
        a.instruction(), a.index(), &use_.instruction)
      {
        continue;
      }
      uses.push(use_.clone());
    }
    if !self.uses_before_value_definition(
      uses, b, dataflow, use_is_always_before_def_in_same_instr)
    {
      println!("uses of {:?} not before {:?} is defined.",
        a.to_short_string(), b.to_short_string());
      return false;
    }

    if a.is_root_of(b.instruction().parent()) {
      println!("{:?} is live out of computation and defined before {:?} which is
        in same computation", a.to_short_string(), b.to_short_string());
      return false;
    }

    true
  }

  // Returns the sequential instruction order for the given computation, or
  // none if the computation does not have a sequential ordering.
  pub fn sequential_order(
    &self, _computation: &HloComputation) -> HloInstructionSequence
  {
    unimplemented!()
  }

  // Return the call graph of the module used to compute ordering.
  pub fn call_graph(&self) -> &CallGraph {
    &self.call_graph
  }
  
  pub fn to_string(&self) -> String {
    unimplemented!()
  }

  pub fn executes_before_in_same_computation(
    &self, _a: &HloInstruction, _b: &HloInstruction) -> bool
  {
    unimplemented!()
  }
}

// Base class for partial orderings implemented by a map of predecessors for
// each instruction.
pub struct PredecessorHloOrdering<'module> {
  ordering: HloOrdering<'module>,
  predecessors: HashMap<HloComputation, HloReachabilityMap>,
}

impl<'module> PredecessorHloOrdering<'module> {
  pub fn new(_module: HloModule) -> Self {
   // PredecessorHloOrdering { ordering: HloOrdering:: }
   unimplemented!()
  }

  // Returns None indicating the computation does not have a sequential ordering.
  pub fn sequential_order(
    &self, _computation: &HloComputation) -> Option<&HloInstructionSequence>
  {
    None
  }

  pub fn reachability_map(
    &self, computation: &HloComputation) -> Option<&HloReachabilityMap>
  {
    self.predecessors.get(computation)
  }

  pub fn executes_before_in_same_computation(
    &self, a: &HloInstruction, b: &HloInstruction) -> bool
  {
    // 'a' executes before 'b' if 'a' is in the strict predecessor set of 'b'.
    debug_assert!(a.parent() == b.parent());
    a != b && self.predecessors.get(a.parent()).unwrap().is_reachable(a, b)
  }
}

// An HLO ordering based on data dependencies in the HLO graph.
pub struct DependencyHloOrdering<'module> {
  ordering: PredecessorHloOrdering<'module>
}

impl<'module> DependencyHloOrdering<'module> {
  pub fn new() {}
  pub fn to_string() {}
}

// An HLO ordering based om a total order of instructions in each computation.
pub struct SequentialHloOrdering<'module> {
  ordering: HloOrdering<'module>,
  schedule: HloSchedule,
  order_position: HashMap<HloInstruction, i64>,
}

impl<'module> SequentialHloOrdering<'module> {
  pub fn new() {}

  pub fn sequential_order(
    &self, computation: &HloComputation) -> Option<&HloInstructionSequence>
  {
    if self.schedule.is_computation_scheduled(computation) {
      return self.schedule.sequence(computation);
    }
    None
  }

  pub fn to_string(&self) -> String {
    let mut out = "SequentialHloOrdering\n".to_string();
    out.push_str(&self.schedule.to_string());
    out
  }

  pub fn executes_before_in_same_computation(
    &self, a: &HloInstruction, b: &HloInstruction) -> bool
  {
    debug_assert!(a.parent() == b.parent());
    // If either instruction is not in the order, then 'a' and 'b' are unordered.
    if !self.order_position.contains_key(a) || !self.order_position.contains_key(b) {
      return false;
    }
    // 'a' is the root instruction of the computation, which lives out. So
    // 'a' cannot execute before 'b'.
    if a.parent().root_instruction() == a {
      return false;
    }
    self.order_position.get(a) < self.order_position.get(b)
  }
}