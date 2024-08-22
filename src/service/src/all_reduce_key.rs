#![allow(dead_code)]

use common::blitz_data::PrimitiveType;
use hlo::{hlo_instruction::HloInstruction, hlo_opcode::HloOpcode};

use crate::hlo_domain_map::HloDomainMap;

// Encapsulates all of the properties which must match for two all-reduce
// instructions to be compatible with each other (and hence be possible to
// combine the instructions).
pub type AllReduceKey = (HloOpcode, PrimitiveType, i64, bool, bool, Vec<i64>);

pub fn get_all_reduce_key(
  _instruction: &HloInstruction,
  _domain_map: &HloDomainMap,
  _ignore_replica_groups: bool) -> Option<AllReduceKey>
{
  unimplemented!()
}