#![allow(dead_code)]

/*
This file define some types which define code generation concepts.
For example, relocation model.
*/

enum RelocModel {
  Static,
  Pic,
  DynamicNoPic,
  Ropi,
  Rwpi,
  RopiRwpi
}

enum PicLevel {
  NotPic,
  SmallPic,
  BigPic
}

enum PieLevel {
  Default,
  Small,
  Large
}

enum TlsModel {
  GeneralDynamic,
  LocalDynamic,
  InitialExec,
  LocalExec
}

// Code generation optimization level.
enum CodeGenOptLevel {
  None = 0,
  Less = 1,
  Default = 2,
  Aggressive = 3
}

enum CodeGenFileType {
  AssemblyFile,
  ObjectFile,
  Null
}

enum FramePointerKind {
  None,
  NonLeaf,
  All
}

const ONLY_USED: u32 = 1 << 1;
const ONLY_GPR: u32 = 1 << 2;
const ONLY_ARG: u32 = 1 << 3;

enum ZeroCallUsedRegs {
  Skip = 1 << 0,
  UsedGprArg,
  UsedGpr,
  UsedArg,
  Used,
  AllGprArg,
  AllGpr,
  AllArg,
  All
}

enum UWTableKind {
  None = 0,
  Sync = 1,
  Async = 2
  //Default = 3
}

enum FunctionReturnThunksKind {
  Keep = 0,
  Extern = 1,
  Invalid = 2
}