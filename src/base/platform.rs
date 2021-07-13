use std::{
  cell::RefCell,
  thread::{self, JoinHandle},
};

// This class has the static methods for the different platform
// specific functions.
// Add methods here to cope with differences between the
// supported platforms.
pub struct OS {}

impl OS {
  // Initialize the OS class.
  pub fn initialize() {}

  // Returns the accumulated user time for the thread.
  // This routine can be used for profiling.
  // The implementation should strive for high-precision timer resolution,
  // preferable micro-second resolution.
  pub fn get_user_time() {}

  // Returns current time as the number of milliseconds since
  // 00:00:00 UTC, January 1, 1970.
  pub fn time_current_millis() {}
}

// Thread objects are used for creating and running threads.
// When the start() method is called the new thread starts
// running the run() method in the new thread.
// The Thread object should not be deallocated before the
// thread has terminated.
pub struct Thread {
  thread_: JoinHandle<()>,
}

thread_local! {
  pub static KEY: RefCell<String> = RefCell::new("key".to_string());
}

impl Thread {
  //thread_local! {
  // pub static KEY: RefCell<String> = RefCell::new("key".to_string());
  //}

  // Start new thread by calling the run() method on the new thread.
  pub fn start(&mut self) {
    self.thread_ = thread::spawn(move || {});
  }

  // Start new thread and wait until run() method is called on the new thread.
  pub fn start_synchronously() {}

  // Wait until thread terminates.
  pub fn join(&self) {
    //self.thread_.join();
  }

  // Abstract method for run handler.
  pub fn run() {}

  // Thread-local storage.
  pub fn create_thread_local_key() {}
  pub fn delete_thread_local_key() {}

  pub fn get_thread_local() -> String {
    KEY.with(|_key| _key.borrow().clone())
  }

  pub fn set_thread_local(key: String) {
    KEY.with(|_key| {
      *_key.borrow_mut() = key;
    })
  }

  pub fn has_thread_local() {}

  pub fn data() {}

  pub fn notify_started_and_return() {}

  pub fn name() {}
  pub fn set_name() {}
}
