#![allow(dead_code)]

// This file declares the different classes involved in low
// level diagnostics.

// Defines the different supported severity of a diagnostic.
enum DiagnosticSeverity {
  Error,
  Warning,
  Remark,
  Note
}

// Defines the different supported kind of a siagnostic.
enum DiagnosticKind {
  InlineAsm,
  ResourceLimit,
  StackSize,
  Linker,
  Lowering,
  DebugMetadataVersion,
  DebugMetadataInvalid,
  ISelFallback,
  SampleProfile,
  OptimizationRemark,
  OptimizationRemarkMissed,
  OptimizationRemarkAnalysis,
  OptimizationRemarkAnalysisFPCommute,
  OptimizationRemarkAnalysisAliasing,
  OptimizationFailure,
  MachineOptimizationRemark,
  MachineOptimizationRemarkMissed,
  MachineOptimizationRemarkAnalysis,
  MIRParser,
  PGOProfile,
  Unsupported,
  SrcMgr,
  DontCall,
  MisExpect,
  FirstPluginKind
}

pub fn get_next_available_plugin_diagnostic_kind() {}

struct DiagnosticInfo {}

struct DiagnosticInfoInlineAsm {}

impl DiagnosticInfoInlineAsm {
  pub fn new() {}

  pub fn get_loc_cookie() {}
  pub fn get_msg_str() {}
  pub fn get_instruction() {}
  pub fn print() {}
}