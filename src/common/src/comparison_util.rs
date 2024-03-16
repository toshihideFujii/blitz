#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
  Eq,
  Ne,
  Ge,
  Gt,
  Le,
  Lt,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonType {
  Float,
  FloatTotalOrder,
  Signed,
  Unsigned,
}