#![allow(dead_code)]

use crate::hlo_value::{HloPosition, HloValue};

// A container which can hold one or more HloValues.
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