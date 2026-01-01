#![allow(dead_code)]

use common::{
  blitz_data::{OpMetadata, OpSharding, OpShardingType}, printer::{Printer, StringPrinter},
  shape::Shape, shape_tree::ShapeTree, shape_util::ShapeUtil
};

use crate::{hlo_op_metadata::{op_metadata_to_string}, tile_assignment::TileAssignment};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShardGroup {
  shard_group_id: i64,
  shard_as: bool,
  shard_like: bool,
}

impl ShardGroup {
  pub fn new(shard_group_id: i64, shard_as: bool, shard_like: bool) -> Self {
    ShardGroup {
      shard_group_id: shard_group_id,
      shard_as: shard_as,
      shard_like: shard_like
    }
  }

  pub fn to_string(&self) -> String {
    let mut result = String::new();
    if self.shard_as {
      result.push_str("shard_as ");
      result.push_str(&self.shard_group_id.to_string());
    } else if self.shard_like {
      result.push_str("shard_like ");
      result.push_str(&self.shard_group_id.to_string());
    }
    result
  }
}

// HLO shardings describe how an HLO instruction is split across multiple
// computations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HloSharding {
  tile_assignment: TileAssignment,
  tuple_elements: Vec<HloSharding>,
  metadata: Vec<OpMetadata>,
  subgroup_types: Vec<OpShardingType>,
  replicated: bool,
  maximal: bool,
  tuple: bool,
  manual: bool,
  unknown: bool,
  unreduced: bool,
  replicate_on_last_tile_dim: bool,
  shard_group: ShardGroup,
}

impl Default for HloSharding {
  fn default() -> Self {
    HloSharding {
      tile_assignment: TileAssignment::default(),
      tuple_elements: Vec::new(),
      metadata: Vec::new(),
      subgroup_types: Vec::new(),
      replicated: false,
      maximal: false,
      tuple: false,
      manual: false,
      unknown: false,
      unreduced: false,
      replicate_on_last_tile_dim: false,
      shard_group: ShardGroup::new(0, false, false)
    }
  }
}

impl HloSharding {
  pub fn new(
    manual: bool,
    replicated: bool,
    unknown: bool,
    unreduced: bool,
    metadata: Vec<OpMetadata>) -> Self
  {
    HloSharding {
      tile_assignment: TileAssignment::default(),
      tuple_elements: Vec::new(),
      metadata: metadata,
      subgroup_types: Vec::new(),
      replicated: replicated,
      maximal: replicated,
      tuple: false,
      manual: manual,
      unknown: unknown,
      unreduced: unreduced,
      replicate_on_last_tile_dim: false,
      shard_group: ShardGroup::new(-1, false, false)
    }
  }

  // device_id values:
  // -2: magic number to mean unassigned device, used by spatial partitioning
  // -1: the id of the host
  //  0 or positive: the id of a device
  // NOTE(dimvar): -1 is needed for outside compilation. It can be removed once
  // we have fully switched to the side-effect tokens.
  pub fn new_from_device_id(
    device_id: i64,
    metadata: Vec<OpMetadata>) -> Self
  {
    HloSharding {
      tile_assignment: TileAssignment::new_from_device_id(device_id),
      tuple_elements: Vec::new(),
      metadata: metadata,
      subgroup_types: Vec::new(),
      replicated: false,
      maximal: true,
      tuple: false,
      manual: false,
      unknown: false,
      unreduced: false,
      replicate_on_last_tile_dim: false,
      shard_group: ShardGroup::new(-1, false, false)
    }
  }

  // Creates a trivial sharding that replicates a maximal tile scross all
  // devices.
  pub fn replicate(metadata: Vec<OpMetadata>) -> Self {
    HloSharding::new(false, true, false,
      false, metadata)
  }

  // Creates a sharding that represents the op is manually partitioned.
  pub fn manual(metadata: Vec<OpMetadata>) -> Self {
    HloSharding::new(true, false, false,
      false, metadata)
  }

  // Creates a sharding that represents the op has a placeholder sharding.
  pub fn unknown(metadata: Vec<OpMetadata>) -> Self {
    HloSharding::new(false, false, true,
      false, metadata)
  }

  pub fn unreduced(metadata: Vec<OpMetadata>) -> Self {
    HloSharding::new(false, false, false,
      true, metadata)
  }

  // Creates a sharding that emulates device placement; a tile shape equal to
  // the input shape (one tile) assigned to a single device.
  pub fn assign_device(
    device_id: i64,
    metadata: Vec<OpMetadata>,
    use_named_sharding: bool) -> Self
  {
    if use_named_sharding {
      // TODO
      //return HloSharding::new_from_device_id(device_id, metadata);
    }
    HloSharding::new_from_device_id(device_id, metadata)
  }

