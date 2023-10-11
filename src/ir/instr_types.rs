#![allow(dead_code)]

use crate::{
  ir::{type_::Type, instruction::OpCode},
  adt::{twine::Twine, string_ref::StringRef},
  support::{alignment::MaybeAlign, mod_ref::MemoryEffects}
};

use super::{
  instruction::{InstructionBase,
  UnaryOps, BinaryOps, Instruction}, value::Value, attributes::{AttributeList,
  AttrKind, Attribute}, use_::Use, type_::FunctionType,
  basic_block::BasicBlock, blits_context::blits_context_mut
};

/*
pub struct UnaryInstruction {
  pub inst: InstructionBase
}

impl UnaryInstruction {
  pub fn new(v_type: Box<dyn Type>, i_type: u32, _v: Box<dyn Value>,
    ib: Option<Box<InstructionBase>>) -> Self
  {
    UnaryInstruction { inst: InstructionBase::new_ib(v_type, i_type, None,
      1, ib) }
  }

  pub fn get_op_code(&self) -> OpCode {
    self.inst.get_op_code()
  }

  pub fn get_operand(&self, _i: u32) -> Option<Box<dyn Value>> { None }
  pub fn set_operand(&self, _v: Option<Box<dyn Value>>) {}
  pub fn get_num_operands(&self) -> usize { 0 }

  pub fn class_of(i: &InstructionBase) -> bool {
    i.is_unary_op() ||
    i.get_op_code() == OpCode::Alloca ||
    i.get_op_code() == OpCode::Load ||
    i.get_op_code() == OpCode::VAArg ||
    i.get_op_code() == OpCode::ExtractValue ||
    i.get_op_code() >= OpCode::Trunc &&
    i.get_op_code() <= OpCode::AddrSpaceCast
  }
}
*/
pub trait UnaryInstruction: Instruction {
  fn get_operand(&self, _i: u32) -> Option<&Box<dyn Value>> { None }
}

struct UnaryOperator {
  uinst: Option<Box<dyn UnaryInstruction>>
}

impl UnaryOperator {
  pub fn new(_i_type: UnaryOps, _s: Box<dyn Value>, _v_type: Box<dyn Type>,
    _name: Twine, _ib: Option<Box<InstructionBase>>) -> Self
  {
    UnaryOperator { uinst: None, /*UnaryInstruction::new(v_type, i_type as u32, s, ib)*/ }
  }

  pub fn create_with_copied_flag() {}
  pub fn create_f_neg_fmf() {}

  pub fn get_op_code(&self) -> Option<UnaryOps> {
    if self.uinst.as_ref().unwrap().get_op_code() == OpCode::FNeg {
      return Some(UnaryOps::FNeg);
    }
    None
  }

  pub fn class_of(i: &InstructionBase) -> bool {
    i.is_unary_op()
  }
}

struct BinaryOperator {
  inst: InstructionBase
}

impl BinaryOperator {
  pub fn new(i_type: BinaryOps, _s1: Box<dyn Value>, _s2: Box<dyn Value>,
    v_type: Box<dyn Type>, _name: Twine, ib: Option<Box<InstructionBase>>) -> Self
  {
    BinaryOperator { inst: InstructionBase::new_ib(v_type, i_type as u32, None,
      0, ib) }
  }

  pub fn create_with_copied_flag() {}
  pub fn create_f_add_fmf() {}
  pub fn create_f_sub_fmf() {}
  pub fn create_f_mul_fmf() {}
  pub fn create_f_div_fmf() {}
  pub fn create_f_rem_fmf() {}
  pub fn create_nsw() {}
  pub fn create_nuw() {}
  pub fn create_exact() {}

  pub fn create_neg() {}
  pub fn create_nsw_neg() {}
  pub fn create_nuw_neg() {}
  pub fn create_not() {}

