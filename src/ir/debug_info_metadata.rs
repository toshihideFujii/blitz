#![allow(dead_code)]

// Declarations for metadata specific to debug info.

//use std::ops::BitOr;
use std::any::Any;
use crate::adt::{string_ref::StringRef, ap_int::APInt,
  string_switch::StringSwitch};
use super::{
  metadata::{MDNode, MDTuple, Metadata, MetadataKind, MDNodeBase,
    StorageType, MDString, Header, MDOperand},
  blits_context::BlitzContext
};

struct DITypeRefArray {
  n: MDTuple
}

impl DITypeRefArray {
  pub fn new(n: MDTuple) -> Self {
    DITypeRefArray { n: n }
  }

  pub fn get(&self) -> &MDTuple {
    &self.n
  }

  pub fn size(&self) -> usize {
    0 // TODO
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DIFlags {
  Zero = 0,
  Private = 1,
  Protected = 2,
  Public = 3,
  FwdDecl = 1 << 2,
  AppleBlock = 1 << 3,
  ReservedBit4 = 1 << 4,
  Virtual = 1 << 5,
  Artificial = 1 << 6,
  Explicit = 1 << 7,
  Prototyped = 1 << 8,
  ObjectPointer = 1 << 10,
  Vector = 1 << 11,
  StaticMember = 1 << 12,
  LValueReference = 1 << 13,
  RValueReference  = 1 << 14,
  ExportSymbols  = 1 << 15,
  SingleInheritance = 1 << 16,
  MultipleInheritance,
  VirtualInheritance,
  IntroducedVirtual = 1 << 18,
  BitField = 1 << 19,
  NoReturn = 1 << 20,
  TypePassByValue = 1 << 22,
  TypePassByReference = 1 << 23,
  EnumClass = 1 << 24,
  Thunk = 1 << 25,
  Nontrivial = 1 << 26,
  BigEndian = 1 << 27,
  LittleEndian = 1 << 28,
  AllCallsDescribe = 1 << 29,
  //Accessibility = DIFlags::Private | DIFlags::Protected
}
/*
impl BitOr<DIFlags> for DIFlags {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    self | rhs
  }
}
*/

// Target DWARF-like metadata node.
// A metadata node with a DWARF tag (i.e. a constant named DW_TAG_*,
// defined in binary_format_dwarf.rs).
// Called DINode because it's potentially used for non-DWARF output.
#[derive(Debug, Clone, PartialEq)]
struct DINode {
  node: MDNodeBase,
  sub_class_data_16: u32
}

impl DINode {
  pub fn new(_c: BlitzContext, id: MetadataKind, storage: StorageType, tag: u32) -> Self
  {
    DINode {
      node: MDNodeBase::new(id, storage),
      sub_class_data_16: tag
    }
  }

  pub fn get_operand_as<T>(&self, _i: u32) -> Option<T> {
    None
  }

  pub fn get_string_operand(&self, i: u32) -> StringRef {
    if self.get_operand_as::<MDString>(i).is_some() {
      return self.get_operand_as::<MDString>(i).as_ref().unwrap().get_string();
    }
    StringRef::new()
  }

  pub fn get_canonical_md_string(&self, c: &BlitzContext, s: StringRef) -> Option<MDString> {
    if s.empty() { return None; }
    Some(MDString::get(c, s))
  }

  // Allow subclasses to mutate the tag.
  pub fn set_tag(&mut self, tag: u32) {
    self.sub_class_data_16 = tag;
  }

  pub fn get_tag(&self) -> u32 {
    self.sub_class_data_16
  }

  pub fn get_flag(_flag: StringRef) -> DIFlags {
    DIFlags::AllCallsDescribe
  }

  pub fn get_flag_string(_flag: DIFlags) -> StringRef {
    StringRef::new()
  }

  pub fn split_flags() {}

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    match md.as_ref().get_metadata_id() {
      MetadataKind::GenericDINodekind => return true,
      MetadataKind::DISubrangeKind => return true,
      MetadataKind::DIEnumeratorKind => return true,
      MetadataKind::DIBasicTypeKind => return true,
      MetadataKind::DIStringTypeKind => return true,
      MetadataKind::DIDerivedTypeKind => return true,
      MetadataKind::DICompositeTypeKind => return true,
      MetadataKind::DISubroutineTypeKind => return true,
      MetadataKind::DIFileKind => return true,
      MetadataKind::DICompileUnitKind => return true,
      MetadataKind::DISubprogramKind => return true,
      MetadataKind::DILexicalBlockKind => return true,
      MetadataKind::DILexicalBlockFileKind => return true,
      MetadataKind::DINamespaceKind => return true,
      MetadataKind::DICommonBlockKind => return true,
      MetadataKind::DITemplateTypeParameterKind => return true,
      MetadataKind::DITemplateValueParameterKind => return true,
      MetadataKind::DIGlobalVariableKind => return true,
      MetadataKind::DILocalVariableKind => return true,
      MetadataKind::DILabelKind => return true,
      MetadataKind::DIImportedEntityKind => return true,
      MetadataKind::DIModuleKind => return true,
      MetadataKind::DIGenericSubrangeKind => return true,
      MetadataKind::DIAssignIDKind => return true,
      _ => return false,
    };
  }
}

impl MDNode for  DINode {
  fn get_header(&self) -> &super::metadata::Header {
    self.node.get_header()
  }

  fn get_operand(&self, i: usize) -> MDOperand {
    self.node.get_operand(i)
  }
}

// Generic tagged DWARF-like metadata node.
// An un-specialized DWARF-like metadata node. The first operand is a
// (possibliy empty) null-separated MDString header that contains arbitrary
// fields. The remaining operands are dwarf-operands(), and are pointers
// to other metadata.
#[derive(Debug)]
struct GenericDINode {
  node: DINode,
  sub_class_data_32: u32
}

impl GenericDINode {
  pub fn new(c: BlitzContext, storage: StorageType, hash: u32, tag: u32) -> Self {
    GenericDINode {
      node: DINode::new(c, MetadataKind::GenericDINodekind, storage, tag),
      sub_class_data_32: hash
    }
  }

  pub fn set_hash(&mut self, hash: u32) {
    self.sub_class_data_32 = hash;
  }

