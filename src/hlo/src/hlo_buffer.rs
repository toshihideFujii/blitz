#![allow(dead_code)]

use crate::hlo_value::{HloPosition, HloValue};

// A container which can hold one or more HloValues. An HLO buffer abstractly
// represents the allocation which HLO instructions write into and read
// from. Generally there is a one-to-one correspondence between HloBuffers and
// HloValue where each HloValue in the module is held in a unique HloBuffer. An
// exception is the while instruction which updates the loop state in-place. In
// this case, we have a single HloBuffer for each HloPosition in the loop state,
// but multiple HloValues. For example:
//
//   %init = ...
//   %while = While(%init, body, condition)
//
//  body:
//   %body_param = Param(0)
//     ...
//   %body_root = ...
//
//  condition:
//   %cond_param = Param(0)
//     ...
//
// For simplicity, assume that %while is array-shaped. In this case, we have a
// single HloBuffer which holds the following HloValues: HloValue{%init},
// HloValue{%while}, HloValue{%body_param}, HloValue{%body_root}, and
// HloValue{%cond_param}.
//
// HloBuffers may appear at different HloPositions in the module mirroring the
// same property of HloValues. For example:
//
//   %sub = Sub(...)
//   %add = Add(...)
//   %tuple = Tuple(%add, %sub)
//   %gte = GetTupleElement(%tuple, 0)
//
// In this case, the HloBuffer containing %add appears at the following
// positions: HloPosition{%add, {}}, HloPosition{%tuple, {0}}, and
// HloPosition{%gte, {}}.
//
// Different HloPositions which share the same HloBuffer indicate mandatory
// aliasing in the HLO module. These positions must share the same memory
// allocation for correctness (the backends rely on this property). This differs
// from incidental aliasing introduced by memory reuse in BufferAssignment where
// different instructions may happen to get the same allocation.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HloBuffer {
  id: i64,
  values: Vec<HloValue>
}

impl HloBuffer {
  pub fn new(id: i64, values: Vec<HloValue>) -> Self {
    HloBuffer { id: id, values: values }
  }

  // Predicate comparing HloBuffers by increasing id.
  pub fn id_less_than(a: &HloBuffer, b: &HloBuffer) -> bool {
    a.id() < b.id()
  }

  // Return the unique identifier for this HloBuffer.
  pub fn id(&self) -> i64 {
    self.id
  }
  
  // Return all values contained in this buffer.
  pub fn values(&self) -> &Vec<HloValue> {
    &self.values
  }

  // Memory space color.
  // Used to indicate the memory space that the hlo beffer needs to live in.
  pub fn color(&self) -> i64 {
    let result = self.values[0].color();
    for value in self.values() {
      debug_assert!(result == value.color());
    }
    result
  }

  // Return the unique HLO value in the buffer.
  pub fn get_unique_value(&self) -> &HloValue {
    debug_assert!(self.values.len() == 1);
    &self.values[0]
  }

  pub fn compute_positions(&self) -> Vec<HloPosition> {
    let mut positions = vec![];
    for value in self.values() {
      for pos in value.positions() {
        if !positions.contains(pos) {
          positions.push(pos.clone());
        }
      }
    }
    positions.sort();
    positions
  }

  pub fn to_string(&self) -> String {
    let mut out = "HloBuffer ".to_string();
    out.push_str(&self.id.to_string());
    out.push_str(", values: ");

    for value in self.values() {
      out.push_str(&value.to_short_string());
    }
    out
  }
}