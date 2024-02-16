#![allow(dead_code)]

use std::cmp::max;

use crate::{
  blitz_data::{DimLevelType, PrimitiveType},
  util::DimensionVector, shape::Shape, printer::{Printer, StringPrinter}, primitive_util, layout_util::LayoutUtil
};

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
      tail_padding_alignment_in_elements: 1,
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
      tail_padding_alignment_in_elements: tail_padding_alignment_in_elements
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
    if !self.tiles.is_empty() {
      print_colon(printer);
      printer.append(&"T".to_string());
      for tile in &self.tiles {
        tile.print(printer);
      }
    }
    if self.tail_padding_alignment_in_elements() != 1 {
      print_colon(printer);
      printer.append(&"L(".to_string());
      printer.append(&self.tail_padding_alignment_in_elements.to_string());
      printer.append(&")".to_string());
    }
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
    if self.pointer_primitive_type != PrimitiveType::Invalid {
      print_colon(printer);
      if primitive_util::is_integral_type(&self.pointer_primitive_type) {
        printer.append(&"*(".to_string());
        let primitive_type_name =
          primitive_util::lowercase_primitive_type_name(&self.index_primitive_type);
        printer.append(&primitive_type_name);
        printer.append(&")".to_string());
      } else {
        printer.append(&"*(invalid".to_string());
      }
    }
    if self.element_size_in_bits != 0 {
      print_colon(printer);
      printer.append(&"E(".to_string());
      printer.append(&self.element_size_in_bits.to_string());
      printer.append(&")".to_string());
    }
    if self.memory_space != 0 {
      print_colon(printer);
      printer.append(&"S(".to_string());
      printer.append(&self.memory_space.to_string());
      printer.append(&")".to_string());
    }
    if self.has_physical_shape() {
      print_colon(printer);
      printer.append(&"P{".to_string());
      printer.append(&self.dynamic_shape_metadata_prefix_bytes.to_string());
      printer.append(&")".to_string());
    }

    printer.append(&"}".to_string())
  }

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_to_string() {
    assert_eq!(Layout::new().to_string(), "{}");
    assert_eq!(Layout::new_from_minor_to_major(vec![4, 5, 6]).to_string(), "{4,5,6}");

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

    let mut layout1 = layout.clone();
    layout1.set_tail_padding_alignment_in_elements(100);
    layout1.set_element_size_in_bits(42);
    assert_eq!(layout1.to_string(), "{3,2,1,0:T(42,123)(4,5)L(100)E(42)}");

    let mut layout2 = layout.clone();
    layout2.set_memory_space(3);
    assert_eq!(layout2.to_string(), "{3,2,1,0:T(42,123)(4,5)S(3)}");
  }

  #[test]
  fn test_equality() {
    assert_eq!(LayoutEqual::new().equal(
      &Layout::new(),
      &Layout::new()),
      true);
    assert_eq!(LayoutEqual::new().equal(
      &Layout::new_from_minor_to_major(vec![]),
      &Layout::new_from_minor_to_major(vec![])),
      true);
    assert_eq!(LayoutEqual::new().equal(
      &Layout::new(),
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
}