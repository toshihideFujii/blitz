#![allow(dead_code)]

struct Operator {}

pub enum OverflowBinOpWrap {
  AnyWrap = 0,
  NoUnsignedWrap = (1 << 0),
  NoSignedWrap = (1 << 1)
}

struct OverflowingBinaryOperator {}

pub enum PossiblyExactOp {
  IsExact = (1 << 0)
}

struct PossiblyExactOperator {}

struct FPMathOperator {}

struct AddOperator {}

struct SubOperator {}

struct MulOperator {}

struct ShlOperator {}

struct SDivOperator {}

struct UDivOperator {}

struct AShrOperator {}

struct LShrOperator {}

struct ZExtOperator {}

struct GEPOperator {}

struct PtrToIntOperator {}

struct BitCastOperator {}

struct AddrSpaceCastOperator {}