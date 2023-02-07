#![allow(dead_code)]

// General N dimensional array class with arbitrary value type.
struct Array {}

impl Array {
  pub fn fill() {}

  // Fills the array with the sequence i * multiplier (i=0,1,...)
  pub fn fill_with_multiples() {}

  // Fills the array with random normal vatiables with the specified mean.
  pub fn fill_random() {}

  pub fn fill_random_uniform() {}

  pub fn fill_random_bool() {}

  // Sets all the values in the array to values specified in the container.
  pub fn set_values() {}

  pub fn each() {}

  pub fn each_status() {}

  pub fn data() {}

  // Returns the size of the dimension at the given index.
  pub fn dim() {}

  // Returns a vector containing the dimensions of the array.
  pub fn dimensions() {}

  pub fn num_dimensions() {}

  // Returns the total number of elements in the array.
  pub fn num_elements() {}

  pub fn slice() {}

  pub fn update_slice() {}

  pub fn reshape() {}

  // Performs a permutation of dimensions.
  pub fn transpose_dimensions() {}

  pub fn to_string() {}
}