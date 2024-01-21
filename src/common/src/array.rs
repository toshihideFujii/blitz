#![allow(dead_code)]

use std::ops::Index;
use rand::Rng;

pub fn all_inside_range() {}

pub struct OwnedBuffer<D> {
  pub data: Vec<D>,
}

impl<D> OwnedBuffer<D> {
  pub fn new(data: Vec<D>) -> Self {
    OwnedBuffer {
      data: data
    }
  }
}

impl<D> Index<i64> for OwnedBuffer<D> {
  type Output = D;
  fn index(&self, index: i64) -> &Self::Output {
    &self.data[index as usize]
  }
}

pub struct Array<T> {
  sizes: OwnedBuffer<i64>,
  values: OwnedBuffer<T>,
  //sizes_vec: Option<Vec<Vec<i64>>>
}

impl<T> Array<T> where T: Clone {
  pub fn new_fill_value(sizes: Vec<i64>, value: T) -> Self {
    let mut mul = 1;
    for i in sizes.iter() {
      mul *= i;
    }
    let mut values = Vec::new();
    values.resize(mul as usize, value);

    Array {
      sizes: OwnedBuffer::new(sizes),
      values: OwnedBuffer::new(values),
      //sizes_vec: None,
    }
  }

  pub fn fill(&mut self, value: T) {
    for i in 0..self.values.data.len() {
      self.values.data[i] = value.clone();
    }
  }

  pub fn fill_iota(&mut self, _value: T) {}

  pub fn fill_repeated_iota() {}
  pub fn fill_with_multiples() {}
  pub fn fill_random() {}
  pub fn fill_random_double() {}

  pub fn fill_random_uniform(&mut self) {}

  pub fn set_values() {}
  pub fn each() {}
  pub fn each_status() {}

  pub fn data(&self) -> &Vec<T> {
    &self.values.data
  }

  pub fn dim(&self, n: i64) -> i64 {
    self.sizes[n]
  }

  pub fn dimensions(&self) -> &Vec<i64> {
    &self.sizes.data
  }

  pub fn num_dimensions(&self) -> usize {
    self.sizes.data.len()
  }

  pub fn num_elements(&self) -> usize {
    self.values.data.len()
  }

  pub fn slice() {}
  pub fn update_slice() {}
  pub fn reshape() {}
  pub fn transpose_dimensions() {}
  pub fn absl_has_value() {}
  pub fn to_string() {}

  fn to_i64_array() {}
  fn calculate_index() {}
  fn next_index() {}
  fn calculate_elements() {}
}

impl Array<i64> {
  pub fn new_i64(sizes: Vec<i64>) -> Self {
    let mut mul = 1;
    for i in sizes.iter() {
      mul *= i;
    }
    let values = vec![0; mul as usize];
    Array {
      sizes: OwnedBuffer::new(sizes),
      values: OwnedBuffer::new(values),
      //sizes_vec: None,
    }
  }

  pub fn value(&self, vec: Vec<i64>) -> i64 {
    if vec.len() == 1 {
      return self.values.data[vec[0] as usize];
    } else if vec.len() == 2 {
      let pos = (vec[0] + vec[1]) as usize;
      return self.values.data[pos];
    } else if vec.len() == 3 {
      let pos = (vec[0] + vec[1] + vec[2]) as usize;
      return self.values.data[pos];
    }
    -1
  }
}

impl Array<bool> {
  pub fn fill_random_bool(&mut self) {
    let mut rng = rand::thread_rng();
    for i in 0..self.values.data.len() {
      self.values.data[i] = rng.gen_bool(1.0 / 2.0);
    }
  }
}

pub trait Arr<T> {
  fn dim(&self, n: usize) -> usize;
  fn value_1(&self, n1: usize) -> T;
  fn value_2(&self, n1: usize, n2: usize) -> T;
  fn set_value_2(&mut self, _n1: usize, _n2: usize, _v: T) {}

  //fn dimensions(&self) -> &Vec<usize>;
  fn num_dimensions(&self) -> usize;
  fn num_elements(&self) -> usize;
}