  pub fn recalculate_hash() {}
  pub fn get_impl() {}

  pub fn get_hash(&self) -> u32 {
    self.sub_class_data_32
  }

  pub fn get_tag(&self) -> u32 {
    self.node.get_tag()
  }

  pub fn get_header(&self) -> StringRef {
    self.node.get_string_operand(0)
  }

  pub fn get_raw_header(&self) -> Option<MDString> {
    self.node.get_operand_as::<MDString>(0)
  }

  pub fn dwarf_operands() {}

  pub fn get_num_dwarf_operands(&self) -> usize {
    self.get_num_operands() - 1
  }

  pub fn get_dwarf_operand(&self, i: usize) -> MDOperand {
    self.get_operand(i + 1)
  }

  pub fn replace_dwarff_operand_with(&self, i: u32, new: Box<dyn Metadata>) {
    self.node.replace_operand_with(i + 1, new);
  }

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::GenericDINodekind
  }
}

impl MDNode for GenericDINode {
  fn get_header(&self) -> &Header {
    self.node.get_header()
  }

  fn get_num_operands(&self) -> usize {
    self.node.get_num_operands()
  }

  fn get_operand(&self, i: usize) -> MDOperand {
    self.node.get_operand(i)
  }
}

// Assignment ID.
// Used to link stores (as an attachment) and dbg.assigns (as an operand).
// DIAssignID metadata is never uniqued as we compare instances using
// referential equality (the instance/address is the ID).
#[derive(Debug)]
struct DIAssignID {
  node: MDNodeBase
}

impl DIAssignID {
  pub fn new(_c: BlitzContext, storage: StorageType) -> Self {
    DIAssignID {
      node: MDNodeBase::new(MetadataKind::DIAssignIDKind, storage)
    }
  }

  fn get_impl(c: BlitzContext, storage: StorageType, _should_create: bool) -> DIAssignID {
    DIAssignID::new(c, storage)
  }

  pub fn replace_operand_with() {}

  pub fn get_distinct(c: BlitzContext) -> DIAssignID {
    DIAssignID::get_impl(c, StorageType::Distinct, false)
  }

  pub fn get_temporary(c: BlitzContext) -> DIAssignID {
    DIAssignID::get_impl(c, StorageType::Temporary, false)
  }

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DIAssignIDKind
  }
}

impl MDNode for DIAssignID {
  fn get_header(&self) -> &Header {
    self.node.get_header()
  }

  fn get_operand(&self, i: usize) -> MDOperand {
    self.node.get_operand(i)
  }
}

// Array subrange.
struct DISubrange {}
impl DISubrange {
  pub fn new() {}
  pub fn get_raw_count_node() {}
  pub fn get_raw_lower_bound() {}
  pub fn get_raw_upper_bound() {}
  pub fn get_raw_stride() {}
  pub fn get_count() {}
  pub fn get_lower_bound() {}
  pub fn get_upper_bound() {}
  pub fn get_stride() {}
  pub fn class_of() {}
}

struct DIGenericSubrange {}
impl DIGenericSubrange {
  pub fn new() {}
  pub fn get_raw_count_node() {}
  pub fn get_raw_lower_bound() {}
  pub fn get_raw_upper_bound() {}
  pub fn get_raw_stride() {}
  pub fn get_count() {}
  pub fn get_lower_bound() {}
  pub fn get_upper_bound() {}
  pub fn get_stride() {}
  pub fn class_of() {}
}

// Enumration value.
pub struct DIEnumerator {
  node: DINode,
  value: APInt,
  is_unsigned: bool
}

impl DIEnumerator {
  pub fn new(c: BlitzContext, storage: StorageType, value: APInt,
    is_unsigned: bool) -> Self
  {
    DIEnumerator {
      node: DINode::new(c, MetadataKind::DIEnumeratorKind, storage, 0),
      value: value, is_unsigned: is_unsigned
    }
  }

  pub fn get(c: BlitzContext, value: APInt,
    is_unsigned: bool, _name: StringRef) -> Self
  {
    DIEnumerator::new(c, StorageType::Distinct, value, is_unsigned) // TODO
  }

  pub fn get_value(&self) -> &APInt {
    &self.value
  }

  pub fn is_unsigned(&self) -> bool {
    self.is_unsigned
  }

  pub fn get_name(&self) -> StringRef {
    self.node.get_string_operand(0)
  }

  pub fn get_raw_name(&self) -> Option<MDString> {
    self.node.get_operand_as::<MDString>(0)
  }

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id()== MetadataKind::DIEnumeratorKind
  }
}

// Base class for scope-like contexts.
// Base class for lexical scopes and types (which are also declaration contexts).
pub trait DIScope {
  fn get_file(&self) -> Option<&DIFile> { None }
  fn get_file_name(&self) -> Option<StringRef> { None }
  fn get_directory(&self) -> Option<StringRef> { None }
  fn get_source(&self) -> Option<StringRef> { None }

  fn get_name(&self) -> StringRef { 
    debug_assert!(false, "Unhandled type of scope.");
    StringRef::new()
  }

  fn get_scope(&self) -> Option<Box<dyn DIScope>> {
    debug_assert!(false, "Unhandled type of scope.");
    None
  }

  fn get_raw_file(&self) {}
}

#[derive(Debug, Clone, PartialEq)]
struct DIScopeBase {
  node: DINode
}

impl DIScopeBase {
  pub fn new(c: BlitzContext, id: MetadataKind,
    storage: StorageType, tag: u32) -> Self
  {
    DIScopeBase { node: DINode::new(c, id, storage, tag) }
  }

  pub fn get_operand_as<T>(&self, i: u32) -> Option<T> {
    self.node.get_operand_as(i)
  }

  pub fn get_string_operand(&self, i: u32) -> StringRef {
    self.node.get_string_operand(i)
  }

  pub fn get_canonical_md_string(&self, c: &BlitzContext,
    s: StringRef) -> Option<MDString>
  {
    self.node.get_canonical_md_string(c, s)
  }

  // Allow subclasses to mutate the tag.
  pub fn set_tag(&mut self, tag: u32) {
    self.node.set_tag(tag)
  }

  pub fn get_tag(&self) -> u32 {
    self.node.get_tag()
  }
}

