#![allow(dead_code)]


// Atomic ordering for Blitz's memory model.
#[derive(Debug, Clone, PartialEq)]
pub enum AtomicOrdering {
  NotAtomic = 0,
  Unordered = 1,
  Monotonic = 2,
  Acquire = 4,
  Release = 5,
  AcquireRelease = 6,
  SequentiallyConsistent = 7
}