pub struct ArrVecI64 {
  //sizes: Vec<usize>,
  values: Vec<i64>
}

impl ArrVecI64 {
  pub fn new(values: Vec<i64>) -> Self {
    //let mut sizes = Vec::new();
    //sizes.push(values.len());
    ArrVecI64 {
      //sizes: sizes,
      values: values
    }
   } 
}

impl Arr<i64> for ArrVecI64 {
  fn dim(&self, n: usize) -> usize {
    self.values[n] as usize
  }

  fn value_1(&self, n1: usize) -> i64 {
    self.values[n1]
  }

  fn value_2(&self, _n1: usize, _n2: usize) -> i64 {
    unimplemented!("Unsupported");
  }

  //fn dimensions(&self) -> &Vec<usize> {
    //let mut dimensions = Vec::new();
    //for i in &self.values {
      //dimensions.push(*i as usize);
    //}
    //&dimensions
  //}

  fn num_dimensions(&self) -> usize {
    self.values.len()
  }

  fn num_elements(&self) -> usize {
    self.values.len()
  }
}

pub struct ArrV2<T> {
  sizes: Vec<usize>,
  values: Vec<Vec<T>>,
}

impl<T> ArrV2<T> {
  pub fn new(values: Vec<Vec<T>>) -> Self {
    let mut sizes = Vec::new();
    sizes.push(values.len());
    sizes.push(values[0].len());
    ArrV2 {
      sizes: sizes,
      values: values
    }
  }
}

impl<T> Arr<T> for ArrV2<T> where T: Copy + Clone {
  fn dim(&self, n: usize) -> usize {
    self.sizes[n]
  }

  fn value_1(&self, _n1: usize) -> T {
    unimplemented!("Unsupported.");
  }

  fn value_2(&self, n1: usize, n2: usize) -> T {
    self.values[n1][n2]
  }

  fn set_value_2(&mut self, n1: usize, n2: usize, v: T) {
    self.values[n1][n2] = v;
  }

  //fn dimensions(&self) -> &Vec<usize> {
    //&self.sizes
  //}

  fn num_dimensions(&self) -> usize {
    self.sizes.len()
  }

  fn num_elements(&self) -> usize {
    let mut num = 0;
    for i in 0..self.values.len() {
      num += self.values[i].len();
    }
    num
  }
}

pub struct ArrV3F64 {
  sizes: Vec<usize>,
  values: Vec<Vec<Vec<f64>>>,
}

impl ArrV3F64 {
  pub fn new(values: Vec<Vec<Vec<f64>>>) -> Self {
    let mut sizes = Vec::new();
    sizes.push(values.len());
    sizes.push(values[0].len());
    sizes.push(values[0][0].len());
    ArrV3F64 {
      sizes: sizes,
      values: values
    }
  }
}

impl Arr<f64> for ArrV3F64 {
  fn dim(&self, n: usize) -> usize {
    self.sizes[n]
  }

  fn value_1(&self, _n1: usize) -> f64 {
    unimplemented!("Unsupported.");
  }

  fn value_2(&self, _n1: usize, _n2: usize) -> f64 {
    //self.values[n1][n2]
    unimplemented!("Unsupported.");
  }

  fn num_dimensions(&self) -> usize {
    self.sizes.len()
  }

  fn num_elements(&self) -> usize {
    let mut num = 0;
    for i in 0..self.values.len() {
      num += self.values[i].len();
    }
    num
  }
}

pub struct ArrV4F64 {
  sizes: Vec<usize>,
  values: Vec<Vec<Vec<Vec<f64>>>>,
}

impl ArrV4F64 {
  pub fn new(values: Vec<Vec<Vec<Vec<f64>>>>) -> Self {
    let mut sizes = Vec::new();
    sizes.push(values.len());
    sizes.push(values[0].len());
    sizes.push(values[0][0].len());
    sizes.push(values[0][0][0].len());
    ArrV4F64 {
      sizes: sizes,
      values: values
    }
  }
}

impl Arr<f64> for ArrV4F64 {
  fn dim(&self, n: usize) -> usize {
    self.sizes[n]
  }

