#![allow(dead_code)]

// This file contains the declarations for the subclasses of
// Constant, which represent the different flavors of constant
// values that live in Bllitz.
// Note that Constants are immutable (once created they never change)
// and are fully shared by structural equivalence.
// This means that two structurally equivalent constants will
// always have the same address.
// Constants are created on demand as needed and never deleted:
// thus clients don't have to worry about the lifetime of the objects.

use crate::{adt::{ap_int::APInt, ap_float::APFloat}, ir::constants_context::ConstantExprKeyType};

use super::{
  blits_context::BlitzContext,
  //type_::{self, /*FixedVectorType*/},
  type_::{IntegerType, Type},
  value::{Value, ValueType},
  instruction::{Instruction, OtherOps, BinaryOps},
  constant,
  constant::Constant,
  constant_fold,
  use_::Use, operator::OverflowBinOpWrap
};


// This is the shared class of boolean and integer constants.
// This class represents both boolean and integral constants.
// Class for constant integers.
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantInt {
  v_type: IntegerType,
  v_id: ValueType,
  val: APInt,
  use_list: Vec<u64>,
  num_user_operands: u32
}

impl ConstantInt {
  pub fn new(v_type: IntegerType, val: APInt) -> Self {
    ConstantInt {
      v_type: v_type,
      v_id: ValueType::ConstantIntVal,
      val: val,
      use_list: Vec::new(),
      num_user_operands: 0
    }
  }

  pub fn get_true(c: &BlitzContext) -> Option<ConstantInt> {
    c.get_impl().as_ref().unwrap().get_true_value()
  }

  pub fn get_false(c: &BlitzContext) -> Option<ConstantInt> {
    c.get_impl().as_ref().unwrap().get_false_value()
  }

  pub fn get_bool(c: &BlitzContext, v: bool) -> Option<ConstantInt> {
    if v {
      ConstantInt::get_true(c)
    } else {
      ConstantInt::get_false(c)
    }
  }

  // Return a ConstantInt with the specified value and an implied Type.
  // The type is the integer type that corresponds to the bit width of the value.
  pub fn get_from_apint(c: &BlitzContext, v: APInt) -> ConstantInt {
    let slot = c.get_impl().as_ref().unwrap().int_constants.find(&v);
    if slot.is_none() {
      let i_type = IntegerType::get(c, v.get_bit_width());
      let c_int = ConstantInt::new(i_type, v.clone());
      //c.get_impl_3().as_deref_mut().unwrap().int_constants.insert(v.clone(), c_int.clone());
      return c_int;
    }
    slot.unwrap().clone()
  }

  // Return a ConstantInt with the specified integer value for the specified type.
  pub fn get(t: &IntegerType, v: i64, is_signed: bool) -> ConstantInt {
    let val = APInt::new(t.get_bit_width(), v, is_signed);
    ConstantInt::get_from_apint(t.get_context(), val)
  }

  // Return a ConstantInt with the specified value for the specified type.
  // The value v will be canonicalized to an unsigned APInt.
  // Accessing it with either get_sext_value() or get_zext_value() will yield
  // a correctly sized and signed value for the type t.
  // Get a ConstantInt for a specific signed value.
  pub fn get_signed(t: &IntegerType, v: i64) -> ConstantInt {
    ConstantInt::get(t, v, true)
  }

  // Return the content as an APInt value reference.
  // This allows clients to obtain a full-precision copy of the value.
  pub fn get_value(&self) -> &APInt {
    &self.val
  }

  // Return the bitwidth of this constant.
  pub fn get_bit_width(&self) -> u32 {
    self.val.get_bit_width()
  }

  // Return the constant as a 64-bit unsigned integer value after it
  // has been zero extended as appropriate for the type of this constant.
  pub fn get_zext_value(&self) -> u64 {
    self.val.get_z_ext_value()
  }

  // Return the constant as a 64-bit integer value after it has been
  // sign extended as appropriate for the type of this constant.
  pub fn get_sext_value(&self) -> i64 {
    self.val.get_s_ext_value()
  }

  pub fn get_maybe_align_value() {}
  pub fn get_align_value() {}

  // A helper method that can be used to deetrmine if the constant contained
  // within is equal to a constant.
  pub fn equals_int(&self, v: u64) -> bool {
    self.val == v
  }

  // Specialize the get_type() method to always return an IntegerType,
  // which reduces the amount of casting needed in parts of the compiler.
  pub fn get_type(&self) -> IntegerType {
    self.v_type.clone()
  }

