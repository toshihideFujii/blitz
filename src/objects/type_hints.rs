// Type hints for an binary operation.
pub enum BinaryOperationHint {
  None,
  SignedSmall,
  SignedSmallInputs,
  Number,
  NumberOrOddball,
  String,
  BigInt,
  Any,
}

// The hints for an compare operation.
pub enum CompareOperationHint {
  None,
  SignedSmall,
  Number,
  NumberOrBoolean,
  NumberOrOddball,
  InternalizedString,
  String,
  Symbol,
  BigInt,
  Receiver,
  ReceiverOrNullOrUndefined,
  Any,
}

// Type hints for for..in statements.
pub enum ForInHint {
  None,
  EnumCacheKeysAndIndices,
  EnumCacheKeys,
  Any,
}

pub enum StringAddFlags {
  StringAddCheckNone,
  StringAddConvertLeft,
  StringAddConvertRight,
}
