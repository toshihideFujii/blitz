#![allow(dead_code)]

// This file contains the declarations for metadata subclasses.
// They represent the different flavors of metadata that live in Blitz.

use std::cmp::max;
use std::fmt::Debug;
use std::mem::size_of;
//use std::ops::Index;

use crate::adt::small_vector::SmallVector;
use crate::adt::string_ref::StringRef;
use super::blits_context::BlitzContext;

#[derive(Debug, Clone, PartialEq)]
pub enum MetadataKind {
  MDStringKind,
  ConstantAsMetadataKind,
  LocalAsMetadataKind,
  DistinctMDOperandPlaceholderKind,
  MDTupleKind,
  GenericDINodekind,
  DIAssignIDKind,
  DISubrangeKind,
  DIGenericSubrangeKind,
  DIEnumeratorKind,
  DIBasicTypeKind,
  DIStringTypeKind,
  DIDerivedTypeKind,
  DICompositeTypeKind,
  DISubroutineTypeKind,
  DIFileKind,
  DICompileUnitKind,
  DISubprogramKind,
  DILexicalBlockKind,
  DILexicalBlockFileKind,
  DINamespaceKind,
  DICommonBlockKind,
  DIModuleKind,
  DIImportedEntityKind,
  DILabelKind,
  DILocalVariableKind,
  DIGlobalVariableKind,
  DITemplateValueParameterKind,
  DITemplateTypeParameterKind,
}

// Active type of storage.
#[derive(Debug, Clone, PartialEq)]
pub enum StorageType {
  Uniqued,
  Distinct,
  Temporary
}

pub trait Metadata : Debug {
  fn get_metadata_id(&self) -> MetadataKind { MetadataKind::MDStringKind }
}

#[derive(Debug, PartialEq)]
pub struct MetadataAsValue {}

// API for tracking metadata references through RAUW and deletion.
pub struct MetadataTracking {}
impl MetadataTracking {
  pub fn track(_md: &Option<Box<dyn Metadata>>) -> bool { false }
  pub fn untrack(_md: &Option<Box<dyn Metadata>>) {}
  pub fn retrack(_md: &Option<Box<dyn Metadata>>, _new: &Option<Box<dyn Metadata>>) -> bool { false }
  pub fn is_replaceable(_md: &Option<Box<dyn Metadata>>) -> bool { false }

  fn track_internal() {}
}

struct ReplaceableMetadataImpl {}

#[derive(Debug)]
pub struct ValueAsMetadata {}

struct ConstantAsMetadata {}

struct LocalAsMetadata {}

// A single uniqued string.
// There are used to efficiently contain a byte sequence for metadata.
// MDString is always unnamed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MDString {}
impl MDString {
  pub fn new() -> Self {
    MDString {}
  }

  pub fn get(_c: &BlitzContext, _s: StringRef) -> MDString {
    MDString::new()
  }

  pub fn get_string(&self) -> StringRef {
    StringRef::new()
  }

  pub fn get_length(&self) -> usize {
    self.get_string().size()
  }

  // Methods for support type inquiry through isa, cast, and dyn_cast.
  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.get_metadata_id() == MetadataKind::MDStringKind
  }
}

// A collection of metadata nodes that might be associated with a 
// memory access used by the alias-analysis infrastructure.
pub struct AAMDNodes {
  // The tag for type-based alias analysis.
  tbaa: Option<Box<dyn MDNode>>,
  // The tag for type-based alias analysis (tbaa struct).
  tbaa_struct: Option<Box<dyn MDNode>>,
  // The tag for alias scope specification (used with noalias).
  scope: Option<Box<dyn MDNode>>,
  // The tag specifying the noalias scope.
  no_alias: Option<Box<dyn MDNode>>
}

impl AAMDNodes {
  pub fn new() {}
  pub fn shift_tbaa() {}
  pub fn shift_tbaa_struct() {}
  pub fn extend_to_tbaa() {}
  pub fn intersect() {}
  pub fn shift() {}
  pub fn extend_to() {}
  pub fn merge() {}
  pub fn concat() {}
}