  pub fn is_value_valid_for_type() {}

  pub fn is_negative(&self) -> bool {
    self.val.is_negative()
  }

  // This is just a convenience method to make client code smaller for 
  // a common code.
  pub fn is_zero(&self) -> bool {
    self.val.is_zero()
  }

  // This is just a convenience method to make client code smaller for 
  // a common code.
  pub fn is_one(&self) -> bool {
    self.val.is_one()
  }

  // This function will return true if every bit in this constant is set
  // to one.
  pub fn is_minus_one(&self) -> bool {
    self.val.is_all_ones()
  }

  // This function will return true if this constant represents the largest
  // value that may be represented by the constants's type.
  pub fn is_max_value(&self, is_signed: bool) -> bool {
    if is_signed {
      self.val.is_max_signed_value()
    } else {
      self.val.is_max_value()
    }
  }

  // This function will return true if this constant represents the smallest
  // value that may be represented by the constants's type.
  pub fn is_min_value(&self, is_signed: bool) -> bool {
    if is_signed {
      self.val.is_min_signed_value()
    } else {
      self.val.is_min_value()
    }
  }

  // This function will return true if this constant represents a value with
  // active bits bigger than 64 bits or a value greater than the given u64 value.
  pub fn uge(&self, num: u64) -> bool {
    self.val.uge(num)
  }

  // If the value is smaller than the specified limit, return it, otherwise
  // return the limit value.
  pub fn get_limited_value(&self, limit: u64) -> u64 {
    self.val.get_limited_value(limit)
  }

  // Methods to support type inquiry through isa, cast, and dyn_cast.
  pub fn class_of(v: &dyn Value) -> bool {
    v.get_value_id() == ValueType::ConstantIntVal
  }
}

impl Value for ConstantInt {
  fn get_type(&self) -> &dyn Type {
    &self.v_type
  }

  fn get_context(&self) -> &BlitzContext {
    self.v_type.get_context()
  }

  fn get_value_id(&self) -> ValueType {
    self.v_id.clone()
  }

  fn use_empty(&self) -> bool {
    self.use_list.is_empty()    
  }

  fn has_one_use(&self) -> bool {
    self.use_list.len() == 1
  }

  fn has_n_uses(&self, n: usize) -> bool {
    self.use_list.len() == n
  }

  fn has_n_uses_or_more(&self, n: usize) -> bool {
    self.use_list.len() >= n
  }

  // ??
  fn has_one_user(&self) -> bool {
    if self.use_empty() {
      return false;
    }
    if self.has_one_use() {
      return true;
    }
    false
  }

  fn get_num_uses(&self) -> usize {
    self.use_list.len()
  }

  fn add_use(&mut self, _u: Use) {
    //self.use_list.push(u);
  }
}

impl Constant for ConstantInt {
  fn is_null_value(&self) -> bool {
    self.is_zero()
  }

  fn is_one_value(&self) -> bool {
    self.is_one()
  }

  fn is_all_ones_value(&self) -> bool {
    self.is_minus_one()
  }

  fn is_not_min_signed_value(&self) -> bool {
    !self.is_min_value(true)
  }

  //fn get_null_value(&self, _t: Box<dyn Type>) -> Box<dyn Constant> {
    //Box::new(ConstantInt::get(&self.v_type, 0, false))
  //}
}

// Floating point values [float, double].
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantFP {
  v_type: IntegerType,
  v_id: ValueType,
  val: APFloat
}

impl ConstantFP {
  pub fn new() {}
  pub fn get_zero_value_for_negation(&self) {}
  pub fn get() {}
  pub fn get_nan() {}
  pub fn get_q_nan() {}
  pub fn get_s_nan() {}

  pub fn get_zero(_t: &Box<dyn Type>, _negative: bool) {}

  pub fn get_negative_zero() {}
  pub fn get_infinity() {}
  pub fn is_value_valid_for_type() {}

  pub fn get_value_apf(&self) -> APFloat {
    self.val.clone()
  }

  pub fn get_value(&self) -> APFloat {
    self.val.clone()
  }

  // Return true if the value is positive or negative zero.
  pub fn is_zero(&self) -> bool {
    self.val.is_zero()
  }

  // Return true if the sign bit is set.
  pub fn is_negative(&self) -> bool {
    self.val.is_negative()
  }

  // Return true if the value is infinity.
  pub fn is_infinity(&self) -> bool {
    self.val.is_infinity()
  }

