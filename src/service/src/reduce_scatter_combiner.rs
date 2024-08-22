#![allow(dead_code)]

use std::collections::HashSet;

use hlo::{
  hlo_instruction::HloInstruction,
  hlo_module::HloModule,
  hlo_opcode::HloOpcode,
  utils::hlo_query
};

use crate::{
  //all_reduce_key::{get_all_reduce_key, AllReduceKey},
  all_reduce_key::AllReduceKey,
  //collective_combiner_utils::combine_instructions_by_key,
  //collective_ops_utils::match_reduction_computation,
  //hlo_domain_map::HloDomainMap
};

// Combines small non-dependent ReduceScatter ops into larger combined
// ReduceScatter ops. A typical ReduceScatter implementation has a minimum
// latency-induced time for a ReduceScatter op so a single combined op can be
// more efficient than many small ones.
pub struct ReduceScatterCombiner {
  combine_threshold_in_bytes: i64,
  combine_threshold_count: i64,
  combine_by_dim: bool
}

impl ReduceScatterCombiner {
  pub fn new(
    combine_threshold_in_bytes: i64,
    combine_threshold_count: i64,
    combine_by_dim: bool) -> Self
  {
    ReduceScatterCombiner {
      combine_threshold_in_bytes: combine_threshold_in_bytes,
      combine_threshold_count: combine_threshold_count,
      combine_by_dim: combine_by_dim
    }
  }

  pub fn name(&self) -> String {
    "reduce-scatter-combiner".to_string()
  }

  pub fn run(
    &mut self,
    module: &HloModule,
    _execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    println!("Running ReduceScatterCombiner with threshold of {:?} bytes.",
      self.combine_threshold_in_bytes);

    if self.combine_threshold_in_bytes <= 0 || self.combine_threshold_count <= 0 {
      println!("Skip ReduceScatterCombiner because the threshold is zero.");
      return Ok(false);
    }

    if hlo_query::contains_layout_constrained_collective(
      module, HloOpcode::ReduceScatter)
    {
      println!("Skip ReduceScatterCombiner because the module contains reduce-scatter
        with constrained layouts.");
      return Ok(false);
    }
    /*
    let mut changed = false;
    for computation in
      module.make_nonfusion_computations_by_exec_threads(execution_threads)
    {
      let domain_map = HloDomainMap::new(computation, "".to_string());
      let key_fn =
        |instruction: &HloInstruction| -> Option<ReduceScatterKey>
      {
        let key =
          get_all_reduce_key(instruction, &domain_map, false);
        if key.is_none() { return None; }

        if match_reduction_computation(instruction.to_apply()).is_none() {
          return None;
        }

        let mut rs_dim_key = -1;
        if self.combine_by_dim {
          rs_dim_key = instruction.scatter_dimension();
        }

        let reduce_scatter_key: ReduceScatterKey = (key.unwrap(), rs_dim_key);
        Some(reduce_scatter_key)
      };

      let computation_changed =
        combine_instructions_by_key::<ReduceScatterKey>(
          computation,
          Box::new(key_fn),
          Box::new(combine_reduce_scatters),
          self.combine_threshold_in_bytes,
          self.combine_threshold_count);
    
      if computation_changed.is_err() {
        return Err(computation_changed.err().unwrap());
      }

      changed |= computation_changed.ok().unwrap();
    }

    Ok(changed)
    */

    unimplemented!()
  }
}

pub type ReduceScatterKey = (AllReduceKey, i64);

// Returns the most frequent scatter dim if it can be a valid scatter dim
// for all shapes involved, else returns 0.
fn find_most_frequent_scatter_dim(to_combine: &Vec<HloInstruction>) -> i64 {
  assert!(!to_combine.is_empty());

  let mut min_rank = i64::MAX;
  let mut frequency = vec![];
  for instruction in to_combine {
    let dim = instruction.scatter_dimension();
    frequency.resize((dim+1).max(frequency.len() as i64) as usize, 0);
    frequency[dim as usize] += 1;
    min_rank = min_rank.max(instruction.shape().rank() as i64);
  }

  let mut max_element = 0;
  let mut most_frequent_dim = 0;
  for i in 0..frequency.len() {
    if max_element < frequency[i] {
      max_element = frequency[i];
      most_frequent_dim = i;
    }
  }
  
  if most_frequent_dim < min_rank as usize { most_frequent_dim as i64 } else { 0 }
}

fn combine_reduce_scatters(_to_combine: &Vec<HloInstruction>) -> Result<(), String> {
  unimplemented!()
}