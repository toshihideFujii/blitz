pub trait VectorOp<T> {
  fn new(data: &str) -> Vector<T>;
  fn new_with_len(length: usize) -> Vector<T>;

  // Returns a vector using the same backing storage as this one,
  // spanning from and including 'from' to but not including 'to'.
  //fn sub_vector(&self, from: usize, to: usize) -> Vector<T>;

  // Returns the length of the vector.
  // Only use this if you really need an integer return value.
  // Use size() otherwise.
  fn length(&self) -> i32;

  // Returns the length of the vector as a usize.
  fn size(&self) -> usize;

  // Returns whether or not the vector is empty.
  fn empty(&self) -> bool;

  fn at(&self, index: usize) -> T;

  fn first(&self) -> T;

  fn last(&self) -> T;

  // Returns a pointer to the start of the data in the vector.
  //fn begin(&self) -> Vec<T>;

  // For consistency with other containers, do also provide a {data} accessor.
  //fn data(&self) -> Vec<T>;

  // Returns a pointer past the end of the data in the vector.
  //fn end(&self) -> Vec<T>;

  fn truncate(&mut self, len: usize);

  // Releases the array underlying this vector.
  // Once disposed the vector is empty.
  fn dispose(&mut self);
}

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<T> {
  start_: Vec<T>,
  length_: usize,
}

impl VectorOp<char> for Vector<char> {
  fn new(data: &str) -> Vector<char> {
    let vec: Vec<char> = data.chars().collect();
    Self {
      start_: vec.clone(),
      length_: vec.clone().len(),
    }
  }
  fn new_with_len(length: usize) -> Vector<char> {
    Self {
      start_: Vec::with_capacity(length),
      length_: length,
    }
  }
  //fn sub_vector(&self, from: usize, to: usize) -> Vector<char> {
  //
  //}
  fn length(&self) -> i32 {
    self.start_.len() as i32
  }
  fn size(&self) -> usize {
    self.start_.len()
  }
  fn empty(&self) -> bool {
    self.start_.is_empty()
  }
  fn at(&self, index: usize) -> char {
    self.start_[index]
  }
  fn first(&self) -> char {
    self.start_[0]
  }
  fn last(&self) -> char {
    self.start_[self.start_.len() - 1]
  }
  //fn begin(&self) -> Vec<char> {
  //    self.start_
  //}
  //fn data(&self) -> Vec<char> {
  //    self.start_
  //}
  //fn end(&self) -> Vec<char> {
  //}
  fn truncate(&mut self, len: usize) {
    self.start_.truncate(len)
  }
  fn dispose(&mut self) {
    self.start_.clear()
  }
}

impl<T> Vector<T> {}

pub fn c_str_vector(data: &str) -> Vector<char> {
  Vector::new(data)
}

#[cfg(test)]

mod tests {
  use super::*;

  #[test]
  fn test_vector_factories() {
    let vec = c_str_vector("foo");
    assert_eq!(vec.size(), 3);
    assert_eq!(vec.at(0), 'f');
    assert_eq!(vec.at(1), 'o');
    assert_eq!(vec.at(2), 'o');
    /*
    let vec = c_str_vector("foo\0\0");
    assert_eq!(vec.size(), 3);
    assert_eq!(vec.at(0), 'f');
    assert_eq!(vec.at(1), 'o');
    assert_eq!(vec.at(2), 'o');
    */
  }
}