// Tracking metadata reference owned by Metadata.
pub struct MDOperand {
  md: Option<Box<dyn Metadata>>
}

impl MDOperand {
  pub fn new() -> Self {
    MDOperand { md: None }
  }

  pub fn get(&self) -> &Option<Box<dyn Metadata>> {
    &self.md
  }

  pub fn reset(&mut self) {
    self.untrack();
    self.md = None
  }

  pub fn reset_by_md(&mut self, md: Option<Box<dyn Metadata>>,
    owner: &Option<Box<dyn Metadata>>)
  {
    self.untrack();
    self.md = md;
    self.track(owner);
  }

  pub fn track(&self, owner: &Option<Box<dyn Metadata>>) {
    if self.md.is_some() {
      if owner.is_some() {
        // TODO
      } else {
        MetadataTracking::track(&self.md);
      }
    }
  }

  pub fn untrack(&self) {
    if self.md.is_some() {
      MetadataTracking::untrack(&self.md)
    }
  }
}

struct ContextAndReplaceableUses {}

const MAX_SMALL_SIZE: usize = 15;
const NUM_OPS_FIT_IN_VECTOR: usize =
    size_of::<SmallVector<MDOperand>>() / size_of::<MDOperand>();

// The header that is coallocated with an MDNode along with
// its 'small' operands.
// It is located immediately before the main body of the node.
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
  is_resizable: bool,
  is_large: bool,
  small_size: usize,
  small_num_ops: usize,
  num_unresolved: u32
}

impl Header {
  pub fn new() -> Self {
    Header { is_resizable: true, is_large: true, small_size: 4,
      small_num_ops: 4, num_unresolved: 0 }
  }

  pub fn get_op_size(_num_ops: usize) -> usize {
    0 //size_of::<dyn MDNode>() * num_ops
  }

  // Returns the number of operands the node has space for based on
  // its allocation characteristics.
  pub fn get_small_size(num_ops: usize, is_resizable: bool, is_large: bool) -> usize {
    if is_large {
      return NUM_OPS_FIT_IN_VECTOR;
    } else {
      let mut multiplier = 0;
      if is_resizable { multiplier = 1; }
      return max(num_ops, NUM_OPS_FIT_IN_VECTOR * multiplier);
    }
  }

  pub fn get_alloc_size_static() {}

  // Only temporary and distinct nodes are resizable.
  pub fn is_resizable(storage: StorageType) -> bool {
    storage != StorageType::Uniqued
  }

  pub fn is_large(num_ops: usize) -> bool {
    num_ops > MAX_SMALL_SIZE
  }

  pub fn get_alloc_size(&self) -> usize {
    Header::get_op_size(self.small_size) + size_of::<Header>()
  }

  pub fn get_allocation() {}
  pub fn get_large_ptr() {}
  pub fn get_small_ptr() {}
  pub fn get_large(&self) -> SmallVector<MDOperand> {
    SmallVector::new()
  }
  pub fn resize_small(&self, _num_ops: usize) {}
  pub fn resize_small_to_large(&self, _num_ops: usize) {}

  pub fn resize(&self, num_ops: usize) {
    debug_assert!(self.is_resizable, "Node is not resizable.");
    if self.operands().size() == num_ops { return; }
    if self.is_large {
      //self.get_large().resize(num_ops);
    } else if num_ops <= self.small_size {
      self.resize_small(num_ops);
    } else {
      self.resize_small_to_large(num_ops);
    }
  }

  pub fn operands(&self) -> SmallVector<MDOperand> {
    if self.is_large {
      return self.get_large();
    }
    SmallVector::new()
  }

  pub fn get_num_operands(&self) -> usize {
    if !self.is_large {
      return self.small_num_ops;
    } else {
      return self.get_large().size();
    }
  }
}