  pub fn get_op_code(&self) -> Option<BinaryOps> {
    /*
    let code = self.inst.get_op_code();
    if code == BinaryOps::Add as u32 {
      return Some(BinaryOps::Add);
    } else if code == BinaryOps::FAdd as u32 {
      return Some(BinaryOps::FAdd);
    } else if code == BinaryOps::Sub as u32 {
      return Some(BinaryOps::Sub);
    } else if code == BinaryOps::FSub as u32 {
      return Some(BinaryOps::FSub);
    } else if code == BinaryOps::Mul as u32 {
      return Some(BinaryOps::Mul);
    } else if code == BinaryOps::FMul as u32 {
      return Some(BinaryOps::FMul);
    } else if code == BinaryOps::UDiv as u32 {
      return Some(BinaryOps::UDiv);
    } else if code == BinaryOps::SDiv as u32 {
      return Some(BinaryOps::SDiv);
    } else if code == BinaryOps::FDiv as u32 {
      return Some(BinaryOps::FDiv);
    } else if code == BinaryOps::URem as u32 {
      return Some(BinaryOps::URem);
    } else if code == BinaryOps::SRem as u32 {
      return Some(BinaryOps::SRem);
    } else if code == BinaryOps::FRem as u32 {
      return Some(BinaryOps::FRem);
    } else if code == BinaryOps::Shl as u32 {
      return Some(BinaryOps::Shl);
    } else if code == BinaryOps::LShr as u32 {
      return Some(BinaryOps::LShr);
    } else if code == BinaryOps::AShr as u32 {
      return Some(BinaryOps::AShr);
    } else if code == BinaryOps::And as u32 {
      return Some(BinaryOps::And);
    } else if code == BinaryOps::Or as u32 {
      return Some(BinaryOps::Or);
    } else if code == BinaryOps::Xor as u32 {
      return Some(BinaryOps::Xor);
    }
    */
    None
  }

  pub fn swap_operands() {}

  pub fn class_of(i: &InstructionBase) -> bool {
    i.is_binary_op()
  }
}

// This is the base class for all instructions that perform data casts.
struct CastInst {
  uinst: Option<Box<dyn UnaryInstruction>>
}

impl CastInst {
  pub fn new(_v_type: Box<dyn Type>, _i_type: u32, _s: Box<dyn Value>,
    _name: Twine, _ib: Option<Box<InstructionBase>>) -> Self
  {
    CastInst { uinst: None /*UnaryInstruction::new(v_type, i_type, s, ib)*/ }
  }

  pub fn create_zext_or_bit_cast() {}
  pub fn create_sext_or_bit_cast() {}
  pub fn create_pointer_cast() {}
  pub fn create_pointer_bit_cast_or_addr_space_cast() {}
  pub fn create_bit_or_pointer_cast() {}
  pub fn crate_integer_cast() {}
  pub fn crate_fp_cast() {}
  pub fn create_trunc_or_bit_cast() {}
  pub fn is_bit_castable() {}
  pub fn is_bit_or_noop_pointer_castable() {}
  pub fn get_cast_op_code() {}
  pub fn is_integer_cast() {}
  pub fn is_noop_cast() {}
  pub fn is_eliminable_cast_pair() {}
  pub fn get_op_code() {}
  pub fn get_src_type() {}
  pub fn get_dest_type() {}
  pub fn cast_is_valid() {}

  pub fn class_of(i: &InstructionBase) -> bool {
    i.is_cast()
  }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Predicate {
  FCmpFalse = 0,
  FCmpOeq = 1,
  FCmpOgt = 2,
  FCmpOge = 3,
  FCmpOlt = 4,
  FCmpOle = 5,
  FCmpOne = 6,
  FCmpOrd = 7,
  FCmpUno = 8,
  FCmpUeq = 9,
  FCmpUgt = 10,
  FCmpUge = 11,
  FCmpUlt = 12,
  FCmpUle = 13,
  FCmpUne = 14,
  FCmpTrue = 15,
  ICmpEq = 32,
  ICmpNe = 33,
  ICmpUgt = 34,
  ICmpUge = 35,
  ICmpUlt = 36,
  ICmpUle = 37,
  ICmpSgt = 38,
  ICmpSge = 39,
  ICmpSlt = 40,
  ICmpSle = 41
}

// This class is the base class for the comparison instructions.
// Abstract base class of comparison instructions.
pub trait CmpInst: Instruction {
  fn fcmp_predicates(&self) {}
  fn icmp_predicates(&self) {}

