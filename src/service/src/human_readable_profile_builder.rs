#![allow(dead_code)]

struct OpInfo {
  name: String,
  short_name: String,
  category: String,
  cycles: i64,
  flop_count: i64,
  transcendental_count: i64,
  bytes_accessed: i64,
  optimal_seconds: f64
}

// HumanReadableProfileBuilder helps you create a textual profile of a
// computation, suitable for consumption by humans.
pub struct HumanReadableProfileBuilder {
  computation_name: String,
  is_entry_computation: bool,
  total_cycles: i64,
  clock_rate_ghz: f64,
  op_infos: Vec<OpInfo>
}

impl HumanReadableProfileBuilder {
  pub fn new(
    computation_name: String,
    is_entry_computation: bool,
    total_sycles: i64,
    clock_rate_ghz: f64) -> Self
  {
    HumanReadableProfileBuilder {
      computation_name: computation_name,
      is_entry_computation: is_entry_computation,
      total_cycles: total_sycles,
      clock_rate_ghz: clock_rate_ghz,
      op_infos: Vec::new()
    }
  }

  pub fn total_cycles(&self) -> i64 {
    self.total_cycles
  }

  pub fn add_op(
    &mut self,
    name: String, short_name: String, category: String, cycles: i64,
    flop_count: i64, transcendental_count: i64, bytes_accessed: i64,
    optimal_seconds: f64)
  {
    let op_info = OpInfo {
      name: name,
      short_name: short_name,
      category: category,
      cycles: cycles,
      flop_count: flop_count,
      transcendental_count: transcendental_count,
      bytes_accessed: bytes_accessed,
      optimal_seconds: optimal_seconds
    };
    self.op_infos.push(op_info);
  }

  pub fn to_string() {}

  fn cycles_to_seconds() {}
  fn cycles_to_microseconds() {}
}