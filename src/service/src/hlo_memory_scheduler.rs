#![allow(dead_code)]

use std::collections::HashSet;

use hlo::{
  dfs_hlo_visitor_with_default::FunctionVisitor,
  hlo_instruction::HloInstruction,
  hlo_module::HloModule,
  hlo_schdule::HloSchedule
};

// A pass which schedules the HLO instructions in a module.
pub struct HloMemoryScheduler {}

impl HloMemoryScheduler {
  pub fn new() {}

  pub fn name(&self) -> String {
    "hlo-memory-scheduler".to_string()
  }

  pub fn run(
    &self,
    _module: &HloModule,
    _execution_threads: HashSet<String>) -> Result<bool, String>
  {
    Ok(true)
  }
}

// A pass which produces a naive, but correct schedule.
pub struct HloTrivialScheduler {}

impl HloTrivialScheduler {
  pub fn new() -> Self {
    HloTrivialScheduler {  }
  }

  pub fn name(&self) -> String {
    "hlo-trivial-scheduler".to_string()
  }

  pub fn run(
    &self,
    module: &mut HloModule,
    execution_threads: HashSet<String>) -> Result<bool, String>
  {
    let mut schedule = HloSchedule::new();
    for computation in
      module.make_computation_post_order(&execution_threads, false)
    {
      if !computation.is_fusion_computation() {
        // TODO
        let _computation_sequence =
          schedule.get_or_create_sequence(module, computation);
        let visitor_func 
          = |_instruction: &HloInstruction| -> Result<(), String> {
            //computation_sequence.push_pack(instruction.clone());
            Ok(())
        };
        let mut visitor =
          FunctionVisitor::new(Box::new(visitor_func));
        visitor.reserve_visit_states(computation.instruction_count());
        let result = computation.accept(&visitor);
        if result.is_err() {
          return Err(result.err().unwrap());
        }
      }    
    }
    module.set_schedule(schedule);
    Ok(true)
  }
}

// A trivial pass which clears the schedule currently set on the HloModule.
pub struct HloDescheduler {}

impl HloDescheduler {
  pub fn new() -> Self {
    HloDescheduler {  }
  }

  pub fn name(&self) -> String {
    "hlo-descheduler".to_string()
  }

  pub fn run(
    &self,
    module: &mut HloModule,
    _execution_threads: HashSet<String>) -> Result<bool, String>
  {
    let changed = module.has_schedule();
    module.clear_schedule();
    Ok(changed)
  }
}