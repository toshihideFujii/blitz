#![allow(dead_code)]

use std::collections::HashSet;

use hlo::{hlo_instruction::HloInstruction, hlo_module::HloModule, hlo_opcode::HloOpcode};

// Domain instruction is the task of placing domain instructions between hlo
// instructions having different sharding .
pub struct HloDomainIsolator {}

impl HloDomainIsolator {
  pub fn new() -> Self {
    HloDomainIsolator {  }
  }

  pub fn name(&self) -> String { "domain-isolator".to_string() }

  // Update domains for an instruction.
  pub fn update_domains(&self, _instruction: &HloInstruction) -> Result<bool, String> {
    unimplemented!()
  }

  pub fn run(
    &self, _module: &HloModule, _execution_threads: HashSet<String>) -> Result<bool, String>
  {
    Ok(true)
  }

  fn run_internal<F>(
    &self,
    _module: &HloModule,
    _execution_threads: HashSet<String>,
    _func: F
  ) -> Result<bool, String>
    where F: Fn(&HloInstruction, &HloInstruction, &HloInstruction) -> HloInstruction
  {
    Ok(true)
  }
}

fn add_exit_domains<F>(instruction: &HloInstruction, creator: F) -> Result<i64, String>
  where F: Fn(&HloInstruction, &HloInstruction, &HloInstruction) -> Option<HloInstruction>
{
  let mut added_domain = 0;
  if instruction.opcode() == HloOpcode::Domain {
    return Ok(added_domain);
  }

  for user in instruction.users() {
    let domain = creator(user, instruction, instruction);
    if domain.is_some() {
      println!("New domain: {:?}", domain.unwrap().to_string_default());
      //let result =
        //instruction.replace_use_with_different_shape(user, domain.unwrap());
      added_domain += 1;
    }
  }
  Ok(added_domain)
}