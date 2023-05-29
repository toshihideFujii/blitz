#![allow(dead_code)]

// This file contains the declarations for metadata subclasses.
// They represent the different flavors of metadata that live in
// Blitz.

use std::cmp::max;
use std::mem::size_of;
//use std::ops::Index;

use crate::adt::small_vector::SmallVector;

pub enum MetadataKind {
  MDStringKind,
  ConstantAsMetadataKind,
  LocalAsMetadataKind,
  DistinctMDOperandPlaceholderKind,
  MDTupleKind,
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
}

// Active type of storage.
#[derive(Debug, PartialEq)]
pub enum StorageType {
  Uniqued,
  Distinct,
  Temporary
}

pub trait Metadata {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MDString {}

// A collection of metadata nodes that might be associated with a 
// memory access used by the alias-analysis infrastructure.
pub struct AAMDNodes {
  // The tag for type-based alias analysis.
  tbaa: Option<MDNode>,
  // The tag for type-based alias analysis (tbaa struct).
  tbaa_struct: Option<MDNode>,
  // The tag for alias scope specification (used with noalias).
  scope: Option<MDNode>,
  // The tag specifying the noalias scope.
  no_alias: Option<MDNode>
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

// The header that is coallocated with an MDNode along with
// its 'small' operands.
// It is located immediately before the main body of the node.
#[derive(Debug, PartialEq)]
pub struct Header{
  is_resizable: bool,
  is_large: bool,
  small_size: usize,
  small_num_ops: usize,
  num_unresolved: u32
}

const MAX_SMALL_SIZE: usize = 15;
const NUM_OPS_FIT_IN_VECTOR: usize =
    size_of::<SmallVector<MDOperand>>() / size_of::<MDOperand>();

impl Header {
  pub fn new() {}

  pub fn get_op_size(num_ops: usize) -> usize {
    size_of::<MDNode>() * num_ops
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
#[derive(Debug, PartialEq)]
pub struct MDNode {
  storage: StorageType,
  header: Header
}

impl MDNode {
  pub fn new() {}
  
  pub fn get_header(&self) -> &Header {
    &self.header
  }

  pub fn drop_all_references() {}
  pub fn get() {}
  pub fn get_if_exists() {}
  pub fn get_distinct() {}
  pub fn get_temporary() {}
  pub fn delete_temporary() {}
  pub fn get_context() {}
  pub fn replace_operand_with() {}

  pub fn is_resolved(&self) -> bool {
    !self.is_temporary() && self.get_num_unresolved() == 0
  }

  pub fn is_unsigned(&self) -> bool {
    self.storage == StorageType::Uniqued
  }

  pub fn is_distinct(&self) -> bool {
    self.storage == StorageType::Distinct
  }

  pub fn is_temporary(&self) -> bool {
    self.storage == StorageType::Temporary
  }

  pub fn replace_all_uses_with() {}
  pub fn resolve_cycles() {}
  pub fn resolve() {}
  pub fn replace_with_permanent() {}
  pub fn replace_with_uniqued() {}
  pub fn replace_with_distinct() {}
  pub fn print_tree() {}
  pub fn dump_tree() {}
  pub fn set_operand() {}

  pub fn get_num_unresolved(&self) -> u32 {
    self.header.num_unresolved
  }

  pub fn set_num_unresolved(&mut self, n: u32) {
    self.header.num_unresolved = n;
  }

  pub fn store_distinct_in_context() {}
  pub fn resize() {}

  pub fn operands(&self) -> SmallVector<MDOperand> {
    self.header.operands()
  }

  pub fn get_operand(&self, i: usize) /*-> &MDOperand*/ {
    debug_assert!(i < self.get_num_operands(), "Out of range.");
    //let ops = &self.header.operands();
    //ops.index(i)
  }

  // return number of MDNode operands.
  pub fn get_num_operands(&self) -> usize {
    self.header.get_num_operands()
  }

  pub fn class_of() {}
  pub fn is_tbaa_vtable_access() {}
  pub fn concatenate() {}
  pub fn intersect() {}
  pub fn get_most_generic_tbaa() {}
  pub fn get_most_generic_fp_math() {}
  pub fn get_most_generic_range() {}
  pub fn get_most_generic_alias_scope() {}
  pub fn get_most_generic_alignment_or_dereferenceable() {}
}

impl Metadata for MDNode {
  fn get_metadata_id(&self) -> MetadataKind {
    MetadataKind::MDTupleKind
  }
}

// Tuple of metadata.
// This is the simple MDNode arbitrary tuple.
struct MDTuple {}
impl MDTuple {
  pub fn new() {}
  pub fn set_hash() {}
  pub fn recalculate_hash() {}
  pub fn get_hash() {}
  pub fn get() {}
  pub fn get_if_exists() {}
  pub fn get_distinct() {}
  pub fn get_temporary() {}
  pub fn push_back() {}
  pub fn class_of() {}
}

struct AliasScopeNode {}

#[derive(Debug, PartialEq)]
pub struct NamedMDNode {}