#![allow(dead_code)]

// Library for comparing literals without taking a dependency on testing
// libraries.

use crate::shape::Shape;

// Returns ok if the given shapes have the same rank, dimension sizes, and
// primitive types.
pub fn equal_shapes(_expected: &Shape, _actual: &Shape) {
  unimplemented!()
}

// Returns ok if the given literals share identical dynamic shapes and
// dimension sizes.
pub fn equal_dynamicshapes_and_dimensions() {
  unimplemented!()
}