  // Return the predicaate for this instruction.
  fn get_predicate(&self) -> &Predicate;

  // Set the predicate for this instruction to the specified type.
  fn set_predicate(&mut self, _p: Predicate) {}

  fn is_fp_predicate(&self) -> bool {
    &Predicate::FCmpFalse <= self.get_predicate() &&
      self.get_predicate() <= &Predicate::FCmpTrue
  }

  fn is_int_predicate(&self) -> bool {
    &Predicate::ICmpEq <= self.get_predicate() &&
      self.get_predicate() <= &Predicate::ICmpSle
  }

  fn get_predicate_name(&self) -> StringRef {
    StringRef::new()
  }

  fn get_inverse_predicate(&self) {}
  fn get_ordered_predicate(&self) {}
  fn get_unordered_predicate(&self) {}

  // Returns the predicate that would be the result of exchanging the
  // two operands of the CmpInst instruction without changing the result
  // produced.
  // Ex: EQ->EQ, SLE->SGE, ULT->UGT, OEQ->OEQ, etc.
  fn get_swapped_predicate(&self) -> &Predicate;

  fn is_strict_predicate(&self) {}
  fn is_non_strict_predicate(&self) {}
  fn get_strict_predicate(&self) {}
  fn get_non_strict_predicate(&self) {}
  fn get_flipped_strictness_predicate(&self) {}
  fn swap_operands(&self) {}

  // Determine if this CmpInst is commutative.
  fn is_commutative(&self) -> bool {
    self.is_equality()
  }

  // Determine if this is an equals/not equals predicate.
  fn is_equality(&self) -> bool;

  // Return true if the predicate is relational (not EQ or NE).
  fn is_relational(&self) -> bool {
    !self.is_equality()
  }

  // Return true if the comparison is signed, false otherwise.
  // Determine if this instruction is using a signed comparison.
  fn is_signed(&self) -> bool {
    match *self.get_predicate() {
      Predicate::ICmpSlt => return true,
      Predicate::ICmpSle => return true,
      Predicate::ICmpSgt => return true,
      Predicate::ICmpSge => return true,
      _ => return false
    }
  }

  // Return true if the comparison is unsigned, false otherwise.
  // Determine if this instruction is using an unsigned comparison.
  fn is_unsigned(&self) -> bool {
    match *self.get_predicate() {
      Predicate::ICmpUlt => return true,
      Predicate::ICmpUle => return true,
      Predicate::ICmpUgt => return true,
      Predicate::ICmpUge => return true,
      _ => return false
    }
  }

  // Returns the signed version of the predicate for this instruction (which
  // has to be an unsigned predicate).
  // Ex: EQ->EQ, SLE->SLE, UGT->SGT, etc.
  fn get_signed_predicate(&self) -> &Predicate {
    debug_assert!(self.is_unsigned(), "Call only with unsigned predicates.");
    let p = self.get_predicate();
    match p {
      Predicate::ICmpUlt => return &Predicate::ICmpSlt,
      Predicate::ICmpUle => return &Predicate::ICmpSle,
      Predicate::ICmpUgt => return &Predicate::ICmpSgt,
      Predicate::ICmpUge => return &Predicate::ICmpSge,
      _ => panic!("Unknown predicate.")
    }
  }

  // Returns the unsigned version of the predicate for this instruction (which
  // has to be an signed predicate).
  fn get_unsigned_predicate(&self) -> &Predicate {
    debug_assert!(self.is_signed(), "Call only with signed predicates.");
    let p = self.get_predicate();
    match p {
      Predicate::ICmpSlt => return &Predicate::ICmpUlt,
      Predicate::ICmpSle => return &Predicate::ICmpUle,
      Predicate::ICmpSgt => return &Predicate::ICmpUgt,
      Predicate::ICmpSge => return &Predicate::ICmpUge,
      _ => panic!("Unknown predicate.")
    }
  }

