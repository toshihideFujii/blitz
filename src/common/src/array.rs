
// General N dimensional array class with arbitrary value type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Array {
  sizes: Vec<i64>,
  values: Vec<i64>
}

impl Array {
  pub fn new(sizes: Vec<i64>) -> Self {
    let mut num_elements = 0;
    if sizes.len() > 0 {
      num_elements = 1;
      for i in 0..sizes.len() {
        num_elements *= sizes[i as usize];
      }
    }
    Array { sizes: sizes, values: vec![0; num_elements as usize] }
  }

  pub fn new_fill(sizes: Vec<i64>, value: i64) -> Self {
    let mut instance = Array::new(sizes);
    instance.values = vec![value; instance.num_elements()];
    instance
  }

  // Creates a 2D array from the given nested initializer list. The outer
  // initializer list is the first dimension, the inner is the second dimension.
  // For example, {{1, 2, 3}, {4, 5, 6}} results in an array with n1=2 and n2=3.
  //pub fn new_2d(values_2d: Vec<Vec<T>>) -> Self {
    //Array { values_2d: Some(values_2d) }
  //}

  // Fills the array with the specified value.
  pub fn fill(&mut self, value: i64) {
    for i in 0..self.values.len() {
      self.values[i] = value;
    }
  }

  // Fills the array with sequentially increasing values.
  pub fn fill_iota(&mut self, value: i64) {
    let mut v = value;
    for i in 0..self.values.len() {
      self.values[i] = v;
      v += 1; 
    }
  }

  pub fn fill_repeated_iota() {}

  // Fills the array with the sequence i*multiplier for i=0,1,...
  pub fn fill_with_multiples(&mut self, multiplier: i64) {
    for i in 0..self.num_elements() {
      self.values[i] = (i as i64) * multiplier;
    }
  }

  pub fn fill_random() {}
  pub fn fill_random_double() {}
  pub fn fill_random_uniform(&mut self) {}

  pub fn set_value(&mut self, pos: &Vec<i64>, value: i64) {
    assert!(pos.len() == self.sizes.len());
    for i in 0..self.sizes.len() {
      assert!(pos[i] <= self.sizes[i]);
    }
    let mut v_pos = 0;
    for i in 0..pos.len() {
      v_pos += pos[i];
    }
    self.values[v_pos as usize] = value;
  }

  pub fn value_at(&self, pos: &Vec<i64>) -> i64 {
    assert!(pos.len() == self.sizes.len());
    for i in 0..self.sizes.len() {
      assert!(pos[i] <= self.sizes[i]);
    }
    let mut v_pos = 0;
    for i in 0..pos.len() {
      v_pos += pos[i];
    }
    self.values[v_pos as usize]
  }

  pub fn values(&self) -> &Vec<i64> {
    &self.values
  }

  // Invokes a callback with the (indices, value) for each cell in the array.
  pub fn each<F>(&mut self, func: &mut F) where F: FnMut(&Vec<i64>, &mut i64) {
    for i in 0..self.num_elements() {
      func(&self.value_pos_vec(self.values[i]), &mut self.values[i]);
    }
  }

  pub fn each_status() {}

  // Returns the size of the dimension at the given index.
  pub fn dim(&self, n: usize) -> i64 {
    self.sizes[n]
  }

  // Returns a vector containing the dimensions of the array.
  pub fn dimensions(&self) -> &Vec<i64> {
    &self.sizes
  }

  pub fn num_dimensions(&self) -> usize {
    self.sizes.len()
  }

  // Returns the total number of elements in the array.
  pub fn num_elements(&self) -> usize {
    if self.sizes.is_empty() { return 0; }

    let mut num_elements = 1;
    for size in &self.sizes {
      num_elements *= *size;
    }
    num_elements as usize
  }

  pub fn slice() {}
  pub fn update_slice() {}

  // Performs an in-place reshape, modifying the dimensions but not the
  // underlying data.
  pub fn reshape(&mut self, new_dimensions: &Vec<i64>) {
    let mut new_num_elements = 0;
    if !new_dimensions.is_empty() {
      new_num_elements = 1;
      for dim in new_dimensions {
        new_num_elements *= dim;
      }
    }
    assert_eq!(self.num_elements(), new_num_elements as usize);
    self.sizes.clone_from(new_dimensions);
  }

    // Performs a permutation of dimensions.
  pub fn transpose_dimensions(&mut self, permutation: &Vec<i64>) {
    assert_eq!(self.sizes.len(), permutation.len());
    let mut permuted_dims = vec![];
    for i in 0..permutation.len() {
      permuted_dims.push(self.dim(permutation[i] as usize));
    }
    let mut permuted = Array::new(permuted_dims);
    let mut src_indices = vec![-1; self.sizes.len()];
    let mut func = |indices: &Vec<i64>, value: &mut i64| {
      for i in 0..self.sizes.len() {
        src_indices[permutation[i] as usize] = indices[i];
      }
      *value = src_indices[0]; // TODO
    };
    permuted.each(&mut func);
    *self = permuted;
  }


