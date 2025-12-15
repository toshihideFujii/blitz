#![allow(dead_code)]

use std::{collections::{HashMap, HashSet}};

use common::{shape::Shape, shape_util::ShapeUtil};

use crate::{analysis::{alias_info::AliasInfo, hlo_dataflow_analysis::HloDataflowAnalysis},
  hlo_buffer::HloBuffer, hlo_instruction::HloInstruction, hlo_module::HloModule,
  hlo_value::{HloValue}};

// analysis which allocates HloBuffers to HloValues.
pub struct HloAliasAnalysis<'module> {
  module: &'module HloModule,
  live_out_buffers: HashSet<HloBuffer>,
  dataflow_analysis: HloDataflowAnalysis<'module>,
  value_to_buffer: HashMap<&'module HloValue, HloBuffer>,
  buffers: Vec<HloBuffer>,
}

impl<'module> HloAliasAnalysis<'module> {
  pub fn new(module: &'module HloModule) -> Self {
    HloAliasAnalysis {
      module: module,
      live_out_buffers: HashSet::new(),
      dataflow_analysis: HloDataflowAnalysis::new(
        module,
        false,
        false,
        HashSet::new()),
      value_to_buffer: HashMap::new(),
      buffers: Vec::new(),
    }
  }

  // The callgraph of the given HloModule must be flattened prior to running
  // the analysis.
  pub fn run(
    &self,
    module: &'module HloModule,
    alias_info: &AliasInfo) -> Result<HloAliasAnalysis<'module>, String>
  {
    println!("HloAliasAnalysis::run on module {:?}", module.name());
    let mut alias_analysis = HloAliasAnalysis::new(module);

    let result = HloDataflowAnalysis::run(
      module, true, false, HashSet::new());
    alias_analysis.dataflow_analysis = result.unwrap();
    alias_analysis.buffers = create_buffers(
      &alias_analysis.dataflow_analysis(), alias_info);
    let num_values = alias_analysis.dataflow_analysis.values().len();

    for buffer in alias_analysis.mutable_buffers() {
      for _value in buffer.values() {
        //alias_analysis.value_to_buffer.insert(value, buffer.clone());
      }
    }

    debug_assert!(alias_analysis.value_to_buffer.len() == num_values);
    let result = alias_analysis.verify();
    check_error(&result);

    let root = module.entry_computation()
      .as_ref().unwrap().root_instruction();

    let mut func = |_shape: &Shape, index: &Vec<i64>| {
      let buffers = alias_analysis
        .compute_buffers_at(root, index);
      for buffer in buffers {
        alias_analysis.live_out_buffers.insert(buffer.clone());
      }
    };
    ShapeUtil::for_each_subshape(root.shape(), &mut func);

    println!("{:?}", alias_analysis.to_string());
    Ok(alias_analysis)
  }

  pub fn to_string(&self) -> String {
    let mut out = "HloAliasAnalysis, module ".to_string();    
    out.push_str(&self.module.name());
    out.push_str("\n");
    out.push_str("  Buffers at each position:\n");

    let func =
      |_module: &HloModule| -> Result<(Vec<Shape>, Shape), String> {
      unimplemented!()
    };
    for comp in self.module.computations(func) {
      for inst in comp.instructions() {
        out.push_str("    ");
        out.push_str(&inst.name());
        out.push_str("\n");
        if inst.shape().is_tuple() {
          let mut tuple_func =
            |_shape: &Shape, index: &Vec<i64>| {
            out.push_str("      tupl index ");
            //out.push_str(index.to_string());
            for buffer in
              self.compute_buffers_at(inst, index) {
              out.push_str("      ");
              out.push_str(&buffer.to_string());
              out.push_str("\n");
            }
            unimplemented!()
          };
          ShapeUtil::for_each_subshape(inst.shape(), &mut tuple_func);
        } else {
          for buffer in
            self.compute_buffers_at(inst, &vec![]) {
            out.push_str("      ");
            out.push_str(&buffer.to_string());
            out.push_str("\n");  
          }
        }
      }
    }
    out.push_str("  Buffers:\n");
    for buffer in self.buffers() {
      out.push_str(&buffer.to_string());
      out.push_str("      positions:\n");
      for pos in &buffer.compute_positions() {
        out.push_str("        ");
        out.push_str(&pos.to_string());
      }
    }
    out
  }

