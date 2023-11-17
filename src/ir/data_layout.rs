#![allow(dead_code)]

// This file defines layout properties related to datatype size/offset/
// alignment information. It uses lazy annotations to cache information
// about how structure types are laid out and used.

use crate::{
  ir::{type_::{Type, TypeID, IntegerType, FixedVectorType, ScalableVectorType, ArrayType},
  blits_context::blits_context},
  support::{alignment::{MaybeAlign, Align, log2}, math_extras::{divide_ceil, self},
  type_size::{TypeSize, align_to}}, /* , type_size::align_to*/
};

use super::type_::{PointerType, get_int_n_type};

// Enum used to categorize the alignment types atored by LayoutAlignElem.
pub enum AlignType {
  IntegerAlign,
  VectorAlign,
  FloatAlign,
  AggregateAlign
}

// Layout alignment element.
// Stores the alignment data associated with a given type bit width.
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutAlignElem {
  type_bit_width: usize,
  abi_align: Align,
  pref_align: Align
}

impl LayoutAlignElem {
  pub fn get(abi_align: Align, pref_align: Align, bit_width: usize) -> Self {
    LayoutAlignElem { type_bit_width: bit_width, abi_align: abi_align,
      pref_align: pref_align }
  }
}

// Layout pointer alignment element.
// Stores the alignment data associated with a given pointer and address space.
#[derive(Debug, Clone, PartialEq)]
pub struct PointerAlignElem {
  abi_align: Align,
  pref_align: Align,
  type_bit_width: usize,
  address_space: usize,
  index_bit_width: usize
}

