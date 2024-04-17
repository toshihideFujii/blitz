#![allow(dead_code)]

//use hlo::hlo_instruction::HloInstruction;

#[derive(Debug, Clone, PartialEq)]
enum PipeliningDirection {
  Backward,
  Forword,
  ForwordSink,
}

struct Config {
  level_to_operate_on: i64,
  max_pipelining_per_loop: i64,
  last_run: bool,
  pipeline_use_tree: bool,
  process_different_sized_ops: bool,
  pipelining_direction: PipeliningDirection,
  //should_process: Fn(&HloInstruction) -> bool,
}

pub struct CollectivePipeliner {
  config: Config
}

impl CollectivePipeliner {
  pub fn new() {}

  pub fn name(&self) -> String {
    if self.config.pipelining_direction == PipeliningDirection::Forword {
      return "collective-pipelinner-forward".to_string();
    } else if self.config.pipelining_direction == PipeliningDirection::Backward {
      return "collective-pipeliner-backward".to_string();
    } else {
      return "collective-pipeliner-forwardsink".to_string();
    }
  }

  pub fn run() {}
}