  fn get_flipped_signedness_predicate(&self) {}

  // Determine if the predicate is true when compareing a value with itself.
  fn is_true_when_equal(_p: Predicate) -> bool { false }

  // Determine if the predicate is false when compareing a value with itself.
  fn is_false_when_equal(_p: Predicate) -> bool { false }
}

// A container for an operand bundle being viewed as a set of values rather
// than a set of uses.
pub struct OperandBundleDefType<InputT> {
  tag: String,
  inputs: Vec<InputT>
}

impl<InputT> OperandBundleDefType<InputT> {
  pub fn new(tag: String, inputs: Vec<InputT>) -> Self {
    OperandBundleDefType { tag: tag, inputs: inputs }
  }

  pub fn inputs(&self) -> &Vec<InputT> {
    &self.inputs
  }

  pub fn input_size(&self) -> usize {
    self.inputs.len()
  }

  pub fn get_tag(&self) -> StringRef {
    StringRef::new_from_string(self.tag.as_str())
  }
}

// Base class for all callable instructions (InvokeInst and CallInst) holds
// everything related to calling a function.
pub struct CallBase {
  pub inst: InstructionBase,
  attrs: AttributeList,
  ft: FunctionType
}

impl CallBase {
  pub fn new_ib(v_type: Box<dyn Type>, i_type: u32, ops: Option<Use>, num_ops: u32,
    ib: Option<Box<InstructionBase>>, attrs: AttributeList, ft: FunctionType) -> Self {
    CallBase {
      inst: InstructionBase::new_ib(v_type, i_type, ops, num_ops, ib),
      attrs: attrs,
      ft: ft
    }
  }

  pub fn new_ie(v_type: Box<dyn Type>, i_type: u32, ops: Option<Use>, _num_ops: u32,
    ie: Option<BasicBlock>, attrs: AttributeList, ft: FunctionType) -> Self {
    CallBase {
      inst: InstructionBase::new_ie(v_type, i_type, ops, 0, ie),
      attrs: attrs,
      ft: ft
    }
  }

  pub fn has_descriptor(&self) -> bool { false }

  pub fn get_num_subclass_extra_operands(&self) -> u32 {
    if self.inst.get_op_code() == OpCode::Call {
      return 0;
    } else if self.inst.get_op_code() == OpCode::Invoke {
      return 2;
    } else if self.inst.get_op_code() == OpCode::CallBr {
      return self.get_num_subclass_extra_operands_dynamic();
    }
    panic!("Invalid opcode.");
  }

  pub fn get_num_subclass_extra_operands_dynamic(&self) -> u32 { 0 }
  pub fn add_operand_bundle() {}
  pub fn remove_operand_bundle() {}
  pub fn class_of() {}
  pub fn get_function_type() {}
  pub fn mutate_function_type() {}
  pub fn is_data_operand() {}
  pub fn get_data_operand_no() {}

  pub fn arg_size(&self) -> usize { 0 }
  pub fn arg_empty(&self) -> bool { false }

  pub fn get_arg_operand(&self, i: u32) -> Option<Box<dyn Value>> {
    debug_assert!(i < self.arg_size() as u32, "Out of bounds.");
    self.get_operand(i)
  }

  pub fn set_arg_operand(&mut self, i: u32, v: Option<Box<dyn Value>>) {
    debug_assert!(i < self.arg_size() as u32, "Out of bounds.");
    self.set_operand(i, v);
  }

  pub fn get_arg_operand_use(&self) {}

  pub fn is_arg_operand(&self, _u: Use) -> bool { false }

  // Given a use for a arg operand, get the arg operand number that
  // corresponds to it.
  pub fn get_arg_operand_no(&self, u: Use) -> u32{ 
    debug_assert!(self.is_arg_operand(u), "Arg operand # out of range.");
    0 // u - arg_begin()
  }

  // Returns true if this CallSite passes the given Value as an argument
  // to the called function.
  pub fn has_argument(&self, _v: Box<dyn Value>) -> bool { false }

  pub fn get_called_operand() {}
  pub fn get_called_operand_use() {}
  pub fn get_called_function() {}