  pub fn tile(_tile_assignment: TileAssignment, _metadata: &Vec<OpMetadata>) -> Self {
    unimplemented!()
  }

  // Similar to `Tile` but use IotaTileAssignment format.
  pub fn iota_tile(
    _tile_assignment_dims: &Vec<i64>,
    _reshape_dims: &Vec<i64>,
    _transpose_perm: &Vec<i64>, _metadata: &Vec<OpMetadata>) -> Self
  {
    unimplemented!()
  }

  pub fn partial_tile() {}

  // Creates a subgroup sharding with device-level tile assignment, the
  // sharding type of each subgroup is defined by subgroup_types. When creating
  // the HloSharding, subgroup dims of the same type will be merged.
  pub fn subgroup(
    _tile_assignment: TileAssignment,
    _subgroup_types: &Vec<OpShardingType>,
    _metadata: &Vec<OpMetadata>) -> Self
  {
    unimplemented!()
  }

  pub fn tile_id() {}
  pub fn tuple() {}

  // Creates a new sharding for a flat tuple type.
  pub fn flat_tuple(_sub_shardings: Vec<HloSharding>) -> HloSharding {
    unimplemented!()
  }

  pub fn single_tuple(&self, _tuple_shape: &Shape, _sharding: &HloSharding) {}

  pub fn single() {}

  // Create a new sharding from a protobuf OpSharding.
  pub fn from_proto(_proto: &OpSharding) -> Result<HloSharding, String> {
    unimplemented!()
  }

  // Checks whether device is a reserved device number.
  pub fn is_reserved_device(device: i64) -> bool {
    device < 0
  }

  pub fn to_proto(&self) -> OpSharding {
    unimplemented!()
  }
  
  // Prints the string representation of this sharding.
  // Note that this string canonically has outer curly braces, e.g. "{replicated}".
  pub fn print(&self, printer: &mut dyn Printer, include_metadata: bool) {
    if self.is_tuple() {
      debug_assert!(self.metadata.is_empty());
      if self.tuple_elements.is_empty() {
        printer.append(&"{}".to_string());
        return;
      }
      printer.append(&"{".to_string());
      self.tuple_elements[0].print(printer, include_metadata);
      for i in 1..self.tuple_elements.len() {
        if i % 5 == 0 {
          let mut msg = ", /*index=".to_string();
          msg.push_str(&i.to_string());
          msg.push_str(&"*/".to_string());
          printer.append(&msg);
        } else {
          printer.append(&", ".to_string());
        }
        self.tuple_elements[i].print(printer, include_metadata);
      }
      printer.append(&"}".to_string());
      return;
    }

    let print_metadata =
      |printer: &mut dyn Printer|
    {
      if include_metadata && !self.metadata.is_empty() {
        printer.append(&" metadata={".to_string());
        if self.metadata.len() == 1 {
          printer.append(&op_metadata_to_string(&self.metadata[0], false));
        } else {
          let mut count = 0;
          let metadata_len = self.metadata.len();
          for metadata in &self.metadata {
            printer.append(&"{".to_string());
            printer.append(&op_metadata_to_string(metadata, false));
            printer.append(&"}".to_string());
            count += 1;
            if count != metadata_len {
              printer.append(&", ".to_string());
            }
          }
        }
        printer.append(&"}".to_string());
      }
    };

    let print_shard_group =
      |printer: &mut dyn Printer|
    {
      let shard_g_str = self.shard_group.to_string();
      if !shard_g_str.is_empty() {
        let str = " ".to_string() + &shard_g_str;
        printer.append(&str);
      }
    };

    if self.replicated {
      printer.append(&"{replicated".to_string());
      print_shard_group(printer);
      print_metadata(printer);
      printer.append(&"}".to_string());
      return;
    }
    if self.manual {
      printer.append(&"{manual".to_string());
      print_shard_group(printer);
      print_metadata(printer);
      printer.append(&"}".to_string());
      return;
    }
    if self.unknown {
      printer.append(&"{unknown".to_string());
      print_shard_group(printer);
      print_metadata(printer);
      printer.append(&"}".to_string());
      return;
    }
    if self.unreduced {
      printer.append(&"{unreduced".to_string());
      print_shard_group(printer);
      print_metadata(printer);
      printer.append(&"}".to_string());
      return;
    }
    if self.maximal {
      printer.append(&"{maximal device=".to_string());
      let mut count = 0;
      for tile in self.tile_assignment.array().values() {
        printer.append(&tile.to_string());
        count += 1;
        if count != self.tile_assignment.array().values().len() {
          printer.append(&", ".to_string());
        }
      }
      print_shard_group(printer);
      print_metadata(printer);
      printer.append(&"}".to_string());
      return;
    }

    let print_last_tile_dims =
      |printer: &mut dyn Printer|
    {
      printer.append(&" last_tile_dims={".to_string());
      printer.append(&"}".to_string());
    };

    printer.append(&"{".to_string());
    self.tile_assignment.print(printer);
    if self.replicate_on_last_tile_dim {
      let op_sharding_type_to_string =
        |t: OpShardingType| -> String
      {
        match t {
          OpShardingType::Manual => return "manual".to_string(),
          OpShardingType::Maximal => return "maximal".to_string(),
          OpShardingType::Replicated => return "replicated".to_string(),
          OpShardingType::Unreduced => return "unreduced".to_string(),
          _ => return "error_type".to_string(),
        }
      };
      printer.append(&"last_tile_dim_replicate".to_string());
      let mut count = 0;
      let subg_len = self.subgroup_types.len();
      for subg_t in &self.subgroup_types {
        printer.append(&op_sharding_type_to_string(subg_t.clone()));
        count += 1;
        if count != subg_len {
          printer.append(&", ".to_string());
        }
      }
      printer.append(&"}".to_string());
    }

    printer.append(&"{".to_string());
    self.tile_assignment.print(printer);
    if self.replicate_on_last_tile_dim {
      printer.append(&"last_tile_dim_replicate".to_string());
    }
    print_last_tile_dims(printer);
    print_shard_group(printer);
    print_metadata(printer);
    printer.append(&"}".to_string());
  }

