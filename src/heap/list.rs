struct List<T> {
  front_: T,
  back_: T,
}

impl<T> List<T> {
  pub fn new() {}
  pub fn shallow_copy_to() {}
  pub fn push_back() {}
  pub fn push_front() {}
  pub fn remove() {}
  pub fn contains() {}

  pub fn empty(&self) -> bool {
    false
  }

  pub fn front(&self) -> &T {
    &self.front_
  }

  pub fn back(&self) -> &T {
    &self.back_
  }

  fn add_first_element() {}
  fn insert_after() {}
  fn insert_before() {}
}

struct ListNode<T> {
  next_: T,
  prev_: T,
}

impl<T> ListNode<T> {
  pub fn new(next: T, prev: T) -> ListNode<T> {
    ListNode {
      next_: next,
      prev_: prev,
    }
  }

  pub fn next(&self) -> &T {
    &self.next_
  }

  pub fn prev(&self) -> &T {
    &self.prev_
  }

  fn set_next(&mut self, next: T) {
    self.next_ = next
  }

  fn set_prev(&mut self, prev: T) {
    self.prev_ = prev
  }
}