  // Return true if the value is a NaN.
  pub fn is_nan(&self) -> bool {
    self.val.is_nan()
  }

  pub fn is_exactly_value(&self, _v: f64) -> bool { false }

  // Methods to support type inquiry through isa, cast, and dyn_cast.
  pub fn class_of(v: &dyn Value) -> bool {
    v.get_value_id() == ValueType::ConstantFPVal
  }
}

impl Value for ConstantFP {
  fn get_type(&self) -> &dyn Type {
    &self.v_type
  }

  fn get_context(&self) -> &BlitzContext {
    self.v_type.get_context()
  }

  fn get_value_id(&self) -> ValueType {
    self.v_id.clone()
  }
}

impl Constant for ConstantFP {
  fn is_null_value(&self) -> bool {
    self.is_exactly_value(0.0)
  }

  fn is_one_value(&self) -> bool {
    self.get_value_apf().bitcast_to_apint().is_one()
  }

  fn is_all_ones_value(&self) -> bool {
    self.get_value_apf().bitcast_to_apint().is_all_ones()
  }

  fn is_negative_zero_value(&self) -> bool {
    self.is_zero() && self.is_negative()
  }

  fn is_zero_value(&self) -> bool {
    self.is_zero()
  }

  fn is_not_min_signed_value(&self) -> bool {
    !self.get_value_apf().bitcast_to_apint().is_min_signed_value()
  }

  fn is_finite_non_zero_fp(&self) -> bool {
    self.get_value_apf().is_finite_non_zero()
  }

  fn is_normal_fp(&self) -> bool {
    self.get_value_apf().is_normal()
  }

  fn has_exact_inverse_fp(&self) -> bool {
    self.get_value_apf().get_exact_inverse(None)
  }

  fn is_nan(&self) -> bool {
    self.is_nan()
  }

  // TODO
  //fn get_null_value(&self, _t: Box<dyn Type>) -> Box<dyn Constant> {
    //Box::new(ConstantInt::get(&self.v_type, 0, false))
  //}
}

// All zero aggregate value.
pub struct ConstantAggregateZero {
  v_type: Box<dyn Type>,
  v_id: ValueType,
}

impl ConstantAggregateZero {
  pub fn new() {}
  pub fn get() {}
  pub fn get_sequential_element() {}
  pub fn get_struct_element() {}
  pub fn get_element_value() {}
  pub fn get_element_count() {}

  // Methods to support type inquiry through isa, cast, and dyn_cast.
  pub fn class_of(v: &dyn Value) -> bool {
    v.get_value_id() == ValueType::ConstantAggregateZeroVal
  }
}

impl Value for ConstantAggregateZero {
  fn get_type(&self) -> &dyn Type {
    self.v_type.as_ref()
  }

  fn get_context(&self) -> &BlitzContext {
    self.v_type.get_context()
  }

  fn get_value_id(&self) -> ValueType {
    self.v_id.clone()
  }
}

struct ConstantArray {}

struct ConstantStruct {}

struct ConstantVector {
  //v_type: FixedVectorType,
  v_id: ValueType
}

impl ConstantVector {
  pub fn new() -> Self {
    ConstantVector { 
      //v_type: 
      v_id: ValueType::ConstantVectorVal }
  }

  pub fn get_operand(&self, _n: u32) -> Option<&dyn Constant> { None }
  pub fn set_operand(&self, _n: u32, _c: &dyn Constant) {}
  pub fn op(&self) {}
  pub fn get_num_operands(&self) -> u32 { 0 }

  pub fn get() {}
  pub fn get_splat() {}
  pub fn get_splat_value() {}

  pub fn class_of(v: &dyn Value) -> bool {
    v.get_value_id() == ValueType::ConstantVectorVal
  }
}
/*
impl Value for ConstantVector {
  fn get_type(&self) -> &dyn Type {
    self.v_type.as_ref()
  }

  fn get_context(&self) -> &BlitzContext {
    self.v_type.get_context()
  }

  fn get_value_id(&self) -> ValueType {
    self.v_id.clone()
  }
}
*/

pub struct ConstantPointerNull {}

struct ConstantDataSequential {}

struct ConstantDataArray {}

struct ConstantDataVector {}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantTokenNone {}
impl ConstantTokenNone {
  pub fn new() -> Self {
    ConstantTokenNone {  }
  }
}

pub struct ConstantTargetNone {}

struct BlockAddress {}