impl MDNode for DIScopeBase {
  fn get_header(&self) -> &Header {
    self.node.get_header()
  }

  fn get_operand(&self, i: usize) -> MDOperand {
    self.node.get_operand(i)
  }
}

impl DIScope for DIScopeBase {}

// Which algorithm (e.g. MD5) a checksum was generated with.
// The encoding is explicit because it is used directly in Bitcode.
// The value 0 is reserved to indicate the absence of a checksum in Bitcode.
#[derive(Debug, Clone, PartialEq)]
pub enum ChecksumKind {
  MD5 = 1,
  SHA1 = 2,
  SHA256 = 3
}

struct ChecksumInfo {}
impl ChecksumInfo {
  pub fn new() {}
}

// File.
pub struct DIFile {
  scope: DIScopeBase,
  source: Option<MDString>
}

impl DIFile {
  pub fn new(c: BlitzContext, storage: StorageType,
      source: Option<MDString>) -> Self
  {
    DIFile {
      scope: DIScopeBase::new(c, MetadataKind::DIFileKind, storage, 0),
      source: source
    }
  }

  pub fn get_filename(&self) -> StringRef {
    self.scope.get_string_operand(0)
  }

  pub fn get_directory(&self) -> StringRef {
    self.scope.get_string_operand(1)
  }

  pub fn get_checksum() {}

  pub fn get_source(&self) -> Option<StringRef> {
    if self.source.is_some() {
      return Some(self.source.as_ref().unwrap().get_string());
    }
    None
  }

  pub fn get_raw_filename(&self) -> Option<MDString> {
    self.scope.get_operand_as::<MDString>(0)
  }

  pub fn get_raw_directory(&self) -> Option<MDString> {
    self.scope.get_operand_as::<MDString>(1)
  }

  pub fn get_raw_source(&self) -> &MDString {
    self.source.as_ref().unwrap()
  }

  pub fn get_checksum_kind_as_string(kind: ChecksumKind) -> StringRef {
    match kind {
      ChecksumKind::MD5 => return StringRef::new_from_string("MD5"),
      ChecksumKind::SHA1 => return StringRef::new_from_string("SHA1"),
      ChecksumKind::SHA256 => return StringRef::new_from_string("SHA256")
    };
  }

  pub fn get_checksum_kind(s: StringRef) -> Option<ChecksumKind> {
    let mut switch = StringSwitch::new(s);
    switch.case(StringRef::new_from_string("MD5"),
    Some(ChecksumKind::MD5))
      .case(StringRef::new_from_string("SHA1"),
      Some(ChecksumKind::SHA1))
      .case(StringRef::new_from_string("SHA256"),
      Some(ChecksumKind::SHA256))
      .default(None)
  }

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DIFileKind
  }
}

impl DIScope for DIFile {
  fn get_file(&self) -> Option<&DIFile> {
    self.scope.get_file()
  }

  fn get_file_name(&self) -> Option<StringRef> {
    self.scope.get_file_name()
  }

  fn get_directory(&self) -> Option<StringRef> {
    self.scope.get_directory()
  }

  fn get_source(&self) -> Option<StringRef> {
    self.scope.get_source()
  }
}

// Base class for types.
pub trait DIType {
  fn get_line(&self) -> u32 { 0 }
  fn get_size_in_bits(&self) -> u64 { 0 }
  fn get_align_in_bits(&self) -> u32 { 0 }
  fn get_offset_in_bits(&self) -> u64 { 0 }
  fn get_flags(&self) -> DIFlags { DIFlags::Zero }
  fn get_scope(&self) {}
  fn get_name(&self) -> StringRef { StringRef::new() }
  fn get_raw_scope(&self) -> Option<Box<dyn Metadata>> { None }
  fn get_raw_name(&self) -> Option<MDString> { None }
  fn is_private(&self) -> bool { false }
  fn is_protected(&self) -> bool { false }
  fn is_public(&self) -> bool { false }
  fn is_forward_decl(&self) -> bool { false }
  fn is_apple_block_extension(&self) -> bool { false }
  fn is_virtual(&self) -> bool { false }
  fn is_artificial(&self) -> bool { false }
  fn is_object_pointer(&self) -> bool { false }
  fn is_objc_class_complete(&self) -> bool { false }
  fn is_vector(&self) -> bool { false }
  fn is_bit_field(&self) -> bool { false }
  fn is_static_member(&self) -> bool { false }
  fn is_l_value_reference(&self) -> bool { false }
  fn is_r_value_reference(&self) -> bool { false }
  fn is_type_pass_by_value(&self) -> bool { false }
  fn is_type_pass_by_reference(&self) -> bool { false }
  fn is_big_endian(&self) -> bool { false }
  fn is_little_endian(&self) -> bool { false }
  fn get_export_symbols(&self) -> bool { false }
}

#[derive(Debug, Clone, PartialEq)]
struct DITypeBase {
  scope: DIScopeBase,
  line: u32,
  size_in_bits: u64,
  align_in_bits: u32,
  offset_in_bits: u64,
  flags: DIFlags
}

impl DITypeBase {
  pub fn new(c: BlitzContext, id: MetadataKind, storage: StorageType,
    tag: u32, line: u32, size_in_bits: u64, align_in_bits: u32,
    offset_in_bits: u64, flags: DIFlags) -> Self
  {
    DITypeBase {
      scope: DIScopeBase::new(c, id, storage, tag),
      line: line, size_in_bits: size_in_bits, align_in_bits: align_in_bits,
      offset_in_bits: offset_in_bits, flags: flags
    }
  }

  pub fn get_operand(&self, i: usize) -> MDOperand {
    self.scope.get_operand(i)
  }

  pub fn get_operand_as<T>(&self, i: u32) -> Option<T> {
    self.scope.get_operand_as(i)
  }

  pub fn get_string_operand(&self, i: u32) -> StringRef {
    self.scope.get_string_operand(i)
  }
}

impl DIType for DITypeBase {
  fn get_line(&self) -> u32 {
    self.line
  }

  fn get_size_in_bits(&self) -> u64 {
    self.size_in_bits
  }

  fn get_align_in_bits(&self) -> u32 {
    self.align_in_bits
  }

  fn get_offset_in_bits(&self) -> u64 {
    self.offset_in_bits
  }

