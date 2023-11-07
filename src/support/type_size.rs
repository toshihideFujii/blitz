#![allow(dead_code)]

// This file provides a struct that can be used to query
// the size of IR types which may be scalable vectors.
// It provides convenience operators so that it can be used
// in much the same way as a single scalar value.

use std::ops::{MulAssign, Mul};
use std::cmp::Ordering;
use crate::support::math_extras;

pub fn report_invalid_size_request() {}

struct StackOffset {}

// Stores the number of elements for a type and whether this type
// is fixed (n-elements) or scalable (e.g., SVE).
// - ElementCount::get_fixed(1): A scalar value.
// - ElementCount::get_fixed(2): A vector type holding 2 values.
// - ElementCount::get_scalable(4): A scalable vector type holding 4 values.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementCount {
  quantity: usize,
  scalable: bool
}

impl ElementCount {
  pub fn new(min_val: usize, scalable: bool) -> Self {
    ElementCount { quantity: min_val, scalable: scalable }
  }

  pub fn get_fixed(min_val: usize) -> Self {
    ElementCount::new(min_val, false)
  }

  pub fn get_scalable(min_val: usize) -> Self {
    ElementCount::new(min_val, true)
  }

  pub fn get(min_val: usize, scalable: bool) -> Self {
    ElementCount::new(min_val, scalable)
  }

  // Exactly one element.
  pub fn is_scalar(&self) -> bool {
    !self.scalable && self.quantity == 1
  }

  // One or more element.
  pub fn is_vector(&self) -> bool {
    (self.scalable && self.quantity != 0) || self.quantity > 1
  }

  pub fn get_known_min_value(&self) -> usize {
    self.quantity
  }
}

// Stores the size of a type.
// If the type is of fixed size, it will represent the exact size.
// If the type is a scalable vector, it will represent the known minimum size.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeSize {
  quantity: usize,
  scalable: bool
}

impl TypeSize {
  pub fn new(quantity: usize, scalable: bool) -> Self {
    TypeSize { quantity: quantity, scalable: scalable }
  }

  pub fn is_zero(&self) -> bool {
    self.quantity == 0
  }

  pub fn is_non_zero(&self) -> bool {
    self.quantity != 0
  }

  pub fn get_with_increment() {}

  // Returns the minimum value this quantity can represent.
  pub fn get_known_min_value(&self) -> usize {
    self.quantity.clone()
  }

  // Returns whether the quantity is scaled by a runtime quantity (vscale).
  pub fn is_scalable(&self) -> bool {
    self.scalable
  }

  // A return value of true indicates we know at compile time that the number
  // of elements (vscale * min) is definitely even.
  // However, returning false does not guarantee that the total number of elements
  // is odd.
  pub fn is_known_even(&self) -> bool {
    (self.get_known_min_value() & 0x1) == 0
  }

  // This function tells the caller whether the element count is known at
  // comppile time to be a multiple of the scalar value rhs.
  pub fn is_known_mutiple_of(&self, rhs: usize) -> bool {
    self.get_known_min_value() % rhs == 0
  }

  // Return the minimum value with the assumption that the count is exact.
  pub fn get_fixed_value(&self) -> usize {
    debug_assert!(!self.is_scalable(), "Request for a fixed element count on a scalable object");
    self.get_known_min_value()
  }

  pub fn is_known_lt(lhs: &TypeSize, rhs: &TypeSize) -> bool {
    if !lhs.is_scalable() || rhs.is_scalable() {
      return lhs.get_known_min_value() < rhs.get_known_min_value();
    }
    false
  }

  pub fn is_known_gt(lhs: &TypeSize, rhs: &TypeSize) -> bool {
    if lhs.is_scalable() || !rhs.is_scalable() {
      return lhs.get_known_min_value() > rhs.get_known_min_value();
    }
    false
  }

  pub fn is_known_le(lhs: &TypeSize, rhs: &TypeSize) -> bool {
    if !lhs.is_scalable() || rhs.is_scalable() {
      return lhs.get_known_min_value() <= rhs.get_known_min_value();
    }
    false
  }

