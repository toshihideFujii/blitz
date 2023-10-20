#![allow(dead_code)]

// This file defines layoout properties related to datatype
// size/offset/alignment information.

use crate::{
  ir::type_::Type,
  support::alignment::MaybeAlign/* , type_size::align_to*/
};

enum AlignType {
  IntegerAlign,
  VectorAlign,
  FloatAlign,
  AggregateAlign
}

// Layout alignment element.
struct LayoutAlignElem {}

// Layout pointer alignment element.
struct PointerAlignElem {}

pub enum ManglingMode {
  None,
  Elf,
  MachO,
  WinCoff,
  WinCoffX86,
  Goff,
  Mios,
  Xcoff
}

// A parsed version of the target data layout string in and methods
// for querying it.
// The target data layout string is specified *by the target* - a frontend
// generating Blitz IR is required to generate the right target data 
// for the target being codegen'd to.
#[derive(Debug, PartialEq)]
pub struct DataLayout {
  big_endian: bool,
  alloca_addr_space: u32,
  program_addr_space: u32,
  default_global_addr_space: u32,
  stack_nature_align: MaybeAlign,
  function_ptr_align: MaybeAlign
}

impl DataLayout {
  pub fn new() {}
  pub fn find_alignment_lower_bound() {}
  pub fn get_pointer_align_elem() {}

  // Attempts to set alignment of the given type.
  pub fn set_alignment() {}
  pub fn set_pointer_alignment_in_bits() {}
  pub fn get_integer_alignment() {}
  pub fn get_alignment() {}
  pub fn parse_specifier() {}
  pub fn clear() {}
  pub fn reset() {}
  pub fn parse() {}
  pub fn is_little_endian() {}
  pub fn is_big_endian() {}

  pub fn get_string_representation(&self) -> String {
    String::new()
  }
  
  pub fn is_default() {}
  pub fn is_legal_integer() {}
  pub fn is_illegal_integer() {}
  pub fn exceed_natural_stack_alignment() {}
  pub fn get_stack_alignment() {}
  pub fn get_alloca_addr_space() {}
  pub fn get_function_ptr_align() {}
  pub fn get_program_address_space() {}
  pub fn get_default_globals_address_space() {}
  pub fn do_not_mangle_leading_question_mark() {}
  pub fn has_linker_private_global_prefix() {}
  pub fn get_linker_private_global_prefix() {}
  pub fn get_global_prefix() {}
  pub fn get_private_global_prefix() {}
  pub fn get_mangling_component() {}
  pub fn fits_in_legal_integer() {}
  pub fn get_pointer_abi_alignment() {}
  pub fn get_pointer_pref_alignment() {}
  pub fn get_pointer_size() {}
  pub fn get_max_index_size() {}
  pub fn get_index_size() {}
  pub fn get_non_integral_address_spaces() {}
  pub fn is_non_integral_address_space() {}
  pub fn is_non_integral_pointer_type() {}
  pub fn get_pointer_size_in_bits() {}
  pub fn get_max_index_size_in_bits() {}
  pub fn get_index_size_in_bits() {}
  pub fn get_pointer_type_size_in_bits() {}
  pub fn get_index_type_size_in_bits() {}
  pub fn get_pointer_type_size() {}
  pub fn get_type_size_in_bits() {}

  // Returns the maximum number of bytes that may be overwritten by
  // storing the specified type.
  // If t is a scalable vector type, the scalable property will be set
  // and the runtime size will be a positive integer multiple of the base size.
  // For example, returns 5 for i36 and 10 for x86_fp80.
  pub fn get_type_store_size(_t: Box<dyn Type>) {}

  pub fn get_type_store_size_in_bits() {}
  pub fn type_size_equal_store_size() {}

  // Returns the offset in bytes between successive objects of the
  // specified type, including alignment padding.
  // If t is a scalable vector type, the scalable property will be set
  // and the runtime size will be a positive integer multiple of the base size.
  // This is the amount that alloca reserves for this type.
  // For example, returns 12 or 16 for x86_fp80, depending on alignment.
  pub fn get_type_alloc_size(_t: Box<dyn Type>) {
    //align_to(size, align)
  }

  pub fn get_type_alloc_size_in_bits() {}
  pub fn get_abi_type_alignment() {}
  pub fn get_abi_type_align() {}
  pub fn get_value_or_abi_type_alignment() {}
  pub fn get_abi_integral_type_alignment() {}
  pub fn get_pref_type_alignment() {}
  pub fn get_pref_type_align() {}
  pub fn get_int_ptr_type() {}
  pub fn get_smallest_legal_int_type() {}
  pub fn get_largest_legal_int_type() {}
  pub fn get_largest_legal_int_type_size_in_bits() {}
  pub fn get_index_type() {}
  pub fn get_indexed_offset_in_type() {}
  pub fn get_gep_indices_for_offset() {}
  pub fn get_struct_layout() {}
  pub fn get_preffered_align() {}
}

// Used to lazily calculate structure layout information for a target
// machine, based on the DataLayout structure.
struct StructLayout {
}

impl StructLayout {
  pub fn new() {}
  pub fn get_size_in_bits() {}
  pub fn get_alignment() {}
  pub fn has_padding() {}
  pub fn get_element_containing_offset() {}
  pub fn get_member_offsets() {}
  pub fn get_element_offset() {}
  pub fn get_element_offset_in_bits() {}
}