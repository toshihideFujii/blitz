#![allow(dead_code)]

// This file defines the DataLayout-independent constant folding
// interface. When possible, the DataLayout-aware constant folding
// interface in analysis/constnt_folding.rs should be preferred.

use crate::{ir::{
  instruction::InstructionBase,
  constants::{ConstantExpr, ConstantInt},
  type_::Type,constant, constant::Constant, instruction::OpCode, blits_context::blits_context
}, adt::{ap_float::APFloat, ap_int::APInt, floating_point_mode::RoundingMode}};


pub fn constant_fold_cast_instruction(opcode: &OpCode, v: &Box<dyn Constant>,
  dst_type: &Box<dyn Type>) -> Option<Box<dyn Constant>>
{
  match opcode {
    OpCode::UIToFP => {
      let ci = v.as_any().downcast_ref::<ConstantInt>();
      if ci.is_some() {
        let apint = ci.unwrap().get_value();
        let size = dst_type.get_primitive_size_in_bits().get_fixed_value() as u32;
        let mut apflt = APFloat::new_from_apint(dst_type.get_flt_semantics(),
          &APInt::get_zero(size));
        apflt.convert_from_apint(apint, false,
          RoundingMode::NearestTiesToEven);
      }
    }
    _ => println!("Failed to cast constant expression.")
  };
  None
}

pub fn constant_fold_select_instruction() {}
pub fn constant_fold_extract_element_instruction() {}
pub fn constant_fold_insert_element_instruction() {}
pub fn constant_fold_shuffle_vector_instruction() {}
pub fn constant_fold_extract_value_instruction() {}
pub fn constant_fold_insert_value_instruction() {}
pub fn constant_fold_unary_instruction() {}

