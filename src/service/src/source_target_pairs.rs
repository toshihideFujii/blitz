#![allow(dead_code)]

use hlo::hlo_instruction::HloInstruction;

use crate::hlo_parser::parse_replica_groups_only;

#[derive(Debug, Clone, PartialEq)]
pub struct SourceTargetPair {
  source: i64,
  target: i64
}

impl SourceTargetPair {
  pub fn default() -> Self {
    SourceTargetPair { source: 0, target: 0 }
  }

  pub fn new(source: i64, target: i64) -> Self {
    SourceTargetPair { source: source, target: target }
  }
}

// SourceTargetPairs represents a list of (source, target)
// pairs used in a collective permute instruction
// e.g. {{0,1},{1,2},{2,3},{3,0}}.
#[derive(Debug, Clone, PartialEq)]
pub struct SourceTargetPairs {
  pairs: Vec<(SourceTargetPair, i64)>
}

impl SourceTargetPairs {
  pub fn default() -> Self {
    SourceTargetPairs { pairs: Vec::new() }
  }

  pub fn from_string(str: &String) -> Result<SourceTargetPairs, String> {
    let groups_wrapper =
      parse_replica_groups_only(str);
    if groups_wrapper.is_err() {
      return Err(groups_wrapper.err().unwrap());
    }
    let mut res = SourceTargetPairs::default();
    let groups = groups_wrapper.unwrap();
    for g in &groups {
      if g.replica_ids().len() != 2 {
        let mut err_msg = "Incorrect element size: ".to_string();
        err_msg.push_str(str);
        return Err(err_msg);
      }
      res.emplace_back(g.replica_ids()[0], g.replica_ids()[1]);
    }
    Ok(res)
  }

  pub fn from_instruction(
    instruction: &HloInstruction) -> Result<SourceTargetPairs, String>
  {
    let src_tgt_pairs = instruction
      .frontend_attributes().map().get("_blitz_send_recv_source_target_pairs");
    if src_tgt_pairs.is_none() {
      let mut err_msg = "Instruction ".to_string();
      err_msg.push_str(&instruction.to_string_default());
      err_msg.push_str("does not have source-target pairs attribute.");
      return Err(err_msg);
    }
    SourceTargetPairs::from_string(src_tgt_pairs.unwrap())
  }

  pub fn join(a: &SourceTargetPairs, b: &SourceTargetPairs) -> Self {
    let mut res = SourceTargetPairs::default();
    res.pairs.clone_from(&a.pairs);
    for b_pair in &b.pairs {
      res.pairs.push((b_pair.0.clone(), 8));
    }
    res
  }

  // Returns a cannoical string such as {{0,1},{1,2},{2,3},{3,0}}.
  pub fn to_string(&self) -> String {
    let mut out = "{".to_string();
    for pair in &self.pairs {
      out.push_str("{");
      out.push_str(&pair.0.source.to_string());
      out.push_str(",");
      out.push_str(&pair.0.target.to_string());
      out.push_str("}");
      if self.pairs.last() != Some(pair) { out.push_str(",");}
    }
    out.push_str("}");
    out
  }

  pub fn data(&self) -> &Vec<(SourceTargetPair, i64)> {
    &self.pairs
  }

  pub fn size(&self) -> usize {
    self.pairs.len()
  }

  pub fn emplace_back(&mut self, source: i64, target: i64) {
    let pair = SourceTargetPair::new(source, target);
    self.pairs.push((pair, 8));
  }

  pub fn push_back(&mut self, pair: SourceTargetPair) {
    self.pairs.push((pair, 8));
  }

  // Converts to a vector of pairs of ints.
  pub fn expand(&self) -> Vec<(i64, i64)> {
    let mut data = vec![];
    for pair in &self.pairs {
      data.push((pair.0.source, pair.0.target));
    }
    data
  }

  pub fn get_max_device_num(&self) -> i64 {
    let mut max_device_num = -1;
    for pair in &self.pairs {
      let device_num = i64::max(pair.0.source, pair.0.target);
      max_device_num = i64::max(device_num, max_device_num);
    }
    max_device_num
  }
}