impl PointerAlignElem {
  pub fn get_in_bits(address_space: usize, abi_align: Align,
    pref_align: Align, type_bit_width: usize, index_bit_width: usize) -> Self
  {
    PointerAlignElem { abi_align: abi_align, pref_align: pref_align,
      type_bit_width: type_bit_width, address_space: address_space,
      index_bit_width: index_bit_width }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionPtrAlignType {
  Independent,
  MultipleOfFunctionAlign
}

#[derive(Debug, PartialEq)]
pub enum ManglingMode {
  None,
  ELF,
  MachO,
  WinCOFF,
  WinCOFFX86,
  GOFF,
  Mips,
  XCOFF
}

// A parsed version of the target data layout string in and methods
// for querying it.
// The target data layout string is specified *by the target* - a frontend
// generating Blitz IR is required to generate the right target data 
// for the target being codegen'd to.
#[derive(Debug, PartialEq)]
pub struct DataLayout {
  big_endian: bool,
  alloca_addr_space: usize,
  program_addr_space: usize,
  default_globals_addr_space: usize,
  stack_nature_align: MaybeAlign,
  function_ptr_align: MaybeAlign,
  the_function_ptr_align_type: FunctionPtrAlignType,
  mangling_mode: ManglingMode,
  legal_int_widths: Vec<usize>,
  int_alignments: Vec<LayoutAlignElem>,
  float_alignments: Vec<LayoutAlignElem>,
  vector_alignments: Vec<LayoutAlignElem>,
  struct_alignment: LayoutAlignElem,
  string_representation: String,
  pointers: Vec<PointerAlignElem>,
  non_integral_address_spaces: Vec<usize>
}

impl DataLayout {
  pub fn new(desc: String) -> Self {
    let mut dl = DataLayout {
      big_endian: false, alloca_addr_space: 0, program_addr_space: 0,
      default_globals_addr_space: 0, stack_nature_align: MaybeAlign::new(0),
      function_ptr_align: MaybeAlign::new(0),
      the_function_ptr_align_type: FunctionPtrAlignType::Independent,
       mangling_mode: ManglingMode::None,
      legal_int_widths: Vec::new(), int_alignments: Vec::new(),
      float_alignments: Vec::new(), vector_alignments: Vec::new(),
      struct_alignment: LayoutAlignElem::get(Align::new(1),
        Align::new(8), 0),
      string_representation: desc.clone(), pointers: Vec::new(),
      non_integral_address_spaces: Vec::new()
    };
    let default_alignments = vec![
      (AlignType::IntegerAlign, LayoutAlignElem::get(Align::new(1), Align::new(1), 1)),
      (AlignType::IntegerAlign, LayoutAlignElem::get(Align::new(1), Align::new(1), 8)),
      (AlignType::IntegerAlign, LayoutAlignElem::get(Align::new(2), Align::new(2), 16)),
      (AlignType::IntegerAlign, LayoutAlignElem::get(Align::new(4), Align::new(4), 32)),
      (AlignType::IntegerAlign, LayoutAlignElem::get(Align::new(4), Align::new(8), 64)),
      (AlignType::FloatAlign, LayoutAlignElem::get(Align::new(2), Align::new(2), 16)),
      (AlignType::FloatAlign, LayoutAlignElem::get(Align::new(4), Align::new(4), 32)),
      (AlignType::FloatAlign, LayoutAlignElem::get(Align::new(8), Align::new(8), 64)),
      (AlignType::FloatAlign, LayoutAlignElem::get(Align::new(16), Align::new(16), 128)),
      (AlignType::VectorAlign, LayoutAlignElem::get(Align::new(8), Align::new(8), 64)),
      (AlignType::VectorAlign, LayoutAlignElem::get(Align::new(16), Align::new(16), 128)),
    ];
    for (kind, layout) in default_alignments {
      dl.set_alignment(kind, layout.abi_align, layout.pref_align, layout.type_bit_width)
    }
    dl.set_pointer_alignment_in_bits(0, Align::new(8),
      Align::new(8), 64, 64);
    dl.parse_specifier(&desc);
    dl
  }

  fn find_alignment_lower_bound() {}

  fn get_pointer_align_elem(&self, address_space: usize) -> &PointerAlignElem {
    if address_space != 0 {
      for ptr in &self.pointers {
        if ptr.address_space == address_space { return ptr; }
      }
    }
    debug_assert!(self.pointers[0].address_space == 0);
    return &self.pointers[0];
  }

  // Attempts to set alignment of the given type.
  fn set_alignment(&mut self, align_t: AlignType, abi_align: Align,
    pref_align: Align, bit_width: usize)
  {
    debug_assert!(log2(&abi_align) < 16 && log2(&pref_align) < 16,
      "Alignment too big.");
    //debug_assert!(pref_align < abi_align,
      //"Preferred alignment cannot be less than ABI alignment.");

    let alignments: &mut Vec<LayoutAlignElem>;
    match align_t {
      AlignType::AggregateAlign => {
        self.struct_alignment.abi_align = abi_align;
        self.struct_alignment.pref_align = pref_align;
        return;
      },
      AlignType::IntegerAlign => alignments = &mut self.int_alignments,
      AlignType::FloatAlign => alignments = &mut self.float_alignments,
      AlignType::VectorAlign => alignments = &mut self.vector_alignments
    }

    let mut found = false;
    for align_elem in alignments.iter_mut() {
      if align_elem.type_bit_width == bit_width {
        align_elem.abi_align = abi_align.clone();
        align_elem.pref_align = pref_align.clone();
        found = true;
      }
    }
    if !found {
      alignments.push(LayoutAlignElem::get(abi_align, pref_align, bit_width));
    }
  }

  // Attempts to set the alignment of a pointer in the given address space.
  fn set_pointer_alignment_in_bits(&mut self, address_space: usize, abi_align: Align,
    pref_align: Align, type_bit_width: usize, index_bit_width: usize)
  {
    //debug_assert!(pref_align < abi_align,
      //"Preferred alignment cannot be less than ABI alignment.");

    let mut found = false;
    for ptr_align in &mut self.pointers {
      if ptr_align.address_space == address_space {
        ptr_align.abi_align = abi_align.clone();
        ptr_align.pref_align = pref_align.clone();
        ptr_align.type_bit_width = type_bit_width;
        ptr_align.index_bit_width = index_bit_width;
        found = true;
      }
    }
    if !found {
      self.pointers.push(PointerAlignElem::get_in_bits(address_space, abi_align,
        pref_align, type_bit_width, index_bit_width));
    }
  }

  // Internal helper to get aligment for integer of given bitwidth.
  fn get_integer_alignment(&self, bit_width: usize, abi_or_pref: bool) -> &Align {
    for int_align in &self.int_alignments {
      if int_align.type_bit_width  == bit_width {
        if abi_or_pref {
          return &int_align.abi_align;
        } else {
          return &int_align.pref_align;
        }
      }
    }
    let last_int_align = self.int_alignments.last();
    if abi_or_pref {
      return &last_int_align.unwrap().abi_align;
    } else {
      return &last_int_align.unwrap().pref_align;
    }
  }

  // Internal helper method that returns requested alignment for type.
  fn get_alignment(&self, t: &dyn Type, abi_or_pref: bool) -> &Align {
    debug_assert!(t.is_sized(), "Cannot get_type_info() on a type that is unsized.");
    match t.get_type_id() {
      TypeID::Label => {
        if abi_or_pref {
          return self.get_pointer_abi_alignment(0)
        } else {
          return self.get_pointer_pref_alignment(0);
        }
      },
      TypeID::Pointer => {
        let ptr = t.as_any().downcast_ref::<PointerType>();
        let adder_space = ptr.unwrap().get_address_space();
        if abi_or_pref {
          return self.get_pointer_abi_alignment(adder_space);
        } else {
          return self.get_pointer_pref_alignment(adder_space);
        }
      },
      TypeID::Array => {
        let array = t.as_any().downcast_ref::<ArrayType>();
        return self.get_alignment(array.unwrap().get_element_type(), abi_or_pref);
      },
      TypeID::Integer =>
        return self.get_integer_alignment(t.get_integer_bit_width(), abi_or_pref),
      _ => unimplemented!("Not implemented.")
    }
  }

  // Attempts to parse a target data specification string.
  fn parse_specifier(&mut self, desc: &String) {
    let desc_splits: Vec<&str> = desc.split('-').collect();
    for desc_split in desc_splits {
      //let split = desc_split.split_once(':');
      //let tok = split.unwrap().0;
      //let _rest = split.unwrap().1;

      //if tok == "ni" {
      //}

      let mut chars = desc_split.chars();
      let next_char_1 = chars.next();
      let next_char_2 = chars.next();
      if next_char_1 == Some('F') {
        if next_char_2 == Some('i') {
          self.the_function_ptr_align_type = FunctionPtrAlignType::Independent;
        } else if next_char_2 == Some('n') {
          self.the_function_ptr_align_type = FunctionPtrAlignType::MultipleOfFunctionAlign;
        } else {
          debug_assert!(false, "Unknown function pointer alignment type in datalayout string.");
        }
        let mut alignment = 0;
        let mut n_str = chars.next().unwrap().to_string();
        if n_str.parse::<u64>().is_ok() && n_str.parse::<u64>().unwrap() % 8 == 0 {
          alignment = get_int_in_bytes(&n_str);
        } else {
          n_str.push(chars.next().unwrap());
          if n_str.parse::<u64>().is_ok() {
            alignment = get_int_in_bytes(&n_str);
          } else {
            debug_assert!(false, "Alignment is not a valid number.");
          }
        }
        if alignment != 0 && !math_extras::is_power_of_2_64(alignment) {
          debug_assert!(false, "Alignment is neither 0 nor a power of 2.")
        }
        self.function_ptr_align = MaybeAlign::new(alignment);
      }
      else if next_char_1 == Some('n') {
        let mut width = 0;
        let mut n_str = chars.next().unwrap().to_string();
        if n_str.parse::<u64>().is_ok() && n_str.parse::<u64>().unwrap() % 8 == 0 {
          width = get_int_in_bytes(&n_str);
        } else {
          n_str.push(chars.next().unwrap());
          if n_str.parse::<u64>().is_ok() {
            width = get_int_in_bytes(&n_str);
          } else {
            debug_assert!(false, "Alignment is not a valid number.");
          }
        }
        debug_assert!(width != 0);
        self.legal_int_widths.push(width as usize);
      }
    }
  }

  // Free all internal data structures.
  pub fn clear(&mut self) {
    self.legal_int_widths.clear();
    self.int_alignments.clear();
    self.float_alignments.clear();
    self.vector_alignments.clear();
    self.pointers.clear();
  }

  pub fn reset() {}
  pub fn parse() {}
  pub fn is_little_endian() {}
  pub fn is_big_endian() {}

  // Returns the string representation of the DataLayout.
  pub fn get_string_representation(&self) -> String {
    self.string_representation.clone()
  }
  
  // Test if the DataLayout was constructed from an empty string.
  pub fn is_default(&self) -> bool {
    self.string_representation.is_empty()
  }

  pub fn is_legal_integer(&self, _width: usize) -> bool { false }

  pub fn is_illegal_integer(&self, width: usize) -> bool {
    !self.is_legal_integer(width)
  }

  pub fn exceed_natural_stack_alignment() {}

  pub fn get_stack_alignment(&self) -> &MaybeAlign {
    &self.stack_nature_align
  }

  pub fn get_alloca_addr_space(&self) -> usize {
    self.alloca_addr_space
  }

  // Returns the alignment of function pointers, which may or may not be
  // related to the alignment of functions.
  pub fn get_function_ptr_align(&self) -> &MaybeAlign {
    &self.function_ptr_align
  }

  // Return the type of function pointer alignment.
  pub fn get_function_ptr_align_type(&self) -> &FunctionPtrAlignType {
    &self.the_function_ptr_align_type
  }

  pub fn get_program_address_space(&self) -> usize {
    self.program_addr_space
  }

  pub fn get_default_globals_address_space(&self) -> usize {
    self.default_globals_addr_space
  }

  // Returns true if symbols with leading question marks should not receive
  // IR mangling. True for Windows mangling modes.
  pub fn do_not_mangle_leading_question_mark(&self) -> bool {
    self.mangling_mode == ManglingMode::WinCOFF ||
    self.mangling_mode ==ManglingMode::WinCOFFX86
  }

  pub fn has_linker_private_global_prefix(&self) -> bool {
    self.mangling_mode == ManglingMode::MachO
  }

  pub fn get_linker_private_global_prefix(&self) -> String {
    if self.mangling_mode == ManglingMode::MachO {
      return String::from("l");
    }
    String::from("")
  }

  pub fn get_global_prefix(&self) -> char {
    match self.mangling_mode {
      ManglingMode::None => return '\0',
      ManglingMode::ELF => return '\0',
      ManglingMode::GOFF => return '\0',
      ManglingMode::Mips => return '\0',
      ManglingMode::WinCOFF => return '\0',
      ManglingMode::XCOFF => return '\0',
      ManglingMode::MachO => return '_',
      ManglingMode::WinCOFFX86 => return '_'
    }
  }

  pub fn get_private_global_prefix(&self) -> String {
    match self.mangling_mode {
      ManglingMode::None => return String::from(""),
      ManglingMode::ELF => return String::from(".L"),
      ManglingMode::GOFF => return String::from("@"),
      ManglingMode::Mips => return String::from("$"),
      ManglingMode::WinCOFF => return String::from(".L"),
      ManglingMode::XCOFF => return String::from("L.."),
      ManglingMode::MachO => return String::from("L"),
      ManglingMode::WinCOFFX86 => return String::from("L")
    }
  }

  pub fn get_mangling_component(&self) {}

  // Returns true if the specified type fits in a native integer type supported
  // by the CPU.
  pub fn fits_in_legal_integer(&self, width: usize) -> bool {
    for legal_int_width in &self.legal_int_widths {
      if width <= *legal_int_width { return true; }
    }
    false
  }

  // Layout pointer alignment.
  pub fn get_pointer_abi_alignment(&self, address_space: usize) -> &Align {
    &self.get_pointer_align_elem(address_space).abi_align
  }

  // Return target's alignment for stack-based pointers.
  pub fn get_pointer_pref_alignment(&self, address_space: usize) -> &Align {
    &self.get_pointer_align_elem(address_space).pref_align
  }

  // Layout pointer size in bytes, rounded up to a whole number of bytes.
  pub fn get_pointer_size(&self, address_space: usize) -> usize {
    divide_ceil(self.get_pointer_align_elem(address_space).type_bit_width, 8)
  }

  // Returns the maximum index size over all address spaces.
  pub fn get_max_index_size(&self) -> usize {
    let mut max_index_size = 0;
    for ptr in &self.pointers {
      let ptr_size = divide_ceil(ptr.type_bit_width, 8);
      if max_index_size < ptr_size {
        max_index_size = ptr_size;
      }
    }
    max_index_size
  }

  // Index size in bytes used for address calculation, rounded up to a whole 
  // number of bytes.
  pub fn get_index_size(&self, address_space: usize) -> usize {
    divide_ceil(self.get_pointer_align_elem(address_space).index_bit_width, 8)
  }

  // Return the address spaces containing non-integral pointers.
  // Pointers in this address space don't have a well-defined bitwise representation.
  pub fn get_non_integral_address_spaces(&self) -> &Vec<usize> {
    &self.non_integral_address_spaces
  }

  pub fn is_non_integral_address_space(&self, address_space: usize) -> bool {
    self.non_integral_address_spaces.contains(&address_space)
  }

  pub fn is_non_integral_pointer_type(&self, ptr: &PointerType) -> bool {
    self.is_non_integral_address_space(ptr.get_address_space())
  }

  // Layout pointer size, in bits.
  pub fn get_pointer_size_in_bits(&self, address_space: usize) -> usize {
    self.get_pointer_align_elem(address_space).type_bit_width
  }

  // Returns the maximum index size over all address spaces.
  pub fn get_max_index_size_in_bits(&self) -> usize {
    self.get_max_index_size() * 8
  }

  // Size in bits of index used for address calculation in get_element_ptr.
  pub fn get_index_size_in_bits(&self, address_space: usize) -> usize {
    self.get_pointer_align_elem(address_space).index_bit_width
  }

  // Layout pointer size, in bits, based on the type.
  pub fn get_pointer_type_size_in_bits(&self, t: &dyn Type) -> usize {
    debug_assert!(t.is_ptr_or_ptr_vector_type(),
      "This should only be called with a pointer or pointer vector type.");
    let scalar_t = t.get_scalar_type();
    let ptr_t = scalar_t.as_any().downcast_ref::<PointerType>();
    debug_assert!(ptr_t.is_some());
    self.get_pointer_size_in_bits(ptr_t.unwrap().get_address_space())
  }

  // Layout size of the index used in GEP calculation.
  pub fn get_index_type_size_in_bits(&self, t: &dyn Type) -> usize {
    debug_assert!(t.is_ptr_or_ptr_vector_type(),
      "This should only be called with a pointer or pointer vector type.");
    let scalar_t = t.get_scalar_type();
    let ptr_t = scalar_t.as_any().downcast_ref::<PointerType>();
    debug_assert!(ptr_t.is_some());
    self.get_index_size_in_bits(ptr_t.unwrap().get_address_space())
  }

  pub fn get_pointer_type_size(&self, t: &dyn Type) -> usize {
    self.get_pointer_type_size_in_bits(t) / 8
  }

  // Returns the number of bits necessary to hold the specified type.
  pub fn get_type_size_in_bits(&self, t: &dyn Type) -> TypeSize {
    debug_assert!(t.is_sized(), "Cannot get_type_info() on a type that is unsized.");
    match t.get_type_id() {
      TypeID::Label =>
        return TypeSize::fixed(self.get_pointer_size_in_bits(0)),
      TypeID::Pointer =>
        return TypeSize::fixed(self.get_pointer_size_in_bits(t.get_pointer_address_space())),
      TypeID::Integer =>
        return TypeSize::fixed(t.get_integer_bit_width()),
      TypeID::Half => return TypeSize::fixed(16),
      TypeID::BFloat => return TypeSize::fixed(16),
      TypeID::Float => return TypeSize::fixed(32),
      TypeID::Double => return TypeSize::fixed(64),
      TypeID::X86Mmx => return TypeSize::fixed(64),
      TypeID::PpcFp128 => return TypeSize::fixed(128),
      TypeID::Fp128 => return TypeSize::fixed(128),
      TypeID::X86Amx => return TypeSize::fixed(8192),
      TypeID::X86Fp80 => return TypeSize::fixed(80),
      _ => unreachable!("DataLayout::get_type_size_in_bits(): Unsupported type.")
    }
  }

  // Returns the maximum number of bytes that may be overwritten by
  // storing the specified type.
  // If t is a scalable vector type, the scalable property will be set
  // and the runtime size will be a positive integer multiple of the base size.
  // For example, returns 5 for i36 and 10 for x86_fp80.
  pub fn get_type_store_size(&self, t: &dyn Type) -> TypeSize {
    let base_size = self.get_type_size_in_bits(t);
    TypeSize::new(divide_ceil(base_size.get_known_min_value(),
      8), base_size.is_scalable())
  }

  // Return the maximum number of bits that may be overwritten by storing the
  // specified type; always a multiple of 8.
  pub fn get_type_store_size_in_bits(&self, t: &dyn Type) -> TypeSize {
    self.get_type_store_size(t) * 8
  }

  // Returns true if no extra padding bits are needed when storing the specified type.
  pub fn type_size_equal_store_size(&self, t: &dyn Type) -> bool {
    self.get_type_size_in_bits(t) == self.get_type_store_size_in_bits(t)
  }

  // Returns the offset in bytes between successive objects of the
  // specified type, including alignment padding.
  // If t is a scalable vector type, the scalable property will be set
  // and the runtime size will be a positive integer multiple of the base size.
  // This is the amount that alloca reserves for this type.
  // For example, returns 12 or 16 for x86_fp80, depending on alignment.
  pub fn get_type_alloc_size(&self, t: &dyn Type) -> TypeSize {
    align_to(self.get_type_store_size(t), self.get_abi_type_align(t).value())
  }

  // Returns the offset in bits between successive objects of the specified type,
  // including alignment padding; always a multiple of 8.
  pub fn get_type_alloc_size_in_bits(&self, t: &dyn Type) -> TypeSize {
    self.get_type_alloc_size(t) * 8
  }

  // Returns the minimum ABI-required alignment for the specified type.
  pub fn get_abi_type_alignment(&self, t: &dyn Type) -> usize {
    self.get_abi_type_align(t).value()
  }

  // Returns the minimum ABI-required  alignment for the spcified type.
  pub fn get_abi_type_align(&self, t: &dyn Type) -> &Align {
    self.get_alignment(t, true)
  }

  // Helper function to return 'alignment' if it's set or the resulit of
  // 'get_abi_type_alignment(t)', in any case the result is a valid alignment. 
  pub fn get_value_or_abi_type_alignment(&self, alignment: MaybeAlign, t: &dyn Type) -> Align {
    if alignment.shift_value() != 0 {
      Align::new(alignment.shift_value() as usize)
    } else {
      self.get_abi_type_align(t).clone()
    }
  }

  // Returns the minimum ABI-required alignment for an integer type of the
  // specified bitwidth.
  pub fn get_abi_integral_type_alignment(&self, bit_width: usize) -> &Align {
    self.get_integer_alignment(bit_width, true)
  }

  // Returns the preffered stack/global alignment for the specified type.
  pub fn get_pref_type_alignment(&self, t: &dyn Type) -> usize {
    self.get_pref_type_align(t).value()
  }

  // Returns the preffered stack/global alignment for the specified type.
  pub fn get_pref_type_align(&self, t: &dyn Type) -> &Align {
    self.get_alignment(t, false)
  }

  // Returns an integer (vector of integer) type with size at least as big as
  // that of a pointer of the given pointer (vector of pointer) type.
  pub fn get_int_ptr_type(&self, t: &dyn Type) -> Box<dyn Type> {
    debug_assert!(t.is_ptr_or_ptr_vector_type(),
      "Expected a pointer or pointer vector type.");
    let num_bits = self.get_pointer_type_size_in_bits(t);
    let int_t = IntegerType::get(blits_context(), num_bits as u32);
    if t.as_any().downcast_ref::<FixedVectorType>().is_some() {
      let _fixed_vec_t = t.as_any().downcast_ref::<FixedVectorType>().unwrap();
      // TODO
    } else if t.as_any().downcast_ref::<ScalableVectorType>().is_some() {
        
    }
    Box::new(int_t)
  }

  // Returns the smallest integer type with size at least as big as width bits.
  pub fn get_smallest_legal_int_type(&self, width: usize) -> Option<Box<dyn Type>> {
    for legal_int_width in &self.legal_int_widths {
      if width <= *legal_int_width {
        return Some(Box::new(get_int_n_type(blits_context(),
          *legal_int_width as u32)));
      }
    }
    None
  }

  // Returns the size of largest legal integer type size, or 0 if none are set.
  pub fn get_largest_legal_int_type(&self) -> Option<Box<dyn Type>> {
    let largest_size = self.get_largest_legal_int_type_size_in_bits();
    if largest_size != 0 {
      return Some(Box::new(get_int_n_type(blits_context(), largest_size as u32)));
    } else {
      return None;
    }
  }

  // Returns the size of larges legal integer type size, or 0 if none are set.
  pub fn get_largest_legal_int_type_size_in_bits(&self) -> usize {
    let mut max_width: usize = 0;
    for width in &self.legal_int_widths {
      if *width > max_width { max_width = *width }
    }
    max_width
  }

  // Returns the type of a GEP index in address space.
  pub fn get_index_type(&self, address_space: usize) -> IntegerType {
    IntegerType::get(blits_context(), address_space as u32)
  }

  pub fn get_indexed_offset_in_type() {}
  pub fn get_gep_indices_for_offset() {}
  pub fn get_struct_layout() {}
  pub fn get_preffered_align() {}
}

// Used to lazily calculate structure layout information for a target
// machine, based on the DataLayout structure.
#[derive(Debug)]
pub struct StructLayout {
  struct_size: TypeSize,
  struct_alignment: Align,
  is_padded: bool,
  num_elements: usize
}

impl StructLayout {
  pub fn new() {}

  pub fn get_size_in_bits(&self) -> &TypeSize {
    &self.struct_size
  }

  pub fn get_alignment(&self) -> &Align {
    &self.struct_alignment
  }

  // Returns whether the struct has padding or not between its fields.
  // Padding in nested element is not taken into account.
  pub fn has_padding(&self) -> bool {
    self.is_padded
  }

  pub fn get_element_containing_offset() {}
  pub fn get_member_offsets() {}
  pub fn get_element_offset() {}
  pub fn get_element_offset_in_bits() {}
}

// Get an unsigned integer, including error checks.
fn get_int(r: &String) -> u64 {
  let result: u64 = r.parse().unwrap();
  result
}

// Get an unsigned integer representing the number of bits and convert it into
// bytes. Error out of not a byte with multiple.
fn get_int_in_bytes(r: &String)  -> u64 {
  let result: u64 = get_int(r);
  debug_assert!(result % 8 == 0, "Number of bits must ne a byte width multiple.");
  result / 8
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::ir::type_;

  #[test]
  fn test_function_ptr_align() {
    assert_eq!(DataLayout::new(String::from("")).get_function_ptr_align(),
      &MaybeAlign::new(0));
    assert_eq!(DataLayout::new(String::from("Fi8")).get_function_ptr_align(),
      &MaybeAlign::new(1));
    assert_eq!(DataLayout::new(String::from("Fi16")).get_function_ptr_align(),
      &MaybeAlign::new(2));
    assert_eq!(DataLayout::new(String::from("Fi32")).get_function_ptr_align(),
      &MaybeAlign::new(4));
    assert_eq!(DataLayout::new(String::from("Fi64")).get_function_ptr_align(),
      &MaybeAlign::new(8));

    assert_eq!(DataLayout::new(String::from("Fn8")).get_function_ptr_align(),
      &MaybeAlign::new(1));
    assert_eq!(DataLayout::new(String::from("Fn16")).get_function_ptr_align(),
      &MaybeAlign::new(2));
    assert_eq!(DataLayout::new(String::from("Fn32")).get_function_ptr_align(),
      &MaybeAlign::new(4));
    assert_eq!(DataLayout::new(String::from("Fn64")).get_function_ptr_align(),
      &MaybeAlign::new(8));

    assert_eq!(DataLayout::new(String::from("")).get_function_ptr_align_type(),
      &FunctionPtrAlignType::Independent);
    assert_eq!(DataLayout::new(String::from("Fi8")).get_function_ptr_align_type(),
      &FunctionPtrAlignType::Independent);
    assert_eq!(DataLayout::new(String::from("Fn8")).get_function_ptr_align_type(),
      &FunctionPtrAlignType::MultipleOfFunctionAlign);

    assert_eq!(DataLayout::new(String::from("Fi8")), DataLayout::new(String::from("Fi8")));
    assert_ne!(DataLayout::new(String::from("Fi8")), DataLayout::new(String::from("Fi16")));
    assert_ne!(DataLayout::new(String::from("Fi8")), DataLayout::new(String::from("Fn8")));
  }

  #[test]
  fn test_value_or_abi_type_alignment() {
    let dl = DataLayout::new(String::from("Fi8"));
    let four_byte_align_t = type_::get_int_32_type(blits_context());
    assert_eq!(dl.get_value_or_abi_type_alignment(MaybeAlign::new(16), &four_byte_align_t),
      Align::new(16));
    assert_eq!(dl.get_value_or_abi_type_alignment(MaybeAlign::new(0), &four_byte_align_t),
      Align::new(4));
  }
}