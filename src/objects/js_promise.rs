// Representation of promise objects in the specification.
struct JSPromise {}

impl JSPromise {
  // Checks that the peomise is settled and returns the result.
  pub fn result() {}

  // Checks that the promise is pending and returns the result.
  pub fn reactions() {}

  // Whether this promise has a reject handler or not.
  pub fn has_handler() {}

  // Whether this promise will be handled by a catch block in an async function.
  pub fn handled_hint() {}

  pub fn async_task_id() {}
  pub fn set_async_task_id() {}

  pub fn status() {}
  pub fn set_status() {}

  pub fn fulfill() {}
  pub fn reject() {}
  pub fn resolve() {}

  fn trigger_promise_reactions() {}
}
