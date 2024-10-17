#![allow(dead_code)]

use std::collections::HashSet;

use hlo::{
  hlo_computation::HloComputation,
  hlo_instruction::HloInstruction,
  hlo_module::HloModule,
  hlo_opcode::HloOpcode,
  utils::hlo_query::is_collective_communication_op
};

use crate::call_graph::{
  //CallContext,
  CallGraph, CallGraphNode
};

// Flattening associates each call site with a unique computation (for
// sequential calling contexts) This simplifies buffer assignment and
// points-to analysis (see b/36865746 for details).
pub struct FlattenCallGraph {}

impl FlattenCallGraph {
  pub fn new() -> Self {
    FlattenCallGraph {  }
  }

  pub fn name() -> String {
    "flatten-call-graph".to_string()
  }

  // Duplicates computations called from multiple call- or while-nodes to
  // flatten the call graph.
  pub fn run(
    &self,
    module: &mut HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    println!("Before flatten call graph: {:?}", module.to_string());

    // Flatten original call graph.
    let call_graph = CallGraph::build(module, execution_threads);
    let result =
      call_graph.visit_nodes(flatten_node, true);
    if result.is_err() {
      return Err(result.err().unwrap());
    }

    // Annotate flattened computations with callee types.
    let call_graph = CallGraph::build(module, execution_threads);
    let result =
      call_graph.visit_nodes(annotate_node, true);
      if result.is_err() {
        return Err(result.err().unwrap());
      }

    println!("After flatten call graph: {:?}", module.to_string());
    Ok(true)
  }
}

// Helper to replace the called computation at a while, call, conditional or
// async instruction. This function replaces exactly one instance of
// 'computation' with 'new_computation' even if 'instruction' calls
// 'computation' more than once.
pub fn replace_called_computation(
  instruction: &mut HloInstruction,
  computation: &HloComputation,
  new_computation: HloComputation)
{
  let opcode = instruction.opcode();
  match opcode {
    HloOpcode::While => {
      if instruction.while_condition() == computation {
        instruction.set_while_condition(new_computation);
      } else {
        assert_eq!(instruction.while_body(), computation);
        instruction.set_while_body(new_computation);
      }
    }
    HloOpcode::Call => {
      assert_eq!(instruction.to_apply(), computation);
      instruction.set_to_apply(new_computation);
    }
    HloOpcode::Conditional => {
      for b in 0..instruction.branch_count() {
        if b == instruction.branch_count() - 1 {
          assert_eq!(instruction.branch_computation(b), computation);
        }
        if computation == instruction.branch_computation(b) {
          instruction.set_branch_computation(b, new_computation);
          break;
        }
      }
    }
    _ => unreachable!("unexpected opcode: {:?}", opcode),
  }
}

// Flatten a single call graph node. Expects to visit nodes in postorder.
pub fn flatten_node(_node: &CallGraphNode) -> Result<(), String> {
  /*
  let computation = node.mutable_computation();
  let module = computation.mutable_parent();

  for i in 0..node.caller_callsites().len() {
    let call_site = &node.caller_callsites()[i];
    // Only consider sequential call contexts.
    if call_site.context() == CallContext::Embedded {
      continue;
    }
    assert_eq!(call_site.context(), CallContext::ControlFlow);
    // Skip first element if this computation is only called from a sequential
    // context.
    if node.context() != CallContext::Both && i == 0 {
      continue;
    }
    if computation.is_async_computation() {
      continue;
    }
    let clone =
      module.as_mut().unwrap().add_embedded_computation(computation.clone());
  }
  */
  Ok(())
}

// Annotates flatten computations with callee instruction types.
pub fn annotate_node(node: &CallGraphNode) -> Result<(), String> {
  for callsite in node.callsites() {
    let instruction = callsite.instruction();
    if instruction.opcode() == HloOpcode::Fusion {

    } else if instruction.opcode() == HloOpcode::CustomCall {
        
    } else if is_collective_communication_op(&instruction.opcode()) {
        
    } else if instruction.opcode() == HloOpcode::While {
      //instruction.mutable_while_body().set_while_call_instruction(instruction);
    } else if instruction.opcode() == HloOpcode::Conditional {
        
    }
  }
  Ok(())
}