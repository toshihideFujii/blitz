#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use hlo::{call_graph::{CallGraph, CallGraphNode}, hlo_computation::HloComputation, hlo_instruction::HloInstruction, hlo_module::HloModule, hlo_opcode::HloOpcode};

use crate::hlo_dce::HloDCE;

pub struct CallInliner {
  single_call_site: bool,
  update_domain: bool,
  uniquely_channel_ids: bool,
  composites_to_preserve: HashSet<String>,
}

impl CallInliner {
  pub fn new(single_call_site: bool, update_domain: bool) -> Self {
    CallInliner {
      single_call_site: single_call_site,
      update_domain: update_domain,
      uniquely_channel_ids: false,
      composites_to_preserve: HashSet::new(),
    }
  }

  // Inlines one call instruction.  Returns a mapping from the original
  // instructions to their inlined versions.
  pub fn inline(
    &self,
    call: &mut HloInstruction) -> Result<HashMap<HloInstruction, HloInstruction>, String>
  {
    if call.opcode() != HloOpcode::Call {
      let err_msg = "Instruction was not a call op".to_string();
      return Err(err_msg);
    }
    if call.is_composite() {
      let frontend_attrs = call.mutable_frontend_attributes();
      frontend_attrs.mutable_map().remove(&"composite.name".to_string());
      frontend_attrs.mutable_map().remove(&"composite.attributes".to_string());
      frontend_attrs.mutable_map().remove(&"composite.version".to_string());
      //call.set_frontend_attributes(frontend_attrs.clone());
    }
    let callees = call.called_computations();
    assert!(callees.len() == 1);
    let callee = &callees[0];

    // Propagate the frontend attributes related to fusion from the call to the
    // inlined instructions.
    if call.has_frontend_attributes() {
      let call_attributes = call.frontend_attributes();
      let mut has_fuse = "".to_string();
      if call_attributes.map().contains_key(&"MUST_FUSE".to_string()) {
        has_fuse = "MUST_FUSE".to_string();
      } else if call_attributes.map().contains_key(&"MAXIMAL_FUSE".to_string()) {
        has_fuse = "MAXIMAL_FUSE".to_string();
      }
      if !has_fuse.is_empty() {
        for instr in callee.instructions() {
          if instr.is_fusible() {
            //let fr_attrs = instr.frontend_attributes();
          }
        }
      }
    }
    
    unimplemented!()
  }

  pub fn name(&self) -> String {
    "CallInliner".to_string()
  }

  pub fn run(
    &self,
    module: &HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    let call_graph = CallGraph::build(module, execution_threads);
    let visitor_func =
      |_node: &mut CallGraphNode| -> Result<bool, String>
    {
      /*
      if !HloInstruction::is_thread_included(
        node.computation().execution_thread(), execution_threads)
      {
        return Ok(false);
      }
      if module.has_schedule() {
        let sequence =
          module.mutable_schedule().get_or_create_mutable_sequence(module, node.computation());
        return self.inline_and_legalize(&call_graph, node.mutable_computation(),
        sequence.instructions());
      }
      */
      unimplemented!()
    };
    let did_mutate = call_graph.visit_nodes_with_return(
      &visitor_func, true);

    if *did_mutate.as_ref().unwrap() {
      // Run DCE to remove called computations which are now becoming unused.
      // This can result then in problems if within the called computation, there
      // were send/recv instructions, which the module group verifier will flag as
      // error finding the same channel ID used for multiple send/recv
      // instructions.
      let mut dce = HloDCE::new(false);
      let result = dce.run(module, execution_threads);
      if result.is_err() { return Err(result.err().unwrap()); }
      if module.has_schedule() {
        // TODO
      }
    }
    did_mutate
  }

  pub fn is_inlineable_call_op(&self, instruction: &HloInstruction) -> bool {
    let prerequisite = instruction.opcode() == HloOpcode::Call &&
      !instruction.has_backend_config() &&
      !instruction.parent().as_ref().unwrap().is_async_computation();
    if !prerequisite {
      return false;
    }
    if !inline_instruction(instruction) {
      // Always prioritize user's explicit requests after fulfilling the
      // prerequisites.
      return false;
    }
    if instruction.get_module().as_ref().unwrap().config().use_shardy_partitioner() &&
      (instruction.to_apply().name().contains("shmap_body") ||
      instruction.to_apply().name().contains("blitz.sdy.manual_computation_body"))
    {
      return false;
    }
    inline_composites(instruction, &self.composites_to_preserve)
  }

  fn inline_and_legalize(
    &self,
    call_graph: &CallGraph,
    computation: &mut HloComputation,
    instruction_sequence: &mut Vec<HloInstruction>) -> Result<bool, String>
  {
    let mut did_node_mutate = false;
    let module = computation.parent().unwrap();
    let mut inlined_instructions: Vec<HloInstruction> = vec![];

    for instr in instruction_sequence {
      if self.should_inline(call_graph, instr) {
        //let callee = instr.to_apply();
        let inline_map_wrapper =
          self.inline(instr);
        if inline_map_wrapper.is_err() {
          return Err(inline_map_wrapper.err().unwrap());
        }
        let inline_map =
          inline_map_wrapper.unwrap();

        let callee = instr.to_apply();
        if module.has_schedule() {
          let inlined_instrs = module.schedule()
            .sequence(callee).as_ref().unwrap().instructions();
          for inlined_instr in inlined_instrs {
            if inlined_instr.opcode() != HloOpcode::Parameter {
              inlined_instructions.push(inline_map.get(inlined_instr).unwrap().clone());
            }
          }
        }
        if self.update_domain {
          // TODO
        }
        did_node_mutate = true;
      } else if module.has_schedule() {
        inlined_instructions.push(instr.clone());
      }
    }
    if did_node_mutate && module.has_schedule() {
      // TODO
    }
    if did_node_mutate && self.uniquely_channel_ids {
      let mut unique_channel_id = 1;
      for instr in computation.mutable_instructions() {
        if instr.is_channel_instruction() {
          instr.set_channel_id(unique_channel_id);
          unique_channel_id += 1;
        }
      }
    }
    Ok(did_node_mutate)
  }

  fn should_inline(
    &self, call_graph: &CallGraph, instruction: &HloInstruction) -> bool
  {
    if !self.is_inlineable_call_op(instruction) {
      return false;
    }
    if !CallInliner::should_inline_func(call_graph, instruction) {
      return false;
    }
    if self.single_call_site {
      return call_graph.get_node(instruction.to_apply())
        .caller_callsites().len() == 1;
    }
    true
  }

  fn should_inline_func(
    _call_graph: &CallGraph, _instruction: &HloInstruction) -> bool
  {
    unimplemented!()    
  }
}

// Introduces a specific attribute so that the frontend has the direct
// control over inlining specific calls.
fn inline_instruction(instruction: &HloInstruction) -> bool {
  let target = instruction.frontend_attributes()
    .map().get(&"inlineable".to_string());
  if target.is_some() {
    return target.unwrap() == &"true".to_string();
  }
  true
}

fn inline_composites(
  instruction: &HloInstruction,
  composites_to_preserve: &HashSet<String>) -> bool
{
  !instruction.is_composite() ||
  !composites_to_preserve.contains(instruction
    .frontend_attributes().map().get(&"composite.name".to_string()).unwrap())
}