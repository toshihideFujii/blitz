#![allow(dead_code)]

use common::{
  printer::{Printer, StringPrinter},
  shape::Shape,
  shape_layout::ShapeLayout
};

// Class which contains the layouts of the parameters and results of a
// computation.
pub struct ComputationLayout {
  parameter_layouts: Vec<ShapeLayout>,
  result_layout: ShapeLayout
}

impl ComputationLayout {
  pub fn new(result_layout: ShapeLayout) -> Self {
    ComputationLayout {
      parameter_layouts: Vec::new(),
      result_layout: result_layout
    }
  }

  // Adds a new parameter layout to the computation layout.
  pub fn add_parameter_layout(&mut self, shape_layout: ShapeLayout) {
    self.parameter_layouts.push(shape_layout);
  }

  // Returns the layout of a particular parameter.
  pub fn parameter_layout(&self, param_no: usize) -> &ShapeLayout {
    &self.parameter_layouts[param_no]
  }

  pub fn mutable_parameter_layout(&mut self, param_no: usize) -> &mut ShapeLayout {
    &mut self.parameter_layouts[param_no]
  }

  // Returns the number of parameters in the computation.
  pub fn parameter_count(&self) -> usize {
    self.parameter_layouts.len()
  }

  // Returns the ShapeLayouts of a result of the computation.
  pub fn parameter_layouts(&self) -> &Vec<ShapeLayout> {
    &self.parameter_layouts
  }

  // Returns the ShapeLayout of a result of the computation.
  pub fn result_layout(&self) -> &ShapeLayout {
    &self.result_layout
  }

  pub fn mutable_result_layout(&mut self) -> &mut ShapeLayout {
    &mut self.result_layout
  }

  // Returns the shape of the particular parameter or result of the computation
  // with layout.
  pub fn parameter_shape(&self, param_no: usize) -> &Shape {
    self.parameter_layouts[param_no].shape()
  }

  pub fn result_shape(&self) -> &Shape {
    self.result_layout.shape()
  }

  // Sets layouts of all parameters and the result to the default layout.
  pub fn set_to_default_layout(&mut self) {
    for parameter_layout in &mut self.parameter_layouts {
      parameter_layout.set_to_default_layout();
    }
    self.result_layout.set_to_default_layout();
  }

  // Returns true if all layouts (parameters and result) have been set.
  pub fn layout_is_set(&self) -> bool {
    for parameter_layout in &self.parameter_layouts {
      if !parameter_layout.layout_is_set() { return false; }
    }
    self.result_layout.layout_is_set()
  }

  // Returns true if any layouts (parameters and result) have been set.
  pub fn any_layout_set(&self) -> bool {
    for parameter_layout in &self.parameter_layouts {
      if parameter_layout.layout_is_set() { return false; }
    }
    self.result_layout.layout_is_set()
  }

  pub fn flattened_parameter_layouts() {}
  pub fn flattened_result_layouts() {}

  // Prints a string representation of this object.
  pub fn print(&self, printer: &mut dyn Printer) {
    printer.append(&"(".to_string());
    if !self.parameter_layouts.is_empty() {
      self.parameter_layouts[0].print(printer);
      for i in 1..self.parameter_layouts.len() {
        if i % 5 == 0 {
          let mut str = ", /*index=".to_string();
          str.push_str(i.to_string().as_str());
          str.push_str("*/");
          printer.append(&str);
        } else {
          printer.append(&", ".to_string());
        }
        self.parameter_layouts[i].print(printer);
      }
    }
    printer.append(&")->".to_string());
    self.result_layout.print(printer);
  }

  // Returns a sstring representation of this object.
  pub fn to_string(&self) -> String {
    let mut printer = StringPrinter::new();
    self.print(&mut printer);
    printer.to_string()
  }

  pub fn compute_program_shape() {} 
}