// Thread objects are used for creating and running threads.
// When the start() method is called the new thread starts
// running the run() method in the new thread.
// The Thread object should not be deallocated before the
// thread has terminated.
pub struct Thread {}

impl Thread {
  // Start new thread by calling the run() method on the new thread.
  pub fn start() {}

  // Start new thread and wait until run() method is called on the new thread.
  pub fn start_synchronously() {}

  // Wait until thread terminates.
  pub fn join() {}

  // Abstract method for run handler.
  pub fn run() {}

  // Thread-local storage.
  pub fn create_thread_local_key() {}
  pub fn delete_thread_local_key() {}
  pub fn get_thread_local() {}
  pub fn set_thread_local() {}
  pub fn has_thread_local() {}

  pub fn data() {}

  pub fn notify_started_and_return() {}

  pub fn name() {}
  pub fn set_name() {}
}
