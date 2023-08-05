#![allow(dead_code)]
#![allow(unused_variables)]

// This file contains the declaration of the Type class.

use std::any::Any;
use std::fmt::Debug;
use crate::adt::ap_int::APInt;
use crate::support::type_size::TypeSize;
use super::blits_context::BlitzContext;
//use super::value::Value;

// Definitions of all of the base types for the Type system.
// Based on this value, you can cast to a class defined below.
#[derive(Debug, Clone, PartialEq)]
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

pub trait Type: Debug /*+ Clone + Sized*/ /*+ std::cmp::PartialEq*/ {
  fn get_subclass_data(&self) -> u32;
  fn set_subclass_data(&mut self, val: u32) {}
  fn dump(&self) {}
  fn get_context(&self) -> &BlitzContext;
  fn get_type_id(&self) -> TypeID;

  // Return true if this is 'void'.
  fn is_void_type(&self) -> bool {
    false
  }

  // Return true if this is 'half', a 16-bit IEEE fp type.
  fn is_half_type(&self) -> bool { false }
  fn is_bfloat_type(&self) -> bool { false }
  fn is_16bit_fp_type(&self) -> bool { false }
  fn is_float_type(&self) -> bool { false }
  fn is_double_type(&self) -> bool { false }
  fn is_x86_fp80_type(&self) -> bool { false }
  fn is_fp128_type(&self) -> bool { false }
  fn is_ppc_fp128_type(&self) -> bool { false }
  fn is_floating_point_type(&self) -> bool { false }
  fn is_x86_mmx_type(&self) -> bool { false }
  fn is_x86_amx_type(&self) -> bool { false }

  // Return true if this is a fp type or a vector of fp.
  fn is_fp_or_fpvector_type(&self) -> bool { false }
  fn is_label_type(&self) -> bool { false }
  fn is_metadata_type(&self) -> bool { false }
  fn is_token_type(&self) -> bool { false }

  // True if this is an instance of IntegerType.
  fn is_integer_type(&self) -> bool {
    false
  }

  fn is_int_or_int_vector_type(&self) {}
  fn is_int_or_int_ptr_type(&self) {}

  // True if this is an instance of FunctionType.
  fn is_function_type(&self) -> bool { false }

  // True if this is an instance of StructType.
  fn is_struct_type(&self) -> bool { false }

  // True if this is an instance of ArrayType.
  fn is_array_type(&self) -> bool { false }

  // True if this is an instance of PointerType.
  fn is_pointer_type(&self) -> bool { false }

  fn is_opaque_pointer_type(&self) {}
  fn is_ptr_or_ptr_vector_type(&self) {}

  // True if this is an instance of VectorType.
  fn is_vector_type(&self) -> bool { false }
  fn is_empty_type(&self) -> bool { false }

  // Return true if the type is "first class", meaning it is a valid type
  // for a value.
  fn is_first_class_type(&self) -> bool {
    self.get_type_id() != TypeID::Function && self.get_type_id() != TypeID::Void
  }

  fn is_single_value_type(&self) -> bool { false }
  fn is_aggregate_type(&self) -> bool { false }
  fn is_sized(&self) -> bool { false }

  fn get_primitive_size_in_bits(&self) -> Option<TypeSize> { None }
  fn get_scalar_size_in_bits(&self) -> u32 { 0 }

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
  fn get_pointer_address_space(&self) -> u32 { 0 }
  fn get_primitive_type(&self) {}

  // For StructType.
  fn contains_scalable_vector_type(&self) -> bool { false }

  fn as_any(&self) -> &dyn Any;
}


//impl Clone for dyn Type {
//  fn clone(&self) -> Self {
//    *self
//  }
//}


/*
impl std::cmp::PartialEq for Type {
  fn eq(&self, other: &Self) -> bool {
    self.get_type_id() == other.get_type_id()
  }
}
*/

pub fn get_void_type(c: &mut BlitzContext) -> VoidType {
  //c.get_impl().void_type.clone()
  c.get_impl_2().void_type.clone()
}

pub fn get_label_type(c: &BlitzContext) -> LabelType {
  c.get_impl().label_type.clone()
}