  // Returns the content printed by Print as a string.
  pub fn to_string(&self, include_metadata: bool) -> String {
    let mut printer = StringPrinter::new();
    self.print(&mut printer, include_metadata);
    printer.to_string()
  }

  // Validate that this sharding can be applied to a tensor with shape `shape`.
  pub fn validate(&self, _shape: &Shape, _num_devices: Option<i64>) -> Result<(), String>
  {
    unimplemented!()
  }

  // Returns true if the sharding has tuple type.
  pub fn is_tuple(&self) -> bool {
    self.tuple
  }

  // Returns true if the sharding is trivial: replicate on all devices.
  pub fn is_replicated(&self) -> bool {
    if !self.is_tuple() {
      return self.replicated;
    }
    for s in &self.tuple_elements {
      if !s.is_replicated() { return false; }
    }
    true
  }

  // Returns true if the tile size is the same as the input size.
  pub fn is_tile_maximal(&self) -> bool {
    if !self.is_tuple() {
      return self.maximal;
    }
    for s in &self.tuple_elements {
      if !s.is_tile_maximal() { return false; }
    }
    true
  }

  // Returns whether the sharding represents manual partitioning.
  pub fn is_manual(&self) -> bool {
    if !self.is_tuple() {
      return self.manual;
    }
    for s in &self.tuple_elements {
      if !s.is_manual() { return false; }
    }
    true
  }

  // Returns whether the sharding represents a placeholder sharding.
  pub fn is_unknown(&self) -> bool {
    if !self.is_tuple() {
      return self.unknown;
    }
    for s in &self.tuple_elements {
      if !s.is_unknown() { return false; }
    }
    true
  }

  pub fn is_shard_group(&self) -> bool {
    if !self.is_tuple() {
      return self.shard_group.shard_group_id != -1 &&
        (self.shard_group.shard_like || self.shard_group.shard_as);
    }
    if !self.tuple_elements.is_empty() {
      for s in &self.tuple_elements {
        if !s.is_shard_group() { return false; }
      }
      return true;
    }
    false
  }

  pub fn is_shard_as(&self) -> bool {
    if !self.is_tuple() {
      return self.shard_group.shard_group_id != -1 &&
        self.shard_group.shard_as;
    }
    if !self.tuple_elements.is_empty() {
      for s in &self.tuple_elements {
        if !s.is_shard_as() { return false; }
      }
      return true;
    }
    false
  }

  pub fn is_shard_like(&self) -> bool {
    if !self.is_tuple() {
      return self.shard_group.shard_group_id != -1 &&
        self.shard_group.shard_like;
    }
    if !self.tuple_elements.is_empty() {
      for s in &self.tuple_elements {
        if !s.is_shard_like() { return false; }
      }
      return true;
    }
    false
  }

