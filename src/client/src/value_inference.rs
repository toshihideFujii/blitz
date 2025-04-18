#![allow(dead_code)]

use common::literal::Literal;

// OptionalLiteral is an augmented literal class which returns optional
// values for each index (the value can be either valid or invalid). The
// implementation keeps two literals, a value literal, holding both the valid
// and garabage value, and a masking literal representing if a value is valid or
// garbage.
pub struct OptionalLiteral<T> where T: Clone + Default + PartialEq + 'static {
  value: Literal<T>,
  mask: Literal<T>,
}

impl<T> OptionalLiteral<T> where T: Clone + Default + PartialEq {
  pub fn new(value: Literal<T>, mask: Literal<T>) -> Self {
    OptionalLiteral { value: value, mask: mask }
  }

  pub fn get(&self, _element_index: &Vec<i64>) {
      
  }

  // Returns true if all values in this literal slice are value.
  pub fn all_valid(&self) -> bool where T: PartialEq {
    self.mask.is_all_int(0)
  }

  pub fn get_value() {
      
  }
}

pub enum ValueInferenceMode {
  Value,
  UpperBound,
  LowerBound,
}

pub struct ValueInference {}

impl ValueInference {
    
}