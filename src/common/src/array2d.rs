use crate::util::log_2_ceiling;

#[derive(Debug, Clone, PartialEq)]
pub struct Array2D<T> {
  values: Vec<Vec<T>>
}

impl<T> Array2D<T> where T: Default + Clone {
  pub fn default() -> Self {
    let mut instance = Array2D { values: vec![vec![]] };
    instance.values.resize(0, vec![T::default(); 0]);
    instance
  }

  pub fn new(n1: usize, n2: usize) -> Self {
    let mut instance = Array2D { values: vec![vec![]] };
    instance.values.resize(n1, vec![T::default(); n2]);
    for i in 0..n1 {
      let vec = &mut instance.values[i];
      vec.resize(n2, T::default());
    }
    instance
  }

  pub fn new_fill_value(n1: usize, n2: usize, value: T) -> Self {
    let mut instance = Array2D::new(n1, n2);
    instance.fill(value);
    instance
  }

  pub fn new_from(values: Vec<Vec<T>>) -> Self {
    Array2D { values: values }
  }

  pub fn n1(&self) -> usize {
    self.values.len()
  }

  pub fn n2(&self) -> usize {
    if self.values.len() == 0 { return 0; }
    self.values[0].len()
  }

  pub fn height(&self) -> usize {
    self.values.len()
  }

  pub fn width(&self) -> usize {
    if self.values.len() == 0 { return 0; }
    self.values[0].len()
  }

  pub fn num_elements(&self) -> usize {
    self.n1() * self.n2()
  }

  // Fills the array with the specified value.
  pub fn fill(&mut self, value: T) {
    for i0 in 0..self.n1() {
      for i1 in 0..self.n2() {
        self.values[i0][i1] = value.clone();
      }
    }
  }
  
  // Fills the array with a pattern of values of the form:
  //    (rowno << log2ceil(width) | colno) + start_value
  // This makes it easy to see distinct row/column values in the array.
  pub fn fill_unique(&mut self, _start_value: T) {
    let _shift = log_2_ceiling(self.n2()) as usize;
    for _i0 in 0..self.n1() {
      for _i1 in 0..self.n2() {
        //self.values[i0][i1] = ((i0 << shift) | i1) + start_value;
      }
    }
  }

  // Applies f to all cells in this array, in row-major order.
  pub fn each(&self, f: &dyn Fn(usize, usize, &T)) {
    for i0 in 0..self.n1() {
      for i1 in 0..self.n2() {
        f(i0, i1, &self.values[i0][i1]);
      }
    }
  } 

  pub fn data(&self, n1: usize, n2: usize) -> &T {
    &self.values[n1][n2]
  }

  pub fn set_data(&mut self, n1: usize, n2: usize, data: T) {
    self.values[n1][n2] = data;
  }
}

