#![allow(dead_code)]

use std::collections::HashSet;

use crate::hlo_instruction::HloInstruction;

pub struct Domain {
  pub reach_set: HashSet<HloInstruction>,
  pub instructions: Vec<HloInstruction>,
  pub enter_domains: HashSet<HloInstruction>,
  pub exit_domains: HashSet<HloInstruction>
}

impl Domain {
  pub fn new() -> Self {
    Domain {
      reach_set: HashSet::new(),
      instructions: Vec::new(),
      enter_domains: HashSet::new(),
      exit_domains: HashSet::new()
    }
  }
}

// The DomainMetadata represents the base class for metadata which can be
// attached to Domain HLO instructions. 
#[derive(Debug, Clone)]
pub struct DomainMetadata {}

impl DomainMetadata {
  pub fn new() {}

  // Returns the matadata type.
  pub fn kind(&self) -> String {
    unimplemented!()
  }

  // Compares the metadata object with another one and return true if the
  // two matches.
  pub fn matches(&self, _other: &DomainMetadata) -> bool {
    unimplemented!()
  }

  pub fn hash() {}

  // Returns a string representation of the metadata.
  pub fn to_string(&self) -> String {
    unimplemented!()
  }
}