  fn get_flags(&self) -> DIFlags {
    self.flags.clone()
  }

  fn get_name(&self) -> StringRef {
    self.scope.get_string_operand(2)
  }

  fn get_raw_name(&self) -> Option<MDString> {
    self.scope.get_operand_as::<MDString>(2)
  }

  fn is_private(&self) -> bool {
    let accessbility = DIFlags::Private as u32 |
      DIFlags::Protected as u32 | DIFlags::Public as u32;
    self.flags.clone() as u32 & accessbility == DIFlags::Private as u32
  }

  fn is_protected(&self) -> bool {
    let accessbility = DIFlags::Private as u32 |
      DIFlags::Protected as u32 | DIFlags::Public as u32;
    self.flags.clone() as u32 & accessbility == DIFlags::Protected as u32
  }

  fn is_public(&self) -> bool {
    let accessbility = DIFlags::Private as u32 |
      DIFlags::Protected as u32 | DIFlags::Public as u32;
    self.flags.clone() as u32 & accessbility == DIFlags::Public as u32
  }

  fn is_forward_decl(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::FwdDecl as u32 != 0
  }

  fn is_virtual(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::Virtual as u32 != 0
  }

  fn is_artificial(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::Artificial as u32 != 0
  }

  fn is_object_pointer(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::ObjectPointer as u32 != 0
  }

  fn is_vector(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::Vector as u32 != 0
  }

  fn is_bit_field(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::BitField as u32 != 0
  }

  fn is_static_member(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::StaticMember as u32 != 0
  }

  fn is_l_value_reference(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::LValueReference as u32 != 0
  }

  fn is_r_value_reference(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::RValueReference as u32 != 0
  }

  fn is_type_pass_by_value(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::TypePassByValue as u32 != 0
  }

  fn is_type_pass_by_reference(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::TypePassByReference as u32 != 0
  }

  fn is_big_endian(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::BigEndian as u32 != 0
  }

  fn is_little_endian(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::LittleEndian as u32 != 0
  }

  fn get_export_symbols(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::ExportSymbols as u32 != 0
  }

}

enum Signedness {
  Signed,
  Unsigned
}

// Basic type, like 'int' or 'float'.
struct DIBasicType {
  node: DITypeBase,
  encoding: u32
}

impl DIBasicType {
  pub fn new(c: BlitzContext, storage: StorageType, tag: u32,
    size_in_bits: u64, align_in_bits: u32, encoding: u32, flags: DIFlags) -> Self
  {
    DIBasicType {
      node: DITypeBase::new(c, MetadataKind::DIBasicTypeKind, storage, tag,
        0, size_in_bits, align_in_bits, 0, flags),
      encoding: encoding
    }
  }

  pub fn get_impl() {}

  pub fn get_encoding(&self) -> u32 {
    self.encoding
  }

  pub fn get_signedness() {}

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DIBasicTypeKind
  }
}

impl DIType for DIBasicType {}

// String type, Fortran CJARACTER(n).
struct DIStringType {
  node: DITypeBase,
  encoding: u32
}

impl DIStringType {
  pub fn new(c: BlitzContext, storage: StorageType, tag: u32,
    size_in_bits: u64, align_in_bits: u32, encoding: u32) -> Self
  {
    DIStringType { 
      node: DITypeBase::new(c, MetadataKind::DIStringTypeKind,
        storage, tag, 0, size_in_bits, align_in_bits, 0,
        DIFlags::Zero),
      encoding: encoding
    }
  }

  pub fn get_impl() {}

  pub fn get_string_length() {}
  pub fn get_string_length_exp() {}
  pub fn get_string_location_exp() {}

  pub fn get_encoding(&self) -> u32 {
    self.encoding
  }

  pub fn get_raw_string_length(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(3)
  }

  pub fn get_raw_string_length_exp(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(4)
  }

  pub fn get_raw_string_location_exp(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(5)
  }

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DIStringTypeKind
  }
}

impl DIType for DIStringType {}

// Derived types.
// This includes qualified types, pointers, references, friends, typedefs,
// and class members.
struct DIDerivedType {
  node: DITypeBase
}

impl DIDerivedType {
  pub fn new(c: BlitzContext, storage: StorageType, tag: u32, line: u32,
    size_in_bits: u64, align_in_bits: u32, offset_in_bits: u64, flags: DIFlags) -> Self
  {
    DIDerivedType { node: DITypeBase::new(c, MetadataKind::DIDerivedTypeKind, storage,
      tag, line, size_in_bits, align_in_bits, offset_in_bits, flags) }
  }

  pub fn get_base_type() {}

  pub fn get_raw_base_type(&self) -> MDOperand { // // TOFO: -> Metadata
    self.node.get_operand(3)
  }

  pub fn get_dwarf_address_space() {}

  // Get etra data associated with this derived type.
  // Class type for pointer members, global constant wrapper for static members,
  // or virtual base pointer offset for inheritance.
  pub fn get_extra_data(&self) -> MDOperand { // TOFO: -> Metadata
    self.get_raw_extra_data()
  }

  pub fn get_raw_extra_data(&self) -> MDOperand { // TOFO: -> Metadata
    self.node.get_operand(4)
  }

  pub fn get_annotations() {}

  pub fn get_raw_annotations(&self) -> MDOperand { // TOFO: -> Metadata
    self.node.get_operand(5)
  }

  pub fn get_class_type() {}
  pub fn get_storage_offset_in_bits() {}
  pub fn get_constant() {}
  pub fn get_discriminat_value() {}

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DIDerivedTypeKind
  }
}

// Composite types.
#[derive(Debug, Clone, PartialEq)]
pub struct DICompositeType {
  node: DITypeBase,
  run_time_lang: u32
}

impl DICompositeType {
  pub fn new(c: BlitzContext, storage: StorageType, tag: u32, line: u32,
    run_time_lang: u32, size_in_bits: u64, align_in_bits: u32,
    offset_in_bits: u64, flags: DIFlags) -> Self
  {
    DICompositeType {
      node: DITypeBase::new(c, MetadataKind::DICompositeTypeKind, storage,
        tag, line, size_in_bits, align_in_bits, offset_in_bits, flags),
      run_time_lang: run_time_lang
    }
  }

