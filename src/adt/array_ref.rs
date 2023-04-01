#![allow(dead_code)]
#![allow(unused_variables)]

// Represent a constant reference to an array.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ArrayRef<T> {
  data: Vec<T>
}

impl<T> ArrayRef<T> where T: Clone {
  pub fn new() -> Self {
    ArrayRef { data: Vec::new() }
  }

  pub fn new_from_vec(vec: &Vec<T>) -> Self {
    //let mut v = Vec::new();
    //v.reserve_exact(vec.len());
    let v = vec.clone();
    //v.clone_from_slice(vec);
    ArrayRef { data: v }
  }

  pub fn begin() {}
  pub fn end() {}
  pub fn rbegin() {}
  pub fn rend() {}

  // Check if the array is empty.
  pub fn empty(&self) -> bool {
    self.data.is_empty()
  }

  //pub fn data(&self) -> Vec<T> {}

  // Get the array size.
  pub fn size(&self) -> usize {
    self.data.len()
  }

  // Get the first element.
  pub fn front(&self) -> Option<&T> {
    self.data.get(0)
  }

  // Get the last element.
  pub fn back(&self) -> Option<&T> {
    self.data.last()
  }

  pub fn copy() {}
  pub fn equals() {}

  // Chop off the first n elements of the array, and keep
  // m elements in the array.
  pub fn slice(&mut self, n: usize, m: usize) -> ArrayRef<T> {
    debug_assert!(n+m <= self.size(), "Invalid specifier");
    let v = self.data.drain(n..n+m).collect();
    ArrayRef { data: v }
  }

  // Drop the first n elements of the array.
  pub fn drop_front(&self, n: usize) -> ArrayRef<T> {
    debug_assert!(self.size() >= n, "Fropping more elements than exist");
    let (abandoned, remained) = self.data.split_at(n);
    ArrayRef::new_from_vec(&remained.to_vec())
  }

  // Drop the last n elements of the array.
  pub fn drop_back(&self, n: usize) -> ArrayRef<T> {
    debug_assert!(self.size() >= n, "Fropping more elements than exist");
    let (remained, abandoned) = self.data.split_at(self.size() - n);
    ArrayRef::new_from_vec(&remained.to_vec())
  }

  // Return a copy of this with the first n elements satisfying the
  // given predicate removed.
  pub fn drop_while<P: FnMut(&&T) -> bool>(&self, pred: P) -> ArrayRef<T> {
    let iter = self.data.iter().skip_while(pred);
    let mut v = Vec::new();
    for item in iter {
      v.push(item.clone());
    }
    ArrayRef::new_from_vec(&v)
  }
  

  pub fn drop_until() {}

  // Return a copy of this with only the first n elements.
  pub fn take_front(&self, n: usize) -> ArrayRef<T> {
    if n >= self.size() {
      return self.clone();
    }
    self.drop_back(self.size() - n)
  }

  // Return a copy of this with only the last n elements.
  pub fn take_back(&self, n: usize) -> ArrayRef<T> {
    if n >= self.size() {
      return self.clone();
    }
    self.drop_front(self.size() - n)
  }

  // Return the first n elements of this array that satisfy the given predicate.
  pub fn take_while<P: FnMut(&&T) -> bool>(&self, pred: P) -> ArrayRef<T> {
    let iter = self.data.iter().take_while(pred);
    let mut v = Vec::new();
    for item in iter {
      v.push(item.clone());
    }
    ArrayRef::new_from_vec(&v)
  }

  pub fn take_until() {}
}

struct MutableArrayRef {}

impl MutableArrayRef {
  pub fn new() {}

  pub fn begin() {}
  pub fn end() {}
  pub fn rbegin() {}
  pub fn rend() {}

  pub fn empty() {}
  pub fn data() {}
  pub fn size() {}
  pub fn front() {}
  pub fn back() {}

  pub fn copy() {}
  pub fn equals() {}
  pub fn slice() {}

  pub fn drop_front() {}
  pub fn drop_back() {}
  pub fn drop_while() {}
  pub fn drop_until() {}

  pub fn take_front() {}
  pub fn take_back() {}
  pub fn take_while() {}
  pub fn take_until() {}
}

struct OwningArrayRef {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_drop_back() {
    let numbers1 = vec![4, 8, 15, 16, 23, 42];
    let numbers2 = vec![4, 8, 15, 16, 23];
    let ar1 = ArrayRef::new_from_vec(&numbers1);
    let ar2 = ArrayRef::new_from_vec(&numbers2);
    assert_eq!(ar1.drop_back(1) == ar2, true);
  }

