#![allow(dead_code)]

pub struct CallInliner {
  single_call_site: bool,
  update_domain: bool,
}

impl CallInliner {
  pub fn new(single_call_site: bool, update_domain: bool) -> Self {
    CallInliner {
      single_call_site: single_call_site,
      update_domain: update_domain
    }
  }

  pub fn inline() {}

  pub fn name(&self) -> String {
    "CallInliner".to_string()
  }

  pub fn run() {}
}