use std::{borrow::BorrowMut, collections::btree_map::Entry};

struct View {}

impl View {
  // Pushes an entry onto the worklist.
  pub fn push() {}

  // Pops an entry from the worklist.
  pub fn pop() {}

  // Returns true if the local portion of the worklist is empty.
  pub fn is_local_empty() {}

  // Returns true if the worklist is empty.
  // Can only be used from the main thread without concurrent access.
  pub fn is_empty() {}

  pub fn is_global_pool_empty() {}

  pub fn local_push_segment_size() {}

  pub fn flush_to_global() {}
}

const MAX_NUM_TASKS: usize = 8;
const SEGMENT_CAPACITY: usize = 100; // TODO SEGMENT_SIZE
const CAPACITY: usize = SEGMENT_CAPACITY;

// A concurrent worklist based on segments.
// Each tasks gets private push and pop segments.
// Empty pop segments are swapped with their corresponding push segments.
// Full push segments are published to a global pool of segments and
// replaced with empty segments.
struct Worklist {
  private_segments_: [PrivateSegmentHolder; MAX_NUM_TASKS],
  global_pool_: GlobalPool,
  num_tasks_: u64,
}

impl Worklist {
  pub fn new() {}

  // Swaps content with the given worklist.
  // Local buffers need to be empty, not thread safe.
  pub fn swap() {}

  pub fn push() {}

  pub fn pop() {}

  pub fn local_push_segment_size() {}

  pub fn is_local_empty() {}

  pub fn is_global_pool_empty() {}

  pub fn is_empty() {}

  pub fn are_locals_empty() {}

  pub fn local_size() {}

  // Thread-safe but may return an outdated result.
  pub fn global_pool_size() {}

  // Clears all segments.
  // Frees the global segment pool.
  // Assume that no other tasks are running.
  pub fn clear() {}

  pub fn update() {}
  pub fn iterate() {}
  pub fn iterate_global_pool() {}
  pub fn flush_to_global() {}
  pub fn merge_global_pool() {}

  fn private_push_segment() {}

  fn private_pop_segment() {}

  fn publish_push_segment_to_global() {}

  fn publish_pop_segment_to_global() {}

  fn steal_pop_segment_from_global() {}

  fn new_segment() {}
}

pub struct Segment<EntryType> {
  //seglist_: SegmentList<EntryType>,
  entries_: Vec<EntryType>, //[EntryType; CAPACITY]
  next_: SegmentLink<EntryType>,
}

type SegmentLink<EntryType> = Option<Box<Segment<EntryType>>>;

struct SegmentList<EntryType> {
  //Next(Box<Segment<EntryType>>),
  head_: SegmentLink<EntryType>, //Nil,
}

impl<EntryType> Segment<EntryType> {
  pub fn new() -> Segment<EntryType> {
    Segment {
      //seglist_: SegmentList { head_: None },
      entries_: Vec::new(),
      next_: None,
    }
  }

  pub fn push(&mut self, entry: EntryType) -> bool {
    if self.is_full() {
      return false;
    }
    self.entries_.push(entry);
    return true;
  }

  pub fn pop(&mut self) -> Option<EntryType> {
    if self.is_empty() {
      return None;
    }
    return self.entries_.pop();
  }

  pub fn size(&self) -> usize {
    self.entries_.len()
  }

  pub fn is_empty(&self) -> bool {
    self.entries_.is_empty()
  }

  pub fn is_full(&self) -> bool {
    self.entries_.len() == CAPACITY
  }

  pub fn clear(&mut self) {
    self.entries_.clear()
  }

  pub fn update() {}

  pub fn iterate<Callback>(&self, _callback: Callback) {}

  pub fn next(self) -> SegmentLink<EntryType> {
    self.next_
  }

  pub fn set_next() {}
}

pub struct PrivateSegmentHolder {}

struct GlobalPool {}

impl GlobalPool {
  pub fn new() {}

  pub fn swap() {}

  pub fn push() {}

  pub fn pop() {}

  pub fn is_empty() {}

  pub fn size() {}

  pub fn clear() {}

  pub fn update() {}

  pub fn iterate() {}

  pub fn merge() {}

  fn set_top() {}
}