pub fn get_half_type() {}
pub fn get_b_float_type() {}
pub fn get_float_type() {}
pub fn get_double_type() {}
pub fn get_metadata_type() {}
pub fn get_x86_fp80_type() {}
pub fn get_fp128_type() {}
pub fn get_ppc_fp128_type() {}
pub fn get_x86_mmx_type() {}
pub fn get_x86_amx_type() {}
pub fn get_token_type() {}

pub fn get_int_1_type(c: &mut BlitzContext) -> IntegerType {
  //c.get_impl().get_int_1_type().clone()
  IntegerType::new(c.clone(), 1)
}

pub fn get_int_8_type(c: &BlitzContext) -> IntegerType {
  //c.get_impl().get_int_8_type().clone()
  IntegerType::new(c.clone(), 1)
}

pub fn get_int_16_type(c: &BlitzContext) -> IntegerType {
  //c.get_impl().get_int_16_type().clone()
  IntegerType::new(c.clone(), 1)
}

pub fn get_int_32_type(c: &BlitzContext) -> IntegerType {
  //c.get_impl().get_int_32_type().clone()
  IntegerType::new(c.clone(), 1)
}

pub fn get_int_64_type(c: &BlitzContext) -> IntegerType {
  //c.get_impl().get_int_64_type().clone()
  IntegerType::new(c.clone(), 1)
}

pub fn get_int_128_type(c: &BlitzContext) -> IntegerType {
  //c.get_impl().get_int_128_type().clone()
  IntegerType::new(c.clone(), 1)
}

pub fn get_int_n_type(c: &mut BlitzContext, n: u32) -> IntegerType {
  IntegerType::get(c, n)
}

// This enum is just used to hold constants we need for IntegerType.
enum IntConstants {
  MinIntBits = 1,
  MaxIntBits = 1<<23
}

#[derive(Debug, Clone, PartialEq)]
pub struct VoidType {
  context: BlitzContext,
  id: TypeID
}

impl VoidType {
  pub fn new(c: BlitzContext) -> Self {
    VoidType { context: c, id: TypeID::Void }
  }
}

impl Type for VoidType {
  fn get_type_id(&self) -> TypeID {
    TypeID::Void
  }

  fn get_context(&self) -> &BlitzContext {
    &self.context
  }

  fn get_subclass_data(&self) -> u32 {
    0
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabelType {
  context: BlitzContext,
  id: TypeID,
}

impl LabelType {
  pub fn new(c: BlitzContext) -> Self {
    LabelType { context: c, id: TypeID::Label }
  }
}

impl Type for LabelType {
  fn get_type_id(&self) -> TypeID {
    TypeID::Label
  }

  fn get_context(&self) -> &BlitzContext {
    &self.context
  }

  fn get_subclass_data(&self) -> u32 {
    0
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}


// Class to represent integer types.
// Note that this class is also used to represent the built-in
// integer types: Int1, Int8, Int16, Int32, Int64.
#[derive(Debug, Clone, PartialEq)]
pub struct IntegerType {
  context: BlitzContext,
  id: TypeID,
  sub_class_data: u32
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
  pub fn get(c: &mut BlitzContext, num_bits: u32) -> IntegerType {
    debug_assert!(num_bits >= IntConstants::MinIntBits as u32, "Bitwidth too small.");
    debug_assert!(num_bits <= IntConstants::MaxIntBits as u32, "Bitwidth too large.");

    match num_bits {
      1 => return get_int_1_type(c),
      8 => return get_int_8_type(c),
      16 => return get_int_16_type(c),
      32 => return get_int_32_type(c),
      64 => return get_int_64_type(c),
      128 => return get_int_128_type(c),
      _ => println!("num_bits: {}", num_bits),
    };

    let entry = c.get_impl_2().integer_types.find(&num_bits);
    if entry.is_none() {
      let new_entry = IntegerType::new(c.clone(), num_bits);
      let ret_v = new_entry.clone();
      c.get_impl_2().integer_types.insert(num_bits, new_entry);
      ret_v
    } else {
      return entry.unwrap().clone();
    }
  }

  // Returns type twice as wide the input type.
  //pub fn get_extended_type(&self) -> IntegerType {
    //get_int_n_type(self.get_context().clone(), 2 * self.get_scalar_size_in_bits())
  //}

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
  fn class_of(t: &dyn Type) -> bool {
    t.get_type_id() == TypeID::Integer
  }

  pub fn get_context_2(&mut self) -> &mut BlitzContext {
    &mut self.context
  }
}

impl Type for IntegerType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data
  }

  fn set_subclass_data(&mut self, val: u32) {
    self.sub_class_data = val;
    debug_assert!(self.get_subclass_data() == val, "Subclass data too large for field.");
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

  fn as_any(&self) -> &dyn Any {
    self
  }
}

// Class to represent function types
#[derive(Debug)]
pub struct FunctionType {
  sub_class_data: u32,
  context: BlitzContext,
  contained_types: Vec<Box<dyn Type>>
}

impl Type for FunctionType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data
  }

