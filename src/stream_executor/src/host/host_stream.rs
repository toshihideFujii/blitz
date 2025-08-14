#![allow(dead_code)]

use std::{collections::VecDeque, sync::Mutex, thread::{self, JoinHandle}};
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