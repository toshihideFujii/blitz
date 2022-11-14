#![allow(dead_code)]

/*
Interfaces for registering analysis passes, producing
common pass manager configurations, and parsing of
pass pipelines.
*/

struct PipelineTuningOptions {}

struct PassBuilder {}

impl PassBuilder {
  pub fn cross_register_proxies() {}

  pub fn register_module_analyses() {}

  pub fn register_cgscc_analyses() {}

  pub fn register_function_analyses() {}

  pub fn register_loop_analyses() {}

  pub fn build_function_simplification_pipeline() {}

  pub fn build_module_simplification_pipeline() {}

  pub fn build_inliner_pipeline() {}

  pub fn build_module_inliner_pipeline() {}

  pub fn build_module_optimization_pipeline() {}

  pub fn build_per_module_default_pipeline() {}

  pub fn build_thin_lto_prelink_default_pipeline() {}

  pub fn build_thin_lto_default_pipeline() {}

  pub fn build_lto_prelink_default_pipeline() {}

  pub fn build_lto_default_pipeline() {}

  pub fn build_o0_default_pipeline() {}

  pub fn build_default_aa_pipeline() {}

  pub fn parse_pass_pipeline() {}

  pub fn parse_aa_pipeline() {}

  pub fn is_aa_pass_name() {}

  pub fn is_analysis_pass_name() {}

  pub fn print_pass_names() {}

  pub fn register_peephole_ep_callback() {}

  pub fn register_late_loop_optimizations_ep_callback() {}

  pub fn register_loop_optimizer_end_ep_callback() {}

  pub fn register_scalar_optimizer_late_ep_callback() {}

  pub fn register_cgscc_optimizer_ep_callback() {}

  pub fn register_vectorizer_start_ep_callback() {}

  pub fn register_pipeline_start_ep_callback() {}

  pub fn register_pipeline_early_simplification_callback() {}

  pub fn register_optimizer_early_ep_callback() {}
}