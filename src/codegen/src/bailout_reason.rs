
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BailoutReason {
  NoReason,
  BailedOutDueToDependencyChange,
  ConcurrentMapDeprecation,
  CodeGenerationfailed,
  FunctionBeingDebugged,
  GraphBuildingFailed,
  FunctionTooBig,
  TooManyArguments,
  LiveEdit,
  NativeFunctionLiteral,
  OptimizationDisabled,
  HigherTierAvailable,
  DetachedNativeContext,
  NeverOptimize,
}

impl BailoutReason {
  pub fn get_bailout_reason(&self) -> &str {
    match self {
      BailoutReason::NoReason => "no reason",
      BailoutReason::BailedOutDueToDependencyChange => "Bailed out due to dependency change",
      BailoutReason::ConcurrentMapDeprecation => "Maps became deprecated during optimization",
      BailoutReason::CodeGenerationfailed => "Code generation failed",
      BailoutReason::FunctionBeingDebugged => "Function is being debugged",
      BailoutReason::GraphBuildingFailed => "Optimized graph construction failed",
      BailoutReason::FunctionTooBig => "Function is too big to be optimized",
      BailoutReason::TooManyArguments => "Function contains a call with too many arguments",
      BailoutReason::LiveEdit => "LiveEdit",
      BailoutReason::NativeFunctionLiteral => "Native function literal",
      BailoutReason::OptimizationDisabled => "Optimization disabled",
      BailoutReason::HigherTierAvailable => "A higher tier is already available",
      BailoutReason::DetachedNativeContext => "The native context is detached",
      BailoutReason::NeverOptimize => "Optimization is always disabled",
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AbortReason {
  NoReason,
  A32BitValueInRegisterIsNotZeroExtended,
  SignedBitOfSmiIsNotZero,
  APICallReturnedInvalidObject,
  AccumulatorClobbered,
  AllocatingNonEmptyPackedArray,
  AllocationIsNotDoubleAligned,
  ExpectedOptimizationSentinel,
  ExpectedUndefinedOrCell,
  ExpectedFeedbackCell,
  ExpectedFeedbackVector,
  ExpectedBaselineData,
  Float64IsNotAInt32,
  FunctionDataShouldBeBytecodeArrayOnInterpreterEntry,
  InputStringTooLong,
  InputDoesNotFitSmi,
  InvalidBytecode,
  InvalidBytecodeAdvance,
  InvalidHandleScopeLevel,
  InvalidJumpTableIndex,
  InvalidParametersAndRegistersInGenerator,
  MissingBytecodeArray,
  ObjectNotTagged,
  ObjectTagged,
  OffsetOutOfRange,
  OperandIsASmi,
  OperandIsASmiAndNotABoundFunction,
  OperandIsASmiAndNotAConstructor,
  OperandIsASmiAndNotAFunction,
  OperandIsASmiAndNotAGeneratorObject,
  OperandIsCleared,
  OperandIsNotABoundFunction,
  OperandIsNotAConstructor,
  OperandIsNotAFixedArray,
  OperandIsNotAFunction,
  OperandIsNotACallableFunction,
  OperandIsNotAGeneratorObject,
  OperandIsNotACode,
  OperandIsNotAMap,
  OperandIsNotASmi,
  MaglevOsrTodo,
  PromiseAlreadySettled,
  ReceivedInvalidReturnAddress,
  RegisterDidNotMatchExpectedRoot,
  ReturnAddressNotFoundInFrame,
  ShouldNotDirectlyEnterOsrFunction,
  StackAccessBelowStackPointer,
  OsrUnexpectedStackSize,
  StackFrameTypesMustMatch,
  Uint32IsNotAInt32,
  UnalignedCellInWriteBarrier,
  UnexpectedAdditionalPopValue,
  UnexpectedElementsKindInArrayConstructor,
  UnexpectedFPCRMode,
  UnexpectedFunctionIDForInvokeIntrinsic,
  UnexpectedInitialMapForArrayFunction,
  UnexpectedLevelAfterReturnFromApiCall,
  UnexpectedNegativeValue,
  UnexpectedReturnFromFrameDropper,
  UnexpectedReturnFromThrow,
  UnexpectedReturnFromWasmTrap,
  UnexpectedStackPointer,
  UnexpectedValue,
  UnsupportedModuleOperation,
  UnsupportedNonPrimitiveCompare,
  WrongAddressOrValuePassedToRecordWrite,
  WrongArgumentCountForInvokeIntrinsic,
  WrongFunctionCodeStart,
  WrongFunctionContext,
  UnexpectedThreadInWasmSet,
  UnexpectedThreadInWasmUnset,
  InvalidReceiver,
  UnexpectedInstanceType,
  TurboshaftTypeAssertionFailed,
}
