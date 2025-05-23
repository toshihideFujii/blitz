#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HloOpcode {
  Abs,
  Add,
  AddDependency,
  AfterAll,
  AllGather,
  AllGatherDone,
  AllGatherStart,
  AllReduce,
  AllReduceDone,
  AllReduceStart,
  AllToAll,
  And,
  AsyncDone,
  AsyncStart,
  AsyncUpdate,
  Atan2,
  BatchNormGrad,
  BatchNormInference,
  BatchNormTraining,
  Bitcast,
  BitcastConvert,
  Broadcast,
  Call,
  Cbrt,
  Ceil,
  Cholsky,
  Clamp,
  Clz,
  CollectivePermute,
  CollectivePermuteDone,
  CollectivePermuteStart,
  Compare,
  Complex,
  Concatenate,
  Conditional,
  Constant,
  Convert,
  Convolution,
  Copy,
  CopyDone,
  CopyStart,
  Cos,
  CustomCall,
  Divide,
  Domain,
  Dot,
  DynamicReshape,
  DynamicSlice,
  DynamicUpdateSlice,
  Erf,
  Exp,
  Expm1,
  Fft,
  Floor,
  Fusion,
  Gather,
  GetDimensionSize,
  GetTupleElement,
  Imag,
  Infeed,
  Iota,
  IsFinite,
  Log,
  Log1p,
  Logistic,
  Map,
  Maximum,
  Minimum,
  Multiply,
  Negate,
  Not,
  OptimizationBarrier,
  Or,
  Outfeed,
  Pad,
  Parameter,
  PartitionId,
  PopulationCount,
  Power,
  RaggedAllToAll,
  Real,
  Recv,
  RecvDone,
  Reduce,
  ReducePrecision,
  ReduceScatter,
  ReduceWindow,
  Remainder,
  ReplicaId,
  Reshape,
  Reverse,
  Rng,
  RngBitGenerator,
  RngGetAndUpdateState,
  RoundNearestAfz,
  RoundNearestEven,
  Rsqrt,
  Scatter,
  Select,
  SelectAndScatter,
  Send,
  SendDone,
  SetDimensionSize,
  ShiftLeft,
  ShiftRightArithmetic,
  ShiftRightLogical,
  Sign,
  Sin,
  Slice,
  Sort,
  Sqrt,
  StochasticConvert,
  Subtract,
  Tan,
  Tanh,
  TopK,
  Transpose,
  TriangularSolve,
  Tuple,
  While,
  Xor,
}

