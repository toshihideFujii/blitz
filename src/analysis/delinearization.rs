#![allow(dead_code)]

// This implements an analysis pass that tries to delinearize all
// GEP instructions in all loops using SCEV analysis functionality.

pub fn find_array_dimensions() {}
pub fn collect_parametric_terms() {}
pub fn compute_access_functions() {}
pub fn delinearize() {}
pub fn get_index_expressions_from_gep() {}
pub fn try_delinearize_fixed_size_impl() {}

struct DelinearizationPrinterPass {}
impl DelinearizationPrinterPass {
  pub fn new() {}
  pub fn run() {}
}
