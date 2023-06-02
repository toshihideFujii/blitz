#![allow(dead_code)]

// This file defines the interface for the loop memory dependence
// framework that was ordinally developed  for the loop vectorizer.

struct VectorizeParams {}
impl VectorizeParams {
  pub fn new() {}
  pub fn is_interleave_forced() {}
}

struct MemoryDepChecker {}
impl MemoryDepChecker {
  pub fn new() {}
  pub fn add_access() {}
  pub fn are_deps_safe() {}
  pub fn is_safe_for_vectorization() {}
  pub fn is_safe_for_any_vector_width() {}
  pub fn get_max_safe_dep_dist_bytes() {}
  pub fn get_max_safe_vector_width_in_bits() {}
  pub fn should_retry_with_runtime_check() {}
  pub fn get_dependences() {}
  pub fn clear_dependences() {}
  pub fn get_memory_instructions() {}
  pub fn generate_instruction_order_map() {}
  pub fn get_instructions_for_access() {}
  pub fn get_order_for_access() {}
  pub fn get_innermost_loop() {}
}