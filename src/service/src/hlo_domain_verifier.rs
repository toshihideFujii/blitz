#![allow(dead_code)]

use std::collections::HashSet;

use hlo::{
  hlo_domain_metadata::{Domain, DomainMetadata},
  hlo_module::HloModule, hlo_opcode::HloOpcode
};


struct RunContext {
  //module: HloModule,
  //verifier: HloDomainVerifier
}

impl RunContext {
  pub fn new(/*module: HloModule, verifier: HloDomainVerifier*/) -> Self {
    //RunContext { module: module, verifier: verifier }
    RunContext {  }
  }

  pub fn run(
    &self,
    module: &HloModule,
    verifier: &mut HloDomainVerifier,
    execution_threads: &HashSet<String>) -> Result<(), String>
  {
    println!("Running hlo domain verifier");
    let result =
      self.populate_domain_kinds(module, verifier, execution_threads);
    if result.is_err() { return result; }

    for _computation in module.computations_by_exec_threads(execution_threads) {
      for _kind in &verifier.kinds {
        // TODO
      }
    }
    Ok(())
  }

  fn populate_domain_kinds(
    &self,
    module: &HloModule,
    verifier: &mut HloDomainVerifier,
    execution_threads: &HashSet<String>) -> Result<(), String>
  {
    if verifier.kinds.is_empty() {
      let mut kinds = Vec::new();
      for computation in module.computations_by_exec_threads(execution_threads) {
        for instruction in computation.instructions() {
          if instruction.opcode() == HloOpcode::Domain {
            debug_assert!(instruction.user_side_metadata().kind() ==
              instruction.operand_side_metadata().kind());
            kinds.push(instruction.user_side_metadata().kind().clone());
          }
        }
      }
      verifier.kinds.append(&mut kinds);
    }
    Ok(())
  }
}

// Verifies that the domain instructions are consistent, and the each domain is
// surrounded by the same metadata.
pub struct HloDomainVerifier {
  kinds: Vec<String>
}

impl HloDomainVerifier {
  pub fn new(kinds: Vec<String>) -> Self {
    HloDomainVerifier { kinds: kinds }
  }

  pub fn name(&self) -> String {
    "domain-verifier".to_string()
  }

  pub fn run(
    &mut self,
    module: &HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    let run_context = RunContext::new();
    let result =
      run_context.run(module, self, execution_threads);
    if result.is_err() {
      return Err(result.err().unwrap());
    }
    Ok(false)
  }

  // Verify that whole domain frontier bounding the instruction reach set,
  // has matching metadata.
  pub fn verify_domain(domain: &Domain) -> Option<DomainMetadata> {
    let mut metadata =  None;
    println!("Reach set:");
    for instruction in &domain.instructions {
      println!("  {:?}", instruction.name());
    }

    println!("  Domains:");
    for instruction in &domain.enter_domains {
      let meta = instruction.user_side_metadata();
      println!("    User side: {:?}", instruction.name());
      println!("      {:?}", meta.to_string());
      if metadata.is_none() {
        metadata = Some(meta.clone());
      } else {
        let result = meta.matches(metadata.as_ref().unwrap());
        debug_assert!(result);
      }
    }

    for instruction in &domain.exit_domains {
      let meta = instruction.operand_side_metadata();
      println!("    Operand side: {:?}", instruction.name());
      println!("      {:?}", meta.to_string());
      if metadata.is_none() {
        metadata = Some(meta.clone());
      } else {
        let result = meta.matches(metadata.as_ref().unwrap());
        debug_assert!(result);
      }
    }

    metadata
  }
}