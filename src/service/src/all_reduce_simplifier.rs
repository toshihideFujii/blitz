#![allow(dead_code)]

use std::collections::HashSet;

use common::shape_util::ShapeUtil;
use hlo::{hlo_instruction::HloInstruction, hlo_module::HloModule, hlo_opcode::HloOpcode};

// A pass that detects all-reduces whose inputs are already the same across
// replicas using the replication analysis, then replicas those all-reduces
// with local computations.
pub struct AllReduceSimplifier {
  replica_count: i64
}

impl AllReduceSimplifier {
  pub fn new() -> Self {
    AllReduceSimplifier { replica_count: 0 }
  }

  pub fn name() -> String {
    "all-reduce-simp".to_string()
  }

  // Run all-reduce simplification on the given computation. Returns whether the
  // computation was changed.
  pub fn run(
    &mut self,
    module: &mut HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    let get_participant_counts_for_replica_group =
      |_all_reduce: &HloInstruction| -> Result<i64, String>
    {
      Ok(0)
    };

    let mut changed = false;
    for comp in
      module.mutable_computations_by_exec_threads(execution_threads)
    {
      for inst in comp.mutable_make_instruction_post_order() {
        if inst.opcode() == HloOpcode::AllGather ||
          inst.opcode() == HloOpcode::ReduceScatter &&
          ShapeUtil::compatible(inst.shape(), inst.operand(0).shape())
        {
          changed = true;
          //let result = comp.replace_instruction(
            //inst, inst.mutable_operand(0).unwrap(),
            //false, false, true);
        }
      }
    }

    for comp in
      module.mutable_computations_by_exec_threads(execution_threads)
    {
      for inst in comp.mutable_make_instruction_post_order() {
        if !inst.shape().is_array() {
          // We currently do not change tuple-shaped all-reduce.
          // Until XLA will support Token fed AllReduce(), the PyTorch client code
          // uses a fake data token (constant) which relies on this pass to not
          // optimize out (being fed within a tuple input).
          continue;
        }
        if !inst.is_cross_replica_all_reduce() && !inst.is_cross_module_all_reduce(){
          continue;
        }
        let group_size =
          get_participant_counts_for_replica_group(inst);
        check_error(&group_size);
        
        // We will not simplify this all reduce if any of the following is true:
        // 1. All group do not have the same size.
        //
        // 2. The AllReduce is not cross replica and the group size is not 1.
        // Since the replication analysis performed earlier is only for cross
        // replica spmd.
        //
        // 3. The AllReduce is not cross replica and the module is not using spmd.
        if *group_size.as_ref().unwrap() == -1 ||
          (!inst.is_cross_replica_all_reduce() && *group_size.as_ref().unwrap() == -1) ||
          (!inst.is_cross_replica_all_reduce() //&&
           // !module.config().use_spmd_partitioning())
          )
        {
          continue;
        }
      }
    }
    Ok(changed)
  }
}

fn check_error<T>(value: &Result<T, String>) {
  if value.is_err() {
    let err_msg = value.as_ref().err().unwrap();
    assert!(false, "{:?}", err_msg);
  }
}