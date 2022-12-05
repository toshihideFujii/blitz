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
  Pointer,
  Struct,
  Array,
  FixedVector,
  ScalableVector,
  TypedPointer
}


trait Type {
  fn get_subclass_data(&self) -> u32;

  fn set_subclass_data() {}

  fn get_type_id() -> TypeID;

  // Return true if this is 'void'.
  fn is_void_type() -> bool {
    false
  }

  // Return true if this is 'half', a 16-bit IEEE fp type.
  fn is_half_type() -> bool {
    false
  }

  fn is_bfloat_type() -> bool {
    false
  }

  fn is_16bit_fp_type() -> bool {
    false
  }

  fn is_float_type() -> bool {
    false
  }

  fn is_double_type() -> bool {
    false
  }

  fn is_x86_fp80_type() -> bool {
    false
  }

  fn is_fp128_type() -> bool {
    false
  }

  fn is_ppc_fp128_type() -> bool {
    false
  }

  fn is_floating_point_type() -> bool {
    false
  }

  fn is_x86_mmx_type() -> bool {
    false
  }

  fn is_x86_amx_type() -> bool {
    false
  }

  fn is_fp_or_fpvector_type() -> bool {
    false
  }

  fn is_label_type() -> bool {
    false
  }

  fn is_metadata_type() -> bool {
    false
  }

  fn is_token_type() -> bool {
    false
  }

  // True if this is an instance of IntegerType.
  fn is_integer_type() -> bool {
    false
  }

  fn is_int_or_int_vector_type() {}

  fn is_int_or_int_ptr_type() {}

  // True if this is an instance of FunctionType.
  fn is_function_type() -> bool {
    false
  }

  // True if this is an instance of StructType.
  fn is_struct_type() -> bool {
    false
  }

  // True if this is an instance of ArrayType.
  fn is_array_type() -> bool {
    false
  }

  // True if this is an instance of PointerType.
  fn is_pointer_type() -> bool {
    false
  }

  fn is_opaque_pointer_type() {}

  fn is_ptr_or_ptr_vector_type() {}

  // True if this is an instance of VectorType.
  fn is_vector_type() -> bool {
    false
  }

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

  fn get_contained_type() {}

  fn get_num_contained_type() {}

  fn get_integer_bit_width() {}

  fn get_function_param_type() {}

  fn get_function_num_params() {}

  fn is_function_var_arg() {}

  fn get_struct_name() {}

  fn get_struct_num_elements() {}

  fn get_struct_element_type() {}

  fn get_array_num_elements() {}

  fn get_array_element_type() {}

  fn get_pointer_element_type() {}

  fn get_non_opaque_pointer_element_type() {}

  fn get_with_new_type() {}

  fn get_with_new_bit_width() {}

  fn get_extended_type() {}

  fn get_pointer_address_space() {}
}

// Class to represent integer types.
struct IntegerType {
  sub_class_data_: u32
}

impl Type for IntegerType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data_
  }

  fn get_type_id() -> TypeID {
    TypeID::Integer
  }

  fn is_integer_type() -> bool {
    true
  }
}

impl IntegerType {
  pub fn new() {}

  pub fn get_extended_type() {}

  // Get the number of bits in this IntegerType
  pub fn get_bit_width(&self) -> u32 {
    self.sub_class_data_
  }

  pub fn get_bit_mask() {}

  // Return a u64 with just the most significant bit set
  // (the sign bit, if the value is treated as a signed number).
  pub fn get_sign_bit(&self) -> u64 {
    (1 as u64) << (self.get_bit_width() - 1)
  }

  pub fn get_mask() {}

  fn get() {}

  fn class_of() {}
}

// Class to represent function types
struct FunctionType {
  sub_class_data_: u32
}

impl Type for FunctionType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data_
  }

  fn get_type_id() -> TypeID {
    TypeID::Function
  }

  fn is_function_type() -> bool {
    true
  }
}

impl FunctionType {
  pub fn new() {}

  pub fn is_var_arg() {}

