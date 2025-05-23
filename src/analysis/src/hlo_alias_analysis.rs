#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use crate::hlo_dataflow_analysis::HloDataflowAnalysis;

use hlo::{
  hlo_buffer::HloBuffer,
  hlo_instruction::HloInstruction,
  hlo_module::HloModule,
  hlo_value::HloValue
};

// analysis which allocates HloBuffers to HloValues.
pub struct HloAliasAnalysis<'module> {
  module: &'module HloModule,
  live_out_buffers: HashSet<HloBuffer>,
  dataflow_analysis: HloDataflowAnalysis<'module>,
  value_to_buffer: HashMap<HloValue, HloBuffer>,
  buffers: Vec<HloBuffer>,
}

impl<'module> HloAliasAnalysis<'module> {
  pub fn new(_module: HloModule) -> Self {
    /*
    HloAliasAnalysis {
      module: module,
      live_out_buffers: HashSet::new(),
      dataflow_analysis: HloDataflowAnalysis::new(module, false, false, HashSet::new()),
      value_to_buffer: HashMap::new(),
      buffers: Vec::new(),
    }
    */
    unimplemented!()
  }

  // The callgraph of the given HloModule must be flattened prior to running
  // the analysis.
  pub fn run(&self, _module: &HloModule) {}

  pub fn to_string(&self) -> String {
    unimplemented!()
  }

  // Return the buffer containing the given value.
  pub fn get_buffer_containing_value(&self, value: &HloValue) -> Option<&HloBuffer> {
    self.value_to_buffer.get(value)
  }

  // Return the HloBuffer with the given id.
  pub fn get_buffer(&self, id: i64) -> &HloBuffer {
    &self.buffers[id as usize]
  }

  // Returns the unique buffer at the given position.
  pub fn get_unique_buffer_at(
    &self,
    _instruction: &HloInstruction,
    _index: usize) -> &HloBuffer
  {
    unimplemented!()
  }

  // Compute the set of buffers at the given instruction and index and
  // return as a vector.
  pub fn compute_buffers_at(
    &self,
    _instruction: &HloInstruction,
    _index: usize) -> Vec<HloBuffer>
  {
    unimplemented!()
  }

  // Return a vector of all HloBuffers stabily sorted by HloBuffer::id.
  pub fn buffers(&self) -> &Vec<HloBuffer> {
    &self.buffers
  }

  // Returns the underlying dataflow analysis used by this alias analysis.
  pub fn dataflow_analysis(&self) -> &HloDataflowAnalysis {
    &self.dataflow_analysis
  }

  // Returns true if a buffe out of the module.
  pub fn buffer_lives_out(&self, buffer: &HloBuffer) -> bool {
    self.live_out_buffers.contains(buffer)
  }

  // Returns true if a hlo value lives out of the module.
  pub fn value_lives_out(&self, value: &HloValue) -> bool {
    let buffer = self.get_buffer_containing_value(value);
    debug_assert!(buffer.is_some());
    self.live_out_buffers.contains(buffer.unwrap())
  }

  pub fn lives_out_buffers() {}

  fn verify() {}
}