// Returns a string representation of the opcode.
pub fn hlo_opcode_string(opcode: &HloOpcode) -> String {
  match opcode {
    HloOpcode::Abs => "abs".to_string(),
    HloOpcode::Add => "add".to_string(),
    HloOpcode::AddDependency => "add-dependency".to_string(),
    HloOpcode::AfterAll => "after-all".to_string(),
    HloOpcode::AllGather => "all-gather".to_string(),
    HloOpcode::AllGatherDone => "all-gather-done".to_string(),
    HloOpcode::AllGatherStart => "all-gather-start".to_string(),
    HloOpcode::AllReduce => "all-reduce".to_string(),
    HloOpcode::AllReduceDone => "all-reduce-done".to_string(),
    HloOpcode::AllReduceStart => "all-reduce-start".to_string(),
    HloOpcode::AllToAll => "all-to-all".to_string(),
    HloOpcode::And => "and".to_string(),
    HloOpcode::AsyncDone => "async-done".to_string(),
    HloOpcode::AsyncStart => "async-start".to_string(),
    HloOpcode::AsyncUpdate => "async-update".to_string(),
    HloOpcode::Atan2 => "atan2".to_string(),
    HloOpcode::BatchNormGrad => "batch-norm-grad".to_string(),
    HloOpcode::BatchNormInference => "batch-norm-inference".to_string(),
    HloOpcode::BatchNormTraining => "batch-norm-training".to_string(),
    HloOpcode::Bitcast => "bitcast".to_string(),
    HloOpcode::BitcastConvert => "bitcast-convert".to_string(),
    HloOpcode::Broadcast => "broadcast".to_string(),
    HloOpcode::Call => "call".to_string(),
    HloOpcode::Cbrt => "cbrt".to_string(),
    HloOpcode::Ceil => "ceil".to_string(),
    HloOpcode::Cholsky => "cholsky".to_string(),
    HloOpcode::Clamp => "clamp".to_string(),
    HloOpcode::Clz => "count-leading-zeros".to_string(),
    HloOpcode::CollectivePermute => "collective-permute".to_string(),
    HloOpcode::CollectivePermuteDone => "collective-permute-done".to_string(),
    HloOpcode::CollectivePermuteStart => "collective-permute-start".to_string(),
    HloOpcode::Compare => "compare".to_string(),
    HloOpcode::Complex => "complex".to_string(),
    HloOpcode::Concatenate => "concatenate".to_string(),
    HloOpcode::Conditional => "conditional".to_string(),
    HloOpcode::Constant => "constant".to_string(),
    HloOpcode::Convert => "convert".to_string(),
    HloOpcode::Convolution => "convolution".to_string(),
    HloOpcode::Copy => "copy".to_string(),
    HloOpcode::CopyDone => "copy-done".to_string(),
    HloOpcode::CopyStart => "copy-start".to_string(),
    HloOpcode::Cos => "cos".to_string(),
    HloOpcode::CustomCall => "custom-call".to_string(),
    HloOpcode::Divide => "divide".to_string(),
    HloOpcode::Domain => "domain".to_string(),
    HloOpcode::Dot => "dot".to_string(),
    HloOpcode::DynamicReshape => "dynamic-reshape".to_string(),
    HloOpcode::DynamicSlice => "dynamic-slice".to_string(),
    HloOpcode::DynamicUpdateSlice => "dynamic-update-slice".to_string(),
    HloOpcode::Erf => "erf".to_string(),
    HloOpcode::Exp => "exponential".to_string(),
    HloOpcode::Expm1 => "exponential-minus-one".to_string(),
    HloOpcode::Fft => "fft".to_string(),
    HloOpcode::Floor => "floor".to_string(),
    HloOpcode::Fusion => "fusion".to_string(),
    HloOpcode::Gather => "gather".to_string(),
    HloOpcode::GetDimensionSize => "get-dimension-size".to_string(),
    HloOpcode::GetTupleElement => "get-tuple-element".to_string(),
    HloOpcode::Imag => "imag".to_string(),
    HloOpcode::Infeed => "infeed".to_string(),
    HloOpcode::Iota => "iota".to_string(),
    HloOpcode::IsFinite => "is-finite".to_string(),
    HloOpcode::Log => "log".to_string(),
    HloOpcode::Log1p => "log-plus-one".to_string(),
    HloOpcode::Logistic => "logistic".to_string(),
    HloOpcode::Map => "map".to_string(),
    HloOpcode::Maximum => "maximum".to_string(),
    HloOpcode::Minimum => "minimum".to_string(),
    HloOpcode::Multiply => "multiply".to_string(),
    HloOpcode::Negate => "negate".to_string(),
    HloOpcode::Not => "not".to_string(),
    HloOpcode::OptimizationBarrier => "opt-barrier".to_string(),
    HloOpcode::Or => "or".to_string(),
    HloOpcode::Outfeed => "outfeed".to_string(),
    HloOpcode::Pad => "pad".to_string(),
    HloOpcode::Parameter => "parameter".to_string(),
    HloOpcode::PartitionId => "partition-id".to_string(),
    HloOpcode::PopulationCount => "pocnt".to_string(),
    HloOpcode::Power => "power".to_string(),
    HloOpcode::RaggedAllToAll => "ragged-all-to-all".to_string(),
    HloOpcode::Real => "real".to_string(),
    HloOpcode::Recv => "recv".to_string(),
    HloOpcode::RecvDone => "recv-done".to_string(),
    HloOpcode::Reduce => "reduce".to_string(),
    HloOpcode::ReducePrecision => "reduce-precision".to_string(),
    HloOpcode::ReduceScatter => "reduce-scatter".to_string(),
    HloOpcode::ReduceWindow => "reduce-window".to_string(),
    HloOpcode::Remainder => "remainder".to_string(),
    HloOpcode::ReplicaId => "replica-id".to_string(),
    HloOpcode::Reshape => "reshape".to_string(),
    HloOpcode::Reverse => "reverse".to_string(),
    HloOpcode::Rng => "rng".to_string(),
    HloOpcode::RngBitGenerator => "rng-bit-generator".to_string(),
    HloOpcode::RngGetAndUpdateState => "rng-get-and-update-state".to_string(),
    HloOpcode::RoundNearestAfz => "round-nearest-afz".to_string(),
    HloOpcode::RoundNearestEven => "round-nearest-even".to_string(),
    HloOpcode::Rsqrt => "rsqrt".to_string(),
    HloOpcode::Scatter => "scatter".to_string(),
    HloOpcode::Select => "select".to_string(),
    HloOpcode::SelectAndScatter => "select-and-scatter".to_string(),
    HloOpcode::Send => "send".to_string(),
    HloOpcode::SendDone => "send-done".to_string(),
    HloOpcode::SetDimensionSize => "set-dimension-size".to_string(),
    HloOpcode::ShiftLeft => "shift-left".to_string(),
    HloOpcode::ShiftRightArithmetic => "shift-right-arithmetic".to_string(),
    HloOpcode::ShiftRightLogical => "shift-right-logical".to_string(),
    HloOpcode::Sign => "sign".to_string(),
    HloOpcode::Sin => "sin".to_string(),
    HloOpcode::Slice => "slice".to_string(),
    HloOpcode::Sort => "sort".to_string(),
    HloOpcode::Sqrt => "sqrt".to_string(),
    HloOpcode::StochasticConvert => "stochastic-convert".to_string(),
    HloOpcode::Subtract => "subtract".to_string(),
    HloOpcode::Tan => "tan".to_string(),
    HloOpcode::Tanh => "tanh".to_string(),
    HloOpcode::TopK => "topk".to_string(),
    HloOpcode::Transpose => "transpose".to_string(),
    HloOpcode::TriangularSolve => "triangular-solve".to_string(),
    HloOpcode::Tuple => "tuple".to_string(),
    HloOpcode::While => "while".to_string(),
    HloOpcode::Xor => "xor".to_string(),
  }
}

