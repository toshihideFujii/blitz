#![allow(dead_code)]

use std::collections::HashSet;

use hlo::{
  hlo_domain_metadata::Domain,
  hlo_instruction::HloInstruction,
  hlo_module::HloModule,
  hlo_opcode::HloOpcode
};

use crate::{hlo_domain_map::HloDomainMap, hlo_domain_verifier::HloDomainVerifier};

struct RunContext {}

impl RunContext {
  pub fn new() -> Self {
    RunContext {  }
  }

  pub fn run(
    &mut self,
    module: &mut HloModule,
    execution_threads: &HashSet<String>,
    remover: &HloDomainRemover) -> Result<bool, String>
  {
    println!("Processing metadata domain: {:?}", remover.kind);

    let mut removed_domains = 0;
    for computation in
       module.mutable_computations_by_exec_threads(execution_threads) {
      // First create the domain instruction sets.
      // A domain instruction set is the set of instructions whose edges never cross
      // a domain instruction.
      let domain_map =
        HloDomainMap::new(computation, remover.kind.clone());
      
      // Verify and normalize every domain populated within the map.
      for domain in domain_map.get_domains() {
        let result = self.verify_and_normalize_domain(remover, domain);
        if result.is_err() { return Err(result.err().unwrap()); }
      }

      // Now remove all the domain instructions of the kind specified by the remover,
      // that are within the currently processed computation from the graph.
      for instruction in computation.make_instruction_post_order() {
        for operand in &instruction.unique_operands() {
          if domain_map.is_domain_instruction(operand) {
            println!("Removing {:?}", operand.name());
            //operand.replace_all_uses_with(
              //operand.mutable_operand(0).unwrap(),
              //"".to_string());
            //computation.remove_instruction(operand);
            removed_domains += 1;
          }
        }
      }

      let root = computation.root_instruction();
      if domain_map.is_domain_instruction(root) {
        println!("Removing {:?}", root.name());
        //computation.set_root_instruction();
        //computation.remove_instruction(root);
        removed_domains += 1;
      }
    }

    println!("Removed {:?} domain instructions of {:?} kind", removed_domains, remover.kind);
    Ok(removed_domains > 0)
  }

  // Verifies the consistency of the domain, and normalizesthe instructions
  // within it.
  fn verify_and_normalize_domain(
    &self,
    _remover: &HloDomainRemover,
    domain: &Domain) -> Result<(), String>
  {
    let metadata = HloDomainVerifier::verify_domain(domain);
    if metadata.is_some() {
      println!("Applying domain normalization: {:?}", metadata.as_ref().unwrap().to_string());
      // TODO: normalizer()
    } else {
      println!("Applying domain-less normalization");
      // TODO: normalizer()
    }
    Ok(())
  }
}

// Removes all the domain instructions of a given kind from the input module,
// and calls the normalizer to propagate the properties on the possibly new born
// instructions.
pub struct HloDomainRemover {
  kind: String
}

impl HloDomainRemover {
  pub fn new(kind: String) -> Self {
    HloDomainRemover { kind: kind }
  }

  pub fn name(&self) -> String {
    "domain-remover".to_string()
  }

  // Remove domains of a given kind which are used at users of a specific instruction.
  pub fn remove_exit_domains(
    &self,
    instruction: &mut HloInstruction,
    domain_kind: String) -> Result<i64, String>
  {
    let mut removed_domains = 0;
    let _computation = instruction.mutable_parent();
    for user in instruction.mutable_users() {
      if user.opcode() == HloOpcode::Domain &&
         user.user_side_metadata().kind() == domain_kind &&
         user.operand_side_metadata().kind() == domain_kind
      {
        println!("Removing exit domain {:?}", user.name());
        //let result =
          //user.replace_all_uses_with(instruction, "".to_string());
        //computation.remove_instruction(user);
        removed_domains += 1;
      } 
    }
    Ok(removed_domains)
  }

  pub fn run(
    &mut self,
    module: &mut HloModule,
    execution_threads: HashSet<String>) -> Result<bool, String>
  {
    let mut run_context = RunContext::new();
    run_context.run(module, &execution_threads, &self)
  }
}