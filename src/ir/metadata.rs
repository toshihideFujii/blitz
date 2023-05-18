#![allow(dead_code)]

// This file contains the declarations for metadata subclasses.
// They represent the different flavors of metadata that live in
// Blitz.

pub enum MetadataKind {
  MDStringKind,
  ConstantAsMetadataKind,
  LocalAsMetadataKind,
  DistinctMDOperandPlaceholderKind,
  DIAssignIDKind,
  DISubrangeKind,
  DIGenericSubrangeKind,
  DIEnumeratorKind,

  DIBasicTypeKind,
  DIStringTypeKind,
  DIDerivedTypeKind,
  DICompositeTypeKind,
  DISubroutineTypeKind,
}

// Active type of storage.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StorageType {
  Uniqued,
  Distinct,
  Temporary
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Metadata {
  sub_class_id: u32,
  storage: StorageType
}

impl Metadata {
  pub fn new(id: u32, storage: StorageType) -> Self {
    Metadata { sub_class_id: id, storage: storage }
  }

  pub fn get_metadata_id(&self) -> u32 {
    self.sub_class_id
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct MetadataAsValue {}

// API for tracking metadata references through RAUW and deletion.
pub struct MetadataTracking {}
impl MetadataTracking {
  pub fn track(_md: &Option<Metadata>) -> bool { false }
  pub fn untrack(_md: &Option<Metadata>) {}
  pub fn retrack(_md: &Option<Metadata>, _new: &Option<Metadata>) -> bool { false }
  pub fn is_replaceable(_md: &Option<Metadata>) -> bool { false }

  fn track_internal() {}
}

struct ReplaceableMetadataImpl {}

#[derive(Debug, Clone)]
pub struct ValueAsMetadata {}

struct ConstantAsMetadata {}

struct LocalAsMetadata {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MDString {}

struct AAMDNodes {}

struct MDOperand {}

struct ContextAndReplaceableUses {}

// Metadata node.
// Metadata nodes can be uniqued, like constants, or distinct.
// Temporary metadata nodes (with full support for RAUW) can be
// used to delay uniquing until forward references are known.
// The basic metadata node is an MDTuple.
#[derive(Debug, Clone, PartialEq)]
pub struct MDNode {}

struct MDTuple {}

struct AliasScopeNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedMDNode {}