#![allow(dead_code)]

// This file defines an abstraction for handling remarks.

const CURRENT_REMARK_VERSION: u64 = 0;

struct RemarkLocation {}

struct Argument {}

enum RemarkType {
  Unknown,
  Passed,
  Missed,
  Analysis,
  AnalysisFPCompute,
  AnalysisAliasing,
  Failure
}

// A remark type used for both emission and parsing.
struct Remark {}
impl Remark {
  pub fn new() {}
}