// Retrieves the opcode enum by name if the opcode exists.
pub fn string_to_hlo_opcode(name: &String) -> Result<HloOpcode, String> {
  if name == "abs" {
    return Ok(HloOpcode::Abs);
  } else if name == "add" {
    return Ok(HloOpcode::Add);
  } else if name == "add-dependency" {
    return Ok(HloOpcode::AddDependency);
  } else if name == "after-all" {
    return Ok(HloOpcode::AfterAll);
  } else if name == "all-gather" {
    return Ok(HloOpcode::AllGather);
  } else if name == "all-gather-done" {
    return Ok(HloOpcode::AllGatherDone);
  } else if name == "all-gather-start" {
    return Ok(HloOpcode::AllGatherStart);
  } else if name == "all-reduce" {
    return Ok(HloOpcode::AllReduce);
  } else if name == "all-reduce-done" {
    return Ok(HloOpcode::AllReduceDone);
  } else if name == "all-reduce-start" {
    return Ok(HloOpcode::AllReduceStart);
  } else if name == "all-to-all" {
    return Ok(HloOpcode::AllToAll);
  } else if name == "and" {
    return Ok(HloOpcode::And);
  } else if name == "async-done" {
    return Ok(HloOpcode::AsyncDone);
  } else if name == "async-start" {
    return Ok(HloOpcode::AsyncStart);
  } else if name == "async-update" {
    return Ok(HloOpcode::AsyncUpdate);
  } else if name == "atan2" {
    return Ok(HloOpcode::Atan2);
  } else if name == "batch-norm-grad" {
    return Ok(HloOpcode::BatchNormGrad);
  } else if name == "batch-norm-inference" {
    return Ok(HloOpcode::BatchNormInference);
  } else if name == "batch-norm-training" {
    return Ok(HloOpcode::BatchNormTraining);
  } else if name == "bitcast" {
    return Ok(HloOpcode::Bitcast);
  } else if name == "bitcast-convert" {
    return Ok(HloOpcode::BitcastConvert);
  } else if name == "broadcast" {
    return Ok(HloOpcode::Broadcast);
  } else if name == "call" {
    return Ok(HloOpcode::Call);
  } else if name == "cbrt" {
    return Ok(HloOpcode::Cbrt);
  } else if name == "ceil" {
    return Ok(HloOpcode::Ceil);
  } else if name == "cholsky" {
    return Ok(HloOpcode::Cholsky);
  } else if name == "clamp" {
    return Ok(HloOpcode::Clamp);
  } else if name == "count-leading-zeros" {
    return Ok(HloOpcode::Clz);
  } else if name == "collective-permute" {
    return Ok(HloOpcode::CollectivePermute);
  } else if name == "collective-permute-done" {
    return Ok(HloOpcode::CollectivePermuteDone);
  } else if name == "collective-permute-start" {
    return Ok(HloOpcode::CollectivePermuteStart);
  } else if name == "compare" {
    return Ok(HloOpcode::Compare);
  } else if name == "complex" {
    return Ok(HloOpcode::Complex);
  } else if name == "concatenate" {
    return Ok(HloOpcode::Concatenate);
  } else if name == "conditional" {
    return Ok(HloOpcode::Conditional);
  } else if name == "constant" {
    return Ok(HloOpcode::Constant);
  } else if name == "convert" {
    return Ok(HloOpcode::Convert);
  } else if name == "convolution" {
    return Ok(HloOpcode::Convolution);
  } else if name == "copy" {
    return Ok(HloOpcode::Copy);
  } else if name == "copy-done" {
    return Ok(HloOpcode::CopyDone);
  } else if name == "copy-start" {
    return Ok(HloOpcode::CopyStart);
  } else if name == "cos" {
    return Ok(HloOpcode::Cos);
  } else if name == "custom-call" {
    return Ok(HloOpcode::CustomCall);
  } else if name == "divide" {
    return Ok(HloOpcode::Divide);
  } else if name == "domain" {
    return Ok(HloOpcode::Domain);
  } else if name == "dot" {
    return Ok(HloOpcode::Dot);
  } else if name == "dynamic-reshape" {
    return Ok(HloOpcode::DynamicReshape);
  } else if name == "dynamic-slice" {
    return Ok(HloOpcode::DynamicSlice);
  } else if name == "dynamic-update-slice" {
    return Ok(HloOpcode::DynamicUpdateSlice);
  } else if name == "erf" {
    return Ok(HloOpcode::Erf);
  } else if name == "exponential" {
    return Ok(HloOpcode::Exp);
  } else if name == "exponential-minus-one" {
    return Ok(HloOpcode::Expm1);
  } else if name == "fft" {
    return Ok(HloOpcode::Fft);
  } else if name == "floor" {
    return Ok(HloOpcode::Floor);
  } else if name == "fusion" {
    return Ok(HloOpcode::Fusion);
  } else if name == "gather" {
    return Ok(HloOpcode::Gather);
  } else if name == "get-dimension-size" {
    return Ok(HloOpcode::GetDimensionSize);
  } else if name == "get-tuple-element" {
    return Ok(HloOpcode::GetTupleElement);
  } else if name == "imag" {
    return Ok(HloOpcode::Imag);
  } else if name == "infeed" {
    return Ok(HloOpcode::Infeed);
  } else if name == "iota" {
    return Ok(HloOpcode::Iota);
  } else if name == "is-finite" {
    return Ok(HloOpcode::IsFinite);
  } else if name == "log" {
    return Ok(HloOpcode::Log);
  } else if name == "log-plus-one" {
    return Ok(HloOpcode::Log1p);
  } else if name == "logistic" {
    return Ok(HloOpcode::Logistic);
  } else if name == "map" {
    return Ok(HloOpcode::Map);
  } else if name == "maximum" {
    return Ok(HloOpcode::Maximum);
  } else if name == "minimum" {
    return Ok(HloOpcode::Minimum);
  } else if name == "multiply" {
    return Ok(HloOpcode::Multiply);
  } else if name == "negate" {
    return Ok(HloOpcode::Negate);
  } else if name == "not" {
    return Ok(HloOpcode::Not);
  } else if name == "opt-barrier" {
    return Ok(HloOpcode::OptimizationBarrier);
  } else if name == "or" {
    return Ok(HloOpcode::Or);
  } else if name == "outfeed" {
    return Ok(HloOpcode::Outfeed);
  } else if name == "pad" {
    return Ok(HloOpcode::Pad);
  } else if name == "parameter" {
    return Ok(HloOpcode::Parameter);
  } else if name == "partition-id" {
    return Ok(HloOpcode::PartitionId);
  } else if name == "pocnt" {
    return Ok(HloOpcode::PopulationCount);
  } else if name == "power" {
    return Ok(HloOpcode::Power);
  } else if name == "ragged-all-to-all" {
    return Ok(HloOpcode::RaggedAllToAll);
  } else if name == "real" {
    return Ok(HloOpcode::Real);
  } else if name == "recv" {
    return Ok(HloOpcode::Recv);
  } else if name == "recv-done" {
    return Ok(HloOpcode::RecvDone);
  } else if name == "reduce" {
    return Ok(HloOpcode::Reduce);
  } else if name == "reduce-precision" {
    return Ok(HloOpcode::ReducePrecision);
  } else if name == "reduce-scatter" {
    return Ok(HloOpcode::ReduceScatter);
  } else if name == "reduce-window" {
    return Ok(HloOpcode::ReduceWindow);
  } else if name == "remainder" {
    return Ok(HloOpcode::Remainder);
  } else if name == "replica-id" {
    return Ok(HloOpcode::ReplicaId);
  } else if name == "reshape" {
    return Ok(HloOpcode::Reshape);
  } else if name == "reverse" {
    return Ok(HloOpcode::Reverse);
  } else if name == "rng" {
    return Ok(HloOpcode::Rng);
  } else if name == "rng-bit-generator" {
    return Ok(HloOpcode::RngBitGenerator);
  } else if name == "rng-get-and-update-state" {
    return Ok(HloOpcode::RngGetAndUpdateState);
  } else if name == "round-nearest-afz" {
    return Ok(HloOpcode::RoundNearestAfz);
  } else if name == "round-nearest-even" {
    return Ok(HloOpcode::RoundNearestEven);
  } else if name == "rsqrt" {
    return Ok(HloOpcode::Rsqrt);
  } else if name == "scatter" {
    return Ok(HloOpcode::Scatter);
  } else if name == "select" {
    return Ok(HloOpcode::Select);
  } else if name == "select-and-scatter" {
    return Ok(HloOpcode::SelectAndScatter);
  } else if name == "send" {
    return Ok(HloOpcode::Send);
  } else if name == "send-done" {
    return Ok(HloOpcode::SendDone);
  } else if name == "set-dimension-size" {
    return Ok(HloOpcode::SetDimensionSize);
  } else if name == "shift-left" {
    return Ok(HloOpcode::ShiftLeft);
  } else if name == "shift-right-arithmetic" {
    return Ok(HloOpcode::ShiftRightArithmetic);
  } else if name == "shift-right-logical" {
    return Ok(HloOpcode::ShiftRightLogical);
  } else if name == "sign" {
    return Ok(HloOpcode::Sign);
  } else if name == "sin" {
    return Ok(HloOpcode::Sin);
  } else if name == "slice" {
    return Ok(HloOpcode::Slice);
  } else if name == "sort" {
    return Ok(HloOpcode::Sort);
  } else if name == "stochastic-convert" {
    return Ok(HloOpcode::StochasticConvert);
  } else if name == "subtract" {
    return Ok(HloOpcode::Subtract);
  } else if name == "tan" {
    return Ok(HloOpcode::Tan);
  } else if name == "tanh" {
    return Ok(HloOpcode::Tanh);
  } else if name == "topk" {
    return Ok(HloOpcode::TopK);
  } else if name == "transpose" {
    return Ok(HloOpcode::Transpose);
  } else if name == "triangular-solve" {
    return Ok(HloOpcode::TriangularSolve);
  } else if name == "tuple" {
    return Ok(HloOpcode::Tuple);
  } else if name == "while" {
    return Ok(HloOpcode::While);
  } else if name == "xor" {
    return Ok(HloOpcode::Xor);
  }

  let mut err_msg = "Unknown opcocde: ".to_string();
  err_msg.push_str(&name);
  Err(err_msg)
}

