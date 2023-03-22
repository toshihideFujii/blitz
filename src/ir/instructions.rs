#![allow(dead_code)]


struct LoadInst {}

struct StoreInst {}

// An instruction for ordering other memory operations.
struct FenceInst {}

struct AtomicCmpXchgInst {}

struct AtomicRMWInst {}

// An instruction for type-safe pointer arithmetic to access elements
// of arrays and structs.
struct GetElementPtrInst {}

struct ICmpInst {}

struct FCmpInst {}

// This class represents a function call, abstracting a target machine's
// calling convention.
struct CallInst {}

struct SelectInst {}

struct VAArgInst {}

struct ExtractElementInst {}

// This instruction inserts a single (scalar) element into a VectorType
// value.
struct InsertElementInst {}

struct ShuffleVectorInst {}

struct ExtractValueInst {}

struct InsertValueInst {}

struct PhiNode {}

struct LandingPadInst {}

struct ReturnInst {}

// Conditional or unconditional branch instruction.
struct BranchInst {}

struct SwitchInst {}

// Indirect branch instruction.
struct IndirectBrInst {}

struct InvokeInst {}

struct CallBrInst {}

// Resume the propagation of an exception.
struct ResumeInst {}

struct CatchSwitchInst {}

struct CleanuppadInst {}

struct CatchPadInst {}

struct CatchReturnInst {}

struct CleanupReturnInst {}

struct UnreachableInst {}

struct TruncInst {}

// This class represents zero extension of integer types.
struct ZExtInst {}

// This class represents sign extension of integer types.
struct SExtInst {}

struct FPTruncInst {}

struct FPExtInst {}

struct UItoFPInst {}

struct SIToFPInst {}

struct FPToUIInst {}

struct FPToSIInst {}

struct IntToPtrInst {}

struct PtrToIntInst {}

struct BitCastInst {}

struct AddrSpaceCastInst {}

struct FreezeInst {}