  // Returns whether the sharding represents manual subgroup sharding.
  pub fn is_manual_subgroup(&self) -> bool {
    if !self.is_tuple() {
      for t in &self.subgroup_types {
        if t == &OpShardingType::Manual { return true; }
      }
      return false;
    }
    for s in &self.tuple_elements {
      if !s.is_manual_subgroup() { return false; }
    }
    true
  }

  // Represents whether the sharding represents a tiled sharding.
  pub fn is_tiled(&self) -> bool {
    !self.is_tile_maximal() && !self.is_manual() && !self.is_unknown()
  }

  // Returns if the sharding has partial replication and partial sharding.
  pub fn replicate_on_last_tile_dim(&self) -> bool {
    self.replicate_on_last_tile_dim
  }

  // Returns whether there is any partial replication.
  pub fn has_partial_replication(&self) -> bool {
    if self.replicate_on_last_tile_dim { return true; }
    for t in &self.subgroup_types {
      if *t == OpShardingType::Replicated { return true; }
    }
    false
  }

  // Returns true if the sharding defines an operation on the given device.
  pub fn uses_device(&self, device: i64) -> bool {
    if self.is_tuple() {
      for s in &self.tuple_elements {
        if s.uses_device(device) { return true; }
      }
      return false;
    }
    self.replicated || self.manual || self.tile_assignment.uses_device(device)
  }

  pub fn used_devices() {}

  // Returns the tile that should be executed on the given device.
  pub fn tile_index_for_device(&self, _device: i64) -> Vec<i64> {
    assert!(!self.maximal);
    assert!(!self.is_manual());
    assert!(!self.is_unknown());
    assert!(!self.is_tuple());

    let mut ret_index = Vec::new();
    
    assert!(!ret_index.is_empty());
    ret_index.resize(self.tiled_data_rank() as usize, 0);
    ret_index
  }

  pub fn device_for_tile_index() {}
  pub fn tile_offset_for_device() {}
  pub fn tile_limit_for_device() {}

  // Returns the single device this op operates on.
  pub fn unique_device(&self) -> Option<i64> {
    if self.is_tuple() {
      if self.tuple_elements.is_empty() {
        return None;
      }
      let mut unique_device = Some(0);
      for tuple_sharding in &self.tuple_elements {
        let device = tuple_sharding.unique_device();
        if device.is_none() ||
          (unique_device.is_some() &&
           device.as_ref().unwrap() != unique_device.as_ref().unwrap())
        {
          return None;
        }
        unique_device = device;
      }
      return unique_device;
    }
    if !self.replicated && self.maximal {
      //let val = self.tile_assignment.array().first().unwrap();
      //return Some(*val);
    }
    None
  }

  // Retrieves the unique device or fails with a check.
  pub fn get_unique_device(&self) -> i64 {
    let device = self.unique_device();
    assert!(device.is_some(), "Sharding does not have a unique device.");
    device.unwrap()
  }

  // Returns true if this op only uses a single device.
  pub fn has_unique_device(&self) -> bool {
    self.unique_device().is_some()
  }

  // Returns the ShapeTree containing the shardings for each element of this
  // tuple, if IsTuple, or a ShapeTree with a single element containing this
  // sharding. Only the leaf elements are populated. This creates a new
  // ShapeTree object so is not cheap.
  pub fn as_shape_tree(&self, _shape: &Shape) -> Result<ShapeTree<HloSharding>, String> {
    unimplemented!()
  }

  pub fn get_as_shape_tree() {}
  pub fn get_sub_sharding() {}
  pub fn get_tuple_sharding() {}

  // If the shape is tuple and the current sharding is not a tuple, attempt to
  // construct a sharding that is compatible with the shape by replicating the
  // current sharding across all tuple elements. Note that the returned
  // sharding is not guaranteed to be compatible with the input shape.
  pub fn normalize_tuple_sharding(&self, _shape: &Shape) -> Self {
    unimplemented!()
  }

  // Extracts the sharding that is common within the current sharding.
  pub fn extract_single_sharding(&self) -> Option<&HloSharding> {
    if !self.is_tuple() {
      return Some(self);
    }
    if self.tuple_elements.is_empty() {
      return None;
    }
    for i in 1..self.tuple_elements.len() {
      if self.tuple_elements[0] != self.tuple_elements[i] {
        return None;
      }
    }
    self.tuple_elements().first()
  }