  pub fn get_impl() {}

  pub fn get_odr_type() {}
  pub fn build_odr_type() {}
  pub fn get_base_type() {}
  pub fn get_elements() {}
  pub fn get_vtable_holder() {}
  pub fn get_template_params() {}

  pub fn get_identifier(&self) -> StringRef {
    self.node.get_string_operand(7)
  }

  pub fn get_runtime_lang() {}

  pub fn get_raw_base_type(&self) -> MDOperand { // TOFO: -> Metadata
    self.node.get_operand(3)
  }

  pub fn get_raw_elements(&self) -> MDOperand { // TOFO: -> Metadata
    self.node.get_operand(4)
  }

  pub fn get_raw_vtable_holder(&self) -> MDOperand { // TOFO: -> Metadata
    self.node.get_operand(5)
  }

  pub fn get_raw_template_params(&self) -> MDOperand { // TOFO: -> Metadata
    self.node.get_operand(6)
  }

  pub fn get_raw_identifier(&self) -> Option<MDString> {
    self.node.get_operand_as::<MDString>(7)
  }

  pub fn get_raw_discriminator(&self) -> MDOperand { // TOFO: -> Metadata
    self.node.get_operand(8)
  }

  pub fn get_discriminator() {}

  pub fn get_raw_data_location(&self) -> MDOperand { // TOFO: -> Metadata
    self.node.get_operand(9)
  }

  pub fn get_data_location() {}
  pub fn get_data_location_exp() {}

  pub fn get_raw_associated(&self) -> MDOperand { // TOFO: -> Metadata
    self.node.get_operand(10)
  }

  pub fn get_associated_exp() {}

  pub fn get_raw_allocated(&self) -> MDOperand {  // TOFO: -> Metadata
    self.node.get_operand(11)
  }

  pub fn get_allocated_exp() {}

  pub fn get_raw_rank(&self) -> MDOperand { // TOFO: -> Metadata
    self.node.get_operand(12)
  }

  pub fn get_rank_exp() {}

  pub fn get_raw_annotations(&self) -> MDOperand { // TOFO: -> Metadata
    self.node.get_operand(13)
  }

  pub fn get_annotations() {}
  pub fn replace_elements() {}
  pub fn replace_vtable_holder() {}
  pub fn replace_template_params() {}

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DICompositeTypeKind
  }
}

// Type array for a subprogram.
struct DISubroutineType {
  node: DITypeBase,
  cc: u8
}

impl DISubroutineType {
  pub fn new(c: BlitzContext, storage: StorageType, flags: DIFlags, cc: u8) -> Self {
    DISubroutineType { node: DITypeBase::new(c, MetadataKind::DISubroutineTypeKind,
      storage, 0, 0, 0, 0, 0, flags),
      cc: cc
    }
  }

  pub fn get_impl() {}

  pub fn get_cc(&self) -> u8 {
    self.cc
  }

  pub fn get_type_array() {}

  pub fn get_raw_type_array(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(3)
  }

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DISubroutineTypeKind
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DebugEmissionKind {
  NoDebug = 0,
  FullDebug,
  LineTablesOnly,
  DebugDirectivesOnly,
}

#[derive(Debug, Clone)]
pub enum DebugNameTableKind {
  Default,
  GNU,
  None
}

// Compile unit.
//#[derive(Debug, Clone)]
pub struct DICompileUnit {
  node: DINode,
  source_languages: u32,
  is_optimized: bool,
  runtime_version: u32,
  emission_kind: DebugEmissionKind,
  dwo_id: u64,
  split_debug_inlining: bool,
  debug_info_for_profiling: bool,
  name_table_kind: DebugNameTableKind,
  ranges_base_address: bool
}

impl DICompileUnit {
  pub fn new() {}

  pub fn get_emission_kind(&self) -> DebugEmissionKind {
    self.emission_kind.clone()
  }

  pub fn emission_kind_string() {}

  pub fn get_name_table_kind(&self) -> DebugNameTableKind {
    self.name_table_kind.clone()
  }

  pub fn name_table_kind_string() {}
  pub fn get_impl() {}

  pub fn get_source_language(&self) -> u32 {
    self.source_languages
  }

  pub fn is_optimized(&self) -> bool {
    self.is_optimized
  }

  pub fn get_runtime_version(&self) -> u32 {
    self.runtime_version
  }

  pub fn is_debug_directives_only(&self) -> bool {
    self.emission_kind == DebugEmissionKind::DebugDirectivesOnly
  }

  pub fn get_debug_info_for_profiling(&self) -> bool {
    self.debug_info_for_profiling
  }

  pub fn get_ranges_base_address(&self) -> bool {
    self.ranges_base_address
  }

  pub fn get_producer(&self) -> StringRef {
    self.node.get_string_operand(1)
  }

  pub fn get_flags(&self) -> StringRef {
    self.node.get_string_operand(2)
  }

  pub fn get_split_debug_filename(&self) -> StringRef {
    self.node.get_string_operand(3)
  }

  pub fn get_enum_types() {}
  pub fn get_retained_types() {}
  pub fn get_clobal_variables() {}
  pub fn get_imported_entities() {}
  pub fn get_macros() {}

  pub fn get_dwo_id(&self) -> u64 {
    self.dwo_id
  }

  pub fn set_dwo_id(&mut self, dwo_id: u64) {
    self.dwo_id = dwo_id;
  }

  pub fn get_split_debug_inlining(&self) -> bool {
    self.split_debug_inlining
  }

  pub fn set_split_debug_inlining(&mut self, split_debug_inlining: bool) {
    self.split_debug_inlining = split_debug_inlining;
  }

  pub fn get_sys_root(&self) -> StringRef {
    self.node.get_string_operand(9)
  }

  pub fn get_sdk(&self) -> StringRef {
    self.node.get_string_operand(10)
  }

  pub fn get_raw_producer(&self) -> Option<MDString> {
    self.node.get_operand_as::<MDString>(1)
  }

  pub fn get_raw_flags(&self) -> Option<MDString> {
    self.node.get_operand_as::<MDString>(2)
  }

  pub fn get_raw_split_debug_filename(&self) -> Option<MDString> {
    self.node.get_operand_as::<MDString>(3)
  }

