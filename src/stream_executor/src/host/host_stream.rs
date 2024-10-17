#![allow(dead_code)]

use std::{collections::VecDeque, sync::Mutex, thread::{self, JoinHandle}};

use crate::{stream::Stream, event::Event, stream_executor::StreamExecutor, };

use super::host_executor::HostExecutor;

// Class declaration for Stream type that enqueues tasks onto a host/CPU-based
// execution context (as opposed to a GPU device), HostExecutor.
pub struct HostStream {
  parent: HostExecutor,
  work_queue: Mutex<VecDeque<i64>>,
  thread: JoinHandle<()>,
}

impl HostStream {
  pub fn new(executor: HostExecutor) -> Self {
    HostStream {
      parent: executor,
      work_queue: Mutex::new(VecDeque::new()),
      thread: thread::spawn(|| {  })
    }
  }

  pub fn enque_task_with_status() {}

  pub fn enque_task() {}

  pub fn block_until_done() {}

  fn work_available(&self) -> bool {
    !self.work_queue.lock().unwrap().is_empty()
  }
  
  fn work_loop(&self) {}
}

impl Stream for HostStream {
  fn get_or_create_sub_stream(&self) -> Result<Box<dyn Stream>, String> {
    unimplemented!()
  }

  fn initialize(&self) -> Result<(), String> {
    unimplemented!()
  }

  fn memcpy(&self) {
    unimplemented!()
  }

  fn mem_zero(&self) {
    unimplemented!()
  }

  fn ok(&self) -> bool {
    unimplemented!()
  }

  fn parent(&self) -> &dyn StreamExecutor {
    &self.parent
  }

  fn platform_specific_handle(&self) -> &crate::stream::PlatformSpecificHandle {
    unimplemented!()
  }

  fn priority(&self) -> crate::platform::StreamPriority {
    unimplemented!()
  }

  fn record_event(&self, _event: &dyn Event) -> Result<(), String> {
    unimplemented!()
  }

  fn refresh_status(&self) -> Result<(), String> {
    unimplemented!()
  }

  fn return_sub_stream(&self, _sub_stream: &dyn Stream) {
    unimplemented!()
  }

  fn then_launch(
      &self,
      _thread_dims: &crate::launch_dim::ThreadDim,
      _block_dims: &crate::launch_dim::BlockDim) -> Result<(), String> {
    unimplemented!()
  }

  fn wait_for(&self, _other: &dyn Stream) -> Result<(), String> {
    unimplemented!()
  }

  fn wait_for_event(&self, _event: &dyn Event) -> Result<(), String> {
    unimplemented!()
  }

  fn launch(&self) {
    unimplemented!()
  }
}