  pub fn get_return_type() {}

  pub fn param_begin() {}

  pub fn param_end() {}

  pub fn params() {}

  pub fn get_param_type() {}

  pub fn get_num_params() {}

  //fn classof(t: Type) -> bool {
  //  t.get_type_id() == TypeID::Function
  //}
}

struct FunctionCalee {}

// This is the contents of the SubClassData field.
enum SCDB {
  HasBody,
  Packed,
  IsLiteral,
  IsSized
}

struct StructType {
  sub_class_data_: u32
}

impl Type for StructType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data_
  }

  fn get_type_id() -> TypeID {
    TypeID::Struct
  }

  fn is_struct_type() -> bool {
    true
  }
}

impl StructType {
  pub fn new() {}

  fn get_type_bby_name() {}

  pub fn is_packed() {}

  pub fn is_literal() {}

  pub fn is_opaque() {}

  pub fn is_sized() {}

  pub fn contains_scalable_vector_type() {}

  pub fn has_name() {}

  pub fn get_name() {}

  pub fn set_name() {}

  pub fn set_body() {}

  pub fn element_begin() {}

  pub fn element_end() {}

  pub fn elements() {}

  pub fn is_layout_identical() {}

  pub fn get_num_elements() {}

  pub fn get_element_type() {}

  pub fn get_type_at_index() {}

  pub fn index_valid() {}
}

// Class to represent array types.
struct ArrayType {
  sub_class_data_: u32
}

impl Type for ArrayType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data_
  }

  fn get_type_id() -> TypeID {
    TypeID::Array
  }

  fn is_array_type() -> bool {
    true
  }
}

impl ArrayType {
  pub fn new() {}

  pub fn get_num_elements() {}

  pub fn get_element_type() {}
}

// Class to represent fixed width SIMD vectors
struct FixedVectorType {
  sub_class_data_: u32
}

impl Type for FixedVectorType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data_
  }

  fn get_type_id() -> TypeID {
    TypeID::FixedVector
  }

  fn is_vector_type() -> bool {
    true
  }
}

impl FixedVectorType {
  pub fn new() {}

  pub fn get() {}

  pub fn get_integer() {}

  pub fn get_extended_element_vector_type() {}

  pub fn get_truncated_element_vector_type() {}

  pub fn get_subdivided_vector_type() {}

  pub fn get_half_elements_vector_type() {}

  pub fn get_double_elements_vector_type() {}

  pub fn class_of() {}

  pub fn get_num_elements() {}
}

struct ScalableVectorType {
  sub_class_data_: u32
}

impl Type for ScalableVectorType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data_
  }

  fn get_type_id() -> TypeID {
    TypeID::ScalableVector
  }

  fn is_vector_type() -> bool {
    true
  }
}

// Class to represent scalable SIMD vectors
impl ScalableVectorType {
  pub fn new() {}

  pub fn get() {}

  pub fn get_integer() {}

  pub fn get_extended_element_vector_type() {}

  pub fn get_truncated_element_vector_type() {}

  pub fn get_subdivided_vector_type() {}

  pub fn get_half_elements_vector_type() {}

  pub fn get_double_elements_vector_type() {}

  pub fn class_of() {}

  pub fn get_num_elements() {}
}

struct PointerType {
  sub_class_data_: u32
}

impl Type for PointerType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data_
  }

  fn get_type_id() -> TypeID {
    TypeID::Pointer
  }

  fn is_pointer_type() -> bool {
    true
  }
}

impl PointerType {
  pub fn new() {}

  pub fn get() {}

  pub fn get_unqual() {}

  pub fn get_with_same_pointee_type() {}

  pub fn is_opaque() {}

  pub fn is_valid_element_type() {}

  pub fn is_loadable_or_storable_type() {}

  pub fn get_address_spave() {}

  pub fn is_opaque_or_pointee_type_matches() {}

  pub fn has_same_element_type_as() {}

  pub fn class_of() {}

  pub fn get_extended_type() {}
}