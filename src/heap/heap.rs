enum AllocationOrigin {
  GeneratedCode,
  Runtime,
  GC,
}

enum GarbageCollectionReason {
  Unknown,
  AllocationFailure,
  AllocationLimit,
  ContextDisposal,
  CountersExtension,
  Debugger,
  Deserializer,
  ExternalMemoryPressure,
  FinalizeMarkingViaStackGuard,
  FinalizeMarkingViaTask,
  FullHashTable,
  HeapProfiler,
  Task,
  LastResort,
  LowMemoryNotification,
  MakeHeapIterable,
  MemoryPressure,
  MemoryReducer,
  Runtime,
  SamplingProfiler,
  SnapshotCreator,
  Testing,
  ExternalFinalize,
  GlobalAllocationLimit,
  MeasureMemory,
  BackgroundAllocationFailure,
}

enum YoungGenerationHandling {
  RegularScavenge,
  FastPromotionDuringScavenge,
  UnusedBucket,
}

enum SkipRoot {
  ExternalStringTable,
  GlobalHandles,
  OldGeneration,
  Stack,
  MainThreadHandles,
  Unserializable,
  Weak,
}

enum UnprotectMemoryOrigin {
  MainThread,
  MaybeOffMainThread,
}

struct StrongRootsEntry {}

struct AllocationResult {}

impl AllocationResult {
  pub fn new() {}

  pub fn is_retry() {}

  pub fn to_object_checked() {}

  pub fn to_object() {}

  pub fn to_address() {}

  pub fn retry_space() {}
}

struct Heap {}

impl Heap {
  pub fn get_code_flush_mode() {}

  pub fn zap_value() {}

  pub fn is_young_generation_collector() {}

  pub fn young_generation_collector() {}

  pub fn collector_name() {}

  pub fn copy_block() {}

  pub fn write_barrier_for_range() {}

  pub fn write_barrier_for_code_slow() {}

  pub fn generational_barrier_slow() {}

  pub fn record_ephmeron_key_write() {}

  pub fn ephemeron_key_write_barrier_from_code() {}

  pub fn generational_barrier_for_code_slow() {}

  pub fn page_flags_are_consistent() {}

  pub fn notify_deserialization_complete() {}

  pub fn notify_bootstrap_complete() {}

  pub fn notify_old_generation_expansion() {}

  pub fn update_current_epoch() {}

  pub fn new_space_allocation_top_address() {}
  pub fn new_space_allocation_limit_address() {}

  pub fn old_space_allocation_top_address() {}
  pub fn old_space_allocation_limit_address() {}

  pub fn new_space_size() {}
  pub fn new_space_capacity() {}

  pub fn move_range() {}
  pub fn copy_range() {}

  pub fn create_filler_object_at() {}
  pub fn create_filler_object_at_background() {}
  pub fn create_filler_for_array() {}

  pub fn can_move_object_start() {}

  pub fn is_immovable() {}
  pub fn is_large_object() {}

  pub fn left_trim_fixed_array() {}
  pub fn right_trim_fixed_array() {}

  pub fn to_boolean() {}

  pub fn notify_context_disposed() {}

  pub fn set_native_contexts_list() {}
  pub fn native_context_list() {}

  pub fn set_allocation_sites_list() {}
  pub fn allocation_sites_list() {}

  pub fn set_dirty_js_finalization_registries_list() {}
  pub fn dirty_js_finalization_registries_list() {}

  pub fn set_dirty_js_finalization_registries_list_tail() {}
  pub fn dirty_js_finalization_registries_list_tail() {}

  pub fn allocation_sites_list_address() {}

  pub fn foreach_allocation_site() {}

  // Number of mark-sweeps.
  pub fn ms_count() {}

  pub fn allowed_to_be_migrated() {}

  pub fn check_handle_count() {}

  pub fn allocations_count() {}

  pub fn print_short_heap_statistics() {}

  pub fn print_free_list_stats() {}

  pub fn dump_json_heap_statistics() {}

  pub fn write_protect_code_memory() {}

  pub fn code_space_memory_modification_scope_depth() {}

  pub fn increment_code_space_memory_modification_scope_depth() {}

  pub fn decrement_code_space_memory_modification_scope_depth() {}

  pub fn unprotect_and_register_memory_chunk() {}

  pub fn unregister_unprotected_memory_chunk() {}

  pub fn enable_unprotected_memory_chunk_registry() {}

  pub fn disable_unprotected_memory_chunk_registory() {}

  pub fn unprotected_memory_chunks_registry_enabled() {}

  pub fn increment_code_page_collection_memory_modification_scope_depth() {}

