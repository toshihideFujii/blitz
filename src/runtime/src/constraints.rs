#![allow(dead_code)]

pub const ARGUMENET_CONSTRAINT_ATTR_NAME: &str = "rt.constraint";

pub enum ArgumentConstraint {
  Resolved,
  Rank,
  Shape,
  Value,
}

pub fn parse_argument_constraint(str: &str) -> Result<ArgumentConstraint, &'static str> {
  if str == "rank" {
    return Ok(ArgumentConstraint::Rank);
  } else if str == "shape" {
    return Ok(ArgumentConstraint::Shape);
  } else if str == "value" {
    return Ok(ArgumentConstraint::Value);
  }
  Err("Unknown operand constraint.")
}

pub fn argument_constraint_to_string(constraint: ArgumentConstraint) -> &'static str {
  match constraint {
    ArgumentConstraint::Resolved => return "resolved",
    ArgumentConstraint::Rank => return "rank",
    ArgumentConstraint::Shape => return "shape",
    ArgumentConstraint::Value => return "value",
  }
}