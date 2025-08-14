
use common::blitz_data::PrimitiveType;
use hlo::{hlo_instruction::HloInstruction, hlo_opcode::HloOpcode};
use crate::hlo_domain_map::HloDomainMap;

// Encapsulates all of the properties which must match for two all-reduce
// instructions to be compatible with each other (and hence be possible to
// combine the instructions).
pub type AllReduceKey = (HloOpcode, PrimitiveType, i64, bool, bool, Vec<i64>);

// Returns a key that will be equal for all-reduce instructions that are
// compatible with each other, and hence might be combined, or different if not.
pub fn get_all_reduce_key(
  instruction: &HloInstruction,
  domain_map: Option<&HloDomainMap>,
  ignore_replica_groups: bool) -> Option<AllReduceKey>
{
  if instruction.has_control_dependencies() {
    return None;
  }
  if instruction.opcode() != HloOpcode::AllReduce &&
    instruction.opcode() != HloOpcode::ReduceScatter
  {
    return None;
  }
  if instruction.to_apply().instruction_count() != 3 ||
    instruction.to_apply().num_parameters() != 2
  {
    println!("Skipping due to non-trivial reduction function: {:?}",
      instruction.to_apply().to_string());
    return None;
  }
  let mut replica_groups = vec![];
  if !ignore_replica_groups {
    for rg in instruction.replica_groups() {
      for id in rg.replica_ids() {
        replica_groups.push(*id);
      }
    }
  }
  let to_apply_root = instruction.to_apply().root_instruction();
  // Domain metadata id returned by `GetDomainMetadataId` is guaranteed to be >=
  // 0, so use -1 when we don't need to track domain metadata id.
  let mut domain_metadata_id = -1;
  if domain_map.is_some() {
    domain_metadata_id = domain_map.unwrap().get_domain_metadata_id(instruction);  
  }
  let key: AllReduceKey = (
    to_apply_root.opcode(),
    to_apply_root.shape().element_type(),
    domain_metadata_id,
    instruction.channel_id().is_some(),
    instruction.use_global_device_ids(),
    replica_groups);
  
  Some(key)
}