// Metadata node.
// Metadata nodes can be uniqued, like constants, or distinct.
// Temporary metadata nodes (with full support for RAUW) can be
// used to delay uniquing until forward references are known.
// The basic metadata node is an MDTuple.
pub trait MDNode : Debug {
  fn get_header(&self) -> &Header;
  fn drop_all_references(&self) {}
  fn get(&self) {}
  fn get_if_exists(&self) {}
  fn get_distinct(&self) {}
  fn get_temporary(&self) {}
  fn delete_temporary(&self) {}
  fn get_context(&self) {}

  // Replace a specific operand.
  fn replace_operand_with(&self, _i: u32, _new: Box<dyn Metadata>) {}
  fn is_resolved(&self) -> bool { false }
  fn is_unsigned(&self) -> bool { false }
  fn is_distinct(&self) -> bool { false }
  fn is_temporary(&self) -> bool { false }
  fn replace_all_uses_with(&self) {}
  fn resolve_cycles(&self) {}
  fn resolve(&self) {}
  fn replace_with_permanent(&self) {}
  fn replace_with_uniqued(&self) {}
  fn replace_with_distinct(&self) {}
  fn print_tree(&self) {}
  fn dump_tree(&self) {}

  // Set an operand.
  // Sets the operand durectly, without worrying about uniquing.
  fn set_operand(&self, _i: u32, _new: Box<dyn Metadata>) {}
  fn get_num_unresolved(&self) -> u32 { 0 }
  fn set_num_unresolved(&mut self, _n: u32) {}
  fn store_distinct_in_context(&self) {}
  fn resize(&self, _num_ops: usize) {}
  fn operands(&self) -> SmallVector<MDOperand> { SmallVector::new() }
  fn get_operand(&self, _i: usize) -> MDOperand;

  // return number of MDNode operands.
  fn get_num_operands(&self) -> usize { 0 }
  fn class_of(&self) {}
  fn is_tbaa_vtable_access(&self) {}
  fn concatenate(&self) {}
  fn intersect(&self) {}
  fn get_most_generic_tbaa(&self) {}
  fn get_most_generic_fp_math(&self) {}
  fn get_most_generic_range(&self) {}
  fn get_most_generic_alias_scope(&self) {}
  fn get_most_generic_alignment_or_dereferenceable(&self) {}
}

#[derive(Debug, Clone, PartialEq)]
pub struct MDNodeBase {
  context: BlitzContext,
  id: MetadataKind,
  storage: StorageType,
  header: Header
}

impl MDNodeBase {
  pub fn new(context: BlitzContext, id: MetadataKind, storage: StorageType) -> Self {
    MDNodeBase { context: context, id: id, storage: storage, header: Header::new() }
  }
}

impl MDNode for MDNodeBase {
  fn get_header(&self) -> &Header {
    &self.header
  }

  fn drop_all_references(&self) {}
  fn get(&self) {}
  fn get_if_exists(&self) {}
  fn get_distinct(&self) {}
  fn get_temporary(&self) {}
  fn delete_temporary(&self) {}
  fn get_context(&self) {}
  fn replace_operand_with(&self, _i: u32, _new: Box<dyn Metadata>) {}

  fn is_resolved(&self) -> bool {
    !self.is_temporary() && self.get_num_unresolved() == 0
  }

  fn is_unsigned(&self) -> bool {
    self.storage == StorageType::Uniqued
  }

  fn is_distinct(&self) -> bool {
    self.storage == StorageType::Distinct
  }

  fn is_temporary(&self) -> bool {
    self.storage == StorageType::Temporary
  }

  fn replace_all_uses_with(&self) {}
  fn resolve_cycles(&self) {}
  fn resolve(&self) {}
  fn replace_with_permanent(&self) {}
  fn replace_with_uniqued(&self) {}
  fn replace_with_distinct(&self) {}
  fn print_tree(&self) {}
  fn dump_tree(&self) {}
  fn set_operand(&self, _i: u32, _md: Box<dyn Metadata>) {}

  fn get_num_unresolved(&self) -> u32 {
    self.header.num_unresolved
  }

  fn set_num_unresolved(&mut self, n: u32) {
    self.header.num_unresolved = n;
  }

