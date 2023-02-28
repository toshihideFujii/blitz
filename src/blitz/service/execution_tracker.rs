#![allow(dead_code)]

// Represents an asynchronously launched execution.
struct AsyncExecution {}
impl AsyncExecution {
  pub fn new() {}
  pub fn block_until_done() {}
  pub fn result() {}
  pub fn profile() {}
}

// Tracks asynchronously launched executions for the BLITZ service.
struct ExecutionTracker {}
impl ExecutionTracker {
  pub fn new() {}
  pub fn register() {}
  pub fn unregister() {}
  pub fn resolve() {}
}