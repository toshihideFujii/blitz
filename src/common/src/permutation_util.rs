#![allow(dead_code)]

// Returns true if permutation is a permutation of the integers
// [0, permutation.size()).
pub fn is_permutation(permutation: &Vec<i64>) -> bool {
  let mut seen = vec![false; permutation.len()];
  for p in permutation {
    if *p < 0 || *p as usize > permutation.len()|| seen[*p as usize] == true {
      return false;
    }
    seen[*p as usize] = true;
  }
  true
}

pub fn permute() {}

pub fn permute_inverse<T>() -> Vec<T> {
  unimplemented!()
}

// Inverts a permutation, i.e., output_permutation[input_permutation[i]] = i.
pub fn inverse_permutation(input_permutation: &Vec<i64>) -> Vec<i64> {
  assert!(is_permutation(input_permutation));

  let mut output_permutation: Vec<i64> = vec![-1; input_permutation.len()];
  for i in 0..input_permutation.len() {
    output_permutation[input_permutation[i] as usize] = i as i64;
  }
  output_permutation
}

pub fn compose_permutation(p1: Vec<i64>, p2: Vec<i64>) -> Vec<i64> {
  debug_assert!(p1.len() == p2.len());
  let mut output = vec![];
  for i in 0..p1.len() {
    output.push(p1[p2[i] as usize]);
  }
  output
}

pub fn is_identity_permutation(permutation: &Vec<i64>) -> bool {
  for i in 0..permutation.len() {
    if permutation[i] as usize != i { return false; }
  }
  true
}