  fn store_distinct_in_context(&self) {}
  fn resize(&self, _num_ops: usize) {}

  fn operands(&self) -> SmallVector<MDOperand> {
    self.header.operands()
  }

  fn get_operand(&self, i: usize) -> MDOperand {
    debug_assert!(i < self.get_num_operands(), "Out of range.");
    //let ops = &self.header.operands();
    //ops.
    MDOperand::new()
  }

  // return number of MDNode operands.
  fn get_num_operands(&self) -> usize {
    self.header.get_num_operands()
  }

  fn class_of(&self) {}
  fn is_tbaa_vtable_access(&self) {}
  fn concatenate(&self) {}
  fn intersect(&self) {}
  fn get_most_generic_tbaa(&self) {}
  fn get_most_generic_fp_math(&self) {}
  fn get_most_generic_range(&self) {}
  fn get_most_generic_alias_scope(&self) {}
  fn get_most_generic_alignment_or_dereferenceable(&self) {}
}

impl Metadata for MDNodeBase {
  fn get_metadata_id(&self) -> MetadataKind {
    MetadataKind::MDTupleKind
  }
}

// Tuple of metadata.
// This is the simple MDNode arbitrary tuple.
// Nodes are uniqued by default based on their operands.
#[derive(Debug)]
pub struct MDTuple {
  node: MDNodeBase,
  sub_class_data: u32
}

impl MDTuple {
  pub fn new(c: BlitzContext, storage: StorageType, hash: u32) -> Self {
    MDTuple {
      node: MDNodeBase::new(c, MetadataKind::MDTupleKind, storage),
      sub_class_data: hash
    }
  }

  pub fn set_hash(&mut self, hash: u32) {
    self.sub_class_data = hash
  }

  pub fn recalculate_hash(&self) {}

  pub fn get_impl(c: BlitzContext, _mds: Vec<Box<dyn Metadata>>,
    storage: StorageType, _should_create: bool) -> MDTuple
  {
    MDTuple::new(c, storage, 0)
  }

  // Get the hash, if any.
  pub fn get_hash(&self) -> u32 {
    self.sub_class_data
  }

  pub fn get(c: BlitzContext, mds: Vec<Box<dyn Metadata>>) -> MDTuple {
    MDTuple::get_impl(c, mds, StorageType::Uniqued, true)
  }

  pub fn get_if_exists(c: BlitzContext, mds: Vec<Box<dyn Metadata>>) -> MDTuple {
    MDTuple::get_impl(c, mds, StorageType::Uniqued, false)
  }

  // Return a distinct node.
  pub fn get_distinct(c: BlitzContext, mds: Vec<Box<dyn Metadata>>) -> MDTuple {
    MDTuple::get_impl(c, mds, StorageType::Distinct, true)
  }

  pub fn get_temporary() {}

  // Append an element to the tuple. This will resize the node.
  pub fn push_back(&mut self, md: Box<dyn Metadata>) {
    let num_ops = self.get_num_operands();
    self.resize(num_ops + 1);
    self.set_operand(num_ops as u32, md);
  }

  // Shrink the operands by 1.
  pub fn pop_back(&self) {
    self.resize(self.get_num_operands() - 1);
  }

  pub fn class_of(md: Box<dyn Metadata>) -> bool {
    md.as_ref().get_metadata_id() == MetadataKind::MDTupleKind
  }
}

impl MDNode for MDTuple {
  fn get_header(&self) -> &Header {
    self.node.get_header()
  }

  fn get_num_operands(&self) -> usize {
    self.node.get_num_operands()
  }

  fn set_operand(&self, i: u32, new: Box<dyn Metadata>) {
    self.node.set_operand(i, new)
  }

  fn resize(&self, num_ops: usize) {
    self.node.resize(num_ops)
  }

  fn get_operand(&self, i: usize) -> MDOperand {
    self.node.get_operand(i)
  }
}

struct AliasScopeNode {}

#[derive(Debug, PartialEq)]
pub struct NamedMDNode {}