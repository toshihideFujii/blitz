// CodeDataContainer is a container for all mutable fields associated with its
// referencing {Code} object.
struct CodeDataContainer {}

impl CodeDataContainer {
  // Clear uninitialized padding space.
  pub fn clear_padding() {}
  pub fn set_code_and_entry_point() {}

  // Updates the value of the code entry point.
  pub fn update_code_entry_point() {}

  pub fn allocate_external_pointer_entries() {}

  // Alias for code_entry_point to make it API compatible with Code.
  pub fn instruction_start() {}
}

// Code describes objects with on-the-fly generated machine code.
struct Code {}

impl Code {
  pub fn raw_body_start() {}
  pub fn raw_body_end() {}
  pub fn raw_body_size() {}

  pub fn raw_instruction_start() {}
  pub fn instruction_start() {}
  pub fn off_heap_instruction_start() {}
  pub fn raw_instruction_end() {}
  pub fn instruction_end() {}
  pub fn off_heap_instruction_end() {}

  pub fn get_offset_from_instruction_start() {}

  pub fn raw_instruction_size() {}
  pub fn set_raw_instruction_size() {}
  pub fn instruction_size() {}
  pub fn off_heap_instruction_size() {}

  pub fn raw_metadata_start() {}
  pub fn metadata_start() {}
  pub fn off_heap_metadata_start() {}
  pub fn raw_metadata_end() {}
  pub fn metadata_end() {}
  pub fn off_heap_metadata_end() {}
  pub fn raw_metadata_size() {}
  pub fn set_raw_metadata_size() {}
  pub fn metadata_size() {}
  pub fn off_heap_metadata_size() {}

  // The offset where the safepoint table starts.
  pub fn safepoint_table_offset() {}
  pub fn safepoint_table_address() {}
  pub fn safepoint_table_size() {}
  pub fn has_safepoint_table() {}

  // The offset where the exception handler table starts.
  pub fn handler_table_offset() {}
  pub fn set_handler_table_offset() {}
  pub fn handler_table_address() {}
  pub fn handler_table_size() {}
  pub fn has_handler_table() {}

  // Offset of the constant pool.
  pub fn constant_pool_offset() {}
  pub fn set_constant_pool_offset() {}
  pub fn constant_pool_address() {}
  pub fn constant_pool_size() {}
  pub fn has_constant_pool() {}

  // Offset of the code comment section.
  pub fn code_comments_offset() {}
  pub fn set_code_comments_offset() {}
  pub fn code_comments_address() {}
  pub fn has_code_comments() {}

  // Offset of the unwinding info section.
  pub fn unwinding_info_offset() {}
  pub fn set_unwinding_info_offset() {}
  pub fn unwinding_info_start() {}
  pub fn unwinding_info_end() {}
  pub fn unwinding_info_size() {}
  pub fn has_unwinding_info() {}

  // Code relocation information
  pub fn relocation_info() {}

  // This function should be called only from GC.
  pub fn clear_embedded_objects() {}

  // Array containing data for deopt.
  pub fn deoptimization_data() {}

  // ByteArray for the source positions table for non-baseline code.
  pub fn source_position_table() {}

  // ByteArray for the bytecode offset for baseline code.
  pub fn bytecode_offset_table() {}

  // If source positions have not been collected or an exception has been thrown
  // this will return empty_byte_array.
  pub fn source_position_table_sfi() {}

  // A container indirection for all mutable fields.
  pub fn code_data_container() {}

  // Link for lists of optimized or deoptimized code.
  pub fn next_code_link() {}
  pub fn set_next_code_link() {}

  // Unchecked accessors to be used during GC.
  pub fn unchecked_relocation_info() {}

  pub fn relocation_size() {}

  // Access to specific code kind.
  pub fn kind() {}

  pub fn is_optimized_code() {}
  pub fn is_wasm_code() {}

  // Testers for interpreter builtins.
  pub fn is_interpreter_trampoline_builtin() {}
  pub fn is_baseline_leave_frame_builtin() {}

  // Tells whether the code checks the optimization marker in the function's feedback
  pub fn checks_optimization_marker() {}

  // Tells whether the outgoing parameters of this code are tagged pointers.
  pub fn has_tagged_outgoing_params() {}

  // Tells whether the code object was generated by the TurboFan optimizing compiler.
  pub fn is_turbofanned() {}