pub fn constant_fold_binary_instruction(opcode: &OpCode,
  c1: &Box<dyn Constant>, c2: &Box<dyn Constant>) -> Option<Box<dyn Constant>>
{
    debug_assert!(InstructionBase::is_binary_op_static(opcode),
      "Non-binary instruction detected.");

    let identity =
      ConstantExpr::get_bin_op_identity(opcode, &Box::new(c1.get_type().clone()),
        false, false);

    if identity.is_some() {
      //if c1.as_ref() == identity.as_ref().unwrap().as_ref() {}
    }

    // Handle simplifications when the RHS is a constant int.
    let ci2 = c2.as_any().downcast_ref::<ConstantInt>();
    if ci2.is_some() {
      match opcode {
        OpCode::Add => {
          if ci2.unwrap().is_zero() { // x + 0 == x
            let ci1 =
              c1.as_any().downcast_ref::<ConstantInt>().unwrap().clone();
            return Some(Box::new(ci1));
          }
        }
        OpCode::Sub => {
          if ci2.unwrap().is_zero() { // x - 0 == x
            let ci1 =
              c1.as_any().downcast_ref::<ConstantInt>().unwrap().clone();
            return Some(Box::new(ci1));
          }
        }
        OpCode::Mul => {
          if ci2.unwrap().is_zero() { // x * 0 == 0
            return Some(Box::new(ci2.unwrap().clone()));
          }
          if ci2.unwrap().is_one() { // x * 1 == x
            let ci1 =
              c1.as_any().downcast_ref::<ConstantInt>().unwrap().clone();
            return Some(Box::new(ci1));
          }
        }
        OpCode::UDiv => {
          if ci2.unwrap().is_one() { // x / 1 == x
            let ci1 =
              c1.as_any().downcast_ref::<ConstantInt>().unwrap().clone();
            return Some(Box::new(ci1));
          }
          if ci2.unwrap().is_zero() { // x / 0 == poison
            // TODO
          }
        }
        OpCode::SDiv => {
          if ci2.unwrap().is_one() { // x / 1 == x
            let ci1 =
              c1.as_any().downcast_ref::<ConstantInt>().unwrap().clone();
            return Some(Box::new(ci1));
          }
          if ci2.unwrap().is_zero() { // x / 0 == poison
            // TODO
          }
        }
        OpCode::URem => {
          if ci2.unwrap().is_one() { // x % 1 == 0
            return Some(constant::get_null_value(&Box::new(&ci2.unwrap().get_type())));
          }
          if ci2.unwrap().is_zero() { // x % 0 == poison
            // TODO
          }
        }
        OpCode::SRem => {
          if ci2.unwrap().is_one() { // x % 1 == 0
            return Some(constant::get_null_value(&Box::new(&ci2.unwrap().get_type())));
          }
          if ci2.unwrap().is_zero() { // x % 0 == poison
            // TODO
          }
        }
        OpCode::And => {
          // TODO
        }
        OpCode::Or => {
          if ci2.unwrap().is_zero() { // x | 0 == x
            let ci1 =
              c1.as_any().downcast_ref::<ConstantInt>().unwrap().clone();
            return Some(Box::new(ci1));
          }
          if ci2.unwrap().is_minus_one() { // x | -1 == -1
            return Some(Box::new(ci2.unwrap().clone()));
          }
        }
        OpCode::Xor => {
          if ci2.unwrap().is_zero() { // x ^ 0 == x
            let ci1 =
              c1.as_any().downcast_ref::<ConstantInt>().unwrap().clone();
            return Some(Box::new(ci1));
          }
          // TODO
        }
        OpCode::AShr => {
          // TODO
        }
        _ => println!("Unsupported opcode.")
      };
    } else if c1.as_any().downcast_ref::<ConstantInt>().is_some() {
      // If c1 is a ConstantInt and c2 is not, swap the operands.
      if InstructionBase::is_commutative_static(opcode) {
        return ConstantExpr::get(opcode.clone(), c2, c1,
          0, None);
      }
    }

    let ci1 = c1.as_any().downcast_ref::<ConstantInt>();
    if ci1.is_some() && ci2.is_some() {
      let c1v = ci1.unwrap().get_value().clone();
      let c2v = ci2.unwrap().get_value().clone();
      match opcode {
        OpCode::Add => return Some(Box::new(ConstantInt::get_from_apint(
          blits_context(), c1v + c2v))),
        OpCode::Sub => return Some(Box::new(ConstantInt::get_from_apint(
          blits_context(), c1v - c2v))),
        OpCode::Mul => return Some(Box::new(ConstantInt::get_from_apint(
          blits_context(), c1v * c2v))),
        OpCode::UDiv => {
          debug_assert!(!ci2.unwrap().is_zero(), "Div by zero handled above.");
          return Some(Box::new(ConstantInt::get_from_apint(
            blits_context(), c1v / c2v)))
        }
        OpCode::SDiv => {
          debug_assert!(!ci2.unwrap().is_zero(), "Div by zero handled above.");
          if c2v.is_all_ones() && c1v.is_min_signed_value() {
            // TODO
          }
          return Some(Box::new(ConstantInt::get_from_apint(
            blits_context(), c1v / c2v)))
        }
        OpCode::URem => {
          debug_assert!(!ci2.unwrap().is_zero(), "Div by zero handled above.");
          return Some(Box::new(ConstantInt::get_from_apint(
            blits_context(), c1v % c2v)))
        }
        OpCode::SRem => {
          debug_assert!(!ci2.unwrap().is_zero(), "Div by zero handled above.");
          if c2v.is_all_ones() && c1v.is_min_signed_value() {
            // TODO
          }
          return Some(Box::new(ConstantInt::get_from_apint(
            blits_context(), c1v % c2v)))
        }
        OpCode::And => return Some(Box::new(ConstantInt::get_from_apint(
          blits_context(), c1v & c2v))),
        OpCode::Or => return Some(Box::new(ConstantInt::get_from_apint(
          blits_context(), c1v | c2v))),
        OpCode::Xor => return Some(Box::new(ConstantInt::get_from_apint(
          blits_context(), c1v ^ c2v))),
        OpCode::Shl => {

        }
        OpCode::LShr => {

        }
        OpCode::AShr => {

        }
        _ => println!("Unsupported opcode."),
      };
    }
    
    None
}

pub fn constant_fold_compare_instruction() {}
pub fn constant_fold_get_element() {}