  // Return true if the callsite is an indirect call.
  pub fn is_indirect_call(&self) -> bool { false }

  // Determine whether this Use is the callee operand's Use.
  pub fn is_callee(&self, _u: Use) -> bool { false }
  pub fn get_caller() {}

  // Tests i fthis call site must be tail call optimized.
  // Only a CallInst can be tail call optimized.
  pub fn is_must_tail_call(&self) -> bool { false }

  // Tests if this call site is marked as a tailcall.
  pub fn is_tail_call(&self) -> bool { false }

  pub fn get_intrinsic_id() {}

  pub fn set_called_operand(&mut self, _v: Box<dyn Value>) {}

  pub fn set_called_function() {}
  pub fn get_calling_conv() {}
  pub fn set_calling_conv() {}

  // Check if this call is an inline asm statement.
  pub fn is_inline_asm(&self) -> bool { false }

  // Return the parameter attributes for this call.
  pub fn get_attributes(&self) -> &AttributeList {
    &self.attrs
  }

  // Set the parameter attributes for this call.
  pub fn set_attributes(&mut self, attrs: AttributeList) {
    self.attrs = attrs;
  }

  // Determine whether this call has the given attribute.
  // If it does not the ndetermine if the called function has the attribute,
  // but only is the attibute is allowed for the call.
  pub fn has_fn_attr(&self, _kind: AttrKind) -> bool { false }

  // Adds the attribute to the list of attributes.
  pub fn add_attribute_at_index(&mut self, i: u32, attr: &Attribute) {
    self.attrs = self.attrs.add_attribute_at_index(
      blits_context_mut(), i as usize, attr);
  }

  // Adds the attribute to the function.
  pub fn add_fn_attr(&mut self, kind: AttrKind) {
    self.attrs = self.attrs.add_fn_attribute_by_kind(blits_context_mut(), &kind);
  }

  // Adds the attribute to the return value.
  pub fn add_ret_attr(&mut self, attr: &Attribute) {
    self.attrs = self.attrs.add_ret_attribute(blits_context_mut(), attr);
  }

  // Adds the attribute to the indicated argument.
  pub fn add_param_attr(&mut self, arg_no: u32, kind: AttrKind) {
    debug_assert!(arg_no > self.arg_size() as u32, "Out of bounds.");
    self.attrs = self.attrs.add_param_attribute_by_kind(
      blits_context_mut(), arg_no as usize, &kind);
  }

  // Remove the attribute from the list of attributes.
  pub fn remove_attribute_at_index(&mut self, i: u32, kind: AttrKind) {
    self.attrs = self.attrs.remove_attribute_at_index_by_kind(
      blits_context_mut(), i as usize, &kind);
  }

  pub fn remove_fn_attrs() {}

  // Removes the attribute from the function.
  pub fn remove_fn_attr(&mut self, kind: AttrKind) {
    self.attrs = self.attrs.remove_fn_attribute(blits_context_mut(), &kind);
  }

  // Removes the attribute from the return value.
  pub fn remove_ret_attr(&mut self, kind: AttrKind) {
    self.attrs = self.attrs.remove_ret_attirbute_by_kind(
      blits_context_mut(), &kind);
  }

  // Removes the attribute from the given argument.
  pub fn remove_param_attr(&mut self, arg_no: u32, kind: AttrKind) {
    self.attrs = self.attrs.remove_param_attribute_by_kind(
      blits_context_mut(), arg_no as usize, &kind);
  }

  pub fn remove_param_attrs() {}

  // Adds the dereferenceable attribute to the list of attributes.
  pub fn add_dereferenceable_param_attr(&mut self, i: u32, bytes: u64) {
    self.attrs = self.attrs.add_dereferenceable_param_attr(
      blits_context_mut(), i as usize, bytes);
  }

  // Adds the dereferenceable attribute to the list of attributes.
  pub fn add_dereferenceable_ret_attr(&mut self, bytes: u64) {
    self.attrs = self.attrs.add_dereferenceable_ret_attr(
      blits_context_mut(), bytes);
  }