  #[test]
  fn test_drop_front() {
    let numbers1 = vec![4, 8, 15, 16, 23, 42];
    let numbers2 = vec![15, 16, 23, 42];
    let ar1 = ArrayRef::new_from_vec(&numbers1);
    let ar2 = ArrayRef::new_from_vec(&numbers2);
    assert_eq!(ar1.drop_front(2) == ar2, true);
  }

  #[test]
  fn test_drop_while() {
    let numbers1 = vec![1, 3, 5, 8, 10, 11];
    let ar1 = ArrayRef::new_from_vec(&numbers1);
    let expected = ar1.drop_front(3);
    assert_eq!(ar1.drop_while(|n| *n % 2 == 1), expected);
    assert_eq!(ar1.drop_while(|n| **n < 0), ar1);

    let empty_list = ArrayRef::new();
    assert_eq!(ar1.drop_while(|n| **n > 0), empty_list);
  }

  #[test]
  fn test_take_back() {
    let numbers1 = vec![4, 8, 15, 16, 23, 42];
    let numbers2 = vec![42];
    let ar1 = ArrayRef::new_from_vec(&numbers1);
    let ar2 = ArrayRef::new_from_vec(&numbers2);
    assert_eq!(ar1.take_back(1) == ar2, true);
  }

  #[test]
  fn test_take_front() {
    let numbers1 = vec![4, 8, 15, 16, 23, 42];
    let numbers2 = vec![4, 8];
    let ar1 = ArrayRef::new_from_vec(&numbers1);
    let ar2 = ArrayRef::new_from_vec(&numbers2);
    assert_eq!(ar1.take_front(2) == ar2, true);
  }

  #[test]
  fn test_take_while() {
    let numbers = vec![1, 3, 5, 8, 10, 11];
    let ar1 = ArrayRef::new_from_vec(&numbers);
    let expected = ar1.take_front(3);
    assert_eq!(ar1.take_while(|n| *n % 2 == 1), expected);
    assert_eq!(ar1.take_while(|n| **n > 0), ar1);

    let empty_list = ArrayRef::new();
    assert_eq!(ar1.take_while(|n| **n < 0), empty_list);
  }

  #[test]
  fn test_equals() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let ar1 = ArrayRef::new_from_vec(&numbers);

    assert_eq!(ar1, ArrayRef::new_from_vec(&vec![1, 2, 3, 4, 5, 6, 7, 8]));
    assert_ne!(ar1, ArrayRef::new_from_vec(&vec![8, 1, 2, 4, 5, 6, 6, 7]));
    assert_ne!(ar1, ArrayRef::new_from_vec(&vec![2, 4, 5, 6, 6, 7, 8, 1]));
    assert_ne!(ar1, ArrayRef::new_from_vec(&vec![0, 1, 2, 4, 5, 6, 6, 7]));
    assert_ne!(ar1, ArrayRef::new_from_vec(&vec![1, 2, 42, 4, 5, 6, 7, 8]));
    assert_ne!(ar1, ArrayRef::new_from_vec(&vec![42, 2, 3, 4, 5, 6, 7, 8]));
    assert_ne!(ar1, ArrayRef::new_from_vec(&vec![1, 2, 3, 4, 5, 6, 7]));
    assert_ne!(ar1, ArrayRef::new_from_vec(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));

    let mut ar1_a = ar1.drop_back(1);
    assert_eq!(ar1_a, ArrayRef::new_from_vec(&vec![1, 2, 3, 4, 5, 6, 7]));
    assert_ne!(ar1_a, ArrayRef::new_from_vec(&vec![1, 2, 3, 4, 5, 6, 7, 8]));

    let ar1_b = ar1_a.slice(2, 4);
    assert_eq!(ar1_b, ArrayRef::new_from_vec(&vec![3, 4, 5, 6]));
    assert_ne!(ar1_b, ArrayRef::new_from_vec(&vec![2, 3, 4, 5, 6]));
    assert_ne!(ar1_b, ArrayRef::new_from_vec(&vec![3, 4, 5, 6, 7]));
  }

  #[test]
  fn test_initializer_list() {
    let init_list = vec![0, 1, 2, 3, 4];
    let mut a = ArrayRef::new_from_vec(&init_list);
    let mut i = 0;
    for v in a.data {
      assert_eq!(v == i, true);
      i += 1;
    }

    let init_list_2 = vec![1, 2];
    a.data = init_list_2;
    assert_eq!(a.data[0], 1);
    assert_eq!(a.data[1], 2);
    assert_eq!(a.size(), 2);
  }

  #[test]
  fn test_empty_initializer_list() {
    let a: ArrayRef<i32> = ArrayRef::new();
    assert_eq!(a.empty(), true);
  }
}