#![allow(dead_code)]

enum InlineCostFeatureIndex {
  SroaSavings,
  SroaLosses,
  LoadElimination,
  CallPenalty,
  CallArgumentSetup,
  LoadRelativeIntrinsic,
  LoweredCallArgSetup,
  IndirectCallPenalty,
  JumpTablePenalty,
  CaseClusterPenalty,
  SwitchPenalty,
  UnsimplifiedCommonInstructions,
  NumLoops,
  DeadBlocks,
  SimplifiedInstructions,
  ConstantArgs,
  ConstantOffsetPtrArgs,
  CallSiteCost,
  ColdCcPenalty,
  LastCallToStaticBonus,
  IsMultipleBlocks,
  NestedInlines,
  NestedInlineCostEstimate,
  Threshold,
  NumberOfFeatures,
}

pub fn is_heuristic_inline_cost_feature() {}