  // Determine whether the return value has the given attribute.
  pub fn has_ret_attr(&self, _kind: AttrKind) -> bool { false }

  // Determine whether the argument or parameter has the given attribute.
  pub fn param_has_attr(&self, _arg_no: u32, _kind: AttrKind) -> bool { false }

  // Get the attribute of a given kind at a position.
  pub fn get_attribute_at_index(&self, i: usize, kind: &AttrKind) -> Option<Attribute> {
    self.attrs.get_attribute_at_index(i, kind)
  }

  // Get the attribute of a given kind for the function.
  pub fn get_fn_attr(&self, kind: &AttrKind) -> Attribute {
    let a = self.attrs.get_fn_attr_by_kind(kind);
    if a.is_some() || a.as_ref().unwrap().is_valid() {
      return a.unwrap();
    }
    a.unwrap() // TODO: get_fn_attr_on_called_function
  }

  // Get the attribute of a given kind from a given arg.
  pub fn get_param_attr(&self, arg_no: usize, kind: &AttrKind) -> Option<Attribute> {
    debug_assert!(arg_no < self.arg_size() as usize, "Out of bounds.");
    self.attrs.get_param_attr(arg_no, kind)
  }

  // Return true if the data operand at index i has the attribute a.
  pub fn data_operand_has_implied_attr(&self, _i: u32, _kind: AttrKind) -> bool {
    false
  }

  // Determine whether this data operand is not captured.
  pub fn does_not_capture(&self, op_no: u32) -> bool {
    self.data_operand_has_implied_attr(op_no, AttrKind::NoCapture)
  }

  // Determine whether this argument is passed by value.
  pub fn is_by_val_argument(&self, arg_no: u32) -> bool {
    self.param_has_attr(arg_no, AttrKind::ByVal)
  }

  // Determine whether this argument is passed in an alloca.
  pub fn is_in_alloca_argument(&self, arg_no: u32) -> bool {
    self.param_has_attr(arg_no, AttrKind::InAlloca)
  }

  // Determine whether this argument is passed by value, in alloca, or
  // is preallocated.
  pub fn is_pass_pointee_by_value_argument(&self, arg_no: u32) -> bool {
    self.param_has_attr(arg_no, AttrKind::ByVal) ||
    self.param_has_attr(arg_no, AttrKind::InAlloca) ||
    self.param_has_attr(arg_no, AttrKind::Preallocated)
  }

  // Determine whether passing undef to this argument is undefined behavior.
  pub fn is_passing_undef_ub(&self, arg_no: u32) -> bool {
    self.param_has_attr(arg_no, AttrKind::NoUndef) ||
    self.param_has_attr(arg_no, AttrKind::Dereferenceable) ||
    self.param_has_attr(arg_no, AttrKind::DereferenceableOrNull)
  }

  // Determine if there are is an inalloca argument.
  // Only the last argument can have the inalloca attribute.
  pub fn has_in_alloca_argument(&self) -> bool {
    !self.arg_empty() &&
    self.param_has_attr(self.arg_size() as u32 - 1, AttrKind::InAlloca)
  }

  pub fn does_not_access_memory(&self, op_no: u32) -> bool {
    self.data_operand_has_implied_attr(op_no, AttrKind::ReadNone)
  }

  pub fn only_reads_memory(&self, op_no: u32) -> bool {
    self.data_operand_has_implied_attr(op_no, AttrKind::ReadOnly) ||
    self.data_operand_has_implied_attr(op_no, AttrKind::ReadNone)
  }

  pub fn only_writes_memory(&self, op_no: u32) -> bool {
    self.data_operand_has_implied_attr(op_no, AttrKind::WriteOnly) ||
    self.data_operand_has_implied_attr(op_no, AttrKind::ReadNone)
  }

  pub fn get_ret_align() {}

  // Extract the alignment for a call or parameter.
  pub fn get_param_align(&self, arg_no: usize) -> Option<MaybeAlign> {
    self.attrs.get_param_alignment(arg_no)
  }

  pub fn get_param_stack_align(&self, arg_no: usize) -> Option<MaybeAlign> {
    self.attrs.get_param_stack_alignment(arg_no)
  }

