#![allow(dead_code)]

// Utility to calculate the size and a few similar metrics
// for a set of basic blocks.
struct CodeMetrics {
  // True if this function contains a call to setjmp or other functions
  // with attribute "returns twice" without having the attribute itself.
  expose_returns_twice_: bool,

  // True if this function calls itself.
  is_recursive_: bool,

  // True if this function cannot be duplicated.
  not_duplicatable_: bool,

  // True if this function contains a call to a convergent function.
  convergent_: bool,

  // True if this function calls alloca.
  uses_dynamic_alloca_: bool,

  // Number of analyzed blocks.
  num_blocks_: u32,

  // Keep track of the number of calls to big functions.
  num_calls_: u32,

  // The number of calls to internal functions with a single caller.
  num_inline_candidates_: u32,

  // How many instructions produce vector values.
  num_vector_insts_: u32,

  // How many ret instructions the blocks contain.
  num_rets_: u32
}

impl CodeMetrics {
  // Add information about a block to the current state.
  pub fn analize_basic_block() {}

  // Collect a function's ephemeral values.
  pub fn collect_ephemeral_values() {}
}