#![allow(dead_code)]

// This file defines layoout properties related to datatype
// size/offset/alignment information.

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

// A parsed version of the target data layout string in and
// methods for querying it.
struct DataLayout {}

impl DataLayout {
  pub fn new() {}
  pub fn find_alignment_lower_bound() {}
  pub fn get_pointer_align_elem() {}
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
  pub fn get_string_representation() {}
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
}