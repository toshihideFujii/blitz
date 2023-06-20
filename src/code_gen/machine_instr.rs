#![allow(dead_code)]

enum CommonFlag {
  ReloadReuse,
  NoSchedComment,
  TAsmComments
}

enum MIFlag {
  NoFlags,
  FrameDestroy,
  BundledPred,
  BundledSucc,
  FmNonNans,
  FmNoInfs,
  FmNsz,
  FmArcp,
  FmContract,
  FmAfn,
  FmReassoc,
  NoUWrap,
  NoSWrap,
  IsExact,
  NoFPExcept,
  NoMerge,
  Unpredictable
}

struct ExtraInfo {}
impl ExtraInfo {
  pub fn new() {}
  pub fn get_mmos() {}
  pub fn get_pre_instr_symbol() {}
  pub fn get_post_instr_symbol() {}
  pub fn get_heap_alloc_marker() {}
  pub fn get_pc_sections() {}
  pub fn get_cfi_type() {}
}

struct MachineInstr {}
impl MachineInstr {
  pub fn new() {}
  pub fn set_parent() {}
  pub fn dumpr_impl() {}
  pub fn op_is_reg_def() {}
  pub fn op_is_reg_use() {}
  pub fn get_parent() {}
  pub fn move_before() {}
  pub fn get_mf() {}
  pub fn get_asm_printer_flags() {}
  pub fn clear_asm_printer_flags() {}
  pub fn get_asm_printer_flag() {}
  pub fn set_asm_printer_flag() {}
  pub fn clear_asm_printer_flag() {}
  pub fn get_flags() {}
  pub fn get_flag() {}
  pub fn set_flag() {}
  pub fn set_flags() {}
  pub fn clear_flag() {}
  pub fn is_inside_bundle() {}
  pub fn is_bundled() {}
  pub fn is_bundled_with_pred() {}
  pub fn is_bundled_with_succ() {}
  pub fn bundle_with_pred() {}
  pub fn bundle_with_succ() {}
  pub fn unbundle_from_pred() {}
  pub fn unbundled_from_succ() {}
  pub fn get_debug_loc() {}
  pub fn get_debug_offset() {}
  pub fn get_debug_variable_op() {}
  pub fn get_debug_variable() {}
  pub fn get_debug_expression_op() {}
  pub fn get_debug_expression() {}
  pub fn get_debug_label() {}
  pub fn get_debug_instr_num() {}
  pub fn peek_debug_instr_num() {}
  pub fn set_debug_instr_num() {}
  pub fn drop_debug_number() {}
  pub fn emit_error() {}
  pub fn get_desc() {}
  pub fn get_op_code() {}
  pub fn get_num_operands() {}
  pub fn get_num_debug_operands() {}
  pub fn get_operand() {}
  pub fn get_debug_operand() {}
  pub fn get_used_debug_regs() {}
  pub fn has_debug_operand_for_reg() {}
  pub fn get_debug_operands_for_reg() {}
  pub fn is_debug_operand() {}
  pub fn get_debug_operand_index() {}
  pub fn get_num_defs() {}
  pub fn has_implicit_def() {}
  pub fn get_num_implicit_operands() {}
  pub fn is_operand_subreg_idx() {}
  pub fn get_num_explicit_operands() {}
  pub fn get_num_explicit_defs() {}
}