  pub fn decrement_code_page_collection_memory_modification_scope_depth() {}

  pub fn code_page_collection_memory_modification_scope_depth() {}

  pub fn gc_state() {}

  pub fn set_gc_state() {}

  pub fn is_tearing_down() {}

  pub fn force_oom() {}

  pub fn is_in_gc_post_processing() {}

  pub fn find_allocation_memento() {}

  pub fn collect_garbage_for_background() {}

  pub fn create_api_objects() {}

  pub fn idle_notification() {}

  pub fn memory_pressure_notification() {}

  pub fn check_memory_pressure() {}

  pub fn add_near_heap_limit_callback() {}

  pub fn remove_near_heap_limit_callback() {}

  pub fn automatically_restore_initial_heap_limit() {}

  pub fn append_array_buffer_extension() {}

  pub fn detach_array_buffer_extension() {}

  pub fn safepoint() {}

  pub fn monotonically_increasing_time_in_ms() {}

  pub fn verify_new_space_top() {}

  pub fn record_stats() {}

  pub fn measure_memory() {}

  pub fn check_new_space_expansion_criteria() {}

  pub fn visit_external_resources() {}

  pub fn should_be_promoted() {}

  pub fn increment_deferred_count() {}

  pub fn next_script_id() {}
  pub fn next_debugging_id() {}
  pub fn get_next_template_serial_number() {}

  pub fn set_serialized_objects() {}
  pub fn set_serialized_global_proxy_sizes() {}

  pub fn set_basic_block_profiling_data() {}

  pub fn remember_unmapped_page() {}

  pub fn external_memory_hard_limit() {}

  pub fn external_memory() {}
  pub fn external_memory_limit() {}
  pub fn update_external_memory() {}

  pub fn young_array_buffer_bytes() {}
  pub fn old_array_buffer_bytes() {}

  pub fn backing_store_bytes() {}

  pub fn compact_weak_array_list() {}

  pub fn compact_weak_array_lists() {}

  pub fn add_retained_map() {}

  pub fn on_allocation_event() {}

  pub fn on_move_event() {}

  pub fn can_allocate_in_read_only_space() {}

  pub fn deserialization_complete() {}

  pub fn can_safepoint() {}

  pub fn has_low_allocation_rate() {}
  pub fn has_high_fragmentation() {}

  pub fn activate_memory_reducer_if_needed() {}

  pub fn should_optimize_for_memory_usage() {}

  pub fn high_memory_pressure() {}

  pub fn collection_requested() {}
  pub fn check_collection_requested() {}

  pub fn restore_heap_limit() {}

  // =====
  // Initialization
  // =====

  pub fn configure_heap() {}
  pub fn configure_heap_default() {}

  // Prepares the heap, setting up for deserialization.
  pub fn setup() {}

  pub fn setup_from_read_only_heap() {}

  pub fn replace_read_only_space() {}

  // Sets up the heap memory without creating any objects.
  pub fn setup_spaces() {}

  // Prepares the heap, setting up for deserialization.
  pub fn initialize_main_thread_local_heap() {}

  pub fn initialize_hash_seed() {}

  pub fn create_heap_objects() {}

  pub fn create_object_stats() {}

  // Sets the teardown state, so no new GC tasks get posted.
  pub fn start_teardown() {}

  // Destroys all memory allocated by the heap.
  pub fn teardown() {}

  // Returns whether setup has been called.
  pub fn has_been_setup() {}

  // Initializes shared spaces.
  pub fn init_shared_spaces() {}

  // Removes shared spaces again.
  pub fn de_init_shared_spaces() {}

  // =====
  // Getters for spaces.
  // =====

  pub fn new_space_top() {}

  pub fn new_space() {}

  pub fn old_space() {}

  pub fn code_space() {}

  pub fn map_space() {}

  pub fn lo_space() {}

  pub fn code_lo_space() {}

  pub fn new_lo_space() {}

  pub fn read_only_space() {}

  pub fn paged_space() {}
  pub fn space() {}

  // =====
  // Getters to other components.
  // =====

  pub fn tracer() {}

  pub fn memory_allocator() {}

  pub fn isolate() {}

  pub fn mark_compact_collector() {}

  pub fn minor_mark_compact_collector() {}

  pub fn array_buffer_sweeper() {}

  pub fn code_region() {}

  pub fn code_range() {}

  pub fn main_thread_local_heap() {}

  pub fn as_heap() {}

  // =====
  // Root set access.
  // =====

  // Shortcut to the roots table stored in the Isolate.
  pub fn roots_table() {}

