#![allow(dead_code)]

use std::{sync::Mutex, collections::HashMap};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct ExecutorResourceRequest {
  resource_name: String,
  amount: u64,
  discovery_script: String,
  vendor: String,
}

impl ExecutorResourceRequest {
  pub fn new(resource_name: String, amount: u64) -> Self {
    ExecutorResourceRequest {
      resource_name: resource_name,
      amount: amount,
      discovery_script: "".to_string(),
      vendor: "".to_string(),
    }
  }
}

pub struct ExecutorResourceRequests {
  executor_resources: Mutex<HashMap<String, ExecutorResourceRequest>>
}

impl ExecutorResourceRequests {
  pub fn new() {}

  pub fn requests(&self) -> &Mutex<HashMap<String, ExecutorResourceRequest>> {
    &self.executor_resources
  }

  pub fn memory(&self) {
      
  }
}