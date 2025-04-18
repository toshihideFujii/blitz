#![allow(dead_code)]

use std::sync::LazyLock;

use crate::{blitz_data::{
  AutotuneCacheMode, CommandBufferCmdType, DebugOptions, LibNvJitLinkMode,
  PartitioningAlgorithm, PgleStrictnessLevel, PipelineParallelismOptLevel,
  WhileLoopUnrolling, XnnGraphFusionMode
}, parse_flags_from_env::parse_flags_from_env_and_die_if_unknown};

// Gets a DebugOptions proto that reflects the defaults as if no flags were set.
pub fn default_debug_options_ignoring_flags() -> DebugOptions {
  let mut opts = DebugOptions::new();

  opts.set_blitz_llvm_enable_alias_scope_metadata(true);
  opts.set_blitz_llvm_enable_noalias_metadata(true);
  opts.set_blitz_llvm_enable_invariant_load_metadata(true);
  opts.set_blitz_backend_optimization_level(3);
  opts.set_blitz_gpu_autotune_level(4);
  opts.set_blitz_gpu_autotune_max_solutions(0);
  opts.set_blitz_cpu_multi_thread_eigen(true);
  opts.set_blitz_cuda_data_dir("./cuda_sdk_lib".to_string());
  opts.set_blitz_gpu_generate_debug_info(false);
  opts.set_blitz_gpu_generate_line_info(false);  

  opts.set_blitz_gpu_use_runtime_fusion(false);

  opts.set_blitz_eliminate_hlo_implicit_broadcast(true);
  opts.set_blitz_dump_hlo_as_html(false);
  opts.set_blitz_dump_fusion_visualization(false);
  opts.set_blitz_dump_include_timestamp(false);
  opts.set_blitz_dump_max_hlo_modules(-1);
  opts.set_blitz_dump_module_metadata(false);
  opts.set_blitz_dump_hlo_as_long_text(true);
  opts.set_blitz_dump_large_constants(false);
  opts.set_blitz_dump_enable_mlir_pretty_form(true);
  opts.set_blitz_dump_full_hlo_config(true);
  opts.set_blitz_gpu_unsupported_annotate_with_emitter_loc(false);
  opts.set_blitz_debug_buffer_assignment_show_max(15);

  opts.set_blitz_cpu_use_fusion_emitters(true);
  opts.set_blitz_cpu_use_thunk_runtime(true);
  opts.set_blitz_cpu_use_xnnpack(false);
  opts.set_blitz_cpu_experimental_xnn_graph_fusion_mode(XnnGraphFusionMode::Diabled);
  opts.set_blitz_cpu_parallel_codegen_split_count(32);
  opts.set_blitz_gpu_copy_insertion_use_region_analysis(false);
  opts.set_blitz_cpu_enable_concurrency_optimized_scheduler(true);
  opts.set_blitz_cpu_prefer_vector_width(256);
  opts.set_blitz_cpu_max_isa(false); // TODO
  opts.set_blitz_cpu_generate_unique_c_style_kernel_entry_points(false);

  opts.set_blitz_cpu_enable_fast_math(false);

  opts.set_blitz_cpu_fast_math_honor_nans(true);
  opts.set_blitz_cpu_fast_maath_honor_infs(true);
  opts.set_blitz_cpu_fast_math_honor_functions(true);
  opts.set_blitz_cpu_fast_math_honor_division(true);

  opts.set_blitz_gpu_fused_attension_use_cudnn_rng(false);

  opts.set_blitz_cpu_enable_fast_min_max(false);

  opts.set_blitz_gpu_enable_cublaslt(false);

  opts.add_blitz_gpu_enable_command_buffer(CommandBufferCmdType::Fusion);
  opts.add_blitz_gpu_enable_command_buffer(CommandBufferCmdType::Cublas);
  opts.add_blitz_gpu_enable_command_buffer(CommandBufferCmdType::Cublaslt);
  opts.add_blitz_gpu_enable_command_buffer(CommandBufferCmdType::CustomCall);
  opts.add_blitz_gpu_enable_command_buffer(CommandBufferCmdType::Cudnn);
  opts.set_blitz_gpu_graph_min_graph_size(5);
  opts.set_blitz_gpu_graph_enable_concurrent_region(false);
  opts.set_blitz_cmd_buffer_trace_cache_size(16);

  opts.set_blitz_gpu_collectives_use_persistent_cliques(false);

  opts.set_blitz_gpu_enable_fast_min_max(false);
  opts.set_blitz_gpu_strict_conv_algorithm_picker(true);

  opts.set_blitz_allow_excess_precision(true);
  opts.set_blitz_force_host_platform_device_count(1);
  opts.set_blitz_gpu_all_reduce_combine_threshold_bytes(30 * 1024 * 1024 + 7);
  opts.set_blitz_gpu_all_gather_combine_threshold_bytes(30 * 1024 * 1024 + 7);
  opts.set_blitz_gpu_reduce_scatter_combine_threshold_bytes(30 * 1024 * 1024 + 7);
  opts.set_blitz_gpu_collective_permute_combine_threshold_bytes(30 * 1024 * 1024 + 7);
  opts.set_blitz_gpu_enable_all_gather_combine_by_dim(false);
  opts.set_blitz_gpu_enable_reduce_scatter_combine_by_dim(false);
  opts.set_blitz_gpu_enable_approx_costly_collectives(false);

  opts.set_blitz_gpu_enable_reassociation_for_converted_ar(true);

  opts.set_blitz_cpu_enable_xprof_traceme(false);
  opts.set_blitz_gpu_unsafe_fallback_to_driver_on_ptxas_not_found(false);
  opts.set_blitz_multiheap_size_constraint_per_heap(-1);
  opts.set_blitz_detailed_logging(true);
  opts.set_blitz_enable_dumping(true);

  opts.set_blitz_gpu_enable_custom_fusions(false);
  opts.set_blitz_gpu_enable_dynamic_slice_fusion(false);
  opts.set_blitz_gpu_nccl_termination_timeout_seconds(-1);
  opts.set_blitz_gpu_enable_shared_constants(true);
  opts.set_blitz_gpu_enable_nccl_user_buffers(false);
  opts.set_blitz_gpu_enable_nccl_comm_splitting(true);
  opts.set_blitz_gpu_nccl_init_max_rank_per_root_ratio(0);

  opts.set_blitz_gpu_temp_buffer_use_separate_color(false);
  opts.set_blitz_gpu_require_exclusive_lock(false);

  // TODO: sol_estimator_defaults

  opts.set_blitz_gpu_pgle_profile_file_or_directory_path("".to_string());
  opts.set_blitz_gpu_memory_limit_slop_factor(95);
  opts.set_blitz_enable_highest_priority_async_stream(true);

  opts.set_blitz_gpu_enable_pipelined_collectives(false);
  opts.set_blitz_gpu_enable_pipelined_all_reduce(false);
  opts.set_blitz_gpu_enable_pipelined_all_gather(false);
  opts.set_blitz_gpu_enable_pipelined_reduce_scatter(true);
  opts.set_blitz_gpu_enable_pipelined_p2p(false);

  opts.set_blitz_gpu_collective_permute_decomposer_threshold(usize::MAX);  
  opts.set_blitz_gpu_experimental_pipeline_parallelism_opt_level(PipelineParallelismOptLevel::Disable);
  opts.set_blitz_gpu_experimental_collective_cse_distance_threshold(0);
  opts.set_blitz_gpu_experimental_enable_subchannel_dequantisaction_fusion(false);
  opts.set_blitz_partitioning_algorithm(PartitioningAlgorithm::Noop);

  opts.set_blitz_gpu_enable_while_loop_reduce_scatter_code_motion(false);
  opts.set_blitz_gpu_collective_inflation_factor(1);
  opts.set_blitz_llvm_force_inline_before_split(true);
  opts.set_blitz_gpu_exhaustive_tiling_search(false);
  opts.set_blitz_gpu_experimental_enable_triton_heroless_priority_fusion(false);

  opts.set_blitz_gpu_auto_spmd_partitioning_memory_budget_gb(0);
  opts.set_blitz_gpu_auto_spmd_partitioning_memory_budget_ratio(1.1);
  opts.set_blitz_gpu_triton_gemm_disable_reduced_precision_reduction(false);
  opts.set_blitz_gpu_unsafe_pipelined_loop_annotator(false);

  opts.set_blitz_gpu_copy_insertion_use_region_analysis(false);
  opts.set_blitz_gpu_collect_cost_model_stats(false);
  opts.set_blitz_gpu_enable_split_k_autotuning(true);

  opts.set_blitz_gpu_enable_reduction_epilogue_fusion(true);
  opts.set_blitz_gpu_enable_nccl_clique_optimization(false);
  opts.set_blitz_gpu_cublas_fallback(false);
  opts.set_blitz_gpu_cudnn_gemm_fusion_level(0);
  opts.set_blitz_enable_while_loop_double_buffering(false);
  opts.set_blitz_gpu_enable_while_loop_unrolling(WhileLoopUnrolling::AutoUnroll);
  opts.set_blitz_gpu_ensure_minor_dot_contraction_dims(false);
  opts.set_blitz_gpu_filter_kernels_spilling_registers_on_autotuning(true);
  opts.set_blitz_gpu_fail_ptx_compilation_on_register_spilling(false);
  opts.set_blitz_gpu_llvm_verification_level(0);
  opts.set_blitz_gpu_target_config_filename("".to_string());
  opts.set_blitz_gpu_enable_cub_radix_sort(true);
  opts.set_blitz_gpu_enable_cudnn_layer_norm(false);
  opts.set_blitz_gpu_threshold_for_windowed_einsum_mib(100000);
  opts.set_blitz_gpu_operand_bytes_threshold_for_windowed_einsum(-1);

  opts.set_blitz_gpu_enable_triton_hopper(false);
  opts.set_blitz_gpu_experimental_enable_dynamic_dot_search_space(false);
  opts.set_blitz_gpu_experimental_enable_fusion_block_level_rewriter(false);

  opts.set_blitz_gpu_enable_llvm_module_compilation_parallelism(false);
  opts.set_blitz_gpu_enable_libnvptxcompiler(false); // TODO
  opts.set_blitz_gpu_libnvjitlink_mode(LibNvJitLinkMode::Auto);

  opts.set_blitz_gpu_nccl_collective_max_channnels(0);
  opts.set_blitz_gpu_nccl_p2p_max_nchannels(0);
  opts.set_blitz_gpu_multi_streamed_windowed_einsum(false);
  opts.set_blitz_gpu_experimental_stream_annotation(false);

  opts.set_blitz_gpu_gemm_rewrite_size_threshold(100);
  opts.set_blitz_gpu_use_memcpy_local_p2p(false);
  opts.set_blitz_reduce_window_rewrite_base_length(16);
  opts.set_blitz_gpu_require_complete_aot_autotune_results(false);
  opts.set_blitz_gpu_enable_host_memory_offloading(false);
  opts.set_blitz_gpu_nccl_terminate_on_error(false);
  opts.set_blitz_gpu_shared_autotuning(true);
  opts.set_blitz_syntax_sugar_async_ops(false);

  opts.set_blitz_gpu_per_fusion_autotune_cache_dir("".to_string());
  opts.set_blitz_gpu_experimental_autotune_cache_mode(AutotuneCacheMode::Update);
  opts.set_blitz_enable_command_buffers_during_profiling(false);
  opts.set_blitz_gpu_cudnn_gemm_max_plans(5);
  opts.set_blitz_gpu_pgle_accuracy_checker(PgleStrictnessLevel::Warn);

  opts.set_blitz_gpu_executable_warn_stuck_timeout_seconds(10);
  opts.set_blitz_gpu_executable_terminate_timeout_aeconds(30);
  opts.set_blitz_gpu_experimental_collective_perf_table_path("".to_string());
  opts.set_blitz_gpu_experimental_matmul_perf_table_path("".to_string());
  opts.set_blitz_gpu_experimental_disable_binary_libraries(false);

  opts.set_blitz_ignore_channel_id(false);
  opts.set_blitz_gpu_dot_merger_threshold_mb(32);
  opts.set_blitz_enable_fast_math(false);
  opts.set_blitz_gpu_experimental_parallel_collective_overlap_limit(1);
  opts.set_blitz_pjrt_allow_auto_layout_in_hlo(false);
  opts.set_blitz_gpu_enable_scatter_determinism_expander(false);
  opts.set_blitz_gpu_unsupported_enable_ragged_all_to_all_decomposer(false);
  opts.set_blitz_gpu_unsupported_use_ragged_all_to_all_one_shot_kernel(false);
  opts.set_blitz_gpu_unsupported_enable_all_reduce_decomposer(false);
  opts
}

