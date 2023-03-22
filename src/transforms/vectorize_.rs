#![allow(dead_code)]

// This file defines prototypes for accessor functions
// that expose passes in the Vectorize transformations
// library.

struct VectorizeConfig {
  // The size of the native vector registers.
  vector_bits: bool,
  // Vectorize boolean values.
  vectorize_bools: bool,
  // Vectorize integer values.
  vectorize_ints: bool,
  // Vectorize floating-point values.
  vectorize_floats: bool,
  // Vectorize pointer values.
  vectorize_pointers: bool,
  // Vectorize casting (conversion) operations.
  vectorize_casts: bool,
  // Vectorize floating-point math intrinsics.
  vectorize_math: bool,
  // Vectorize bit intrinsics.
  vectorize_bit_manipulations: bool,
  // Vectorize the fused-multiply-add intrinsic.
  vectorize_fma: bool,
  // Vectorize select instructions.
  vectorize_select: bool,
  // Vectorize comparison instructions.
  vectorize_cmp: bool,
  // Vectorize getelementptr instructions.
  vectorize_gep: bool,
  // Vectorize loads and stores.
  vectorize_mem_ops: bool,
  // Only generate aligned loads and stores.
  aligned_only: bool,
  // The required chain depth for vectorization.
  req_chain_depth: u32,
  // The maximum search distance for instruction pairs.
  search_limit: u32,
  // The maximum number of candidaet pairs with which to use
  // a full cycle check.
  max_cand_pairs_for_cycle_check: u32,
  // Replicating one element to a pair breaks the chain.
  splat_breaks_chain: bool,
  // The maximum number of pairable instructions per group.
  max_insts: u32,
  // The maximum number of candidate instruction pairs per group.
  max_pairs: u32,
  // The maximum number of pairing iterations.
  max_iter: u32,
  // Don't try to form odd-length vectors.
  pow2_len_only: bool,
  // Don't boost the chain-depth contribution of loads and stores.
  no_mem_op_boost: bool,
  // Use a fast instruction dependency analysis.
  fast_dep: bool,
}

pub fn create_loop_vectorize_pass() {}

pub fn create_slp_vectorizer_pass() {}

pub fn vectorize_basic_block() {}

pub fn create_load_store_vectorizer_pass() {}

pub fn create_vector_combine_pass() {}