  // Return the buffer containing the given value.
  pub fn get_buffer_containing_value(
    &self, value: &HloValue) -> Option<&HloBuffer> {
    self.value_to_buffer.get(value)
  }

  // Return the HloBuffer with the given id.
  pub fn get_buffer(&self, id: i64) -> &HloBuffer {
    &self.buffers[id as usize]
  }

  // Returns the unique buffer at the given position. CHECK fails if the buffer
  // set at that position does not contain exactly one buffer.
  pub fn get_unique_buffer_at(
    &self,
    instruction: &HloInstruction,
    index: &Vec<i64>) -> HloBuffer
  {
    let buffers = self.compute_buffers_at(instruction, index);
    debug_assert!(buffers.len() == 1);
    buffers[0].clone()
  }

  // Compute the set of buffers at the given instruction and index and
  // return as a vector.
  pub fn compute_buffers_at(
    &self,
    instruction: &HloInstruction,
    index: &Vec<i64>) -> Vec<HloBuffer>
  {
    let value_set =
      self.dataflow_analysis.get_value_set(instruction, index);
    let mut buffers = vec![];
    for value in value_set.values() {
      buffers.push(self.get_buffer_containing_value(value).unwrap().clone());
    }
    // Sort and uniquify vector before returning.
    // TODO
    buffers
  }

  // Return a vector of all HloBuffers stabily sorted by HloBuffer::id.
  pub fn buffers(&self) -> &Vec<HloBuffer> {
    &self.buffers
  }

  pub fn mutable_buffers(&mut self) -> &mut Vec<HloBuffer> {
    &mut self.buffers
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

  pub fn lives_out_buffers(&self) -> Vec<HloBuffer> {
    let mut result = vec![];
    let mut ids_vec = vec![];
    for buffer in &self.live_out_buffers {
      ids_vec.push(buffer.id());
    }
    ids_vec.sort();
    for id in ids_vec {
      for buffer in &self.live_out_buffers {
        if id == buffer.id() {
          result.push(buffer.clone());
        }
      }
    }
    result
  }

  // Verify various invariants of the alias analysis.
  fn verify(&self) -> Result<(), String> {
    // Verify consistency between the value_to_buffer_ map and
    // HloBuffer::values().
    for pair in &self.value_to_buffer {
      let value = *pair.0;
      let buffer = pair.1;
      for v in buffer.values() {
        debug_assert!(value == v);
      }
    }
    for id in 0..self.buffers().len() {
      let buffer = &self.buffers[id];
      debug_assert!(buffer.id() == id as i64);

      let mut last_value_id = -1;
      for value in buffer.values() {
        debug_assert!(Some(buffer) == self.get_buffer_containing_value(value));
        // Also verify the values in HloBuffer are unique and sorted by id.
        debug_assert!(value.id() > last_value_id);
        last_value_id = value.id();
      }
    }
    Ok(())
  }
}

fn create_buffers(
  dataflow: &HloDataflowAnalysis,
  _alias_info: &AliasInfo) -> Vec<HloBuffer>
{
  let values = dataflow.values();
  let num_buffers = values.len();

  let mut buffer_values: Vec<HashSet<HloValue>> = vec![];
  let _value_to_set: HashMap<&HloValue, HashSet<HloValue>> = HashMap::new();
  for i in 0..values.len() {
    buffer_values[i].insert(values[i].clone());
    //value_to_set[&values[i]] = buffer_values[i];
  }

  // Merge together sets of HloValues which must be in the same HloBuffer
  // because of aliasing rules (e.g. in-place kWhile instruction).
  for value in values {
    println!("Merging colocated values, value: {:?}", value);
    
  }

  // Create a vector of HloBuffers, one for each non-empty set of values.
  let buffers: Vec<HloBuffer> = vec![];
  for value_set in &buffer_values {
    if !value_set.is_empty() {
      let _id = buffers.len();
      //let vset = HloValueSet::new(values)
    }
  }

  debug_assert_eq!(buffers.len(), num_buffers);
  buffers
}

fn check_error<T>(value: &Result<T, String>) {
  if value.is_err() {
    let err_msg = value.as_ref().err().unwrap();
    assert!(false, "{:?}", err_msg);
  }
}