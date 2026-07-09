#![allow(dead_code)]

use std::cmp::max;

use crate::{
  blitz_data::{DimLevelType, PrimitiveType},
  util::DimensionVector, shape::Shape, printer::{Printer, StringPrinter}, primitive_util, layout_util::LayoutUtil
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tile {
  dimensions: Vec<i64>,
}

impl Tile {
  pub const COMBINE_DIMENSION: i64 = i64::MIN;

  pub fn default() -> Self {
    Tile {
      dimensions: Vec::new()
    }
  }

  pub fn new(dimensions: Vec<i64>) -> Self {
    Tile {
      dimensions: dimensions
    }
  }

  pub fn print(&self, printer: &mut dyn Printer) {
    printer.append(&"(".to_string());
    let print_dim = |dim: i64, printer: &mut dyn Printer| {
      if dim >= 0 {
        printer.append(&dim.to_string());
      } else {
        if dim == Tile::COMBINE_DIMENSION {
          printer.append(&"*".to_string());
        } else {
          printer.append(&"Invalid value ".to_string());
          printer.append(&dim.to_string());
        }
      }
    };
    let last_index = self.dimensions.len();
    let mut loop_count = 1;
    let mut iter = self.dimensions.iter();
    loop {
      let dim = iter.next();
      if dim != None && loop_count < last_index {
        print_dim(dim.unwrap().clone(), printer);
        printer.append(&",".to_string());
        loop_count += 1;
      } else if dim != None && loop_count == last_index {
        print_dim(dim.unwrap().clone(), printer);
        break;
      } else {
        break;
      }
    }
    printer.append(&")".to_string());
  }

  pub fn to_string(&self) -> String {
    let mut printer = StringPrinter::new();
    self.print(&mut printer);
    printer.to_string()
  }

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Layout {
  dim_attributes: Vec<DimInfo>,
  n_dim_level_types: usize,
  n_dim_unique: usize,
  n_dim_ordered: usize,
  element_size_in_bits: i64,
  index_primitive_type: PrimitiveType,
  pointer_primitive_type: PrimitiveType,
  memory_space: i64,
  dynamic_shape_metadata_prefix_bytes: i64,
  minor_to_major: DimensionVector,
  tiles: Vec<Tile>,
  physical_shape: Option<Box<Shape>>,
  tail_padding_alignment_in_elements: i64,
  split_configs: Vec<SplitConfig>,
}

impl Layout {
  pub const DEFAULT_MEMORY_SPACE: i64 = 0;

  pub fn default() -> Self {
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
      tail_padding_alignment_in_elements: 1,
      split_configs: Vec::new()
    }
  }

  pub fn new_from_minor_to_major(minor_to_major: Vec<i64>) -> Self {
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
      minor_to_major: minor_to_major,
      tiles: Vec::new(),
      physical_shape: None,
      tail_padding_alignment_in_elements: 1,
      split_configs: Vec::new()
    }
  }

  pub fn new_from(
    minor_to_major: Vec<i64>,
    dim_level_types: Vec<DimLevelType>,
    dim_unique: Vec<bool>,
    dim_orered: Vec<bool>,
    tiles: Vec<Tile>,
    tail_padding_alignment_in_elements: i64,
    index_primitive_type: PrimitiveType,
    element_primitive_type: PrimitiveType,
    element_size_in_bits: i64,
    memory_space: i64,
    physical_shape: Option<Box<Shape>>,
    dynamic_shape_metadata_prefix_bytes: i64
  ) -> Self
  {
    let mut result = Layout {
      dim_attributes: Vec::new(),
      n_dim_level_types: dim_level_types.len(),
      n_dim_unique: dim_unique.len(),
      n_dim_ordered: dim_orered.len(),
      element_size_in_bits: element_size_in_bits,
      index_primitive_type: index_primitive_type,
      pointer_primitive_type: element_primitive_type,
      memory_space: memory_space,
      dynamic_shape_metadata_prefix_bytes: dynamic_shape_metadata_prefix_bytes,
      minor_to_major: minor_to_major,
      tiles: tiles,
      physical_shape: physical_shape,
      tail_padding_alignment_in_elements: tail_padding_alignment_in_elements,
      split_configs: vec![]
    };

    let n_attributes = max(result.n_dim_level_types, 
      max(result.n_dim_unique, result.n_dim_ordered));
    result.dim_attributes.resize(n_attributes, DimInfo::new());
    for i in 0..n_attributes {
      if i < result.n_dim_level_types {
        result.dim_attributes[i].dim_level_type = dim_level_types[i].clone();
      }
      if i < result.n_dim_unique {
        result.dim_attributes[i].dim_unique = dim_unique[i];
      }
      if i < result.n_dim_ordered {
        result.dim_attributes[i].dim_orderd = dim_orered[i];
      }
    }

    result
  }

  // Prints this layout as human-readable string, in the format
  // "{minor_to_major:properties}", where the fields are:
  //
  //   minor_to_major: Comma-separated minor-to-major order of the dimensions.
  //                   E.g. "{1,0}" means that dimension 1 is the most minor
  //                   dimension, and dimension 0 is the most major dimension.
  //   properties: concatenation of the following, separated by nothing (a
  //               property is ommitted if it is the default):
  //     T(...)...(...): The tiling (each (...) is acomma-separated list of
  //                     tile bound sizes). E.g.
  //             T(2,4)(3,5): The shape is tiled with 2x4 and 3x5 tiles.
  //             T(*,*,2,*,4): The dimensions corresponding the '*' are first
  //                 combined with the next more minor dimension, and then the
  //                 result shape is tiled with 2x4 tiles.
  //             If omitted, the shape is not tiled.
  //     L(n): The tail padding alignment in elements. Omitted if n is 1.
  //     #(type): The type of the indices.
  //     *(type): The type of the pointers.
  //     E(n): The element size in bits.
  //     S(n): The numeric value of thememory space. See the definition of
  //           Layout::memory_space() for details.
  //     SC(...)...(...): List of split configs, separated by nothing. Each
  //              (...) is a string of the form "(dimension:split_indices)".
  //              E.g. SC(1:512)(2:1024,2048): dimension 1 is split into 2 parts
  //              at index 512, and dimension 2 is split into 3 parts at index
  //              1024 and 2048.
  //     P(shape): The physical shape.
  //     M(n): The dynamic shape metadata prefix bytes. Omitted if n is 0.
  pub fn print(&self, printer: &mut dyn Printer) {
    printer.append(&"{".to_string());
    self.append_join_minor_to_major(printer);

    let mut colon_printed = false;
    let mut print_colon = |printer: &mut dyn Printer| {
      if colon_printed { return; }
      printer.append(&":".to_string());
      colon_printed = true;
    };
    if self.n_dim_level_types > 0 {
      let print_one = |i: i64, printer: &mut dyn Printer| {
        printer.append(&Layout::dim_level_type_abbrev(&self.dim_level_type(i as usize)));
        if self.n_dim_unique > 0 && !self.dim_unique(i as usize) {
          printer.append(&"+".to_string());
        }
        if self.n_dim_ordered > 0 && !self.dim_ordered(i as usize) {
          printer.append(&"~".to_string());
        }
      };
      print_colon(printer);
      printer.append(&"D(".to_string());
      print_one(0, printer);
      for i in 1..self.n_dim_level_types {
        printer.append(&",".to_string());
        print_one(i as i64, printer);
      }
      printer.append(&")".to_string());
    }

    // Print the tiles as T(...)...(...).
    if !self.tiles.is_empty() {
      print_colon(printer);
      printer.append(&"T".to_string());
      for tile in &self.tiles {
        tile.print(printer);
      }
    }

    // Print the tail padding alignment as L(n). Omit this if n is 1.
    if self.tail_padding_alignment_in_elements() != 1 {
      print_colon(printer);
      printer.append(&"L(".to_string());
      printer.append(&self.tail_padding_alignment_in_elements.to_string());
      printer.append(&")".to_string());
    }

    // Print the primitive type used for indices as #(type). Print
    // #(invalid) if the type is valid but not an integer. Omit this if the type
    // is PRIMITIVE_TYPE_INVALID.
    if self.index_primitive_type() != PrimitiveType::Invalid {
      print_colon(printer);
      if primitive_util::is_integral_type(&self.index_primitive_type) {
        printer.append(&"#(".to_string());
        let primitive_type_name =
          primitive_util::lowercase_primitive_type_name(&self.index_primitive_type);
        printer.append(&primitive_type_name);
        printer.append(&")".to_string());
      } else {
        printer.append(&"#(invalid)".to_string());
      }
    }

    // Print the primitive type used for poitners as *(type). Print *(invalid) if
    // the type is valid but not a pointer. Omit this if the type is
    // PRIMITIVE_TYPE_INVALID.
    if self.pointer_primitive_type != PrimitiveType::Invalid {
      print_colon(printer);
      if primitive_util::is_integral_type(&self.pointer_primitive_type) {
        printer.append(&"*(".to_string());
        let primitive_type_name =
          primitive_util::lowercase_primitive_type_name(&self.pointer_primitive_type);
        printer.append(&primitive_type_name);
        printer.append(&")".to_string());
      } else {
        printer.append(&"*(invalid".to_string());
      }
    }

    // Print the element size in bits as E(n). Omit this if n is 0.
    if self.element_size_in_bits != 0 {
      print_colon(printer);
      printer.append(&"E(".to_string());
      printer.append(&self.element_size_in_bits.to_string());
      printer.append(&")".to_string());
    }

    // Print the memory space as S(n). Omit this if n is 0.
    if self.memory_space != 0 {
      print_colon(printer);
      printer.append(&"S(".to_string());
      printer.append(&self.memory_space.to_string());
      printer.append(&")".to_string());
    }

    // Print the split configs as SC(...)...(...). Omit this if the split configs
    // are empty.
    if !self.split_configs().is_empty() {
      print_colon(printer);
      printer.append(&"SC".to_string());
      for sc in self.split_configs() {
        printer.append(&sc.to_string());
      }
    }

    // Print the physical shape as P(physical_shape). Omit this if the physical
    // shape is not set.
    if self.has_physical_shape() {
      print_colon(printer);
      printer.append(&"P(".to_string());
      self.physical_shape().as_ref().unwrap().print(printer, true);
      printer.append(&")".to_string());
    }

    // Print the dynamic shape metadata prefix bytes as M(n). Omit this if n is 0.
    if self.dynamic_shape_metadata_prefix_bytes > 0 {
      print_colon(printer);
      printer.append(&"M(".to_string());
      printer.append(&self.dynamic_shape_metadata_prefix_bytes.to_string());
      printer.append(&")".to_string());
    }

    printer.append(&"}".to_string())
  }

  // Returns a human-readable string that represents this layout.
  pub fn to_string(&self) -> String {
    let mut printer = StringPrinter::new();
    self.print(&mut printer);
    printer.to_string()
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

  pub fn dim_unique_size(&self) -> usize {
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

  pub fn dim_ordered_size(&self) -> usize {
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

  // Removes the given dimension from 'minor_to_major_', and adjusts the other
  // dimensions accordingly.
  // Precondition: dim_to_delete is in the range [0, minor_to_major_size()).
  pub fn delete_dimension(&mut self, dim_to_delete: i64) {
    debug_assert!(dim_to_delete >= 0);
    debug_assert!(dim_to_delete < self.minor_to_major.len() as i64);
    for i in 0..self.minor_to_major.len() {
      if self.minor_to_major[i] == dim_to_delete {
        self.minor_to_major.remove(i);
        continue;
      }
      if self.minor_to_major[i] > dim_to_delete {
        self.minor_to_major[i] -= 1;
      }
    }
  }

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

  pub fn tiles_vec(&self) -> &Vec<Tile> {
    &self.tiles
  }

  pub fn add_tiles(&mut self, tile: Tile) {
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

  pub fn tail_padding_alignment_in_elements(&self) -> i64 {
    self.tail_padding_alignment_in_elements
  }

  pub fn set_tail_padding_alignment_in_elements(&mut self, value: i64) {
    self.tail_padding_alignment_in_elements = value;
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

  pub fn mutable_physical_shape(&mut self) -> &mut Option<Box<Shape>> {
    &mut self.physical_shape
  }

  pub fn clear_physical_shape() {}

  pub fn dynamic_shape_metadata_prefix_bytes(&self) -> i64 {
    self.dynamic_shape_metadata_prefix_bytes
  }

  pub fn set_dynamic_shape_metadata_prefix_bytes(&mut self, bytes: i64) {
    self.dynamic_shape_metadata_prefix_bytes = bytes;
  }

  pub fn split_configs(&self) -> &Vec<SplitConfig> {
    &self.split_configs
  }

  pub fn split_config(&self, index: usize) -> &SplitConfig {
    &self.split_configs[index]
  }

  pub fn mutable_split_config(&mut self, index: usize) -> &mut SplitConfig {
    &mut self.split_configs[index]
  }

  pub fn add_split_config(&mut self, split_config: SplitConfig) {
    self.split_configs.push(split_config);
  }

  pub fn clear_split_configs(&mut self) {
    self.split_configs.clear();
  }

  pub fn swap() {}
  pub fn clear() {}
  pub fn absl_hash_value() {}

  fn dim_level_type_abbrev(t: &DimLevelType) -> String {
    match *t {
      DimLevelType::Dense => "D".to_string() ,
      DimLevelType::Compressed => "C".to_string(),
      DimLevelType::Singleton => "S".to_string(),
      DimLevelType::LooseCompressed => "H".to_string(),
    }
  }

  fn append_join_minor_to_major(&self, printer: &mut dyn Printer) {
    let last_index = self.minor_to_major.len();
    let mut loop_count = 1;
    let mut iter = self.minor_to_major.iter();
    loop {
      let elt = iter.next();
      if elt != None && loop_count < last_index {
        printer.append(&elt.unwrap().to_string());
        printer.append(&",".to_string());
        loop_count += 1;
      } else if elt != None && loop_count == last_index {
        printer.append(&elt.unwrap().to_string());
        return;
      } else {
        return;
      }
    }
  }
}

pub struct LayoutEqual {
  ignore_tiles: bool,
  ignore_tail_padding_alignment_in_elements: bool,
  ignore_element_size: bool,
  ignore_index_primitive_type: bool,
  ignore_pointer_primitive_type: bool,
  ignore_memory_space: bool,
  ignore_physical_shape: bool,
}

impl LayoutEqual {
  pub fn new() -> Self {
    LayoutEqual {
      ignore_tiles: false,
      ignore_tail_padding_alignment_in_elements: false,
      ignore_element_size: false,
      ignore_index_primitive_type: false,
      ignore_pointer_primitive_type: false,
      ignore_memory_space: false,
      ignore_physical_shape: false,
    }
  }

  pub fn equal(&self, lhs: &Layout, rhs: &Layout) -> bool {
    if !LayoutUtil::is_dense(lhs) || !LayoutUtil::is_dense(rhs) {
      // dim_level_types
      if lhs.dim_level_types_size() != rhs.dim_level_types_size() {
        return false;
      }
      for i in 0..lhs.dim_level_types_size() {
        if lhs.dim_level_type(i) != rhs.dim_level_type(i) {
          return false;
        }
      }
      // dim_unique
      if lhs.dim_unique_size() != rhs.dim_unique_size() {
        return false;
      }
      for i in 0..lhs.dim_unique_size() {
        if lhs.dim_unique(i as usize) != rhs.dim_unique(i as usize) {
          return false;
        }
      }
      // dim_ordered
      if lhs.dim_ordered_size() != rhs.dim_ordered_size() {
        return false;
      }
      for i in 0..lhs.dim_ordered_size() {
        if lhs.dim_ordered(i as usize) != rhs.dim_ordered(i as usize) {
          return false;
        }
      }
    }
    if lhs.minor_to_major_vec() != rhs.minor_to_major_vec() {
      return false;
    }
    if !self.ignore_tiles && lhs.tiles != rhs.tiles {
      return false;
    }
    if !self.ignore_tail_padding_alignment_in_elements &&
      lhs.tail_padding_alignment_in_elements() != rhs.tail_padding_alignment_in_elements() {
        return false;
    }
    if !self.ignore_index_primitive_type &&
      lhs.index_primitive_type() != rhs.index_primitive_type() {
      return false;
    }
    if !self.ignore_pointer_primitive_type &&
      lhs.pointer_primitive_type() != rhs.pointer_primitive_type() {
      return false;
    }
    if !self.ignore_element_size &&
      lhs.element_size_in_bits() != rhs.element_size_in_bits() {
      return false;
    }
    if !self.ignore_memory_space &&
      lhs.memory_space() != rhs.memory_space() {
      return false;
    }
    if !self.ignore_physical_shape {
      if lhs.has_physical_shape() || rhs.has_physical_shape() {
        if !lhs.has_physical_shape() || !rhs.has_physical_shape() {
          return false;
        }
        //if lhs.physical_shape() != rhs.physical_shape() {
          //return false;
        //}
      }
    }
    true
  }

  pub fn ignore_tiles(&mut self) -> &Self {
    self.ignore_tiles = true;
    self
  }

  pub fn ignore_tail_padding_alignment_in_elements(&mut self) -> &Self {
    self.ignore_tail_padding_alignment_in_elements = true;
    self
  }

  pub fn ignore_index_primitive_type(&mut self) -> &Self {
    self.ignore_index_primitive_type = true;
    self
  }

  pub fn ignore_pointer_primitive_type(&mut self) -> &Self {
    self.ignore_pointer_primitive_type = true;
    self
  }

  pub fn ignore_memory_space(&mut self) -> &Self {
    self.ignore_memory_space = true;
    self
  }

  pub fn ignore_physical_shape(&mut self) -> &Self {
    self.ignore_physical_shape = true;
    self
  }

  pub fn ignore_element_size(&mut self) -> &Self {
    self.ignore_element_size = true;
    self
  }

  pub fn minor_to_major_only(&mut self) -> &Self {
    self.ignore_tiles = true;
    self.ignore_index_primitive_type = true;
    self.ignore_pointer_primitive_type = true;
    self.ignore_memory_space = true;
    self.ignore_physical_shape = true;
    self.ignore_element_size = true;
    self.ignore_tail_padding_alignment_in_elements = true;
    self
  }
}

// Describes how data is split between different memories. Each SplitConfig
// object represents a split in one dimension. Each SplitConfig is associated
// with a vector of split indices which point to the points in the iteration
// where the splits occur. For example, if the dimension contains 1024 elements,
// a split indices value of {512} indicates splitting this dimension into two
// right through the middle. The dimension here refers to the physical dimension
// such that 0 is the majormost dimension and (number of dimensions - 1) is the
// minormost dimension.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SplitConfig {
  dimension: i64,
  split_indices: Vec<i64>,
}

impl SplitConfig {
  pub fn new(dimension: i64, split_indices: Vec<i64>) -> Self {
    SplitConfig { dimension: dimension, split_indices: split_indices }
  }

  // Returns the dimension that is split.
  pub fn dimension(&self) -> i64 {
    self.dimension
  }

  pub fn set_dimension(&mut self, dimension: i64) {
    self.dimension = dimension;
  }

  // Returns the indices where splits occur.
  pub fn split_indices(&self) -> &Vec<i64> {
    &self.split_indices
  }

  pub fn split_indices_at(&self, index: usize) -> i64 {
    self.split_indices[index]
  }

  pub fn split_indices_size(&self) -> usize {
    self.split_indices.len()
  }

  pub fn add_split_indices(&mut self, split_index: i64) {
    self.split_indices.push(split_index);
  }

  pub fn clear_split_indices(&mut self) {
    self.split_indices.clear();
  }

  pub fn to_string(&self) -> String {
    let mut result = "(".to_string();
    result.push_str(&self.dimension.to_string());
    result.push_str(&":".to_string());
    let mut counter = self.split_indices.len();
    for i in &self.split_indices {
      result.push_str(&i.to_string());
      counter -= 1;
      if counter != 0 {
        result.push_str(",");
      }
    }
    result.push_str(")");
    result
  }
}

#[cfg(test)]
mod tests {
  use crate::shape_util::ShapeUtil;

use super::*;

  #[test]
  fn test_to_string_for_empty() {
    assert_eq!(Layout::default().to_string(), "{}");
  }

  #[test]
  fn test_to_string_for_minor_to_major_only() {
    let layout = Layout::new_from_minor_to_major(vec![1, 2, 0]);
    assert_eq!(layout.to_string(), "{1,2,0}");
  }

  #[test]
  fn test_to_string_for_tiles() {
    let layout = Layout::new_from(
      vec![3, 2, 1, 0],
      vec![], vec![], vec![],
      vec![Tile::new(vec![42, 123]), Tile::new(vec![4, 5])],
      1,
      PrimitiveType::Invalid,
      PrimitiveType::Invalid,
      0, 0, None,
      0);
    assert_eq!(layout.to_string(), "{3,2,1,0:T(42,123)(4,5)}");
  }

  #[test]
  fn test_to_string_for_tile_with_combined_dimensions() {
    let layout = Layout::new_from(
      vec![3, 2, 1, 0],
      vec![], vec![], vec![],
      vec![Tile::new(vec![i64::MIN, i64::MIN, 42, 123])],
      1,
      PrimitiveType::Invalid,
      PrimitiveType::Invalid,
      0, 0, None,
      0);
    assert_eq!(layout.to_string(), "{3,2,1,0:T(*,*,42,123)}");
  }

  #[test]
  fn test_to_string_for_tail_padding_alignment() {
    let mut layout = Layout::new_from_minor_to_major(vec![3, 2, 1, 0]);
    layout.set_tail_padding_alignment_in_elements(100);
    assert_eq!(layout.to_string(), "{3,2,1,0:L(100)}");
  }

  #[test]
  fn test_to_string_for_index_primitive_type() {
    let mut layout = Layout::new_from_minor_to_major(vec![3, 2, 1, 0]);
    layout.set_index_primitive_type(PrimitiveType::U32);
    assert_eq!(layout.to_string(), "{3,2,1,0:#(u32)}");
  }

  #[test]
  fn test_to_string_for_pointer_primitive_type() {
    let mut layout = Layout::new_from_minor_to_major(vec![3, 2, 1, 0]);
    layout.set_pointer_primitive_type(PrimitiveType::U16);
    assert_eq!(layout.to_string(), "{3,2,1,0:*(u16)}");
  }

  #[test]
  fn test_to_string_for_element_size() {
    let mut layout = Layout::new_from_minor_to_major(vec![3, 2, 1, 0]);
    layout.set_element_size_in_bits(42);
    assert_eq!(layout.to_string(), "{3,2,1,0:E(42)}");
  }

  #[test]
  fn test_to_string_for_memory_space() {
    let mut layout = Layout::new_from_minor_to_major(vec![3, 2, 1, 0]);
    layout.set_memory_space(3);
    assert_eq!(layout.to_string(), "{3,2,1,0:S(3)}");
  }

  #[test]
  fn test_to_string_for_split_configs() {
    let mut layout = Layout::new_from_minor_to_major(vec![0, 1]);
    layout.add_split_config(SplitConfig::new(0, vec![3]));
    layout.add_split_config(SplitConfig::new(1, vec![0, 4]));
    assert_eq!(layout.to_string(), "{0,1:SC(0:3)(1:0,4)}");
  }

  #[test]
  fn test_to_string_for_physical_shape() {
    let mut layout = Layout::new_from_minor_to_major(vec![0, 1]);
    let shape = ShapeUtil::make_shape(
      &PrimitiveType::S32, vec![10, 20]);
    layout.set_physical_shape(shape);
    assert_eq!(layout.to_string(), "{0,1:P(s32[10,20]{1,0})}");
  }

  #[test]
  fn test_to_string_for_dynamic_shape_metadata_prefix_bytes() {
    let mut layout = Layout::new_from_minor_to_major(vec![0, 1]);
    layout.set_dynamic_shape_metadata_prefix_bytes(123);
    assert_eq!(layout.to_string(), "{0,1:M(123)}");
  }

  #[test]
  fn test_to_string_for_multiple_properties() {
    let mut layout = Layout::new_from(
      vec![3, 2, 1, 0],
      vec![], vec![], vec![],
      vec![Tile::new(vec![42, 123]), Tile::new(vec![4, 5])],
      1,
      PrimitiveType::Invalid,
      PrimitiveType::Invalid,
      0, 0, None,
      0);
    layout.set_tail_padding_alignment_in_elements(100);
    layout.set_element_size_in_bits(42);
    assert_eq!(layout.to_string(), "{3,2,1,0:T(42,123)(4,5)L(100)E(42)}");
  }

  #[test]
  fn test_equality() {
    assert_eq!(LayoutEqual::new().equal(
      &Layout::default(),
      &Layout::default()),
      true);
    assert_eq!(LayoutEqual::new().equal(
      &Layout::new_from_minor_to_major(vec![]),
      &Layout::new_from_minor_to_major(vec![])),
      true);
    assert_eq!(LayoutEqual::new().equal(
      &Layout::default(),
      &Layout::new_from_minor_to_major(vec![])),
      true);
    assert_eq!(LayoutEqual::new().equal(
      &Layout::new_from_minor_to_major(vec![0, 1, 2, 3]),
      &Layout::new_from_minor_to_major(vec![0, 1, 2, 3])),
      true);
    assert_eq!(LayoutEqual::new().equal(
      &Layout::new_from_minor_to_major(vec![0, 1, 2, 3]),
      &Layout::new_from_minor_to_major(vec![0, 1, 2])),
      false);

    let mut l1 = Layout::new_from(vec![0, 1, 2],
      vec![], vec![], vec![],
      vec![Tile::new(vec![42, 44])],
      1, PrimitiveType::Invalid,
      PrimitiveType::Invalid, 0, 0,
      None, 0);

    let mut l2 = Layout::new_from(vec![0, 1, 2],
      vec![], vec![], vec![],
      vec![Tile::new(vec![42, 44])],
      1, PrimitiveType::Invalid,
      PrimitiveType::Invalid, 0, 0,
      None, 0);
    assert_eq!(LayoutEqual::new().equal(&l1, &l2), true);

    l2 = Layout::new_from(vec![0, 1, 2],
      vec![], vec![], vec![],
      vec![Tile::new(vec![42, 45])],
      1, PrimitiveType::Invalid,
      PrimitiveType::Invalid, 0, 0,
      None, 0);
    assert_eq!(LayoutEqual::new().equal(&l1, &l2), false);
    assert_eq!(LayoutEqual::new().equal(&l1,
      &Layout::new_from_minor_to_major(vec![0, 1, 2, 3])),
      false);

    l1 = Layout::new_from_minor_to_major(vec![0, 1, 2]);
    l1.set_element_size_in_bits(33);
    l2 = Layout::new_from_minor_to_major(vec![0, 1, 2]);
    l2.set_element_size_in_bits(33);
    assert_eq!(LayoutEqual::new().equal(&l1, &l2), true);

    l2 = Layout::new_from_minor_to_major(vec![0, 1, 2]);
    l2.set_element_size_in_bits(7);
    assert_eq!(LayoutEqual::new().equal(&l1, &l2), false);

    l1 = Layout::new_from_minor_to_major(vec![0, 1, 2]);
    l1.set_memory_space(3);
    l2 = Layout::new_from_minor_to_major(vec![0, 1, 2]);
    l2.set_memory_space(3);
    assert_eq!(LayoutEqual::new().equal(&l1, &l2), true);

    l1 = Layout::new_from_minor_to_major(vec![0, 1, 2]);
    l1.set_memory_space(1);
    assert_eq!(LayoutEqual::new().equal(&l1, &l2), false);

    l1 = Layout::new_from(vec![0, 1, 2],
      vec![], vec![], vec![],
      vec![Tile::new(vec![42, 44])],
      1, PrimitiveType::Invalid,
      PrimitiveType::Invalid, 0, 0,
      None, 0);
    l2 = Layout::new_from_minor_to_major(vec![0, 1, 2]);
    assert_eq!(LayoutEqual::new().ignore_tiles().equal(&l1, &l2), true);

    l1 = Layout::new_from(vec![0, 1, 2],
      vec![], vec![], vec![],vec![],
      1, PrimitiveType::Invalid,
      PrimitiveType::Invalid, 32, 0,
      None, 0);

    l2 = Layout::new_from(vec![0, 1, 2],
      vec![], vec![], vec![], vec![],
      1, PrimitiveType::Invalid,
      PrimitiveType::Invalid, 1, 0,
      None, 0);
    assert_eq!(LayoutEqual::new().equal(&l1, &l2), false);

    l1 = Layout::new_from_minor_to_major(vec![0, 1, 2]);
    l1.set_element_size_in_bits(32);
    l2 = Layout::new_from_minor_to_major(vec![0, 1, 2]);
    l2.set_element_size_in_bits(1);
    assert_eq!(LayoutEqual::new().ignore_element_size().equal(&l1, &l2), true);

    l1 = Layout::new_from_minor_to_major(vec![0, 1, 2]);
    l1.set_memory_space(1);
    l2 = Layout::new_from_minor_to_major(vec![0, 1, 2]);
    l2.set_memory_space(3);
    assert_eq!(LayoutEqual::new().ignore_memory_space().equal(&l1, &l2), true);
  }

  #[test]
  fn test_delete_dimension_works_for_deleting_last_dim_from_dense_layout() {
    let mut layout = Layout::new_from_minor_to_major(vec![0, 1]);
    assert_eq!(layout.minor_to_major_size(), 2);

    layout.delete_dimension(1);
    assert_eq!(layout.minor_to_major_vec(), &vec![0]);
  }

  #[test]
  fn test_delete_dimension_works_for_deleting_non_last_dim_from_dense_layout() {
    let mut layout = Layout::new_from_minor_to_major(vec![1, 0]);
    assert_eq!(layout.minor_to_major_size(), 2);

    layout.delete_dimension(0);
    assert_eq!(layout.minor_to_major_vec(), &vec![0]);
  }
}