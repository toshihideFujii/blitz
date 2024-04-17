#![allow(dead_code)]

use crate::{layout::Layout, layout_util::LayoutUtil, printer::Printer, shape::{Shape, ShapeEqual}, shape_util::ShapeUtil};

// A shapeLayout object encapsulates the layout of a particular shape
// (including tuples). This differs from the Layout proto which describes
// the layout of a single array.
pub struct ShapeLayout {
  shape: Shape
}

impl ShapeLayout {
  pub fn new(shape: Shape) -> Self {
    ShapeLayout { shape: shape }
  }

  // Assigns the layouts in this ShapeLayout to the layout fields of the shape.
  pub fn assign_layout_to_shape(&self, mut to_shape: Shape) -> Shape {
    if !ShapeUtil::compatible(&to_shape, &self.shape) {
      assert!(false, "Shape is not compatible.");
    }
    to_shape = self.shape.clone();
    to_shape
  }

  // Returns true if the layouts in this ShapeLayout match the layouts in the
  // given shape.
  pub fn matches_layout_in_shape(
    &self,
    shape: &Shape,
    minor_to_major_only: bool,
    ignore_fully_empty_tiling: bool) -> bool
  {
    let mut equal = ShapeEqual::new();
    equal.ignore_dynamic_dimension();
    if ignore_fully_empty_tiling {
      let mut fully_empty_tiling = true;
      let mut check_tiling = |subshape: &Shape, _index: usize| {
        if !fully_empty_tiling {
          return;
        }
        if subshape.is_array() && !subshape.layout().as_ref().unwrap().tiles_vec().is_empty() {
          fully_empty_tiling = false;
        }
      };
      ShapeUtil::for_each_mutable_subshape(shape, &mut check_tiling);
      /* TODO
      if fully_empty_tiling {
        equal.minor_to_major_only_in_layout();
      } else {
        fully_empty_tiling = true;
        // Check the other shape.
        ShapeUtil::for_each_mutable_subshape(&self.shape, &mut check_tiling);
        if fully_empty_tiling {
          equal.minor_to_major_only_in_layout();
        }
      }
      */
    }
    if minor_to_major_only {
      equal.minor_to_major_only_in_layout();
    }
    equal.equal(shape, &self.shape)
  }

  // Copies the layout from the given shape into this ShapeLayout.
  pub fn copy_layout_from_shape(&mut self, other_shape: &Shape) {
    if !ShapeUtil::compatible(other_shape, &self.shape) {
      assert!(false, "Shape is not compatible.");
    }
    self.shape = other_shape.clone();
  }

  // Clears all the Layouts stored in this object.
  pub fn clear(&mut self) {
    LayoutUtil::clear_layout(&mut self.shape);
  }

  // Sets all layouts stored in this object to the default layout.
  pub fn set_to_default_layout(&mut self) {
    LayoutUtil::set_to_default_layout(&mut self.shape);
  }

  // Returns the shape (with layouts).
  pub fn shape(&self) -> &Shape {
    &self.shape
  }

  // Clear dynamic dimensions of this module.
  pub fn clear_dynamic_shape(&mut self) {
    self.shape.clear_dynamic_dimensions();
  }

  // Checks that a layout is set for the shape, and return a reference to the
  // layout directly on the shape.
  pub fn layout(&self) -> &Option<Layout> {
    debug_assert!(self.layout_is_set());
    debug_assert!(!self.shape.is_tuple());
    self.shape.layout()
  }

  // Returns true if all layouts have been set for this shapeLayout object.
  pub fn layout_is_set(&self) -> bool {
    LayoutUtil::has_layout(&self.shape)
  }

  // Resets the layout on the shape to the provided layout.
  pub fn reset_layout(&mut self, layout: Layout) {
    debug_assert!(!self.shape.is_tuple());
    debug_assert!(!self.shape.is_opaque());
    self.shape.set_layout(layout);
    debug_assert!(ShapeUtil::validate_shape(&self.shape).is_ok());
  }

  // Returns a string representation of this object.
  pub fn print(&self, printer: &mut dyn Printer) {
    self.shape.print(printer, true);
  }

  // Returns a string representation of this object.
  pub fn to_string(&self) -> String {
    self.shape.to_string(true)
  }
}