  pub fn set_root_materialized_objects() {}
  pub fn set_root_script_list() {}
  pub fn set_root_no_script_shared_function_infos() {}
  pub fn set_message_listeners() {}
  pub fn set_pending_optimize_for_test_bytecode() {}

  pub fn regester_strong_roots() {}
  pub fn unregister_strong_roots() {}
  pub fn update_strong_roots() {}

  pub fn set_builtins_constants_table() {}
  pub fn set_detached_contexts() {}

  pub fn set_interpreter_entry_trampoline_for_profiling() {}

  pub fn enqueue_dirty_js_finalization_registry() {}

  pub fn dequeue_dirty_js_finalization_registry() {}

  pub fn remove_dirty_finalization_registries_on_context() {}

  pub fn has_dirty_js_finalization_registries() {}

  pub fn post_finalization_registry_cleanup_task_if_needed() {}

  pub fn set_is_finalization_registry_cleanup_task_posted() {}

  pub fn is_finalization_registry_cleanup_task_posted() {}

  pub fn keep_during_job() {}

  pub fn clear_kept_objects() {}

  // =====
  // Inline allocation.
  // =====

  pub fn inline_allocation_disabled() {}
  pub fn enable_inline_allocation() {}
  pub fn disable_inline_allocation() {}

  // =====
  // Methods triggering GCs.
  // =====

  // Performs garbage collection operation.
  pub fn collect_garbage() {}

  // Performs a full garbage collection.
  pub fn collect_all_garbage() {}

  // Last hope GC, should try to squeeze as much as possible.
  pub fn collect_all_available_garbage() {}

  pub fn presice_collect_all_garbage() {}

  // Performs garbage collection operation for the shared heap.
  pub fn collect_shared_garbage() {}

  pub fn report_external_memory_pressure() {}

  pub fn set_get_externally_allcoated_memory_in_bytes_callback() {}

  // Invoked when GC was requested via the stack guard.
  pub fn handle_gc_request() {}

  // =====
  // Builtins.
  // =====

  pub fn builtin() {}
  pub fn builtin_address() {}
  pub fn builtin_tier0_address() {}
  pub fn set_builtin() {}

  // =====
  // Iterators.
  // =====

  pub fn iterate_roots() {}
  pub fn iterate_roots_including_clients() {}
  pub fn iterate_smi_roots() {}
  pub fn iterate_weak_roots() {}
  pub fn iterate_weak_global_handles() {}
  pub fn iterate_builtins() {}
  pub fn iterate_stack_roots() {}

  // =====
  // Store buffer API.
  // =====

  pub fn is_marking_flag_address() {}
  pub fn set_is_marking_flag() {}
  pub fn store_buffer_top_address() {}
  pub fn store_buffer_mask_constant() {}
  pub fn store_buffer_overflow_function_address() {}

  pub fn clear_redorded_slot() {}
  pub fn clear_recorded_slot_range() {}
  pub fn insert_into_remembered_set_from_code() {}

  // =====
  // Incremental marking API.
  // =====

  pub fn gc_flags_for_incremental_marking() {}
  pub fn start_idle_incremental_marking() {}
  pub fn start_incremental_marking() {}
  pub fn start_incremental_marking_if_allocation_limit_is_reached() {}
  pub fn start_incremental_marking_if_allocation_limit_is_reached_background() {
  }
  pub fn finalize_incremental_marking_if_complete() {}
  pub fn finalize_incremental_marking_atomically() {}
  pub fn complete_sweeping_full() {}
  pub fn complete_sweeping_young() {}
  pub fn ensure_sweeping_completed() {}
  pub fn incremental_marking() {}
  pub fn marking_barrier() {}

  // =====
  // Concurrent marking API.
  // =====

  pub fn concurrent_marking() {}
  pub fn notify_object_layout_change() {}
  pub fn notify_code_object_change_start() {}
  pub fn notify_code_object_change_end() {}

  // =====
  // Deoptimization support API.
  // =====

  pub fn set_construct_stub_create_deopt_pc_offset() {}
  pub fn set_construct_stub_invoke_deopt_pc_offset() {}
  pub fn set_interpreter_entry_return_pc_offset() {}

  pub fn invalidate_code_deoptimization_data() {}
  pub fn deopt_marked_allocation_sites() {}
  pub fn deopt_maybe_tenured_allocation_sites() {}

  // =====
  // Embedder heap tracer support.
  // =====

  pub fn local_embedder_heap_tracer() {}
  pub fn set_embedder_heap_tracer() {}
  pub fn get_embedder_heap_tracer() {}
  pub fn register_externally_referenced_object() {}
  pub fn set_embedder_stack_state_for_next_finalization() {}
  pub fn flags_for_embedder_tracer() {}
}
