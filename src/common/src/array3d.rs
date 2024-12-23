#![allow(dead_code)]

// Simple 3D array structure.
pub struct Array3D<T> {
  values: Vec<Vec<Vec<T>>>
}

impl<T> Array3D<T> where  T: Default + Clone {
  pub fn default() -> Self {
    Array3D { values: Vec::new() }
  }

  // Creates an array of dimensions n1 x n2 x n3, uninitialized values.
  pub fn new(n1: usize, n2: usize, n3: usize) -> Self {
    let mut instance = Array3D { values: Vec::new() };
    instance.values.resize(n1, vec![vec![T::default(); n3]; n2]);
    for i in 0..n1 {
      let vec2 = &mut instance.values[i];
      vec2.resize(n2, vec![T::default(); n3]);
      for j in 0..n2 {
        let vec3 = &mut instance.values[i][j];
        vec3.resize(n3, T::default());
      }
    }
    instance
  }

  // Creates an array of dimensions n1 x n2 x n3, initialized to value.
  pub fn new_fill_value(n1: usize, n2: usize, n3: usize, value: T) -> Self {
    let mut instance = Array3D::new(n1, n2, n3);
    instance.fill(value);
    instance
  }

  // Creates an array from the given nested initializer list. The outer
  // initializer list is the first dimension, and so on.
  //
  // For example {{{1, 2}, {3, 4}, {5, 6}, {7, 8}},
  //              {{9, 10}, {11, 12}, {13, 14}, {15, 16}},
  //              {{17, 18}, {19, 20}, {21, 22}, {23, 24}}}
  // results in an array with n1=3, n2=4, n3=2.
  pub fn new_from(values: Vec<Vec<Vec<T>>>) -> Self {
    Array3D { values: values }
  }

  pub fn n1(&self) -> usize {
    self.values.len()
  }

  pub fn n2(&self) -> usize {
    if self.values.len() == 0 { return 0; }
    self.values[0].len()
  }

  pub fn n3(&self) -> usize {
    if self.values.len() == 0 || self.values[0].len() == 0 { return 0; }
    self.values[0][0].len()
  }

  pub fn num_elements(&self) -> usize {
    self.n1() * self.n2() * self.n3()
  }

  pub fn fill(&mut self, value: T) {
    for i in 0..self.n1() {
      for j in 0..self.n2() {
        for k in 0..self.n3() {
          self.values[i][j][k] = value.clone();
        }
      }
    }
  }

  pub fn data(&self, n1: usize, n2: usize, n3: usize) -> &T {
    &self.values[n1][n2][n3]
  }

  pub fn set_data(&mut self, n1: usize, n2: usize, n3: usize, data: T) {
    self.values[n1][n2][n3] = data;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_uninitialized_dims_ctor() {
    let uninit: Array3D<i64> = Array3D::new(2, 3, 4);
    assert_eq!(uninit.n1(), 2);
    assert_eq!(uninit.n2(), 3);
    assert_eq!(uninit.n3(), 4);
    assert_eq!(uninit.num_elements(), 24);
  }

  #[test]
  fn test_fill_ctor() {
    let full_of_7 = Array3D::new_fill_value(2, 3, 4, 7);
    assert_eq!(full_of_7.n1(), 2);
    assert_eq!(full_of_7.n2(), 3);
    assert_eq!(full_of_7.n3(), 4);

    for n1 in 0..full_of_7.n1() {
      for n2 in 0..full_of_7.n2() {
        for n3 in 0..full_of_7.n3() {
          assert_eq!(full_of_7.values[n1][n2][n3], 7);
        }
      }
    }
  }

  #[test]
  fn test_initializer_list_ctor() {
    let values = vec![
      vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]],
      vec![vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16]], 
      vec![vec![17, 18], vec![19, 20], vec![21, 22], vec![23, 24]], 
    ];
    let arr = Array3D::new_from(values);
    assert_eq!(arr.n1(), 3);
    assert_eq!(arr.n2(), 4);
    assert_eq!(arr.n3(), 2);
    assert_eq!(arr.num_elements(), 24);

    assert_eq!(arr.data(0, 0, 0), &1);
    assert_eq!(arr.data(0, 0, 1), &2);
    assert_eq!(arr.data(0, 1, 0), &3);
    assert_eq!(arr.data(0, 3, 1), &8);
    assert_eq!(arr.data(1, 0, 0), &9);
    assert_eq!(arr.data(1, 1, 1), &12);
    assert_eq!(arr.data(2, 0, 0), &17);
    assert_eq!(arr.data(2, 1, 1), &20);
    assert_eq!(arr.data(2, 2, 0), &21);
    assert_eq!(arr.data(2, 3, 1), &24);
  }

  #[test]
  fn test_initializer_list_ctor_f64() {
    let values = vec![
      vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0], vec![7.0, 8.0]],
      vec![vec![9.0, 10.0], vec![11.0, 12.0], vec![13.0, 14.0], vec![15.0, 16.0]], 
      vec![vec![17.0, 18.0], vec![19.0, 20.0], vec![21.0, 22.0], vec![23.0, 24.0]], 
    ];
    let arr = Array3D::new_from(values);
    assert_eq!(arr.n1(), 3);
    assert_eq!(arr.n2(), 4);
    assert_eq!(arr.n3(), 2);
    assert_eq!(arr.num_elements(), 24);

    assert_eq!(arr.data(0, 0, 0), &1.0);
    assert_eq!(arr.data(0, 0, 1), &2.0);
    assert_eq!(arr.data(0, 1, 0), &3.0);
    assert_eq!(arr.data(0, 3, 1), &8.0);
    assert_eq!(arr.data(1, 0, 0), &9.0);
    assert_eq!(arr.data(1, 1, 1), &12.0);
    assert_eq!(arr.data(2, 0, 0), &17.0);
    assert_eq!(arr.data(2, 1, 1), &20.0);
    assert_eq!(arr.data(2, 2, 0), &21.0);
    assert_eq!(arr.data(2, 3, 1), &24.0);
  }

  #[test]
  fn test_fill() {
    let mut full_of_7 = Array3D::new_fill_value(2, 3, 4, 7);
    for n1 in 0..full_of_7.n1() {
      for n2 in 0..full_of_7.n2() {
        for n3 in 0..full_of_7.n3() {
          assert_eq!(full_of_7.data(n1, n2, n3), &7);
        }
      }
    }

    full_of_7.fill(11);
    for n1 in 0..full_of_7.n1() {
      for n2 in 0..full_of_7.n2() {
        for n3 in 0..full_of_7.n3() {
          assert_eq!(full_of_7.data(n1, n2, n3), &11);
        }
      }
    }
  }
}