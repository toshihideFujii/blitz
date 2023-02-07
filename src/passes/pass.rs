#![allow(dead_code)]

enum PassManagerType {
  Unkkown,
  ModulePassManager,
  CallGraphPassManager,
  FunctionPassManager,
  LoopPassManager,
  RegionPassManager
}

enum PassKind {
  Region,
  Loop,
  Function,
  CallGraphSCC,
  Module,
  PassManager
}

enum ThinOrFullLTOPhase {
  None,
  ThinLTOPreLink,
  ThinLTOPostLink,
  FullLTOPreLink,
  FullLTOPostLink
}

struct Pass {}

impl Pass {
  pub fn get_pass_kind() {}

  pub fn get_pass_name() {}

  pub fn get_pass_id() {}

  pub fn do_initialization() {}

  pub fn do_finalization() {}

  pub fn print() {}

  pub fn dump() {}

  pub fn create_printer_pass() {}

  pub fn assign_pass_namager() {}

  pub fn prepare_pass_manager() {}

  pub fn get_potential_pass_manager_type() {}

  pub fn set_resolver() {}
  pub fn get_resolver() {}

  pub fn get_analysis_usage() {}

  pub fn release_memeory() {}

  pub fn get_adjusted_analysis_pointer() {}
  pub fn get_as_immutable_pass() {}
  pub fn get_as_pm_data_manager() {}

  pub fn verify_analysis() {}

  pub fn dump_pass_structure() {}

  pub fn loohup_pass_info() {}

  pub fn create_pass() {}

  pub fn get_analysis_if_available() {}

  pub fn must_preserve_analysis_id() {}

  pub fn get_analysis() {}

  pub fn get_analysis_id() {}
}

struct ModulePass {}

struct ImmutablePass {}

struct FunctionPass {}