pub fn make_linspace_array2d(from: f64, to: f64, n1: usize, n2: usize) -> Array2D<f64> {
  let mut array = Array2D::new(n1, n2);
  let count = n1 * n2;
  let mut step: f64 = 0.0;
  if count > 1 { step = (to - from) / (count - 1) as f64; }

  let mut set = |index: usize, value: f64| {
    array.set_data(index / n2, index % n2, value);
  };

  for i in 0..count - 1 {
    set(i, from + (i as f64) * step);
  }
  set(count - 1, to);
  array
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default_ctor() {
    let empty: Array2D<i64> = Array2D::default();
    assert_eq!(empty.n1(), 0);
    assert_eq!(empty.n2(), 0);
    assert_eq!(empty.num_elements(), 0);
  }

  #[test]
  fn test_uninitialized_dims_ctor() {
    let uninit: Array2D<i64> = Array2D::new(2, 3);
    assert_eq!(uninit.n1(), 2);
    assert_eq!(uninit.n2(), 3);
    assert_eq!(uninit.num_elements(), 6);
  }

  #[test]
  fn test_fill_ctor() {
    let full_of_7 = Array2D::new_fill_value(2, 3, 7);
    assert_eq!(full_of_7.n1(), 2);
    assert_eq!(full_of_7.n2(), 3);
    for n1 in 0..full_of_7.n1() {
      for n2 in 0..full_of_7.n2() {
        assert_eq!(full_of_7.values[n1][n2], 7);
      }
    }
  }

  #[test]
  fn test_initializer_list_ctor() {
    let values = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let arr = Array2D::new_from(values);

    assert_eq!(arr.n1(), 2);
    assert_eq!(arr.n2(), 3);

    assert_eq!(arr.values[0][0], 1);
    assert_eq!(arr.values[0][1], 2);
    assert_eq!(arr.values[0][2], 3);
    assert_eq!(arr.values[1][0], 4);
    assert_eq!(arr.values[1][1], 5);
    assert_eq!(arr.values[1][2], 6);
  }

  #[test]
  fn test_initializer_list_ctor_f64() {
    let values = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let arr = Array2D::new_from(values);

    assert_eq!(arr.n1(), 2);
    assert_eq!(arr.n2(), 3);

    assert_eq!(arr.values[0][0], 1.0);
    assert_eq!(arr.values[0][1], 2.0);
    assert_eq!(arr.values[0][2], 3.0);
    assert_eq!(arr.values[1][0], 4.0);
    assert_eq!(arr.values[1][1], 5.0);
    assert_eq!(arr.values[1][2], 6.0);
  }

  #[test]
  fn test_accessors() {
    let values = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let arr = Array2D::new_from(values);

    assert_eq!(arr.n1(), 2);
    assert_eq!(arr.n2(), 3);
    assert_eq!(arr.height(), 2);
    assert_eq!(arr.width(), 3);
    assert_eq!(arr.num_elements(), 6);
  }

  #[test]
  fn test_indexing_read_write() {
    let values = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut arr = Array2D::new_from(values);

    assert_eq!(arr.data(1, 1), &5);
    assert_eq!(arr.data(1, 2), &6);
    arr.set_data(1, 1, 51);
    arr.set_data(1, 2, 61);
    assert_eq!(arr.data(1, 1), &51);
    assert_eq!(arr.data(1, 2), &61);
  }

  #[test]
  fn test_indexing_read_write_bool() {
    let values = vec![vec![false, true, false], vec![true, true, false]];
    let mut arr = Array2D::new_from(values);

    assert_eq!(arr.data(1, 1), &true);
    assert_eq!(arr.data(1, 2), &false);
    arr.set_data(1, 1, false);
    arr.set_data(1, 2, true);
    assert_eq!(arr.data(1, 1), &false);
    assert_eq!(arr.data(1, 2), &true);
  }

  #[test]
  fn test_fill() {
    let mut full_of_7 = Array2D::new_fill_value(2, 3, 7);
    for n1 in 0..full_of_7.n1() {
      for n2 in 0..full_of_7.n2() {
        assert_eq!(full_of_7.data(n1, n2), &7);
      }
    }

    full_of_7.fill(11);
    for n1 in 0..full_of_7.n1() {
      for n2 in 0..full_of_7.n2() {
        assert_eq!(full_of_7.data(n1, n2), &11);
      }
    }
  }

  #[test]
  fn test_linspace() {
    let arr = make_linspace_array2d(1.0, 3.5, 3, 2);
    assert_eq!(arr.n1(), 3);
    assert_eq!(arr.n2(), 2);

    assert_eq!(arr.data(0, 0), &1.0);
    assert_eq!(arr.data(0, 1), &1.5);
    assert_eq!(arr.data(1, 0), &2.0);
    assert_eq!(arr.data(1, 1), &2.5);
    assert_eq!(arr.data(2, 0), &3.0);
    assert_eq!(arr.data(2, 1), &3.5);
  }

  #[test]
  fn test_equals() {
    let arr0 = Array2D::new_from(
      vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
    let arr1 = Array2D::new_from(
      vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
    assert_eq!(arr0 == arr1, true);
    assert_eq!(arr0 != arr1, false);
    assert_eq!(arr1 == arr0, true);
    assert_eq!(arr1 != arr0, false);

    let arr2 = Array2D::new_from(
      vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]);
    assert_eq!(arr0 != arr2, true);
    assert_eq!(arr0 == arr2, false);

    let arr3 = Array2D::new_from(
      vec![vec![1, 2, 3], vec![4, 5, 6]]);
    assert_eq!(arr0 != arr3, true);
    assert_eq!(arr0 == arr3, false);

    let arr4 = Array2D::new_from(
      vec![vec![1, 2], vec![3, 4]]);
    assert_eq!(arr0 != arr4, true);
    assert_eq!(arr0 == arr4, false);

    let arr5 = Array2D::new_from(
      vec![vec![1, 2], vec![13, 4], vec![5, 6]]);
    assert_eq!(arr0 != arr5, true);
    assert_eq!(arr0 == arr5, false);

    let bool_arr0 = Array2D::new_from(
      vec![vec![false], vec![true]]);
    let bool_arr1 = Array2D::new_from(
      vec![vec![false], vec![true]]);
    assert_eq!(bool_arr0 == bool_arr1, true);
    assert_eq!(bool_arr0 != bool_arr1, false);

    let bool_arr2 = Array2D::new_from(
    vec![vec![false], vec![false]]);
    assert_eq!(bool_arr0 == bool_arr2, false);
    assert_eq!(bool_arr0 != bool_arr2, true);
  }
}