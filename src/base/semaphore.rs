// A semaphore object is a synchronization object that maintains a count.
// The count is decremented each time a thread completes a wait for the
// semaphore object and incremented each time a thread signals the semaphore.
// When the count reaches zero, threads waiting for the semaphore blocks
// until the count becomes non-zero.
pub struct Semaphore {}

impl Semaphore {
  pub fn new(_count: u32) -> Self {
    Semaphore {}
  }

  // Imcrements the semaphore counter.
  pub fn signal(&self) {}

  // Decrements the semaphore counter if it is positive, or blocks until
  // it becomes positive and then decrements the counter.
  pub fn wait(&self) {}

  pub fn wait_for() {}

  pub fn native_handle() {}
}
