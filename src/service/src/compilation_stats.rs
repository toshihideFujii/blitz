#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct PassInfo {
  name: String,
  num_runs: i64,
  duration_ms: f64
}

impl PassInfo {
  pub fn new(name: String, dutration: f64) -> Self {
    PassInfo {
      name: name,
      num_runs: 1,
      duration_ms: dutration
    }
  }
}

// This class is used to collect information about HLO passes and print some
// statistics at the end of compilation.
pub struct CompilationStats {
  passes: Vec<PassInfo>,
  pass_running: bool,
  current_pass: String,
  start_micros: u64
}

impl CompilationStats {
  pub fn make_noop_stats() {}
  pub fn make_stats() {}

  pub fn start_pass(&mut self, pass_name: String) {
    assert!(!self.pass_running, "Can's start {:?} while running {:?}",
      pass_name, self.current_pass);
    self.pass_running = true;
    self.current_pass = pass_name;
    // TODO: start_micros 
  }

  pub fn end_pass(&mut self, pass_name: String) {
    assert!(self.pass_running);
    assert_eq!(self.current_pass, pass_name);
    self.pass_running = false;
    // TODO: end_micros
  }

  pub fn compilation_report(&self) {
    assert!(!self.pass_running, "end_pass never called for {:?}", self.current_pass);
    let mut summary: HashMap<String, PassInfo> = HashMap::new();
    let mut total_duration = 0.0;

    for pass_run in &self.passes {
      let pass_name = &pass_run.name;
      total_duration += pass_run.duration_ms;
      let mut pass_info = summary.get_mut(pass_name);
      if pass_info.is_none() {
        summary.insert(pass_name.clone(), pass_run.clone());
      } else {
        pass_info.as_mut().unwrap().num_runs += 1;
        pass_info.as_mut().unwrap().duration_ms += pass_run.duration_ms;
      }
    }

    // TODO: summary.sort

    println!("Total runtime (ms) of HLO passes: {:?}", total_duration);
    println!("Pass name, num runs, time (ms)");
    let pass_infos: Vec<PassInfo> = summary.into_values().collect();
    for pass_info in &pass_infos {
      println!("{:?}, {:?}, {:?}", pass_info.name, pass_info.num_runs, pass_info.duration_ms);
    }
  }

  pub fn get_passes_size(&self) -> usize {
    self.passes.len()
  }

  pub fn record_pass_error() {}
}