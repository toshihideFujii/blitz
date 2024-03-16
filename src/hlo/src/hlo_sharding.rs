#![allow(dead_code)]

#[derive(Clone, PartialEq)]
pub struct HloSharding {}

impl HloSharding {
  pub fn new() {}
  pub fn replicate() {}
  pub fn manual() {}
  pub fn unknown() {}
  pub fn assign_device() {}
  pub fn tile() {}
  pub fn iota_tile() {}
  pub fn partial_tile() {}
  pub fn subgroup() {}
  pub fn tile_id() {}
  pub fn tuple() {}
  pub fn single_tuple() {}
  pub fn single() {}
  pub fn from_proto() {}
  pub fn is_reserved_device() {}
  pub fn to_proto() {}
  pub fn print() {}
  pub fn to_string() {}
  pub fn validate() {}
  pub fn is_tuple() {}
  pub fn is_replicated() {}
  pub fn is_tile_maximal() {}
  pub fn is_manual() {}
  pub fn is_unknown() {}
  pub fn is_shard_group() {}
  pub fn is_shard_as() {}
  pub fn is_shard_like() {}
  pub fn is_manual_subgroup() {}
  pub fn is_tiled() {}
  pub fn replicate_on_last_tile_dim() {}
  pub fn has_partial_replication() {}
  pub fn uses_device() {}
  pub fn used_devices() {}
  pub fn tile_index_for_device() {}
  pub fn device_for_tile_index() {}
  pub fn tile_offset_for_device() {}
  pub fn tile_limit_for_device() {}
  pub fn unique_device(&self) -> Option<i64> { None }
  pub fn get_unique_device() {}
  pub fn has_unique_device() {}
  pub fn as_shape_tree() {}
  pub fn get_as_shape_tree() {}
  pub fn get_sub_sharding() {}
  pub fn get_tuple_sharding() {}
  pub fn normalize_tuple_sharding() {}
  pub fn extract_single_sharding() {}
  pub fn without_metadata() {}
  pub fn with_metadata() {}
  pub fn tile_assignment() {}
  pub fn subgroup_types() {}
  pub fn tuple_elements() {}
  pub fn tile_shape() {}
  pub fn total_num_tiles() {}
  pub fn num_tiles() {}
  pub fn metadata() {}
  pub fn subgroup_replication_dim() {}
  pub fn subgroup_manual_dim() {}
  pub fn tiled_data_rank() {}
  pub fn required_leaves() {}
}