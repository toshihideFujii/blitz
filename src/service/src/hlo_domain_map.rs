#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use hlo::{
  hlo_computation::HloComputation,
  hlo_domain_metadata::Domain, 
  hlo_instruction::HloInstruction,
  hlo_opcode::HloOpcode
};

// The HloDomainMap isplits a sest of instructions within a module or computation,
// into different domains, separated by domain instructions.
pub struct HloDomainMap {
  domain_kind: String,
  instruction_domains: Vec<Domain>,
  instruction_to_domain: HashMap<HloInstruction, i64>,
  domain_metadata_id: HashMap<HloInstruction, i64>
}

impl HloDomainMap {
  pub fn new(computation: &HloComputation, domain_kind: String) -> Self {
    let instance = HloDomainMap {
      domain_kind: domain_kind,
      instruction_domains: Vec::new(),
      instruction_to_domain: HashMap::new(),
      domain_metadata_id: HashMap::new()
    };
    let result = instance.populate(computation);
    if result.is_err() {
      assert!(false);
    }
    instance
  }

  // Retrieves all the domains the input module or computation are composed by.
  pub fn get_domains(&self) -> &Vec<Domain> {
    &self.instruction_domains
  }

  // Checks whether two instructions are within the same domain.
  pub fn is_same_domain(
    &self,
    instruction_1: &HloInstruction,
    instruction_2: &HloInstruction) -> bool
  {
    let domain_id_1 = self.get_domain_id(instruction_1);
    let domain_id_2 = self.get_domain_id(instruction_2);
    domain_id_1 >= 0 && domain_id_1 == domain_id_2
  }

  // Checks whether instruction is a domain instruction of the kind we are
  // currenttly processing.
  pub fn is_domain_instruction(&self, instruction: &HloInstruction) -> bool {
    if instruction.opcode() != HloOpcode::Domain {
      return false;
    }
    if !self.domain_kind.is_empty() {
      if instruction.user_side_metadata().kind() != self.domain_kind {
        return false;
      }
      // Both user and operand side of the metadata must be of the same kind.
      debug_assert!(instruction.operand_side_metadata().kind() == self.domain_kind,
        "instruction has mismatching metadakinds.");
    }
    true
  }

  // Retrieves the domain identifier of the instruction, or -1 in case instruction
  // is not found within any domain.
  pub fn get_domain_id(&self, instruction: &HloInstruction) -> i64 {
    *self.instruction_to_domain.get(instruction).unwrap_or(&-1)
  }

  // Returns the unique id of the domain metadata for the domain the given instruction
  // belongs to.
  pub fn get_domain_metadata_id(&self, instruction: &HloInstruction) -> i64 {
    *self.domain_metadata_id.get(instruction).unwrap()
  }

  // Check if the domain instruction is facing (via its oprand link) another
  // domain instruction of the same kind, hence defining an empty domain.
  fn try_process_empty_dpmain(&mut self, instruction: &HloInstruction) -> Result<(), String> {
    debug_assert!(instruction.opcode() == HloOpcode::Domain);
    for operand in &instruction.unique_operands() {
      if self.is_domain_instruction(operand) {
        let mut domain = Domain::new();
        domain.enter_domains.insert(operand.clone());
        domain.exit_domains.insert(instruction.clone());
        let result = self.insert_domain(domain);
        if result.is_err() { return result; }
      }
    }
    if instruction == instruction.parent().root_instruction() {
      let mut domain = Domain::new();
      domain.enter_domains.insert(instruction.clone());
      let result = self.insert_domain(domain);
      if result.is_err() { return result; }
    }
    Ok(())
  }

  fn populate(&self, _computation: &HloComputation) -> Result<(), String> {
    unimplemented!()
  }

  // Inserts the provided domain into the ones tracked by this object, creating a
  // new domain id.
  fn insert_domain(&mut self, domain: Domain) -> Result<(), String> {
    let _domain_id = self.instruction_domains.len();
    self.instruction_domains.push(domain);

    //let reach_set =
      //self.instruction_domains.last().as_ref().unwrap().reach_set;
    //for instruction in reach_set {
      //self.instruction_to_domain.insert(instruction.clone(), domain_id as i64);
    //}

    Ok(())
  }

  // From the given instruction, expands operand and user wise, the set of
  // instructions which can be reached without crossing a domain instruction
  // of the kind apecified by domain_kind.
  fn expand_domain(
    &self,
    _instruction: &HloInstruction,
    _domain: &Domain) -> Result<(), String>
  {
    Ok(())
  }

  // Creates a domain data structure using the expand_domain() api.
  fn create_domain(
    &self,
    instruction: &HloInstruction,
    instructions_order: &HashMap<HloInstruction, i64>) -> Result<Domain, String>
  {
    let mut domain = Domain::new();
    let result = self.expand_domain(instruction, &domain);
    if result.is_err() {
      return Err(result.err().unwrap());
    }
    domain.instructions = self.make_non_domain_instructions(
      &domain.reach_set, instructions_order);
    Ok(domain)
  }

  // Out of an instruction set, returns a vector of all the ones which are not
  // a domain kind.
  fn make_non_domain_instructions(
    &self,
    instruction_set: &HashSet<HloInstruction>,
    _instructions_order: &HashMap<HloInstruction, i64>) -> Vec<HloInstruction>
  {
    let mut instructions = vec![];
    for instruction in instruction_set {
      if instruction.opcode() != HloOpcode::Domain {
        instructions.push(instruction.clone());
      }
    }
    // TODO: instructions.sort()
    instructions
  }

  fn populate_domain_metadata_map() {}
}