  pub fn is_known_ge(lhs: &TypeSize, rhs: &TypeSize) -> bool {
    if lhs.is_scalable() || !rhs.is_scalable() {
      return lhs.get_known_min_value() >= rhs.get_known_min_value();
    }
    false
  }

  pub fn divide_coefficient_by(&self, rhs: usize) -> TypeSize {
    TypeSize::get(self.get_known_min_value() / rhs, self.is_scalable())
  }

  pub fn multiply_coefficient_by(&self, rhs: usize) -> TypeSize {
    TypeSize::get(self.get_known_min_value() * rhs, self.is_scalable())
  }

  pub fn coefficient_next_power_of_2(&self) -> TypeSize {
    TypeSize::get(math_extras::next_power_of_2(
      self.get_known_min_value() as u64) as usize,
      self.is_scalable())
  }

  // Returns true if there exists a value x where rhs.multiply_coefficient_by(x)
  // will result in a value whose quantity matches our own.
  pub fn has_known_scalar_factor(&self, rhs: &TypeSize) -> bool {
    self.is_scalable() == rhs.is_scalable() &&
    self.get_known_min_value() % rhs.get_known_min_value() == 0
  }

  // Returns a value x where rhs.multiply_coefficient_by(x) will result in a
  // value whose quantity matches our own.
  pub fn get_known_scalar_factor(&self, rhs: &TypeSize) -> usize {
    debug_assert!(self.has_known_scalar_factor(rhs),
      "Expected rhs to be a known factor!");
    self.get_known_min_value() / rhs.get_known_min_value()
  }

  pub fn print() {}

  pub fn get_fixed(exact_size: usize) -> TypeSize {
    TypeSize::new(exact_size, false)
  }

  pub fn get_scalable(minimum_size: usize) -> TypeSize {
    TypeSize::new(minimum_size, true)
  }

  pub fn get(quantity: usize, scalable: bool) -> TypeSize {
    TypeSize::new(quantity, scalable)
  }

  pub fn fixed(exact_size: usize) -> TypeSize {
    TypeSize::new(exact_size, false)
  }

  pub fn scalable(minimum_size: usize) -> TypeSize {
    TypeSize::new(minimum_size, true)
  }
}

pub fn align_to(size: TypeSize, align: usize) -> TypeSize {
  TypeSize::new((size.get_known_min_value() + align - 1) / align * align,
    size.is_scalable())
}

impl Mul<usize> for TypeSize {
  type Output = TypeSize;
  fn mul(self, rhs: usize) -> Self::Output {
    TypeSize::new(self.quantity * rhs, self.scalable)
  }
}

impl MulAssign<usize> for TypeSize {
  fn mul_assign(&mut self, rhs: usize) {
    self.quantity *= rhs;
  }
}

impl PartialOrd<TypeSize> for TypeSize {
  fn partial_cmp(&self, other: &TypeSize) -> Option<Ordering> {
    if self.lt(other) {
      return Some(Ordering::Less);
    } else if self.gt(other) {
      return Some(Ordering::Greater);
    }
    Some(std::cmp::Ordering::Equal)
  }

  fn lt(&self, other: &TypeSize) -> bool {
    self.quantity < other.quantity
  }

  fn le(&self, other: &TypeSize) -> bool {
    self.quantity <= other.quantity
  }

  fn gt(&self, other: &TypeSize) -> bool {
    self.quantity > other.quantity
  }

  fn ge(&self, other: &TypeSize) -> bool {
    self.quantity >= other.quantity
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_type_size() {
    let ts_fixed_0 = TypeSize::fixed(0);
    let ts_fixed_1 = TypeSize::fixed(1);
    let ts_fixed_32 = TypeSize::fixed(32);

    assert_eq!(ts_fixed_0.get_fixed_value(), 0);
    assert_eq!(ts_fixed_1.get_fixed_value(), 1);
    assert_eq!(ts_fixed_32.get_fixed_value(), 32);
    assert_eq!(ts_fixed_32.get_known_min_value(), 32);

    assert_eq!(TypeSize::scalable(32).get_known_min_value(), 32);

    assert_eq!(ts_fixed_32 * 2, TypeSize::fixed(64));
    assert_eq!(align_to(TypeSize::fixed(7), 8), TypeSize::fixed(8));
  }
}