  fn set_subclass_data(&mut self, val: u32) {
    self.sub_class_data = val;
    debug_assert!(self.get_subclass_data() == val, "Subclass data too large for field.");
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

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl FunctionType {
  /*
  pub fn new(result: &dyn Type, params: Vec<Box<dyn Type>>, is_var_args: bool) -> Self {
    let mut data: u32 = 0;
    if is_var_args { data = 1; }
    let fn_type = FunctionType { sub_class_data: data,
      context: result.get_context().clone(), contained_types: params };

    debug_assert!(fn_type.is_valid_return_type(result), "Invalid return type for functioin.");
    let params_len = fn_type.contained_types.len();
    for i in 0.. params_len {
      debug_assert!(fn_type.is_valid_argument_type(&*fn_type.contained_types[i]),
        "Not a valid type for function argument.");
    }
    fn_type
  }
  */

  pub fn get() {}

  // Return true if the specified type is valid as a return type.
  pub fn is_valid_return_type(&self, ret_type: &dyn Type) -> bool {
    !ret_type.is_function_type() && !ret_type.is_label_type() &&
    !ret_type.is_metadata_type()
  }

  // Return true if the specified type is valid as an argument type.
  pub fn is_valid_argument_type(&self, arg_type: &dyn Type) -> bool {
    arg_type.is_first_class_type()
  }

  pub fn is_var_arg(&self) -> bool {
    self.get_subclass_data() != 0
  }

  pub fn get_return_type(&self) /*-> Box<dyn Type>*/ {
    //self.contained_types[0]
    //let mut cp = Vec::new();
    //cp.clone_from_slice(&self.contained_types);
  }

  pub fn param_begin() {}
  pub fn param_end() {}
  pub fn params(&self) -> &Vec<Box<dyn Type>> {
    &self.contained_types
  }

  // Parameter type accessors.
  pub fn get_param_type(&self, i: usize) -> &Box<dyn Type> {
    &self.contained_types[i]
  }

  // Return the number of fixed parameters this function type requires.
  // This does not consider varargs.
  pub fn get_num_params(&self) -> usize {
    self.contained_types.len()
  }

  pub fn classof(t: &dyn Type) -> bool {
    t.get_type_id() == TypeID::Function
  }
}

// A handy container for a FunctionType + Callee-pointer pair, which can
// be passed araound as a single entity.
// This assists in replacing the use of PointerType::getElementType() to
// access the Function's type, since that's slated for removal as part of
// the [opaque pointer types] project.
struct FunctionCalee {
  fn_type: FunctionType,
  //callee: Value
}

impl FunctionCalee {
  pub fn new(fn_type: FunctionType /*callee: Value*/) -> Self {
    FunctionCalee { fn_type: fn_type /*callee: callee*/ }
  }

  pub fn get_function_type(&self) -> &FunctionType {
    &self.fn_type
  }

  pub fn get_callee(&self) /*-> &Value*/ {
    //&self.callee
  }
}

// This is the contents of the SubClassData field.
enum SCDB {
  HasBody,
  Packed,
  IsLiteral,
  IsSized
}

// Class to represent struct types.
// There are two different kinds of struct types: Literal structs and
// Identified structs.
// Literal struct types (e.g. {i32, i32}) are uniqued structurally, and
// must always have a body when created.
// You can get one of these by using one of the StructType::get() forms.
#[derive(Debug)]
struct StructType {
  sub_class_data: u32,
  context: BlitzContext,
  contained_types: Vec<Box<dyn Type>>
}

impl Type for StructType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data
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

  // Returns true if this struct contains a scalable vector.
  fn contains_scalable_vector_type(&self) -> bool {
    for element in &self.contained_types {
      if element.get_type_id() == TypeID::ScalableVector {
        return true;
      }
      if element.get_type_id() == TypeID::Struct {
        if element.contains_scalable_vector_type() {
          return true;
        }
      }
    }
    false
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl StructType {
  pub fn new() {}
  pub fn create() {}
  pub fn get() {}
  pub fn get_type_by_name() {}

  pub fn is_packed(&self) -> bool {
    SCDB::Packed as u32 & self.get_subclass_data() != 0
  }

  // Return true if this type is uniqued by structural equivalence,
  // false if it is a struct definition.
  pub fn is_literal(&self) -> bool {
    SCDB::IsLiteral as u32 & self.get_subclass_data() != 0
  }

  // Return true if this is a type with an identity that has no body
  // specified yet, These prints an 'opaque' in .ll files.
  pub fn is_opaque(&self) -> bool {
    SCDB::HasBody as u32 & self.get_subclass_data() == 0
  }

  pub fn is_sized() {}
  pub fn has_name() {}
  pub fn get_name() {}
  pub fn set_name() {}
  pub fn set_body() {}

  pub fn is_valid_element_type(elem_type: &dyn Type) -> bool {
    !elem_type.is_void_type() && !elem_type.is_label_type() && !elem_type.is_metadata_type() &&
    !elem_type.is_function_type() && !elem_type.is_token_type()
  }

  pub fn element_begin() {}
  pub fn element_end() {}

  pub fn elements(&self) -> &Vec<Box<dyn Type>> {
    &self.contained_types
  }

  pub fn is_layout_identical() {}

  // Random access to the elements.
  pub fn get_num_elements(&self) -> usize {
    self.contained_types.len()
  }

  pub fn get_element_type(&self, n: usize) -> &Box<dyn Type> {
    debug_assert!(n < self.contained_types.len(), "Element number out of range.");
    &self.contained_types[n]
  }

  // Given an index value into the type, return the type of the element.
  pub fn get_type_at_index(&self, n: usize) -> &Box<dyn Type> {
    self.get_element_type(n)
  }

  pub fn index_valid(&self, index: usize) -> bool {
    index < self.get_num_elements()
  }

  pub fn classof(t: &dyn Type) -> bool {
    t.get_type_id() == TypeID::Struct
  }
}

impl PartialEq for StructType {
  fn eq(&self, other: &Self) -> bool {
    if self.sub_class_data != other.sub_class_data {
      return false;
    }
    if self.is_packed() != other.is_packed() {
      return false;
    }
    if self.contained_types.len() != other.contained_types.len() {
      return false;
    }
    //for t1 in self.contained_types {
      //for t2 in other.contained_types {
        //if *t1 != *t2 {
          //return false;
        //}
      //}
    //}
    true
  }
}

// Class to represent array types.
#[derive(Debug)]
struct ArrayType {
  sub_class_data: u32,
  context: BlitzContext,
  contained_type: Box<dyn Type>,
  num_elements: usize
}

impl Type for ArrayType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data
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

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl ArrayType {
  pub fn new() {}

  pub fn get_num_elements(&self) -> usize {
    self.num_elements
  }

  pub fn get_element_type(&self) -> &Box<dyn Type> {
    &self.contained_type
  }

  pub fn get() {}

  // Return true if the specified type is valid as a element type.
  pub fn is_valid_element_type(elem_type: &dyn Type) -> bool {
    !elem_type.is_void_type() && !elem_type.is_label_type() &&
    !elem_type.is_metadata_type() && !elem_type.is_function_type() &&
    !elem_type.is_token_type() && !elem_type.is_x86_amx_type() &&
    elem_type.get_type_id() != TypeID::ScalableVector
  }

  pub fn classof(t: &dyn Type) -> bool {
    t.get_type_id() == TypeID::Array
  }
}

// Class to represent fixed width SIMD vectors
#[derive(Debug)]
pub struct FixedVectorType {
  sub_class_data: u32,
  context: BlitzContext,
  contained_type: Box<dyn Type>,
  element_quantity: usize
}

impl Type for FixedVectorType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data
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

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl FixedVectorType {
  pub fn new(v_type: Box<dyn Type>) -> Self {
    FixedVectorType {
      sub_class_data: 0,
      context: v_type.get_context().clone(),
      contained_type: v_type,
      element_quantity: 0
    }
  }

  pub fn get_element_type(&self) -> &Box<dyn Type> {
    &self.contained_type
  }

  pub fn get() {}

  pub fn get_integer() {}

  pub fn get_extended_element_vector_type() {}

  pub fn get_truncated_element_vector_type() {}

  pub fn get_subdivided_vector_type() {}

  pub fn get_half_elements_vector_type() {}

  pub fn get_double_elements_vector_type() {}

  pub fn classof(t: &dyn Type) -> bool {
    t.get_type_id() == TypeID::FixedVector
  }

  pub fn get_num_elements(&self) -> usize {
    self.element_quantity
  }
}

#[derive(Debug)]
struct ScalableVectorType {
  sub_class_data: u32,
  context: BlitzContext,
  contained_type: Box<dyn Type>,
  element_quantity: usize
}

impl Type for ScalableVectorType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data
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

  fn as_any(&self) -> &dyn Any {
    self
  }
}

// Class to represent scalable SIMD vectors
impl ScalableVectorType {
  pub fn new() {}

  pub fn get_element_type(&self) -> &Box<dyn Type> {
    &self.contained_type
  }

  pub fn get() {}

  pub fn get_integer() {}

  pub fn get_extended_element_vector_type() {}

  pub fn get_truncated_element_vector_type() {}

  pub fn get_subdivided_vector_type() {}

  pub fn get_half_elements_vector_type() {}

  pub fn get_double_elements_vector_type() {}

  pub fn classof(t: &dyn Type) -> bool {
    t.get_type_id() == TypeID::ScalableVector
  }

  // Get the minimum number of elements in this vector.
  // The actual number of elements in the vector is an integer multiple of
  // this value.
  pub fn get_num_elements(&self) -> usize {
    self.element_quantity
  }
}

// Class to represent pointers.
#[derive(Debug)]
struct PointerType {
  sub_class_data: u32,
  context: BlitzContext,
  pointee_type: Option<Box<dyn Type>>
}

impl Type for PointerType {
  fn get_subclass_data(&self) -> u32 {
    self.sub_class_data
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

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl PointerType {
  pub fn new() {}

  pub fn get() {}

  pub fn get_unqual() {}

  pub fn get_with_same_pointee_type() {}

  pub fn is_opaque(&self) -> bool {
    if self.pointee_type.is_none() {
      return true;
    } else {
      return false;
    }
  }

  // Return true if the specified type is valid as a element type.
  pub fn is_valid_element_type(elem_type: &dyn Type) -> bool {
    !elem_type.is_void_type() && !elem_type.is_label_type() &&
    !elem_type.is_metadata_type() && !elem_type.is_token_type() &&
    !elem_type.is_x86_amx_type()
  }

  // Return true if we can load or store from a pointer to this type.
  pub fn is_loadable_or_storable_type(elem_type: &dyn Type) -> bool {
    PointerType::is_valid_element_type(elem_type) && !elem_type.is_function_type()
  }

  // Return the address space of the pointer type.
  pub fn get_address_space(&self) -> u32 {
    self.sub_class_data
  }

  pub fn is_opaque_or_pointee_type_matches() {}

  pub fn has_same_element_type_as() {}

  pub fn classof(t: &dyn Type) -> bool {
    t.get_type_id() == TypeID::Pointer
  }
}

struct TargetExtType {}
impl TargetExtType {
  pub fn new() {}
}