  fn value_1(&self, _n1: usize) -> f64 {
    unimplemented!("Unsupported.");
  }

  fn value_2(&self, _n1: usize, _n2: usize) -> f64 {
    //self.values[n1][n2]
    unimplemented!("Unsupported.");
  }

  //fn dimensions(&self) -> &Vec<usize> {
    //&self.sizes
  //}

  fn num_dimensions(&self) -> usize {
    self.sizes.len()
  }

  fn num_elements(&self) -> usize {
    let mut num = 0;
    for i in 0..self.values.len() {
      num += self.values[i].len();
    }
    num
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_uninitialized_dims_ctor() {
    //let uninit = Array::new_i64(vec![2, 3]);
    let uninit = ArrVecI64::new(vec![2, 3]);
    assert_eq!(uninit.num_dimensions(), 2);
    assert_eq!(uninit.dim(0), 2);
    assert_eq!(uninit.dim(1), 3);
    assert_eq!(uninit.num_elements(), 6); // <- ?????
  }
/*
  #[test]
  fn test_fill_ctor() {
    let full_of_7: Array<i64> = Array::new_fill_value(vec![1, 2, 3], 7);
    assert_eq!(full_of_7.dim(0), 1);
    assert_eq!(full_of_7.dim(1), 2);
    assert_eq!(full_of_7.dim(2), 3);

    for n0 in 0..full_of_7.dim(0) {
      for n1 in 0..full_of_7.dim(1) {
        for n2 in 0..full_of_7.dim(2) {
          assert_eq!(full_of_7.value(vec![n0, n1, n2]), 7);
        }
      }
    }
  }
*/
  #[test]
  fn test_initializer_list_ctor() {
    let arr_vec: Vec<Vec<i64>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let arr = ArrV2::new(arr_vec);
    assert_eq!(arr.dim(0), 2);
    assert_eq!(arr.dim(1), 3);

    assert_eq!(arr.value_2(0, 0), 1);
    assert_eq!(arr.value_2(0, 1), 2);
    assert_eq!(arr.value_2(0, 2), 3);
    assert_eq!(arr.value_2(1, 0), 4);
    assert_eq!(arr.value_2(1, 1), 5);
    assert_eq!(arr.value_2(1, 2), 6);
  }

  #[test]
  fn test_initializer_list_ctor_half() {
    let d2_vec: Vec<Vec<f64>> = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let d2 = ArrV2::new(d2_vec);
    assert_eq!(d2.dim(0), 2);
    assert_eq!(d2.dim(1), 3);

    let d3_vec = vec![vec![vec![1.0],vec![4.0]], vec![vec![1.0], vec![4.0]], vec![vec![1.0], vec![4.0]]];
    let d3 = ArrV3F64::new(d3_vec);
    assert_eq!(d3.dim(0), 3);
    assert_eq!(d3.dim(1), 2);
    assert_eq!(d3.dim(2), 1);

    let d4_vec = vec![
      vec![vec![vec![1.0], vec![4.0]], vec![vec![1.0], vec![4.0]], vec![vec![1.0], vec![4.0]]],
      vec![vec![vec![1.0], vec![4.0]], vec![vec![1.0], vec![4.0]], vec![vec![1.0], vec![4.0]]]
    ];
    let d4 = ArrV4F64::new(d4_vec);
    assert_eq!(d4.dim(0), 2);
    assert_eq!(d4.dim(1), 3);
    assert_eq!(d4.dim(2), 2);
    assert_eq!(d4.dim(3), 1);
  }

  #[test]
  fn test_indexing_read_write_bool() {
    let values = vec![vec![false, true, false], vec![false, true, false]];
    let mut arr = ArrV2::new(values);

    assert_eq!(arr.value_2(0, 1), true);
    assert_eq!(arr.value_2(0, 2), false);

    arr.set_value_2(0, 1, false);
    arr.set_value_2(0, 2, true);
    assert_eq!(arr.value_2(0, 1), false);
    assert_eq!(arr.value_2(0, 2), true);
  }
}
