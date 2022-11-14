#![allow(dead_code)]

/*
This file contains the declaration of the Type class.
*/

enum TypeID {
  // Primitive types
  Half,
  BFloat,
  Double,
  X86Fp80,
  Fp128,
  PpcFp128,
  Void,
  Label,
  Metadata,
  X86Mmx,
  X86Amx,
  Token,
  // Derived types
  Integer,
  Function,
  POinter,
  Struct,
  Array,
  FixedVector,
  ScalableVector,
  TypedPointer
}

pub trait Type {
  fn get_type_id() {}

  // Return true if this is 'void'.
  fn is_void_type() -> bool {
    false
  }

  // Return true if this is 'half', a 16-bit IEEE fp type.
  fn is_half_type() -> bool {
    false
  }

  fn is_bfloat_type() {}

  fn is_16bit_fp_type() {}

  fn is_float_type() {}

  fn is_double_type() {}

  fn is_x86_fp80_type() {}

  fn is_fp128_type() {}

  fn is_ppc_fp128_type() {}

  fn is_floating_point_type() {}

  fn is_x86_mmx_type() {}

  fn is_x86_amx_type() {}

  fn is_fp_or_fpvector_type() {}

  fn is_label_type() {}

  fn is_metadata_type() {}

  fn is_token_type() {}

  fn is_integer_type() {}

  fn is_int_or_int_vector_type() {}

  fn is_int_or_int_ptr_type() {}

  fn is_function_type() {}

  fn is_struct_type() {}

  fn is_array_type() {}

  fn is_pointer_type() {}

  fn is_opaque_pointer_type() {}

  fn is_ptr_or_ptr_vector_type() {}

  fn is_vector_type() {}

  fn is_empty_type() {}

  fn is_first_calss_type() {}

  fn is_single_value_type() {}

  fn is_aggregate_type() {}

  fn is_sized() {}

  fn get_primitive_size_in_bits() {}

  fn get_scalar_size_in_bits() {}

  fn get_fp_mantissa_width() {}

  fn is_ieee() {}

  fn get_scalar_type() {}
}