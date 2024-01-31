#![allow(dead_code)]

use crate::{
  blitz_data::{DimLevelType, PrimitiveType},
  util::DimensionVector, shape::Shape, printer::Printer
};

#[derive(Clone)]
pub struct Tile {
  dimensions: Vec<i64>,
}

impl Tile {
  pub const COMBINE_DIMENSION: i64 = i64::MIN;

  pub fn new_default() -> Self {
    Tile {
      dimensions: Vec::new()
    }
  }

  pub fn new(dimensions: Vec<i64>) -> Self {
    Tile {
      dimensions: dimensions
    }
  }

  pub fn print() {}
  pub fn to_string() {}

  pub fn dimension(&self, i: usize) -> i64 {
    self.dimensions[i]
  }

  pub fn dimensions(&self) -> &Vec<i64> {
    &self.dimensions
  }

  pub fn add_dimensions(&mut self, value: i64) {
    self.dimensions.push(value);
  }

  pub fn clear_dimensions(&mut self) {
    self.dimensions.clear();
  }

  pub fn absl_hash_value() {}
}

#[derive(Clone)]
pub struct DimInfo {
  dim_level_type: DimLevelType,
  dim_unique: bool,
  dim_orderd: bool,
}

impl DimInfo {
  pub fn new() -> Self {
    DimInfo {
      dim_level_type: DimLevelType::Dense,
      dim_unique: false,
      dim_orderd: false,
    }
  }
}

#[derive(Clone)]
pub struct Layout {
  dim_attributes: Vec<DimInfo>,
  n_dim_level_types: usize,
  n_dim_unique: i64,
  n_dim_ordered: i64,
  element_size_in_bits: i64,
  index_primitive_type: PrimitiveType,
  pointer_primitive_type: PrimitiveType,
  memory_space: i64,
  dynamic_shape_metadata_prefix_bytes: i64,
  minor_to_major: DimensionVector,
  tiles: Vec<Tile>,
  physical_shape: Option<Box<Shape>>,
}

impl Layout {
  pub const DEFAULT_MEMORY_SPACE: i64 = 0;

  pub fn new() -> Self {
    Layout {
      dim_attributes: Vec::new(),
      n_dim_level_types: 0,
      n_dim_unique: 0,
      n_dim_ordered: 0,
      element_size_in_bits: 0,
      index_primitive_type: PrimitiveType::Invalid,
      pointer_primitive_type: PrimitiveType::Invalid,
      memory_space: 0,
      dynamic_shape_metadata_prefix_bytes: 0,
      minor_to_major: Vec::new(),
      tiles: Vec::new(),
      physical_shape: None,
    }
  }

  pub fn print(&self, _printer: &dyn Printer) {}

  pub fn to_string(&self) -> String {
    "".to_string() // TODO
  }

  pub fn dim_level_types_size(&self) -> usize {
    self.n_dim_level_types
  }

  pub fn dim_level_type(&self, index: usize) -> DimLevelType {
    self.dim_attributes[index].dim_level_type.clone()
  }

  pub fn set_dim_level_type(&mut self, index: usize, dim_level_type: DimLevelType) {
    self.dim_attributes[index].dim_level_type = dim_level_type;
  }

  pub fn add_dim_level_type(&mut self, dim_level_type: DimLevelType) {
    while self.n_dim_level_types as usize >= self.dim_attributes.len() {
      self.dim_attributes.push(DimInfo::new());
    };
    self.dim_attributes[self.n_dim_level_types as usize].dim_level_type = dim_level_type;
    self.n_dim_level_types += 1;
  }

  pub fn clear_dim_level_types(&mut self) {
    self.n_dim_level_types = 0;
  }

  pub fn dim_unique_size(&self) -> i64 {
    self.n_dim_unique
  }

  pub fn dim_unique(&self, index: usize) -> bool {
    self.dim_attributes[index].dim_unique
  }

  pub fn set_dim_unique(&mut self, index: usize, unique: bool) {
    self.dim_attributes[index].dim_unique = unique;
  }