pub fn hlo_opcode_is_comparison(opcode: &HloOpcode) -> bool {
  *opcode == HloOpcode::Compare
}

pub fn hlo_opcode_is_variadic(opcode: &HloOpcode) -> bool {
  match opcode {
    HloOpcode::AfterAll => true,
    HloOpcode::AllGather => true,
    HloOpcode::AllGatherStart => true,
    HloOpcode::AllReduce => true,
    HloOpcode::AllReduceStart => true,
    HloOpcode::AllToAll => true,
    HloOpcode::AsyncStart => true,
    HloOpcode::Call => true,
    HloOpcode::CollectivePermute => true,
    HloOpcode::CollectivePermuteStart => true,
    HloOpcode::Concatenate => true,
    HloOpcode::Conditional => true,
    HloOpcode::CustomCall => true,
    HloOpcode::DynamicReshape => true,
    HloOpcode::DynamicSlice => true,
    HloOpcode::DynamicUpdateSlice => true,
    HloOpcode::Fusion => true,
    HloOpcode::Map => true,
    HloOpcode::Reduce => true,
    HloOpcode::ReduceScatter => true,
    HloOpcode::ReduceWindow => true,
    HloOpcode::Rng => true,
    HloOpcode::Scatter => true,
    HloOpcode::Sort => true,
    HloOpcode::Tuple => true,
    _ => false,
  }
}

// Returns the arity of opcode or nullopt for variadic opcodes.
pub fn hlo_opcode_arity(_opcode: &HloOpcode) -> Option<i8> {
  unimplemented!()
}

// Returns true for kAsyncStart, kAsyncUpdate, kAsyncDone.
pub fn hlo_opcode_is_async(opcode: &HloOpcode) -> bool {
  *opcode == HloOpcode::AsyncStart ||
  *opcode == HloOpcode::AsyncUpdate ||
  *opcode == HloOpcode::AsyncDone
}

// True if the op takes two arguments and order doesn't matter.
pub fn hlo_opcode_is_binary_commutative(opcode: &HloOpcode) -> bool {
  match opcode {
    HloOpcode::Add => true,
    HloOpcode::Multiply => true,
    HloOpcode::Maximum => true,
    HloOpcode::And => true,
    HloOpcode::Or => true,
    HloOpcode::Xor => true,
    _ => false,
  }
}

// Returns the number of HloOpcode values.
pub fn hlo_opcode_count() -> usize {
  118
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_usage() {
    assert_eq!(hlo_opcode_string(&HloOpcode::Multiply), "multiply".to_string());
  }
}