  pub fn get_raw_enum_types(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(4)
  }

  pub fn get_raw_retained_types(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(5)
  }

  pub fn get_raw_global_variables(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(6)
  }

  pub fn get_raw_imported_entities(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(7)
  }

  pub fn get_raw_macros(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(8)
  }

  pub fn get_raw_sys_root(&self) -> Option<MDString> {
    self.node.get_operand_as::<MDString>(9)
  }

  pub fn get_raw_sdk(&self) -> Option<MDString> {
    self.node.get_operand_as::<MDString>(10)
  }

  pub fn replace_enum_types() {}
  pub fn replace_retained_types() {}
  pub fn replace_global_variables() {}
  pub fn replace_imported_entities() {}
  pub fn replace_macros() {}

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DICompileUnitKind
  }
}

// A scope for locals.
// A legal scope for lexical blocks, local variables, and debuh info
// locations.
struct DILocalScope {}
impl DILocalScope {
  pub fn new() {}
  pub fn get_subprogram() {}
  pub fn get_non_lexical_block_file_scope() {}
  pub fn class_of() {}
}

#[derive(Debug, Clone, PartialEq)]
pub enum DISPFlags {
  Zero,
  Virtual,
  PreVirtual,
  LocalToUnit,
  Definition,
  Optimized,
  Pure,
  Elemental,
  Recursive,
  MainSubprogram,
  Deleted
}

// Subprogram description.
#[derive(Debug)]
pub struct DISubprogram {
  line: u32,
  scope_line: u32,
  virtual_index: u32,
  this_adjustment: i32,
  flags: DIFlags,
  sp_flags: DISPFlags
}

impl DISubprogram {
  pub fn new() {}
  pub fn get_flag() {}
  pub fn get_flag_string() {}
  pub fn split_flags() {}
  pub fn to_sp_flags() {}
  pub fn clone_with_flags() {}

  pub fn get_line(&self) -> u32 {
    self.line
  }

  pub fn get_virtuality() {}

  pub fn get_virtual_index(&self) -> u32 {
    self.virtual_index
  }

  pub fn get_this_adjustment(&self) -> i32 {
    self.this_adjustment
  }

  pub fn get_scope_line(&self) -> u32 {
    self.scope_line
  }

  pub fn get_flags(&self) -> DIFlags {
    self.flags.clone()
  }

  pub fn get_sp_flags(&self) -> DISPFlags {
    self.sp_flags.clone()
  }

  pub fn is_local_to_unit(&self) -> bool {
    self.sp_flags.clone() as u32 & DISPFlags::LocalToUnit as u32 != 0
  }

  pub fn is_definition(&self) -> bool {
    self.sp_flags.clone() as u32 & DISPFlags::Definition as u32 != 0
  }

  pub fn is_optimized(&self) -> bool {
    self.sp_flags.clone() as u32 & DISPFlags::Optimized as u32 != 0
  }

  pub fn is_main_subprogram(&self) -> bool {
    self.sp_flags.clone() as u32 & DISPFlags::MainSubprogram as u32 != 0
  }

  pub fn is_artificial(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::Artificial as u32 != 0
  }

  pub fn is_private(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::Private as u32 != 0
  }

  pub fn is_protected(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::Protected as u32 != 0
  }

  pub fn is_public(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::Public as u32 != 0
  }

  pub fn is_explicit(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::Explicit as u32 != 0
  }

  pub fn is_prototyped(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::Prototyped as u32 != 0
  }

  pub fn are_all_calls_described(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::AllCallsDescribe as u32 != 0
  }

  pub fn is_pure(&self) -> bool {
    self.sp_flags.clone() as u32 & DISPFlags::Pure as u32 != 0
  }

  pub fn is_elemental(&self) -> bool {
    self.sp_flags.clone() as u32 & DISPFlags::Elemental as u32 != 0
  }

  pub fn is_recursive(&self) -> bool {
    self.sp_flags.clone() as u32 & DISPFlags::Recursive as u32 != 0
  }

  // Check if this is deleted member function.
  pub fn is_deleted(&self) -> bool {
    self.sp_flags.clone() as u32 & DISPFlags::Deleted as u32 != 0
  }

  // Check if this is reference-qualified.
  pub fn is_l_value_reference(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::LValueReference as u32 != 0
  }

  // Check if this is rvalue-reference-qualified.
  pub fn is_r_value_reference(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::RValueReference as u32 != 0
  }

  // Check if this is marked as noreturn.
  pub fn is_no_return(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::NoReturn as u32 != 0
  }

  // Check if this routine is a compiler-generated thunk.
  pub fn is_thunk(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::Thunk as u32 != 0
  }

  pub fn get_scope() {}
  pub fn get_name() {}
  pub fn get_linkage_name() {}
  pub fn replace_linkage_name() {}
  pub fn get_type() {}
  pub fn get_containing_type() {}
  pub fn replace_type() {}
  pub fn get_unit() {}
  pub fn replace_unit() {}
  pub fn get_declaration() {}
  pub fn get_retained_nodes() {}
  pub fn get_thrown_types() {}
  pub fn get_annotations() {}
  pub fn get_target_func_name() {}
  pub fn get_raw_scope() {}
  pub fn get_raw_name() {}
  pub fn get_raw_linkage_name() {}
  pub fn get_raw_type() {}
  pub fn get_raw_unit() {}
  pub fn get_raw_declaration() {}
  pub fn get_raw_retained_nodes() {}
  pub fn get_raw_containing_type() {}
  pub fn get_raw_template_params() {}
  pub fn get_raw_thrown_types() {}
  pub fn get_raw_annotations() {}
  pub fn get_raw_target_func_name() {}
  pub fn replace_raw_linkage_name() {}
  pub fn describes() {}

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DISubprogramKind
  }
}

//impl MDNode for DISubprogram {
//}

// Debug location.
// A debug location in source code, used for debug info and otherwise.
#[derive(Debug)]
pub struct DILocation {
  node: Box<dyn MDNode>,
  sub_class_data_1: bool,
  sub_class_data_16: u32,
  sub_class_data_32: u32 
}