  // If CodeKindIsOptimizedJSFunction, tells whether the embedded objects in code
  // should be treated weakly.
  pub fn can_have_weak_objects() {}
  pub fn set_can_have_weak_objects() {}

  // For builtins, tells which builtin index the code object has.
  // The builtin index is a non-negative integer for builtins,
  // and Builtin::NO_BUILTIN_ID(-1) otherwise.
  pub fn builtin_id() {}
  pub fn set_builtin_id() {}
  pub fn is_builtin() {}

  pub fn inlined_bytecode_size() {}
  pub fn set_inlined_bytecode_size() {}

  pub fn has_safepoint_info() {}

  // If has_safepoint_info(), the number of stack slots reserved in the code prologue.
  pub fn stack_slots() {}

  // If CodeKindCanDeoptimize, tells whether the code is going to be deoptimized.
  pub fn marked_for_deoptimization() {}
  pub fn set_marked_for_deoptimization() {}

  pub fn deoptimization_count() {}
  pub fn increment_deoptimization_count() {}

  // If CodeKindIsOptimizedJSFunction, tells whether the embedded objects in the code
  // marked for deoptimization were cleared.
  pub fn embedded_objects_cleared() {}
  pub fn set_embedded_objects_cleared() {}

  // If CodeKindCanDeoptimize, tells whether the code was already deoptimized.
  pub fn deopt_already_counted() {}
  pub fn set_deopt_already_counted() {}

  // For kind BUILTIN, tells whether the exception thrown by the code will lead to
  // promise rejection or uncaught if both this and is_exeption_caight is set.
  // Use GetBuiltinCaatchPrediction to access this.
  pub fn set_is_promise_rejection() {}

  // For kind BUILTIN, tells whether the exception thrown by the code will be caught
  // internally or uncaught if both this and is_promise_rejection is set.
  // Use GetBuiltinCaatchPrediction to access this.
  pub fn set_is_exeption_caught() {}

  // For kind BUILTIN tells whether this is a trampoline to an off-heap builtin.
  pub fn is_off_heap_trampoline() {}

  // Get the safepoint entry for the given pc.
  pub fn get_safepoint_entry() {}

  pub fn wipe_out_header() {}

  // Clear uninitialized padding space.
  pub fn clear_padding() {}

  // Initialize the flags field.
  pub fn initialize_flags() {}

  // Convert a target address into a code object.
  pub fn get_code_from_target_address() {}

  // Convert an entry address into an object.
  pub fn get_object_from_entry_address() {}

  // Returns the size of code and its metadata.
  // This includes the size of code relocation information, deoptimization data.
  pub fn size_including_metadata() {}

  // Returns the address of the first relocation info.
  pub fn relocation_start() {}

  // Returns the address right after the relocation info.
  pub fn relocation_end() {}

  // Code entry point.
  pub fn entry() {}

  // Returns true if pc is inside this object's instructions.
  pub fn contains() {}

  // Relocate the code by delta butes.
  // Called to signal that this code object has been moved by delta bytes.
  pub fn relocate() {}

  // Migrate code from desc without flushing the instruction cache.
  pub fn copy_from_no_flush() {}
  pub fn relocate_from_desc() {}

  // Copy the RelocInfo portion of |desc| to |dest|.
  // The ByteArray must be exactly the same size as the RelocInfo in |desc|.
  pub fn copy_reloc_info_to_byte_array() {}

  pub fn get_baseline_start_pc_for_bytecode_offset() {}

  pub fn get_baseline_end_pc_for_bytecode_offset() {}

  // Returns the PC of the next bytecode in execution order.
  pub fn get_baseline_pc_for_next_executed_bytecode() {}

  pub fn get_bytecode_offset_for_baseline_pc() {}

  // Flushes the instruction cache for the executable instructions of this code object.
  pub fn flush_i_cache() {}

  // Returns the object size for a given body (used for allocation).
  pub fn size_for() {}

  // Dispatched behavior.
  pub fn code_size() {}

  pub fn can_deopt_at() {}

  pub fn get_builtin_catch_prediction() {}

  pub fn is_isolate_independent() {}

  pub fn can_contain_weak_objects() {}

  pub fn is_weak_object() {}

  pub fn is_weak_object_in_optimized_code() {}

  // Returns false if this is an embedded builtin Code object that's in read_only_space
  // and hence doesn't have executable permissions.
  pub fn is_executable() {}

  // Returns true if the function is inlined in the code.
  pub fn inlines() {}
}
