#![allow(dead_code)]

use common::{array::Array, blitz_data::{OpSharding, OpShardingType}, shape::Shape, shape_tree::ShapeTree};

// Creates a replicated sharding - replicate a tensor on every device.
pub fn replicate() -> OpSharding {
  let mut result = OpSharding::new();
  result.set_type(OpShardingType::Replicated);
  result
}

// Creates a manual sharding - the partitioner will not change the shape.
pub fn manual() -> OpSharding {
  let mut result = OpSharding::new();
  result.set_type(OpShardingType::Manual);
  result
}

// Creates a sharding that assigns a tensor to just one device.
pub fn assign_device(device: i64) -> OpSharding {
  let mut result = OpSharding::new();
  result.set_type(OpShardingType::Maximal);
  result.add_tile_assignment_dimensions(1);
  result.add_tile_assignment_devices(device);
  result
}

// Creates a tiled sharding with the given tile shape and assignment of tiles
// to devices.
//
// If tile_shape is not evenly divisible by the number of devices in
// tile_assignment, operations behave as if implicit padding had been inserted.
// The value of this padding is undefined.
pub fn tile(tile_shape: &Shape, tile_assignment: &Array) -> OpSharding {
  let mut result = OpSharding::new();
  result.set_type(OpShardingType::Other);
  result.set_tile_shape(tile_shape.clone());
  
  for dim in tile_assignment.dimensions() {
    result.add_tile_assignment_dimensions(*dim);
  }
  // TODO
  //for device in tile_assignment {
  //}
  result
}

// Creates a sharding in one dimension, with the given tile shape which must
// be rank 1 and using devices [0..num_tiles).
//
// This is simply a convenience wrapper for Tile().
pub fn tile_1d(_tile_shape: &Shape, _num_tiles: i64) -> OpSharding {
  unimplemented!()
}

// Creates a tuple sharding from the given ShapeTree of element shardings.
pub fn tuple(_shardings: &ShapeTree<OpSharding>) -> OpSharding {
  let mut result = OpSharding::new();
  result.set_type(OpShardingType::Tuple);

  // TODO
  //for index_to_sharding in shardings {
  //}
  result
}