#![allow(dead_code)]

use std::{collections::HashSet};

struct DeltaAlgorithm {
  failed_tests_cache: Vec<HashSet<u32>>//HashSet<HashSet<u32>>
}

impl DeltaAlgorithm {
  pub fn new() {}

  // Minimize the set changes by executing execution_one_test()
  // on subsets of changes and returning the smallest set which
  // still satisfies the test predicate.
  pub fn run() {}

  // Get the test result for the changes from the cache,
  // executing the test if necessary.
  fn get_test_result(&mut self, changes: &HashSet<u32>) -> bool {
    if self.failed_tests_cache.contains(changes) {
      return false;
    }
    let result = self.execute_one_test(changes);
    if !result {
      self.failed_tests_cache.push(changes.clone());
    }
    result
  }

  // Partition a set of changes s into one or two subsets.
  fn split() {}

  // Minimize a set of changes which has been partitioned into
  // smaller sets, by attempting to remove individual subsets.
  fn delta() {}

  // Search for a subset in sets which can be removed from changes
  // while still satisfying the predicate.
  fn search() {}

  fn update_search_state() {}

  // Execute a single test predicate on the change sest s.
  fn execute_one_test(&self, _s: &HashSet<u32>) -> bool {
    true
  }
}