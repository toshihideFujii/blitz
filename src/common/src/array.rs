#![allow(dead_code)]

use std::ops::Index;
//use rand::Rng;

pub fn all_inside_range() {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

// General N dimensional array class with arbitrary value type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Array<T> {
  //sizes: Vec<T>, //OwnedBuffer<i64>,
  values_2d: Option<Vec<Vec<T>>>, //OwnedBuffer<T>,
  //sizes_vec: Option<Vec<Vec<i64>>>
}

impl<T> Array<T> where T: Clone {
  pub fn new_fill_value(_sizes: Vec<i64>, _value: T) -> Self {
    /*
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
    */
    unimplemented!()
  }

  // Creates a 2D array from the given nested initializer list. The outer
  // initializer list is the first dimension, the inner is the second dimension.
  // For example, {{1, 2, 3}, {4, 5, 6}} results in an array with n1=2 and n2=3.
  pub fn new_2d(values_2d: Vec<Vec<T>>) -> Self {
    Array { values_2d: Some(values_2d) }
  }

  // Fills the array with the specified value.
  pub fn fill(&mut self, value: T) {
    if self.values_2d.is_some() {
      for vec in self.values_2d.as_mut().unwrap() {
        vec.fill(value.clone());
      }
    }
  }

  // Fills the array with sequentially increasing values.
  pub fn fill_iota(&mut self, _value: T) {
    unimplemented!()
  }

  pub fn fill_repeated_iota() {}
  pub fn fill_with_multiples() {}
  pub fn fill_random() {}
  pub fn fill_random_double() {}

  pub fn fill_random_uniform(&mut self) {}

  pub fn set_values() {}
  pub fn each() {}
  pub fn each_status() {}

  pub fn data_2d(&self) -> &Vec<Vec<T>> {
    assert!(self.values_2d.is_some());
    self.values_2d.as_ref().unwrap()
  }

  pub fn dim(&self, _n: i64) -> i64 {
    //self.sizes[n]
    unimplemented!()
  }

  pub fn dimensions(&self) -> &Vec<i64> {
    //&self.sizes.data
    unimplemented!()
  }

  pub fn num_dimensions(&self) -> usize {
    //self.sizes.data.len()
    unimplemented!()
  }

  pub fn num_elements(&self) -> usize {
    //self.values.data.len()
    unimplemented!()
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
  pub fn new_i64(_sizes: Vec<i64>) -> Self {
    /*
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
    */
    unimplemented!()
  }

  pub fn value(&self, _vec: Vec<i64>) -> i64 {
    /*
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
    */
    unimplemented!()
  }
}

impl Array<bool> {
  pub fn fill_random_bool(&mut self) {
    /*
    let mut rng = rand::thread_rng();
    for i in 0..self.values.data.len() {
      self.values.data[i] = rng.gen_bool(1.0 / 2.0);
    }
    */
    unimplemented!()
  }
}