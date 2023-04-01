#![allow(dead_code)]

// This file provides declarations for things used in the
// various types of remark containers.

enum BitstreamRemarkContainerType {
  SeparateRemarksMeta,
  SeparateRemarksFile,
  Standalone
}

enum BlockIDs {
  MetaBlockId,
  RemarkBlockId
}

enum RecordIDs {
  MetaContainerInfo,
  MetaRemarkVersion,
  MetaStrtab,
  MetaExternalFile,
  RemarkHeader,
  RemarkDebugLoc,
  RemarkHotness,
  RemarkArgWithDebugLoc,
  RemarkArgWithoutDebugLoc
}