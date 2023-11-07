#![allow(dead_code)]

use std::any::Any;
use crate::{
  adt::{twine::Twine, string_ref::StringRef},
  ir::{
    basic_block::BasicBlock, instruction::{Instruction, OpCode},
    value::{Value, ValueType}, attributes::{AttributeList, AttrKind, Attribute},
    use_::Use, user::User, type_::{Type, FixedVectorType, VectorType, ScalableVectorType},
  },
  support::{alignment::MaybeAlign, mod_ref::MemoryEffects, type_size::ElementCount}
};

use super::{function::Function, global_value::IntrinsicID, type_::PointerType};

pub trait UnaryInstruction: Instruction {}

#[derive(Debug)]
pub struct UnaryOperator {
  s: Box<dyn Value>,
  t: Box<dyn Type>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl UnaryOperator {
  pub fn new_insert_before(s: Box<dyn Value>, t: Box<dyn Type>, name: Twine,
    ib: Option<Box<dyn Instruction>>) -> Self
  {
    UnaryOperator { s: s, t: t, name: name, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(s: Box<dyn Value>, t: Box<dyn Type>, name: Twine,
    ie: Option<BasicBlock>) -> Self
  {
    UnaryOperator { s: s, t: t, name: name, insert_before: None, insert_at_end: ie }
  }

  pub fn create_with_copied_flag(&self) {}
  pub fn create_fneg_fmf(&self) {}
  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::FNeg
  }
}

impl Value for UnaryOperator {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}
impl User for UnaryOperator {}
impl Instruction for UnaryOperator {
  fn get_op_code(&self) -> OpCode { OpCode::FNeg }
  fn as_any_inst(&self) -> &dyn Any { self }
}
impl UnaryInstruction for UnaryOperator {}

#[derive(Debug)]
pub struct BinaryOperator {
  opcode: OpCode,
  s1: Box<dyn Value>,
  s2: Box<dyn Value>,
  v_type: Box<dyn Type>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl BinaryOperator {
  pub fn new_insert_before(opcode: OpCode, s1: Box<dyn Value>,
    s2: Box<dyn Value>, v_type: Box<dyn Type>, name: Twine,
    ib: Option<Box<dyn Instruction>>) -> Self
  {
    BinaryOperator { opcode: opcode, s1: s1, s2: s2, v_type: v_type,
      name: name, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(opcode: OpCode, s1: Box<dyn Value>,
    s2: Box<dyn Value>, v_type: Box<dyn Type>, name: Twine,
    ie: Option<BasicBlock>) -> Self
  {
    BinaryOperator { opcode: opcode, s1: s1, s2: s2, v_type: v_type,
      name: name, insert_before: None, insert_at_end: ie }
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
  pub fn swap_operands() {}

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.is_binary_op()
  }
}

impl Value for BinaryOperator {
  fn get_type(&self) -> &dyn Type {
    self.v_type.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}
impl User for BinaryOperator {}
impl Instruction for BinaryOperator {
  fn get_op_code(&self) -> OpCode { self.opcode.clone() }
  fn as_any_inst(&self) -> &dyn Any { self }
}

// This is the base class for all instructions that perform data casts.
// It is simply provided so that instruction category testing can be performed
// with code like: if (isa<CastInst>(Instr)) { ... }
pub trait CastInst: UnaryInstruction {
  fn create_zext_or_bit_cast(&self) {}
  fn create_sext_or_bit_cast(&self) {}
  fn create_pointer_cast(&self) {}
  fn create_pointer_bit_cast_or_addr_space_cast(&self) {}
  fn create_bit_or_pointer_cast(&self) {}
  fn crate_integer_cast(&self) {}
  fn crate_fp_cast(&self) {}
  fn create_trunc_or_bit_cast(&self) {}
  //fn is_bit_castable(&self) {}
  fn is_bit_or_noop_pointer_castable(&self) {}
  //fn get_cast_op_code(&self) {}
  fn is_integer_cast(&self) {}
  fn is_noop_cast(&self) {}
  fn is_eliminable_cast_pair(&self) {}
  fn get_op_code(&self) {}
  fn get_src_type(&self) {}
  fn get_dest_type(&self) {}
  fn cast_is_valid(&self) {}
}

// Check whether a bitcast between these types is valid.
pub fn is_bit_castable(src_org_t: &dyn Type, dst_org_t: &dyn Type) -> bool {
  let mut src_t = src_org_t;
  let mut dst_t = dst_org_t;
  if !src_t.is_first_class_type() || !dst_t.is_first_class_type() {
    return false;
  }

  // src_t == dst_t

  if src_t.as_any().downcast_ref::<FixedVectorType>().is_some() {
    if dst_t.as_any().downcast_ref::<FixedVectorType>().is_some() {
      let src_vec_t = src_t.as_any().downcast_ref::<FixedVectorType>();
      let dst_vec_t = dst_t.as_any().downcast_ref::<FixedVectorType>();
      if src_vec_t.unwrap().get_element_count() == dst_vec_t.unwrap().get_element_count() {
        src_t = src_vec_t.unwrap().get_element_type();
        dst_t = dst_vec_t.unwrap().get_element_type();
      }
    }
  }

  if src_t.as_any().downcast_ref::<ScalableVectorType>().is_some() {
    if dst_t.as_any().downcast_ref::<ScalableVectorType>().is_some() {
      let src_vec_t = src_t.as_any().downcast_ref::<ScalableVectorType>();
      let dst_vec_t = dst_t.as_any().downcast_ref::<ScalableVectorType>();
      if src_vec_t.unwrap().get_element_count() == dst_vec_t.unwrap().get_element_count() {
        //src_t = src_vec_t.unwrap().get_element_type();
        //dst_t = dst_vec_t.unwrap().get_element_type();
      }
    }
  }

  if src_t.as_any().downcast_ref::<PointerType>().is_some() {
    if dst_t.as_any().downcast_ref::<PointerType>().is_some() {
      let src_ptr_t = src_t.as_any().downcast_ref::<PointerType>();
      let dst_ptr_t = dst_t.as_any().downcast_ref::<PointerType>();
      return src_ptr_t.unwrap().get_address_space() == dst_ptr_t.unwrap().get_address_space();
    }
  }

  let src_bits = src_t.get_primitive_size_in_bits();
  let dst_bits = dst_t.get_primitive_size_in_bits();
  if src_bits.get_known_min_value() == 0 || dst_bits.get_known_min_value() == 0 {
    return false;
  }

  if src_bits != dst_bits {
    return false;
  }

  if dst_t.is_x86_mmx_type() || src_t.is_x86_mmx_type() {
    return  false;
  }

  true
}

// Returns the opcode necessary to cast src_val into dest_t using usual casting rules.
// Infer the opcode for cast operand and type.
pub fn get_cast_op_code(src_val: &dyn Value, src_is_signed: bool,
  dst_orig_t: &dyn Type, dst_is_signed: bool) -> OpCode
{
  let mut src_t = src_val.get_type();
  let mut dst_t = dst_orig_t;
  debug_assert!(src_t.is_first_class_type() && dst_t.is_first_class_type(),
    "Only first class types are castable.");
  
  // if src_t == dst_t { retturn BitCast; }

  if src_t.as_any().downcast_ref::<FixedVectorType>().is_some() {
    if dst_t.as_any().downcast_ref::<FixedVectorType>().is_some() {
      let src_vec_t = src_t.as_any().downcast_ref::<FixedVectorType>();
      let dst_vec_t = dst_t.as_any().downcast_ref::<FixedVectorType>();
      if src_vec_t.unwrap().get_element_count() == dst_vec_t.unwrap().get_element_count() {
        src_t = src_vec_t.unwrap().get_element_type();
        dst_t = dst_vec_t.unwrap().get_element_type();
      }
    }
  }
  
  if src_t.as_any().downcast_ref::<ScalableVectorType>().is_some() {
    if dst_t.as_any().downcast_ref::<ScalableVectorType>().is_some() {
      let src_vec_t = src_t.as_any().downcast_ref::<ScalableVectorType>();
      let dst_vec_t = dst_t.as_any().downcast_ref::<ScalableVectorType>();
      if src_vec_t.unwrap().get_element_count() == dst_vec_t.unwrap().get_element_count() {
        //src_t = src_vec_t.unwrap().get_element_type();
        //dst_t = dst_vec_t.unwrap().get_element_type();
      }
    }
  }

  let src_bits = src_t.get_primitive_size_in_bits();
  let dst_bits = dst_t.get_primitive_size_in_bits();

  if dst_t.is_integer_type() {
    if src_t.is_integer_type() {
      if dst_bits < src_bits {
        return OpCode::Trunc;
      } else if dst_bits > src_bits {
        if src_is_signed {
          return OpCode::SExt;
        } else {
          return OpCode::ZExt;
        }
      } else {
        return OpCode::BitCast;
      }
    } else if src_t.is_floating_point_type() {
      if dst_is_signed {
        return OpCode::FPToSI;
      } else {
        return OpCode::FPToUI;
      }
    } else if src_t.is_vector_type() {
      debug_assert!(dst_bits == src_bits,
        "Casting vector to integer of different width.");
      return OpCode::BitCast;
    } else {
      debug_assert!(src_t.is_pointer_type(),
        "Casting from a valur that is not first-class type.");
      return OpCode::PtrToInt;
    }
  } else if dst_t.is_floating_point_type() {
    if src_t.is_integer_type() {
      if src_is_signed {
        return OpCode::SIToFp;
      } else {
        return OpCode::UIToFP;
      }
    } else if src_t.is_floating_point_type() {
      if dst_bits < src_bits {
        return OpCode::FPTrunc;
      } else if dst_bits > src_bits {
        return OpCode::FPExt;
      } else {
        return OpCode::BitCast;
      }
    } else if src_t.is_vector_type() {
      debug_assert!(dst_bits == src_bits,
        "Casting vector to floating point of different width.");
      return OpCode::BitCast;
    }
    unreachable!("Casting pointer or non-first class to float.");
  } else if dst_t.is_vector_type() {
    debug_assert!(dst_bits == src_bits,
      "Illegal cast to vector (wrong type or size).");
    return OpCode::BitCast;
  } else if dst_t.is_pointer_type() {
    if src_t.is_pointer_type() {
      if dst_t.get_pointer_address_space() != src_t.get_pointer_address_space() {
        return OpCode::AddrSpaceCast;
      }
      return OpCode::BitCast;
    } else if src_t.is_integer_type() {
      return OpCode::IntToPtr;
    }
    unreachable!("Casting pointer to other than pointer or int.");
  } else if dst_t.is_x86_mmx_type() {
    if src_t.is_vector_type() {
      debug_assert!(dst_bits == src_bits,
        "Casting vector of wrong width to X86_MMX.");
      return OpCode::BitCast;
    }
    unreachable!("Illegal cast to X86_MMX.");
  }

  unreachable!("Casting to type that is not first-class.");
}

pub fn cast_is_valid(op: OpCode, src_t: &dyn Type, dst_t: &dyn Type) -> bool {
  if !src_t.is_first_class_type() || !dst_t.is_first_class_type() ||
    src_t.is_aggregate_type() || dst_t.is_aggregate_type() {
    return false;
  }

  let src_scalar_bit_size = src_t.get_scalar_size_in_bits();
  let dst_scalar_bit_size = dst_t.get_scalar_size_in_bits();

  let mut src_ec = ElementCount::get_fixed(0);
  if src_t.is_vector_type() {
    let src_fvec_t =
      src_t.as_any().downcast_ref::<FixedVectorType>();
    if src_fvec_t.is_some() {
      src_ec = src_fvec_t.unwrap().get_element_count();
    }
    // sclar vector type
    let src_svec_t =
      src_t.as_any().downcast_ref::<ScalableVectorType>();
    if src_svec_t.is_some() {
      src_ec = src_svec_t.unwrap().get_element_count();
    }
  }

  let mut dst_ec = ElementCount::get_fixed(0);
  if dst_t.is_vector_type() {
    let dst_fvec_t =
      dst_t.as_any().downcast_ref::<FixedVectorType>();
    if dst_fvec_t.is_some() {
      dst_ec = dst_fvec_t.unwrap().get_element_count();
    }
    // sclar vector type
    let dst_svec_t =
      dst_t.as_any().downcast_ref::<ScalableVectorType>();
    if dst_svec_t.is_some() {
      dst_ec = dst_svec_t.unwrap().get_element_count();
    }
  }

  match op {
    OpCode::Trunc => {
      return src_t.is_int_or_int_vector_type() && dst_t.is_int_or_int_vector_type() &&
        src_ec == dst_ec && src_scalar_bit_size > dst_scalar_bit_size;
    },
    OpCode::ZExt => {
      return src_t.is_int_or_int_vector_type() && dst_t.is_int_or_int_vector_type() &&
        src_ec == dst_ec && src_scalar_bit_size < dst_scalar_bit_size;
    },
    OpCode::SExt => {
      return src_t.is_int_or_int_vector_type() && dst_t.is_int_or_int_vector_type() &&
        src_ec == dst_ec && src_scalar_bit_size < dst_scalar_bit_size;
    },
    OpCode::FPTrunc => {
      return src_t.is_fp_or_fpvector_type() && dst_t.is_fp_or_fpvector_type() &&
        src_ec == dst_ec && src_scalar_bit_size > dst_scalar_bit_size;
    },
    OpCode::FPExt => {
      return src_t.is_fp_or_fpvector_type() && dst_t.is_fp_or_fpvector_type() &&
        src_ec == dst_ec && src_scalar_bit_size < dst_scalar_bit_size;
    },
    OpCode::FPToUI => {
      return src_t.is_fp_or_fpvector_type() && dst_t.is_int_or_int_vector_type() &&
        src_ec == dst_ec;
    },
    OpCode::FPToSI => {
      return src_t.is_fp_or_fpvector_type() && dst_t.is_int_or_int_vector_type() &&
        src_ec == dst_ec;
    },
    OpCode::PtrToInt => {
      if src_ec != dst_ec { return false; }
      return src_t.is_ptr_or_ptr_vector_type() && dst_t.is_int_or_int_vector_type();
    },
    OpCode::IntToPtr => {
      if src_ec != dst_ec { return false; }
      return src_t.is_int_or_int_vector_type() && dst_t.is_ptr_or_ptr_vector_type();
    },
    OpCode::BitCast => {
      let src_ptr_t =
        src_t.get_scalar_type().as_any().downcast_ref::<PointerType>();
      let dst_ptr_t =
        dst_t.get_scalar_type().as_any().downcast_ref::<PointerType>();
      // For non-pointer cases, the cast is okay if the source and destination bit
      // widths are identical.
      if src_ptr_t.is_none() {
        return src_t.get_primitive_size_in_bits() == dst_t.get_primitive_size_in_bits();
      }
      // If both are pointers then the address spaces must match.
      if dst_ptr_t.is_some() {
        if src_ptr_t.unwrap().get_address_space() != dst_ptr_t.unwrap().get_address_space() {
          return false;
        }
      }
      // A vector of pointers must have the same number of elements.
      if src_t.is_vector_type() && dst_t.is_vector_type() {
        return src_ec == dst_ec;
      }
      if src_t.is_vector_type() {
        return src_ec == ElementCount::get_fixed(1);
      }
      if dst_t.is_vector_type() {
        return dst_ec == ElementCount::get_fixed(1);
      }
      return true;
    },
    OpCode::AddrSpaceCast => {
      let src_ptr_t =
        src_t.get_scalar_type().as_any().downcast_ref::<PointerType>();
      if src_ptr_t.is_none() { return false; }
      let dst_ptr_t =
        dst_t.get_scalar_type().as_any().downcast_ref::<PointerType>();
      if dst_ptr_t.is_none() { return false; }

      if src_ptr_t.unwrap().get_address_space() == dst_ptr_t.unwrap().get_address_space() {
        return false;
      }
      return src_ec == dst_ec;
    },
    _ => return false
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
#[derive(Debug)]
pub struct OperandBundleDef<InputT> {
  tag: String,
  inputs: Vec<InputT>
}

impl<InputT> OperandBundleDef<InputT> {
  pub fn new(tag: String, inputs: Vec<InputT>) -> Self {
    OperandBundleDef { tag: tag, inputs: inputs }
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
/*
pub struct CallBase {
  pub inst: InstructionBase,
  attrs: AttributeList,
  ft: FunctionType
}
*/

pub trait CallBase: Instruction {

  fn has_descriptor(&self) -> bool { false }

  fn get_num_subclass_extra_operands(&self) -> u32 {
    /*
    if self.inst.get_op_code() == OpCode::Call {
      return 0;
    } else if self.inst.get_op_code() == OpCode::Invoke {
      return 2;
    } else if self.inst.get_op_code() == OpCode::CallBr {
      return self.get_num_subclass_extra_operands_dynamic();
    }
    panic!("Invalid opcode.");
    */
    0
  }

  fn get_num_subclass_extra_operands_dynamic(&self) -> u32 { 0 }
  fn add_operand_bundle(&self) {}
  fn remove_operand_bundle(&self) {}
  fn class_of(&self) {}
  fn get_function_type(&self) {}
  fn mutate_function_type(&self) {}
  fn is_data_operand(&self) {}
  fn get_data_operand_no(&self) {}

  fn arg_size(&self) -> usize { 0 }
  fn arg_empty(&self) -> bool { false }

  fn get_arg_operand(&self, i: usize) -> Option<&Box<dyn Value>> {
    debug_assert!(i < self.arg_size(), "Out of bounds.");
    self.get_operand(i)
  }

  fn set_arg_operand(&mut self, i: usize, v: Option<Box<dyn Value>>) {
    debug_assert!(i < self.arg_size(), "Out of bounds.");
    self.set_operand(i, v.unwrap());
  }

  fn get_arg_operand_use(&self) {}

  fn is_arg_operand(&self, _u: Use) -> bool { false }

  // Given a use for a arg operand, get the arg operand number that
  // corresponds to it.
  fn get_arg_operand_no(&self, u: Use) -> u32{ 
    debug_assert!(self.is_arg_operand(u), "Arg operand # out of range.");
    0 // u - arg_begin()
  }

  // Returns true if this CallSite passes the given Value as an argument
  // to the called function.
  fn has_argument(&self, _v: Box<dyn Value>) -> bool { false }

  fn get_called_operand(&self) -> Option<Box<dyn Value>> { None }

  fn get_called_operand_use(&self) {}

  // Returns the function called, or None if this is an indirect function
  // invocation or the function signature does not match the call signature.
  fn get_called_function(&self) -> Option<Function> {
    //let f =
      //self.get_called_operand().as_ref().unwrap().as_any().downcast_ref::<Function>();
    //if f.is_some() {

    //}
    None
  }

  // Return true if the callsite is an indirect call.
  fn is_indirect_call(&self) -> bool { false }

  // Determine whether this Use is the callee operand's Use.
  fn is_callee(&self, _u: Use) -> bool { false }
  fn get_caller(&self) {}

  // Tests i fthis call site must be tail call optimized.
  // Only a CallInst can be tail call optimized.
  fn is_must_tail_call(&self) -> bool { false }

  // Tests if this call site is marked as a tailcall.
  fn is_tail_call(&self) -> bool { false }

  // Returns the intrinsic ID of the intrinsic called or NotIntrinsic if the
  // called function is not an intrinsic, or if this is an indirect call.
  fn get_intrinsic_id(&self) -> IntrinsicID { IntrinsicID::NotIntrinsic }

  fn set_called_operand(&mut self, _v: Box<dyn Value>) {}

  fn set_called_function(&self) {}
  fn get_calling_conv(&self) {}
  fn set_calling_conv(&self) {}

  // Check if this call is an inline asm statement.
  fn is_inline_asm(&self) -> bool { false }

  // Return the parameter attributes for this call.
  //fn get_attributes(&self) -> &AttributeList {
    //&self.attrs
  //}

  // Set the parameter attributes for this call.
  fn set_attributes(&mut self, _attrs: AttributeList) {
    //self.attrs = attrs;
  }

  // Determine whether this call has the given attribute.
  // If it does not the ndetermine if the called function has the attribute,
  // but only is the attibute is allowed for the call.
  fn has_fn_attr(&self, _kind: AttrKind) -> bool { false }

  // Adds the attribute to the list of attributes.
  fn add_attribute_at_index(&mut self, _i: u32, _attr: &Attribute) {
    //self.attrs = self.attrs.add_attribute_at_index(
      //blits_context_mut(), i as usize, attr);
  }

  // Adds the attribute to the function.
  fn add_fn_attr(&mut self, _kind: AttrKind) {
    //self.attrs = self.attrs.add_fn_attribute_by_kind(blits_context_mut(), &kind);
  }

  // Adds the attribute to the return value.
  fn add_ret_attr(&mut self, _attr: &Attribute) {
    //self.attrs = self.attrs.add_ret_attribute(blits_context_mut(), attr);
  }

  // Adds the attribute to the indicated argument.
  fn add_param_attr(&mut self, arg_no: u32, _kind: AttrKind) {
    debug_assert!(arg_no > self.arg_size() as u32, "Out of bounds.");
    //self.attrs = self.attrs.add_param_attribute_by_kind(
      //blits_context_mut(), arg_no as usize, &kind);
  }

  // Remove the attribute from the list of attributes.
  fn remove_attribute_at_index(&mut self, _i: u32, _kind: AttrKind) {
    //self.attrs = self.attrs.remove_attribute_at_index_by_kind(
      //blits_context_mut(), i as usize, &kind);
  }

  fn remove_fn_attrs(&self) {}

  // Removes the attribute from the function.
  fn remove_fn_attr(&mut self, _kind: AttrKind) {
    //self.attrs = self.attrs.remove_fn_attribute(blits_context_mut(), &kind);
  }

  // Removes the attribute from the return value.
  fn remove_ret_attr(&mut self, _kind: AttrKind) {
    //self.attrs = self.attrs.remove_ret_attirbute_by_kind(
      //blits_context_mut(), &kind);
  }

  // Removes the attribute from the given argument.
  fn remove_param_attr(&mut self, _arg_no: u32, _kind: AttrKind) {
    //self.attrs = self.attrs.remove_param_attribute_by_kind(
      //blits_context_mut(), arg_no as usize, &kind);
  }

  fn remove_param_attrs(&self) {}

  // Adds the dereferenceable attribute to the list of attributes.
  fn add_dereferenceable_param_attr(&mut self, _i: u32, _bytes: u64) {
    //self.attrs = self.attrs.add_dereferenceable_param_attr(
      //blits_context_mut(), i as usize, bytes);
  }

  // Adds the dereferenceable attribute to the list of attributes.
  fn add_dereferenceable_ret_attr(&mut self, _bytes: u64) {
    //self.attrs = self.attrs.add_dereferenceable_ret_attr(
      //blits_context_mut(), bytes);
  }

  // Determine whether the return value has the given attribute.
  fn has_ret_attr(&self, _kind: AttrKind) -> bool { false }

  // Determine whether the argument or parameter has the given attribute.
  fn param_has_attr(&self, _arg_no: u32, _kind: AttrKind) -> bool { false }

  // Get the attribute of a given kind at a position.
  fn get_attribute_at_index(&self, _i: usize, _kind: &AttrKind) -> Option<Attribute> {
    //self.attrs.get_attribute_at_index(i, kind)
    None
  }

  // Get the attribute of a given kind for the function.
  //fn get_fn_attr(&self, _kind: &AttrKind) -> Attribute {
    //let a = self.attrs.get_fn_attr_by_kind(kind);
    //if a.is_some() || a.as_ref().unwrap().is_valid() {
      //return a.unwrap();
    //}
    //a.unwrap() // TODO: get_fn_attr_on_called_function
  //}

  // Get the attribute of a given kind from a given arg.
  fn get_param_attr(&self, arg_no: usize, _kind: &AttrKind) -> Option<Attribute> {
    debug_assert!(arg_no < self.arg_size() as usize, "Out of bounds.");
    //self.attrs.get_param_attr(arg_no, kind)
    None
  }

  // Return true if the data operand at index i has the attribute a.
  fn data_operand_has_implied_attr(&self, _i: u32, _kind: AttrKind) -> bool {
    false
  }

  // Determine whether this data operand is not captured.
  fn does_not_capture(&self, op_no: u32) -> bool {
    self.data_operand_has_implied_attr(op_no, AttrKind::NoCapture)
  }

  // Determine whether this argument is passed by value.
  fn is_by_val_argument(&self, arg_no: u32) -> bool {
    self.param_has_attr(arg_no, AttrKind::ByVal)
  }

  // Determine whether this argument is passed in an alloca.
  fn is_in_alloca_argument(&self, arg_no: u32) -> bool {
    self.param_has_attr(arg_no, AttrKind::InAlloca)
  }

  // Determine whether this argument is passed by value, in alloca, or
  // is preallocated.
  fn is_pass_pointee_by_value_argument(&self, arg_no: u32) -> bool {
    self.param_has_attr(arg_no, AttrKind::ByVal) ||
    self.param_has_attr(arg_no, AttrKind::InAlloca) ||
    self.param_has_attr(arg_no, AttrKind::Preallocated)
  }

  // Determine whether passing undef to this argument is undefined behavior.
  fn is_passing_undef_ub(&self, arg_no: u32) -> bool {
    self.param_has_attr(arg_no, AttrKind::NoUndef) ||
    self.param_has_attr(arg_no, AttrKind::Dereferenceable) ||
    self.param_has_attr(arg_no, AttrKind::DereferenceableOrNull)
  }

  // Determine if there are is an inalloca argument.
  // Only the last argument can have the inalloca attribute.
  fn has_in_alloca_argument(&self) -> bool {
    !self.arg_empty() &&
    self.param_has_attr(self.arg_size() as u32 - 1, AttrKind::InAlloca)
  }

  fn does_not_access_memory(&self, op_no: u32) -> bool {
    self.data_operand_has_implied_attr(op_no, AttrKind::ReadNone)
  }

  fn only_reads_memory(&self, op_no: u32) -> bool {
    self.data_operand_has_implied_attr(op_no, AttrKind::ReadOnly) ||
    self.data_operand_has_implied_attr(op_no, AttrKind::ReadNone)
  }

  fn only_writes_memory(&self, op_no: u32) -> bool {
    self.data_operand_has_implied_attr(op_no, AttrKind::WriteOnly) ||
    self.data_operand_has_implied_attr(op_no, AttrKind::ReadNone)
  }

  fn get_ret_align(&self) {}

  // Extract the alignment for a call or parameter.
  fn get_param_align(&self, _arg_no: usize) -> Option<MaybeAlign> {
    //self.attrs.get_param_alignment(arg_no)
    None
  }

  fn get_param_stack_align(&self, _arg_no: usize) -> Option<MaybeAlign> {
    //self.attrs.get_param_stack_alignment(arg_no)
    None
  }

  // Extract the byval type for a call or parameter.
  fn get_param_by_val_type(&self, _arg_no: usize) -> Option<Box<dyn Type>> {
    //if self.attrs.get_param_by_val_type(arg_no).is_some() {
      //return self.attrs.get_param_by_val_type(arg_no);
    //}
    None // TODO: get_called_function
  }

  // Extract the preallocated type for a call or parameter.
  fn get_param_preallocated_type(&self, _arg_no: usize) -> Option<Box<dyn Type>> {
    //if self.attrs.get_param_preallocated_type(arg_no).is_some() {
      //return self.attrs.get_param_preallocated_type(arg_no);
    //}
    None // TODO: get_called_function
  }

  // Extract the inalloca type for a call or parameter.
  fn get_param_in_alloca_type(&self, _arg_no: usize) -> Option<Box<dyn Type>> {
    //if self.attrs.get_param_in_alloca_type(arg_no).is_some() {
      //return self.attrs.get_param_in_alloca_type(arg_no);
    //}
    None // TODO: get_called_function
  }

  // Extract the sret type for a call or parameter.
  fn get_param_struct_ret_type(&self, _arg_no: usize) -> Option<Box<dyn Type>> {
    //if self.attrs.get_param_struct_ret_type(arg_no).is_some() {
      //return self.attrs.get_param_struct_ret_type(arg_no);
    //}
    None // TODO: get_called_function
  }

  // Extract the elementtype type for a parameter.
  fn get_param_element_type(&self, _arg_no: usize) -> Option<Box<dyn Type>> {
    //self.attrs.get_param_element_type(arg_no)
    None
  }

  // Extract the number of dereferenceable bytes for a call or parameter.
  fn get_ret_dereferenceable_bytes(&self) -> u64 { 0 }

  // Extract the number of dereferenceable bytes for a call or parameter.
  fn get_param_dereferenceable_bytes(&self, _i: usize) -> u64 {
    //self.attrs.get_param_dereferenceable_bytes(i)
    0
  }

  fn get_ret_dereferenceable_or_null_bytes(&self) {}

  // Extract the number of dereferenceable_or_null bytes for a call or parameter.
  fn get_param_dereferenceable_or_null_bytes(&self, _i: usize) -> u64 {
    //self.attrs.get_param_dereferenceable_or_null_bytes(i)
    0
  }

  fn get_ret_no_fp_class(&self) {}
  fn get_param_no_fp_class(&self) {}
  fn is_return_non_null(&self) {}

  // Determine if the return value is merked with NoAlias attribute,
  fn return_does_not_alias(&self) -> bool {
    //self.attrs.has_ret_attr(&AttrKind::NoAlias)
    false
  }

  fn get_returned_arg_operand(&self) {}
  fn get_arg_operand_with_attribute(&self) {}

  // Return true if the call should not be treated as a call to a builtin.
  fn is_no_builtin(&self) -> bool {
    self.has_fn_attr(AttrKind::NoBuiltin) &&
    !self.has_fn_attr(AttrKind::Builtin)
  }

  // Determine if the call requires strict floating point semantics.
  fn is_strict_fp(&self) -> bool {
    self.has_fn_attr(AttrKind::StrictFP)
  }

  // Return true if the call should not be inlined.
  fn is_no_inline(&self) -> bool {
    self.has_fn_attr(AttrKind::NoInline)
  }

  fn set_is_no_inline(&mut self) {
    self.add_fn_attr(AttrKind::NoInline)
  }

  fn get_memory_effects(&self) -> MemoryEffects {
    MemoryEffects::new(0)
  }

  fn set_memory_effects(&self, _me: MemoryEffects) {}

  //fn does_not_access_memory() {}
  fn set_does_not_access_memory(&self) {}
  //fn only_reads_memory() {}
  fn set_only_reads_memory(&self) {}
  //fn only_writes_memory() {}
  fn set_only_writes_memory(&self) {}
  fn only_accesses_arg_memory(&self) {}
  fn set_only_accesses_arg_memory(&self) {}
  fn only_accesses_inaccessible_memory(&self) {}
  fn set_only_accesses_inaccessible_memory(&self) {}

  // Determine if the call cannot return.
  fn does_not_return(&self) -> bool {
    self.has_fn_attr(AttrKind::NoReturn)
  }

  fn set_does_not_return(&mut self) {
    self.add_fn_attr(AttrKind::NoReturn)
  }

  // Determine if the call should not perform indirect branch tracking.
  fn does_no_cf_check(&self) -> bool {
    self.has_fn_attr(AttrKind::NoCfCheck)
  }

  // Determine if the call cannot unwind.
  fn does_not_throw(&self) -> bool {
    self.has_fn_attr(AttrKind::NoUnwind)
  }

  fn set_does_not_throw(&mut self) {
    self.add_fn_attr(AttrKind::NoUnwind)
  }

  // Determine if the invoke cannot be duplicated.
  fn cannot_duplicate(&self) -> bool {
    self.has_fn_attr(AttrKind::NoDuplicate)
  }

  fn set_cannot_duplicate(&mut self) {
    self.add_fn_attr(AttrKind::NoDuplicate)
  }

  // Determine if the call cannot be tail merged.
  fn cannot_merge(&self) -> bool {
    self.has_fn_attr(AttrKind::NoMerge)
  }

  fn set_cannot_merge(&mut self) {
    self.add_fn_attr(AttrKind::NoMerge)
  }

  // Determine if the invoke is convergent.
  fn is_convergent(&self) -> bool {
    self.has_fn_attr(AttrKind::Convergent)
  }

  fn set_convergent(&mut self) {
    self.add_fn_attr(AttrKind::Convergent)
  }

  fn set_not_convergent(&mut self) {
    self.remove_fn_attr(AttrKind::Convergent)
  }

  // Determine if the call returns a structure through first pointer argument.
  fn has_struct_ret_attr(&self) -> bool {
    if self.arg_empty() {
      return false;
    }
    self.param_has_attr(0, AttrKind::StructRet)
  }

  // Determine if any call argument is an aggregate passed by value.
  fn has_by_val_argument(&self) -> bool {
    //self.attrs.has_attr_somewhere(&AttrKind::ByVal, None)
    false
  }

  // Return the number of operand bundles associated with this user.
  fn get_num_operand_bundles(&self) -> u32 { 0 }

  // Return true if this user has any operand bundles.
  fn has_operand_bundles(&self) -> bool {
    self.get_num_operand_bundles() != 0
  }
  
  fn get_bundle_operands_start_index(&self) {}
  fn get_bundle_operands_end_index(&self) {}
  fn is_bundle_operand(&self) {}
  fn is_operand_bundle_of_type(&self) {}
  fn get_num_total_bundle_operands(&self) {}
  fn get_operand_bundle_at(&self) {}
  fn cannot_operand_bundles_of_type(&self) {}
  fn count_operand_bundles_of_type(&self) {}
  fn get_operand_bundle(&self) {}
  fn get_operand_bundles_as_defs(&self) {}
  fn get_operand_bundle_for_operand(&self) {}
  fn has_reading_operand_bundles(&self) {}
  fn has_clobbering_operand_bundles(&self) {}
  fn bundle_operand_has_attr(&self) {}
  fn has_identical_operand_bundle_schema(&self) {}
  fn has_operand_bundles_other_than(&self) {}
  fn operand_bundle_from_bundle_op_info(&self) {}

  // Return the total number of values used in bundles.
  fn count_bundle_inputs(&self,
    bundles: Vec<OperandBundleDef<Box<dyn Value>>>) -> usize
  {
    let mut total = 0;
    for b in bundles {
      total += b.input_size();
    }
    total
  }
}

pub trait FuncletPadInst: Instruction {
  fn arg_size(&self) -> usize { 0 }

  // Return the outer EH-pad this funclet is nested within.
  fn get_parent_pad(&self) -> Option<&Box<dyn Value>> { None }

  fn set_parent_pad(&mut self, _parent_pad: Box<dyn Value>) {}

  fn get_arg_operand(&self, _i: usize) -> Option<&Box<dyn Value>> { None }

  fn set_arg_operand(&mut self, _i: usize, _v: Box<dyn Value>) {}

  fn arg_operands(&self) {}
}


