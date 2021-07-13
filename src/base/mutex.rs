use std::sync::{Mutex, MutexGuard};

// This class is a synchronization primitive that can be used to
// protect shared data from being simultaneously accessed by
// multiple threads.
pub struct _Mutex {
  native_handle_: std::sync::Mutex<u32>,
}

impl _Mutex {
  pub fn new() -> _Mutex {
    _Mutex {
      native_handle_: Mutex::new(0),
      //guard_: MutexGuard<'_, u32>
    }
  }

  // Locks the given mutex.
  // If the mutex is currently unlocked, it becomes locked and
  // owned by the calling thread, and immediatry.
  // If the mutex is already locked by other thread, suspends the
  // calling thread until the mutex is unlocked.
  pub fn lock(&self) -> MutexGuard<u32> {
    /*
    match self.native_handle_.lock() {
      Ok(_) => print!("OK"),
      Err(_) => panic!("Error")
    };
    */
    self.native_handle_.lock().unwrap()
  }

  // Unlocks the given mutex.
  // The mutex is assumed to be locked and owned by the calling
  // thread on entrance.
  pub fn unlock(&self, guard: MutexGuard<u32>) {
    drop(guard)
  }

  // Tries to lock the given mutex.
  // Returns whether the mutex was successfully locked.
  pub fn try_lock() {}

  pub fn native_handle() {}
}
