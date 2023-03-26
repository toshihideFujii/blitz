#![allow(dead_code)]

// This file contains the declaration of the Type class.

use crate::adt::ap_int::APInt;
use crate::support::type_size::TypeSize;
use super::blits_context::BlitzContext;

// Definitions of all of the base types for the Type system.
// Based on this value, you can cast to a class defined below.
#[derive(Debug, PartialEq)]
pub enum TypeID {
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
  TypedPointer,
  TargetExt
}


pub trait Type {
  fn get_subclass_data(&self) -> u32;

  fn set_subclass_data(&self) {}

  fn dump(&self) {}

  fn get_context(&self) -> &BlitzContext;

  fn get_type_id(&self) -> TypeID;

  // Return true if this is 'void'.
  fn is_void_type(&self) -> bool {
    false
  }

  // Return true if this is 'half', a 16-bit IEEE fp type.
  fn is_half_type(&self) -> bool {
    false
  }

  fn is_bfloat_type(&self) -> bool {
    false
  }

  fn is_16bit_fp_type(&self) -> bool {
    false
  }

  fn is_float_type(&self) -> bool {
    false
  }

  fn is_double_type(&self) -> bool {
    false
  }

  fn is_x86_fp80_type(&self) -> bool {
    false
  }

  fn is_fp128_type(&self) -> bool {
    false
  }

  fn is_ppc_fp128_type(&self) -> bool {
    false
  }

  fn is_floating_point_type(&self) -> bool {
    false
  }

  fn is_x86_mmx_type(&self) -> bool {
    false
  }

  fn is_x86_amx_type(&self) -> bool {
    false
  }

  fn is_fp_or_fpvector_type(&self) -> bool {
    false
  }

  fn is_label_type(&self) -> bool {
    false
  }

  fn is_metadata_type(&self) -> bool {
    false
  }

  fn is_token_type(&self) -> bool {
    false
  }

  // True if this is an instance of IntegerType.
  fn is_integer_type(&self) -> bool {
    false
  }

  fn is_int_or_int_vector_type(&self) {}

  fn is_int_or_int_ptr_type(&self) {}

  // True if this is an instance of FunctionType.
  fn is_function_type(&self) -> bool {
    false
  }

  // True if this is an instance of StructType.
  fn is_struct_type(&self) -> bool {
    false
  }

  // True if this is an instance of ArrayType.
  fn is_array_type(&self) -> bool {
    false
  }

  // True if this is an instance of PointerType.
  fn is_pointer_type(&self) -> bool {
    false
  }

  fn is_opaque_pointer_type(&self) {}

  fn is_ptr_or_ptr_vector_type(&self) {}

  // True if this is an instance of VectorType.
  fn is_vector_type(&self) -> bool {
    false
  }

  fn is_empty_type(&self) {}
  fn is_first_calss_type(&self) {}
  fn is_single_value_type(&self) {}
  fn is_aggregate_type(&self) {}
  fn is_sized(&self) {}

  fn get_primitive_size_in_bits(&self) -> Option<TypeSize> {
    None
  }

  fn get_scalar_size_in_bits(&self) -> u32 {
    0
  }

  fn get_fp_mantissa_width(&self) {}
  fn is_ieee(&self) {}

  fn get_scalar_type(&self) {}

  fn get_contained_type(&self) {}
  fn get_num_contained_type(&self) {}
  fn get_integer_bit_width(&self) {}
  fn get_function_param_type(&self) {}
  fn get_function_num_params(&self) {}
  fn is_function_var_arg(&self) {}
  fn get_struct_name(&self) {}
  fn get_struct_num_elements(&self) {}
  fn get_struct_element_type(&self) {}
  fn get_array_num_elements(&self) {}
  fn get_array_element_type(&self) {}
  fn get_pointer_element_type(&self) {}
  fn get_non_opaque_pointer_element_type(&self) {}
  fn get_with_new_type(&self) {}
  fn get_with_new_bit_width(&self) {}
  fn get_extended_type(&self) {}
  fn get_pointer_address_space(&self) {}
  fn get_primitive_type() {}
  fn get_void_type() {}
  fn get_label_type() {}
  fn get_half_type() {}
  fn get_b_float_type() {}
  fn get_float_type() {}
  fn get_double_type() {}
  fn get_metadata_type() {}
  fn get_x86_fp80_type() {}
  fn get_fp128_type() {}
  fn get_ppc_fp128_type() {}
  fn get_x86_mmx_type() {}
  fn get_x86_amx_type() {}
  fn get_token_type() {}

}

pub fn get_int_n_type(c: BlitzContext, n: u32) -> IntegerType {
  IntegerType::get(c, n)
}

fn get_int_1_type() {}
fn get_int_8_type() {}
fn get_int_16_type() {}
fn get_int_32_type() {}
fn get_int_64_type() {}
fn get_int_128_type() {}


// This enum is just used to hold constants we need for IntegerType.
enum IntConstants {
  MinIntBits = 1,
  MaxIntBits = 1<<23
}

