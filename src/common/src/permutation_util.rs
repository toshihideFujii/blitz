#![allow(dead_code)]

pub fn permute() {}

pub fn permute_inverse<T>() -> Vec<T> {
  unimplemented!()
}

pub fn inverse_permutation(_input_permutation: &Vec<i64>) -> Vec<i64> {
  unimplemented!()
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