#![allow(dead_code)]

// This file contains the declarations for metadata subclasses.
// They represent the different flavors of metadata that live in
// Blitz.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Metadata {}

#[derive(Debug, Clone, PartialEq)]
pub struct MetadataAsValue {}

struct MetadataTracking {}

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

#[derive(Debug, Clone, PartialEq)]
pub struct MDNode {}

struct MDTuple {}

struct AliasScopeNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedMDNode {}