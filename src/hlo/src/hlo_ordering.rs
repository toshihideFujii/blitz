#![allow(dead_code)]

#[derive(Clone, PartialEq)]
pub enum ExecutionConstraint {
  IsSame,
  RunBeforeStart,
  RunBeforeEnd,
  RunExclusiveBefore,
  RunExclusiveAfter,
  RunAfter,
  Unordered,
}

pub trait HloOrdering {
  fn get_execution_constraint();
  fn executes_before();
  fn is_defined_before();
  fn uses_before_value_definition();
  fn may_interfere();
  fn live_range_strictly_before();
  fn sequential_order();
  fn call_graph();
  fn to_string();
  fn executes_before_in_same_computation();
}

pub struct PredecessorHloOrdering {}

pub struct DependencyHloOrdering {}

pub struct SequentialHloOrdering {}