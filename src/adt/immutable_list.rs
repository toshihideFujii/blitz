#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq, Clone)]
struct ImmutableList<T> {
  list_: Vec<T>,
  tail_: Option<Box<ImmutableList<T>>>
}

impl<T> ImmutableList<T> where T: PartialEq + Clone {
  pub fn new() -> Self {
    ImmutableList { list_: Vec::new(), tail_: None }
  }

  // Returns true if the list is empty.
  pub fn is_empty(&self) -> bool {
    self.list_.is_empty()
  }

  pub fn contains(&self, v: &T) -> bool {
    self.list_.contains(v)
  }

  // Returns true if two lists are equal.
  pub fn is_equal(&self, rhs: &ImmutableList<T>) -> bool {
    self.list_ == rhs.list_
  }

  // Returns the head of the list.
  pub fn get_head(&self) -> Option<&T> {
    self.list_.first()
  }

  // Returns the tail of the list.
  pub fn get_tail(&self) -> Option<Box<ImmutableList<T>>> {
    self.tail_.clone()
  }

  pub fn add(v: T, l: &ImmutableList<T>) -> ImmutableList<T> {
    let mut list = ImmutableList::new();
    list.list_.extend_from_slice(&l.list_);
    list.list_.insert(0, v);
    list.tail_ = Some(Box::new(l.clone()));
    list
  }

  pub fn profile() {}
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty_int_list() {
    let l: ImmutableList<i32> = ImmutableList::new();
    assert_eq!(l.is_empty(), true);
    let l2: ImmutableList<i32> = ImmutableList::new();
    assert_eq!(l.is_equal(&l2), true);
  }

  #[test]
  fn test_one_elem_int_list() {
    let l: ImmutableList<i32> = ImmutableList::new();
    let l2 = ImmutableList::add(3, &l);
    assert_eq!(l.is_empty(), true);
    assert_eq!(l2.is_empty(), false);
    assert_eq!(l2.get_tail().take().unwrap().is_empty(), true);

    assert_eq!(l == l2, false);
    assert_eq!(&l == l2.get_tail().take().unwrap().as_ref(), true);
    assert_eq!(l.is_equal(&l2), false);
    assert_eq!(l.is_equal(&l2.get_tail().take().unwrap()), true);

    assert_eq!(l.contains(&3), false);
    assert_eq!(l2.get_head(), Some(&3));
    assert_eq!(l2.contains(&3), true);

    let l3 = ImmutableList::add(2, &l);
    assert_eq!(l3.is_empty(), false);
    assert_eq!(l == l3, false);
    assert_eq!(l.contains(&2), false);
    assert_eq!(l3.contains(&2), true);
    assert_eq!(l3.get_head(), Some(&2));

    assert_eq!(l2 == l3, false);
    assert_eq!(l2.contains(&2), false);
  }

  #[test]
  fn test_multi_elem_int_list_test() {
    let l: ImmutableList<i32> = ImmutableList::new();
    let l2 = ImmutableList::add(5, &ImmutableList::add(4, &ImmutableList::add(3, &l)));
    let l3 = ImmutableList::add(43, &ImmutableList::add(20, &ImmutableList::add(9, &l2)));
    let l4 = ImmutableList::add(9, &l2);
    let l5 = ImmutableList::add(9, &l2);

    assert_eq!(l.is_empty(), true);
    assert_eq!(l2.is_empty(), false);
    assert_eq!(l3.is_empty(), false);
    assert_eq!(l4.is_empty(), false);

    assert_eq!(l.contains(&3), false);
    assert_eq!(l.contains(&9), false);

    assert_eq!(l2.contains(&3), true);
    assert_eq!(l2.contains(&4), true);
    assert_eq!(l2.contains(&5), true);
    assert_eq!(l2.contains(&9), false);
    assert_eq!(l2.contains(&0), false);

    assert_eq!(l2.get_head(), Some(&5));
    assert_eq!(l2.get_tail().take().unwrap().get_head(), Some(&4));
    assert_eq!(l2.get_tail().take().unwrap().get_tail().take().unwrap().get_head(), Some(&3));

    assert_eq!(l3.contains(&43), true);
    assert_eq!(l3.contains(&20), true);
    assert_eq!(l3.contains(&9), true);
    assert_eq!(l3.contains(&3), true);
    assert_eq!(l3.contains(&4), true);
    assert_eq!(l3.contains(&5), true);
    assert_eq!(l3.contains(&0), false);

    assert_eq!(l3.get_head(), Some(&43));
    assert_eq!(l3.get_tail().take().unwrap().get_head(), Some(&20));
    assert_eq!(l3.get_tail().take().unwrap().get_tail().take().unwrap().get_head(), Some(&9));

    assert_eq!(l4.contains(&9), true);
    assert_eq!(l4.contains(&3), true);
    assert_eq!(l4.contains(&4), true);
    assert_eq!(l4.contains(&5), true);
    assert_eq!(l4.contains(&20), false);
    assert_eq!(l4.contains(&43), false);
    assert_eq!(l4.is_equal(&l4), true);
    assert_eq!(l4.is_equal(&l5), true);

    assert_eq!(l5.is_equal(&l4), true);
    assert_eq!(l5.is_equal(&l5), true);
  }
}