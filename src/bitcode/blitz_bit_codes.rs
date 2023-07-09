#![allow(dead_code)]

// This header defines butcode enum values for Blitz IR
// bitcode files.
// This enum values defined in this file should be considered
// permanent. If new features are added, they should have
// values added at the end of the respective lists.

pub enum BlockIDs {
  ModuleBlockId,
  ParameterBlockId,
  ParameterGroupBlockId,
  ConstantsBlockId,
  FunctionBlockId,
  IdentificationBlockId,
  ValueSymtabBlockId,
  MetadataBlockId,
  MetadataAttachmentId,
  TypeBlockIdNew,
  UselistBlockId,
  ModuleSymtabBlockId,
  GlobalValSummaryBlockId,
  OperandBundleTagsBlockId,
  MetadataKindBlockId,
  FullLtoGlobalValSummaryBlockId,
  SymtabBlockId,
  SyncScopeNamesBlockId
}