impl DILocation {
  pub fn new(_c: BlitzContext, storage: StorageType, line: u32,
    column: u32, implicit_code: bool) -> Self
  {
    DILocation {
      node: Box::new(MDNodeBase::new(MetadataKind::DILocationKind, storage)),
      sub_class_data_1: implicit_code,
      sub_class_data_16: column,
      sub_class_data_32: line
    }
  }

  pub fn replace_operand_with() {}

  pub fn get_line(&self) -> u32 {
    self.sub_class_data_32
  }

  pub fn get_column(&self) -> u32 {
    self.sub_class_data_16
  }

  pub fn get_scope() {}
  pub fn get_subprogram_linkage_name() {}
  pub fn get_inlined_at() {}

  pub fn is_implicit_code(&self) -> bool {
    self.sub_class_data_1
  }

  pub fn set_implicit_code(&mut self, implicit_code: bool) {
    self.sub_class_data_1 = implicit_code;
  }

  pub fn get_file() {}
  pub fn get_filename() {}
  pub fn get_directory() {}
  pub fn get_source() {}
  pub fn get_inlined_at_scope() {}
  pub fn get_discriminator() {}
  pub fn is_pseudo_probe_discriminator() {}
  pub fn clone_with_discriminator() {}
  pub fn clone_with_base_discriminator() {}
  pub fn get_duplication_factor() {}
  pub fn get_copy_identifier() {}
  pub fn get_base_discriminator() {}
  pub fn clone_by_multiplying_duplication_factor() {}
  pub fn get_merged_location() {}
  pub fn get_merged_locations() {}
  pub fn get_masked_discriminator() {}
  pub fn get_base_discriminator_from_discriminator() {}
  pub fn encode_discriminator() {}
  pub fn decode_discriminator() {}
  pub fn get_duplication_factor_from_discriminator() {}
  pub fn get_copy_identifier_from_discriminator() {}

  pub fn get_raw_scope(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(0)
  }

  pub fn get_raw_inlined_at(&self) -> MDOperand { // TODO: -> Metadata
    if self.node.get_num_operands() == 2 {
      return self.node.get_operand(1);
    }
    MDOperand::new()
  }

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DILocationKind
  }
}

impl Metadata for DILocation {
  fn get_metadata_id(&self) -> MetadataKind {
    MetadataKind::DILocationKind
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

struct DILexicalBlock {
  line: u32,
  column: u32
}

impl DILexicalBlock {
  pub fn new() {}
  pub fn get_impl() {}

  pub fn get_line(&self) -> u32 {
    self.line
  }

  pub fn get_column(&self) -> u32 {
    self.column
  }

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DILexicalBlockKind
  }
}

struct DILexicalBlockFile {
  discriminator: u32
}

impl DILexicalBlockFile {
  pub fn new() {}
  pub fn get_impl() {}

  pub fn get_discriminator(&self) -> u32 {
    self.discriminator
  }

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DILexicalBlockFileKind
  }
}

struct DINamespace {
  export_symbols: bool
}

impl DINamespace {
  pub fn new() {}
  pub fn get_impl() {}

  pub fn get_export_symbols(&self) -> bool {
    self.export_symbols
  }

  pub fn get_scope() {}
  pub fn get_name() {}
  pub fn get_raw_scope() {}
  pub fn get_raw_name() {}

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DINamespaceKind
  }
}

// Represents a module in the programming language.
struct DIModule {
  line_no: u32,
  is_decl: bool
}

impl DIModule {
  pub fn new() {}
  pub fn get_impl() {}
  pub fn get_scope() {}
  pub fn get_name() {}
  pub fn get_configuration_macros() {}
  pub fn get_include_path() {}
  pub fn get_api_notes_file() {}

  pub fn get_line_no(&self) -> u32 {
    self.line_no
  }

  pub fn get_is_decl(&self) -> bool {
    self.is_decl
  }

  pub fn get_raw_scope() {}
  pub fn get_raw_name() {}
  pub fn get_raw_configuration_macros() {}
  pub fn get_raw_include_path() {}
  pub fn get_raw_api_notes_file() {}

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DIModuleKind
  }
}

struct DITemplateTypeParameter {
  node: DINode,
  is_default: bool
}

impl DITemplateTypeParameter {
  pub fn new(c: BlitzContext, storage: StorageType, tag: u32, is_default: bool) -> Self {
    DITemplateTypeParameter {
      node: DINode::new(c, MetadataKind::DITemplateTypeParameterKind, storage, tag),
      is_default: is_default
    }
  }

  pub fn get_impl() {}

  pub fn get_name(&self) -> StringRef {
    self.node.get_string_operand(0)
  }

  pub fn get_type() {}

  pub fn get_raw_name(&self) -> Option<MDString> {
    self.node.get_operand_as::<MDString>(0)
  }

  pub fn get_raw_type(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(1)
  }

  pub fn is_default(&self) -> bool {
    self.is_default
  }

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DITemplateTypeParameterKind
  }
}

struct DITemplateValueParameter {
  node: DINode,
  is_default: bool
}

impl DITemplateValueParameter {
  pub fn new(c: BlitzContext, storage: StorageType, tag: u32, is_default: bool) -> Self {
    DITemplateValueParameter {
      node: DINode::new(c, MetadataKind::DITemplateValueParameterKind, storage, tag),
      is_default: is_default
    }
  }

  pub fn get_impl() {}

  pub fn get_name(&self) -> StringRef {
    self.node.get_string_operand(0)
  }

  pub fn get_type() {}

  pub fn get_value(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(2)
  }

  pub fn get_raw_name(&self) -> Option<MDString> {
    self.node.get_operand_as::<MDString>(0)
  }

  pub fn get_raw_type(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(1)
  }

  pub fn is_default(&self) -> bool {
    self.is_default
  }

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DITemplateValueParameterKind
  }
}

struct DIVariable {
  node: DINode,
  line: u32,
  align_in_bits: u32
}

impl DIVariable {
  pub fn new() {}
  pub fn get_line() {}
  pub fn get_scope() {}
  pub fn get_name() {}
  pub fn get_file() {}
  pub fn get_type() {}
  pub fn get_align_in_bits() {}
  pub fn get_align_in_bytes() {}
  pub fn get_size_in_bits() {}
}

struct DIExpression {}

struct ExprOperand {}