  // Extract the byval type for a call or parameter.
  pub fn get_param_by_val_type(&self, arg_no: usize) -> Option<Box<dyn Type>> {
    if self.attrs.get_param_by_val_type(arg_no).is_some() {
      return self.attrs.get_param_by_val_type(arg_no);
    }
    None // TODO: get_called_function
  }

  // Extract the preallocated type for a call or parameter.
  pub fn get_param_preallocated_type(&self, arg_no: usize) -> Option<Box<dyn Type>> {
    if self.attrs.get_param_preallocated_type(arg_no).is_some() {
      return self.attrs.get_param_preallocated_type(arg_no);
    }
    None // TODO: get_called_function
  }

  // Extract the inalloca type for a call or parameter.
  pub fn get_param_in_alloca_type(&self, arg_no: usize) -> Option<Box<dyn Type>> {
    if self.attrs.get_param_in_alloca_type(arg_no).is_some() {
      return self.attrs.get_param_in_alloca_type(arg_no);
    }
    None // TODO: get_called_function
  }

  // Extract the sret type for a call or parameter.
  pub fn get_param_struct_ret_type(&self, arg_no: usize) -> Option<Box<dyn Type>> {
    if self.attrs.get_param_struct_ret_type(arg_no).is_some() {
      return self.attrs.get_param_struct_ret_type(arg_no);
    }
    None // TODO: get_called_function
  }

  // Extract the elementtype type for a parameter.
  pub fn get_param_element_type(&self, arg_no: usize) -> Option<Box<dyn Type>> {
    self.attrs.get_param_element_type(arg_no)
  }

  // Extract the number of dereferenceable bytes for a call or parameter.
  pub fn get_ret_dereferenceable_bytes(&self) -> u64 { 0 }

  // Extract the number of dereferenceable bytes for a call or parameter.
  pub fn get_param_dereferenceable_bytes(&self, i: usize) -> u64 {
    self.attrs.get_param_dereferenceable_bytes(i)
  }

  pub fn get_ret_dereferenceable_or_null_bytes() {}

  // Extract the number of dereferenceable_or_null bytes for a call or parameter.
  pub fn get_param_dereferenceable_or_null_bytes(&self, i: usize) -> u64 {
    self.attrs.get_param_dereferenceable_or_null_bytes(i)
  }

  pub fn get_ret_no_fp_class() {}
  pub fn get_param_no_fp_class() {}
  pub fn is_return_non_null() {}

  // Determine if the return value is merked with NoAlias attribute,
  pub fn return_does_not_alias(&self) -> bool {
    self.attrs.has_ret_attr(&AttrKind::NoAlias)
  }

  pub fn get_returned_arg_operand() {}
  pub fn get_arg_operand_with_attribute() {}

  // Return true if the call should not be treated as a call to a builtin.
  pub fn is_no_builtin(&self) -> bool {
    self.has_fn_attr(AttrKind::NoBuiltin) &&
    !self.has_fn_attr(AttrKind::Builtin)
  }

  // Determine if the call requires strict floating point semantics.
  pub fn is_strict_fp(&self) -> bool {
    self.has_fn_attr(AttrKind::StrictFP)
  }

  // Return true if the call should not be inlined.
  pub fn is_no_inline(&self) -> bool {
    self.has_fn_attr(AttrKind::NoInline)
  }

  pub fn set_is_no_inline(&mut self) {
    self.add_fn_attr(AttrKind::NoInline)
  }

  pub fn get_memory_effects(&self) -> MemoryEffects {
    MemoryEffects::new(0)
  }

  pub fn set_memory_effects(&self, _me: MemoryEffects) {}

  //pub fn does_not_access_memory() {}
  pub fn set_does_not_access_memory() {}
  //pub fn only_reads_memory() {}
  pub fn set_only_reads_memory() {}
  //pub fn only_writes_memory() {}
  pub fn set_only_writes_memory() {}
  pub fn only_accesses_arg_memory() {}
  pub fn set_only_accesses_arg_memory() {}
  pub fn only_accesses_inaccessible_memory() {}
  pub fn set_only_accesses_inaccessible_memory() {}