  pub fn add_dim_unique(&mut self, unique: bool) {
    while self.n_dim_unique as usize >= self.dim_attributes.len() {
      self.dim_attributes.push(DimInfo::new());
    };
    self.dim_attributes[self.n_dim_unique as usize].dim_unique = unique;
    self.n_dim_unique += 1;
  }

  pub fn dim_ordered_size(&self) -> i64 {
    self.n_dim_ordered
  }

  pub fn dim_ordered(&self, index: usize) -> bool {
    self.dim_attributes[index].dim_orderd
  }

  pub fn set_dim_ordered(&mut self, index: usize, ordered: bool) {
    self.dim_attributes[index].dim_orderd = ordered;
  }

  pub fn add_dim_ordered(&mut self, ordered: bool) {
    while self.n_dim_ordered as usize >= self.dim_attributes.len() {
      self.dim_attributes.push(DimInfo::new());
    };
    self.dim_attributes[self.n_dim_ordered as usize].dim_orderd = ordered;
  }

  pub fn minor_to_major_size(&self) -> usize {
    self.minor_to_major.len()
  }

  pub fn minor_to_major(&self, index: usize) -> i64 {
    self.minor_to_major[index]
  }

  pub fn set_minor_to_major(&mut self, index: usize, value: i64) {
    self.minor_to_major[index] = value;
  }

  pub fn add_minor_to_major(&mut self, value: i64) {
    self.minor_to_major.push(value);
  }

  pub fn clear_minor_to_major(&mut self) {
    self.minor_to_major.clear();
  }

  pub fn delete_dimension(&mut self, _dim_to_delete: i64) {}

  pub fn minor_to_major_vec(&self) -> &DimensionVector {
    &self.minor_to_major
  }

  pub fn minor_to_major_vec_mut(&mut self) -> &mut DimensionVector{
    &mut self.minor_to_major
  }

  pub fn tiles_size(&self) -> usize {
    self.tiles.len()
  }

  pub fn tiles(&self, index: usize) -> &Tile {
    &self.tiles[index]
  }

  pub fn add_tiles(&mut self, tile: Tile) {
    //self.tiles.push(Tile::new_default());
    self.tiles.push(tile);
  }

  pub fn clear_tiles(&mut self) {
    self.tiles.clear();
  }

  pub fn element_size_in_bits(&self) -> i64 {
    self.element_size_in_bits
  }

  pub fn set_element_size_in_bits(&mut self, value: i64) {
    self.element_size_in_bits = value;
  }

  pub fn index_primitive_type(&self) -> PrimitiveType {
    self.index_primitive_type.clone()
  }

  pub fn set_index_primitive_type(&mut self, value: PrimitiveType) {
    self.index_primitive_type = value;
  }

  pub fn pointer_primitive_type(&self) -> PrimitiveType {
    self.pointer_primitive_type.clone()
  }

  pub fn set_pointer_primitive_type(&mut self, value: PrimitiveType) {
    self.pointer_primitive_type = value;
  }

  pub fn memory_space(&self) -> i64 {
    self.memory_space
  }

  pub fn set_memory_space(&mut self, value: i64) {
    self.memory_space = value;
  }
  pub fn has_physical_shape(&self) -> bool {
    self.physical_shape.is_some()
  }

  pub fn set_physical_shape(&mut self, shape: Shape) {
    self.physical_shape = Some(Box::new(shape));
  }

  pub fn physical_shape(&self) -> &Option<Box<Shape>> {
    &self.physical_shape
  }

  pub fn clear_physical_shape() {}

  pub fn dynamic_shape_metadata_prefix_bytes(&self) -> i64 {
    self.dynamic_shape_metadata_prefix_bytes
  }

  pub fn set_dynamic_shape_metadata_prefix_bytes(&mut self, bytes: i64) {
    self.dynamic_shape_metadata_prefix_bytes = bytes;
  }

  pub fn swap() {}
  pub fn clear() {}
  pub fn absl_hash_value() {}
}