// global variables.
struct DIGlobalVariable {
  node: DINode,
  line: u32,
  align_in_bits: u32,
  is_local_to_unit: bool,
  is_definition: bool
}

impl DIGlobalVariable {
  pub fn new(c: BlitzContext, storage: StorageType, line: u32,
    is_local_to_unit: bool, is_definition: bool, align_in_bits: u32) -> Self
  {
    DIGlobalVariable {
      node: DINode::new(c, MetadataKind::DIGlobalVariableKind, storage, 0),
      line: line, align_in_bits: align_in_bits, is_local_to_unit: is_local_to_unit,
      is_definition: is_definition
    }
  }

  pub fn get_line(&self) -> u32 {
    self.line
  }

  pub fn get_scope() {}
  pub fn get_name() {}
  pub fn get_file() {}
  pub fn get_type() {}
  pub fn get_align_in_bits() {}
  pub fn get_align_in_bytes() {}
  pub fn get_size_in_bits() {}

  pub fn is_local_to_unit(&self) -> bool {
    self.is_local_to_unit
  }

  pub fn is_definition(&self) -> bool {
    self.is_definition
  }

  pub fn get_display_name(&self) -> StringRef {
    self.node.get_string_operand(4)
  }

  pub fn get_linkage_name(&self) -> StringRef {
    self.node.get_string_operand(5)
  }

  pub fn get_static_data_member_declaration() {}
  pub fn get_annotations() {}

  pub fn get_raw_linkage_name(&self) -> Option<MDString> {
    self.node.get_operand_as::<MDString>(5)
  }

  pub fn get_raw_static_data_member_declaration(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(6)
  }

  pub fn get_raw_template_params(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(7)
  }

  pub fn get_template_params(&self) -> Option<MDTuple> {
    self.node.get_operand_as::<MDTuple>(7)
  }

  pub fn get_raw_annotations(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(8)
  }

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DIGlobalVariableKind
  }
}

struct DICommonBlock {}
impl DICommonBlock {
  pub fn new() {}
  pub fn get_impl() {}
  pub fn get_scope() {}
  pub fn get_decl() {}
  pub fn get_name() {}
  pub fn get_file() {}
  pub fn get_line_no() {}
  pub fn get_raw_scope() {}
  pub fn get_raw_decl() {}
  pub fn get_raw_name() {}
  pub fn get_raw_file() {}

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DICommonBlockKind
  }
}

struct DILocalVariable {
  node: DINode,
  line: u32,
  align_in_bits: u32,
  arg: u32,
  flags: DIFlags
}

impl DILocalVariable {
  pub fn new(c: BlitzContext, storage: StorageType, line: u32,
    arg: u32, flags: DIFlags, align_in_bits: u32) -> Self {
    DILocalVariable {
      node: DINode::new(c, MetadataKind::DILocalVariableKind, storage, 0),
      line: line, align_in_bits: align_in_bits, arg: arg, flags: flags
    }
  }

  pub fn get_line(&self) -> u32 {
    self.line
  }

  pub fn get_scope() {}

  pub fn get_name(&self) -> StringRef {
    self.node.get_string_operand(1)
  }

  pub fn get_file() {}
  pub fn get_type() {}

  pub fn get_align_in_bits(&self) -> u32 {
    self.align_in_bits
  }

  pub fn get_align_in_bytes() {}
  pub fn get_size_in_bits() {}
  
  pub fn get_signedness() {}
  pub fn get_file_name() {}
  pub fn get_directory() {}
  pub fn get_source() {}

  pub fn get_raw_scope(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(0)
  }

  pub fn get_raw_name(&self) -> Option<MDString> {
    self.node.get_operand_as::<MDString>(1)
  }

  pub fn get_raw_file(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(2)
  }

  pub fn get_raw_type(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(3)
  }

  pub fn is_parameter(&self) -> bool {
    self.arg != 0
  }

  pub fn get_arg(&self) -> u32 {
    self.arg
  }

  pub fn get_flags(&self) -> DIFlags {
    self.flags.clone()
  }

  pub fn get_annotations() {}

  pub fn is_artificial(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::Artificial as u32 != 0
  }

  pub fn is_object_pointer(&self) -> bool {
    self.flags.clone() as u32 & DIFlags::ObjectPointer as u32 != 0
  }

  pub fn is_valid_location_for_intrinsic() {}

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DILocalVariableKind
  }
}

// Label.
struct DILabel {
  node: DINode,
  line: u32
}

impl DILabel {
  pub fn new(c: BlitzContext, storage: StorageType, line: u32) -> Self {
    DILabel {
      node: DINode::new(c, MetadataKind::DILabelKind, storage, 0),
      line: line
    }
  }

  pub fn get_impl() {}
  pub fn get_scope() {}

  pub fn get_line(&self) -> u32 {
    self.line
  }

  pub fn get_name(&self) -> StringRef {
    self.node.get_string_operand(1)
  }

  pub fn get_file() {}

  pub fn get_raw_scope(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(0)
  }

  pub fn get_raw_name(&self) -> Option<MDString> {
    self.node.get_operand_as::<MDString>(1)
  }

  pub fn get_raw_file(&self) -> MDOperand { // TODO: -> Metadata
    self.node.get_operand(2)
  }

  pub fn is_valid_location_for_intrinsic() {}

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DILabelKind
  }
}

struct DIImportedEntity {}

struct DIMacroNode {
  node: Box<dyn MDNode>
}

impl DIMacroNode {
  pub fn new() {}
  pub fn get_operand_as() {}
  pub fn get_string_operand() {}
  pub fn get_canonical_md_string() {}
}

struct DIMacro {}

struct DIMacroFile {}

struct DIArgList {
  node: Box<dyn MDNode>
}

impl DIArgList {
  pub fn new() {}
  pub fn track() {}
  pub fn untrack() {}
  pub fn drop_all_references() {}
  pub fn get_args() {}

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::DIArgListKind
  }
}

// Identifies a unique instance of a variable.
struct DebugVariable {}
impl DebugVariable {
  pub fn new() {}
  pub fn get_variable() {}
  pub fn get_fragment() {}
  pub fn get_inlined_at() {}
  pub fn get_fragment_or_default() {}
  pub fn is_default_fragment() {}
}

struct DebugVariableAggregate {}