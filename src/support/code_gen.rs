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

#[derive(Debug, Clone, PartialEq)]
pub enum UWTableKind {
  None = 0, // No unwind table requested
  Sync = 1, // 'Synchronous' unwind tables
  Async = 2 // 'Asynchronous' unwind tables (instr precise)
}

enum FunctionReturnThunksKind {
  Keep = 0,
  Extern = 1,
  Invalid = 2
}