struct DS0LocalEquivalent {}

struct NoCFIValue {}

// This class uses the standard instruction opcodes to define the various
// constant expressions. The opcode field for the ConstantExpr class id
// maintainedin the Value::sub_class_data field.
//#[derive(Debug, Clone, PartialEq)]
pub struct ConstantExpr {
  v_type: Box<dyn Type>,
  v_id: ValueType
}

impl ConstantExpr {
  pub fn new(v_type: Box<dyn Type>, _op_code: u32, _ops: Option<Use>, _num_ops: u32) -> Self {
    ConstantExpr { v_type: v_type, v_id: ValueType::ConstantExprVal }
  }

  pub fn get_align_of() {}
  pub fn get_size_of() {}
  pub fn get_neg() {}
  pub fn get_not() {}

  pub fn get_add(c1: Box<dyn Constant>, c2: Box<dyn Constant>,
    has_nuw: bool, has_nsw: bool) -> Option<Box<dyn Constant>>
  {
    let mut f1 = 0;
    if has_nuw { f1 = OverflowBinOpWrap::NoUnsignedWrap as u32; }

    let mut f2 = 0;
    if has_nsw { f2 = OverflowBinOpWrap::NoSignedWrap as u32; }

    let flags = f1 | f2;
    ConstantExpr::get(BinaryOps::Add as u32, c1, c2, flags)
  }

  pub fn get_sub() {}
  pub fn get_mul() {}
  pub fn get_and() {}
  pub fn get_or() {}
  pub fn get_xor() {}
  pub fn get_shl() {}
  pub fn get_lshr() {}
  pub fn get_ashr() {}
  pub fn get_trunc() {}
  pub fn get_sext() {}
  pub fn get_zext() {}
  pub fn get_fp_trunc() {}
  pub fn get_fp_extend() {}
  pub fn get_ui_to_fp() {}
  pub fn get_si_to_fp() {}
  pub fn get_fp_to_ui() {}
  pub fn get_fp_to_si() {}
  pub fn get_ptr_to_int() {}
  pub fn get_int_to_ptr() {}
  pub fn get_bit_cast() {}
  pub fn get_addr_space_cast() {}
  pub fn get_nsw_neg() {}
  pub fn get_nuw_neg() {}
  pub fn get_nsw_add() {}
  pub fn get_nuw_add() {}
  pub fn get_nsw_sub() {}
  pub fn get_nuw_sub() {}
  pub fn get_nsw_mul() {}
  pub fn get_nuw_mul() {}
  pub fn get_nsw_shl() {}
  pub fn get_nuw_shl() {}
  pub fn get_exact_ashr() {}
  pub fn get_exact_lshr() {}
  pub fn get_exact_log_base_2() {}

  // Return the identity constant for a binary opcode.
  // The identity constant c is defined as x op c = x and c op x = x for
  // every x when the binary operation is commutative.
  pub fn get_bin_op_identity(opcode: u32, t: &Box<dyn Type>,
    allow_rhs_constant: bool, _nsz: bool) -> Option<Box<dyn Constant>>
  {
    debug_assert!(Instruction::is_binary_op_static(opcode), "Only binops allowed.");
    if Instruction::is_commutative_static(opcode) {
      if opcode == 13 || // Add: x + 0 = x
        opcode == 29 || // Or: x | 0 = x
        opcode == 30 // Xor: x ^ 0 = x
      {
        return Some(constant::get_null_value(t));
      } else if opcode == 17 { // Mul: x * 1 = x
        return Some(Box::new(ConstantInt::get(&t.as_any().
          downcast_ref::<IntegerType>().unwrap(), 1, false)));
      } else if opcode == 28 { // And: x & -1 = x
        return Some(constant::get_all_ones_value(t));
      } else if opcode == 14 { // FAdd: x + -0.0 = x
        // TODO
      } else if opcode == 18 { // FMul: x * 1.0 = x
        // TODO
      } else {
        //panic!("Every commutative binop has an identity constant.");
      }
    }

    // Non-commutative opcodes: allow_rhs_constant must be set.
    if !allow_rhs_constant { return None; }

    if opcode == 15 || // Sub: x - 0 = x
      opcode == 25 || // Shl: x << 0 = x
      opcode == 26 || // LShr: x >>u 0 = x
      opcode == 27 || // AShr: x >> 0 = x
      opcode == 16 // FSub: x - 0.0 = x
    {
      return Some(constant::get_null_value(t));
    } else if
      opcode == 20 || // SDiv: x / 1 = x
      opcode == 19 // UDiv: x /u 1 = x
    {
      return Some(Box::new(ConstantInt::get(&t.as_any().
        downcast_ref::<IntegerType>().unwrap(), 1, false)));
    } else if opcode == 21 { // FDiv: x / 1.0 = x
      // TODO
    } else {
      return None;
    }

    None
  }

