// Returns true if permutation is a permutation of the integers
// [0, permutation.size()).
pub fn is_permutation(permutation: &Vec<i64>) -> bool {
  let mut seen = vec![false; permutation.len()];
  for p in permutation {
    if *p < 0 || *p as usize >= permutation.len()|| seen[*p as usize] == true {
      return false;
    }
    seen[*p as usize] = true;
  }
  true
}

// Applies `permutation` on `input` and returns the permuted array.
// For each i, output[i] = input[permutation[i]].
//
// Precondition:
// 1. `permutation` is a permutation of 0..permutation.size()-1.
// 2. permutation.size() == input.size().
pub fn permute() {

}

pub fn permute_inverse<T>(input: &Vec<T>, permutation: &Vec<i64>) -> Vec<T>
  where T: Clone + Default
{
  debug_assert!(permutation.len() == input.len());
  debug_assert!(is_permutation(&permutation));

  let mut output = vec![];
  output.resize(input.len(), T::default());
  for i in 0..permutation.len() {
    output[permutation[i] as usize] = input[i].clone();
  }
  output
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

pub fn compose_permutations(p1: &Vec<i64>, p2: &Vec<i64>) -> Vec<i64> {
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

pub fn move_single_element(permutation: &mut Vec<i64>, from: i64, to: i64) {
  if from < to {
    let value = permutation[from as usize];
    permutation.insert((to+1) as usize, value);
    permutation.remove(from as usize);
  } else if from > to {
    let value = permutation[from as usize];
    permutation.insert(to as usize, value);
    permutation.remove((from+1) as usize);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_permutation_true_cases() {
    assert!(is_permutation(&vec![]));
    assert!(is_permutation(&vec![0]));
    assert!(is_permutation(&vec![0, 1]));
    assert!(is_permutation(&vec![1, 0]));
    assert!(is_permutation(&vec![3, 1, 0, 2]));
  }

  #[test]
  fn test_is_permutation_false_cases() {
    assert!(!is_permutation(&vec![-3]));
    assert!(!is_permutation(&vec![1, 1]));
    assert!(!is_permutation(&vec![3, 0, 2]));
  }

  #[test]
  fn test_is_identity_permutation_true_cases() {
    assert!(is_identity_permutation(&vec![]));
    assert!(is_identity_permutation(&vec![0]));
    assert!(is_identity_permutation(&vec![0, 1]));
    assert!(is_identity_permutation(&vec![0, 1, 2]));
    assert!(is_identity_permutation(&vec![0, 1, 2, 3]));
  }

  #[test]
  fn test_permute_inverse() {
    let result: Vec<&str> = vec![];
    assert_eq!(permute_inverse::<&str>(&vec![], &vec![]),
      result);
    assert_eq!(permute_inverse::<&str>(&vec!["a", "b", "c"], &vec![0, 1, 2]),
      vec!["a", "b", "c"]);
    assert_eq!(permute_inverse::<&str>(&vec!["a", "b", "c"], &vec![2, 1, 0]),
      vec!["c", "b", "a"]);
    assert_eq!(permute_inverse::<&str>(&vec!["a", "b", "c"], &vec![2, 0, 1]),
      vec!["b", "c", "a"]);
  }

  #[test]
  fn test_inverse_permutation() {
    assert_eq!(inverse_permutation(&vec![]), vec![]);
  }

  #[test]
  fn test_compose_permutations() {
    assert_eq!(compose_permutations(&vec![0, 1, 2], &vec![1, 2, 0]),
      vec![1, 2, 0]);
    assert_eq!(compose_permutations(&vec![1, 2, 0], &vec![0, 1, 2]),
      vec![1, 2, 0]);
    assert_eq!(compose_permutations(&vec![1, 3, 2, 0], &vec![2, 1, 3, 0]),
      vec![2, 3, 0, 1]);
  }

  #[test]
  fn test_move_single_element() {
    let mut p1 = vec![0, 1, 2, 3, 4];
    move_single_element(&mut p1, 1, 3);
    assert_eq!(p1, vec![0, 2, 3, 1, 4]);

    let mut p2 = vec![0, 1, 2, 3, 4];
    move_single_element(&mut p2, 3, 1);
    assert_eq!(p2, vec![0, 3, 1, 2, 4]);
  }
}