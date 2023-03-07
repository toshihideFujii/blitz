#![allow(dead_code)]

// Represents analyses that rely on function's control flow.
struct CFGAnalyses {}

// A set of analyses that are preserved following a run of transformation
// pass.
struct PreservedAnalyses {}

// A checker object that makes it easy to query for whether an analysis or
// some set converting it is preserved.
struct PreservedAnalysisChecker {}

// Manages a sequence of passes over a particular unit of IR.
struct PassManager {}