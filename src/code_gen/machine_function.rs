#![allow(dead_code)]

// Collect native machine code for a function.
// This class contains a list of MachineBasicBlock instances that
// make up the current compiled function.
//
// This class also contains pointers to various classes which hold
// target-specific information sbout the generated code.

struct MachineFunctionInfo {}
impl MachineFunctionInfo {
  pub fn new() {}
}

enum Property {
  IsSSA,
  NoPHIs,
  TracksLiveness,
  NoVRegs,
  FailedISel,
  Legalized,
  RegBankSelected,
  Selected,
  TiedOpsRewritten,
  FailsVerification,
  TrackDebugUserValues
}

struct MachineFunctionProperties {}
impl MachineFunctionProperties {
  pub fn new() {}
  pub fn has_property() {}
  pub fn set() {}
  pub fn reset() {}
  pub fn verify_required_properties() {}
  pub fn print() {}
}

struct SEHHandler {}

struct LandingPadInfo {}

struct MachineFunction {}
impl MachineFunction {
  pub fn new() {}

  pub fn make_debug_value_substitution() {}
  pub fn substitute_debug_values_for_inst() {}
  pub fn salvage_copy_ssa() {}
  pub fn finalize_debug_instr_refs() {}
  pub fn should_use_debug_instr_ref() {}
  pub fn use_debug_instr_ref() {}
  pub fn set_use_debug_instr_ref() {}
  pub fn reset() {}
  pub fn reset_delegate() {}
  pub fn set_delegate() {}
  pub fn set_observer() {}
  pub fn get_observer() {}
  pub fn get_mmi() {}
  pub fn get_context() {}
  pub fn get_section() {}
  pub fn set_section() {}
  pub fn get_psv_manager() {}
  pub fn get_data_layout() {}
  pub fn get_function() {}
  pub fn get_name() {}
  pub fn get_function_number() {}
  pub fn has_bb_sections() {}
  pub fn has_bb_labels() {}
  pub fn set_bb_section_type() {}
  pub fn assign_begin_end_sections() {}
  pub fn get_target() {}
  pub fn get_subtarget() {}
  pub fn get_reg_info() {}
  pub fn get_frame_info() {}
  pub fn get_jump_table_info() {}
  pub fn get_or_create_jump_table_info() {}
  pub fn get_constant_pool() {}
  pub fn get_wasm_eh_function_info() {}
  pub fn get_win_eh_func_info() {}
  pub fn get_alignment() {}
  pub fn set_alignment() {}
  pub fn ensure_alignment() {}
  pub fn expose_returns_twice() {}
  pub fn set_exposes_returns_twice() {}
  pub fn has_inline_asm() {}
  pub fn set_has_inline_asm() {}
  pub fn has_win_cfi() {}
  pub fn set_has_win_cfi() {}
  pub fn needs_frame_moves() {}
  pub fn get_properties() {}
  pub fn get_info() {}
  pub fn clone_info() {}
  pub fn init_target_mechine_function_info() {}
  pub fn clone_info_from() {}
  pub fn get_denormal_mode() {}
  pub fn get_block_numbered() {}
  pub fn sjould_split_stack() {}
  pub fn get_num_block_ids() {}
  pub fn renumber_blocks() {}
  pub fn print() {}
  pub fn view_cfg() {}
  pub fn view_cfg_only() {}
  pub fn dump() {}
  pub fn verify() {}
  pub fn get_sublist_access() {}
  pub fn add_live_in() {}
  pub fn push_back() {}
  pub fn push_front() {}
  pub fn insert() {}
  pub fn splice() {}
  pub fn remove() {}
  pub fn erase() {}
  pub fn sort() {}
  pub fn get_instruction_count() {}
  pub fn add_to_mbb_numbering() {}
  pub fn remove_from_mbb_numbering() {}
  pub fn create_machine_instr() {}
  pub fn clone_machine_instr() {}
  pub fn clone_machine_instr_bundle() {}
  pub fn delete_machine_instr() {}
  pub fn create_machine_basic_block() {}
  pub fn delete_machine_basic_block() {}
  pub fn get_machine_mem_operand() {}
  pub fn allocate_operand_array() {}
  pub fn deallocate_operand_array() {}
  pub fn allocate_reg_mask() {}
  pub fn allocate_shuffle_mask() {}
  pub fn create_mi_extra_info() {}
  pub fn create_extra_symbol_name() {}
  pub fn get_jti_symbol() {}
  pub fn get_pic_base_symbol() {}
  pub fn get_frame_insstructions() {}
  pub fn add_frame_inst() {}
  pub fn get_longjmp_targets() {}
  pub fn add_long_jmp_target() {}
  pub fn get_catchret_targets() {}
  pub fn add_catchret_target() {}
  pub fn calls_eh_return() {}
  pub fn set_calls_eh_return() {}
  pub fn calls_unwind_init() {}
  pub fn set_calls_unwind_init() {}
  pub fn has_eh_catchert() {}
  pub fn set_has_eh_catchret() {}
  pub fn has_eh_scopes() {}
  pub fn set_has_eh_scopes() {}
  pub fn has_eh_funclets() {}
  pub fn set_has_eh_funclets() {}
  pub fn is_outlined() {}
  pub fn set_is_outlined() {}
  pub fn get_or_create_landing_pad_info() {}
  pub fn get_landing_pads() {}
  pub fn add_invoke() {}
}