// Class to represent integer types.
// Note that this class is also used to represent the built-in
// integer types: Int1, Int8, Int16, Int32, Int64.
pub struct IntegerType {
  context: BlitzContext,
  id: TypeID,
  sub_class_data: u32
}

impl Type for IntegerType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data
  }

  fn get_context(&self) -> &BlitzContext {
    &self.context
  }

  fn get_type_id(&self) -> TypeID {
    TypeID::Integer
  }

  fn is_integer_type(&self) -> bool {
    true
  }

  fn get_primitive_size_in_bits(&self) -> Option<TypeSize> {
    Some(TypeSize::fixed(self.get_bit_width() as u64))
  }

  fn get_scalar_size_in_bits(&self) -> u32 {
    self.get_primitive_size_in_bits().unwrap().get_fixed_value() as u32
  }
}

impl IntegerType {
  pub fn new(c: BlitzContext, num_bits: u32) -> Self {
    IntegerType { context: c, id: TypeID::Integer, sub_class_data: num_bits }
  }

  // This static method is the primary way of constructing an IntegerType.
  // If an IntegerType with the same num_bits value was previously instantiated,
  // that instance will be returned.
  // Otherwise a new one will be created.
  // Only one instance with a given num_bits value is ever created.
  // Get or create an IntegerType instance.
  fn get(c: BlitzContext, num_bits: u32) -> IntegerType {
    debug_assert!(num_bits >= IntConstants::MinIntBits as u32, "bitwidth too small");
    debug_assert!(num_bits <= IntConstants::MaxIntBits as u32, "bitwidth too large");

    // TODO
    let entry = IntegerType::new(c, num_bits);
    entry
  }

  // Returns type twice as wide the input type.
  pub fn get_extended_type(&self) -> IntegerType {
    get_int_n_type(self.get_context().clone(), 2 * self.get_scalar_size_in_bits())
  }

  // Get the number of bits in this IntegerType
  pub fn get_bit_width(&self) -> u32 {
    self.sub_class_data
  }

  // Return a bitmask with ones set for all of the bits that can be set
  // by an unsigned version of this type.
  pub fn get_bit_mask(&self) -> u64 {
    !(0 as u64) >> (64 - self.get_bit_width())
  }

  // Return a u64 with just the most significant bit set
  // (the sign bit, if the value is treated as a signed number).
  pub fn get_sign_bit(&self) -> u64 {
    (1 as u64) << (self.get_bit_width() - 1)
  }

  // For wxample, this is 0xFF for an 8 bit integer, 0xFFFF for i16, etc.
  // Returns a bit mask with ones set for all the bits of this type.
  // Get a bit mask for this type.
  pub fn get_mask(&self) -> APInt {
    APInt::get_all_ones(self.get_bit_width())
  }

  // Methods for support type inquiry through isa, cast, and dyn_cast.
  fn class_of(t: &impl Type) -> bool {
    t.get_type_id() == TypeID::Integer
  }
}

// Class to represent function types
struct FunctionType {
  sub_class_data: u32,
  context: BlitzContext
}

impl Type for FunctionType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data
  }

  fn get_context(&self) -> &BlitzContext {
    &self.context
  }

  fn get_type_id(&self) -> TypeID {
    TypeID::Function
  }

  fn is_function_type(&self) -> bool {
    true
  }
}

impl FunctionType {
  pub fn new() {}

  pub fn is_var_arg(&self) -> bool {
    self.get_subclass_data() != 0
  }

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
  sub_class_data_: u32,
  context: BlitzContext
}

impl Type for StructType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data_
  }

  fn get_context(&self) -> &BlitzContext {
    &self.context
  }

  fn get_type_id(&self) -> TypeID {
    TypeID::Struct
  }

  fn is_struct_type(&self) -> bool {
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
  sub_class_data_: u32,
  context: BlitzContext
}

impl Type for ArrayType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data_
  }

  fn get_context(&self) -> &BlitzContext {
    &self.context
  }

  fn get_type_id(&self) -> TypeID {
    TypeID::Array
  }

  fn is_array_type(&self) -> bool {
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
  sub_class_data_: u32,
  context: BlitzContext
}

impl Type for FixedVectorType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data_
  }

  fn get_context(&self) -> &BlitzContext {
    &self.context
  }

  fn get_type_id(&self) -> TypeID {
    TypeID::FixedVector
  }

  fn is_vector_type(&self) -> bool {
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
  sub_class_data_: u32,
  context: BlitzContext
}

impl Type for ScalableVectorType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data_
  }

  fn get_context(&self) -> &BlitzContext {
    &self.context
  }

  fn get_type_id(&self) -> TypeID {
    TypeID::ScalableVector
  }

  fn is_vector_type(&self) -> bool {
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
  sub_class_data_: u32,
  context: BlitzContext
}

impl Type for PointerType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data_
  }

  fn get_type_id(&self) -> TypeID {
    TypeID::Pointer
  }

  fn get_context(&self) -> &BlitzContext {
    &self.context
  }

  fn is_pointer_type(&self) -> bool {
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