  // Returns a copy of the sharding with no metadata. If sharding is of tuple
  // type, sub shardings will have no metadata.
  pub fn without_metadata(&self) -> HloSharding {
    let mut sharding = self.clone();
    sharding.metadata.clear();
    for sub_sharding in sharding.mutable_tuple_elements() {
      sub_sharding.metadata.clear()
    }
    sharding
  }

  pub fn with_metadata() {}

  // Gets the tile assignment tensor.
  pub fn tile_assignment(&self) -> &TileAssignment {
    &self.tile_assignment
  }

  // Gets the subgroup types array.
  pub fn subgroup_types(&self) -> &Vec<OpShardingType> {
    &self.subgroup_types
  }

  // Returns the flattened list of all the leaf shardings in a tuple shape,
  // by pre-order walk (ShapeTree iterator order).
  pub fn tuple_elements(&self) -> &Vec<HloSharding> {
    &self.tuple_elements
  }

  pub fn mutable_tuple_elements(&mut self) -> &mut Vec<HloSharding> {
    &mut self.tuple_elements
  }

  // Gets the tile shape.
  pub fn tile_shape(&self, shape: &Shape) -> Shape {
    if self.is_tile_maximal() || self.is_manual() || self.is_unknown() {
      return shape.clone();
    }
    let mut result_shape = shape.clone();
    for i in 0..self.tiled_data_rank() {
      let ceil_of_ratio =
        ((shape.dimensions(i as usize) / self.tile_assignment.dim(i)) as f64).ceil();
      result_shape.set_dimensions(i as usize, ceil_of_ratio as i64);
    }
    result_shape
  }

  // Gets the total number of tiles including subgroups and partial replication.
  pub fn total_num_tiles(&self) -> i64 {
    if self.is_tile_maximal() {
      return 1;
    }
    assert!(!self.is_manual());
    assert!(!self.is_unknown());
    let mut num = 1;
    for dim in self.tile_assignment.dimensions() {
      num *= *dim
    }
    num
  }

  // Gets the number of tiles.
  pub fn num_tiles(&self) -> i64 {
    if self.is_tile_maximal() {
      return 1;
    }
    assert!(!self.is_manual());
    assert!(!self.is_unknown());
    let mut num = 1;
    for i in 0..self.tiled_data_rank() {
        num *= self.tile_assignment.dimensions().get(i as usize).unwrap();
    }
    num
  }

  // Gets metadata from sharding.
  pub fn metadata(&self) -> &Vec<OpMetadata> {
    &self.metadata
  }

  // Returns the replication subgroup dim, or -1
  pub fn subgroup_replication_dim(&self) -> i64 {
    for i in 0..self.subgroup_types.len() {
      if self.subgroup_types[i] == OpShardingType::Replicated {
        return (i as i64) + self.tiled_data_rank();
      }
    }
    if self.replicate_on_last_tile_dim {
      return (self.tile_assignment.num_dimensions() as i64) - 1;
    }
    -1
  }

  // Returns the manual subgroup dim, or -1 if it doesn't exist.
  pub fn subgroup_manual_dim(&self) -> i64 {
    for i in 0..self.subgroup_types.len() {
      if self.subgroup_types[i] == OpShardingType::Manual {
        return (i as i64) + self.tiled_data_rank();
      }
    }
    -1
  }

  // Returns the data rank for tiled sharding. It doesn't include subgroup dims.
  pub fn tiled_data_rank(&self) -> i64 {
    assert!(self.is_tiled());
    let mut rank = self.tile_assignment.num_dimensions() as i64;
    if self.replicate_on_last_tile_dim() {
      rank -= 1;
    }
    rank -= self.subgroup_types.len() as i64;
    rank
  }

  // Returns the number of tuple_elements entries to fit the shape.
  pub fn required_leaves(shape: &Shape) -> i64 {
    let leaf_count = ShapeUtil::get_leaf_count(shape) as i64;
    if leaf_count == 0 {
      1
    } else {
      leaf_count
    }
  }

  pub fn not_shard_group() -> ShardGroup {
    ShardGroup::new(-1, false, false)
  }

  pub fn shard_as(shard_group_id: i64) -> ShardGroup {
    ShardGroup::new(shard_group_id, true, false)
  }

  pub fn shard_like(shard_group_id: i64) -> ShardGroup {
    ShardGroup::new(shard_group_id, false, true)
  }

  pub fn set_shard_group(&mut self, shard_group: ShardGroup) {
    self.shard_group = shard_group;
  }

  pub fn clear_shard_group(&mut self) {
    self.shard_group = HloSharding::not_shard_group();
  }

  pub fn get_shard_group(&self) -> &ShardGroup {
    &self.shard_group
  }
}