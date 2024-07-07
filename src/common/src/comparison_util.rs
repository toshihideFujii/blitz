#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonDirection {
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


pub fn string_to_comparison_direction(
  _direction: &String) -> Result<ComparisonDirection, String>
{
  unimplemented!()    
}

pub fn string_to_comparison_type(
  _comparison: &String) -> Result<ComparisonType, String>
{
  unimplemented!()    
}