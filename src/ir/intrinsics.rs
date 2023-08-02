#![allow(dead_code)]

// This file defines a set of enums which allow processing of
// instrinsic functions.

use crate::adt::string_ref::StringRef;

// Abstraction for the arguments of the noalias intrinsics.
const NO_ALIAS_SCOPE_DECL_SCOPE_ARG: i32 = 0;

enum IndependentIntrinsics {
    NotIntrinsic = 0,
}

// Return the Blitz name for an intrinsic, such as "blitz.ppc.alitivec.lvx".
pub fn get_name(_id: u32) -> StringRef {
  StringRef::new()
}

// Return the Blitz name for an intrinsic, without encoded types for
// overloading, usch as "blitz.ssa.copy".
pub fn get_base_name(_id: u32) -> StringRef {
  StringRef::new()
}

pub fn get_name_no_unnamed_types() {}

pub fn get_type() {}

// Return true if the intrinsic can be overloaded.
pub fn is_overloaded(_id: u32) -> bool { false }

pub fn lookup_blitz_intrinsic_by_name() {}

struct IITDescriptor {}
impl IITDescriptor {
  pub fn new() {}
  pub fn get_argumennt_number() {}
  pub fn get_argumennt_kind() {}
  pub fn get_overloaded_arg_numer() {}
  pub fn get_ref_arg_number() {}
  pub fn get() {}
  pub fn get_vector() {}
}

pub fn get_intrinsic_info_table_entries() {}

enum MatchIntrinsicTypesResult {
  Match,
  NoMatchRet,
  NoMatchArg
}

pub fn match_intrinsic_signature() {}
pub fn match_intrinsic_var_arg() {}
pub fn get_intrinsic_signature() {}
pub fn remangle_intrinsic_function() {}