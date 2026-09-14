#![allow(dead_code)]

use std::collections::{HashSet};

use crate::shape::Shape;

// Shape pool provides a mechanism to deduplicate identical shapes and
// share them across multiple HLO instructions (and HLO modules).
pub struct ShapePool {
  canonical_shapes: HashSet<Shape>
}

impl ShapePool {
  // Returns a default shape pool that can be used across multiple HLO modules
  // in a process.
  pub fn default() -> Self {
    ShapePool { canonical_shapes: HashSet::new() }
  }

  // Returns a canonical shape from the pool. If the shape is not in the
  // pool, it is added to the pool and returned back.
  pub fn get_canonical_shape(&self, shape: &Shape) -> Option<&Shape> {
    if !self.canonical_shapes.contains(shape) {
      return None;
    }
    return self.canonical_shapes.get(shape);
  }

  pub fn set_canonical_shape(&mut self, shape: &Shape) {
    if self.canonical_shapes.contains(shape) {
      return;
    }
    self.canonical_shapes.insert(shape.clone());
  }

  // Runs garbage collection on all shapes in the pool. Returns the number
  // of shapes that were garbage collected.
  pub fn garbage_collect() -> usize {
    unimplemented!()
  }
}

#[cfg(test)]
mod tests {
use crate::shape_util::ShapeUtil;
use crate::blitz_data::PrimitiveType;
use super::*;

  #[test]
  fn test_get_canonical_shape() {
    let mut pool = ShapePool::default();
    let s0 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![1, 2]);
    let s1 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![2, 1]);

    pool.set_canonical_shape(&s0);
    pool.set_canonical_shape(&s1);
    let cs0_0 = pool.get_canonical_shape(&s0);
    let cs0_1 = pool.get_canonical_shape(&s0);
    assert_eq!(cs0_0, cs0_1);

    let cs1_0 = pool.get_canonical_shape(&s1);
    let cs1_1 = pool.get_canonical_shape(&s1);
    assert_ne!(cs0_0, cs1_0);
    assert_eq!(cs1_0, cs1_1);
  }
}