#![allow(dead_code)]

// This file defines a set of enums which allow processing of instrinsic
// functions. Values of these enum types are returned by get_intrinsic_id().

use crate::{adt::string_ref::StringRef, support::type_size::ElementCount};

use super::{global_value::IntrinsicID, module::Module, type_::Type, blits_context::BlitzContext};

// Abstraction for the arguments of the noalias intrinsics.
const NO_ALIAS_SCOPE_DECL_SCOPE_ARG: i32 = 0;

enum IndependentIntrinsics {
    NotIntrinsic = 0,
}

// Return the Blitz name for an intrinsic, such as "blitz.ppc.alitivec.lvx".
pub fn get_name(_id: IntrinsicID) -> String {
  String::from("")
}

// Return the Blitz name for an intrinsic, without encoded types for
// overloading, usch as "blitz.ssa.copy".
pub fn get_base_name(_id: u32) -> StringRef {
  StringRef::new()
}

pub fn get_name_no_unnamed_types() {}

// Return the function type for an intrinsic.
pub fn get_type(_c: &BlitzContext, _id: IntrinsicID, _types: Vec<&dyn Type>) {}

// Return true if the intrinsic can be overloaded.
pub fn is_overloaded(_id: u32) -> bool { false }

pub fn get_attributes() {}

// Create or insert an Blitz function declaration for an intrinsic,
// and return it.
pub fn get_declaration(_m: Module, _id: IntrinsicID, _types: Vec<&dyn Type>) {}

pub fn lookup_blitz_intrinsic_by_name() {}

#[derive(Debug, Clone)]
pub enum IITDescriptorKind {
  Void,
  Vararg,
  Mmx,
  Token,
  Metadata,
  Half,
  Bfloat,
  Float,
  Double,
  Quad,
  Integer,
  Vector,
  Pointer,
  Struct,
  Argument,
  ExtendArgument,
  TruncArgument,
  HalfVecArgument,
  SameVecWidthArgument,
  PtrToArgument,
  PtrToElt,
  VecOfAnyPtrsToElt,
  VecElementArgument,
  SubDivide2Argument,
  SubDivide4Argument,
  VecOfBitcastsToInt,
  Amx,
  PpcQuad,
  AnyPtrToElt,
  Aarch64Svcount
}

// This is a type descriptor which explains the type requirements of an intrinsic.
pub struct IITDescriptor {
  kind: IITDescriptorKind,
  //argument_info: usize
  vector_width: ElementCount
}

impl IITDescriptor {
  pub fn new() {}
  pub fn get_argumennt_number() {}
  pub fn get_argumennt_kind() {}
  pub fn get_overloaded_arg_numer() {}
  pub fn get_ref_arg_number() {}

  pub fn get(k: &IITDescriptorKind, _field: usize) -> Self {
    IITDescriptor {
      kind: k.clone(),
      vector_width: ElementCount::new(0, false)
    }
  }

  pub fn get_vector(width: usize, is_scalable: bool) -> Self {
    IITDescriptor {
      kind: IITDescriptorKind::Vector,
      vector_width: ElementCount::get(width, is_scalable)
    }
  }
}

// Return the IIT table descriptor for the specified intrinsic into an
// array of IITDescriptors.
fn get_intrinsic_info_table_entries() {}

enum IITInfo {
  Done,
  Vararg,
  Mmx,
  Amx,
  Token,
  Metadata,
  F16,
  BF16,
  F32,
  F64,
  F128,
  PpcF128,
  I1,
  I2,
  I4,
  Aarch64Svcount,
  I8,
  I16,
  I32,
  I64,
  I128,
  V1,
  V2,
  V3,
  V4,
  V8,
  V16,
  V32,
  V64,
  V128,
  V256,
  V512,
  V1024,
  ExternRef,
  FuncRef,
  Ptr,
  AnyPtr,
  Arg,
  ExtendArg,
  TruncArg,
  HalfVecArg,
  SameVecWidthArg,
  PtrToArg,
  PtrToElt,
  AnyPtrToElt,
  VecOfAnyPtrsToElt,
  EmptyStruct,
  Struct2,
  Struct3,
  Struct4,
  Struct5,
  Struct6,
  Struct7,
  Struct8,
  Struct9,
  SubDivide2Arg,
  SubDivide4Arg,
  VecElement,
  ScalableVec,
  BitcastsToInt,
}

fn decode_iit_type(next_elt: usize, infos: Vec<IITInfo>, _last_info: IITInfo,
  output_table: &mut Vec<IITDescriptor>)
{
  let info = infos.get(next_elt).unwrap();
  match info {
    IITInfo::Done => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Void, 0));
      return;
    },
    IITInfo::Vararg => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Vararg, 0));
      return;
    },
    IITInfo::Mmx => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Mmx, 0));
      return;
    },
    IITInfo::Amx => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Amx, 0));
      return;
    },
    IITInfo::Token => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Token, 0));
      return;
    },
    IITInfo::Metadata => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Metadata, 0));
      return;
    },
    IITInfo::F16 => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Half, 0));
      return;
    },
    IITInfo::BF16 => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Bfloat, 0));
      return;
    },
    IITInfo::F32 => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Float, 0));
      return;
    },
    IITInfo::F64 => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Double, 0));
      return;
    },
    IITInfo::F128 => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Quad, 0));
      return;
    },
    IITInfo::PpcF128 => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::PpcQuad, 0));
      return;
    },
    IITInfo::I1 => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Integer, 1));
      return;
    },
    IITInfo::I2 => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Integer, 2));
      return;
    },
    IITInfo::I4 => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Integer, 4));
      return;
    },
    IITInfo::Aarch64Svcount => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Aarch64Svcount, 0));
      return;
    },
    IITInfo::I8 => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Integer, 8));
      return;
    },
    IITInfo::I16 => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Integer, 16));
      return;
    },
    IITInfo::I32 => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Integer, 32));
      return;
    },
    IITInfo::I64 => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Integer, 64));
      return;
    },
    IITInfo::I128 => {
      output_table.push(IITDescriptor::get(&IITDescriptorKind::Integer, 128));
      return;
    },
    _ => unreachable!("Unhandled.")
  }
}

enum MatchIntrinsicTypesResult {
  Match,
  NoMatchRet,
  NoMatchArg
}

pub fn match_intrinsic_signature() {}
pub fn match_intrinsic_var_arg() {}
pub fn get_intrinsic_signature() {}
pub fn remangle_intrinsic_function() {}