  pub fn absl_has_value() {}

  // Returns a string representation of the array suitable for debugging.
  pub fn to_string(&self) -> String {
    if self.sizes.is_empty() { return "".to_string(); }
    let mut result = "".to_string();
    result.push_str("[");
    let mut counter = 0;
    for v in &self.values {
      result.push_str(&v.to_string());
      if counter == self.values.len()-1 { break; }
      result.push_str(",");
      counter += 1;
    }
    result.push_str("]");
    result
  }

  #[allow(dead_code)]
  fn to_i64_array(&self) {
    unimplemented!()
  }

  #[allow(dead_code)]
  fn calculate_index(&self) {
    unimplemented!()
  }

  #[allow(dead_code)]
  fn next_index(&self) {
    unimplemented!()
  }

  #[allow(dead_code)]
  fn calculate_elements(&self) {
    unimplemented!()
  }

  fn value_pos_vec(&self, index: i64) -> Vec<i64> {
    let mut result = vec![];
    let mut target = index;
    for i in 0..self.sizes.len() { 
      let mut co = 1;
      for j in i+1..self.sizes.len() {
        co *= self.sizes[j];
      }
      for k in (0..self.sizes[i]).rev() {
        if target >= co * k {
          result.push(k);
          target -= co * k;
          break;
        }
      }
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_uninitialized_dims_ctor() {
    let uninit = Array::new(vec![2, 3]);
    assert_eq!(uninit.num_dimensions(), 2);
    assert_eq!(uninit.dim(0), 2);
    assert_eq!(uninit.dim(1), 3);
    assert_eq!(uninit.num_elements(), 6);
  }

  #[test]
  fn test_fill_ctor() {
    let full_of_7 = Array::new_fill(vec![1, 2, 3], 7);
    assert_eq!(full_of_7.dim(0), 1);
    assert_eq!(full_of_7.dim(1), 2);
    assert_eq!(full_of_7.dim(2), 3);

    for n0 in 0..full_of_7.dim(0) {
      for n1 in 0..full_of_7.dim(1) {
        for n2 in 0..full_of_7.dim(2) {
          assert_eq!(full_of_7.value_at(&vec![n0, n1, n2]), 7);
        }
      }
    }
  }

  #[test]
  fn test_indexing_read_write() {
    let mut arr = Array::new(vec![2, 3]);
    assert_eq!(arr.value_at(&vec![1, 1]), 0);
    assert_eq!(arr.value_at(&vec![1, 2]), 0);

    arr.set_value(&vec![1, 1], 51);
    arr.set_value(&vec![1, 2], 61);
    assert_eq!(arr.value_at(&vec![1, 1]), 51);
    assert_eq!(arr.value_at(&vec![1, 2]), 61);
  }

  #[test]
  fn test_fill() {
    let mut full_of_7 = Array::new_fill(vec![2, 3], 7);
    for n1 in 0..full_of_7.dim(0) {
      for n2 in 0..full_of_7.dim(1) {
        assert_eq!(full_of_7.value_at(&vec![n1, n2]), 7);
      }
    }
    full_of_7.fill(11);
    for n1 in 0..full_of_7.dim(0) {
      for n2 in 0..full_of_7.dim(1) {
        assert_eq!(full_of_7.value_at(&vec![n1, n2]), 11);
      }
    }
  }

  #[test]
  fn test_stringification_empty() {
    let arr = Array::new(vec![]);
    assert_eq!(arr.to_string(), "".to_string());
  }

  #[test]
  fn test_stringification_1d() {
    let arr = Array::new_fill(vec![2], 1);
    assert_eq!(arr.to_string(), "[1,1]".to_string());
  }

  #[test]
  fn test_stringification_empty_1d() {
    let arr = Array::new_fill(vec![0], 0);
    assert_eq!(arr.to_string(), "[]".to_string());
  }

  #[test]
  fn test_each() {
    let mut arr = Array::new(vec![2, 3, 4]);
    arr.fill_with_multiples(1);

    let mut each_count = 0;
    let mut each_sum = 0;
    let mut func = |idx: &Vec<i64>, cell: &mut i64| {
      let lin_idx = idx[0] * 12 + idx[1] * 4 + idx[2];
      assert_eq!(lin_idx, *cell);
      each_count += 1;
      each_sum += *cell;
    };
    arr.each(&mut func);

    assert_eq!(arr.num_elements(), each_count);
    assert_eq!(arr.num_elements() * (arr.num_elements() - 1) / 2, each_sum as usize);
  }
}