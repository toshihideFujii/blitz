#![allow(dead_code)]

use std::collections::HashMap;

use hlo::{hlo_computation::HloComputation, hlo_instruction::HloInstruction};

use crate::hlo_profile_printer_data::HloProfilePrinterData;

// Maps all HloInstructions and HloComputations in an HloModule to integers.
pub struct HloProfileIndexMap {
  instruction_to_profile_idx: HashMap<HloInstruction, i64>,
  computation_to_profile_idx: HashMap<HloComputation, i64>,
  extra_metric_to_profile_idx: HashMap<String, i64>
}

impl HloProfileIndexMap {
  pub fn new() {}

  pub fn get_profile_index_for_insstruction(
    &self, instruction: &HloInstruction) -> Option<&i64>
  {
    self.instruction_to_profile_idx.get(instruction)
  }

  pub fn get_profile_index_for_computation(
    &self, computation: &HloComputation) -> Option<&i64>
  {
    self.computation_to_profile_idx.get(computation)
  }

  pub fn get_profile_index_for_string(&self, key: &String) -> Option<&i64> {
    self.extra_metric_to_profile_idx.get(key)
  }

  pub fn instruction_count(&self) -> usize {
    self.instruction_to_profile_idx.len()
  }

  pub fn computation_count(&self) -> usize {
    self.computation_to_profile_idx.len()
  }

  pub fn extra_metrics_count(&self) -> usize {
    self.extra_metric_to_profile_idx.len()
  }

  pub fn total_count(&self) -> usize {
    self.instruction_count() + self.computation_count() + self.extra_metrics_count()
  }

  pub fn instruction_to_profile_idx(&self) -> &HashMap<HloInstruction, i64> {
    &self.instruction_to_profile_idx
  }

  pub fn computation_to_profile_idx(&self) -> &HashMap<HloComputation, i64> {
    &self.computation_to_profile_idx
  }

  pub fn extra_metric_to_profile_idx(&self) -> &HashMap<String, i64> {
    &self.extra_metric_to_profile_idx
  }
}

// Describes how much time each HLO operation took.
pub struct HloExecutionProfile {
  hlo_profile_printer_data: HloProfilePrinterData,
  hlo_profile_index_map: HloProfileIndexMap,
  profile_counters: Vec<u64>
}

impl HloExecutionProfile {
  pub fn new(
    hlo_profile_printer_data: HloProfilePrinterData,
    hlo_profile_index_map: HloProfileIndexMap) -> Self
  {
    HloExecutionProfile {
      hlo_profile_printer_data: hlo_profile_printer_data,
      hlo_profile_index_map: hlo_profile_index_map,
      profile_counters: Vec::new()
    }
  }

  // Record haw many cycles this HLO took to execute.
  pub fn set_cycles_taken_by(&mut self, hlo: &HloInstruction, cycles_taken: u64) {
    let index = *self.hlo_profile_index_map
      .get_profile_index_for_insstruction(hlo).unwrap() as usize;
    self.set_cycles_taken_by_index(index, cycles_taken)
  }

  // Record haw many cycles this HLO took to execute.
  pub fn set_cycles_taken_by_index(&mut self, index: usize, cycles_taken: u64) {
    self.profile_counters.insert(index, cycles_taken);
  }

  // Returns how many cycles this HLO took to execute.
  pub fn get_cycles_taken_by(&self, hlo: &HloInstruction) -> Option<&u64> {
    let index = *self.hlo_profile_index_map
      .get_profile_index_for_insstruction(hlo).unwrap() as usize;
    self.get_cycles_taken_by_index(index)
  }

  // Returns how many cycles this HLO took to execute.
  pub fn get_cycles_taken_by_index(&self, index: usize) -> Option<&u64> {
    self.profile_counters.get(index)
  }

  // Return the number of cycles this computation took to execute.
  pub fn total_cycles_executed(&self, computation: &HloComputation) -> Option<&u64> {
    let index = *self.hlo_profile_index_map
      .get_profile_index_for_computation(computation).unwrap() as usize;
    self.profile_counters.get(index)
  }

  // Record how many cycles a computation took to execute.
  pub fn set_total_cycles_executed(
    &mut self, computation: &HloComputation, total_cycles_executed: u64)
  {
    let index = *self.hlo_profile_index_map
      .get_profile_index_for_computation(computation).unwrap() as usize;
    self.profile_counters.insert(index, total_cycles_executed);
  }

  // Record extra metric.
  pub fn set_extra_metrics(&mut self, metric: &String, value: u64) {
    let index = *self.hlo_profile_index_map
      .get_profile_index_for_string(metric).unwrap() as usize;
    self.profile_counters.insert(index, value);
  }

  // Returns a version of the execution profile suitable for performance debugging.
  pub fn to_string(_clock_rate_ghz: f64) -> String {
    unimplemented!()
  }

  pub fn profile_counters(&self) -> &Vec<u64> {
    &self.profile_counters
  }

  pub fn mutable_profile_counters(&mut self) -> &mut Vec<u64> {
    &mut self.profile_counters
  }
}