  pub fn get_bin_op_absorber() {}
  pub fn get_cast() {}
  pub fn get_zext_or_bit_cast() {}
  pub fn get_sext_or_bit_cast() {}
  pub fn get_trunc_or_bit_cast() {}
  pub fn get_sext_or_trunc() {}
  pub fn get_pointer_cast() {}
  pub fn get_pointer_bit_cast_or_addr_scace_cast() {}
  pub fn get_integer_cast() {}
  pub fn get_fp_cast() {}

  pub fn is_cast(&self) -> bool {
    Instruction::is_cast_static(self.get_opcode())
  }

  // Return true if this is a compare constant expression.
  pub fn is_compare(&self) -> bool {
    self.get_opcode() == OtherOps::ICmp as u32 ||
    self.get_opcode() == OtherOps::FCmp as u32
  }
  
  // Return a binary or shift operator constant expression,
  // folding if possible.
  pub fn get(opcode: u32, c1: Box<dyn Constant>, c2: Box<dyn Constant>,
    flags: u32) -> Option<Box<dyn Constant>>
  {
    debug_assert!(Instruction::is_binary_op_static(opcode),
      "Invalid opcode in binary constant expression.");

    debug_assert!(ConstantExpr::is_supported_bin_op(opcode),
      "Binop not supported as constant expression.");

    //debug_assert!(c1.get_type() == c2.get_type(),
    //  "Operand types in binary constant expression should match.");

    let fc =
      constant_fold::constant_fold_binary_instruction(opcode, &c1, &c2);
    if fc.is_some() { return fc; }
    
    // TODO: only_if_reduced_type == c1.get_type()
    let arg_vec = vec![c1, c2];
    let _key =ConstantExprKeyType::new(opcode, arg_vec,
      0, flags);

    //let pimpl = c1.get_context().p_impl;

    None
  }

  pub fn get_compare() {}
  pub fn get_icmp() {}
  pub fn get_fcmp() {}
  pub fn get_get_element_ptr() {}
  pub fn get_inbounds_get_element_ptr() {}
  pub fn get_extract_element() {}
  pub fn get_insert_element() {}
  pub fn get_shuffle_vector() {}

  // Return the opcode at the root of this constant expression.
  pub fn get_opcode(&self) -> u32 {
    0 //self.sub_class_data
  }

  pub fn get_predicate() {}
  pub fn get_shuffle_mask() {}
  pub fn get_shuffle_mask_for_bitcode() {}
  pub fn get_opcode_name() {}
  pub fn get_with_operands() {}
  pub fn get_as_instruction() {}
  pub fn is_desirable_bin_op() {}

  // Whether creating a constant expression for this binary operator
  // supported.
  pub fn is_supported_bin_op(opcode: u32) -> bool {
    match opcode {
      // true
      13 => return true, // Add
      15 => return true, // Sub
      17 => return true, // Mul
      25 => return true, // Shl
      26 => return true, // LShr
      27 => return true, // AShr
      28 => return true, // And
      29 => return true, // Or
      30 => return true, // Xor
      // false
      19 => return false, // UDiv
      20 => return false, // SDiv
      22 => return false, // URem
      23 => return false, // SRem
      14 => return false, // FAdd
      16 => return false, // FSub
      18 => return false, // FMul
      21 => return false, // FDiv
      24 => return false, // FRem
      _ => panic!("Argument must be binop code."),
    };
  }

  pub fn is_supported_get_element_ptr() {}
  pub fn class_of() {}
}

struct UndefValue {}

struct PoisonValue {}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::ir::blits_context::BlitzContext;
  use crate::ir::type_::IntegerType;


  #[test]
  fn test_integer_i1() {
    let c = BlitzContext::new();
    let int1 = IntegerType::get(&c, 1);
    let _one = ConstantInt::get(&int1, 1, true);
    let _zero = ConstantInt::get(&int1, 0, false);
    let neg_one = ConstantInt::get(&int1, -1, true);
    assert_eq!(neg_one, ConstantInt::get_signed(&int1, -1))
  }
}