  // Determine if the call cannot return.
  pub fn does_not_return(&self) -> bool {
    self.has_fn_attr(AttrKind::NoReturn)
  }

  pub fn set_does_not_return(&mut self) {
    self.add_fn_attr(AttrKind::NoReturn)
  }

  // Determine if the call should not perform indirect branch tracking.
  pub fn does_no_cf_check(&self) -> bool {
    self.has_fn_attr(AttrKind::NoCfCheck)
  }

  // Determine if the call cannot unwind.
  pub fn does_not_throw(&self) -> bool {
    self.has_fn_attr(AttrKind::NoUnwind)
  }

  pub fn set_does_not_throw(&mut self) {
    self.add_fn_attr(AttrKind::NoUnwind)
  }

  // Determine if the invoke cannot be duplicated.
  pub fn cannot_duplicate(&self) -> bool {
    self.has_fn_attr(AttrKind::NoDuplicate)
  }

  pub fn set_cannot_duplicate(&mut self) {
    self.add_fn_attr(AttrKind::NoDuplicate)
  }

  // Determine if the call cannot be tail merged.
  pub fn cannot_merge(&self) -> bool {
    self.has_fn_attr(AttrKind::NoMerge)
  }

  pub fn set_cannot_merge(&mut self) {
    self.add_fn_attr(AttrKind::NoMerge)
  }

  // Determine if the invoke is convergent.
  pub fn is_convergent(&self) -> bool {
    self.has_fn_attr(AttrKind::Convergent)
  }

  pub fn set_convergent(&mut self) {
    self.add_fn_attr(AttrKind::Convergent)
  }

  pub fn set_not_convergent(&mut self) {
    self.remove_fn_attr(AttrKind::Convergent)
  }

  // Determine if the call returns a structure through first pointer argument.
  pub fn has_struct_ret_attr(&self) -> bool {
    if self.arg_empty() {
      return false;
    }
    self.param_has_attr(0, AttrKind::StructRet)
  }

  // Determine if any call argument is an aggregate passed by value.
  pub fn has_by_val_argument(&self) -> bool {
    self.attrs.has_attr_somewhere(&AttrKind::ByVal, None)
  }

  // Return the number of operand bundles associated with this user.
  pub fn get_num_operand_bundles(&self) -> u32 { 0 }

  // Return true if this user has any operand bundles.
  pub fn has_operand_bundles(&self) -> bool {
    self.get_num_operand_bundles() != 0
  }
  
  pub fn get_bundle_operands_start_index() {}
  pub fn get_bundle_operands_end_index() {}
  pub fn is_bundle_operand() {}
  pub fn is_operand_bundle_of_type() {}
  pub fn get_num_total_bundle_operands() {}
  pub fn get_operand_bundle_at() {}
  pub fn cannot_operand_bundles_of_type() {}
  pub fn count_operand_bundles_of_type() {}
  pub fn get_operand_bundle() {}
  pub fn get_operand_bundles_as_defs() {}
  pub fn get_operand_bundle_for_operand() {}
  pub fn has_reading_operand_bundles() {}
  pub fn has_clobbering_operand_bundles() {}
  pub fn bundle_operand_has_attr() {}
  pub fn has_identical_operand_bundle_schema() {}
  pub fn has_operand_bundles_other_than() {}
  pub fn operand_bundle_from_bundle_op_info() {}

  // Return the total number of values used in bundles.
  pub fn count_bundle_inputs(
    bundles: Vec<OperandBundleDefType<Box<dyn Value>>>) -> usize
  {
    let mut total = 0;
    for b in bundles {
      total += b.input_size();
    }
    total
  }

  // TODO
  pub fn get_operand(&self, _i: u32) -> Option<Box<dyn Value>> {
    None
  }

  // TODO
  pub fn set_operand(&mut self, _i: u32, _v: Option<Box<dyn Value>>) {}
}

struct FuncletPadInst {}
impl FuncletPadInst {
  pub fn new() {}
}