static FLAG_VALUES: LazyLock<DebugOptions> =
  LazyLock::new(default_debug_options_ignoring_flags);

static FLAG_OBJECTS: Vec<i64> = Vec::new();

// Allocates flag_values and flag_objects; this function must not be called more
// than once - its call done via call_once.
fn allocate_flags() {
  make_debug_options_flags(&FLAG_OBJECTS, &FLAG_VALUES);
  parse_flags_from_env_and_die_if_unknown(
    "BLITZ_FLAGS".to_string(), &FLAG_OBJECTS);
}

// Construct flags which write to the debug_options proto when parsed. Existing
// contents of debug_options is used as the default. Can be called multiple
// times.
pub fn make_debug_options_flags(
  _flag_list: &Vec<i64>, _debug_options: &DebugOptions)
{
  unimplemented!()
}

// Appends flag definitions for debug options to flag_list. Existing
// contents of debug_options is used as the default. If debug_options is null,
// uses global defaults. Modifies global state on first call.
pub fn append_debug_options_flags(
  _flag_list: &Vec<i64>, _debug_options: &Option<DebugOptions>)
{
  unimplemented!()
}

// Fetches a DebugOptions proto message from flags provided to the program.
// Flags must be registered with the flags parser using AppendDebugOptionsFlags
// first.
pub fn get_debug_options_from_flags() -> &'static DebugOptions {
  if FLAG_VALUES.blitz_flags_reset() {
    parse_flags_from_env_and_die_if_unknown(
      "BLITZ_FLAGS".to_string(),
    &FLAG_OBJECTS);
  }
  &FLAG_VALUES
}