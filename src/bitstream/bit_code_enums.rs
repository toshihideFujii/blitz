#![allow(dead_code)]

// This header defines "core" bitstream enum vlaues.

enum BtstreamWrapperHeader {
  MagidField,
  VersionField,
  OffsetField,
  SizeField,
  CPUTypeField,
  HeaderSize
}

enum StandardWidths {
  BlockIdWidth,
  CodeLenWidth,
  BlockSizeWidth
}

enum FixedAbbrevIds {
  EndBlock,
  EnterSubblock,
  DefineAbbrev,
  UnabbrevRecord,
  FirstApplicationAbbrev
}

enum StandardBlockIds {
  BlockinfoBlockId,
  FirstApplicationBlocked
}

enum BlockInfoCodes {
  Setbid,
  BlockName,
  SetRecordName
}