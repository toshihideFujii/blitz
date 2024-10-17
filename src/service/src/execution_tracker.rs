#![allow(dead_code)]

use std::{collections::HashMap, sync::Mutex};

use common::blitz_data::{ExecutionHandle, ExecutionProfile, GlobalDataHandle};

use crate::{backend::Backend, stream_pool::StreamPool};

// Represents an asynchronously launched execution. Owns the stream (from the
// passed run_options->stream()) on which the execution is launched and releases
// the stream when destructed.
pub struct AsyncExecution {
  backend: Backend,
  streams: Vec<StreamPool>,
  profile: ExecutionProfile,
  result: GlobalDataHandle
}

impl AsyncExecution {
  pub fn new(
    backend: Backend,
    streams: Vec<StreamPool>,
    profile: ExecutionProfile,
    result: GlobalDataHandle) -> Self
  {
    AsyncExecution {
      backend: backend,
      streams: streams,
      profile: profile,
      result: result
    }
  }

  pub fn block_until_done(&self) -> Result<(), String> {
    unimplemented!()
  }

  pub fn result(&self) -> &GlobalDataHandle {
    &self.result
  }

  pub fn profile(&self) -> &ExecutionProfile {
    &self.profile
  }
}

// Tracks asynchronously launched executions for the Blitz service.
pub struct ExecutionTracker {
  next_handle: i64,
  handle_to_execution: Mutex<HashMap<i64, AsyncExecution>>
}

impl ExecutionTracker {
  pub fn new() -> Self {
    ExecutionTracker {
      next_handle: 1,
      handle_to_execution: Mutex::new(HashMap::new())
    }
  }

  // Registers an execution with its backend, streams, and data handle to the
  // execution result. Returns a handle for the registered execution.
  pub fn register(
    &mut self,
    backend: Backend,
    stream: Vec<StreamPool>,
    profile: ExecutionProfile,
    data: GlobalDataHandle) -> ExecutionHandle
  {
    let mut handle_map =
      self.handle_to_execution.lock().unwrap();
    let async_exec =
      AsyncExecution::new(backend, stream, profile, data);
    self.next_handle += 1;
    handle_map.insert(self.next_handle, async_exec);

    let mut exec_handle = ExecutionHandle::new();
    exec_handle.set_handle(self.next_handle);
    exec_handle
  }

  // Unregisters the execution for the given handle.
  pub fn unregister(&mut self, handle: &ExecutionHandle) -> Result<(), String> {
    let mut handle_map =
      self.handle_to_execution.lock().unwrap();
    let result = handle_map.get(&handle.handle());
    if result.is_none() {
      let mut err_msg = "No execution record for execution handle: ".to_string();
      err_msg.push_str(&handle.handle().to_string());
      return Err(err_msg);
    }
    handle_map.remove(&handle.handle());
    Ok(())
  }

  // Resolves the given ExecutionHandle to an AsyncExecution. Returns an
  // error status if the given handle is not found, which means that the
  // execution is not yet registered or already unregistered.
  pub fn resolve(&self, handle: &ExecutionHandle) -> Result<&AsyncExecution, String> {
    let handle_map =
      self.handle_to_execution.lock().unwrap();
    let result = handle_map.get(&handle.handle());
    if result.is_none() {
      let mut err_msg = "No execution record for execution handle: ".to_string();
      err_msg.push_str(&handle.handle().to_string());
      return Err(err_msg);
    }
    unimplemented!()
    //Ok(result.unwrap())
  }
}